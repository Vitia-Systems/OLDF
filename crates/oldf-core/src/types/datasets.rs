use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Clone, Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    Raw,
    Derived,
    Simulated,
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct GenerationMethod {
    pub tool: String,
    pub version: String, // Not required to follow semver rules
    pub algorithm: String,
    pub parameters: BTreeMap<String, serde_json::Value>,
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub unit: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Dataset {
    pub id: String,
    pub path: camino::Utf8PathBuf,
    pub profile: String,
    pub media_type: String,
    pub role: Role,
    pub sha256: String,
    pub size: u64,

    pub derived_from: Option<Vec<String>>, // Only if derived
    pub method: Option<GenerationMethod>,  // Only if derived or simulated

    pub instrument: Option<String>,
    pub sample: Option<String>,
    pub acquired: Option<chrono::DateTime<Utc>>,
    pub columns: Option<Vec<Column>>, // Only if tabular data and columns can be provided
}
