package main

import (
	"log"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"

	"github.com/g00gol/frieren/backend/db"
	"github.com/g00gol/frieren/backend/routes"
)

func main() {
	port := "8080"

	r := chi.NewRouter()
	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"http://*"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "PATCH", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token"},
		ExposedHeaders:   []string{"Link"},
		AllowCredentials: false,
		MaxAge:           300, // Maximum value not ignored by any of major browsers
	}))
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
