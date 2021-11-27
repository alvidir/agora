package agora

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/alvidir/go-util"
	"github.com/sirupsen/logrus"
)

// Handler represents the core of any agora's handler
type Handler interface {
	UserID(*http.Request) (string, error)
}

// HandlerImplementation is a mock implementation for the Handler iterface
type HandlerImplementation struct {
	UserIdFunc func(*http.Request) (string, error)
}

// UserID is supposed to provide the user ID of the one performing the request
func (handler *HandlerImplementation) UserID(r *http.Request) (string, error) {
	return handler.UserIdFunc(r)
}

// UniverseHandler manages all these requets related with the Universe model
type UniverseHandler struct {
	Handler
	app    *UniverseApplication
	logger *logrus.Logger
}

// NewUniverseHandler builds a UniverseHandler instance
func NewUniverseHandler(handler Handler, repo UniverseRepository, logger *logrus.Logger) *UniverseHandler {
	app := &UniverseApplication{
		repo: repo,
	}

	return &UniverseHandler{
		Handler: handler,
		app:     app,
		logger:  logger,
	}
}

// CreateUniverse manages the creation request for a Universe
func (handler *UniverseHandler) CreateUniverse(w http.ResponseWriter, r *http.Request) {
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

	userId, err := handler.UserID(r)
	if err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusUnauthorized)
		return
	}

	universe, err := handler.app.TxCreateUniverse(r.Context(), payload.Name, userId, payload.Description)
	if cerr := util.CatchError(&err, util.ErrorHandler); cerr != nil {
		if err := cerr.Send(w); err != nil {
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
	Handler
	app    *MomentApplication
	logger *logrus.Logger
}

// NewMomentHandler builds a UniverseHandler instance
func NewMomentHandler(handler Handler, repo MomentRepository, logger *logrus.Logger) *MomentHandler {
	app := &MomentApplication{
		repo: repo,
	}

	return &MomentHandler{
		Handler: handler,
		app:     app,
		logger:  logger,
	}
}

// CreateMoment handles all request for universe creation
func (handler *MomentHandler) CreateMoment(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var payload Moment
	if err = json.Unmarshal(data, &payload); err != nil {
		handler.logger.Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	if len(payload.Universe.Id) == 0 {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var beforeId string
	if payload.Before != nil {
		beforeId = payload.Before.Id
	}

	var afterId string
	if payload.After != nil {
		afterId = payload.After.Id
	}

	moment, txErr := handler.app.TxCreateMoment(r.Context(), payload.Universe.Id, payload.Date, beforeId, afterId)
	if err := util.CatchError(&txErr, util.ErrorHandler); err != nil {
		if err := err.Send(w); err != nil {
			handler.logger.Error(err)
		}

		return
	}

	response, err := json.Marshal(moment)
	if err != nil {
		handler.logger.Error(err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Write(response)
}
