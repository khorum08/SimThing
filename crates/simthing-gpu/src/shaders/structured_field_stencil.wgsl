// Generic structured 2D field stencil — flat buffers, dimensions, columns, kernel weights.

struct FieldStencilParams {
    width: u32,
    height: u32,
    n_dims: u32,
    source_col: u32,
    target_col: u32,
    alpha_self_decay: f32,
    gamma_neighbor: f32,
    cap: f32,
    source_cap: f32,
    boundary_mode: u32,
    variant: u32,
    directed_mode: u32,
    use_active_mask: u32,
    _pad: u32,
}

@group(0) @binding(0) var<uniform> params: FieldStencilParams;
@group(0) @binding(1) var<storage, read> input_values: array<f32>;
@group(0) @binding(2) var<storage, read_write> output_values: array<f32>;
@group(0) @binding(3) var<storage, read> active_mask: array<u32>;

fn in_bounds(x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && x < i32(params.width) && y < i32(params.height);
}

fn sample_source(x: i32, y: i32) -> f32 {
    if !in_bounds(x, y) {
        if params.boundary_mode == 1u {
            let cx = clamp(x, 0, i32(params.width) - 1);
            let cy = clamp(y, 0, i32(params.height) - 1);
            let idx = u32(cy) * params.width + u32(cx);
            let base = idx * params.n_dims;
            return input_values[base + params.source_col];
        }
        return 0.0;
    }
    let idx = u32(y) * params.width + u32(x);
    let base = idx * params.n_dims;
    return input_values[base + params.source_col];
}

@compute @workgroup_size(8, 8, 1)
fn stencil_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if x >= params.width || y >= params.height {
        return;
    }

    let idx = y * params.width + x;
    let base = idx * params.n_dims;

    if params.use_active_mask != 0u && active_mask[idx] == 0u {
        for (var d = 0u; d < params.n_dims; d = d + 1u) {
            output_values[base + d] = input_values[base + d];
        }
        return;
    }

    let ix = i32(x);
    let iy = i32(y);
    let center = sample_source(ix, iy);
    let north = sample_source(ix, iy - 1);
    let south = sample_source(ix, iy + 1);
    let west = sample_source(ix - 1, iy);
    let east = sample_source(ix + 1, iy);

    var neighbor_sum = 0.0;
    var neighbor_count = 0.0;

    if params.variant == 2u {
        if params.directed_mode == 0u {
            neighbor_sum = south + east;
            neighbor_count = 2.0;
        } else {
            neighbor_sum = north + west;
            neighbor_count = 2.0;
        }
    } else {
        neighbor_sum = north + south + east + west;
        neighbor_count = 4.0;
    }

    var next = params.alpha_self_decay * center;
    if params.variant == 1u || params.variant == 4u || params.variant == 5u {
        if neighbor_count > 0.0 {
            next = next + params.gamma_neighbor * (neighbor_sum / neighbor_count);
        }
    } else {
        next = next + params.gamma_neighbor * neighbor_sum;
    }

    if params.variant == 3u && params.cap > 0.0 {
        next = min(params.cap, max(0.0, next));
    }
    if params.variant == 5u && params.source_cap > 0.0 {
        next = min(params.source_cap, max(0.0, next));
    }

    for (var d = 0u; d < params.n_dims; d = d + 1u) {
        output_values[base + d] = input_values[base + d];
    }
    output_values[base + params.target_col] = next;
}
