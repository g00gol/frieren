package db

import (
	"context"
	"log"

	"github.com/g00gol/frieren/backend/types"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo/options"
)

func GetReposByFilters(filter any) ([]types.Repo, error) {
	collection := GetCollection("repos")

	// Specify options to use for string comparison
	opts := options.Find().SetCollation(&options.Collation{
		Locale:   "en",
		Strength: 1,
	})
	cursor, err := collection.Find(context.TODO(), filter, opts)
	if err != nil {
		log.Println("Error finding repos:", err)
		return nil, err
	}
	defer cursor.Close(context.TODO())

	var data []types.Repo
	if err := cursor.All(context.Background(), &data); err != nil {
		log.Println("Error decoding cursor results:", err)
		return nil, err
	}

	return data, err
}

func GetRepoByName(name string) (types.Repo, error) {
	collection := GetCollection("repos")

	var data types.Repo

	filter := bson.D{{Key: "name", Value: name}}
	err := collection.FindOne(context.TODO(), filter).Decode(&data)

	if err != nil {
		log.Println("Error finding repo:", err)
		return types.Repo{}, err
	}

	log.Println("Found repo:", data)
	return data, err
}

func DeleteRepoByName(name string) (int64, error) {
	collection := GetCollection("repos")

	filter := bson.D{{Key: "name", Value: name}}
	result, err := collection.DeleteOne(context.TODO(), filter)
	if err != nil {
		log.Println("Error deleting repo:", err)
		return 0, err
	}

	return result.DeletedCount, err
}
