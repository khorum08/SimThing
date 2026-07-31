//! FIELD-SWEEP-IR-PROBE-0 — disposable workshop-leaf field-sweep IR measurement probe.
//!
//! Private/test-only. Consumes public engine/GPU doors from integration tests only.
//! No production export, opcode, registration, or reverse dependency.
//! Born mortal: birth track 0.0.8.7 envelope, dsu_survivals=0, no permanent-residue.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use simthing_core::{EML_STACK_MAX, MAX_EML_TREE_NODES};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Limits, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderStages,
};

pub const PROBE_STACK_MAX: u32 = 32;
pub const PROBE_NODE_CAP: u32 = 32;
pub const WARM_RUNS: usize = 8;
pub const SAMPLE_RUNS: usize = 16;
pub const WORKGROUP_SIZE: u32 = 64;

/// Measurement label only — not an engine-admitted resource_class enum.
pub const RESOURCE_CLASS_LABEL: &str = "legacy_fixed_32_stack";

pub const OP_CONST: u32 = 0;
pub const OP_TARGET_VALUE: u32 = 1;
pub const OP_NEIGHBOR_VALUE: u32 = 2;
pub const OP_ACC: u32 = 3;
pub const OP_MAPPED: u32 = 4;
pub const OP_FOLDED: u32 = 5;
pub const OP_ADD: u32 = 6;
pub const OP_SUB: u32 = 7;
pub const OP_MUL: u32 = 8;
pub const OP_MIN: u32 = 9;
pub const OP_MAX: u32 = 10;
pub const OP_DIV: u32 = 11;
pub const OP_CLAMP01: u32 = 12;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct IrNode {
    pub op: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub value: f32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CellRange {
    pub offset: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SweepParams {
    pub n_cells: u32,
    pub n_dims: u32,
    pub out_col: u32,
    pub dest_cell: u32,
    pub force_dest_zero: u32,
    pub map_root: u32,
    pub fold_root: u32,
    pub post_root: u32,
    pub fold_identity: f32,
    /// 0 = use fold_identity const; 1 = seed from values[cell*n_dims + fold_seed_col]
    pub fold_seed_from_target: u32,
    pub fold_seed_col: u32,
    pub _pad0: u32,
}

/// Per-cell fold seed. `TargetValue` is required for Gu-Yang-shaped `next = u_i; next += …`
/// bit-exactness (float non-associativity forbids `u_i + Σterms` vs `(u_i+t1)+t2`).
#[derive(Clone, Copy, Debug)]
pub enum FoldSeed {
    Const(f32),
    TargetValue(u32),
}

#[derive(Clone, Debug)]
pub struct IrProgram {
    pub map: Vec<IrNode>,
    pub fold: Vec<IrNode>,
    pub post: Vec<IrNode>,
    pub fold_seed: FoldSeed,
}

#[derive(Clone, Debug)]
pub struct GatherTable {
    pub adjacency_kind: &'static str,
    pub offsets: Vec<(i32, i32)>,
    pub ranges: Vec<CellRange>,
    pub neighbors: Vec<u32>,
    pub edge_count: u32,
    pub degree_histogram: Vec<(u32, u32)>,
}

#[derive(Clone, Debug)]
pub struct ProgramMetrics {
    pub map_nodes: u32,
    pub fold_nodes: u32,
    pub post_nodes: u32,
    pub total_nodes: u32,
    pub actual_max_stack_depth: u32,
    pub column_reads_per_edge: u32,
}

#[derive(Clone, Debug)]
pub struct EmlCapFacts {
    pub configured_max_tree_nodes: u32,
    pub configured_stack_max: u32,
    pub probe_node_cap: u32,
    pub probe_stack_max: u32,
    pub observed_max_nodes: u32,
    pub observed_max_stack_depth: u32,
    pub resource_class_label: &'static str,
}

#[derive(Clone, Debug)]
pub struct MeasurementRow {
    pub case_name: String,
    pub adapter_backend: String,
    pub adjacency_kind: String,
    pub theater_size: String,
    pub degree_distribution: String,
    pub nodes_per_edge: u32,
    pub actual_max_stack_depth: u32,
    pub column_reads_per_edge: u32,
    pub resource_class: String,
    pub matched_occupancy: bool,
    pub matched_occupancy_basis: String,
    pub warmup_count: usize,
    pub sample_count: usize,
    pub time_per_sweep_us_median: f64,
    pub time_per_sweep_us_worst: f64,
    pub edges_per_s_median: f64,
    pub stall_memory_counters: String,
    pub counter_surface_status: String,
    pub path_kind: String,
}

#[derive(Clone, Debug)]
pub struct ParityCaseResult {
    pub case_name: String,
    pub bit_exact: bool,
    pub max_ulp: u32,
    pub cells_compared: usize,
}

fn node(op: u32, a: u32, b: u32, c: u32, value: f32) -> IrNode {
    IrNode {
        op,
        a,
        b,
        c,
        value,
        _pad0: 0,
        _pad1: 0,
        _pad2: 0,
    }
}

fn eval_nodes(
    nodes: &[IrNode],
    root: usize,
    target_base: usize,
    neighbor_base: Option<usize>,
    values: &[f32],
    _n_dims: usize,
    acc: f32,
    mapped: f32,
    folded: f32,
) -> f32 {
    let mut scratch = [0.0f32; PROBE_STACK_MAX as usize];
    for i in 0..=root {
        let n = nodes[i];
        scratch[i] = match n.op {
            OP_CONST => n.value,
            OP_TARGET_VALUE => values[target_base + n.a as usize],
            OP_NEIGHBOR_VALUE => {
                let nb = neighbor_base.expect("neighbor context");
                values[nb + n.a as usize]
            }
            OP_ACC => acc,
            OP_MAPPED => mapped,
            OP_FOLDED => folded,
            OP_ADD => scratch[n.a as usize] + scratch[n.b as usize],
            OP_SUB => scratch[n.a as usize] - scratch[n.b as usize],
            OP_MUL => scratch[n.a as usize] * scratch[n.b as usize],
            OP_MIN => scratch[n.a as usize].min(scratch[n.b as usize]),
            OP_MAX => scratch[n.a as usize].max(scratch[n.b as usize]),
            OP_DIV => scratch[n.a as usize] / scratch[n.b as usize],
            OP_CLAMP01 => scratch[n.a as usize].clamp(0.0, 1.0),
            _ => 0.0,
        };
    }
    scratch[root]
}

/// Stack-depth walk over a tree-shaped node array (parent indices < child index).
pub fn measure_tree_stack_depth(nodes: &[IrNode]) -> u32 {
    // Conservative: each node materializes one scratch slot; report root+1 as observed depth proxy,
    // and separately count peak live binary-op stack if lowered postfix-style.
    let mut max_sp = 0u32;
    let mut sp = 0u32;
    for n in nodes {
        match n.op {
            OP_CONST | OP_TARGET_VALUE | OP_NEIGHBOR_VALUE | OP_ACC | OP_MAPPED | OP_FOLDED => {
                sp += 1;
            }
            OP_CLAMP01 => {}
            OP_ADD | OP_SUB | OP_MUL | OP_MIN | OP_MAX | OP_DIV => {
                if sp >= 2 {
                    sp -= 1;
                }
            }
            _ => {}
        }
        max_sp = max_sp.max(sp);
    }
    max_sp.max(nodes.len() as u32).min(PROBE_STACK_MAX)
}

pub fn program_metrics(program: &IrProgram) -> ProgramMetrics {
    let column_reads = program
        .map
        .iter()
        .filter(|n| n.op == OP_TARGET_VALUE || n.op == OP_NEIGHBOR_VALUE)
        .count() as u32;
    let map_stack = measure_tree_stack_depth(&program.map);
    let fold_stack = measure_tree_stack_depth(&program.fold);
    let post_stack = measure_tree_stack_depth(&program.post);
    ProgramMetrics {
        map_nodes: program.map.len() as u32,
        fold_nodes: program.fold.len() as u32,
        post_nodes: program.post.len() as u32,
        total_nodes: (program.map.len() + program.fold.len() + program.post.len()) as u32,
        actual_max_stack_depth: map_stack.max(fold_stack).max(post_stack),
        column_reads_per_edge: column_reads,
    }
}

pub fn live_eml_cap_facts(observed_nodes: u32, observed_stack: u32) -> EmlCapFacts {
    EmlCapFacts {
        configured_max_tree_nodes: MAX_EML_TREE_NODES,
        configured_stack_max: EML_STACK_MAX,
        probe_node_cap: PROBE_NODE_CAP,
        probe_stack_max: PROBE_STACK_MAX,
        observed_max_nodes: observed_nodes,
        observed_max_stack_depth: observed_stack,
        resource_class_label: RESOURCE_CLASS_LABEL,
    }
}

/// Pre-named fallback: MIN × INPUT_LIST (PALMA-shaped map/fold/post; no field-identity tag).
pub fn program_min_x_input_list(d_col: u32, w_col: u32) -> IrProgram {
    // map: NEIGHBOR_VALUE(d)
    let map = vec![node(OP_NEIGHBOR_VALUE, d_col, 0, 0, 0.0)];
    // fold: MIN(ACC, MAPPED)
    let fold = vec![
        node(OP_ACC, 0, 0, 0, 0.0),
        node(OP_MAPPED, 0, 0, 0, 0.0),
        node(OP_MIN, 0, 1, 0, 0.0),
    ];
    // post: TARGET_VALUE(w) + FOLDED
    let post = vec![
        node(OP_TARGET_VALUE, w_col, 0, 0, 0.0),
        node(OP_FOLDED, 0, 0, 0, 0.0),
        node(OP_ADD, 0, 1, 0, 0.0),
    ];
    IrProgram {
        map,
        fold,
        post,
        fold_seed: FoldSeed::Const(f32::INFINITY),
    }
}

/// Pre-named fallback: PRODUCT × INPUT_LIST for conductance band (chi * Π (1-σ(u_j))).
pub fn program_product_conductance(u_col: u32, u_sat: f32, chi: f32) -> IrProgram {
    // map: 1 - min(max(NEIGHBOR_VALUE(u) / u_sat, 0), 1)  — mirrors bespoke sigma_u if/else
    let map = vec![
        node(OP_NEIGHBOR_VALUE, u_col, 0, 0, 0.0), // 0
        node(OP_CONST, 0, 0, 0, u_sat),            // 1
        node(OP_DIV, 0, 1, 0, 0.0),                // 2 = u/u_sat
        node(OP_CONST, 0, 0, 0, 0.0),              // 3
        node(OP_MAX, 2, 3, 0, 0.0),                // 4 = max(x,0)
        node(OP_CONST, 0, 0, 0, 1.0),              // 5
        node(OP_MIN, 4, 5, 0, 0.0),                // 6 = min(.,1) = σ
        node(OP_CONST, 0, 0, 0, 1.0),              // 7
        node(OP_SUB, 7, 6, 0, 0.0),                // 8 = 1-σ
    ];
    let fold = vec![
        node(OP_ACC, 0, 0, 0, 0.0),
        node(OP_MAPPED, 0, 0, 0, 0.0),
        node(OP_MUL, 0, 1, 0, 0.0),
    ];
    // post: FOLDED (identity write)
    let post = vec![node(OP_FOLDED, 0, 0, 0, 0.0)];
    IrProgram {
        map,
        fold,
        post,
        fold_seed: FoldSeed::Const(chi),
    }
}

/// Pre-named fallback: banded flux — seed at u_i, then Σ ((c_i+c_j)/2)*(u_j-u_i).
pub fn program_banded_flux(u_col: u32, c_col: u32) -> IrProgram {
    // map: ((TARGET(c)+NEIGHBOR(c))*0.5) * (NEIGHBOR(u)-TARGET(u))
    let map = vec![
        node(OP_TARGET_VALUE, c_col, 0, 0, 0.0),
        node(OP_NEIGHBOR_VALUE, c_col, 0, 0, 0.0),
        node(OP_ADD, 0, 1, 0, 0.0),
        node(OP_CONST, 0, 0, 0, 0.5),
        node(OP_MUL, 2, 3, 0, 0.0),
        node(OP_NEIGHBOR_VALUE, u_col, 0, 0, 0.0),
        node(OP_TARGET_VALUE, u_col, 0, 0, 0.0),
        node(OP_SUB, 5, 6, 0, 0.0),
        node(OP_MUL, 4, 7, 0, 0.0),
    ];
    let fold = vec![
        node(OP_ACC, 0, 0, 0, 0.0),
        node(OP_MAPPED, 0, 0, 0, 0.0),
        node(OP_ADD, 0, 1, 0, 0.0),
    ];
    // post: FOLDED (already seeded at u_i — matches bespoke next=u_i; next+=term)
    let post = vec![node(OP_FOLDED, 0, 0, 0, 0.0)];
    IrProgram {
        map,
        fold,
        post,
        fold_seed: FoldSeed::TargetValue(u_col),
    }
}

/// Canonical PALMA N4 order: W → E → N → S.
pub const N4_OFFSETS_WENS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
/// Canonical Gu-Yang N4 order: N → S → E → W.
pub const N4_OFFSETS_NSEW: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
/// Throwaway workshop N8 (engine N8 stays 5.6). Order: N4-NSEW then diagonals NW,NE,SW,SE.
pub const N8_OFFSETS_THROWAWAY: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

pub fn build_gather(
    width: u32,
    height: u32,
    offsets: &[(i32, i32)],
    adjacency_kind: &'static str,
) -> GatherTable {
    let w = width as i32;
    let h = height as i32;
    let mut ranges = Vec::with_capacity((width * height) as usize);
    let mut neighbors = Vec::new();
    let mut degree_counts = std::collections::BTreeMap::<u32, u32>::new();
    for y in 0..h {
        for x in 0..w {
            let offset = neighbors.len() as u32;
            let mut len = 0u32;
            for &(dx, dy) in offsets {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < w && ny < h {
                    neighbors.push((ny as u32 * width + nx as u32) as u32);
                    len += 1;
                }
            }
            *degree_counts.entry(len).or_insert(0) += 1;
            ranges.push(CellRange { offset, len });
        }
    }
    let edge_count = neighbors.len() as u32;
    GatherTable {
        adjacency_kind,
        offsets: offsets.to_vec(),
        ranges,
        neighbors,
        edge_count,
        degree_histogram: degree_counts.into_iter().collect(),
    }
}

pub fn format_degree_distribution(hist: &[(u32, u32)]) -> String {
    hist.iter()
        .map(|(deg, count)| format!("deg{deg}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn cpu_sweep_once(
    values: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    out_col: u32,
    gather: &GatherTable,
    program: &IrProgram,
    dest_cell: Option<u32>,
) -> Vec<f32> {
    let n_cells = (width * height) as usize;
    let nd = n_dims as usize;
    let mut out = values.to_vec();
    let map_root = program.map.len().saturating_sub(1);
    let fold_root = program.fold.len().saturating_sub(1);
    let post_root = program.post.len().saturating_sub(1);
    for cell in 0..n_cells {
        if dest_cell == Some(cell as u32) {
            out[cell * nd + out_col as usize] = 0.0;
            continue;
        }
        let range = gather.ranges[cell];
        let target_base = cell * nd;
        let mut acc = match program.fold_seed {
            FoldSeed::Const(v) => v,
            FoldSeed::TargetValue(col) => values[target_base + col as usize],
        };
        for k in 0..range.len {
            let neighbor = gather.neighbors[(range.offset + k) as usize] as usize;
            let neighbor_base = neighbor * nd;
            let mapped = eval_nodes(
                &program.map,
                map_root,
                target_base,
                Some(neighbor_base),
                values,
                nd,
                0.0,
                0.0,
                0.0,
            );
            acc = eval_nodes(
                &program.fold,
                fold_root,
                target_base,
                Some(neighbor_base),
                values,
                nd,
                acc,
                mapped,
                0.0,
            );
        }
        let written = eval_nodes(
            &program.post,
            post_root,
            target_base,
            None,
            values,
            nd,
            0.0,
            0.0,
            acc,
        );
        out[target_base + out_col as usize] = written;
    }
    let _ = n_cells;
    out
}

pub fn cpu_sweep_iters(
    values: &[f32],
    width: u32,
    height: u32,
    n_dims: u32,
    out_col: u32,
    gather: &GatherTable,
    program: &IrProgram,
    dest_cell: Option<u32>,
    iterations: u32,
) -> Vec<f32> {
    let mut cur = values.to_vec();
    for _ in 0..iterations {
        cur = cpu_sweep_once(
            &cur, width, height, n_dims, out_col, gather, program, dest_cell,
        );
    }
    cur
}

pub fn bits_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.to_bits() == y.to_bits())
}

pub fn max_ulp_diff(a: &[f32], b: &[f32]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x.to_bits().abs_diff(y.to_bits()))
        .max()
        .unwrap_or(0)
}

pub fn median_f64(mut xs: Vec<f64>) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = xs.len() / 2;
    if xs.len() % 2 == 0 {
        (xs[mid - 1] + xs[mid]) / 2.0
    } else {
        xs[mid]
    }
}

