use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, email_client::EmailClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Cannot read config file");
    let db_pool = PgPoolOptions::new()
        .connect_with(config.database.with_db())
        .await
        .expect("Cannot connect to PostgreSQL database");
    let sender = config
        .email_client
        .sender()
        .expect("Cannot read email sender address");
    let email_client = EmailClient::new(config.email_client.base_url, sender);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))?;
    run(listener, db_pool, email_client)?.await
}
