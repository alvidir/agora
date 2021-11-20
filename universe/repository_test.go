//go:build all || integration
// +build all integration

package universe

import (
	"context"
	"log"
	"os"
	"testing"

	"github.com/joho/godotenv"
	"github.com/shurcooL/graphql"
)

var graphqlUri string = "http://localhost:8080/graphql"

func init() {
	if err := godotenv.Load(); err != nil {
		log.Fatal(err)
	}

	graphqlUri = os.Getenv("TEST_GRAPHQL_URI")
}

func TestDgraphUniverseRepositoryFind(t *testing.T) {
	wantUniverse := &Universe{
		Name:        "TestDgraphUniverseRepositoryFind_name",
		User:        "TestDgraphUniverseRepositoryFind_user",
		Description: "TestDgraphUniverseRepositoryFind_description",
	}

	graphql := graphql.NewClient(graphqlUri, nil)
	repo := &graphqlUniverseRepository{graphql}
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
		t.Fatalf("Got id = %v, want %v", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Fatalf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Fatalf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if gotUniverse.Description != wantUniverse.Description {
		t.Fatalf("Got description = %s, want %s", gotUniverse.Description, wantUniverse.Description)
	} else if err := repo.Delete(ctx, gotUniverse); err != nil {
		t.Fatal(err)
	}
}

func TestDgraphUniverseRepositoryFindByNameAndUser(t *testing.T) {
	wantUniverse := &Universe{
		Name:        "TestDgraphUniverseRepositoryFindByNameAndUser_name",
		User:        "TestDgraphUniverseRepositoryFindByNameAndUser_user",
		Description: "TestDgraphUniverseRepositoryFindByNameAndUser_description",
	}

	graphql := graphql.NewClient(graphqlUri, nil)
	repo := &graphqlUniverseRepository{graphql}
	ctx := context.Background()
	if err := repo.Create(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	if gotUniverse, err := repo.FindByNameAndUser(ctx, wantUniverse.Name, wantUniverse.User); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Fatalf("Got id = %v, want %v", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Fatalf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Fatalf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if gotUniverse.Description != wantUniverse.Description {
		t.Fatalf("Got description = %s, want %s", gotUniverse.Description, wantUniverse.Description)
	} else if err := repo.Delete(ctx, gotUniverse); err != nil {
		t.Fatal(err)
	}
}
