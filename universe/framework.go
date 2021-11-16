package universe

import (
	"context"
	"encoding/json"
	"errors"

	"github.com/alvidir/agora"
	"github.com/alvidir/go-util"
	"github.com/dgraph-io/dgo/v210/protos/api"
)

var (
	ErrTransactionFailed = errors.New("transaction has failed")
)

// DgraphUniverseRepository implements the UniverseRepository interface for Dgraph databases
type DgraphUniverseRepository struct {
	client *agora.Dgraph
}

// Find provides the unique universe with the given id, if any
func (repo *DgraphUniverseRepository) Find(ctx context.Context, id string) (uni *Universe, err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

	q := `query universe($id: string) {
		universe(func: uid($id)) {
			uid
			name
			user
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
		return nil, util.ErrNotFound
	}

	return &result.Universe[0], nil
}

// FindByName provides the unique universe with the given name, if any
func (repo *DgraphUniverseRepository) FindByNameAndUser(ctx context.Context, name string, user string) (uni *Universe, err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

	q := `query universe($name: string) {
		universe(func: eq(name, $name)) {
			uid
			name
			user
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
		return nil, util.ErrNotFound
	}

	return &result.Universe[0], nil
}

// Create persists the provided universe
func (repo *DgraphUniverseRepository) Create(ctx context.Context, universe *Universe) (err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

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

	if len(res.GetUids()) == 0 {
		return ErrTransactionFailed
	}

	for _, value := range res.GetUids() {
		universe.Id = value
		break
	}

	return
}

// Delete removes the given universe from the graph
func (repo *DgraphUniverseRepository) Delete(ctx context.Context, universe *Universe) (err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

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
