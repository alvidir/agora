package main

import (
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
	GraphqlUriKey  = "GRAPHQL_URI"
)

var WriteOnly = []string{"POST", "PUT", "DELETE"}

func setUniverseRouter(r *mux.Router, client *graphql.Client, logger *logrus.Logger) *mux.Router {
	repo := &agora.GraphqlUniverseRepository{
		Graphql: client,
	}

	handler := agora.NewUniverseHandler(repo, logger)
	r.Handle("/universe", handler).Methods(WriteOnly...)
	return r
}

func setMomentRouter(r *mux.Router, client *graphql.Client, logger *logrus.Logger) *mux.Router {
	repo := &agora.GraphqlMomentRepository{
		Graphql: client,
	}

	handler := agora.NewMomentHandler(repo, logger)
	r.Handle("/moment", handler).Methods(WriteOnly...)
	return r
}

func main() {
	if err := godotenv.Load(); err != nil {
		log.Printf("no dotenv file has been found")
	}

	graphqlUri, err := util.LookupEnv(GraphqlUriKey)
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

	setUniverseRouter(router, graphql, logger)
	setMomentRouter(router, graphql, logger)

	logger.WithField("address", address).Info("server ready to accept connections")
	if err := http.Serve(lis, router); err != nil {
		log.Fatalf("server abruptly terminated: %s", err.Error())
	}
}
