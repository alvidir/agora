package agora

import (
	"encoding/json"
	"net/http"
)

// HttpError represents an error that will be provided as an http response
type HttpError struct {
	Code    string `json:"error"`
	Message string `json:"message"`
	Status  int
}

// CatchError takes an error pointer and build an HttpError if, and only if, the pointer is not null and there is an actual error;
// otherwise it returns nil
func CatchError(err *error, callback func(error, *HttpError)) *HttpError {
	if err == nil || *err == nil {
		return nil
	}

	httperr := &HttpError{
		Message: (*err).Error(),
		Status:  http.StatusBadRequest,
	}

	if callback != nil {
		callback(*err, httperr)
	}

	return httperr
}

// Send retrieves the current HttpError as an http response
func (httperr *HttpError) Send(w http.ResponseWriter) (err error) {
	response, err := json.Marshal(httperr)
	if err != nil {
		return
	}

	w.WriteHeader(httperr.Status)
	_, err = w.Write(response)
	return
}
