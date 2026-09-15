struct CompiledOverlayOp {
    slot: u32,
    col: u32,
    combine_kind: u32,
    order_band: u32,
    value: f32,
    priority: u32,
    _pad0: u32,
    _pad1: u32,
}

struct OverlayCellRange {
    start: u32,
    count: u32,
}

struct OverlayOrderParams {
    n_slots: u32,
    n_cols: u32,
    n_values: u32,
    n_overlays: u32,
    n_compiled_ops: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

const COMBINE_SUM: u32 = 0u;
const COMBINE_PRODUCT: u32 = 1u;
const COMBINE_LAST_BY_PRIORITY: u32 = 2u;

@group(0) @binding(0) var<storage, read> base_values: array<f32>;
@group(0) @binding(1) var<storage, read> compiled_ops: array<CompiledOverlayOp>;
@group(0) @binding(2) var<storage, read_write> output_values: array<f32>;
@group(0) @binding(3) var<uniform> params: OverlayOrderParams;
@group(0) @binding(4) var<storage, read> cell_ranges: array<OverlayCellRange>;

@compute @workgroup_size(64)
fn apply_pivot(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= params.n_values) {
        return;
    }

    var value = base_values[idx];
    let range = cell_ranges[idx];
    let end = range.start + range.count;
    for (var i = range.start; i < end; i = i + 1u) {
        let op = compiled_ops[i];
        if (op.combine_kind == COMBINE_SUM) {
            value = value + op.value;
        } else if (op.combine_kind == COMBINE_PRODUCT) {
            value = value * op.value;
        } else if (op.combine_kind == COMBINE_LAST_BY_PRIORITY) {
            value = op.value;
        }
    }
    output_values[idx] = value;
}
