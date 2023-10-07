package routes

import (
	"github.com/go-chi/chi/v5"

	"github.com/g00gol/frieren/backend/controllers"
)

func ReposRoute(r *chi.Mux) {
	r.Route("/repos", func(r chi.Router) {
		r.Post("/", controllers.GetRepos)
	})
}
