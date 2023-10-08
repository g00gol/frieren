package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/charmbracelet/bubbles/progress"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/common-nighthawk/go-figure"
	"github.com/go-git/go-git/v5"
)

const (
	padding  = 2
	maxWidth = 120
)

type model struct {
	root_dir  string
	repo      *git.Repository
	origin    string
	status    int
	init_err  error
	progress  progress.Model
	prog_msg  string
	not_found bool
	fern      fernStruct
	ti_arr    []textinput.Model
}

func sleep() {
	time.Sleep(300 * time.Millisecond)
}

func findRepo() tea.Msg {
	sleep()

	cwd, err := os.Getwd()
	if err != nil {
		return errMsg{err}
	}

	repo, err := git.PlainOpenWithOptions(cwd, &git.PlainOpenOptions{DetectDotGit: true})
	if err != nil {
		return errMsg{err}
	}

	return repoMsg{repo, changeDir(repo)}
}

func changeDir(repo *git.Repository) tea.Cmd {
	return func() tea.Msg {
		sleep()
		wt, err := repo.Worktree()
		if err != nil {
			return errMsg{err}
		}
		dir := wt.Filesystem.Root()
		os.Chdir(dir)
		return dirMsg{dir, fetchOrigin(repo)}
	}
}

func fetchOrigin(repo *git.Repository) tea.Cmd {
	return func() tea.Msg {
		sleep()
		origin, err := repo.Remote("origin")
		if err != nil {
			return errMsg{err}
		}
		url := origin.Config().URLs[0]
		url = url[:len(url)-4]

		return originMsg{url}
	}
}

type repoMsg struct {
	repo      *git.Repository
	changeDir tea.Cmd
}
type dirMsg struct {
	dir         string
	fetchOrigin tea.Cmd
}
type originMsg struct{ origin string }

type errMsg struct{ err error }

type fernStruct struct {
	name                     string   `json:"name"`
	technologies             []string `json:"technologies"`
	difficulty               int      `json:"difficulty"`
	description              string   `json:"description"`
	recommended_issue_labels []string `json:"recommended_issue_labels"`
	repo_origin              string   `json:"repo_origin,omitempty"`
}

func checkForFern() tea.Msg {
	sleep()
	fernFile, err := os.Open("open-source.fern")
	if err != nil {
		return notFoundMsg{}
	}
	log.Println(*fernFile)

	var fern fernStruct
	byteValue, _ := ioutil.ReadAll(fernFile)
	err = json.Unmarshal(byteValue, &fern)
	if err != nil {
		return errMsg{err}
	}
	return foundMsg{fern}
}

type notFoundMsg struct{}

func sendPOST(fern fernStruct) tea.Cmd {
	fernJson, err := json.Marshal(&fern)
	if err != nil {
		log.Printf("[Error] %v\n", err)
		return func() tea.Msg {
			return postFailureMsg{}
		}
	}
	return func() tea.Msg {
		sleep()
		request, error := http.NewRequest("POST", "http://127.0.0.1:8080/repos", bytes.NewBuffer(fernJson))
		request.Header.Set("Content-Type", "application/json; charset=UTF-8")

		client := &http.Client{}
		response, error := client.Do(request)
		if error != nil {
			log.Printf("[Error] %v\n", err)
			return postFailureMsg{}
		}
		defer response.Body.Close()

		if response.StatusCode >= 200 && response.StatusCode < 300 {
			// Dump json to .fern file
			var fern fernStruct
			json.Unmarshal([]byte(fernJson), &fern)
			fern.repo_origin = ""

			log.Printf("%#v\n", fern)
			_ := os.WriteFile("open-source.fern", fern, os.ModePerm)
			return postSuccessMsg{}
		}
		log.Printf("[Error] Issue registering project (status code = %v)\n", response.StatusCode)
		return postFailureMsg{}
	}
}

type postSuccessMsg struct{}

type postFailureMsg struct{}

type foundMsg struct{ fern fernStruct }

// For messages that contain errors it's often handy to also implement the
// error interface on the message.
func (e errMsg) Error() string { return e.err.Error() }

