package http

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/madhanganesh/todopad/api/config"
	"github.com/madhanganesh/todopad/api/controller"
	"github.com/madhanganesh/todopad/api/model"
	"github.com/madhanganesh/todopad/api/repository"
	"github.com/rs/cors"
)

func NewServer(appConfig *config.App) http.Server {

	userRepository := repository.NewUserRepository(appConfig.Db)
	authController := controller.NewAuthController(userRepository, appConfig.SecretKey)

	router := chi.NewRouter()
	router.Use(middleware.RequestID)
	router.Use(middleware.RealIP)
	router.Use(middleware.Logger)
	router.Use(middleware.Recoverer)
	router.Use(cors.Default().Handler)

	router.Post("/signup", authController.SignUpUser)
	router.Post("/login", authController.Login)
	router.Get("/secureping", authController.Middleware(ping))

	server := http.Server{
		Addr:    ":" + appConfig.Port,
		Handler: router,
	}

	return server
}

func ping(w http.ResponseWriter, r *http.Request) {
	var user model.User
	user.Email = r.Header.Get("email")
	userid, _ := strconv.ParseInt(r.Header.Get("userid"), 10, 64)
	user.ID = userid

	w.WriteHeader(200)
	w.Header().Set("Content-Type", "application/json; charset=UTF-8")
	err := json.NewEncoder(w).Encode(&user)
	if err != nil {
		w.WriteHeader(500)
	}
}