pub fn worst_f64(xs: &[f64]) -> f64 {
    xs.iter().cloned().fold(0.0, f64::max)
}

/// Honest counter surface: stall/SM-occupancy/memory counters are not available on public doors.
pub fn counter_surface_report(timestamp_supported: bool) -> (String, String) {
    let stall = "UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)";
    let status = if timestamp_supported {
        "STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;memory_shadow_not_inferred_from_timing)"
    } else {
        "STOP(required_stall_memory_counters_unavailable;timestamp_query_unsupported;memory_shadow_not_inferred_from_timing)"
    };
    (stall.to_string(), status.to_string())
}

pub struct ProbeGpuHarness {
    pub device: Device,
    pub queue: Queue,
    pub adapter_name: String,
    pub backend: String,
    pub timestamp_supported: bool,
    pipeline: ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl ProbeGpuHarness {
    pub fn new_blocking() -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .context("no GPU adapter")?;
        let info = adapter.get_info();
        let timestamp_supported = adapter.features().contains(Features::TIMESTAMP_QUERY);
        let required_features = if timestamp_supported {
            Features::TIMESTAMP_QUERY
        } else {
            Features::empty()
        };
        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: Some("field_sweep_ir_probe"),
                required_features,
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
            },
            None,
        ))
        .context("request_device")?;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("field_sweep_ir_probe.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("field_sweep_ir_probe.wgsl").into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("fsir_bgl"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, false),
                storage_entry(2, true),
                storage_entry(3, true),
                storage_entry(4, true),
                storage_entry(5, true),
                storage_entry(6, true),
                uniform_entry(7),
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("fsir_pl"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("fsir_pipe"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            adapter_name: info.name,
            backend: format!("{:?}", info.backend),
            timestamp_supported,
            pipeline,
            bind_group_layout,
        })
    }

    pub fn run_sweep(
        &self,
        values: &[f32],
        gather: &GatherTable,
        program: &IrProgram,
        n_cells: u32,
        n_dims: u32,
        out_col: u32,
        dest_cell: Option<u32>,
        iterations: u32,
    ) -> Result<Vec<f32>> {
        if program.map.len() as u32 > PROBE_NODE_CAP
            || program.fold.len() as u32 > PROBE_NODE_CAP
            || program.post.len() as u32 > PROBE_NODE_CAP
        {
            bail!("program exceeds probe node cap {PROBE_NODE_CAP}");
        }
        let mut cur = values.to_vec();
        for _ in 0..iterations {
            cur = self.dispatch_once(
                &cur, gather, program, n_cells, n_dims, out_col, dest_cell,
            )?;
        }
        Ok(cur)
    }

    fn dispatch_once(
        &self,
        values: &[f32],
        gather: &GatherTable,
        program: &IrProgram,
        n_cells: u32,
        n_dims: u32,
        out_col: u32,
        dest_cell: Option<u32>,
    ) -> Result<Vec<f32>> {
        let in_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_in"),
            contents: bytemuck::cast_slice(values),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        });
        let out_size = (values.len() * std::mem::size_of::<f32>()) as u64;
        let out_buf = self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_out"),
            size: out_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue
            .write_buffer(&out_buf, 0, bytemuck::cast_slice(values));

        let ranges_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_ranges"),
            contents: bytemuck::cast_slice(&gather.ranges),
            usage: BufferUsages::STORAGE,
        });
        let neighbors_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_neighbors"),
            contents: bytemuck::cast_slice(&gather.neighbors),
            usage: BufferUsages::STORAGE,
        });
        let map_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_map"),
            contents: bytemuck::cast_slice(&program.map),
            usage: BufferUsages::STORAGE,
        });
        let fold_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_fold"),
            contents: bytemuck::cast_slice(&program.fold),
            usage: BufferUsages::STORAGE,
        });
        let post_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_post"),
            contents: bytemuck::cast_slice(&program.post),
            usage: BufferUsages::STORAGE,
        });
        let (fold_identity, fold_seed_from_target, fold_seed_col) = match program.fold_seed {
            FoldSeed::Const(v) => (v, 0u32, 0u32),
            FoldSeed::TargetValue(col) => (0.0, 1u32, col),
        };
        let params = SweepParams {
            n_cells,
            n_dims,
            out_col,
            dest_cell: dest_cell.unwrap_or(u32::MAX),
            force_dest_zero: if dest_cell.is_some() { 1 } else { 0 },
            map_root: program.map.len().saturating_sub(1) as u32,
            fold_root: program.fold.len().saturating_sub(1) as u32,
            post_root: program.post.len().saturating_sub(1) as u32,
            fold_identity,
            fold_seed_from_target,
            fold_seed_col,
            _pad0: 0,
        };
        let params_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("fsir_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("fsir_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: in_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: out_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: ranges_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: neighbors_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: map_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: fold_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: post_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: params_buf.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("fsir_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("fsir_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            let groups = (n_cells + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
            pass.dispatch_workgroups(groups, 1, 1);
        }
        let readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_rb"),
            size: out_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(&out_buf, 0, &readback, 0, out_size);
        self.queue.submit(Some(encoder.finish()));
        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(Maintain::Wait);
        rx.recv().context("map_async channel")??;
        let data = slice.get_mapped_range();
        let out: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        readback.unmap();
        Ok(out)
    }

    pub fn time_sweep_us(
        &self,
        values: &[f32],
        gather: &GatherTable,
        program: &IrProgram,
        n_cells: u32,
        n_dims: u32,
        out_col: u32,
        dest_cell: Option<u32>,
        iterations: u32,
        warm: usize,
        samples: usize,
    ) -> Result<Vec<f64>> {
        for _ in 0..warm {
            let _ = self.run_sweep(
                values, gather, program, n_cells, n_dims, out_col, dest_cell, iterations,
            )?;
        }
        let mut times = Vec::with_capacity(samples);
        for _ in 0..samples {
            let t0 = Instant::now();
            let _ = self.run_sweep(
                values, gather, program, n_cells, n_dims, out_col, dest_cell, iterations,
            )?;
            times.push(t0.elapsed().as_secs_f64() * 1_000_000.0);
        }
        Ok(times)
    }
}

fn storage_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::COMPUTE,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn threshold_verdict(median_ratio: f64, worst_ratio: f64) -> (&'static str, &'static str) {
    const MEDIAN_CAP: f64 = 1.25;
    const WORST_CAP: f64 = 1.5;
    if median_ratio <= MEDIAN_CAP && worst_ratio <= WORST_CAP {
        (
            "THRESHOLD-MET",
            "generic within median≤1.25× and worst≤1.5× at matched work occupancy",
        )
    } else {
        (
            "ROUTE-SPECIALIZATION/JIT",
            "threshold miss — IR retained as specification; bespoke kernels are not final architecture",
        )
    }
}
