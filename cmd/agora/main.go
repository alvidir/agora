package main

import (
	"errors"
	"log"
	"net"
	"net/http"

	"github.com/alvidir/agora"
	"github.com/alvidir/go-util"
	"github.com/gorilla/mux"
	"github.com/joho/godotenv"
	"github.com/shurcooL/graphql"
	"github.com/sirupsen/logrus"
)

const (
	EnvServiceNetw = "SERVICE_NETW"
	EnvServiceAddr = "SERVICE_ADDR"
	EnvGraphqlUri  = "GRAPHQL_URI"
	EnvAuthHeader  = "AUTH_HEADER"
)

func setUniverseRouter(r *mux.Router, handler agora.Handler, client *graphql.Client, logger *logrus.Logger) *mux.Router {
	repo := &agora.GraphqlUniverseRepository{
		Graphql: client,
	}

	h := agora.NewUniverseHandler(handler, repo, logger)
	r.HandleFunc("/universe/create", h.CreateUniverse).Methods("POST")
	return r
}

func setMomentRouter(r *mux.Router, handler agora.Handler, client *graphql.Client, logger *logrus.Logger) *mux.Router {
	repo := &agora.GraphqlMomentRepository{
		Graphql: client,
	}

	h := agora.NewMomentHandler(handler, repo, logger)
	r.HandleFunc("/moment/create", h.CreateMoment).Methods("POST")
	return r
}

func main() {
	if err := godotenv.Load(); err != nil {
		log.Printf("no dotenv file has been found")
	}

	graphqlUri, err := util.LookupEnv(EnvGraphqlUri)
	if err != nil {
		log.Fatal(err)
	}

	network, err := util.LookupEnv(EnvServiceNetw)
	if err != nil {
		log.Fatalf("%s: %s", EnvServiceNetw, err)
	}

	address, err := util.LookupEnv(EnvServiceAddr)
	if err != nil {
		log.Fatalf("%s: %s", EnvServiceAddr, err)
	}

	lis, err := net.Listen(network, address)
	if err != nil {
		log.Fatal(err)
	}

	logger := logrus.New()
	graphql := graphql.NewClient(graphqlUri, nil)
	router := mux.NewRouter()

	auth, err := util.LookupEnv(EnvAuthHeader)
	if err != nil {
		log.Fatalf("%s: %s", EnvAuthHeader, err)
	}

	handler := &agora.HandlerImplementation{
		UserIdFunc: func(r *http.Request) (string, error) {
			if values := r.Header[auth]; len(values) == 0 {
				return "", errors.New("unauthorized")
			} else {
				return values[0], nil
			}
		},
	}

	setUniverseRouter(router, handler, graphql, logger)
	setMomentRouter(router, handler, graphql, logger)

	logger.WithField("address", address).Info("server ready to accept connections")
	if err := http.Serve(lis, router); err != nil {
		log.Fatalf("server abruptly terminated: %s", err.Error())
	}
}
