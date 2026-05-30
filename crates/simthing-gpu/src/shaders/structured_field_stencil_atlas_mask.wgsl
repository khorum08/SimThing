// Generic semantic-free atlas tile-local mask stencil (C-0 / M-4).
// Flat buffers, indices, dimensions, masks, strides, tile metadata only.

struct AtlasMaskParams {
    width: u32,
    height: u32,
    n_dims: u32,
    source_col: u32,
    target_col: u32,
    tile_size: u32,
    alpha_self_decay: f32,
    gamma_neighbor: f32,
    source_cap: f32,
    variant: u32, // 1 = normalized, 5 = source_capped
    use_tile_local_mask: u32,
    renorm_valid_neighbors: u32, // 0 = fixed denominator, 1 = valid-neighbor renorm
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<uniform> params: AtlasMaskParams;
@group(0) @binding(1) var<storage, read> input_values: array<f32>;
@group(0) @binding(2) var<storage, read_write> output_values: array<f32>;

fn in_bounds(x: i32, y: i32) -> bool {
    return x >= 0 && y >= 0 && x < i32(params.width) && y < i32(params.height);
}

fn neighbor_valid(gx: u32, gy: u32, dx: i32, dy: i32) -> bool {
    let ngx = i32(gx) + dx;
    let ngy = i32(gy) + dy;
    if !in_bounds(ngx, ngy) {
        return false;
    }
    if params.use_tile_local_mask == 0u {
        return true;
    }
    let tile_x = gx / params.tile_size;
    let tile_y = gy / params.tile_size;
    let local_x = gx - tile_x * params.tile_size;
    let local_y = gy - tile_y * params.tile_size;
    let nlx = i32(local_x) + dx;
    let nly = i32(local_y) + dy;
    return nlx >= 0 && nlx < i32(params.tile_size) && nly >= 0 && nly < i32(params.tile_size);
}

fn sample_neighbor(gx: u32, gy: u32, dx: i32, dy: i32) -> f32 {
    if !neighbor_valid(gx, gy, dx, dy) {
        return 0.0;
    }
    let ngx = i32(gx) + dx;
    let ngy = i32(gy) + dy;
    let idx = u32(ngy) * params.width + u32(ngx);
    let base = idx * params.n_dims;
    return input_values[base + params.source_col];
}

@compute @workgroup_size(8, 8, 1)
fn atlas_mask_stencil_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if x >= params.width || y >= params.height {
        return;
    }

    let idx = y * params.width + x;
    let base = idx * params.n_dims;

    let center = sample_neighbor(x, y, 0, 0);
    let north = sample_neighbor(x, y, 0, -1);
    let south = sample_neighbor(x, y, 0, 1);
    let west = sample_neighbor(x, y, -1, 0);
    let east = sample_neighbor(x, y, 1, 0);

    var neighbor_sum = north + south + east + west;
    var neighbor_count = 4.0;

    if params.renorm_valid_neighbors != 0u {
        neighbor_count = 0.0;
        neighbor_sum = 0.0;
        if neighbor_valid(x, y, 0, -1) {
            neighbor_count = neighbor_count + 1.0;
            neighbor_sum = neighbor_sum + sample_neighbor(x, y, 0, -1);
        }
        if neighbor_valid(x, y, 0, 1) {
            neighbor_count = neighbor_count + 1.0;
            neighbor_sum = neighbor_sum + sample_neighbor(x, y, 0, 1);
        }
        if neighbor_valid(x, y, -1, 0) {
            neighbor_count = neighbor_count + 1.0;
            neighbor_sum = neighbor_sum + sample_neighbor(x, y, -1, 0);
        }
        if neighbor_valid(x, y, 1, 0) {
            neighbor_count = neighbor_count + 1.0;
            neighbor_sum = neighbor_sum + sample_neighbor(x, y, 1, 0);
        }
    }

    var next = params.alpha_self_decay * center;
    if neighbor_count > 0.0 {
        next = next + params.gamma_neighbor * (neighbor_sum / neighbor_count);
    }

    if params.variant == 5u && params.source_cap > 0.0 {
        next = min(params.source_cap, max(0.0, next));
    }

    for (var d = 0u; d < params.n_dims; d = d + 1u) {
        output_values[base + d] = input_values[base + d];
    }
    output_values[base + params.target_col] = next;
}
