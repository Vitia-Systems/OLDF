use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// How a dataset came to exist: measured, computed or simulated.
///
/// This is the most important distinction in the format. It lets anyone who
/// opens a file tell what was observed from what was inferred, and it decides
/// which other fields a [`Dataset`] must carry.
///
/// | Role | Meaning | Must also have |
/// |---|---|---|
/// | [`Raw`](Role::Raw) | Measured, or taken from the original source | nothing extra |
/// | [`Derived`](Role::Derived) | Computed from other datasets in the file | `derived_from` and `method` |
/// | [`Simulated`](Role::Simulated) | Produced by a model, not measured | `method` |
///
/// In JSON the role is written in lowercase: `"raw"`, `"derived"` or
/// `"simulated"`.
///
/// The default is [`Raw`](Role::Raw). That is only a convenience for building
/// values in code: a file must always state its role explicitly.
///
/// # Examples
///
/// ```
/// use oldf_core::Role;
///
/// assert_eq!(serde_json::to_string(&Role::Derived).unwrap(), r#""derived""#);
///
/// let role: Role = serde_json::from_str(r#""simulated""#).unwrap();
/// assert_eq!(role, Role::Simulated);
/// ```
#[derive(Default, Clone, Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Data exactly as it came from the instrument or the original source.
    ///
    /// Raw data is never modified. A file keeps its raw data so that
    /// everything computed from it can be recomputed later.
    #[default]
    Raw,

    /// Data computed from other datasets in the same file.
    ///
    /// It must say which datasets it comes from
    /// ([`Dataset::derived_from`]) and how it was computed
    /// ([`Dataset::method`]). Derived data is an optimization, not a source of
    /// truth: a reader that cannot reproduce or trust the method should
    /// recompute it from the raw data.
    Derived,

    /// Data produced by a model rather than measured.
    ///
    /// It must say how it was produced ([`Dataset::method`]), so that a
    /// simulation can never be mistaken for a measurement.
    Simulated,
}

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
/// use oldf_core::GenerationMethod;
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

/// Description of one column of a tabular dataset, such as a CSV file.
///
/// # Examples
///
/// ```
/// use oldf_core::Column;
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

/// One data file stored in an OLDF archive, with everything needed to
/// understand and verify it.
///
/// Each entry of the manifest's list of datasets is described by one
/// `Dataset`. It says where the file is ([`path`](Self::path)), what it
/// contains ([`profile`](Self::profile) and [`media_type`](Self::media_type)),
/// how it came to exist ([`role`](Self::role)) and how to check that it has not
/// been damaged ([`sha256`](Self::sha256) and [`size`](Self::size)).
///
/// Optional fields that have no value are left out when the dataset is
/// written, rather than written as `null`.
///
/// # What reading this type checks
///
/// Reading a `Dataset` with serde only checks that each field has the right
/// type (a known role, a valid date, a number for the size). The rules that
/// relate fields to each other or to the archive are checked by the validation
/// layer:
///
/// - ids are unique within the manifest;
/// - `path` is relative, uses `/`, has no `..`, and exists in the archive;
/// - `sha256` and `size` match the stored bytes;
/// - `derived_from` is present and not empty when the role is
///   [`Role::Derived`], and every id in it points to an existing dataset;
/// - `method` is present when the role is [`Role::Derived`] or
///   [`Role::Simulated`];
/// - `instrument` and `sample`, when present, point to entries that exist.
///
/// # Examples
///
/// A raw dataset, with only the required fields:
///
/// ```
/// use oldf_core::{Dataset, Role};
///
/// let json = r#"{
///     "id": "d-1",
///     "path": "data/water.pdb",
///     "profile": "structure.pdb",
///     "media_type": "chemical/x-pdb",
///     "role": "raw",
///     "sha256": "4e9dcccee6e45be03a5867ff16fecaebc7d3a94b63c52bb83f325877c3e40626",
///     "size": 273
/// }"#;
///
/// let dataset: Dataset = serde_json::from_str(json).unwrap();
/// assert_eq!(dataset.role, Role::Raw);
/// assert_eq!(dataset.size, 273);
/// assert!(dataset.method.is_none());
/// ```
///
/// A derived table, which says where it comes from and how it was computed:
///
/// ```
/// use oldf_core::{Dataset, Role};
///
/// let json = r#"{
///     "id": "d-2",
///     "path": "data/water_bonds.csv",
///     "profile": "structure.bonds",
///     "media_type": "text/csv",
///     "role": "derived",
///     "sha256": "fc10146d6f7adeeff77f038eb91b052e32c08c02a4f2ebb1c01f91c1f6464f88",
///     "size": 57,
///     "derived_from": ["d-1"],
///     "method": { "tool": "example-tool", "version": "0.0.1", "algorithm": "distance" },
///     "columns": [ { "name": "atom_a", "type": "int32" } ]
/// }"#;
///
/// let dataset: Dataset = serde_json::from_str(json).unwrap();
/// assert_eq!(dataset.role, Role::Derived);
/// assert_eq!(dataset.derived_from, Some(vec!["d-1".to_string()]));
/// assert_eq!(dataset.columns.as_ref().unwrap().len(), 1);
/// ```
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Dataset {
    /// Identifier of this dataset, such as `"d-1"`.
    ///
    /// It is a label that must be unique within the manifest, and other parts
    /// of the manifest refer to the dataset by it, for example in
    /// [`derived_from`](Self::derived_from). It should be short, contain no
    /// spaces, and stay the same when the file is rewritten.
    pub id: String,

    /// Where the file is inside the archive.
    ///
    /// The path must be relative, use `/` as the separator, contain no `..`
    /// and not start with `/`. A backslash is not a separator in OLDF paths.
    /// Reading does not enforce any of this: the validation layer does.
    pub path: camino::Utf8PathBuf,

    /// The kind of data in the file, written as `family.kind`, such as
    /// `"structure.pdb"` or `"electrophoresis.image"`.
    ///
    /// In v0.1 this is a label that tells a reader what the dataset is. The
    /// fields each profile requires are not defined yet.
    pub profile: String,

    /// The MIME type of the file, such as `"text/csv"` or `"image/png"`.
    pub media_type: String,

    /// How the dataset came to exist. See [`Role`].
    pub role: Role,

    /// SHA-256 of the file's exact bytes, as 64 lowercase hexadecimal
    /// characters.
    ///
    /// It lets a reader detect damaged or altered files. Reading does not
    /// check the format of this text or whether it matches the file.
    pub sha256: String,

    /// Size of the file in bytes, as stored in the archive.
    pub size: u64,

    /// Ids of the datasets this one was computed from.
    ///
    /// Required, and not empty, when the role is [`Role::Derived`]. Left out
    /// otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<Vec<String>>,

    /// How this dataset was produced.
    ///
    /// Required when the role is [`Role::Derived`] or [`Role::Simulated`].
    /// Left out for raw data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<GenerationMethod>,

    /// Id of the instrument that produced the data, taken from the
    /// manifest's list of instruments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,

    /// Id of the sample the data was taken from, taken from the manifest's
    /// list of samples.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,

    /// When the data was measured, in UTC.
    ///
    /// This is the time of the measurement, not the time the file was written;
    /// that is recorded in the manifest. A date with a time zone offset is
    /// converted to UTC when read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquired: Option<chrono::DateTime<Utc>>,

    /// Description of the columns, in order, for tabular data such as CSV.
    ///
    /// Only given when the file is a table and its columns are known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
}
