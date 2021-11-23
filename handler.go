package agora

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/alvidir/go-util"
	"github.com/sirupsen/logrus"
)

// UniverseHandler manages all these requets related with the Universe model
type UniverseHandler struct {
	app    *UniverseApplication
	logger *logrus.Logger
}

// NewUniverseHandler builds a UniverseHandler instance
func NewUniverseHandler(repo UniverseRepository, logger *logrus.Logger) *UniverseHandler {
	app := &UniverseApplication{
		repo: repo,
	}

	return &UniverseHandler{
		app:    app,
		logger: logger,
	}
}

// ServeHTTP handles requests about Universe
func (handler *UniverseHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var payload Universe
	if err = json.Unmarshal(data, &payload); err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	universe, err := handler.app.UniverseCreate(r.Context(), payload.Name, payload.User)
	if httperr := util.CatchError(&err, util.HttpErrorHandler); httperr != nil {
		if err := httperr.Send(w); err != nil {
			handler.logger.Error(err)
		}

		return
	}

	response, err := json.Marshal(universe)
	if err != nil {
		handler.logger.Error(err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Write(response)
}

// MomentHandler manages all these requets related with the Universe model
type MomentHandler struct {
	app    *MomentApplication
	logger *logrus.Logger
}

// NewMomentHandler builds a UniverseHandler instance
func NewMomentHandler(repo MomentRepository, logger *logrus.Logger) *MomentHandler {
	app := &MomentApplication{
		repo: repo,
	}

	return &MomentHandler{
		app:    app,
		logger: logger,
	}
}

// ServeHTTP handles all request for universe creation
func (handler *MomentHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var payload Universe
	if err = json.Unmarshal(data, &payload); err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	universe, err := handler.app.MomentCreate(r.Context(), payload.Name, payload.User)
	if httperr := util.CatchError(&err, util.HttpErrorHandler); httperr != nil {
		if err := httperr.Send(w); err != nil {
			handler.logger.Error(err)
		}

		return
	}

	response, err := json.Marshal(universe)
	if err != nil {
		handler.logger.Error(err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Write(response)
}
