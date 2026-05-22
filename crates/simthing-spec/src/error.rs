use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SpecError {
    #[error("failed to parse RON: {0}")]
    RonParse(String),

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
}
