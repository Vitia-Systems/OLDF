//! Tests for `Dataset`.

use crate::common::{SHA_A, derived_json, raw_json, with, without};
use chrono::{TimeZone, Utc};
use oldf_core::types::{Dataset, Role};
use serde_json::{Value, json};

fn parse(value: Value) -> Result<Dataset, serde_json::Error> {
    serde_json::from_value(value)
}

#[test]
fn reads_a_raw_dataset_with_only_required_fields() {
    let dataset = parse(raw_json()).expect("a valid dataset must parse");

    assert_eq!(dataset.id, "d-1");
    assert_eq!(dataset.path.as_str(), "data/water.pdb");
    assert_eq!(dataset.profile, "structure.pdb");
    assert_eq!(dataset.media_type, "chemical/x-pdb");
    assert_eq!(dataset.role, Role::Raw);
    assert_eq!(dataset.sha256, SHA_A);
    assert_eq!(dataset.size, 273);

    assert!(dataset.derived_from.is_none());
    assert!(dataset.method.is_none());
    assert!(dataset.instrument.is_none());
    assert!(dataset.sample.is_none());
    assert!(dataset.acquired.is_none());
    assert!(dataset.columns.is_none());
}

#[test]
fn reads_a_derived_dataset_with_every_field() {
    let dataset = parse(derived_json()).unwrap();

    assert_eq!(dataset.role, Role::Derived);
    assert_eq!(dataset.derived_from, Some(vec!["d-1".to_string()]));
    assert_eq!(dataset.instrument.as_deref(), Some("inst-1"));
    assert_eq!(dataset.sample.as_deref(), Some("s-1"));
    assert_eq!(
        dataset.acquired,
        Some(Utc.with_ymd_and_hms(2026, 10, 24, 11, 0, 0).unwrap())
    );

    let method = dataset.method.as_ref().unwrap();
    assert_eq!(method.tool, "example-tool");
    assert_eq!(method.algorithm, "distance");
    assert_eq!(method.parameters["tolerance_angstrom"], 0.4);

    let columns = dataset.columns.as_ref().unwrap();
    assert_eq!(columns.len(), 2);
    assert!(columns[0].unit.is_none());
    assert_eq!(columns[1].unit.as_deref(), Some("bp"));
}

#[test]
fn converts_acquired_with_an_offset_to_utc() {
    let dataset = parse(with(
        raw_json(),
        "acquired",
        json!("2026-10-24T13:00:00+02:00"),
    ))
    .unwrap();

    assert_eq!(
        dataset.acquired,
        Some(Utc.with_ymd_and_hms(2026, 10, 24, 11, 0, 0).unwrap())
    );
}

#[test]
fn accepts_sizes_above_4_gib() {
    let dataset = parse(with(raw_json(), "size", json!(5_000_000_000_u64))).unwrap();
    assert_eq!(dataset.size, 5_000_000_000);
}

#[test]
fn keeps_the_path_text_as_it_is() {
    let dataset = parse(with(raw_json(), "path", json!("data/a b/ñ.csv"))).unwrap();
    assert_eq!(dataset.path.as_str(), "data/a b/ñ.csv");
}

#[test]
fn reads_null_as_absent_for_optional_fields() {
    for field in [
        "derived_from",
        "method",
        "instrument",
        "sample",
        "acquired",
        "columns",
    ] {
        let dataset = parse(with(raw_json(), field, json!(null)))
            .unwrap_or_else(|e| panic!("`{field}: null` should be accepted: {e}"));

        let written = serde_json::to_value(&dataset).unwrap();
        assert!(
            written.get(field).is_none(),
            "`{field}` should read as absent"
        );
    }
}

#[test]
fn ignores_unknown_fields() {
    let value = with(
        raw_json(),
        "field_from_the_future",
        json!({ "anything": [1, 2, 3] }),
    );
    assert!(parse(value).is_ok());
}

#[test]
fn leaves_out_optional_fields_that_have_no_value() {
    let written = serde_json::to_value(parse(raw_json()).unwrap()).unwrap();
    let object = written.as_object().unwrap();

    assert_eq!(
        object.len(),
        7,
        "only the required fields should be written"
    );
    for field in [
        "derived_from",
        "method",
        "instrument",
        "sample",
        "acquired",
        "columns",
    ] {
        assert!(
            object.get(field).is_none(),
            "`{field}` should not be written"
        );
    }
}

#[test]
fn raw_round_trips_to_the_same_json() {
    let written = serde_json::to_value(parse(raw_json()).unwrap()).unwrap();
    assert_eq!(written, raw_json());
}

#[test]
fn derived_round_trips_to_the_same_json() {
    let written = serde_json::to_value(parse(derived_json()).unwrap()).unwrap();
    assert_eq!(written, derived_json());
}

#[test]
fn rejects_a_missing_required_field() {
    for field in [
        "id",
        "path",
        "profile",
        "media_type",
        "role",
        "sha256",
        "size",
    ] {
        assert!(
            parse(without(raw_json(), field)).is_err(),
            "a dataset without `{field}` should fail"
        );
    }
}

#[test]
fn rejects_wrong_types() {
    let cases = [
        ("size", json!("273")),
        ("size", json!(-1)),
        ("size", json!(1.5)),
        ("role", json!("unknown")),
        ("role", json!(1)),
        ("id", json!(5)),
        ("path", json!(5)),
        ("profile", json!(null)),
        ("sha256", json!(null)),
        ("derived_from", json!("d-1")),
        ("method", json!("x")),
        ("columns", json!("a")),
        ("columns", json!([{ "name": "x" }])),
        ("acquired", json!("yesterday")),
        ("acquired", json!("2026-10-24T11:00:00")),
    ];

    for (field, bad) in cases {
        assert!(
            parse(with(derived_json(), field, bad.clone())).is_err(),
            "`{field}` = {bad} should fail"
        );
    }
}

#[test]
fn rejects_duplicate_keys() {
    let text = r#"{
        "id": "d-1",
        "id": "d-1",
        "path": "data/water.pdb",
        "profile": "structure.pdb",
        "media_type": "chemical/x-pdb",
        "role": "raw",
        "sha256": "4e9dcccee6e45be03a5867ff16fecaebc7d3a94b63c52bb83f325877c3e40626",
        "size": 273
    }"#;

    assert!(serde_json::from_str::<Dataset>(text).is_err());
}

//

#[test]
fn parsing_leaves_cross_field_rules_to_validation() {
    assert!(parse(with(raw_json(), "role", json!("derived"))).is_ok());

    let method = json!({ "tool": "t", "version": "1", "algorithm": "a" });
    assert!(parse(with(raw_json(), "method", method)).is_ok());

    assert!(parse(with(derived_json(), "derived_from", json!([]))).is_ok());
}

#[test]
fn parsing_leaves_path_and_hash_rules_to_validation() {
    for path in ["/etc/passwd", "../outside.txt", "data\\file.csv", ""] {
        assert!(
            parse(with(raw_json(), "path", json!(path))).is_ok(),
            "{path:?}"
        );
    }

    for sha in ["", "not-a-hash", "ABC", "4E9D"] {
        assert!(
            parse(with(raw_json(), "sha256", json!(sha))).is_ok(),
            "{sha:?}"
        );
    }

    assert!(parse(with(raw_json(), "id", json!(""))).is_ok());
}
