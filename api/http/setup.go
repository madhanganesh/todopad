package http

import (
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"strings"

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

	workDir, _ := os.Getwd()
	filesDir := http.Dir(filepath.Join(workDir, "public"))
	FileServer(router, "/", filesDir)

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

func FileServer(r chi.Router, path string, root http.FileSystem) {
	if strings.ContainsAny(path, "{}*") {
		panic("FileServer does not permit any URL parameters.")
	}

	if path != "/" && path[len(path)-1] != '/' {
		r.Get(path, http.RedirectHandler(path+"/", 301).ServeHTTP)
		path += "/"
	}
	path += "*"

	r.Get(path, func(w http.ResponseWriter, r *http.Request) {
		rctx := chi.RouteContext(r.Context())
		pathPrefix := strings.TrimSuffix(rctx.RoutePattern(), "/*")
		fs := http.StripPrefix(pathPrefix, http.FileServer(root))
		fs.ServeHTTP(w, r)
	})
}
