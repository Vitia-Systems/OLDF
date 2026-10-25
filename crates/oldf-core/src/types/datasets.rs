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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, serde_json::Value>,
}

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<Vec<String>>, // Only if derived

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<GenerationMethod>, // Only if derived or simulated

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquired: Option<chrono::DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>, // Only if tabular data and columns can be provided
}
