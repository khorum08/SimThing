//! BH-2B: generic GPU W impedance composition from base W and choke columns.
//!
//! Opened by named consumer `CT-4b_Local_Automata_W_Feedstock`. Numeric only — no faction,
//! border, movement, or pathfinding semantics.

/// Named traversal consumer that opens BH-2.
pub const CT_4B_LOCAL_AUTOMATA_W_FEEDSTOCK: &str = "CT-4b_Local_Automata_W_Feedstock";

/// Maximum profile outputs per compose operator (v1 cap).
pub const W_IMPEDANCE_COMPOSE_MAX_PROFILES: usize = 8;

/// One admitted impedance profile: `output_w = base_w + weight_a * choke_a + weight_b * choke_b`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WImpedanceComposeProfileSpec {
    pub weight_a: f32,
    pub weight_b: f32,
    pub output_w_col: u32,
}

/// Authored W composition operator (semantic-free numeric field columns only).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WImpedanceComposeSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub base_w_col: u32,
    pub choke_a_col: u32,
    pub choke_b_col: u32,
    pub profiles: Vec<WImpedanceComposeProfileSpec>,
}
