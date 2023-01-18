#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use agora::project::application::ProjectApplication;
use agora::project::grpc::{GrpcProjectServer, ProjectServer};
use agora::project::repository::SurrealProjectRepository;
use async_once::AsyncOnce;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use std::env;
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tonic::transport::Server;

const ENV_SURREAL_DSN: &str = "SURREAL_DSN";
const ENV_SURREAL_NS: &str = "SURREAL_NS";
const ENV_SURREAL_DB: &str = "SURREAL_DB";
const ENV_SURREAL_USER: &str = "SURREAL_USER";
const ENV_SURREAL_PASS: &str = "SURREAL_PASS";
const ENV_RABBITMQ_FILES_EXCHANGE: &str = "RABBITMQ_FILES_EXCHANGE";
const ENV_RABBITMQ_FILES_QUEUE: &str = "RABBITMQ_FILES_QUEUE";
const ENV_RABBITMQ_DSN: &str = "RABBITMQ_DSN";
const ENV_TOKEN_ISSUER: &str = "TOKEN_ISSUER";

lazy_static! {
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
    static ref RABBITMQ_CONN: AsyncOnce<Channel> = AsyncOnce::new(async {
        let rabbitmq_dsn = env::var(ENV_RABBITMQ_DSN).expect("rabbitmq url must be set");
        let conn = Connection::connect(&rabbitmq_dsn, ConnectionProperties::default())
            .await
            .map(|pool| {
                info!("connection with rabbitmq cluster established");
                pool
            })
            .map_err(|err| format!("establishing connection with {}: {}", rabbitmq_dsn, err))
            .unwrap();

        conn.create_channel()
            .await
            .map_err(|err| format!("creating rabbitmq channel: {}", err))
            .unwrap()
    });
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    if let Err(err) = dotenv::dotenv() {
        warn!("processing dotenv file {}", err);
    }

    start_server().await
}
