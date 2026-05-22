use crate::keys::{CapabilityEntryKey, CategoryKey};
use simthing_core::SubFieldRole;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CapabilityTreeError {
    #[error("duplicate tree id `{0}`")]
    DuplicateTreeId(String),

    #[error("duplicate category `{0}` in tree `{1}`")]
    DuplicateCategory(CategoryKey, String),

    #[error("duplicate entry `{0}` in tree `{1}`")]
    DuplicateEntry(CapabilityEntryKey, String),

    #[error("unknown prereq category `{0}` in entry `{1}`")]
    UnknownPrereqCategory(String, CapabilityEntryKey),

    #[error("unknown prereq entry `{0}` in category `{1}`")]
    UnknownPrereqEntry(String, String),

    #[error("self prereq on entry `{0}`")]
    SelfPrereq(CapabilityEntryKey),

    #[error("negative research_cost on entry `{0}`")]
    NegativeResearchCost(CapabilityEntryKey),

    #[error("Threshold activation requires research_cost > 0 on entry `{0}`")]
    ThresholdRequiresPositiveCost(CapabilityEntryKey),

    #[error("effect references unknown property `{0}`")]
    UnknownEffectProperty(String),

    #[error("effect references unknown sub-field role `{0:?}` on property `{1}`")]
    UnknownEffectSubField(SubFieldRole, String),

    #[error("invalid overlay lifecycle payload for entry `{0}` effect {1}")]
    InvalidLifecycle(CapabilityEntryKey, usize),

    #[error("invalid max_active count {0} on category `{1}`")]
    InvalidMaxActive(usize, CategoryKey),

    #[error("unsupported max_active count {0} (v0 supports unlimited or 1)")]
    UnsupportedMaxActive(usize),

    #[error("unknown owner kind `{0}`")]
    UnknownOwnerKind(String),

    #[error("entry `{0}` not found in definition")]
    UnknownEntry(CapabilityEntryKey),

    #[error("capability tree instance not found for owner `{0:?}`")]
    UnknownOwner(simthing_core::SimThingId),

    #[error("capability tree instance not found for tree `{0:?}`")]
    UnknownTree(simthing_core::SimThingId),

    #[error("failed to parse RON: {0}")]
    RonParse(String),

    #[error("failed to parse property key `{0}` (expected namespace::name)")]
    InvalidPropertyKey(String),
}
