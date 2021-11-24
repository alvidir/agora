//go:build all || integration
// +build all integration

package agora

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

	graphqlUri = os.Getenv("GRAPHQL_URI")
}

func TestDgraphUniverseRepositoryFind(t *testing.T) {
	wantUniverse := &Universe{
		Name:        "TestDgraphUniverseRepositoryFind_name",
		User:        "TestDgraphUniverseRepositoryFind_user",
		Description: "TestDgraphUniverseRepositoryFind_description",
	}

	graphql := graphql.NewClient(graphqlUri, nil)
	repo := &GraphqlUniverseRepository{graphql}
	ctx := context.Background()
	if err := repo.Insert(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	defer func(u *Universe) {
		if err := repo.Delete(ctx, u); err != nil {
			t.Fatal(err)
		}
	}(wantUniverse)

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	if gotUniverse, err := repo.Find(ctx, wantUniverse.Id); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Errorf("Got id = %v, want %v", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Errorf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Errorf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if gotUniverse.Description != wantUniverse.Description {
		t.Errorf("Got description = %s, want %s", gotUniverse.Description, wantUniverse.Description)
	}
}

func TestDgraphUniverseRepositoryFindByNameAndUser(t *testing.T) {
	wantUniverse := &Universe{
		Name:        "TestDgraphUniverseRepositoryFindByNameAndUser_name",
		User:        "TestDgraphUniverseRepositoryFindByNameAndUser_user",
		Description: "TestDgraphUniverseRepositoryFindByNameAndUser_description",
	}

	graphql := graphql.NewClient(graphqlUri, nil)
	repo := &GraphqlUniverseRepository{graphql}
	ctx := context.Background()
	if err := repo.Insert(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	defer func(u *Universe) {
		if err := repo.Delete(ctx, u); err != nil {
			t.Fatal(err)
		}
	}(wantUniverse)

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	if gotUniverse, err := repo.FindByNameAndUser(ctx, wantUniverse.Name, wantUniverse.User); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Errorf("Got id = %v, want %v", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Errorf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Errorf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if gotUniverse.Description != wantUniverse.Description {
		t.Errorf("Got description = %s, want %s", gotUniverse.Description, wantUniverse.Description)
	}
}

func TestDgraphUniverseRepositoryUpdate(t *testing.T) {
	wantUniverse := &Universe{
		Name:        "TestDgraphUniverseRepositoryUpdate_name_before",
		User:        "TestDgraphUniverseRepositoryUpdate_user_before",
		Description: "TestDgraphUniverseRepositoryUpdate_description_before",
	}

	graphql := graphql.NewClient(graphqlUri, nil)
	repo := &GraphqlUniverseRepository{graphql}
	ctx := context.Background()
	if err := repo.Insert(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	defer func(u *Universe) {
		if err := repo.Delete(ctx, u); err != nil {
			t.Fatal(err)
		}
	}(wantUniverse)

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	wantUniverse.Name = "TestDgraphUniverseRepositoryUpdate_name_after"
	wantUniverse.User = "TestDgraphUniverseRepositoryUpdate_user_after"
	wantUniverse.Description = "TestDgraphUniverseRepositoryUpdate_description_after"

	if err := repo.Update(ctx, wantUniverse); err != nil {
		t.Fatal(err)
	}

	if gotUniverse, err := repo.Find(ctx, wantUniverse.Id); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Errorf("Got id = %v, want %v", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Errorf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Errorf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if gotUniverse.Description != wantUniverse.Description {
		t.Errorf("Got description = %s, want %s", gotUniverse.Description, wantUniverse.Description)
	}
}
