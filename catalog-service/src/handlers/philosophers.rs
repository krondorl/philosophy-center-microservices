/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

use axum::{Json, extract::Path, extract::State, http::StatusCode};
use sqlx::PgPool;

use super::internal_error;
use crate::models::{Philosopher, PhilosopherRow, RelatedSchool, RelatedSchoolRow};

pub(crate) async fn get_philosophers(
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

pub(crate) async fn get_philosopher(
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
