package controllers

import (
	"context"
	"encoding/json"
	"log"
	"net/http"

	"github.com/g00gol/frieren/backend/db"
	"go.mongodb.org/mongo-driver/bson"
)

func GetRepos(w http.ResponseWriter, r *http.Request) {
	reposColl := db.GetRepos()
	var data bson.M
	reposColl.FindOne(context.TODO(), bson.D{}).Decode(&data)

	// Return data as JSON
	w.Header().Set("Content-Type", "application/json")
	err := json.NewEncoder(w).Encode(data)
	if err != nil {
		log.Println(err)
	}
}
