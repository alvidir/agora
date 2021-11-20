package universe

import (
	"context"
	"errors"
	"fmt"

	"github.com/shurcooL/graphql"
)

var (
	ErrTransactionFailed = errors.New("transaction has failed")
)

// graphqlUniverseRepository implements the UniverseRepository interface for Dgraph databases
type graphqlUniverseRepository struct {
	graphql *graphql.Client
}

// Find provides the unique universe with the given id, if any
func (repo *graphqlUniverseRepository) Find(ctx context.Context, id string) (uni *Universe, err error) {
	var query struct {
		Universe Universe `graphql:"getUniverse(id: $id)"`
	}

	variables := map[string]interface{}{
		"id": graphql.ID(id),
	}

	if err = repo.graphql.Query(ctx, &query, variables); err != nil {
		return
	}

	return &query.Universe, nil
}

// FindByName provides the unique universe with the given name, if any
// func (repo *graphqlUniverseRepository) FindByNameAndUser(ctx context.Context, name string, user string) (uni *Universe, err error) {
// 	tx := repo.dgraph.Begin(ctx)
// 	defer tx.Finish(&err)

// 	req := &api.Request{
// 		Query: queryFindByNameAndUser,
// 		Vars: map[string]string{
// 			"$name": name,
// 			"$user": user,
// 		},
// 	}

// 	res, err := tx.Do(ctx, req)
// 	if err != nil {
// 		return
// 	}

// 	type graphqlUniverseTuple struct {
// 		Universes []graphqlUniverse `json:"queryUniverse"`
// 	}

// 	var result graphqlUniverseTuple
// 	if err = json.Unmarshal(res.GetJson(), &result); err != nil {
// 		return
// 	}

// 	if len(result.Universes) != 1 {
// 		return nil, util.ErrNotFound
// 	}

// 	return result.Universes[0].toModel(), nil
// }

// Create persists the provided universe
func (repo *graphqlUniverseRepository) Create(ctx context.Context, universe *Universe) (err error) {
	type AddUniverseInput struct {
		Name        string `json:"name,omitempty"`
		User        string `json:"user,omitempty"`
		Description string `json:"description,omitempty"`
	}

	type UniverseInputPayload struct {
		Id          string `json:"id,omitempty"`
		Name        string `json:"name,omitempty"`
		User        string `json:"user,omitempty"`
		Description string `json:"description,omitempty"`
	}

	var mutation struct {
		AddUniverse struct {
			Universe []UniverseInputPayload
		} `graphql:"addUniverse(input: [$universe])"`
	}

	variables := map[string]interface{}{
		"universe": AddUniverseInput{
			Name:        universe.Name,
			User:        universe.User,
			Description: universe.Description,
		},
	}

	if err = repo.graphql.Mutate(ctx, &mutation, variables); err != nil {
		return
	}

	if len(mutation.AddUniverse.Universe) == 0 {
		return ErrTransactionFailed
	}

	universe.Id = mutation.AddUniverse.Universe[0].Id
	return
}

// Delete removes the given universe from the graph
func (repo *graphqlUniverseRepository) Delete(ctx context.Context, universe *Universe) error {
	var mutation struct {
		DeleteUniverse struct {
			Universe []Universe
			NumUids  graphql.Int
			Msg      graphql.String
		} `graphql:"deleteUniverse(filter: { id: [$id]})"`
	}

	variables := map[string]interface{}{
		"id": universe.Id,
	}

	if err := repo.graphql.Mutate(ctx, &mutation, variables); err != nil {
		return err
	}

	if mutation.DeleteUniverse.NumUids == 0 {
		return fmt.Errorf("%w: %s", ErrTransactionFailed, mutation.DeleteUniverse.Msg)
	}

	universe.Id = ""
	return nil
}
