//! Overlay order-band semantics spike — current ordered apply vs compiled bands.

use std::time::Instant;

use anyhow::{Context, Result};
use bytemuck::{Pod, Zeroable};
use serde::Serialize;
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderStages,
};

use crate::weighted_mean::TIMING_NOTE;

pub const OVERLAY_ADD: u32 = 0;
pub const OVERLAY_MUL: u32 = 1;
pub const OVERLAY_SET: u32 = 2;

pub const SOURCE_ANCESTOR: u32 = 0;
pub const SOURCE_LOCAL: u32 = 1;
pub const SOURCE_GLOBAL: u32 = 2;

pub const COMBINE_SUM: u32 = 0;
pub const COMBINE_PRODUCT: u32 = 1;
pub const COMBINE_LAST_BY_PRIORITY: u32 = 2;

pub const WORKGROUP_SIZE: u32 = 64;
pub const WARM_RUNS: usize = 10;
pub const STRICT_TOLERANCE: f32 = 0.0;
pub const LOOSE_TOLERANCE: f32 = 1e-6;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct OverlayDelta {
    pub slot: u32,
    pub col: u32,
    pub op: u32,
    pub source_kind: u32,
    pub order_band: u32,
    pub priority: u32,
    pub authored_order: u32,
    pub value: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq)]
