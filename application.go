package agora

import (
	"context"
	"errors"

	"github.com/alvidir/go-util"
)

// A UniverseRepository represents the persistency gateway for any Universe
type UniverseRepository interface {
	Find(context.Context, string) (*Universe, error)
	FindByNameAndUser(context.Context, string, string) (*Universe, error)
	Insert(context.Context, *Universe) error
	Delete(context.Context, *Universe) error
}

// UniverseApplication implements all available transactions for any Universe
type UniverseApplication struct {
	repo UniverseRepository
}

// TxCreateUniverse creates a new universe for the given name an user
func (app *UniverseApplication) TxCreateUniverse(ctx context.Context, name, user, description string) (*Universe, error) {
	if _, err := app.repo.FindByNameAndUser(ctx, name, user); !errors.Is(err, util.ErrNotFound) {
		return nil, util.ErrAlreadyExists
	}

	universe := &Universe{
		Name:        name,
		User:        user,
		Description: description,
	}

	return universe, app.repo.Insert(ctx, universe)
}

// MomentApplication implements all available transactions for any Moment
type MomentApplication struct {
	repo MomentRepository
}

func (app *MomentApplication) TxCreateMoment(ctx context.Context, universe, date, before, after string) (*Moment, error) {
	return nil, errors.New("not implemented")
}
