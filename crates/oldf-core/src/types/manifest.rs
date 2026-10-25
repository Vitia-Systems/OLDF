use crate::types::dataset::Dataset;
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
    pub datasets: Vec<Dataset>, // TODO: implement dataset struct
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::{Value, json};

    fn valid() -> Value {
        json!({
            "format": "OLDF",
            "format_version": "0.1.0",
            "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
            "created": "2026-10-24T12:00:00Z",
            "created_by": "example",
            "datasets": ["d-1", "d-2"]
        })
    }

    fn parse(value: Value) -> Result<Manifest, serde_json::Error> {
        serde_json::from_value(value)
    }

    fn with(field: &str, new_value: Value) -> Value {
        let mut value = valid();
        value[field] = new_value;
        value
    }

    #[test]
    fn reads_a_valid_manifest() {
        let manifest = parse(valid()).expect("a valid manifest must parse");

        assert_eq!(manifest.format, "OLDF");
        assert_eq!(manifest.format_version, semver::Version::new(0, 1, 0));
        assert_eq!(
            manifest.id.to_string(),
            "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf"
        );
        assert_eq!(
            manifest.created,
            Utc.with_ymd_and_hms(2026, 10, 24, 12, 0, 0).unwrap()
        );
        assert_eq!(manifest.created_by, "example");
        // assert_eq!(manifest.datasets, vec!["d-1", "d-2"]);
    }

    #[test]
    fn reads_a_manifest_from_text() {
        let text = r#"{
            "format": "OLDF",
            "format_version": "0.1.0",
            "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
            "created": "2026-10-24T12:00:00Z",
            "created_by": "example",
            "datasets": []
        }"#;

        let manifest: Manifest = serde_json::from_str(text).expect("valid text must parse");
        assert!(manifest.datasets.is_empty());
    }

    #[test]
    fn accepts_an_empty_dataset_list() {
        let manifest = parse(with("datasets", json!([]))).unwrap();
        assert!(manifest.datasets.is_empty());
    }

    #[test]
    fn converts_dates_with_an_offset_to_utc() {
        let manifest = parse(with("created", json!("2026-10-24T14:00:00+02:00"))).unwrap();
        assert_eq!(
            manifest.created,
            Utc.with_ymd_and_hms(2026, 10, 24, 12, 0, 0).unwrap()
        );
    }

    #[test]
    fn writes_back_the_same_json() {
        let manifest = parse(valid()).unwrap();
        let written = serde_json::to_value(&manifest).unwrap();
        assert_eq!(written, valid());
    }

    #[test]
    fn keeps_the_order_of_the_datasets() {
        let original_manifest = with("datasets", json!(["c", "a", "b"]));
        let parsed_manifest = parse(original_manifest.clone()).unwrap();

        // assert_eq!(parsed_manifest.datasets, vec!["c", "a", "b"]);
        assert_eq!(
            serde_json::to_value(&parsed_manifest).unwrap()["datasets"],
            original_manifest["datasets"]
        );
    }

    #[test]
    fn rejects_an_invalid_id() {
        for bad in [
            json!(""),
            json!("not-a-uuid"),
            json!("968f89fd-aa83-52d0-8eb1"),
            json!(42),
        ] {
            assert!(
                parse(with("id", bad.clone())).is_err(),
                "id {bad} should fail"
            );
        }
    }

    #[test]
    fn rejects_an_invalid_version() {
        for bad in [
            json!(""),
            json!("0.1"),
            json!("v0.1.0"),
            json!("latest"),
            json!("0.1.0.0"),
            json!(1),
        ] {
            assert!(
                parse(with("format_version", bad.clone())).is_err(),
                "format_version {bad} should fail"
            );
        }
    }

    #[test]
    fn rejects_an_invalid_date() {
        for bad in [
            json!(""),
            json!("yesterday"),
            json!("2026-10-24"),
            json!("2026-10-24T12:00:00"), // no timezone
            json!("2026-13-45T12:00:00Z"),
        ] {
            assert!(
                parse(with("created", bad.clone())).is_err(),
                "created {bad} should fail"
            );
        }
    }

    #[test]
    fn rejects_wrong_types() {
        assert!(parse(with("format", json!(1))).is_err());
        assert!(parse(with("created_by", json!(5))).is_err());
        assert!(parse(with("datasets", json!("d-1"))).is_err());
        assert!(parse(with("datasets", json!({}))).is_err());
    }

    #[test]
    fn rejects_a_missing_required_field() {
        for field in [
            "format",
            "format_version",
            "id",
            "created",
            "created_by",
            "datasets",
        ] {
            let mut value = valid();
            value.as_object_mut().unwrap().remove(field);
            assert!(
                parse(value).is_err(),
                "a manifest without `{field}` should fail"
            );
        }
    }

    #[test]
    fn rejects_text_that_is_not_a_manifest() {
        for bad in ["", "{ not json", "[]", "null", "42", "\"oldf\""] {
            assert!(
                serde_json::from_str::<Manifest>(bad).is_err(),
                "{bad:?} should fail"
            );
        }
    }

    #[test]
    fn rejects_duplicate_keys() {
        let text = r#"{
            "format": "OLDF",
            "format_version": "0.1.0",
            "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
            "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
            "created": "2026-10-24T12:00:00Z",
            "created_by": "example",
            "datasets": []
        }"#;

        assert!(serde_json::from_str::<Manifest>(text).is_err());
    }

    #[test]
    fn ignores_unknown_fields() {
        let mut value = valid();
        value["field_from_the_future"] = json!({ "anything": [1, 2, 3] });

        assert!(parse(value).is_ok());
    }

    #[test]
    fn parsing_does_not_judge_the_format_value() {
        let m = parse(with("format", json!("OLDF-X"))).unwrap();
        assert_eq!(m.format, "OLDF-X");
    }
}
