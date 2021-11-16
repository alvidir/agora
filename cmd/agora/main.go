package main

import (
	"log"
	"net"
	"net/http"

	"github.com/alvidir/agora/universe"
	"github.com/alvidir/go-util"
	"github.com/joho/godotenv"
	"github.com/sirupsen/logrus"
)

const (
	EnvServiceNetw = "SERVICE_NETW"
	EnvServiceAddr = "SERVICE_ADDR"
	DgraphUriKey   = "DGRAPH_URI"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Printf("no dotenv file has been found")
	}

	uri, err := util.LookupEnv(DgraphUriKey)
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
	universeHandler := &universe.UniverseHandler{
		Uri:    uri,
		Logger: logger,
	}

	http.HandleFunc("/", universeHandler.UniverseCreateHandler)
	logger.WithField("address", address).Info("server ready to accept connections")
	if err := http.Serve(lis, nil); err != nil {
		log.Fatalf("server abruptly terminated: %s", err.Error())
	}
}
