struct PoolState {
    amount: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

struct QueueState {
    accum: u32,
    units: u32,
    unit_cost: u32,
    is_active: u32,
}

struct TransferRequest {
    pool: u32,
    queue: u32,
    amount_requested: u32,
    priority_band: u32,
    priority: u32,
    authored_order: u32,
    is_active: u32,
    pad0: u32,
}

struct PoolRequestRange {
    start: u32,
    count: u32,
}

struct TransferDeltaRecord {
    tick: u32,
    request: u32,
    pool: u32,
    queue: u32,
    requested: u32,
    allocated: u32,
    emitted_units: u32,
    is_active: u32,
}

struct PoolTickSummary {
    pool: u32,
    tick: u32,
    amount_before: u32,
    amount_after: u32,
    total_requested: u32,
    total_allocated: u32,
    total_emitted_units: u32,
    active_requests: u32,
}

struct TransferResidentParams {
    n_pools: u32,
    n_queues: u32,
    n_requests: u32,
    tick_index: u32,
    record_stride: u32,
    summary_stride: u32,
    write_records: u32,
    pad0: u32,
}

@group(0) @binding(0) var<storage, read_write> pools: array<PoolState>;
@group(0) @binding(1) var<storage, read_write> queues: array<QueueState>;
@group(0) @binding(2) var<storage, read> requests: array<TransferRequest>;
@group(0) @binding(3) var<storage, read> pool_ranges: array<PoolRequestRange>;
@group(0) @binding(4) var<storage, read_write> summaries: array<PoolTickSummary>;
@group(0) @binding(5) var<storage, read_write> records: array<TransferDeltaRecord>;
@group(0) @binding(6) var<uniform> params: TransferResidentParams;

@compute @workgroup_size(64)
fn resolve_transfer_contention_tick(@builtin(global_invocation_id) gid: vec3<u32>) {
    let pool_idx = gid.x;
    if (pool_idx >= params.n_pools) {
        return;
    }

    let range = pool_ranges[pool_idx];

    var pool = pools[pool_idx];
    let amount_before = pool.amount;
    var remaining = pool.amount;

    var total_requested: u32 = 0u;
    var total_allocated: u32 = 0u;
    var total_emitted_units: u32 = 0u;
    var active_requests: u32 = 0u;

    for (var i: u32 = 0u; i < range.count; i = i + 1u) {
        let request_idx = range.start + i;
        let req = requests[request_idx];

        var allocated: u32 = 0u;
        var emitted: u32 = 0u;
        var is_active: u32 = 0u;

        if (req.is_active != 0u && req.queue < params.n_queues) {
            var q = queues[req.queue];
            if (q.is_active != 0u && q.unit_cost != 0u) {
                is_active = 1u;
                active_requests = active_requests + 1u;
                total_requested = total_requested + req.amount_requested;

                allocated = min(remaining, req.amount_requested);
                remaining = remaining - allocated;

                let pre_emit = q.accum + allocated;
                emitted = pre_emit / q.unit_cost;
                q.accum = pre_emit - emitted * q.unit_cost;
                q.units = q.units + emitted;
                queues[req.queue] = q;

                total_allocated = total_allocated + allocated;
                total_emitted_units = total_emitted_units + emitted;
            }
        }

        if (params.write_records != 0u) {
            let rec_idx = params.tick_index * params.record_stride + request_idx;
            records[rec_idx] = TransferDeltaRecord(
                params.tick_index,
                request_idx,
                pool_idx,
                req.queue,
                req.amount_requested,
                allocated,
                emitted,
                is_active
            );
        }
    }

    pool.amount = remaining;
    pools[pool_idx] = pool;

    let summary_idx = params.tick_index * params.summary_stride + pool_idx;
    summaries[summary_idx] = PoolTickSummary(
        pool_idx,
        params.tick_index,
        amount_before,
        remaining,
        total_requested,
        total_allocated,
        total_emitted_units,
        active_requests
    );
}
