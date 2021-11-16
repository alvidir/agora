package universe

import (
	"context"
	"encoding/json"
	"errors"

	"github.com/alvidir/agora"
	"github.com/alvidir/go-util"
	"github.com/dgraph-io/dgo/v210/protos/api"
)

const (
	dgraphUniverseType = "Universe"
	queryFindById      = `query universe($id: string) {
		universe(func: uid($id)) {
			uid
			name
			user
		}
	}`

	queryFindByNameAndUser = `query universe($name: string, $user: string) {
		universe(func: eq(name, $name) and eq(user, $user)) {
			uid
			name
			user
		}
	}`
)

var (
	ErrTransactionFailed = errors.New("transaction has failed")
)

type dgraphUniverse struct {
	*Universe
	Uid   string   `json:"uid,omitempty"`
	DType []string `json:"dgraph.type,omitempty"`
}

type dgraphUniverseTuple struct {
	Universe []dgraphUniverse `json:"universe"`
}

func parseModel(universe *Universe) *dgraphUniverse {
	return &dgraphUniverse{
		Universe: universe,
		Uid:      universe.Id,
		DType:    []string{dgraphUniverseType},
	}
}

func (universe *dgraphUniverse) toModel() *Universe {
	universe.Id = universe.Uid
	return universe.Universe
}

// DgraphUniverseRepository implements the UniverseRepository interface for Dgraph databases
type DgraphUniverseRepository struct {
	client *agora.Dgraph
}

// Find provides the unique universe with the given id, if any
func (repo *DgraphUniverseRepository) Find(ctx context.Context, id string) (uni *Universe, err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

	req := &api.Request{
		Query: queryFindById,
		Vars:  map[string]string{"$id": id},
	}

	res, err := tx.Do(ctx, req)
	if err != nil {
		return
	}

	var result dgraphUniverseTuple
	if err = json.Unmarshal(res.GetJson(), &result); err != nil {
		return
	}

	if len(result.Universe) != 1 {
		return nil, util.ErrNotFound
	}

	return result.Universe[0].toModel(), nil
}

// FindByName provides the unique universe with the given name, if any
func (repo *DgraphUniverseRepository) FindByNameAndUser(ctx context.Context, name string, user string) (uni *Universe, err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

	req := &api.Request{
		Query: queryFindByNameAndUser,
		Vars: map[string]string{
			"$name": name,
			"$user": user,
		},
	}

	res, err := tx.Do(ctx, req)
	if err != nil {
		return
	}

	var result dgraphUniverseTuple
	if err = json.Unmarshal(res.GetJson(), &result); err != nil {
		return
	}

	if len(result.Universe) != 1 {
		return nil, util.ErrNotFound
	}

	return result.Universe[0].toModel(), nil
}

// Create persists the provided universe
func (repo *DgraphUniverseRepository) Create(ctx context.Context, universe *Universe) (err error) {
	tx := repo.client.Begin(ctx)
	defer tx.Finish(&err)

	dgraphUniverse := parseModel(universe)
	pb, err := json.Marshal(dgraphUniverse)
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

	dgraphUniverse := parseModel(universe)
	pb, err := json.Marshal(dgraphUniverse)
	if err != nil {
		return
	}

	mu := &api.Mutation{
		DeleteJson: pb,
	}

	req := &api.Request{
		Mutations: []*api.Mutation{mu},
	}

	if _, err = tx.Do(ctx, req); err != nil {
		return
	}

	universe.Id = ""
	return
}
