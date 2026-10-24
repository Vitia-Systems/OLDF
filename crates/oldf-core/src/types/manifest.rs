use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub format_version: semver::Version,
    pub id: uuid::Uuid,
    pub created: chrono::DateTime<Utc>,
    pub created_by: String, // TODO: implement createdBy struct
    pub datasets: Vec<String>, // TODO: implement dataset struct
}
