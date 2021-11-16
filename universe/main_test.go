package universe

import (
	"log"
	"os"
	"testing"

	"github.com/joho/godotenv"
)

var uri string = "localhost:9080"

func TestMain(m *testing.M) {
	if err := godotenv.Load(); err != nil {
		log.Fatal(err)
	}

	uri = os.Getenv("TEST_DGRAPH_URI")
	code := m.Run()
	os.Exit(code)
}
