struct QueueState {
    source_pool: u32,
    queue_accum: u32,
    units: u32,
    pad0: u32,
}

struct QueueParams {
    daily_request: u32,
    unit_cost: u32,
    is_active: u32,
    pad0: u32,
}

struct CompactDeltaRecord {
    item: u32,
    transfer_amount: u32,
    emit_count: u32,
    is_active: u32,
}

struct FullDeltaRecord {
    item: u32,
    source_before: u32,
    queue_before: u32,
    units_before: u32,
    source_after: u32,
    queue_after: u32,
    units_after: u32,
    transfer_amount: u32,
    emit_count: u32,
    is_active: u32,
    pad0: u32,
}

struct MultiTargetParams {
    n_items: u32,
    replay_mode: u32,
    pad0: u32,
    pad1: u32,
}

const REPLAY_MODE_FULL: u32 = 1u;

@group(0) @binding(0) var<storage, read> initial_states: array<QueueState>;
@group(0) @binding(1) var<storage, read> queue_params: array<QueueParams>;
@group(0) @binding(2) var<storage, read_write> final_states: array<QueueState>;
@group(0) @binding(3) var<storage, read_write> compact_records: array<CompactDeltaRecord>;
@group(0) @binding(4) var<storage, read_write> full_records: array<FullDeltaRecord>;
@group(0) @binding(5) var<uniform> params: MultiTargetParams;

@compute @workgroup_size(64)
fn resolve_multitarget(@builtin(global_invocation_id) gid: vec3<u32>) {
    let item = gid.x;
    if (item >= params.n_items) {
        return;
    }

    let s = initial_states[item];
    let p = queue_params[item];

    var source_after = s.source_pool;
    var queue_after = s.queue_accum;
    var units_after = s.units;
    var transfer: u32 = 0u;
    var emit_count: u32 = 0u;

    if (p.is_active != 0u && p.unit_cost != 0u) {
        transfer = min(s.source_pool, p.daily_request);
        source_after = s.source_pool - transfer;

        let queue_pre_emit = s.queue_accum + transfer;
        emit_count = queue_pre_emit / p.unit_cost;
        queue_after = queue_pre_emit - emit_count * p.unit_cost;
        units_after = s.units + emit_count;
    }

    final_states[item] = QueueState(source_after, queue_after, units_after, 0u);
    compact_records[item] = CompactDeltaRecord(item, transfer, emit_count, p.is_active);

    if (params.replay_mode == REPLAY_MODE_FULL) {
        var full = FullDeltaRecord(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
        full.item = item;
        full.source_before = s.source_pool;
        full.queue_before = s.queue_accum;
        full.units_before = s.units;
        full.source_after = source_after;
        full.queue_after = queue_after;
        full.units_after = units_after;
        full.transfer_amount = transfer;
        full.emit_count = emit_count;
        full.is_active = p.is_active;
        full.pad0 = 0u;
        full_records[item] = full;
    }
}

struct ResidentParams {
    n_items: u32,
    tick_index: u32,
    record_stride: u32,
    write_per_item_records: u32,
}

@group(0) @binding(0) var<storage, read_write> resident_states: array<QueueState>;
@group(0) @binding(1) var<storage, read> resident_params: array<QueueParams>;
@group(0) @binding(2) var<storage, read_write> resident_compact_records: array<CompactDeltaRecord>;
@group(0) @binding(3) var<storage, read_write> resident_summary: array<atomic<u32>>;
@group(0) @binding(4) var<uniform> resident_uniform: ResidentParams;

@compute @workgroup_size(64)
fn resolve_multitarget_resident_tick(@builtin(global_invocation_id) gid: vec3<u32>) {
    let item = gid.x;
    if (item >= resident_uniform.n_items) {
        return;
    }

    let p = resident_params[item];
    var s = resident_states[item];

    var transfer: u32 = 0u;
    var emit_count: u32 = 0u;

    if (p.is_active != 0u && p.unit_cost != 0u) {
        transfer = min(s.source_pool, p.daily_request);
        s.source_pool = s.source_pool - transfer;

        let queue_pre_emit = s.queue_accum + transfer;
        emit_count = queue_pre_emit / p.unit_cost;
        s.queue_accum = queue_pre_emit - emit_count * p.unit_cost;
        s.units = s.units + emit_count;
    }

    resident_states[item] = s;

    let summary_base = resident_uniform.tick_index * 4u;
    atomicAdd(&resident_summary[summary_base + 0u], transfer);
    atomicAdd(&resident_summary[summary_base + 1u], emit_count);
    if (p.is_active != 0u) {
        atomicAdd(&resident_summary[summary_base + 2u], 1u);
    }

    if (resident_uniform.write_per_item_records != 0u) {
        let record_index = resident_uniform.tick_index * resident_uniform.record_stride + item;
        resident_compact_records[record_index] =
            CompactDeltaRecord(item, transfer, emit_count, p.is_active);
    }
}
