package agora

import (
	"log"
	"os"
	"testing"

	"github.com/joho/godotenv"
)

var uri string

func TestMain(m *testing.M) {
	if err := godotenv.Load(); err != nil {
		log.Fatal(err)
	}

	uri = os.Getenv("DGRAPH_URI")
	code := m.Run()
	os.Exit(code)
}
