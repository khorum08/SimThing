// PALMA-PATH-9: compact downstream probe over GPU-resident traversal D column.
// Numeric gather + min reduction only — no route objects, no semantic interpretation.

struct TraversalDProbeParams {
    n_dims: u32,
    d_col: u32,
    n_candidates: u32,
    inf_sentinel: f32,
}

@group(0) @binding(0) var<uniform> params: TraversalDProbeParams;
@group(0) @binding(1) var<storage, read> resident_values: array<f32>;
@group(0) @binding(2) var<storage, read> candidate_indices: array<u32>;
@group(0) @binding(3) var<storage, read_write> probe_output: array<f32>;

var<workgroup> shared_d: array<f32, 64>;

@compute @workgroup_size(64, 1, 1)
fn probe_d_candidates(
    @builtin(local_invocation_id) lid: vec3<u32>,
) {
    let n = params.n_candidates;
    let lane = lid.x;

    if lane < n {
        let cell = candidate_indices[lane];
        let base = cell * params.n_dims;
        shared_d[lane] = resident_values[base + params.d_col];
    }
    workgroupBarrier();

    if lane < n {
        probe_output[lane] = shared_d[lane];
    }

    if lane == 0u {
        var best = params.inf_sentinel;
        for (var i = 0u; i < n; i = i + 1u) {
            best = min(best, shared_d[i]);
        }
        probe_output[n] = best;
    }
}
