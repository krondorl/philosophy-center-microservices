/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

mod handlers;
mod models;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use tokio::net::TcpListener;

use handlers::{get_health, get_philosopher, get_philosophers, get_school, get_schools};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");

    let port: u16 = std::env::var("DATABASE_PORT")
        .expect("DATABASE_PORT missing")
        .parse()
        .expect("DATABASE_PORT must be a valid number");

    let database = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");

    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");

    let options = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .database(&database)
        .username(&user)
        .password(&password);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Could not connect to PostgreSQL");

    let app = Router::new()
        .route("/health", get(get_health))
        .route("/philosophers", get(get_philosophers))
        .route("/philosophers/{id}", get(get_philosopher))
        .route("/schools", get(get_schools))
        .route("/schools/{id}", get(get_school))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3636")
        .await
        .expect("Could not bind server");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Server failed");
}
