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

struct StructuralValidationReportGpu {
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
@group(0) @binding(2) var<storage, read_write> validation_report: StructuralValidationReportGpu;
@group(0) @binding(3) var<uniform> link_count: u32;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= link_count) {
        return;
    }

    let location_count = frame_rows[0].location_count;
    let link = structural_links[idx];

    if (link.from_dense_index == link.to_dense_index) {
        atomicAdd(&validation_report.self_link_count, 1u);
    }
    if (link.from_dense_index >= location_count || link.to_dense_index >= location_count) {
        atomicAdd(&validation_report.invalid_link_endpoint_count, 1u);
    }
}