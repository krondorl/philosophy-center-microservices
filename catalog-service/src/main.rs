/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

use axum::{Json, Router, extract::Path, extract::State, http::StatusCode, routing::get};
use chrono::Utc;
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{
    FromRow, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::env;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    timestamp: String,
}

#[derive(Debug, FromRow)]
struct PhilosopherRow {
    philosopher_id: String,
    name: String,
    description: String,
}

#[derive(Debug, FromRow)]
struct SchoolRow {
    school_id: String,
    name: String,
    description: String,
}

#[derive(Debug, FromRow)]
struct RelatedSchoolRow {
    school_id: String,
    name: String,
}

#[derive(Debug, FromRow)]
struct RelatedPhilosopherRow {
    philosopher_id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct Philosopher {
    id: String,
    name: String,
    description: String,
    schools: Vec<RelatedSchool>,
}

#[derive(Debug, Serialize)]
struct RelatedSchool {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct School {
    id: String,
    name: String,
    description: String,
    philosophers: Vec<RelatedPhilosopher>,
}

#[derive(Debug, Serialize)]
struct RelatedPhilosopher {
    id: String,
    name: String,
}

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

async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "catalog-service",
        timestamp: Utc::now().to_rfc3339(),
    })
}

async fn get_philosophers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Philosopher>>, StatusCode> {
    let rows = sqlx::query_as::<_, PhilosopherRow>(
        r#"
        SELECT
            philosopher_id,
            name,
            description
        FROM philosophers
        ORDER BY name
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    let mut philosophers = Vec::with_capacity(rows.len());

    for row in rows {
        let schools = get_schools_for_philosopher(&pool, &row.philosopher_id).await?;

        philosophers.push(Philosopher {
            id: row.philosopher_id,
            name: row.name,
            description: row.description,
            schools,
        });
    }

    Ok(Json(philosophers))
}

async fn get_philosopher(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Philosopher>, StatusCode> {
    let row = sqlx::query_as::<_, PhilosopherRow>(
        r#"
        SELECT
            philosopher_id,
            name,
            description
        FROM philosophers
        WHERE philosopher_id = $1
        "#,
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(internal_error)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let schools = get_schools_for_philosopher(&pool, &row.philosopher_id).await?;

    Ok(Json(Philosopher {
        id: row.philosopher_id,
        name: row.name,
        description: row.description,
        schools,
    }))
}

async fn get_schools_for_philosopher(
    pool: &PgPool,
    philosopher_id: &str,
) -> Result<Vec<RelatedSchool>, StatusCode> {
    let rows = sqlx::query_as::<_, RelatedSchoolRow>(
        r#"
        SELECT
            s.school_id,
            s.name
        FROM philosopher_school ps
        JOIN philosophers p
            ON p.id = ps.philosopher_id
        JOIN schools s
            ON s.id = ps.school_id
        WHERE p.philosopher_id = $1
        ORDER BY s.name
        "#,
    )
    .bind(philosopher_id)
    .fetch_all(pool)
    .await
    .map_err(internal_error)?;

    Ok(rows
        .into_iter()
        .map(|row| RelatedSchool {
            id: row.school_id,
            name: row.name,
        })
        .collect())
}

async fn get_schools(State(pool): State<PgPool>) -> Result<Json<Vec<School>>, StatusCode> {
    let rows = sqlx::query_as::<_, SchoolRow>(
        r#"
        SELECT
            school_id,
            name,
            description
        FROM schools
        ORDER BY name
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    let mut schools = Vec::with_capacity(rows.len());

    for row in rows {
        let philosophers = get_philosophers_for_school(&pool, &row.school_id).await?;

        schools.push(School {
            id: row.school_id,
            name: row.name,
            description: row.description,
            philosophers,
        });
    }

    Ok(Json(schools))
}

async fn get_school(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<School>, StatusCode> {
    let row = sqlx::query_as::<_, SchoolRow>(
        r#"
        SELECT
            school_id,
            name,
            description
        FROM schools
        WHERE school_id = $1
        "#,
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(internal_error)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let philosophers = get_philosophers_for_school(&pool, &row.school_id).await?;

    Ok(Json(School {
        id: row.school_id,
        name: row.name,
        description: row.description,
        philosophers,
    }))
}

async fn get_philosophers_for_school(
    pool: &PgPool,
    school_id: &str,
) -> Result<Vec<RelatedPhilosopher>, StatusCode> {
    let rows = sqlx::query_as::<_, RelatedPhilosopherRow>(
        r#"
            SELECT
                p.philosopher_id,
                p.name
            FROM philosopher_school ps
            JOIN schools s
                ON s.id = ps.school_id
            JOIN philosophers p
                ON p.id = ps.philosopher_id
            WHERE s.school_id = $1
            ORDER BY p.name
            "#,
    )
    .bind(school_id)
    .fetch_all(pool)
    .await
    .map_err(internal_error)?;

    Ok(rows
        .into_iter()
        .map(|row| RelatedPhilosopher {
            id: row.philosopher_id,
            name: row.name,
        })
        .collect())
}

fn internal_error(error: sqlx::Error) -> StatusCode {
    eprintln!("Database error: {error}");

    StatusCode::INTERNAL_SERVER_ERROR
}
