use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMilestonesResponse {
    // Add fields based on the actual API response
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMilestoneResponse {
    // Add fields based on the actual API response
    pub milestone: Milestone,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Milestone {
    pub id: String,
    // Add other milestone fields as needed
}