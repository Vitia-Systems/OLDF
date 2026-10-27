//! Tests for `Column`.

use oldf_core::types::Column;
use serde_json::json;

use crate::common::without;

#[test]
fn reads_the_json_field_type_as_data_type() {
    let column: Column = serde_json::from_value(json!({
        "name": "size_bp", "type": "int32", "unit": "bp"
    }))
    .unwrap();

    assert_eq!(column.name, "size_bp");
    assert_eq!(column.data_type, "int32");
    assert_eq!(column.unit.as_deref(), Some("bp"));
}

#[test]
fn is_written_with_type_and_not_data_type() {
    let column = Column {
        name: "lane".into(),
        data_type: "int32".into(),
        unit: None,
    };

    let written = serde_json::to_value(&column).unwrap();
    assert_eq!(written, json!({ "name": "lane", "type": "int32" }));
    assert!(written.get("data_type").is_none());
}

#[test]
fn does_not_accept_data_type_as_the_json_name() {
    let result = serde_json::from_value::<Column>(json!({ "name": "x", "data_type": "int32" }));
    assert!(result.is_err());
}

#[test]
fn unit_is_optional() {
    let column: Column =
        serde_json::from_value(json!({ "name": "lane", "type": "int32" })).unwrap();
    assert!(column.unit.is_none());
}

#[test]
fn requires_name_and_type() {
    for field in ["name", "type"] {
        let base = json!({ "name": "lane", "type": "int32" });
        assert!(
            serde_json::from_value::<Column>(without(base, field)).is_err(),
            "a column without `{field}` should fail"
        );
    }
}
