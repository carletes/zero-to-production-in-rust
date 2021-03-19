use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Cannot read config file");
    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Cannot connect to PostgreSQL database");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;
    run(listener, db_pool)?.await
}
