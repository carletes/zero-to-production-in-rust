use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::{self, get_configuration};

use reqwest;
use tokio;

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
    let config = get_configuration().expect("Cannot read configuration");
    let conn_string = config.database.connection_string();
    let mut conn = PgConnection::connect(&conn_string)
        .await
        .expect("Cannot connect to PostgreSQL database");

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
        .fetch_one(&mut conn)
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
    let config = configuration::get_configuration().expect("Cannot get configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Cannot connect to PostgreSQL");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let base_url = format!("http://127.0.0.1:{}", port);

    let server = zero2prod::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    TestApp {
        base_url,
        db_pool: connection_pool,
    }
}
