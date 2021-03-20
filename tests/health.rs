use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};

use reqwest;
use tokio;

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else {""};
        let subscriber = get_subscriber("test".into(), filter.into());
        init_subscriber(subscriber);
    };
}

pub struct TestApp {
    pub base_url: String,
    pub db_pool: PgPool,
}

#[actix_rt::test]
async fn health_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &app.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.base_url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Cannot retrieve subscription");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "Missing email"),
        ("email=ursula_le_guin%40gmail.com", "Missing name"),
        ("", "Missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.base_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 Bad Request for payload: '{}' ({})",
            invalid_body,
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    let mut config = get_configuration().expect("Cannot get configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&config.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let base_url = format!("http://127.0.0.1:{}", port);

    let server =
        zero2prod::startup::run(listener, db_pool.clone()).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    TestApp { base_url, db_pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Cannot connect to PostgreSQL");
    conn.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Cannot create database");
    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Cannot connect to PostgreSQL");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Cannot run database migrations");

    db_pool
}
