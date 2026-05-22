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

    // ── Capability tree builder (PR 3) ───────────────────────────────────────
    #[error("entry `{0}` was authored with ActivationMode::OnPrereqMet — that state is runtime-only")]
    OnPrereqMetAuthoredDefault(String),

    #[error("entry `{entry_id}` in tree `{in_tree}` references unknown prereq category `{category}` (expected `namespace::name`)")]
    UnknownPrereqCategory {
        in_tree:  String,
        entry_id: String,
        category: String,
    },

    #[error("entry `{entry_id}` in tree `{in_tree}` references unknown prereq entry `{prereq_entry_id}` in category `{category}`")]
    UnknownPrereqEntry {
        in_tree:         String,
        entry_id:        String,
        category:        String,
        prereq_entry_id: String,
    },

    #[error("entry `{0}` declares itself as a prereq")]
    SelfReferentialPrereq(String),

    #[error("category `{category}` in tree `{in_tree}` sets max_active = {count}; v0 supports only Unlimited (None) or Limited(1)")]
    UnsupportedMaxActive {
        in_tree:  String,
        category: String,
        count:    usize,
    },

    #[error("entry `{entry_id}` effect #{effect_index} targets property `{targets_property}`: {reason}")]
    InvalidEffectTarget {
        entry_id:         String,
        effect_index:     usize,
        targets_property: String,
        reason:           String,
    },

    // ── Scripted trigger/effect/event compiler (PR 8) ───────────────────────
    #[error("trigger references unknown property `{namespace}::{name}`")]
    InvalidTriggerProperty { namespace: String, name: String },

    #[error("trigger references role `{role}` not present in property `{property}`")]
    InvalidTriggerRole { property: String, role: String },
}
