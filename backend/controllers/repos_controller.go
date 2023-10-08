package controllers

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/g00gol/frieren/backend/db"
	"github.com/g00gol/frieren/backend/types"
	"github.com/g00gol/frieren/backend/utils"
)

func GetRepos(w http.ResponseWriter, r *http.Request) {
	// Get the filters from the request
	filter := utils.ConstructFilters(r, types.Repo{})

	// Get data from database
	data, err := db.GetReposByFilters(filter)
	if err != nil {
		log.Println(err)
	}

	// Return data as JSON
	w.Header().Set("Content-Type", "application/json")
	err = json.NewEncoder(w).Encode(data)
	if err != nil {
		log.Println(err)
	}
}

func CreateRepo(w http.ResponseWriter, r *http.Request) {
	var repo types.Repo

	// Decode the request body into repo
	err := json.NewDecoder(r.Body).Decode(&repo)
	if err != nil {
		log.Println(err)
	}

	// Insert repo into database
}
