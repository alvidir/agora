package agora

import "testing"

func TestDgraphUniverseRepositoryCreate(t *testing.T) {
	wantUniverse := &Universe{
		Name: "TestingUniverse",
		User: "TestingUser",
	}

	client := NewDgraphClient(uri)
	repo := &DgraphUniverseRepository{client}

	if err := repo.Create(wantUniverse); err != nil {
		t.Fatal(err)
	}

	if len(wantUniverse.Id) == 0 {
		t.Fatalf("Got empty universe Id")
	}

	if gotUniverse, err := repo.Find(wantUniverse.Id); err != nil {
		t.Fatal(err)
	} else if gotUniverse.Id != wantUniverse.Id {
		t.Fatalf("Got id = %s, want %s", gotUniverse.Id, wantUniverse.Id)
	} else if gotUniverse.Name != wantUniverse.Name {
		t.Fatalf("Got name = %s, want %s", gotUniverse.Name, wantUniverse.Name)
	} else if gotUniverse.User != wantUniverse.User {
		t.Fatalf("Got user = %s, want %s", gotUniverse.User, wantUniverse.User)
	} else if err := repo.Delete(gotUniverse); err != nil {
		t.Fatal(err)
	}
}
