use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorData {
    pub software: String,
    pub version: String,
}
