use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// How a derived or simulated dataset was produced.
///
/// It records enough to reproduce the result and to judge it: which tool, in
/// which version, using which algorithm and which settings.
///
/// The settings are optional. A method without parameters is valid, and it is
/// written without a `parameters` entry.
///
/// # Examples
///
/// ```
/// use oldf_core::types::GenerationMethod;
///
/// let json = r#"{
///     "tool": "example-tool",
///     "version": "0.0.1",
///     "algorithm": "distance",
///     "parameters": { "tolerance_angstrom": 0.4 }
/// }"#;
///
/// let method: GenerationMethod = serde_json::from_str(json).unwrap();
/// assert_eq!(method.algorithm, "distance");
/// assert_eq!(method.parameters["tolerance_angstrom"], 0.4);
///
/// // Without parameters, the field can be left out.
/// let simple: GenerationMethod =
///     serde_json::from_str(r#"{ "tool": "t", "version": "1", "algorithm": "a" }"#).unwrap();
/// assert!(simple.parameters.is_empty());
/// assert_eq!(
///     serde_json::to_string(&simple).unwrap(),
///     r#"{"tool":"t","version":"1","algorithm":"a"}"#
/// );
/// ```
#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct GenerationMethod {
    /// Name of the program or library that produced the data.
    pub tool: String,

    /// Version of the tool, as the tool names it.
    ///
    /// It is free text and does not have to follow semantic versioning, since
    /// many tools use dates, build numbers or commit hashes instead.
    pub version: String,

    /// Name of the algorithm or procedure that was applied, such as
    /// `"distance"` or `"dssp"`.
    pub algorithm: String,

    /// The settings used, as names with their values.
    ///
    /// Values can be any JSON value. Keys are kept sorted, so writing the same
    /// parameters always gives the same output. When there are none, the field
    /// is left out of the JSON, and a missing field is read as an empty map.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, serde_json::Value>,
}
