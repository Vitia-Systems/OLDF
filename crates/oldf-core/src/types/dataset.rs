use crate::types::{Column, GenerationMethod, Role};
use chrono::Utc;
use serde::{Deserialize, Serialize};

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
/// use oldf_core::types::{Dataset, Role};
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
/// use oldf_core::types::{Dataset, Role};
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
