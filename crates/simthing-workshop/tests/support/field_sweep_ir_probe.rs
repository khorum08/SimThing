//! FIELD-SWEEP-IR-PROBE-0 — integration-test-only support (not a workshop library API).
//!
//! Compiled solely by `field_sweep_ir_probe_0` tests. No `pub mod` in `simthing-workshop` lib.
//! Disposable: birth track 0.0.8.7 envelope, dsu_survivals=0, no unbounded residue target.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use simthing_core::{EML_STACK_MAX, MAX_EML_TREE_NODES};
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Limits, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderStages,
};

pub const PROBE_SCRATCH_CAPACITY: u32 = 32;
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
    pub fold_seed_from_target: u32,
    pub fold_seed_col: u32,
    pub _pad0: u32,
}

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
    /// Peak live operand-stack depth under postfix lowering of the trees (not scratch slots).
    pub actual_peak_operand_stack: u32,
    pub configured_scratch_capacity: u32,
    pub column_reads_per_edge: u32,
    /// Runtime evaluator materializes a scratch[i] per node (DAG), not a stack machine.
    pub runtime_eval_model: &'static str,
}

#[derive(Clone, Debug)]
pub struct EmlCapFacts {
    pub configured_max_tree_nodes: u32,
    pub configured_stack_max: u32,
    pub probe_node_cap: u32,
    pub probe_scratch_capacity: u32,
    /// max(map, fold, post) — compared to `MAX_EML_TREE_NODES` (per expression tree).
    pub observed_max_tree_nodes: u32,
    /// map+fold+post sum — descriptive composition only; never compared to the per-tree cap.
    pub observed_total_program_nodes: u32,
    pub observed_peak_operand_stack: u32,
    pub resource_class_label: &'static str,
}