func (m model) Init() tea.Cmd {
	return tea.Sequence(tea.EnterAltScreen, findRepo)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmds []tea.Cmd
	var cmd tea.Cmd

	switch msg := msg.(type) {

	case repoMsg:
		// The server returned a status message. Save it to our model. Also
		// tell the Bubble Tea runtime we want to exit because we have nothing
		// else to do. We'll still be able to render a final view with our
		// status message.
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		cmds = append(cmds, msg.changeDir)
		m.prog_msg += "\x1b[32m✓\x1b[39m" + "\nChanging to root directory... "
		m.repo = msg.repo
	case dirMsg:
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		cmds = append(cmds, msg.fetchOrigin)
		m.prog_msg += "\x1b[32m✓\x1b[39m" + "\nFetching origin url... "
		m.root_dir = msg.dir
	case originMsg:
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		cmds = append(cmds, checkForFern)
		m.prog_msg += "\x1b[32m✓\x1b[39m" + "\nChecking for `open-source.fern` file... "
		m.origin = msg.origin
	case notFoundMsg:
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		m.prog_msg += "❌" + "\nFern file not found... Generating new file... "
		m.not_found = true
	case foundMsg:
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		cmds = append(cmds, sendPOST(m.fern))
		m.prog_msg += "\x1b[32m✓\x1b[39m" + "\nFern file found!  Sending POST request... "
		m.not_found = false
		m.fern = msg.fern

	case postSuccessMsg:
		// Show successful message
		cmd = m.progress.IncrPercent(0.2)
		cmds = append(cmds, cmd)
		m.prog_msg += "\x1b[32m✓" + "\nPOST request succeeded!  Your project has been registered.  You may close this window with Ctrl+C."

	case postFailureMsg:
		// Show unsuccessful message
		m.prog_msg += "❌" + "\n\x1b[31mPOST request failed... Your project has not been registered.  Please close this window with Ctrl+C and try again."

	case tea.WindowSizeMsg:
		m.progress.Width = msg.Width - padding*2 - 4
		if m.progress.Width > maxWidth {
			m.progress.Width = maxWidth
		}
		return m, nil

	case errMsg:
		// There was an error. Note it in the log. And tell the runtime
		// we're done and want to quit.
		log.Printf("[Error] '%v'\n", msg)
		m.init_err = msg
		return m, tea.Quit

	case tea.KeyMsg:
		// Ctrl+c exits. Even with short running programs it's good to have
		// a quit key, just in case your logic is off. Users will be very
		// annoyed if they can't exit.
		if msg.Type == tea.KeyCtrlC {
			return m, tea.Quit
		}
		if m.not_found {
			if msg.Type == tea.KeyUp {
				for i, ti := range m.ti_arr {
					if ti.Focused() {
						if i != 0 {
							m.ti_arr[i].Blur()
							m.ti_arr[i-1].Focus()
						}
						break
					}
				}
			}
			if msg.Type == tea.KeyEnter {
				// Validate data
				// TODO
				// If valid, send POST
				if m.not_found == true {
					for i, _ := range m.ti_arr {
						m.ti_arr[i].Blur()
					}
					m.fern.repo_origin = m.origin
					cmds = append(cmds, sendPOST(m.fern))
				}
				m.not_found = false
			}
			if msg.Type == tea.KeyDown {
				for i, ti := range m.ti_arr {
					if ti.Focused() {
						if i != len(m.ti_arr)-1 {
							m.ti_arr[i].Blur()
							m.ti_arr[i+1].Focus()
						}
						break
					}
				}
			}
		}

	// FrameMsg is sent when the progress bar wants to animate itself
	case progress.FrameMsg:
		progressModel, cmd := m.progress.Update(msg)
		m.progress = progressModel.(progress.Model)
		cmds = append(cmds, cmd)
	}

	focused_index := -1
	for i, ti := range m.ti_arr {
		if ti.Focused() {
			focused_index = i
		}
	}
	if focused_index > -1 {
		m.ti_arr[focused_index], cmd = m.ti_arr[focused_index].Update(msg)
		cmds = append(cmds, cmd)
	}

	return m, tea.Batch(cmds...)
}

func (m model) View() string {
	// If there's an error, print it out and don't do anything else.
	helpMessage := ""
	myFigure := figure.NewColorFigure("Frieren  Open  Source", "jazmine", "blue", true)
	cstr := myFigure.ColorString()
	s := cstr + "\n" +
		m.prog_msg + "\n\n" +
		m.progress.View() + "\n"

	if m.not_found == true {
		s += "\n"
		for _, ti := range m.ti_arr {
			s += ti.View() + "\n\n"
		}
		helpMessage = "Up Arrow - Move to Previous Text Box  |  Down Arrow - Move to Next Text Box  |  Enter/Return - Submit Text Entries"
	}
	if m.init_err != nil {
		return fmt.Sprintf("\nWe had some trouble: %v\n\n", m.init_err)
	}

	// Send off whatever we came up with above for rendering.
	return s + lipgloss.NewStyle().Foreground(lipgloss.Color("#626262")).Render(helpMessage+"\n"+"Press Ctrl+C to quit.")
}

func initialModel() model {
	var ti_arr []textinput.Model
	var ti textinput.Model

	// Technology input
	ti = textinput.New()
	ti.Prompt = "\x1b[36mInput technologies separated by a comma> \x1b[39m"
	ti.Placeholder = "Tech 1[, Tech 2, Tech 3, ...]"
	ti.CharLimit = 250
	ti_arr = append(ti_arr, ti)

	// Difficulty input
	ti = textinput.New()
	ti.Prompt = "\x1b[36mHow difficult is your project to contribute to (1 to 5)> \x1b[39m"
	ti.Placeholder = ""
	ti.Width = 8
	ti.CharLimit = 1
	ti_arr = append(ti_arr, ti)

	// Description input
	ti = textinput.New()
	ti.Prompt = "\x1b[36mPlease enter a brief description> \x1b[39m"
	ti.Placeholder = ""
	ti.CharLimit = 500
	ti_arr = append(ti_arr, ti)

	// Issue Labels input
	ti = textinput.New()
	ti.Prompt = "\x1b[36mInput any tags/labels that represent good first issues separated by a comma> \x1b[39m"
	ti.Placeholder = "Label 1[, Label 2, Label 3, ...]"
	ti.CharLimit = 250
	ti_arr = append(ti_arr, ti)

	ti_arr[0].Focus()
	m := model{
		progress: progress.New(progress.WithDefaultGradient()),
		prog_msg: "Finding git repository... ",
		ti_arr:   ti_arr,
	}
	m.progress.Width = 120
	return m
}

func main() {
	file, err := os.OpenFile("app.log", os.O_WRONLY|os.O_CREATE|os.O_APPEND, 0644)
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	log.SetOutput(file)
	log.Println("Log file created successfully.")

	// if _, err := tea.NewProgram(initialModel(), tea.WithAltScreen()).Run(); err != nil {
	if _, err := tea.NewProgram(initialModel()).Run(); err != nil {
		fmt.Printf("Uh oh, there was an error: %v\n", err)
		os.Exit(1)
	}
}
