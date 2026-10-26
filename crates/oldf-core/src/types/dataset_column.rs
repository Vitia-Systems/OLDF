use serde::{Deserialize, Serialize};

/// Description of one column of a tabular dataset, such as a CSV file.
///
/// # Examples
///
/// ```
/// use oldf_core::types::Column;
///
/// let with_unit: Column =
///     serde_json::from_str(r#"{ "name": "size_bp", "type": "int32", "unit": "bp" }"#).unwrap();
/// assert_eq!(with_unit.data_type, "int32");
/// assert_eq!(with_unit.unit.as_deref(), Some("bp"));
///
/// // A column with no unit simply leaves the field out.
/// let without_unit: Column =
///     serde_json::from_str(r#"{ "name": "lane", "type": "int32" }"#).unwrap();
/// assert!(without_unit.unit.is_none());
/// assert_eq!(
///     serde_json::to_string(&without_unit).unwrap(),
///     r#"{"name":"lane","type":"int32"}"#
/// );
/// ```
#[derive(Default, Clone, Deserialize, Serialize, Debug)]
pub struct Column {
    /// Name of the column, as it appears in the file.
    pub name: String,

    /// Type of the values, such as `"int32"`, `"float64"` or `"string"`.
    ///
    /// It is called `type` in the JSON; the Rust field has a different name
    /// because `type` is a reserved word. In v0.1 the set of type names is not
    /// fixed.
    #[serde(rename = "type")]
    pub data_type: String,

    /// Unit of the values, written as a [UCUM](https://ucum.org/) code such as
    /// `"bp"` or `"mL"`.
    ///
    /// Left out for columns that have no unit, such as identifiers or text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
