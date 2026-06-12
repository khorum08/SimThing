// BH-1R / BH-1R-SCALE: staged parallel GPU sum/max/count over resident choke column.
// Pass 1: parallel workgroups gather per-cell partials; pass 2: reduce partials to compact output.

struct ChokeThresholdParams {
    width: u32,
    height: u32,
    n_dims: u32,
    choke_col: u32,
    threshold: f32,
    n_partials: u32,
    _pad: u32,
}

@group(0) @binding(0) var<uniform> params: ChokeThresholdParams;
@group(0) @binding(1) var<storage, read> read_buffer: array<f32>;
@group(0) @binding(2) var<storage, read_write> write_buffer: array<f32>;

var<workgroup> shared_sum: array<f32, 256>;
var<workgroup> shared_max: array<f32, 256>;
var<workgroup> shared_count: array<f32, 256>;

fn tree_reduce_workgroup(lane: u32) {
    var offset = 128u;
    loop {
        if offset == 0u {
            break;
        }
        if lane < offset {
            shared_sum[lane] = shared_sum[lane] + shared_sum[lane + offset];
            let other_max = shared_max[lane + offset];
            if other_max > shared_max[lane] {
                shared_max[lane] = other_max;
            }
            shared_count[lane] = shared_count[lane] + shared_count[lane + offset];
        }
        workgroupBarrier();
        offset = offset >> 1u;
    }
}

@compute @workgroup_size(256, 1, 1)
fn reduce_choke_partials_pass1(
    @builtin(workgroup_id) wid: vec3<u32>,
    @builtin(local_invocation_id) lid: vec3<u32>,
) {
    let lane = lid.x;
    let wg = wid.x;
    let cells = params.width * params.height;
    let total_threads = 256u * params.n_partials;

    var local_sum = 0.0;
    var local_max = 0.0;
    var local_count = 0.0;

    var slot = wg * 256u + lane;
    while (slot < cells) {
        let base = slot * params.n_dims;
        let v = read_buffer[base + params.choke_col];
        local_sum = local_sum + v;
        if v > local_max {
            local_max = v;
        }
        if v > params.threshold {
            local_count = local_count + 1.0;
        }
        slot = slot + total_threads;
    }

    shared_sum[lane] = local_sum;
    shared_max[lane] = local_max;
    shared_count[lane] = local_count;
    workgroupBarrier();

    tree_reduce_workgroup(lane);

    if lane == 0u {
        let out_base = wg * 3u;
        write_buffer[out_base + 0u] = shared_sum[0u];
        write_buffer[out_base + 1u] = shared_max[0u];
        write_buffer[out_base + 2u] = shared_count[0u];
    }
}

@compute @workgroup_size(256, 1, 1)
fn reduce_choke_final_pass2(@builtin(local_invocation_id) lid: vec3<u32>) {
    let lane = lid.x;
    var local_sum = 0.0;
    var local_max = 0.0;
    var local_count = 0.0;

    var idx = lane;
    while (idx < params.n_partials) {
        let base = idx * 3u;
        local_sum = local_sum + read_buffer[base + 0u];
        let partial_max = read_buffer[base + 1u];
        if partial_max > local_max {
            local_max = partial_max;
        }
        local_count = local_count + read_buffer[base + 2u];
        idx = idx + 256u;
    }

    shared_sum[lane] = local_sum;
    shared_max[lane] = local_max;
    shared_count[lane] = local_count;
    workgroupBarrier();

    tree_reduce_workgroup(lane);

    if lane == 0u {
        let sum_choke = shared_sum[0u];
        write_buffer[0u] = sum_choke;
        write_buffer[1u] = shared_max[0u];
        write_buffer[2u] = shared_count[0u];
        if sum_choke > params.threshold {
            write_buffer[3u] = 1.0;
        } else {
            write_buffer[3u] = 0.0;
        }
    }
}
