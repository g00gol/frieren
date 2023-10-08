package routes

import (
	"github.com/go-chi/chi/v5"
)

func RegisterRoutes(r *chi.Mux) {
	ReposRoute(r)
}
