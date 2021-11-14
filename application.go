package agora

import (
	"errors"

	"github.com/alvidir/go-util"
)

var (
	ErrNameAlreadyExists = errors.New("provided name already exists")
)

type UniverseRepository interface {
	Find(string) (*Universe, error)
	FindByName(string) (*Universe, error)
	Create(*Universe) error
	Delete(*Universe) error
}

type Application struct {
	repo UniverseRepository
}

func (app *Application) UniverseCreate(name string, user string) (*Universe, error) {
	if _, err := app.repo.FindByName(name); err == nil ||
		!errors.Is(err, util.ErrNotFound) {
		return nil, ErrNameAlreadyExists
	}

	universe := &Universe{
		Name: name,
		User: user,
	}

	return universe, app.repo.Create(universe)
}
