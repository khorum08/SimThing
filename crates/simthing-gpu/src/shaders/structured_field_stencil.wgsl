// Generic structured 2D field stencil — flat buffers, dimensions, columns, kernel weights.

struct FieldStencilParams {
    width: u32,
    height: u32,
    n_dims: u32,
    source_col: u32,
    target_col: u32,
    alpha_self_decay: f32,
    weight_north: f32,
    weight_south: f32,
    weight_east: f32,
    weight_west: f32,
    cap: f32,
    source_cap: f32,
    boundary_mode: u32,
    variant: u32,
    directed_mode: u32,
    use_active_mask: u32,
    target_col_y: u32,
    u_sat: f32,
    chi: f32,
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

fn read_u_source(x: i32, y: i32) -> f32 {
    let idx = u32(y) * params.width + u32(x);
    let base = idx * params.n_dims;
    return input_values[base + params.source_col];
}

fn sigma_u(u: f32) -> f32 {
    let x = u / params.u_sat;
    if (x < 0.0) {
        return 0.0;
    }
    if (x > 1.0) {
        return 1.0;
    }
    return x;
}

fn compute_c_at(x: i32, y: i32) -> f32 {
    var c = params.chi;
    let ny = y - 1;
    if in_bounds(x, ny) {
        c = c * (1.0 - sigma_u(read_u_source(x, ny)));
    }
    let sy = y + 1;
    if in_bounds(x, sy) {
        c = c * (1.0 - sigma_u(read_u_source(x, sy)));
    }
    let ex = x + 1;
    if in_bounds(ex, y) {
        c = c * (1.0 - sigma_u(read_u_source(ex, y)));
    }
    let wx = x - 1;
    if in_bounds(wx, y) {
        c = c * (1.0 - sigma_u(read_u_source(wx, y)));
    }
    return c;
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

    if params.variant == 7u {
        let c_i = compute_c_at(ix, iy);
        let u_i = read_u_source(ix, iy);
        var next = u_i;

        let ny = iy - 1;
        if in_bounds(ix, ny) {
            let c_n = compute_c_at(ix, ny);
            let u_n = read_u_source(ix, ny);
            next = next + ((c_i + c_n) * 0.5) * (u_n - u_i);
        }
        let sy = iy + 1;
        if in_bounds(ix, sy) {
            let c_s = compute_c_at(ix, sy);
            let u_s = read_u_source(ix, sy);
            next = next + ((c_i + c_s) * 0.5) * (u_s - u_i);
        }
        let ex = ix + 1;
        if in_bounds(ex, iy) {
            let c_e = compute_c_at(ex, iy);
            let u_e = read_u_source(ex, iy);
            next = next + ((c_i + c_e) * 0.5) * (u_e - u_i);
        }
        let wx = ix - 1;
        if in_bounds(wx, iy) {
            let c_w = compute_c_at(wx, iy);
            let u_w = read_u_source(wx, iy);
            next = next + ((c_i + c_w) * 0.5) * (u_w - u_i);
        }

        for (var d = 0u; d < params.n_dims; d = d + 1u) {
            output_values[base + d] = input_values[base + d];
        }
        output_values[base + params.target_col] = next;
        return;
    }

    // Dual-output gradient: axis-X (E/W weights) -> target_col, axis-Y (N/S weights) -> target_col_y.
    if params.variant == 6u {
        let gx = params.weight_east * east + params.weight_west * west;
        let gy = params.weight_north * north + params.weight_south * south;
        for (var d = 0u; d < params.n_dims; d = d + 1u) {
            output_values[base + d] = input_values[base + d];
        }
        output_values[base + params.target_col] = gx;
        output_values[base + params.target_col_y] = gy;
        return;
    }

    var next = params.alpha_self_decay * center
        + params.weight_north * north
        + params.weight_south * south
        + params.weight_east * east
        + params.weight_west * west;

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
