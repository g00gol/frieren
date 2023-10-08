package types

import "time"

type Repo struct {
	Hash                   string    `bson:"hash" json:"hash"`
	Name                   string    `bson:"name" json:"name"`
	Description            string    `bson:"description" json:"description"`
	RepoOrigin             string    `bson:"repo_origin" json:"repo_origin"`
	FernBranch             string    `bson:"fern_branch" json:"fern_branch"`
	Languages              []string  `bson:"languages" json:"languages"`
	Technologies           []string  `bson:"technologies" json:"technologies"`
	RecommendedIssueLabels []string  `bson:"recommended_issue_labels" json:"recommended_issue_labels"`
	RecommendedIssuesCount int       `bson:"recommended_issues_count" json:"recommended_issues_count"`
	Difficulty             int       `bson:"difficulty" json:"difficulty"`
	LastUpdated            time.Time `bson:"last_updated" json:"last_updated"`
	DateCreated            time.Time `bson:"date_created" json:"date_created"`
	Stars                  int       `bson:"stars" json:"stars"`
}
