use serde::{Deserialize, Serialize};

/// How a dataset came to exist: measured, computed or simulated.
///
/// This is the most important distinction in the format. It lets anyone who
/// opens a file tell what was observed from what was inferred, and it decides
/// which other fields a [`Dataset`] must carry.
///
/// | Role | Meaning | Must also have |
/// |---|---|---|
/// | [`Raw`](Role::Raw) | Measured, or taken from the original source | nothing extra |
/// | [`Derived`](Role::Derived) | Computed from other datasets in the file | `derived_from` and `method` |
/// | [`Simulated`](Role::Simulated) | Produced by a model, not measured | `method` |
///
/// In JSON the role is written in lowercase: `"raw"`, `"derived"` or
/// `"simulated"`.
///
/// The default is [`Raw`](Role::Raw). That is only a convenience for building
/// values in code: a file must always state its role explicitly.
///
/// # Examples
///
/// ```
/// use oldf_core::types::Role;
///
/// assert_eq!(serde_json::to_string(&Role::Derived).unwrap(), r#""derived""#);
///
/// let role: Role = serde_json::from_str(r#""simulated""#).unwrap();
/// assert_eq!(role, Role::Simulated);
/// ```
#[derive(Default, Clone, Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Data exactly as it came from the instrument or the original source.
    ///
    /// Raw data is never modified. A file keeps its raw data so that
    /// everything computed from it can be recomputed later.
    #[default]
    Raw,

    /// Data computed from other datasets in the same file.
    ///
    /// It must say which datasets it comes from
    /// ([`Dataset::derived_from`]) and how it was computed
    /// ([`Dataset::method`]). Derived data is an optimization, not a source of
    /// truth: a reader that cannot reproduce or trust the method should
    /// recompute it from the raw data.
    Derived,

    /// Data produced by a model rather than measured.
    ///
    /// It must say how it was produced ([`Dataset::method`]), so that a
    /// simulation can never be mistaken for a measurement.
    Simulated,
}
