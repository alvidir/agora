package universe

import (
	"encoding/json"
	"errors"
	"io"
	"net/http"

	"github.com/alvidir/go-util"
	"github.com/shurcooL/graphql"
	"github.com/sirupsen/logrus"
)

const (
	ErrNameAlreadyExistsCode = "U+NAE"
)

type UniverseHandler struct {
	GraphqlUri string
	Logger     *logrus.Logger
}

func (handler *UniverseHandler) logger() *logrus.Logger {
	if handler.Logger == nil {
		handler.Logger = logrus.New()
	}

	return handler.Logger
}

func (handler *UniverseHandler) errorsHandler(err error, httperr *util.HttpError) {
	if errors.Is(err, ErrNameAlreadyExists) {
		httperr.Code = ErrNameAlreadyExistsCode
	}
}

// UniverseCreateHandler handles all request for universe creation
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

	graphql := graphql.NewClient(handler.GraphqlUri, nil)
	repo := &graphqlUniverseRepository{graphql}
	app := &Application{repo}

	universe, err := app.UniverseCreate(r.Context(), payload.Name, payload.User)
	if httperr := util.CatchError(&err, handler.errorsHandler); httperr != nil {
		if err := httperr.Send(w); err != nil {
			handler.logger().Error(err)
		}

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
