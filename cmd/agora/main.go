package main

import (
	"github.com/joho/godotenv"
	"go.uber.org/zap"
)

const (
	EnvServiceNetw = "SERVICE_NETW"
	EnvServiceAddr = "SERVICE_ADDR"
	EnvAuthHeader  = "AUTH_HEADER"
	EnvGraphqlUrl  = "DGRAPH_DSN"
)

func main() {
	logger, _ := zap.NewProduction()
	defer logger.Sync()

	if err := godotenv.Load(); err != nil {
		logger.Warn("no dotenv file has been found",
			zap.Error(err))
	}
}
