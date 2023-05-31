#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use agora::file::event_bus::RabbitMqFileBus;
use agora::file::event_handler::FileEventHandler;
use agora::project::application::ProjectApplication;
use agora::project::repository::SurrealProjectRepository;
use agora::rabbitmq::RabbitMqEventBus;
use async_once::AsyncOnce;
use lapin::{Channel, Connection, ConnectionProperties};
use std::env;
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

const DEFAULT_APP_ID: &str = "agora";

const ENV_SURREAL_DSN: &str = "SURREAL_DSN";
const ENV_SURREAL_NS: &str = "SURREAL_NS";
const ENV_SURREAL_DB: &str = "SURREAL_DB";
const ENV_SURREAL_USER: &str = "SURREAL_USER";
const ENV_SURREAL_PASS: &str = "SURREAL_PASS";
const ENV_RABBITMQ_FILES_EXCHANGE: &str = "RABBITMQ_FILES_EXCHANGE";
const ENV_RABBITMQ_FILES_QUEUE: &str = "RABBITMQ_FILES_QUEUE";
const ENV_RABBITMQ_DSN: &str = "RABBITMQ_DSN";
const ENV_EVENT_ISSUER: &str = "EVENT_ISSUER";
const ENV_ISSUERS_WHITELIST: &str = "ISSUERS_WHITELIST";
const ENV_APP_ID: &str = "APP_ID";

lazy_static! {
    static ref APP_ID: String = env::var(ENV_APP_ID).unwrap_or(DEFAULT_APP_ID.to_string());
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
    static ref RABBITMQ_FILES_EXCHANGE: String =
        env::var(ENV_RABBITMQ_FILES_EXCHANGE).expect("rabbitmq files exchange must be set");
    static ref RABBITMQ_FILES_QUEUE: String =
        env::var(ENV_RABBITMQ_FILES_QUEUE).expect("rabbitmq files queue must be set");
    static ref EVENT_ISSUER: String = env::var(ENV_EVENT_ISSUER).expect("event issuer must be set");
    static ref ISSUERS_WHITELIST: Vec<String> = env::var(ENV_ISSUERS_WHITELIST)
        .map(|s| s.split(';').map(Into::into).collect())
        .expect("issuers whitelist must be set");
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
        event_bus: file_event_bus,
    };

    let file_event_handler = FileEventHandler {
        project_app,
        issuers_whitelist: &ISSUERS_WHITELIST,
    };

    let bus = RabbitMqEventBus {
        chann: Arc::new(RABBITMQ_CONN.get().await),
    };

    bus.queue_bind(&RABBITMQ_FILES_EXCHANGE, &RABBITMQ_FILES_QUEUE)
        .await
        .unwrap();

    bus.consume(&RABBITMQ_FILES_QUEUE, file_event_handler)
        .await
        .unwrap();

    Ok(())
}
