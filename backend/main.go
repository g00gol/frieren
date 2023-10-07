package main

import (
  "net/http"
  "github.com/g00gol/frieren/backend/routes"
  "github.com/go-chi/chi/v5"
  "github.com/go-chi/chi/v5/middleware"
)

func main(){
  port := "8080"
  
  r.Use(middleware.Logger)

  r := chi.NewRouter()

	http.ListenAndServe(":" + port, r)
}