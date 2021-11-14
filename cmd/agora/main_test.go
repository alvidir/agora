package main

import (
	"context"
	"encoding/json"
	"log"
	"os"
	"testing"

	"github.com/alvidir/agora"
	"github.com/dgraph-io/dgo/v210"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"github.com/joho/godotenv"
)

const (
	testAttr = "test"
	testName = "TestItemName"
)

var testDgraphClient *dgo.Dgraph

func TestMain(m *testing.M) {
	if err := godotenv.Load("../../.env"); err != nil {
		log.Printf("no dotenv file has been found")
	}

	uri := os.Getenv(DgraphUriKey)
	client, close, err := agora.NewConn(uri)
	if err != nil {
		log.Fatal(err)
	}

	defer close()
	testDgraphClient = client

	code := m.Run()
	os.Exit(code)
}

func TestInsertion(t *testing.T) {
	op := &api.Operation{
		Schema:          `test: string @index(exact) .`,
		RunInBackground: true,
	}

	ctx := context.Background()
	if err := testDgraphClient.Alter(ctx, op); err != nil {
		t.Fatal(err)
	}

	setTx := testDgraphClient.NewTxn()
	defer setTx.Commit(ctx)

	type Test struct {
		Uid   string   `json:"uid,omitempty"`
		Test  string   `json:"test,omitempty"`
		DType []string `json:"dgraph.type,omitempty"`
	}

	tt := Test{
		Uid:   "_:test",
		Test:  testName,
		DType: []string{"Test"},
	}

	pb, err := json.Marshal(tt)
	if err != nil {
		t.Fatal(err)
	}

	mu := &api.Mutation{
		SetJson: pb,
	}

	req := &api.Request{
		Mutations: []*api.Mutation{mu},
		// When CommitNow is set to True the transaction cannot be reused, since the transaction is imediatelly commited.
		// So not only the commit or discard statement must be removed, but the get transaction must be a new one.
		//CommitNow: true,
	}

	if _, err := setTx.Do(ctx, req); err != nil {
		t.Fatal(err)
	}
}

func TestQuery(t *testing.T) {
	q := `query all($a: string) {
		all(func: eq(test, $a)) {
		  test
		}
	  }`

	req := &api.Request{
		Query: q,
		Vars:  map[string]string{"$a": testName},
	}

	getTx := testDgraphClient.NewTxn()
	ctx := context.Background()

	res, err := getTx.Do(ctx, req)
	if err != nil {
		t.Fatal(err)
	}

	responseMap := make(map[string]interface{})
	if err := json.Unmarshal(res.Json, &responseMap); err != nil {
		t.Fatal(err)
	}

	want := 1
	if got, exists := responseMap["all"]; !exists {
		t.Fatalf("Data has no 'all' entry")
	} else if s, ok := got.([]interface{}); !ok {
		t.Fatalf("Value for 'all' entry is not an slice")
	} else if len(s) != want {
		t.Fatalf("Got lenght %v, want %v", len(s), want)
	} else if item := s[0].(map[string]interface{}); item[testAttr] != testName {
		t.Fatalf("Got node test %v, want %v", item[testAttr], testName)
	}
}

func TestDrop(t *testing.T) {
	op := &api.Operation{
		DropAttr: testAttr,
	}

	ctx := context.Background()
	if err := testDgraphClient.Alter(ctx, op); err != nil {
		t.Fatal(err)
	}
}
