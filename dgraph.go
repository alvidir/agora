package agora

import (
	"context"

	"github.com/dgraph-io/dgo/v210"
	"github.com/dgraph-io/dgo/v210/protos/api"
	"google.golang.org/grpc"
)

type Dgraph struct {
	*grpc.ClientConn
	*dgo.Dgraph
}

type Tx struct {
	context.Context
	*dgo.Txn
}

func Open(uri string) (*Dgraph, error) {
	conn, err := grpc.Dial(uri, grpc.WithInsecure())
	if err != nil {
		return nil, err
	}

	dgraph := dgo.NewDgraphClient(
		api.NewDgraphClient(conn),
	)

	return &Dgraph{conn, dgraph}, nil
}

func (dg *Dgraph) Begin(ctx context.Context) *Tx {
	return &Tx{ctx, dg.NewTxn()}
}

func (tx *Tx) Finish(err *error) {
	if err == nil || *err == nil {
		tx.Commit(tx)
	} else {
		tx.Discard(tx)
	}
}
