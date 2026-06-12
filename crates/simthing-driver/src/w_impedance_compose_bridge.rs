//! BH-2B bridge: map admitted W composition spec to GPU config only.
//!
//! No semantics — numeric column/weight plumbing. Not a home for movement or faction logic.

use simthing_gpu::{WImpedanceComposeConfig, WImpedanceComposeProfile};
use simthing_spec::CompiledWImpedanceCompose;

/// Bridge compiled admission output to GPU operator configuration.
pub fn compiled_w_impedance_compose_to_gpu_config(
    compiled: &CompiledWImpedanceCompose,
) -> WImpedanceComposeConfig {
    WImpedanceComposeConfig {
        width: compiled.width,
        height: compiled.height,
        n_dims: compiled.n_dims,
        base_w_col: compiled.base_w_col,
        choke_a_col: compiled.choke_a_col,
        choke_b_col: compiled.choke_b_col,
        profiles: compiled
            .profiles
            .iter()
            .map(|p| WImpedanceComposeProfile {
                weight_a: p.weight_a,
                weight_b: p.weight_b,
                output_w_col: p.output_w_col,
            })
            .collect(),
    }
}
