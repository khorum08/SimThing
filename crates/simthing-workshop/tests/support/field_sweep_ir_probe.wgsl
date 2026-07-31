// Disposable workshop FIELD-SWEEP-IR-PROBE-0 interpreter. Not a production kernel.

struct IrNode {
    op: u32,
    a: u32,
    b: u32,
    c: u32,
    value: f32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

struct CellRange {
    offset: u32,
    len: u32,
}

struct SweepParams {
    n_cells: u32,
    n_dims: u32,
    out_col: u32,
    dest_cell: u32,
    force_dest_zero: u32,
    map_root: u32,
    fold_root: u32,
    post_root: u32,
    fold_identity: f32,
    fold_seed_from_target: u32,
    fold_seed_col: u32,
    pad0: u32,
}

@group(0) @binding(0) var<storage, read> values_in: array<f32>;
@group(0) @binding(1) var<storage, read_write> values_out: array<f32>;
@group(0) @binding(2) var<storage, read> ranges: array<CellRange>;
@group(0) @binding(3) var<storage, read> neighbors: array<u32>;
@group(0) @binding(4) var<storage, read> map_nodes: array<IrNode>;
@group(0) @binding(5) var<storage, read> fold_nodes: array<IrNode>;
@group(0) @binding(6) var<storage, read> post_nodes: array<IrNode>;
@group(0) @binding(7) var<uniform> params: SweepParams;

const OP_CONST: u32 = 0u;
const OP_TARGET_VALUE: u32 = 1u;
const OP_NEIGHBOR_VALUE: u32 = 2u;
const OP_ACC: u32 = 3u;
const OP_MAPPED: u32 = 4u;
const OP_FOLDED: u32 = 5u;
const OP_ADD: u32 = 6u;
const OP_SUB: u32 = 7u;
const OP_MUL: u32 = 8u;
const OP_MIN: u32 = 9u;
const OP_MAX: u32 = 10u;
const OP_DIV: u32 = 11u;
const OP_CLAMP01: u32 = 12u;
const MAX_NODES: u32 = 32u;

fn eval_map(root: u32, target_base: u32, neighbor_base: u32) -> f32 {
    var scratch: array<f32, MAX_NODES>;
    for (var i: u32 = 0u; i <= root; i = i + 1u) {
        let n = map_nodes[i];
        switch n.op {
            case OP_CONST: { scratch[i] = n.value; }
            case OP_TARGET_VALUE: { scratch[i] = values_in[target_base + n.a]; }
            case OP_NEIGHBOR_VALUE: { scratch[i] = values_in[neighbor_base + n.a]; }
            case OP_ADD: { scratch[i] = scratch[n.a] + scratch[n.b]; }
            case OP_SUB: { scratch[i] = scratch[n.a] - scratch[n.b]; }
            case OP_MUL: { scratch[i] = scratch[n.a] * scratch[n.b]; }
            case OP_MIN: { scratch[i] = min(scratch[n.a], scratch[n.b]); }
            case OP_MAX: { scratch[i] = max(scratch[n.a], scratch[n.b]); }
            case OP_DIV: { scratch[i] = scratch[n.a] / scratch[n.b]; }
            case OP_CLAMP01: { scratch[i] = clamp(scratch[n.a], 0.0, 1.0); }
            default: { scratch[i] = 0.0; }
        }
    }
    return scratch[root];
}

fn eval_fold(root: u32, target_base: u32, neighbor_base: u32, acc: f32, mapped: f32) -> f32 {
    var scratch: array<f32, MAX_NODES>;
    for (var i: u32 = 0u; i <= root; i = i + 1u) {
        let n = fold_nodes[i];
        switch n.op {
            case OP_CONST: { scratch[i] = n.value; }
            case OP_TARGET_VALUE: { scratch[i] = values_in[target_base + n.a]; }
            case OP_NEIGHBOR_VALUE: { scratch[i] = values_in[neighbor_base + n.a]; }
            case OP_ACC: { scratch[i] = acc; }
            case OP_MAPPED: { scratch[i] = mapped; }
            case OP_ADD: { scratch[i] = scratch[n.a] + scratch[n.b]; }
            case OP_SUB: { scratch[i] = scratch[n.a] - scratch[n.b]; }
            case OP_MUL: { scratch[i] = scratch[n.a] * scratch[n.b]; }
            case OP_MIN: { scratch[i] = min(scratch[n.a], scratch[n.b]); }
            case OP_MAX: { scratch[i] = max(scratch[n.a], scratch[n.b]); }
            case OP_DIV: { scratch[i] = scratch[n.a] / scratch[n.b]; }
            case OP_CLAMP01: { scratch[i] = clamp(scratch[n.a], 0.0, 1.0); }
            default: { scratch[i] = 0.0; }
        }
    }
    return scratch[root];
}

fn eval_post(root: u32, target_base: u32, folded: f32) -> f32 {
    var scratch: array<f32, MAX_NODES>;
    for (var i: u32 = 0u; i <= root; i = i + 1u) {
        let n = post_nodes[i];
        switch n.op {
            case OP_CONST: { scratch[i] = n.value; }
            case OP_TARGET_VALUE: { scratch[i] = values_in[target_base + n.a]; }
            case OP_FOLDED: { scratch[i] = folded; }
            case OP_ADD: { scratch[i] = scratch[n.a] + scratch[n.b]; }
            case OP_SUB: { scratch[i] = scratch[n.a] - scratch[n.b]; }
            case OP_MUL: { scratch[i] = scratch[n.a] * scratch[n.b]; }
            case OP_MIN: { scratch[i] = min(scratch[n.a], scratch[n.b]); }
            case OP_MAX: { scratch[i] = max(scratch[n.a], scratch[n.b]); }
            case OP_DIV: { scratch[i] = scratch[n.a] / scratch[n.b]; }
            case OP_CLAMP01: { scratch[i] = clamp(scratch[n.a], 0.0, 1.0); }
            default: { scratch[i] = 0.0; }
        }
    }
    return scratch[root];
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let cell = gid.x;
    if (cell >= params.n_cells) {
        return;
    }

    let nd = params.n_dims;
    let base = cell * nd;

    for (var c: u32 = 0u; c < nd; c = c + 1u) {
        values_out[base + c] = values_in[base + c];
    }

    if (params.force_dest_zero != 0u && cell == params.dest_cell) {
        values_out[base + params.out_col] = 0.0;
        return;
    }

    let range = ranges[cell];
    var acc = params.fold_identity;
    if (params.fold_seed_from_target != 0u) {
        acc = values_in[base + params.fold_seed_col];
    }
    for (var k: u32 = 0u; k < range.len; k = k + 1u) {
        let neighbor = neighbors[range.offset + k];
        let nbase = neighbor * nd;
        let mapped = eval_map(params.map_root, base, nbase);
        acc = eval_fold(params.fold_root, base, nbase, acc, mapped);
    }

    values_out[base + params.out_col] = eval_post(params.post_root, base, acc);
}
