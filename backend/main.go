package main

import (
	"log"
	"net/http"

	"github.com/g00gol/frieren/backend/routes"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

func main() {
	port := "8080"

	r := chi.NewRouter()
	r.Use(middleware.Logger)
	routes.RegisterRoutes(r)

	log.Println("Starting server on port " + port)
	http.ListenAndServe(":"+port, r)
}
