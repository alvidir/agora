package universe

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/alvidir/agora"
	"github.com/sirupsen/logrus"
)

type UniverseHandler struct {
	Uri    string
	Logger *logrus.Logger
}

func (handler *UniverseHandler) logger() *logrus.Logger {
	if handler.Logger == nil {
		handler.Logger = logrus.New()
	}

	return handler.Logger
}

func (handler *UniverseHandler) UniverseCreateHandler(w http.ResponseWriter, r *http.Request) {
	data, err := io.ReadAll(r.Body)
	if err != nil {
		handler.logger().Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var payload Universe
	if err = json.Unmarshal(data, &payload); err != nil {
		handler.logger().Warn(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	dgraph, err := agora.Open(handler.Uri)
	if err != nil {
		handler.logger().Error(err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	repo := &DgraphUniverseRepository{dgraph}
	app := &Application{repo}

	universe, err := app.UniverseCreate(r.Context(), payload.Name, payload.User)
	if err != nil {
		handler.logger().Error(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	response, err := json.Marshal(universe)
	if err != nil {
		handler.logger().Error(err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Write(response)
}