#[derive(Clone, Debug)]
pub struct MeasurementRow {
    pub case_name: String,
    pub adapter_backend: String,
    pub adjacency_kind: String,
    pub theater_size: String,
    pub degree_distribution: String,
    pub map_nodes: u32,
    pub fold_nodes: u32,
    pub post_nodes: u32,
    pub actual_peak_operand_stack: u32,
    pub configured_scratch_capacity: u32,
    pub column_reads_per_edge: u32,
    pub resource_class: String,
    pub matched_occupancy: String,
    pub matched_work_basis: String,
    pub dispatch_count_per_sample: u32,
    pub warmup_count: usize,
    pub sample_count: usize,
    pub dispatch_time_us_median: f64,
    pub dispatch_time_us_worst: f64,
    pub e2e_time_us_median: f64,
    pub e2e_time_us_worst: f64,
    pub edges_per_s_dispatch_median: f64,
    pub stall_memory_counters: String,
    pub counter_surface_status: String,
    pub path_kind: String,
    pub timing_note: String,
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

fn is_leaf(op: u32) -> bool {
    matches!(
        op,
        OP_CONST | OP_TARGET_VALUE | OP_NEIGHBOR_VALUE | OP_ACC | OP_MAPPED | OP_FOLDED
    )
}

fn is_unary(op: u32) -> bool {
    op == OP_CLAMP01
}

fn is_binary(op: u32) -> bool {
    matches!(op, OP_ADD | OP_SUB | OP_MUL | OP_MIN | OP_MAX | OP_DIV)
}

/// Peak live operand-stack depth when `nodes[0..=root]` is lowered post-order to a stack machine.
///
/// Distinct from the runtime scratch-indexed evaluator (`scratch[i]` per node). Planted left-fold
/// chains have high node counts and peak stack 2 — proves this is not `nodes.len()`.
pub fn peak_operand_stack_depth(nodes: &[IrNode], root: usize) -> u32 {
    if nodes.is_empty() {
        return 0;
    }
    let root = root.min(nodes.len() - 1);
    let mut sp = 0u32;
    let mut max_sp = 0u32;
    fn walk(nodes: &[IrNode], i: usize, sp: &mut u32, max_sp: &mut u32) {
        let n = nodes[i];
        if is_leaf(n.op) {
            *sp += 1;
            *max_sp = (*max_sp).max(*sp);
            return;
        }
        if is_unary(n.op) {
            walk(nodes, n.a as usize, sp, max_sp);
            // pop1/push1 — net zero
            return;
        }
        if is_binary(n.op) {
            walk(nodes, n.a as usize, sp, max_sp);
            walk(nodes, n.b as usize, sp, max_sp);
            // pop2/push1
            *sp = sp.saturating_sub(1);
            *max_sp = (*max_sp).max(*sp);
            return;
        }
        *sp += 1;
        *max_sp = (*max_sp).max(*sp);
    }
    walk(nodes, root, &mut sp, &mut max_sp);
    max_sp
}

/// Planted program: left-fold `((((c0+c1)+c2)+c3)+c4)` — 9 nodes, peak operand stack = 2.
pub fn planted_left_fold_stack_probe() -> (IrProgram, u32, u32) {
    let map = vec![
        node(OP_CONST, 0, 0, 0, 1.0), // 0
        node(OP_CONST, 0, 0, 0, 2.0), // 1
        node(OP_ADD, 0, 1, 0, 0.0),   // 2
        node(OP_CONST, 0, 0, 0, 3.0), // 3
        node(OP_ADD, 2, 3, 0, 0.0),   // 4
        node(OP_CONST, 0, 0, 0, 4.0), // 5
        node(OP_ADD, 4, 5, 0, 0.0),   // 6
        node(OP_CONST, 0, 0, 0, 5.0), // 7
        node(OP_ADD, 6, 7, 0, 0.0),   // 8
    ];
    let node_count = map.len() as u32;
    let peak = peak_operand_stack_depth(&map, map.len() - 1);
    let prog = IrProgram {
        map,
        fold: vec![node(OP_MAPPED, 0, 0, 0, 0.0)],
        post: vec![node(OP_FOLDED, 0, 0, 0, 0.0)],
        fold_seed: FoldSeed::Const(0.0),
    };
    (prog, node_count, peak)
}

pub fn program_metrics(program: &IrProgram) -> ProgramMetrics {
    let column_reads = program
        .map
        .iter()
        .filter(|n| n.op == OP_TARGET_VALUE || n.op == OP_NEIGHBOR_VALUE)
        .count() as u32;
    let map_peak = if program.map.is_empty() {
        0
    } else {
        peak_operand_stack_depth(&program.map, program.map.len() - 1)
    };
    let fold_peak = if program.fold.is_empty() {
        0
    } else {
        peak_operand_stack_depth(&program.fold, program.fold.len() - 1)
    };
    let post_peak = if program.post.is_empty() {
        0
    } else {
        peak_operand_stack_depth(&program.post, program.post.len() - 1)
    };
    ProgramMetrics {
        map_nodes: program.map.len() as u32,
        fold_nodes: program.fold.len() as u32,
        post_nodes: program.post.len() as u32,
        total_nodes: (program.map.len() + program.fold.len() + program.post.len()) as u32,
        actual_peak_operand_stack: map_peak.max(fold_peak).max(post_peak),
        configured_scratch_capacity: PROBE_SCRATCH_CAPACITY,
        column_reads_per_edge: column_reads,
        runtime_eval_model: "scratch_indexed_dag",
    }
}

pub fn live_eml_cap_facts(
    observed_max_tree_nodes: u32,
    observed_total_program_nodes: u32,
    observed_peak_stack: u32,
) -> EmlCapFacts {
    EmlCapFacts {
        configured_max_tree_nodes: MAX_EML_TREE_NODES,
        configured_stack_max: EML_STACK_MAX,
        probe_node_cap: PROBE_NODE_CAP,
        probe_scratch_capacity: PROBE_SCRATCH_CAPACITY,
        observed_max_tree_nodes,
        observed_total_program_nodes,
        observed_peak_operand_stack: observed_peak_stack,
        resource_class_label: RESOURCE_CLASS_LABEL,
    }
}

impl MeasurementRow {
    /// One complete machine-readable row for evidence transcription.
    pub fn to_tsv_line(&self) -> String {
        format!(
            "FSIR_ROW\tcase={}\tpath={}\tadapter_backend={}\tadjacency={}\ttheater={}\tdegree_distribution={}\tmap_nodes={}\tfold_nodes={}\tpost_nodes={}\tpeak_operand_stack={}\tscratch_capacity={}\tcolumn_reads_per_edge={}\tresource_class={}\toccupancy={}\tmatched_work_basis={}\tdispatch_count={}\twarmup={}\tsamples={}\tdispatch_med_us={:.3}\tdispatch_worst_us={:.3}\te2e_med_us={:.3}\te2e_worst_us={:.3}\tedges_per_s_dispatch_med={:.6e}\tstall_memory={}\tcounter_status={}\ttiming_note={}",
            self.case_name,
            self.path_kind,
            self.adapter_backend,
            self.adjacency_kind,
            self.theater_size,
            self.degree_distribution,
            self.map_nodes,
            self.fold_nodes,
            self.post_nodes,
            self.actual_peak_operand_stack,
            self.configured_scratch_capacity,
            self.column_reads_per_edge,
            self.resource_class,
            self.matched_occupancy,
            self.matched_work_basis,
            self.dispatch_count_per_sample,
            self.warmup_count,
            self.sample_count,
            self.dispatch_time_us_median,
            self.dispatch_time_us_worst,
            self.e2e_time_us_median,
            self.e2e_time_us_worst,
            self.edges_per_s_dispatch_median,
            self.stall_memory_counters,
            self.counter_surface_status,
            self.timing_note,
        )
    }
}

pub fn program_min_x_input_list(d_col: u32, w_col: u32) -> IrProgram {
    let map = vec![node(OP_NEIGHBOR_VALUE, d_col, 0, 0, 0.0)];
    let fold = vec![
        node(OP_ACC, 0, 0, 0, 0.0),
        node(OP_MAPPED, 0, 0, 0, 0.0),
        node(OP_MIN, 0, 1, 0, 0.0),
    ];
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

pub fn program_product_conductance(u_col: u32, u_sat: f32, chi: f32) -> IrProgram {
    let map = vec![
        node(OP_NEIGHBOR_VALUE, u_col, 0, 0, 0.0),
        node(OP_CONST, 0, 0, 0, u_sat),
        node(OP_DIV, 0, 1, 0, 0.0),
        node(OP_CONST, 0, 0, 0, 0.0),
        node(OP_MAX, 2, 3, 0, 0.0),
        node(OP_CONST, 0, 0, 0, 1.0),
        node(OP_MIN, 4, 5, 0, 0.0),
        node(OP_CONST, 0, 0, 0, 1.0),
        node(OP_SUB, 7, 6, 0, 0.0),
    ];
    let fold = vec![
        node(OP_ACC, 0, 0, 0, 0.0),
        node(OP_MAPPED, 0, 0, 0, 0.0),
        node(OP_MUL, 0, 1, 0, 0.0),
    ];
    let post = vec![node(OP_FOLDED, 0, 0, 0, 0.0)];
    IrProgram {
        map,
        fold,
        post,
        fold_seed: FoldSeed::Const(chi),
    }
}

pub fn program_banded_flux(u_col: u32, c_col: u32) -> IrProgram {
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
    let post = vec![node(OP_FOLDED, 0, 0, 0, 0.0)];
    IrProgram {
        map,
        fold,
        post,
        fold_seed: FoldSeed::TargetValue(u_col),
    }
}

pub const N4_OFFSETS_WENS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub const N4_OFFSETS_NSEW: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
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

fn eval_nodes(
    nodes: &[IrNode],
    root: usize,
    target_base: usize,
    neighbor_base: Option<usize>,
    values: &[f32],
    acc: f32,
    mapped: f32,
    folded: f32,
) -> f32 {
    let mut scratch = [0.0f32; PROBE_SCRATCH_CAPACITY as usize];
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
            0.0,
            0.0,
            acc,
        );
        out[target_base + out_col as usize] = written;
    }
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

pub fn counter_surface_report(timestamp_supported: bool) -> (String, String) {
    let stall = "UNAVAILABLE(no_public_stall_or_sm_occupancy_or_memory_counter_door)";
    let status = if timestamp_supported {
        "STOP(required_stall_memory_counters_unavailable;timestamp_query_available_timing_only;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)"
    } else {
        "STOP(required_stall_memory_counters_unavailable;timestamp_query_unsupported;occupancy_UNMEASURED;memory_shadow_not_inferred_from_timing)"
    };
    (stall.to_string(), status.to_string())
}

/// Threshold adjudication is only lawful when occupancy is measured and envelopes match.
/// With occupancy UNMEASURED / counter STOP, timing is diagnostic only.
pub fn threshold_adjudication_status(
    occupancy_measured: bool,
    envelope_matched: bool,
) -> &'static str {
    if !occupancy_measured {
        "DIAGNOSTIC_ONLY(occupancy_UNMEASURED;no_threshold_verdict)"
    } else if !envelope_matched {
        "DIAGNOSTIC_ONLY(envelope_unmatched;no_threshold_verdict)"
    } else {
        "THRESHOLD_ADMISSIBLE"
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

/// Persistent GPU session: one pipeline + reusable ping-pong buffers (matched envelope).
pub struct ProbeGpuSession {
    pub device: Device,
    pub queue: Queue,
    pub adapter_name: String,
    pub backend: String,
    pub timestamp_supported: bool,
    pipeline: ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    input: Option<Buffer>,
    output: Option<Buffer>,
    ranges: Option<Buffer>,
    neighbors: Option<Buffer>,
    map_nodes: Option<Buffer>,
    fold_nodes: Option<Buffer>,
    post_nodes: Option<Buffer>,
    params: Option<Buffer>,
    n_cells: u32,
    n_dims: u32,
    values_len: usize,
    read_input: bool,
}

impl ProbeGpuSession {
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
                label: Some("field_sweep_ir_probe_test"),
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
            input: None,
            output: None,
            ranges: None,
            neighbors: None,
            map_nodes: None,
            fold_nodes: None,
            post_nodes: None,
            params: None,
            n_cells: 0,
            n_dims: 0,
            values_len: 0,
            read_input: true,
        })
    }

    pub fn configure(
        &mut self,
        n_cells: u32,
        n_dims: u32,
        gather: &GatherTable,
        program: &IrProgram,
    ) -> Result<()> {
        if program.map.len() as u32 > PROBE_NODE_CAP
            || program.fold.len() as u32 > PROBE_NODE_CAP
            || program.post.len() as u32 > PROBE_NODE_CAP
        {
            bail!("program exceeds probe node cap {PROBE_NODE_CAP}");
        }
        let values_len = (n_cells * n_dims) as usize;
        let bytes = (values_len * std::mem::size_of::<f32>()) as u64;
        self.n_cells = n_cells;
        self.n_dims = n_dims;
        self.values_len = values_len;
        self.read_input = true;

        self.input = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_in"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        self.output = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_out"),
            size: bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        self.ranges = Some(
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("fsir_ranges"),
                    contents: bytemuck::cast_slice(&gather.ranges),
                    usage: BufferUsages::STORAGE,
                }),
        );
        self.neighbors = Some(
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("fsir_neighbors"),
                    contents: bytemuck::cast_slice(&gather.neighbors),
                    usage: BufferUsages::STORAGE,
                }),
        );
        // Fixed node capacity so C→flux program swaps never overflow.
        let node_bytes = (PROBE_NODE_CAP as usize * std::mem::size_of::<IrNode>()) as u64;
        self.map_nodes = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_map"),
            size: node_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        self.fold_nodes = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_fold"),
            size: node_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        self.post_nodes = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_post"),
            size: node_bytes,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        self.upload_program(program);
        self.params = Some(self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_params"),
            size: std::mem::size_of::<SweepParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
        Ok(())
    }

    fn upload_program(&self, program: &IrProgram) {
        self.queue.write_buffer(
            self.map_nodes.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&program.map),
        );
        self.queue.write_buffer(
            self.fold_nodes.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&program.fold),
        );
        self.queue.write_buffer(
            self.post_nodes.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(&program.post),
        );
    }

    fn write_params(&self, program: &IrProgram, out_col: u32, dest_cell: Option<u32>) {
        let (fold_identity, fold_seed_from_target, fold_seed_col) = match program.fold_seed {
            FoldSeed::Const(v) => (v, 0u32, 0u32),
            FoldSeed::TargetValue(col) => (0.0, 1u32, col),
        };
        let params = SweepParams {
            n_cells: self.n_cells,
            n_dims: self.n_dims,
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
        self.queue.write_buffer(
            self.params.as_ref().unwrap(),
            0,
            bytemuck::bytes_of(&params),
        );
    }

    pub fn upload_values(&mut self, values: &[f32]) -> Result<()> {
        if values.len() != self.values_len {
            bail!("values len mismatch");
        }
        self.queue.write_buffer(
            self.input.as_ref().unwrap(),
            0,
            bytemuck::cast_slice(values),
        );
        self.read_input = true;
        Ok(())
    }

    fn bind_group(&self, src: &Buffer, dst: &Buffer) -> wgpu::BindGroup {
        self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("fsir_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: src.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: dst.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.ranges.as_ref().unwrap().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: self.neighbors.as_ref().unwrap().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: self.map_nodes.as_ref().unwrap().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: self.fold_nodes.as_ref().unwrap().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: self.post_nodes.as_ref().unwrap().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: self.params.as_ref().unwrap().as_entire_binding(),
                },
            ],
        })
    }

    /// GPU-resident ping-pong; no map/readback. Returns dispatch count.
    pub fn dispatch_iters(
        &mut self,
        program: &IrProgram,
        out_col: u32,
        dest_cell: Option<u32>,
        iterations: u32,
    ) -> Result<u32> {
        self.write_params(program, out_col, dest_cell);
        self.upload_program(program);

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("fsir_dispatch"),
            });
        for _ in 0..iterations {
            let (src, dst) = if self.read_input {
                (self.input.as_ref().unwrap(), self.output.as_ref().unwrap())
            } else {
                (self.output.as_ref().unwrap(), self.input.as_ref().unwrap())
            };
            let bg = self.bind_group(src, dst);
            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("fsir_pass"),
                    timestamp_writes: None,
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bg, &[]);
                let groups = (self.n_cells + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
                pass.dispatch_workgroups(groups, 1, 1);
            }
            self.read_input = !self.read_input;
        }
        self.queue.submit(Some(encoder.finish()));
        self.device.poll(Maintain::Wait);
        Ok(iterations)
    }

    /// Two-stage conductance→flux without CPU intermediate. Returns total dispatches.
    pub fn dispatch_c_then_flux(
        &mut self,
        prog_c: &IrProgram,
        prog_flux: &IrProgram,
        gather_for_flux: bool,
    ) -> Result<u32> {
        let _ = gather_for_flux;
        // Stage C writes col1; stage flux writes col0; both GPU-resident.
        let d1 = self.dispatch_iters(prog_c, 1, None, 1)?;
        // After odd iters, data is in output; dispatch_iters toggled read_input.
        // Next dispatch uses current side as src — already correct via read_input flag.
        let d2 = self.dispatch_iters(prog_flux, 0, None, 1)?;
        Ok(d1 + d2)
    }

    pub fn readback(&self) -> Result<Vec<f32>> {
        let src = if self.read_input {
            self.input.as_ref().unwrap()
        } else {
            self.output.as_ref().unwrap()
        };
        let bytes = (self.values_len * std::mem::size_of::<f32>()) as u64;
        let staging = self.device.create_buffer(&BufferDescriptor {
            label: Some("fsir_rb"),
            size: bytes,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("fsir_rb_enc"),
            });
        encoder.copy_buffer_to_buffer(src, 0, &staging, 0, bytes);
        self.queue.submit(Some(encoder.finish()));
        let slice = staging.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        self.device.poll(Maintain::Wait);
        rx.recv().context("map_async")??;
        let data = slice.get_mapped_range();
        let out: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging.unmap();
        Ok(out)
    }

    /// Upload + GPU-resident dispatch (no readback). Timed region for matched envelope.
    pub fn time_dispatch_us(
        &mut self,
        values: &[f32],
        program: &IrProgram,
        out_col: u32,
        dest_cell: Option<u32>,
        iterations: u32,
        warm: usize,
        samples: usize,
    ) -> Result<(Vec<f64>, Vec<f64>)> {
        for _ in 0..warm {
            self.upload_values(values)?;
            self.dispatch_iters(program, out_col, dest_cell, iterations)?;
        }
        let mut dispatch_times = Vec::with_capacity(samples);
        let mut e2e_times = Vec::with_capacity(samples);
        for _ in 0..samples {
            let e2e0 = Instant::now();
            self.upload_values(values)?;
            let d0 = Instant::now();
            self.dispatch_iters(program, out_col, dest_cell, iterations)?;
            dispatch_times.push(d0.elapsed().as_secs_f64() * 1_000_000.0);
            e2e_times.push(e2e0.elapsed().as_secs_f64() * 1_000_000.0);
        }
        Ok((dispatch_times, e2e_times))
    }

    pub fn time_c_then_flux_us(
        &mut self,
        values: &[f32],
        prog_c: &IrProgram,
        prog_flux: &IrProgram,
        warm: usize,
        samples: usize,
    ) -> Result<(Vec<f64>, Vec<f64>)> {
        for _ in 0..warm {
            self.upload_values(values)?;
            self.dispatch_c_then_flux(prog_c, prog_flux, true)?;
        }
        let mut dispatch_times = Vec::with_capacity(samples);
        let mut e2e_times = Vec::with_capacity(samples);
        for _ in 0..samples {
            let e2e0 = Instant::now();
            self.upload_values(values)?;
            let d0 = Instant::now();
            self.dispatch_c_then_flux(prog_c, prog_flux, true)?;
            dispatch_times.push(d0.elapsed().as_secs_f64() * 1_000_000.0);
            e2e_times.push(e2e0.elapsed().as_secs_f64() * 1_000_000.0);
        }
        Ok((dispatch_times, e2e_times))
    }
}
