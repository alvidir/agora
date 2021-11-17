package agora

import (
	"encoding/json"
	"net/http"
)

// HttpError represents an error that will be provided as an http response
type HttpError struct {
	Code    string `json:"error"`
	Message string `json:"message"`
	status  int
}

func NewHttpError(status int, code, msg string) *HttpError {
	return &HttpError{code, msg, status}
}

func (httperr *HttpError) Send(w http.ResponseWriter) (err error) {
	response, err := json.Marshal(httperr)
	if err != nil {
		return
	}

	w.WriteHeader(httperr.status)
	_, err = w.Write(response)
	return
}
