//! BH-2S bridge: map admitted stress compose spec to GPU config only.

use simthing_gpu::{StressComposeConfig, StressComposeProfile};
use simthing_spec::CompiledStressCompose;

pub fn compiled_stress_compose_to_gpu_config(
    compiled: &CompiledStressCompose,
) -> StressComposeConfig {
    StressComposeConfig {
        width: compiled.width,
        height: compiled.height,
        n_dims: compiled.n_dims,
        choke_a_col: compiled.choke_a_col,
        choke_b_col: compiled.choke_b_col,
        profiles: compiled
            .profiles
            .iter()
            .map(|p| StressComposeProfile {
                operator_kind: p.operator_kind,
                weight_a: p.weight_a,
                weight_b: p.weight_b,
                output_col: p.output_col,
                choke_now_col: p.choke_now_col,
                choke_prev_col: p.choke_prev_col,
            })
            .collect(),
    }
}
