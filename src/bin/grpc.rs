#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use agora::file::event_bus::RabbitMqFileBus;
use agora::project::application::ProjectApplication;
use agora::project::grpc::{GrpcProjectServer, ProjectServiceServer};
use agora::project::repository::SurrealProjectRepository;
use async_once::AsyncOnce;
use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties, ExchangeKind};
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
const DEFAULT_APP_ID: &str = "agora";

const ENV_SERVICE_PORT: &str = "SERVICE_PORT";
const ENV_SERVICE_NETW: &str = "SERVICE_NETW";
const ENV_UID_HEADER: &str = "UID_HEADER";
const ENV_SURREAL_DSN: &str = "SURREAL_DSN";
const ENV_SURREAL_NS: &str = "SURREAL_NS";
const ENV_SURREAL_DB: &str = "SURREAL_DB";
const ENV_SURREAL_USER: &str = "SURREAL_USER";
const ENV_SURREAL_PASS: &str = "SURREAL_PASS";
const ENV_RABBITMQ_FILES_EXCHANGE: &str = "RABBITMQ_FILES_EXCHANGE";
const ENV_RABBITMQ_FILES_QUEUE: &str = "RABBITMQ_FILES_QUEUE";
const ENV_RABBITMQ_DSN: &str = "RABBITMQ_DSN";
const ENV_EVENT_ISSUER: &str = "EVENT_ISSUER";
const ENV_APP_ID: &str = "APP_ID";

lazy_static! {
    static ref APP_ID: String = env::var(ENV_APP_ID).unwrap_or(DEFAULT_APP_ID.to_string());
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
    static ref RABBITMQ_FILES_EXCHANGE: String =
        env::var(ENV_RABBITMQ_FILES_EXCHANGE).expect("rabbitmq files exchange must be set");
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

        let channel = conn
            .create_channel()
            .await
            .map_err(|err| format!("creating rabbitmq channel: {}", err))
            .unwrap();

        let exchange_options = ExchangeDeclareOptions {
            durable: true,
            auto_delete: false,
            internal: false,
            nowait: false,
            passive: false,
        };

        channel
            .exchange_declare(
                &RABBITMQ_FILES_EXCHANGE,
                ExchangeKind::Fanout,
                exchange_options,
                FieldTable::default(),
            )
            .await
            .map_err(|err| {
                format!(
                    "creating rabbitmq exchange {}: {}",
                    &*RABBITMQ_FILES_EXCHANGE, err
                )
            })
            .unwrap();

        channel
    });
    static ref RABBITMQ_FILES_QUEUE: String =
        env::var(ENV_RABBITMQ_FILES_QUEUE).expect("rabbitmq files queue must be set");
    static ref EVENT_ISSUER: String = env::var(ENV_EVENT_ISSUER).expect("event issuer must be set");
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

    let file_event_bus: Arc<RabbitMqFileBus> = Arc::new(RabbitMqFileBus {
        channel: RABBITMQ_CONN.get().await,
        app_id: &APP_ID,
        issuer: &EVENT_ISSUER,
        exchange: &RABBITMQ_FILES_EXCHANGE,
    });

    let project_app = ProjectApplication {
        project_repo: project_repo.clone(),
        event_bus: file_event_bus.clone(),
    };

    let project_server = GrpcProjectServer {
        project_app,
        uid_header: &UID_HEADER,
    };

    let addr = SERVER_ADDR.parse().unwrap();
    info!("server listening on {}", addr);
    Server::builder()
        .add_service(ProjectServiceServer::new(project_server))
        .serve(addr)
        .await?;
    Ok(())
}
