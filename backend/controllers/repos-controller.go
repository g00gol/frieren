package controllers

import (
	"net/http"

	"github.com/g00gol/frieren/backend/db"
)

func GetRepos(w http.ResponseWriter, r *http.Request) {
	repos := db.GetRepos()
}
