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
	repos := db.GetRepos()
	var result bson.M
	repos.FindOne(context.TODO(), bson.D{}).Decode(&result)
	log.Println(result)
	jsonData, err := json.MarshalIndent(result, "", "    ")
	if err != nil {
		log.Println(err)
	}
	w.Header().Set("Content-Type", "application/json")
	w.Write(jsonData)
}
