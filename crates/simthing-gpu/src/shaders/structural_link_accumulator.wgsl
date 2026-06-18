// PROBATION proof scaffolding — proof_only / smoke_only / not_runtime.
// Exists only to prove GPU residency and bit-exact i32 neighbor accumulation over canonical
// structural links. Must not mature into production resource-flow accumulation; converge on
// accumulator_op_generic.wgsl (AO-WGSL-0) via simthing-driver compile/assembly and simthing-sim tick.

struct StructuralFrameGpuRow {
    width: u32,
    height: u32,
    occupied_cells: u32,
    location_count: u32,
    link_count: u32,
    reserved0: u32,
    reserved1: u32,
    reserved2: u32,
}

struct StructuralLinkGpuRow {
    from_dense_index: u32,
    to_dense_index: u32,
    reserved0: u32,
    reserved1: u32,
}

struct StructuralLinkAccumulatorReportGpu {
    location_count: u32,
    link_count: u32,
    invalid_link_endpoint_count: atomic<u32>,
    self_link_count: atomic<u32>,
    reserved0: u32,
    reserved1: u32,
    reserved2: u32,
    reserved3: u32,
}

@group(0) @binding(0) var<storage, read> frame_rows: array<StructuralFrameGpuRow>;
@group(0) @binding(1) var<storage, read> structural_links: array<StructuralLinkGpuRow>;
@group(0) @binding(2) var<storage, read> input_values: array<i32>;
@group(0) @binding(3) var<storage, read_write> output_values: array<atomic<i32>>;
@group(0) @binding(4) var<storage, read_write> accumulator_report: StructuralLinkAccumulatorReportGpu;
@group(0) @binding(5) var<uniform> link_count: u32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= link_count) {
        return;
    }

    let location_count = frame_rows[0].location_count;
    let link = structural_links[idx];

    if (link.from_dense_index == link.to_dense_index) {
        atomicAdd(&accumulator_report.self_link_count, 1u);
        return;
    }
    if (link.from_dense_index >= location_count || link.to_dense_index >= location_count) {
        atomicAdd(&accumulator_report.invalid_link_endpoint_count, 1u);
        return;
    }

    let from_idx = link.from_dense_index;
    let to_idx = link.to_dense_index;
    atomicAdd(&output_values[from_idx], input_values[to_idx]);
    atomicAdd(&output_values[to_idx], input_values[from_idx]);
}