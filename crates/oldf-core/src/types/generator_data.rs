use serde::{Deserialize, Serialize};

/// The software that wrote an OLDF file.
///
/// It is stored in the manifest under `created_by`, and it answers a practical
/// question: when a file looks wrong, which program, and which version of it,
/// produced the file?
///
/// Do not confuse it with [`GenerationMethod`](crate::GenerationMethod). A
/// `GeneratorData` describes the program that wrote the **whole archive**,
/// while a `GenerationMethod` describes how **one dataset** inside it was
/// computed.
///
/// Both fields are required.
///
/// # Examples
///
/// ```
/// use oldf_core::types::GeneratorData;
///
/// let json = r#"{ "software": "oldf-cli", "version": "0.1.0" }"#;
///
/// let generator: GeneratorData = serde_json::from_str(json).unwrap();
/// assert_eq!(generator.software, "oldf-cli");
/// assert_eq!(generator.version, "0.1.0");
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorData {
    /// Name of the program or library that wrote the file, such as
    /// `"oldf-cli"`.
    ///
    /// For a file produced by a script, use the name of the script.
    pub software: String,

    /// Version of that software, as the software names it.
    ///
    /// It is free text and does not have to follow semantic versioning, since
    /// many tools use dates, build numbers or commit hashes instead.
    pub version: String,
}
