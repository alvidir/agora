//go:build all || integration
// +build all integration

package universe

import (
	"context"
	"log"
	"os"
	"testing"

	"github.com/alvidir/agora"
	"github.com/joho/godotenv"
)

var uri string = "localhost:8080"

func init() {
	if err := godotenv.Load(); err != nil {
		log.Fatal(err)
	}

	uri = os.Getenv("TEST_DGRAPH_URI")
}

func TestDgraphUniverseRepositoryCreate(t *testing.T) {
	wantUniverse := &Universe{
		Name: "TestDgraphUniverseRepositoryCreateName",
		User: "TestDgraphUniverseRepositoryCreateUser",
	}

	client, err := agora.Open(uri)
	if err != nil {
		t.Fatal(err)
	}

	repo := &DgraphUniverseRepository{client}
	ctx := context.Background()
	if err := repo.Create(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	if gotUniverse, err := repo.Find(ctx, wantUniverse.Id); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Fatalf("Got id = %s, want %s", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Fatalf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Fatalf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if err := repo.Delete(ctx, gotUniverse); err != nil {
		t.Fatal(err)
	}
}
