package controllers

import (
	"context"
	"encoding/json"
	"net/http"

	"github.com/g00gol/frieren/backend/db"
	"go.mongodb.org/mongo-driver/bson"
)

func GetRepos(w http.ResponseWriter, r *http.Request) {
	repos := db.GetRepos()
	var result bson.M
	repos.FindOne(context.TODO(), bson.M{}).Decode(&result)
	jsonData, _ := json.MarshalIndent(result, "", "    ")
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonData)
}
