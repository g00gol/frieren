package db

import (
	"go.mongodb.org/mongo-driver/mongo"
)

func GetRepos() *mongo.Collection {
	collection := GetCollection("repos")
	return collection
}
