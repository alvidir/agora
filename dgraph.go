package agora

import (
	"github.com/dgraph-io/dgo/v210"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"google.golang.org/grpc"
)

// CloseFunc closes the connection, it's call should be deferred asap
type CloseFunc func() error

type DGraphClient interface {
	Connect() (*dgo.Dgraph, CloseFunc, error)
}

type dgraphClient struct {
	uri string
}

func NewDgraphClient(uri string) DGraphClient {
	return &dgraphClient{
		uri: uri,
	}
}

func (dg *dgraphClient) Connect() (*dgo.Dgraph, CloseFunc, error) {
	client, err := grpc.Dial(dg.uri, grpc.WithInsecure())
	if err != nil {
		return nil, nil, err
	}

	dclient := dgo.NewDgraphClient(
		api.NewDgraphClient(client),
	)

	return dclient, client.Close, nil
}
