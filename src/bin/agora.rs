#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use agora::project::application::ProjectApplication;
use agora::project::grpc::{GrpcProjectServer, ProjectServer};
use agora::project::repository::SurrealProjectRepository;
use async_once::AsyncOnce;
use std::env;
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tonic::transport::Server;

const DEFAULT_NETW: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8000";
const DEFAULT_UID_HEADER: &str = "X-Uid";

const ENV_SERVICE_PORT: &str = "SERVICE_PORT";
const ENV_SERVICE_NETW: &str = "SERVICE_NETW";
const ENV_UID_HEADER: &str = "UID_HEADER";
const ENV_SURREAL_DSN: &str = "SURREAL_DSN";
const ENV_SURREAL_NS: &str = "SURREAL_NS";
const ENV_SURREAL_DB: &str = "SURREAL_DB";
const ENV_SURREAL_USER: &str = "SURREAL_USER";
const ENV_SURREAL_PASS: &str = "SURREAL_PASS";

lazy_static! {
    static ref SERVER_ADDR: String = {
        let netw = env::var(ENV_SERVICE_NETW).unwrap_or_else(|_| DEFAULT_NETW.to_string());
        let port = env::var(ENV_SERVICE_PORT).unwrap_or_else(|_| DEFAULT_PORT.to_string());
        format!("{}:{}", netw, port)
    };
    static ref UID_HEADER: String =
        env::var(ENV_UID_HEADER).unwrap_or_else(|_| DEFAULT_UID_HEADER.to_string());
    static ref SURREAL_CLIENT: AsyncOnce<Surreal<Client>> = AsyncOnce::new(async {
        let surreal_dsn = env::var(ENV_SURREAL_DSN).expect("surreal url must be set");
        let client = Surreal::new::<Ws>(&*surreal_dsn)
            .await
            .map_err(|err| format!("establishing connection with {}: {}", surreal_dsn, err))
            .unwrap();

        let surreal_user = env::var(ENV_SURREAL_USER).expect("surreal user must be set");
        let surreal_pass = env::var(ENV_SURREAL_PASS).expect("surreal password must be set");

        client
            .signin(Root {
                username: &surreal_user,
                password: &surreal_pass,
            })
            .await
            .map_err(|err| format!("signing in surreal service: {}", err))
            .unwrap();

        let surreal_ns = env::var(ENV_SURREAL_NS).expect("surreal namespace must be set");
        let surreal_db = env::var(ENV_SURREAL_DB).expect("surreal database must be set");

        client
            .use_ns(surreal_ns)
            .use_db(surreal_db)
            .await
            .map_err(|err| format!("setting up surreal namespace and database: {}", err))
            .unwrap();

        info!("connection with surreal cluster established");
        client
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    if let Err(err) = dotenv::dotenv() {
        warn!("processing dotenv file {}", err);
    }

    let project_repo = Arc::new(SurrealProjectRepository {
        client: SURREAL_CLIENT.get().await,
    });

    let project_app = ProjectApplication {
        project_repo: project_repo.clone(),
    };

    let project_server = GrpcProjectServer {
        project_app,
        uid_header: &UID_HEADER,
    };

    let addr = SERVER_ADDR.parse().unwrap();
    info!("server listening on {}", addr);
    Server::builder()
        .add_service(ProjectServer::new(project_server))
        .serve(addr)
        .await?;
    Ok(())
}
