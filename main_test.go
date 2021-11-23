package agora

import (
	"log"
	"os"
	"testing"

	"github.com/joho/godotenv"
)

var graphqlUri string = "http://localhost:8080/graphql"

func TestMain(m *testing.M) {
	if err := godotenv.Load(); err != nil {
		log.Print(err)
	}

	graphqlUri = os.Getenv("GRAPHQL_URI")
	os.Exit(m.Run())
}
