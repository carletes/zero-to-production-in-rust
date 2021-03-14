use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Cannot read config file");

    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Cannot connect to PostgreSQL database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;

    run(listener, connection_pool)?.await
}
