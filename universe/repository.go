package universe

import (
	"context"
	"fmt"

	"github.com/alvidir/go-util"
	"github.com/shurcooL/graphql"
)

// graphqlUniverseRepository implements the UniverseRepository interface for Graphql endpoints
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
func (repo *graphqlUniverseRepository) FindByNameAndUser(ctx context.Context, name string, user string) (*Universe, error) {
	var query struct {
		Universe []Universe `graphql:"queryUniverse(filter: {name: { eq: $name }, user: { eq: $user }})"`
	}

	variables := map[string]interface{}{
		"name": graphql.String(name),
		"user": graphql.String(user),
	}

	if err := repo.graphql.Query(ctx, &query, variables); err != nil {
		return nil, err
	}

	if len(query.Universe) == 0 {
		return nil, util.ErrNotFound
	}

	return &query.Universe[0], nil
}

// Create persists the provided universe
func (repo *graphqlUniverseRepository) Create(ctx context.Context, universe *Universe) (err error) {
	var mutation struct {
		AddUniverse struct {
			Universe []Universe
		} `graphql:"addUniverse(input: [$universe])"`
	}

	type AddUniverseInput Universe
	variables := map[string]interface{}{
		"universe": AddUniverseInput(*universe),
	}

	if err = repo.graphql.Mutate(ctx, &mutation, variables); err != nil {
		return
	}

	if len(mutation.AddUniverse.Universe) == 0 {
		return util.ErrUnknownError
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
		return fmt.Errorf("%w: %s", util.ErrUnknownError, mutation.DeleteUniverse.Msg)
	}

	universe.Id = ""
	return nil
}
