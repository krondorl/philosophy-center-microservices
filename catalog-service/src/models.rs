/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize)]
pub(crate) struct HealthResponse {
    pub(crate) status: &'static str,
    pub(crate) service: &'static str,
    pub(crate) timestamp: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct Philosopher {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) schools: Vec<RelatedSchool>,
}

#[derive(Debug, FromRow)]
pub(crate) struct PhilosopherRow {
    pub(crate) philosopher_id: String,
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct School {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) philosophers: Vec<RelatedPhilosopher>,
}

#[derive(Debug, FromRow)]
pub(crate) struct SchoolRow {
    pub(crate) school_id: String,
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct RelatedSchoolRow {
    pub(crate) school_id: String,
    pub(crate) name: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct RelatedPhilosopherRow {
    pub(crate) philosopher_id: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RelatedSchool {
    pub(crate) id: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RelatedPhilosopher {
    pub(crate) id: String,
    pub(crate) name: String,
}
