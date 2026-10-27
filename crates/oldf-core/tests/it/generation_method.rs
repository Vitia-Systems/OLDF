//! Tests for `GenerationMethod`.

use oldf_core::types::GenerationMethod;
use serde_json::json;

use crate::common::without;

#[test]
fn reads_parameters_of_any_json_type() {
    let method: GenerationMethod = serde_json::from_value(json!({
        "tool": "t",
        "version": "1",
        "algorithm": "a",
        "parameters": {
            "number": 1,
            "text": "x",
            "list": [1, 2],
            "object": { "inner": true },
            "nothing": null
        }
    }))
    .unwrap();

    assert_eq!(method.parameters["number"], 1);
    assert_eq!(method.parameters["text"], "x");
    assert_eq!(method.parameters["list"], json!([1, 2]));
    assert_eq!(method.parameters["object"], json!({ "inner": true }));
    assert!(method.parameters["nothing"].is_null());
}

#[test]
fn without_parameters_reads_as_an_empty_map() {
    let method: GenerationMethod =
        serde_json::from_value(json!({ "tool": "t", "version": "1", "algorithm": "a" })).unwrap();

    assert!(method.parameters.is_empty());
}

#[test]
fn leaves_out_empty_parameters_when_written() {
    let method = GenerationMethod {
        tool: "t".into(),
        version: "1".into(),
        algorithm: "a".into(),
        ..Default::default()
    };

    let written = serde_json::to_value(&method).unwrap();
    assert!(written.get("parameters").is_none());
}

#[test]
fn without_parameters_survives_a_round_trip() {
    let method = GenerationMethod {
        tool: "t".into(),
        version: "1".into(),
        algorithm: "a".into(),
        ..Default::default()
    };

    let text = serde_json::to_string(&method).unwrap();
    let back: GenerationMethod = serde_json::from_str(&text).unwrap();

    assert_eq!(back.tool, "t");
    assert!(back.parameters.is_empty());
}

#[test]
fn writes_parameters_in_sorted_order() {
    let mut method = GenerationMethod::default();
    method.parameters.insert("b".into(), json!(2));
    method.parameters.insert("a".into(), json!(1));

    let text = serde_json::to_string(&method).unwrap();
    assert!(text.contains(r#""parameters":{"a":1,"b":2}"#), "got {text}");
}

#[test]
fn version_is_free_text() {
    for version in ["1", "0.0.1", "2026.10.24", "build-77", "a1b2c3d"] {
        let method: GenerationMethod =
            serde_json::from_value(json!({ "tool": "t", "version": version, "algorithm": "a" }))
                .unwrap();
        assert_eq!(method.version, version);
    }
}

#[test]
fn requires_tool_version_and_algorithm() {
    for field in ["tool", "version", "algorithm"] {
        let base = json!({ "tool": "t", "version": "1", "algorithm": "a" });
        assert!(
            serde_json::from_value::<GenerationMethod>(without(base, field)).is_err(),
            "a method without `{field}` should fail"
        );
    }
}