pub struct CompiledOverlayOp {
    pub slot: u32,
    pub col: u32,
    pub combine_kind: u32,
    pub order_band: u32,
    pub value: f32,
    pub priority: u32,
    /// Monotonic emission order; used for apply ordering within a cell.
    pub sequence: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OverlayCellRange {
    pub start: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct OverlayOrderParams {
    pub n_slots: u32,
    pub n_cols: u32,
    pub n_values: u32,
    pub n_overlays: u32,
    pub n_compiled_ops: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[derive(Debug, Clone)]
pub struct OverlayOrderScenario {
    pub name: String,
    pub n_slots: usize,
    pub n_cols: usize,
    pub base_values: Vec<f32>,
    pub overlays: Vec<OverlayDelta>,
}

#[derive(Debug, Clone)]
pub struct CompiledOverlayScenario {
    pub compiled_ops: Vec<CompiledOverlayOp>,
    pub compile_stats: OverlayCompileStats,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverlayCompileStats {
    pub raw_overlay_count: usize,
    pub compiled_op_count: usize,
    pub compression_ratio: f32,
    pub add_count: usize,
    pub mul_count: usize,
    pub set_count: usize,
    pub band_count: usize,
    pub max_ops_per_slot_col: usize,
    pub unsafe_grouping_detected: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverlayOrderReport {
    pub scenario_name: String,
    pub n_slots: usize,
    pub n_cols: usize,
    pub n_values: usize,
    pub raw_overlay_count: usize,
    pub compiled_op_count: usize,
    pub compression_ratio: f32,
    pub band_count: usize,
    pub max_ops_per_slot_col: usize,
    pub cpu_current_us: u64,
    pub cpu_compile_us: u64,
    pub current_warm_mean_us: u64,
    pub current_warm_min_us: u64,
    pub current_warm_max_us: u64,
    pub pivot_warm_mean_us: u64,
    pub pivot_warm_min_us: u64,
    pub pivot_warm_max_us: u64,
    pub speedup_pivot_vs_current: f32,
    pub max_abs_error: f32,
    pub mean_abs_error: f32,
    pub bit_exact: bool,
    pub within_loose_tolerance: bool,
    pub current_deterministic: bool,
    pub pivot_deterministic: bool,
    pub semantic_gate: String,
    pub determinism_gate: String,
    pub performance_interpretation: String,
    pub risk_interpretation: String,
    pub timing_note: String,
}

pub struct OverlayOrderHarness {
    device: Device,
    queue: Queue,
    current_pipeline: ComputePipeline,
    pivot_pipeline: ComputePipeline,
    current_layout: wgpu::BindGroupLayout,
    pivot_layout: wgpu::BindGroupLayout,
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

fn warm_stats(samples: &[u64]) -> (u64, u64, u64) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let sum: u64 = samples.iter().sum();
    let mean = sum / samples.len() as u64;
    let min = *samples.iter().min().unwrap();
    let max = *samples.iter().max().unwrap();
    (mean, min, max)
}

fn f32_slices_identical(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.to_bits() == y.to_bits())
}

fn compare_errors(reference: &[f32], candidate: &[f32]) -> (f32, f32, bool) {
    let mut max_abs = 0.0f32;
    let mut sum_abs = 0.0f32;
    let mut bit_exact = true;
    for (r, c) in reference.iter().zip(candidate.iter()) {
        let err = (r - c).abs();
        max_abs = max_abs.max(err);
        sum_abs += err;
        if r.to_bits() != c.to_bits() {
            bit_exact = false;
        }
    }
    let mean_abs = if reference.is_empty() {
        0.0
    } else {
        sum_abs / reference.len() as f32
    };
    (max_abs, mean_abs, bit_exact)
}

/// Canonical overlay order: slot, col, order_band, priority, authored_order.
pub fn canonical_overlay_order(a: &OverlayDelta, b: &OverlayDelta) -> std::cmp::Ordering {
    a.slot
        .cmp(&b.slot)
        .then(a.col.cmp(&b.col))
        .then(a.order_band.cmp(&b.order_band))
        .then(a.priority.cmp(&b.priority))
        .then(a.authored_order.cmp(&b.authored_order))
}

pub fn sort_overlays_canonical(overlays: &mut [OverlayDelta]) {
    overlays.sort_by(canonical_overlay_order);
}

fn canonical_compiled_order(a: &CompiledOverlayOp, b: &CompiledOverlayOp) -> std::cmp::Ordering {
    a.slot
        .cmp(&b.slot)
        .then(a.col.cmp(&b.col))
        .then(a.sequence.cmp(&b.sequence))
}

fn pad_storage_bytes<T: Pod>(items: &[T]) -> Vec<u8> {
    let elem_size = std::mem::size_of::<T>();
    if items.is_empty() {
        vec![0u8; elem_size.max(4)]
    } else {
        bytemuck::cast_slice(items).to_vec()
    }
}

fn warm_runs_for(n_values: usize) -> usize {
    if n_values >= 100_000 {
        3
    } else {
        WARM_RUNS
    }
}

pub fn apply_single_overlay(value: f32, overlay: &OverlayDelta) -> f32 {
    match overlay.op {
        OVERLAY_ADD => value + overlay.value,
        OVERLAY_MUL => value * overlay.value,
        OVERLAY_SET => overlay.value,
        _ => value,
    }
}

pub fn apply_overlays_cpu_current(scenario: &OverlayOrderScenario) -> Vec<f32> {
    let mut values = scenario.base_values.clone();
    let mut overlays = scenario.overlays.clone();
    sort_overlays_canonical(&mut overlays);

    for overlay in &overlays {
        let idx = overlay.slot as usize * scenario.n_cols + overlay.col as usize;
        if idx < values.len() {
            values[idx] = apply_single_overlay(values[idx], overlay);
        }
    }
    values
}

struct CompileRun {
    op: u32,
    band: u32,
    priority: u32,
    sum_add: f32,
    product_mul: f32,
    set_value: f32,
    set_priority: u32,
    set_authored: u32,
}

impl CompileRun {
    fn flush(
        &self,
        slot: u32,
        col: u32,
        sequence: u32,
        out: &mut Vec<CompiledOverlayOp>,
        stats: &mut OverlayCompileStats,
    ) {
        match self.op {
            OVERLAY_ADD => {
                out.push(CompiledOverlayOp {
                    slot,
                    col,
                    combine_kind: COMBINE_SUM,
                    order_band: self.band,
                    value: self.sum_add,
                    priority: self.priority,
                    sequence,
                    _pad1: 0,
                });
                stats.add_count += 1;
            }
            OVERLAY_MUL => {
                out.push(CompiledOverlayOp {
                    slot,
                    col,
                    combine_kind: COMBINE_PRODUCT,
                    order_band: self.band,
                    value: self.product_mul,
                    priority: self.priority,
                    sequence,
                    _pad1: 0,
                });
                stats.mul_count += 1;
            }
            OVERLAY_SET => {
                out.push(CompiledOverlayOp {
                    slot,
                    col,
                    combine_kind: COMBINE_LAST_BY_PRIORITY,
                    order_band: self.band,
                    value: self.set_value,
                    priority: self.set_priority,
                    sequence,
                    _pad1: 0,
                });
                stats.set_count += 1;
            }
            _ => {}
        }
    }
}

fn compile_slot_col(
    slot: u32,
    col: u32,
    overlays: &[OverlayDelta],
    out: &mut Vec<CompiledOverlayOp>,
    stats: &mut OverlayCompileStats,
    next_sequence: &mut u32,
) {
    if overlays.is_empty() {
        return;
    }

    let mut sorted = overlays.to_vec();
    sorted.sort_by(|a, b| {
        a.order_band
            .cmp(&b.order_band)
            .then(a.priority.cmp(&b.priority))
            .then(a.authored_order.cmp(&b.authored_order))
    });

    let mut bands = std::collections::BTreeSet::new();
    for o in &sorted {
        bands.insert(o.order_band);
    }

    let mut current_band = sorted[0].order_band;
    let mut band_has_mixed_ops = false;
    let mut first_op_in_band = sorted[0].op;
    for w in sorted.windows(2) {
        if w[0].order_band == w[1].order_band && w[0].op != w[1].op {
            band_has_mixed_ops = true;
        }
        if w[1].order_band != current_band {
            if band_has_mixed_ops {
                stats.unsafe_grouping_detected = true;
            }
            current_band = w[1].order_band;
            band_has_mixed_ops = false;
            first_op_in_band = w[1].op;
        } else if w[1].op != first_op_in_band {
            band_has_mixed_ops = true;
        }
    }
    if band_has_mixed_ops {
        stats.unsafe_grouping_detected = true;
    }

    let mut run: Option<CompileRun> = None;

    for overlay in sorted {
        let extend = match &run {
            None => false,
            Some(r) => r.op == overlay.op && r.band == overlay.order_band,
        };

        if !extend {
            if let Some(r) = run.take() {
                r.flush(slot, col, *next_sequence, out, stats);
                *next_sequence += 1;
            }
            run = Some(match overlay.op {
                OVERLAY_ADD => CompileRun {
                    op: OVERLAY_ADD,
                    band: overlay.order_band,
                    priority: overlay.priority,
                    sum_add: overlay.value,
                    product_mul: 1.0,
                    set_value: 0.0,
                    set_priority: 0,
                    set_authored: 0,
                },
                OVERLAY_MUL => CompileRun {
                    op: OVERLAY_MUL,
                    band: overlay.order_band,
                    priority: overlay.priority,
                    sum_add: 0.0,
                    product_mul: overlay.value,
                    set_value: 0.0,
                    set_priority: 0,
                    set_authored: 0,
                },
                OVERLAY_SET => CompileRun {
                    op: OVERLAY_SET,
                    band: overlay.order_band,
                    priority: overlay.priority,
                    sum_add: 0.0,
                    product_mul: 1.0,
                    set_value: overlay.value,
                    set_priority: overlay.priority,
                    set_authored: overlay.authored_order,
                },
                _ => CompileRun {
                    op: overlay.op,
                    band: overlay.order_band,
                    priority: overlay.priority,
                    sum_add: 0.0,
                    product_mul: 1.0,
                    set_value: 0.0,
                    set_priority: 0,
                    set_authored: 0,
                },
            });
        } else if let Some(r) = run.as_mut() {
            match overlay.op {
                OVERLAY_ADD => r.sum_add += overlay.value,
                OVERLAY_MUL => r.product_mul *= overlay.value,
                OVERLAY_SET => {
                    let wins = overlay.priority > r.set_priority
                        || (overlay.priority == r.set_priority
                            && overlay.authored_order >= r.set_authored);
                    if wins {
                        r.set_value = overlay.value;
                        r.set_priority = overlay.priority;
                        r.set_authored = overlay.authored_order;
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(r) = run {
        r.flush(slot, col, *next_sequence, out, stats);
        *next_sequence += 1;
    }
}

pub fn compile_overlay_order_bands(scenario: &OverlayOrderScenario) -> CompiledOverlayScenario {
    let mut stats = OverlayCompileStats {
        raw_overlay_count: scenario.overlays.len(),
        compiled_op_count: 0,
        compression_ratio: 1.0,
        add_count: 0,
        mul_count: 0,
        set_count: 0,
        band_count: 0,
        max_ops_per_slot_col: 0,
        unsafe_grouping_detected: false,
    };

    let mut all_bands = std::collections::BTreeSet::new();
    for o in &scenario.overlays {
        all_bands.insert(o.order_band);
    }

    let n_values = scenario.n_slots * scenario.n_cols;
    let mut buckets: Vec<Vec<OverlayDelta>> = vec![Vec::new(); n_values];
    for overlay in &scenario.overlays {
        let idx = overlay.slot as usize * scenario.n_cols + overlay.col as usize;
        if idx < n_values {
            buckets[idx].push(*overlay);
        }
    }

    let mut compiled_ops = Vec::new();
    let mut next_sequence = 0u32;

    for idx in 0..n_values {
        let cell = &buckets[idx];
        stats.max_ops_per_slot_col = stats.max_ops_per_slot_col.max(cell.len());
        if cell.is_empty() {
            continue;
        }
        let slot = (idx / scenario.n_cols) as u32;
        let col = (idx % scenario.n_cols) as u32;
        compile_slot_col(
            slot,
            col,
            cell,
            &mut compiled_ops,
            &mut stats,
            &mut next_sequence,
        );
    }

    stats.band_count = all_bands.len();

    compiled_ops.sort_by(canonical_compiled_order);
    stats.compiled_op_count = compiled_ops.len();
    if stats.raw_overlay_count > 0 {
        stats.compression_ratio =
            stats.compiled_op_count as f32 / stats.raw_overlay_count as f32;
    }

    CompiledOverlayScenario {
        compiled_ops,
        compile_stats: stats,
    }
}

pub fn apply_compiled_overlays_cpu(
    scenario: &OverlayOrderScenario,
    compiled: &CompiledOverlayScenario,
) -> Vec<f32> {
    let mut values = scenario.base_values.clone();
    let mut ops = compiled.compiled_ops.clone();
    ops.sort_by(canonical_compiled_order);

    for op in &ops {
        let idx = op.slot as usize * scenario.n_cols + op.col as usize;
        if idx >= values.len() {
            continue;
        }
        values[idx] = match op.combine_kind {
            COMBINE_SUM => values[idx] + op.value,
            COMBINE_PRODUCT => values[idx] * op.value,
            COMBINE_LAST_BY_PRIORITY => op.value,
            _ => values[idx],
        };
    }
    values
}

pub fn make_manual_adversarial_scenario() -> OverlayOrderScenario {
    let n_slots = 8;
    let n_cols = 1;
    let base = 10.0f32;
    let base_values = vec![base; n_slots * n_cols];
    let mut overlays = Vec::new();
    let mut authored = 0u32;

    let mut push = |slot: u32, band: u32, op: u32, value: f32, priority: u32| {
        overlays.push(OverlayDelta {
            slot,
            col: 0,
            op,
            source_kind: SOURCE_LOCAL,
            order_band: band,
            priority,
            authored_order: authored,
            value,
        });
        authored += 1;
    };

    // case 0: Add +5 band 10, Mul *2 band 20 => 30
    push(0, 10, OVERLAY_ADD, 5.0, 0);
    push(0, 20, OVERLAY_MUL, 2.0, 0);

    // case 1: Mul *2 band 10, Add +5 band 20 => 25
    push(1, 10, OVERLAY_MUL, 2.0, 0);
    push(1, 20, OVERLAY_ADD, 5.0, 0);

    // case 2: Add +5 band 10, Set 3 band 20 => 3
    push(2, 10, OVERLAY_ADD, 5.0, 0);
    push(2, 20, OVERLAY_SET, 3.0, 0);

    // case 3: Set 3 band 10, Add +5 band 20 => 8
    push(3, 10, OVERLAY_SET, 3.0, 0);
    push(3, 20, OVERLAY_ADD, 5.0, 0);

    // case 4: Set 3 pri 1, Set 7 pri 2 same band => 7
    push(4, 10, OVERLAY_SET, 3.0, 1);
    push(4, 10, OVERLAY_SET, 7.0, 2);

    // case 5: Add +1, +2, +3 same band => base + 6
    push(5, 10, OVERLAY_ADD, 1.0, 0);
    push(5, 10, OVERLAY_ADD, 2.0, 0);
    push(5, 10, OVERLAY_ADD, 3.0, 0);

    // case 6: unsafe trap Add +5, Mul *2, Add +1 => 31
    push(6, 10, OVERLAY_ADD, 5.0, 0);
    push(6, 10, OVERLAY_MUL, 2.0, 0);
    push(6, 10, OVERLAY_ADD, 1.0, 0);

    OverlayOrderScenario {
        name: "manual_adversarial".to_string(),
        n_slots,
        n_cols,
        base_values,
        overlays,
    }
}

pub fn make_unsafe_grouping_trap_scenario() -> OverlayOrderScenario {
    let mut scenario = make_manual_adversarial_scenario();
    scenario.name = "unsafe_grouping_trap".to_string();
    scenario
}

fn push_overlay(
    overlays: &mut Vec<OverlayDelta>,
    slot: u32,
    col: u32,
    op: u32,
    source_kind: u32,
    order_band: u32,
    priority: u32,
    authored_order: u32,
    value: f32,
) {
    overlays.push(OverlayDelta {
        slot,
        col,
        op,
        source_kind,
        order_band,
        priority,
        authored_order,
        value,
    });
}

pub fn make_overlay_order_scenario(
    name: &str,
    n_slots: usize,
    n_cols: usize,
    overlays_per_value: usize,
    adversarial_mix: bool,
) -> OverlayOrderScenario {
    let n_values = n_slots * n_cols;
    let base_values: Vec<f32> = (0..n_values)
        .map(|idx| 10.0 + (idx % 17) as f32 * 0.25)
        .collect();

    let mut overlays = Vec::new();
    let mut authored = 0u32;

    if adversarial_mix && n_slots >= 7 && n_cols >= 1 {
        let manual = make_manual_adversarial_scenario();
        overlays.extend(manual.overlays.iter().map(|o| {
            authored = authored.max(o.authored_order + 1);
            *o
        }));
    }

    let bands = [10u32, 20, 30, 40, 50];
    let sources = [SOURCE_ANCESTOR, SOURCE_LOCAL, SOURCE_LOCAL, SOURCE_LOCAL, SOURCE_GLOBAL];

    for slot in 0..n_slots {
        for col in 0..n_cols {
            if adversarial_mix && slot < 7 && col == 0 {
                continue;
            }
            for j in 0..overlays_per_value {
                let band_idx = j % bands.len();
                let band = bands[band_idx];
                let source = sources[band_idx];
                let pattern = (slot * 31 + col * 17 + j * 7) % 11;
                let (op, value, priority) = match pattern {
                    0 => (OVERLAY_ADD, 0.5 + (j as f32 * 0.1), 0),
                    1 => (OVERLAY_MUL, 1.01 + (j as f32 * 0.02), 0),
                    2 => (OVERLAY_SET, 3.0 + (j as f32), j as u32),
                    3 if j > 0 && overlays_per_value > 2 => {
                        (OVERLAY_ADD, 1.0, 0)
                    }
                    4 if j > 0 => (OVERLAY_MUL, 0.99, 0),
                    5 => (OVERLAY_ADD, -0.25, 0),
                    6 => (OVERLAY_SET, 99.0, (j + 1) as u32),
                    _ => (OVERLAY_ADD, 0.25, 0),
                };
                push_overlay(
                    &mut overlays,
                    slot as u32,
                    col as u32,
                    op,
                    source,
                    band,
                    priority,
                    authored,
                    value,
                );
                authored += 1;
            }
        }
    }

    sort_overlays_canonical(&mut overlays);

    OverlayOrderScenario {
        name: name.to_string(),
        n_slots,
        n_cols,
        base_values,
        overlays,
    }
}

fn rebuild_contiguous_overlay_list(
    n_slots: usize,
    n_cols: usize,
    overlays: &[OverlayDelta],
) -> (Vec<OverlayDelta>, Vec<OverlayCellRange>) {
    let n_values = n_slots * n_cols;
    let mut buckets: Vec<Vec<OverlayDelta>> = vec![Vec::new(); n_values];
    for overlay in overlays {
        let idx = overlay.slot as usize * n_cols + overlay.col as usize;
        if idx < n_values {
            buckets[idx].push(*overlay);
        }
    }

    let mut flat = Vec::with_capacity(overlays.len());
    let mut ranges = vec![OverlayCellRange { start: 0, count: 0 }; n_values];
    for (idx, bucket) in buckets.iter_mut().enumerate() {
        bucket.sort_by(|a, b| {
            a.order_band
                .cmp(&b.order_band)
                .then(a.priority.cmp(&b.priority))
                .then(a.authored_order.cmp(&b.authored_order))
        });
        if bucket.is_empty() {
            continue;
        }
        ranges[idx] = OverlayCellRange {
            start: flat.len() as u32,
            count: bucket.len() as u32,
        };
        flat.extend(bucket.iter().copied());
    }
    (flat, ranges)
}

fn rebuild_contiguous_compiled_list(
    n_slots: usize,
    n_cols: usize,
    ops: &[CompiledOverlayOp],
) -> (Vec<CompiledOverlayOp>, Vec<OverlayCellRange>) {
    let n_values = n_slots * n_cols;
    let mut buckets: Vec<Vec<CompiledOverlayOp>> = vec![Vec::new(); n_values];
    for op in ops {
        let idx = op.slot as usize * n_cols + op.col as usize;
        if idx < n_values {
            buckets[idx].push(*op);
        }
    }

    let mut flat = Vec::with_capacity(ops.len());
    let mut ranges = vec![OverlayCellRange { start: 0, count: 0 }; n_values];
    for (idx, bucket) in buckets.iter_mut().enumerate() {
        bucket.sort_by_key(|op| op.sequence);
        if bucket.is_empty() {
            continue;
        }
        ranges[idx] = OverlayCellRange {
            start: flat.len() as u32,
            count: bucket.len() as u32,
        };
        flat.extend(bucket.iter().copied());
    }
    (flat, ranges)
}

impl OverlayOrderHarness {
    pub fn new() -> Result<Self> {
        pollster::block_on(Self::new_async())
    }

    async fn new_async() -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .context("no suitable GPU adapter found")?;

        let limits = adapter.limits();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("simthing-workshop overlay_order"),
                    required_features: Features::empty(),
                    required_limits: limits,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let current_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("overlay_order_current"),
            source: wgpu::ShaderSource::Wgsl(include_str!("overlay_order_current.wgsl").into()),
        });
        let pivot_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("overlay_order_pivot"),
            source: wgpu::ShaderSource::Wgsl(include_str!("overlay_order_pivot.wgsl").into()),
        });

        let current_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("overlay_order_current_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                uniform_entry(3),
                storage_entry(4, true),
            ],
        });
        let pivot_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("overlay_order_pivot_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                uniform_entry(3),
                storage_entry(4, true),
            ],
        });

        let current_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("overlay_order_current_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("overlay_order_current_pl"),
                bind_group_layouts: &[&current_layout],
                push_constant_ranges: &[],
            })),
            module: &current_shader,
            entry_point: "apply_current",
            compilation_options: Default::default(),
            cache: None,
        });

        let pivot_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("overlay_order_pivot_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("overlay_order_pivot_pl"),
                bind_group_layouts: &[&pivot_layout],
                push_constant_ranges: &[],
            })),
            module: &pivot_shader,
            entry_point: "apply_pivot",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            current_pipeline,
            pivot_pipeline,
            current_layout,
            pivot_layout,
        })
    }

    fn params(scenario: &OverlayOrderScenario, n_compiled_ops: u32) -> OverlayOrderParams {
        OverlayOrderParams {
            n_slots: scenario.n_slots as u32,
            n_cols: scenario.n_cols as u32,
            n_values: (scenario.n_slots * scenario.n_cols) as u32,
            n_overlays: scenario.overlays.len() as u32,
            n_compiled_ops,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }

    pub fn run_current(&self, scenario: &OverlayOrderScenario) -> Result<Vec<f32>> {
        let mut overlays = scenario.overlays.clone();
        sort_overlays_canonical(&mut overlays);
        let (overlays, cell_ranges) =
            rebuild_contiguous_overlay_list(scenario.n_slots, scenario.n_cols, &overlays);
        let n_values = scenario.n_slots * scenario.n_cols;
        let output_size = (n_values * std::mem::size_of::<f32>()) as u64;

        let base_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_base"),
            contents: bytemuck::cast_slice(&scenario.base_values),
            usage: BufferUsages::STORAGE,
        });
        let overlay_bytes = pad_storage_bytes(&overlays);
        let overlay_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_raw"),
            contents: &overlay_bytes,
            usage: BufferUsages::STORAGE,
        });
        let range_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_cell_ranges"),
            contents: bytemuck::cast_slice(&cell_ranges),
            usage: BufferUsages::STORAGE,
        });
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("overlay_order_current_out"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let params = Self::params(scenario, 0);
        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_current_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("overlay_order_current_bg"),
            layout: &self.current_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: base_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: overlay_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: range_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups = ((n_values as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("overlay_order_current_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("overlay_order_current_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.current_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        Self::readback(
            &self.device,
            &self.queue,
            encoder,
            &output_buffer,
            n_values,
        )
    }

    pub fn run_pivot(
        &self,
        scenario: &OverlayOrderScenario,
        compiled: &CompiledOverlayScenario,
    ) -> Result<Vec<f32>> {
        let (compiled_ops, cell_ranges) = rebuild_contiguous_compiled_list(
            scenario.n_slots,
            scenario.n_cols,
            &compiled.compiled_ops,
        );
        let n_values = scenario.n_slots * scenario.n_cols;
        let output_size = (n_values * std::mem::size_of::<f32>()) as u64;

        let base_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_pivot_base"),
            contents: bytemuck::cast_slice(&scenario.base_values),
            usage: BufferUsages::STORAGE,
        });
        let compiled_bytes = pad_storage_bytes(&compiled_ops);
        let compiled_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_compiled"),
            contents: &compiled_bytes,
            usage: BufferUsages::STORAGE,
        });
        let range_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_compiled_ranges"),
            contents: bytemuck::cast_slice(&cell_ranges),
            usage: BufferUsages::STORAGE,
        });
        let output_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("overlay_order_pivot_out"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let params = Self::params(scenario, compiled.compiled_ops.len() as u32);
        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("overlay_order_pivot_params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("overlay_order_pivot_bg"),
            layout: &self.pivot_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: base_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: compiled_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: range_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups = ((n_values as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("overlay_order_pivot_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("overlay_order_pivot_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pivot_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        Self::readback(
            &self.device,
            &self.queue,
            encoder,
            &output_buffer,
            n_values,
        )
    }

    fn readback(
        device: &Device,
        queue: &Queue,
        mut encoder: wgpu::CommandEncoder,
        output_buffer: &Buffer,
        n_values: usize,
    ) -> Result<Vec<f32>> {
        let output_size = (n_values * std::mem::size_of::<f32>()) as u64;
        let readback = device.create_buffer(&BufferDescriptor {
            label: Some("overlay_order_readback"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(output_buffer, 0, &readback, 0, output_size);
        queue.submit(Some(encoder.finish()));

        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        device.poll(Maintain::Wait);
        rx.recv()
            .context("map_async sender dropped")?
            .context("buffer map failed")?;

        let mapped = slice.get_mapped_range();
        let outputs: Vec<f32> =
            bytemuck::cast_slice(&mapped[..n_values * std::mem::size_of::<f32>()]).to_vec();
        drop(mapped);
        readback.unmap();
        Ok(outputs)
    }
}

fn performance_interpretation(speedup: f32) -> String {
    if speedup > 1.5 {
        "Pivot wins (>1.5x)".to_string()
    } else if speedup <= 1.0 {
        "Current competitive (pivot <= 1.0x)".to_string()
    } else {
        format!("Modest pivot advantage ({speedup:.2}x, 1.0–1.5x band)")
    }
}

fn risk_interpretation(stats: &OverlayCompileStats) -> String {
    let compression_note = if stats.compression_ratio < 0.9 {
        format!(
            "meaningful compression ({:.1}% of raw overlays)",
            stats.compression_ratio * 100.0
        )
    } else {
        "low compression — compiler mostly preserves per-overlay ops".to_string()
    };

    let unsafe_note = if stats.unsafe_grouping_detected {
        "mixed op kinds detected in bands — compiler remained conservative (unsafe_grouping_detected=true)"
    } else {
        "no unsafe grouping temptation detected"
    };

    format!("{compression_note}; {unsafe_note}")
}

fn semantic_classification(
    cpu_match: bool,
    gpu_pivot_ok: bool,
    stats: &OverlayCompileStats,
) -> String {
    if !cpu_match {
        return "FAIL".to_string();
    }
    if !gpu_pivot_ok {
        return "FAIL".to_string();
    }
    if stats.compression_ratio < 0.85 && stats.compression_ratio > 0.0 {
        "STRONG_PASS".to_string()
    } else if stats.compression_ratio <= 1.0 {
        "WEAK_PASS".to_string()
    } else {
        "WEAK_PASS".to_string()
    }
}

pub fn compare_overlay_order_rich(scenario: &OverlayOrderScenario) -> Result<OverlayOrderReport> {
    let harness = OverlayOrderHarness::new()?;
    compare_overlay_order_rich_with_harness(&harness, scenario)
}

pub fn compare_overlay_order_rich_with_harness(
    harness: &OverlayOrderHarness,
    scenario: &OverlayOrderScenario,
) -> Result<OverlayOrderReport> {
    let mut overlays = scenario.overlays.clone();
    sort_overlays_canonical(&mut overlays);
    let mut scenario_sorted = scenario.clone();
    scenario_sorted.overlays = overlays;

    let t0 = Instant::now();
    let cpu_current = apply_overlays_cpu_current(&scenario_sorted);
    let cpu_current_us = t0.elapsed().as_micros() as u64;

    let t1 = Instant::now();
    let compiled = compile_overlay_order_bands(&scenario_sorted);
    let cpu_compile_us = t1.elapsed().as_micros() as u64;

    let cpu_compiled = apply_compiled_overlays_cpu(&scenario_sorted, &compiled);
    let cpu_semantic_match = f32_slices_identical(&cpu_current, &cpu_compiled);

    let _ = harness.run_current(&scenario_sorted)?;
    let _ = harness.run_pivot(&scenario_sorted, &compiled)?;

    let n_values = scenario.n_slots * scenario.n_cols;
    let warm_runs = warm_runs_for(n_values);
    let mut current_warm = Vec::with_capacity(warm_runs);
    let mut pivot_warm = Vec::with_capacity(warm_runs);
    let mut current_base: Option<Vec<f32>> = None;
    let mut pivot_base: Option<Vec<f32>> = None;
    let mut current_deterministic = true;
    let mut pivot_deterministic = true;

    for _ in 0..warm_runs {
        let t = Instant::now();
        let cur = harness.run_current(&scenario_sorted)?;
        current_warm.push(t.elapsed().as_micros() as u64);
        match &current_base {
            None => current_base = Some(cur),
            Some(base) if !f32_slices_identical(&cur, base) => current_deterministic = false,
            _ => {}
        }

        let t = Instant::now();
        let piv = harness.run_pivot(&scenario_sorted, &compiled)?;
        pivot_warm.push(t.elapsed().as_micros() as u64);
        match &pivot_base {
            None => pivot_base = Some(piv),
            Some(base) if !f32_slices_identical(&piv, base) => pivot_deterministic = false,
            _ => {}
        }
    }

    let gpu_current = current_base.unwrap_or_default();
    let gpu_pivot = pivot_base.unwrap_or_default();

    let (_, _, current_bit_exact) = compare_errors(&cpu_current, &gpu_current);
    let (max_abs, mean_abs, pivot_bit_exact) = compare_errors(&cpu_current, &gpu_pivot);
    let within_loose = max_abs <= LOOSE_TOLERANCE;
    let gpu_pivot_ok = within_loose;

    let semantic_gate = if cpu_semantic_match && gpu_pivot_ok {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };
    let determinism_gate = if current_deterministic && pivot_deterministic {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };

    let (current_warm_mean_us, current_warm_min_us, current_warm_max_us) =
        warm_stats(&current_warm);
    let (pivot_warm_mean_us, pivot_warm_min_us, pivot_warm_max_us) = warm_stats(&pivot_warm);
    let speedup = if pivot_warm_mean_us > 0 {
        current_warm_mean_us as f32 / pivot_warm_mean_us as f32
    } else {
        0.0
    };

    let classification = semantic_classification(cpu_semantic_match, gpu_pivot_ok, &compiled.compile_stats);

    Ok(OverlayOrderReport {
        scenario_name: scenario.name.clone(),
        n_slots: scenario.n_slots,
        n_cols: scenario.n_cols,
        n_values: scenario.n_slots * scenario.n_cols,
        raw_overlay_count: compiled.compile_stats.raw_overlay_count,
        compiled_op_count: compiled.compile_stats.compiled_op_count,
        compression_ratio: compiled.compile_stats.compression_ratio,
        band_count: compiled.compile_stats.band_count,
        max_ops_per_slot_col: compiled.compile_stats.max_ops_per_slot_col,
        cpu_current_us,
        cpu_compile_us,
        current_warm_mean_us,
        current_warm_min_us,
        current_warm_max_us,
        pivot_warm_mean_us,
        pivot_warm_min_us,
        pivot_warm_max_us,
        speedup_pivot_vs_current: speedup,
        max_abs_error: max_abs,
        mean_abs_error: mean_abs,
        bit_exact: pivot_bit_exact && current_bit_exact,
        within_loose_tolerance: within_loose,
        current_deterministic,
        pivot_deterministic,
        semantic_gate,
        determinism_gate,
        performance_interpretation: performance_interpretation(speedup),
        risk_interpretation: format!(
            "{}; overlay_semantic_class={}",
            risk_interpretation(&compiled.compile_stats),
            classification
        ),
        timing_note: TIMING_NOTE.to_string(),
    })
}

pub fn format_overlay_order_report(report: &OverlayOrderReport) -> String {
    crate::overlay_order_report::format_overlay_order_report(report)
}

pub fn write_overlay_order_reports(report: &OverlayOrderReport) -> Result<()> {
    crate::overlay_order_report::write_overlay_order_reports(report).map_err(Into::into)
}

pub fn write_overlay_order_semantics_reports_bundle(
    reports: &[OverlayOrderReport],
) -> Result<()> {
    crate::overlay_order_report::write_overlay_order_semantics_reports_bundle(reports)
        .map_err(Into::into)
}
