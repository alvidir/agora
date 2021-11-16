package universe

import (
	"context"
	"testing"

	"github.com/alvidir/agora"
)

func TestDgraphUniverseRepositoryCreate(t *testing.T) {
	wantUniverse := &Universe{
		Name: "TestingUniverse",
		User: "TestingUser",
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
