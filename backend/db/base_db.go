package db

import (
	"context"
	"log"
	"os"

	"github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client

func Connect() error {
	var err error
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}
	uri := os.Getenv("MONGODB_URI")
	if uri == "" {
		log.Fatal("You must set your 'MONGODB_URI' environment variable. See\n\t https://www.mongodb.com/docs/drivers/go/current/usage-examples/#environment-variable")
	}

	opts := options.Client().ApplyURI(uri)
	client, err = mongo.Connect(context.Background(), opts)
	return err
}

func Disconnect() error {
	return client.Disconnect(context.Background())
}

func GetClient() *mongo.Client {
	return client
}

func GetCollection(name string) *mongo.Collection {
	return client.Database("frieren").Collection(name)
}
