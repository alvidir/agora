package agora

import (
	"github.com/dgraph-io/dgo/v210"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"google.golang.org/grpc"
)

type CloseFunc func() error

func NewConn(uri string) (*dgo.Dgraph, CloseFunc, error) {
	conn, err := grpc.Dial(uri, grpc.WithInsecure())
	if err != nil {
		return nil, nil, err
	}

	client := dgo.NewDgraphClient(
		api.NewDgraphClient(conn),
	)

	return client, conn.Close, nil
}
