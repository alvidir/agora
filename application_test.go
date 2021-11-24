//go:build all || unitary
// +build all unitary

package agora

import (
	"context"
	"testing"

	"github.com/alvidir/go-util"
)

type mockUniverseRepository struct {
	find              func(context.Context, string) (*Universe, error)
	findByNameAndUser func(context.Context, string, string) (*Universe, error)
	insert            func(context.Context, *Universe) error
	delete            func(context.Context, *Universe) error
}

func (repo *mockUniverseRepository) Find(ctx context.Context, id string) (*Universe, error) {
	return repo.find(ctx, id)
}

func (repo *mockUniverseRepository) FindByNameAndUser(ctx context.Context, name, user string) (*Universe, error) {
	return repo.findByNameAndUser(ctx, name, user)
}

func (repo *mockUniverseRepository) Insert(ctx context.Context, u *Universe) error {
	return repo.insert(ctx, u)
}

func (repo *mockUniverseRepository) Delete(ctx context.Context, u *Universe) error {
	return repo.delete(ctx, u)
}

func TestTxCreateUniverseShouldNotFail(t *testing.T) {
	wantId := "TestTxCreateUniverseShouldNotFail_id"
	wantName := "TestTxCreateUniverseShouldNotFail_name"
	wantUser := "TestTxCreateUniverseShouldNotFail_user"
	wantDescription := "TestTxCreateUniverseShouldNotFail_description"

	app := UniverseApplication{
		repo: &mockUniverseRepository{
			insert: func(ctx context.Context, u *Universe) error {
				u.Id = wantId
				return nil
			},

			findByNameAndUser: func(context.Context, string, string) (*Universe, error) {
				return nil, util.ErrNotFound
			},
		},
	}

	ctx := context.Background()
	if got, err := app.TxCreateUniverse(ctx, wantName, wantUser, wantDescription); err != nil {
		t.Fatal(err)
	} else if got.Id != wantId {
		t.Errorf("Got id = %v, want %v", got.Id, wantId)
	} else if got.Name != wantName {
		t.Errorf("Got name = %s, want %s", got.Name, wantName)
	} else if got.User != wantUser {
		t.Errorf("Got user = %s, want %s", got.User, wantUser)
	} else if got.Description != wantDescription {
		t.Errorf("Got description = %s, want %s", got.Description, wantDescription)
	}
}
