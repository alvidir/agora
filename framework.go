package agora

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"github.com/alvidir/go-util"
	"github.com/dgraph-io/dgo/v210"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"github.com/gorilla/mux"
)

var (
	ErrTransactionFailed = errors.New("transaction has failed")
)

func RegisterRoutes(router *mux.Router) {
	router.HandleFunc("/universe", UniverseCreateHandler).Methods("POST", "PUT")
}

func UniverseCreateHandler(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "Category: %v\n", vars["category"])
}

// DgraphUniverseRepository implements the UniverseRepository interface for Dgraph databases
type DgraphUniverseRepository struct {
	client DGraphClient
}

func (repo *DgraphUniverseRepository) finishTx(ctx context.Context, tx *dgo.Txn, err *error) {
	if err == nil || *err == nil {
		tx.Commit(ctx)
	} else {
		tx.Discard(ctx)
	}
}

// Find provides the unique universe with the given id, if any
func (repo *DgraphUniverseRepository) Find(id string) (uni *Universe, err error) {
	client, close, err := repo.client.Connect()
	if err != nil {
		return
	}

	defer close()

	ctx := context.Background()
	tx := client.NewTxn()
	defer repo.finishTx(ctx, tx, &err)

	q := `query universe($id: string) {
		universe(func: uid($id)) {
			uid
			Universe.name
			Universe.user
			dgraph.type
		}
	}`

	req := &api.Request{
		Query: q,
		Vars:  map[string]string{"$id": id},
	}

	res, err := tx.Do(ctx, req)
	if err != nil {
		return
	}

	type tuple struct {
		Universe []Universe `json:"universe"`
	}

	var result tuple
	if err = json.Unmarshal(res.GetJson(), &result); err != nil {
		return
	}

	if len(result.Universe) != 1 {
		err = util.ErrNotFound
		return
	}

	return &result.Universe[0], nil
}

// FindByName provides the unique universe with the given name, if any
func (repo *DgraphUniverseRepository) FindByName(name string) (uni *Universe, err error) {
	client, close, err := repo.client.Connect()
	if err != nil {
		return
	}

	defer close()

	ctx := context.Background()
	tx := client.NewTxn()
	defer repo.finishTx(ctx, tx, &err)

	q := `query universe($name: string) {
		universe(func: eq(name, $name)) {
			uid
			Universe.name
			Universe.user
			dgraph.type
		}
	}`

	req := &api.Request{
		Query: q,
		Vars:  map[string]string{"$name": name},
	}

	res, err := tx.Do(ctx, req)
	if err != nil {
		return
	}

	type tuple struct {
		Universe []Universe `json:"universe"`
	}

	var result tuple
	if err = json.Unmarshal(res.GetJson(), &result); err != nil {
		return
	}

	if len(result.Universe) != 1 {
		err = util.ErrNotFound
		return
	}

	return &result.Universe[0], nil
}

// Create presists the provided universe
func (repo *DgraphUniverseRepository) Create(universe *Universe) (err error) {
	client, close, err := repo.client.Connect()
	if err != nil {
		return
	}

	defer close()

	ctx := context.Background()
	tx := client.NewTxn()
	defer repo.finishTx(ctx, tx, &err)

	pb, err := json.Marshal(universe)
	if err != nil {
		return
	}

	mu := &api.Mutation{
		SetJson: pb,
	}

	req := &api.Request{
		Mutations: []*api.Mutation{mu},
	}

	res, err := tx.Do(ctx, req)
	if err != nil {
		return
	}

	for _, value := range res.GetUids() {
		universe.Id = value
	}

	if len(universe.Id) == 0 {
		return ErrTransactionFailed
	}

	return
}

// Delete removes the given universe from the graph
func (repo *DgraphUniverseRepository) Delete(universe *Universe) (err error) {
	client, close, err := repo.client.Connect()
	if err != nil {
		return
	}

	defer close()

	ctx := context.Background()
	tx := client.NewTxn()
	defer repo.finishTx(ctx, tx, &err)

	pb, err := json.Marshal(universe)
	if err != nil {
		return
	}

	mu := &api.Mutation{
		DeleteJson: pb,
	}

	req := &api.Request{
		Mutations: []*api.Mutation{mu},
	}

	_, err = tx.Do(ctx, req)
	return
}
