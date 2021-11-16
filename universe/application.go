package universe

import (
	"context"
	"errors"

	"github.com/alvidir/go-util"
)

var (
	ErrNameAlreadyExists = errors.New("provided name already exists")
)

type UniverseRepository interface {
	Find(context.Context, string) (*Universe, error)
	FindByNameAndUser(context.Context, string, string) (*Universe, error)
	Create(context.Context, *Universe) error
	Delete(context.Context, *Universe) error
}

type Application struct {
	repo UniverseRepository
}

func (app *Application) UniverseCreate(ctx context.Context, name string, user string) (*Universe, error) {
	if _, err := app.repo.FindByNameAndUser(ctx, name, user); err == nil ||
		!errors.Is(err, util.ErrNotFound) {
		return nil, ErrNameAlreadyExists
	}

	universe := &Universe{
		Name: name,
		User: user,
	}

	return universe, app.repo.Create(ctx, universe)
}
