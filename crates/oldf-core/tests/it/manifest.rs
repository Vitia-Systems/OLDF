//! Tests for `Manifest`.

use crate::common::{manifest_json, raw_json, with};
use chrono::{TimeZone, Utc};
use oldf_core::types::Manifest;
use serde_json::{Value, json};

fn parse(value: Value) -> Result<Manifest, serde_json::Error> {
    serde_json::from_value(value)
}

#[test]
fn reads_a_valid_manifest() {
    let manifest = parse(manifest_json()).expect("a valid manifest must parse");

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

    assert_eq!(manifest.created_by.software, "example");
    for (got, expected) in manifest.datasets.iter().zip(vec!["d-1", "d-2"]) {
        assert_eq!(got.id, expected);
    }
}

#[test]
fn reads_a_manifest_from_text() {
    let text = r#"{
            "format": "OLDF",
            "format_version": "0.1.0",
            "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
            "created": "2026-10-24T12:00:00Z",
            "created_by": { "software": "example", "version": "0.1.0" },
            "datasets": []
        }"#;

    let manifest: Manifest = serde_json::from_str(text).expect("valid text must parse");
    assert!(manifest.datasets.is_empty());
}

#[test]
fn accepts_an_empty_dataset_list() {
    let manifest = parse(with(manifest_json(), "datasets", json!([]))).unwrap();
    assert!(manifest.datasets.is_empty());
}

#[test]
fn converts_dates_with_an_offset_to_utc() {
    let manifest = parse(with(
        manifest_json(),
        "created",
        json!("2026-10-24T14:00:00+02:00"),
    ))
    .unwrap();
    assert_eq!(
        manifest.created,
        Utc.with_ymd_and_hms(2026, 10, 24, 12, 0, 0).unwrap()
    );
}

#[test]
fn writes_back_the_same_json() {
    let manifest = parse(manifest_json()).unwrap();
    let written = serde_json::to_value(&manifest).unwrap();
    assert_eq!(written, manifest_json());
}

#[test]
fn keeps_the_order_of_the_datasets() {
    let datasets: Vec<Value> = ["c", "a", "b"]
        .iter()
        .map(|id| {
            let mut dataset = raw_json();
            dataset["id"] = json!(id);
            dataset["path"] = json!(format!("data/{id}.pdb"));
            dataset
        })
        .collect();

    let original = with(manifest_json(), "datasets", json!(datasets));
    let parsed = parse(original.clone()).unwrap();

    let ids: Vec<&str> = parsed.datasets.iter().map(|d| d.id.as_str()).collect();
    assert_eq!(ids, ["c", "a", "b"]);
    assert_eq!(
        serde_json::to_value(&parsed).unwrap()["datasets"],
        original["datasets"]
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
            parse(with(manifest_json(), "id", bad.clone())).is_err(),
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
            parse(with(manifest_json(), "format_version", bad.clone())).is_err(),
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
            parse(with(manifest_json(), "created", bad.clone())).is_err(),
            "created {bad} should fail"
        );
    }
}

#[test]
fn rejects_wrong_types() {
    assert!(parse(with(manifest_json(), "format", json!(1))).is_err());
    assert!(parse(with(manifest_json(), "created_by", json!(5))).is_err());
    assert!(parse(with(manifest_json(), "datasets", json!("d-1"))).is_err());
    assert!(parse(with(manifest_json(), "datasets", json!({}))).is_err());
}

#[test]
fn rejects_a_missing_required_field() {
    for field in ["format", "format_version", "created_by", "datasets"] {
        let mut value = manifest_json();
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
    let mut value = manifest_json();
    value["field_from_the_future"] = json!({ "anything": [1, 2, 3] });

    assert!(parse(value).is_ok());
}

#[test]
fn parsing_does_not_judge_the_format_value() {
    let m = parse(with(manifest_json(), "format", json!("OLDF-X"))).unwrap();
    assert_eq!(m.format, "OLDF-X");
}
