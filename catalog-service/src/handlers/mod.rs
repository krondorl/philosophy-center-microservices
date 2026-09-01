/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

mod health;
mod philosophers;
mod schools;

use axum::http::StatusCode;

pub(crate) use health::get_health;
pub(crate) use philosophers::{get_philosopher, get_philosophers};
pub(crate) use schools::{get_school, get_schools};

fn internal_error(error: sqlx::Error) -> StatusCode {
    eprintln!("Database error: {error}");

    StatusCode::INTERNAL_SERVER_ERROR
}
