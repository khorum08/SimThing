//! BH-2S: generic GPU stress field algebra over resident choke columns.
//!
//! Scenario-track addendum under CT-4b. Numeric only — no border, culture, or movement semantics.

/// Maximum stress profiles per compose operator (binding budget).
pub const STRESS_COMPOSE_MAX_PROFILES: usize = 8;

/// Maximum distinct input field columns referenced by one compose operator.
pub const STRESS_COMPOSE_MAX_INPUT_FIELDS: usize = 4;

/// GPU stress operator: overlap = choke_a * choke_b.
pub const STRESS_OP_OVERLAP: u32 = 0;
/// GPU stress operator: mismatch = abs(choke_a - choke_b).
pub const STRESS_OP_MISMATCH: u32 = 1;
/// GPU stress operator: weighted = weight_a * choke_a + weight_b * choke_b.
pub const STRESS_OP_WEIGHTED: u32 = 2;
/// GPU stress operator: velocity = abs(choke_now - choke_prev).
pub const STRESS_OP_VELOCITY: u32 = 3;

/// Semantic-free stress operator variant.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StressOperatorSpec {
    Overlap,
    Mismatch,
    Weighted {
        weight_a: f32,
        weight_b: f32,
    },
    Velocity {
        choke_now_col: u32,
        choke_prev_col: u32,
    },
}

/// One admitted stress profile writing a single output column.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StressComposeProfileSpec {
    pub operator: StressOperatorSpec,
    pub output_col: u32,
}

/// Authored stress composition operator (column indices + profile table only).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StressComposeSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<StressComposeProfileSpec>,
}
