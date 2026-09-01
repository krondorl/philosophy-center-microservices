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
use crate::models::{RelatedPhilosopher, RelatedPhilosopherRow, School, SchoolRow};

pub(crate) async fn get_schools(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<School>>, StatusCode> {
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

pub(crate) async fn get_school(
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
