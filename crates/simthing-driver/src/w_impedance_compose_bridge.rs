//! BH-2B bridge: map admitted W composition spec to GPU config only.
//!
//! No semantics — numeric column/weight plumbing. Not a home for movement or faction logic.
//!
//! BH-2C live API: [`composed_w_min_plus_stencil_config`] maps a composed W profile column to
//! PALMA min-plus stencil `w_col` on the same interleaved resident buffer (zero-copy handoff).

use simthing_gpu::{MinPlusStencilConfig, WImpedanceComposeConfig, WImpedanceComposeProfile};
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

/// BH-2C live API: build min-plus stencil config consuming composed W from the same interleaved buffer.
///
/// After `WImpedanceComposeOp::compose_resident_field`, PALMA reads `w_col = profile.output_w_col`
/// from the shared buffer via `MinPlusTraversalInput::GpuInterleavedW`. No CPU readback required.
pub fn composed_w_min_plus_stencil_config(
    compose: &WImpedanceComposeConfig,
    profile_index: usize,
    d_col: u32,
    dest: (u32, u32),
    inf_sentinel: f32,
) -> MinPlusStencilConfig {
    let profile = &compose.profiles[profile_index];
    MinPlusStencilConfig {
        width: compose.width,
        height: compose.height,
        n_dims: compose.n_dims,
        d_col,
        w_col: profile.output_w_col,
        dest_x: dest.0,
        dest_y: dest.1,
        inf_sentinel,
    }
}
