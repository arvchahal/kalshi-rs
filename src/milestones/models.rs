//! Milestones module models.
//!
//! This module contains data structures for milestones functionality.

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GetMilestonesResponse {
    pub milestones: Vec<Milestone>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GetMilestoneResponse {
    pub milestone: Milestone,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Milestone {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct MilestonesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
