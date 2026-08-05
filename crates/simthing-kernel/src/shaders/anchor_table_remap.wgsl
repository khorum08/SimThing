//! ANCHOR-TABLE-SURFACE-0 GPU-resident structural remap (orch remand 5120847431).
//!
//! Applies typed move/retire ops to the live GPU table and appends birth seeds.
//! Never requires reading the live table to CPU for mutation.

struct AnchorTableRowGpu {
    sim_thing_id: u32,
    property_id: u32,
    slot: u32,
    col: u32,
    band_idx: i32,
    last_crossing_generation: i32,
    urgency: f32,
    observed_value: f32,
}

struct AnchorRemapOpGpu {
    sim_thing_id: u32,
    property_id: u32,
    kind: u32,
    from_slot: u32,
    from_col: u32,
    to_slot: u32,
    to_col: u32,
    _pad: u32,
}

struct AnchorRemapParams {
    n_src_rows: u32,
    n_ops: u32,
    n_births: u32,
    _pad: u32,
}

const KIND_MOVE: u32 = 0u;
const KIND_RETIRE: u32 = 1u;
// SLOT-LOGICAL-IDENTITY-0 ObjectRow epoch rebind: whole-row move, columns
// untouched; matches by sim_thing_id + from_slot only (no property key).
const KIND_ROW_MOVE: u32 = 2u;

@group(0) @binding(0) var<storage, read> src_table: array<AnchorTableRowGpu>;
@group(0) @binding(1) var<storage, read> ops: array<AnchorRemapOpGpu>;
@group(0) @binding(2) var<storage, read> birth_rows: array<AnchorTableRowGpu>;
@group(0) @binding(3) var<storage, read_write> dest_table: array<AnchorTableRowGpu>;
@group(0) @binding(4) var<storage, read_write> out_count: atomic<u32>;
@group(0) @binding(5) var<uniform> params: AnchorRemapParams;

/// Single-threaded compact apply: preserve dynamic fields on move, drop on retire,
/// then append birth seeds (minted from registry, not live-table readback).
@compute @workgroup_size(1)
fn apply_anchor_remaps(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) {
        return;
    }
    var out_i = 0u;
    let n_src = params.n_src_rows;
    for (var r = 0u; r < n_src; r++) {
        var row = src_table[r];
        var drop_row = false;
        for (var i = 0u; i < params.n_ops; i++) {
            let op = ops[i];
            if (op.sim_thing_id != row.sim_thing_id) {
                continue;
            }
            if (op.kind == KIND_ROW_MOVE) {
                if (row.slot == op.from_slot) {
                    row.slot = op.to_slot;
                }
                continue;
            }
            if (op.property_id != row.property_id) {
                continue;
            }
            if (op.kind == KIND_RETIRE) {
                drop_row = true;
                break;
            }
            if (op.kind == KIND_MOVE) {
                let delta = i32(op.to_col) - i32(op.from_col);
                row.slot = op.to_slot;
                row.col = u32(i32(row.col) + delta);
            }
        }
        if (!drop_row) {
            dest_table[out_i] = row;
            out_i = out_i + 1u;
        }
    }
    for (var b = 0u; b < params.n_births; b++) {
        dest_table[out_i] = birth_rows[b];
        out_i = out_i + 1u;
    }
    atomicStore(&out_count, out_i);
}
