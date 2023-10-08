package types

import "time"

type Repo struct {
	Hash                   string    `bson:"hash"`
	Name                   string    `bson:"name"`
	Description            string    `bson:"description"`
	RepoOrigin             string    `bson:"repo_origin"`
	FernBranch             string    `bson:"fern_branch"`
	Languages              []string  `bson:"languages"`
	Technologies           []string  `bson:"technologies"`
	RecommendedIssueLabels []string  `bson:"recommended_issue_labels"`
	RecommendedIssuesCount int       `bson:"recommended_issues_count"`
	Difficulty             int       `bson:"difficulty"`
	LastUpdated            time.Time `bson:"last_updated"` // MongoDB Datetime
	DateCreated            time.Time `bson:"date_created"` // MongoDB Datetime
	Stars                  int       `bson:"stars"`
}
