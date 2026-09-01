/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

use axum::Json;
use chrono::Utc;

use crate::models::HealthResponse;

pub(crate) async fn get_health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "catalog-service",
        timestamp: Utc::now().to_rfc3339(),
    })
}
