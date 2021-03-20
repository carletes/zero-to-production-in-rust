use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Cannot read config file");
    let db_pool = PgPoolOptions::new()
        .connect_with(config.database.with_db())
        .await
        .expect("Cannot connect to PostgreSQL database");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))?;
    run(listener, db_pool)?.await
}
