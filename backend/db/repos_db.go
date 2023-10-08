package db

import (
	"context"
	"log"

	"github.com/g00gol/frieren/backend/types"
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
