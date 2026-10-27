//! Helpers shared by the integration tests.

use serde_json::{Value, json};

pub const SHA_A: &str = "4e9dcccee6e45be03a5867ff16fecaebc7d3a94b63c52bb83f325877c3e40626";
pub const SHA_B: &str = "fc10146d6f7adeeff77f038eb91b052e32c08c02a4f2ebb1c01f91c1f6464f88";

/// A manifest that is valid in every respect. Tests start from this one and
/// break a single thing at a time.
pub fn manifest_json() -> Value {
    json!({
        "format": "OLDF",
        "format_version": "0.1.0",
        "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
        "created": "2026-10-24T12:00:00Z",
        "created_by": { "software": "example", "version": "0.1.0" },
        "datasets": [raw_json(), derived_json()]
    })
}

/// A raw dataset with only the required fields.
pub fn raw_json() -> Value {
    json!({
        "id": "d-1",
        "path": "data/water.pdb",
        "profile": "structure.pdb",
        "media_type": "chemical/x-pdb",
        "role": "raw",
        "sha256": SHA_A,
        "size": 273
    })
}

/// A valid generator data value.
pub fn generator_json() -> Value {
    json!({
        "software": "example-tool",
        "version": "0.0.1"
    })
}

/// A derived dataset with every optional field present.
pub fn derived_json() -> Value {
    json!({
        "id": "d-2",
        "path": "data/water_bonds.csv",
        "profile": "structure.bonds",
        "media_type": "text/csv",
        "role": "derived",
        "sha256": SHA_B,
        "size": 57,
        "derived_from": ["d-1"],
        "method": {
            "tool": "example-tool",
            "version": "0.0.1",
            "algorithm": "distance",
            "parameters": { "tolerance_angstrom": 0.4 }
        },
        "instrument": "inst-1",
        "sample": "s-1",
        "acquired": "2026-10-24T11:00:00Z",
        "columns": [
            { "name": "atom_a", "type": "int32" },
            { "name": "size_bp", "type": "int32", "unit": "bp" }
        ]
    })
}

/// `base` with one field replaced.
pub fn with(mut base: Value, field: &str, new_value: Value) -> Value {
    base[field] = new_value;
    base
}

/// `base` without one field.
pub fn without(mut base: Value, field: &str) -> Value {
    base.as_object_mut().unwrap().remove(field);
    base
}
