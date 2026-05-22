use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SpecError {
    #[error("failed to parse RON: {0}")]
    RonParse(String),

    // ── Capability tree validation (PR 1) ────────────────────────────────────
    #[error("duplicate capability tree id `{0}`")]
    DuplicateTreeId(String),

    #[error("duplicate category `{0}` in tree `{1}`")]
    DuplicateCategory(String, String),

    #[error("duplicate entry `{0}` in tree `{1}`")]
    DuplicateEntry(String, String),

    #[error("negative research_cost on entry `{0}`")]
    NegativeResearchCost(String),

    #[error("Threshold activation requires research_cost > 0 on entry `{0}`")]
    ThresholdRequiresPositiveCost(String),

    #[error("validation failed")]
    ValidationFailed,

    // ── Property compilation (PR 2) ──────────────────────────────────────────
    #[error("duplicate property registration `{namespace}::{name}`")]
    DuplicateProperty { namespace: String, name: String },

    #[error("sub-field `{sub_field}` on property `{property}` declares governed_by `{governed_by}` which is not present in the same layout")]
    InvalidGovernedByRole {
        property:    String,
        sub_field:   String,
        governed_by: String,
    },

    // ── Overlay compilation (PR 2) ───────────────────────────────────────────
    #[error("overlay `{overlay}` targets unknown property `{namespace}::{name}`")]
    UnknownProperty {
        overlay:   String,
        namespace: String,
        name:      String,
    },

    #[error("overlay `{overlay}` references sub-field role `{role}` not present in property `{property}`'s layout")]
    InvalidSubFieldRole {
        overlay:  String,
        property: String,
        role:     String,
    },

    #[error("overlay `{overlay}` has malformed targets_property `{targets_property}` (expected `namespace::name`)")]
    InvalidPropertyReference {
        overlay:          String,
        targets_property: String,
    },
}
