use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = get_configuration().expect("Cannot read config file");

    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Cannot connect to PostgreSQL database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))?;

    run(listener, db_pool)?.await
}
