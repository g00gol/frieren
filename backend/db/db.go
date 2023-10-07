package db

import (
	"context"

	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client

func Connect() error {
	var err error
	opts := options.Client().ApplyURI("mongodb://localhost:27017")

	client, err = mongo.Connect(context.Background(), opts)
	return err
}

func Disconnect() error {
	return client.Disconnect(context.Background())
}

func GetClient() *mongo.Client {
	return client
}
