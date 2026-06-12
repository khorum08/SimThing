// PALMA-PATH-2: bounded min-plus neighbor relaxation over Location gridcell D/W columns.
// Numeric field arithmetic only — no semantic interpretation, no route objects.

struct MinPlusParams {
    width: u32,
    height: u32,
    n_dims: u32,
    d_col: u32,
    w_col: u32,
    dest_x: u32,
    dest_y: u32,
    inf_sentinel: f32,
}

@group(0) @binding(0) var<uniform> params: MinPlusParams;
@group(0) @binding(1) var<storage, read> input_values: array<f32>;
@group(0) @binding(2) var<storage, read_write> output_values: array<f32>;

fn in_bounds(x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && x < i32(params.width) && y < i32(params.height);
}

fn read_d(x: i32, y: i32) -> f32 {
    if !in_bounds(x, y) {
        return params.inf_sentinel;
    }
    let idx = u32(y) * params.width + u32(x);
    let base = idx * params.n_dims;
    return input_values[base + params.d_col];
}

fn read_w(x: u32, y: u32) -> f32 {
    let idx = y * params.width + x;
    let base = idx * params.n_dims;
    return input_values[base + params.w_col];
}

@compute @workgroup_size(8, 8, 1)
fn min_plus_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if x >= params.width || y >= params.height {
        return;
    }

    let idx = y * params.width + x;
    let base = idx * params.n_dims;

    for (var d = 0u; d < params.n_dims; d = d + 1u) {
        if d != params.d_col {
            output_values[base + d] = input_values[base + d];
        }
    }

    if x == params.dest_x && y == params.dest_y {
        output_values[base + params.d_col] = 0.0;
        return;
    }

    let ix = i32(x);
    let iy = i32(y);
    var best = params.inf_sentinel;
    best = min(best, read_d(ix - 1, iy));
    best = min(best, read_d(ix + 1, iy));
    best = min(best, read_d(ix, iy - 1));
    best = min(best, read_d(ix, iy + 1));

    let w_cell = read_w(x, y);
    var next_d = params.inf_sentinel;
    if best < params.inf_sentinel && w_cell < params.inf_sentinel {
        next_d = w_cell + best;
    }

    output_values[base + params.d_col] = next_d;
}
