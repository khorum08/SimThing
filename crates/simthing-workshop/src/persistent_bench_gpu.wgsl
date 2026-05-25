struct BenchPool {
    amount: u32,
    regen_per_tick: u32,
    pad0: u32,
    pad1: u32,
}

struct BenchQueue {
    pool: u32,
    accum: u32,
    units: u32,
    unit_cost: u32,
    request_per_tick: u32,
    priority: u32,
    is_active: u32,
    pad0: u32,
}

struct BenchPoolRange {
    start: u32,
    count: u32,
}

struct BenchRecord {
    tick: u32,
    queue: u32,
    pool: u32,
    requested: u32,
    allocated: u32,
    emitted_units: u32,
    is_active: u32,
    pad0: u32,
}

struct BenchParams {
    n_pools: u32,
    n_queues: u32,
    tick_index: u32,
    log_mode: u32,
    record_stride: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<storage, read_write> pools: array<BenchPool>;
@group(0) @binding(1) var<storage, read_write> queues: array<BenchQueue>;
@group(0) @binding(2) var<storage, read> pool_ranges: array<BenchPoolRange>;
@group(0) @binding(3) var<storage, read_write> global_accum: array<atomic<u32>>;
@group(0) @binding(4) var<storage, read_write> records: array<BenchRecord>;
@group(0) @binding(5) var<uniform> params: BenchParams;

@compute @workgroup_size(64)
fn persistent_tick(@builtin(global_invocation_id) gid: vec3<u32>) {
    let pool_idx = gid.x;
    if (pool_idx >= params.n_pools) {
        return;
    }

    var pool = pools[pool_idx];
    let before = pool.amount;
    pool.amount = pool.amount + pool.regen_per_tick;

    let range = pool_ranges[pool_idx];

    var total_allocated: u32 = 0u;
    var total_emitted: u32 = 0u;
    var active_queues: u32 = 0u;

    for (var i: u32 = 0u; i < range.count; i = i + 1u) {
        let q_idx = range.start + i;
        var q = queues[q_idx];

        var allocated: u32 = 0u;
        var emitted: u32 = 0u;
        var is_active: u32 = 0u;

        if (q.is_active != 0u && q.unit_cost != 0u) {
            is_active = 1u;
            active_queues = active_queues + 1u;
            allocated = min(pool.amount, q.request_per_tick);
            pool.amount = pool.amount - allocated;

            let pre_emit = q.accum + allocated;
            emitted = pre_emit / q.unit_cost;
            q.accum = pre_emit - emitted * q.unit_cost;
            q.units = q.units + emitted;
            queues[q_idx] = q;

            total_allocated = total_allocated + allocated;
            total_emitted = total_emitted + emitted;
        }

        if (params.log_mode != 0u) {
            let rec_idx = params.tick_index * params.record_stride + q_idx;
            records[rec_idx] = BenchRecord(
                params.tick_index,
                q_idx,
                pool_idx,
                q.request_per_tick,
                allocated,
                emitted,
                is_active,
                0u
            );
        }
    }

    pools[pool_idx] = pool;

    let base = params.tick_index * 6u;
    atomicAdd(&global_accum[base + 0u], before);
    atomicAdd(&global_accum[base + 1u], pool.amount);
    atomicAdd(&global_accum[base + 2u], total_allocated);
    atomicAdd(&global_accum[base + 3u], total_emitted);
    atomicAdd(&global_accum[base + 4u], active_queues);
}
