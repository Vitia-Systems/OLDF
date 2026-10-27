//! Tests for `GeneratorData`.

use oldf_core::types::GeneratorData;
use serde_json::{Value, json};

use crate::common::{generator_json, with, without};

fn parse(value: Value) -> Result<GeneratorData, serde_json::Error> {
    serde_json::from_value(value)
}

#[test]
fn reads_software_and_version() {
    let generator = parse(generator_json()).expect("valid generator data must parse");

    assert_eq!(generator.software, "example-tool");
    assert_eq!(generator.version, "0.0.1");
}

#[test]
fn version_is_free_text() {
    for version in ["1", "0.0.1", "2026.10.24", "build-77", "a1b2c3d"] {
        let generator = parse(with(generator_json(), "version", json!(version))).unwrap();
        assert_eq!(generator.version, version);
    }
}

#[test]
fn keeps_non_ascii_text() {
    let value = json!({ "software": "herramienta-ñ", "version": "1.0-β" });

    let generator = parse(value.clone()).unwrap();
    assert_eq!(generator.software, "herramienta-ñ");
    assert_eq!(serde_json::to_value(&generator).unwrap(), value);
}

#[test]
fn ignores_unknown_fields() {
    let value = with(generator_json(), "field_from_the_future", json!([1, 2, 3]));
    assert!(parse(value).is_ok());
}

#[test]
fn writes_exactly_two_fields() {
    let written = serde_json::to_value(parse(generator_json()).unwrap()).unwrap();
    let object = written.as_object().unwrap();

    assert_eq!(object.len(), 2);
    assert!(object.contains_key("software"));
    assert!(object.contains_key("version"));
}

#[test]
fn round_trips_to_the_same_json() {
    let written = serde_json::to_value(parse(generator_json()).unwrap()).unwrap();
    assert_eq!(written, generator_json());
}

#[test]
fn rejects_a_missing_field() {
    for field in ["software", "version"] {
        assert!(
            parse(without(generator_json(), field)).is_err(),
            "generator data without `{field}` should fail"
        );
    }
}

#[test]
fn rejects_wrong_types() {
    let cases = [
        ("software", json!(5)),
        ("software", json!(null)),
        ("software", json!(["a"])),
        ("version", json!(1)),
        ("version", json!(null)),
        ("version", json!({})),
    ];

    for (field, bad) in cases {
        assert!(
            parse(with(generator_json(), field, bad.clone())).is_err(),
            "`{field}` = {bad} should fail"
        );
    }
}

#[test]
fn rejects_values_that_are_not_objects() {
    for bad in [json!("example-tool"), json!(5), json!([]), json!(null)] {
        assert!(parse(bad.clone()).is_err(), "{bad} should fail");
    }
}

#[test]
fn rejects_duplicate_keys() {
    let text = r#"{
        "software": "example-tool",
        "software": "example-tool",
        "version": "0.0.1"
    }"#;

    assert!(serde_json::from_str::<GeneratorData>(text).is_err());
}

#[test]
fn parsing_leaves_content_rules_to_validation() {
    assert!(parse(with(generator_json(), "software", json!(""))).is_ok());
    assert!(parse(with(generator_json(), "version", json!(""))).is_ok());
}
