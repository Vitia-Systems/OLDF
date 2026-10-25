//! Tests for `Role`.

use oldf_core::types::Role;
use serde_json::json;

#[test]
fn is_written_and_read_in_lowercase() {
    for (role, text) in [
        (Role::Raw, "raw"),
        (Role::Derived, "derived"),
        (Role::Simulated, "simulated"),
    ] {
        assert_eq!(serde_json::to_value(&role).unwrap(), json!(text));
        assert_eq!(serde_json::from_value::<Role>(json!(text)).unwrap(), role);
    }
}

#[test]
fn rejects_unknown_values() {
    for bad in [
        json!("Raw"),
        json!("RAW"),
        json!("derive"),
        json!(""),
        json!(1),
        json!(null),
    ] {
        assert!(
            serde_json::from_value::<Role>(bad.clone()).is_err(),
            "role {bad} should fail"
        );
    }
}

#[test]
fn defaults_to_raw() {
    assert_eq!(Role::default(), Role::Raw);
}
