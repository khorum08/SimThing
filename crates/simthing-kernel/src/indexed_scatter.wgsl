// Generic indexed f32 gather-scatter: dst[entry.dst] = src[entry.src].
// Pure data movement over admitted bounded buffers — no semantics, no
// arithmetic. Entry indices are validated host-side before dispatch.

struct ScatterEntry {
    src_index: u32,
    dst_index: u32,
}

struct ScatterParams {
    n_entries: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

@group(0) @binding(0) var<storage, read> src_values: array<f32>;
@group(0) @binding(1) var<storage, read_write> dst_values: array<f32>;
@group(0) @binding(2) var<storage, read> entries: array<ScatterEntry>;
@group(0) @binding(3) var<uniform> params: ScatterParams;

@compute @workgroup_size(64)
fn scatter_indexed(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= params.n_entries) {
        return;
    }
    let entry = entries[gid.x];
    dst_values[entry.dst_index] = src_values[entry.src_index];
}
