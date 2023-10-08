package main

import (
	"log"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"

	"github.com/g00gol/frieren/backend/db"
	"github.com/g00gol/frieren/backend/routes"
)

func main() {
	port := "8080"

	r := chi.NewRouter()
	r.Use(middleware.Logger)
	routes.RegisterRoutes(r)

	// Connect to MongoDB
	err := db.Connect()
	defer db.Disconnect()
	if err != nil {
		log.Fatal(err)
	} else {
		log.Println("Connected to MongoDB")
	}

	log.Println("Starting server on port " + port)
	http.ListenAndServe(":"+port, r)
}
