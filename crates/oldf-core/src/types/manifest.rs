use chrono::Utc;
use serde::{Deserialize, Serialize};

/// The root of an OLDF archive: the contents of `manifest.json`.
///
/// The manifest is the index of the whole file. It says which specification
/// (and which version of it) the file follows, identifies the file, records
/// who created it and when, and lists every dataset stored in the archive.
///
/// This type only describes the *shape* of the manifest. Reading one with
/// serde checks that each field has the right type (a valid UUID, a valid
/// version, a valid date), but not the rules that relate fields to each other,
/// such as unique ids or hashes that match the stored files. Those checks
/// belong to the validation layer.
///
/// See the "Manifest" section of the specification for the full definition.
///
/// # Examples
///
/// ```
/// use oldf_core::Manifest;
///
/// let json = r#"{
///     "format": "OLDF",
///     "format_version": "0.1.0",
///     "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
///     "created": "2026-10-24T12:00:00Z",
///     "created_by": "example",
///     "datasets": []
/// }"#;
///
/// let manifest: Manifest = serde_json::from_str(json).unwrap();
/// assert_eq!(manifest.format, "OLDF");
/// assert_eq!(manifest.format_version.major, 0);
/// ```

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// Identifier of the specification this file follows.
    ///
    /// For this specification the value is `"OLDF"`. If a different or derived
    /// specification appears in the future (for example `"OLDF-X/Y"`), files
    /// following it declare it here, so that readers can tell which rules
    /// apply.
    ///
    /// Together with [`format_version`](Self::format_version) it decides
    /// whether a reader can open the file. A reader must reject a value it
    /// does not recognize.
    pub format: String,

    /// Version of the specification named by [`format`](Self::format), as
    /// `MAJOR.MINOR.PATCH`.
    ///
    /// Versions are numbered separately for each specification. A reader must
    /// reject a major version it does not support.
    pub format_version: semver::Version,

    /// Unique identifier of this file, as a UUID.
    ///
    /// It identifies the archive as a whole. Datasets have their own local
    /// ids, which are only unique inside this manifest.
    pub id: uuid::Uuid,

    /// When the file was created, in UTC.
    pub created: chrono::DateTime<Utc>,

    /// The software that wrote the file.
    pub created_by: String, // TODO: implement createdBy struct

    /// Every data file stored in the archive.
    ///
    /// The order is the one in the file and is preserved when writing.
    pub datasets: Vec<String>, // TODO: implement dataset struct
}
