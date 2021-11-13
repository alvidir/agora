package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/alvidir/agora"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"github.com/joho/godotenv"
)

const (
	DgraphUriKey = "DGRAPH_URI"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Printf("no dotenv file has been found")
	}

	uri := os.Getenv(DgraphUriKey)
	client, close, err := agora.NewConn(uri)
	if err != nil {
		log.Fatal(err)
	}

	defer close()

	txn := client.NewTxn()
	ctx := context.Background()
	defer txn.Discard(ctx)

	type Person struct {
		Uid   string   `json:"uid,omitempty"`
		Name  string   `json:"name,omitempty"`
		DType []string `json:"dgraph.type,omitempty"`
	}

	p := Person{
		Uid:   "_:alice",
		Name:  "Alice",
		DType: []string{"Person"},
	}

	pb, err := json.Marshal(p)
	if err != nil {
		log.Fatal(err)
	}

	mu := &api.Mutation{
		SetJson: pb,
	}

	req := &api.Request{
		Mutations: []*api.Mutation{mu},
	}

	res, err := txn.Do(ctx, req)
	if err != nil {
		log.Fatal(err)
	}

	q := `query all($a: string) {
		all(func: eq(name, $a)) {
		  name
		}
	  }`

	req = &api.Request{
		Query: q,
		Vars:  map[string]string{"$a": "Alice"},
	}

	if res, err = txn.Do(ctx, req); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("%s\n", res.Json)
}
