//! Multi-target replay / delta logging spike — CPU boundary settlement vs GPU pivot.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use serde::Serialize;
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePipelineDescriptor, Device, DeviceDescriptor, Features,
    Instance, InstanceDescriptor, Maintain, MapMode, MemoryHints, PipelineLayoutDescriptor,
    PowerPreference, Queue, RequestAdapterOptions, ShaderModuleDescriptor, ShaderStages,
};

pub const WORKGROUP_SIZE: u32 = 64;
pub const WARM_RUNS: usize = 10;

pub const REPLAY_MODE_COMPACT: u32 = 0;
pub const REPLAY_MODE_FULL: u32 = 1;

pub const RESIDENT_TICKS: u32 = 64;
pub const RESIDENT_WARM_RUNS: usize = 5;

pub const RESIDENT_MODE_SUMMARY_ONLY: u32 = 0;
pub const RESIDENT_MODE_RECORDS: u32 = 1;

pub const TIMING_NOTE: &str =
    "Warm timings include upload, dispatch, wait, and readback; not pure shader timestamp time (upload/readback envelope mode).";

pub const RESIDENT_TIMING_NOTE: &str =
    "Resident total_validation timings include initial buffer upload, encode, submit, GPU wait, and final state/summary/records readback. submit timings cover encode+submit only. Neither is pure shader timestamp time.";

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct QueueState {
    pub source_pool: u32,
    pub queue_accum: u32,
    pub units: u32,
    pub _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct QueueParams {
    pub daily_request: u32,
    pub unit_cost: u32,
    pub is_active: u32,
    pub _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MultiTargetParams {
    pub n_items: u32,
    pub replay_mode: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct CompactDeltaRecord {
    pub item: u32,
    pub transfer_amount: u32,
    pub emit_count: u32,
    pub is_active: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct FullDeltaRecord {
    pub item: u32,
    pub source_before: u32,
    pub queue_before: u32,
    pub units_before: u32,
    pub source_after: u32,
    pub queue_after: u32,
    pub units_after: u32,
    pub transfer_amount: u32,
    pub emit_count: u32,
    pub is_active: u32,
    pub _pad0: u32,
}

#[derive(Debug, Clone)]
pub struct MultiTargetScenario {
    pub name: String,
    pub states: Vec<QueueState>,
    pub params: Vec<QueueParams>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MultiTargetReplayReport {
    pub scenario_name: String,
    pub n_items: usize,
    pub active_count: usize,
    pub unit_cost_min: u32,
    pub unit_cost_max: u32,
    pub total_source_before: u64,
    pub total_queue_before: u64,
    pub total_units_before: u64,
    pub total_source_after: u64,
    pub total_queue_after: u64,
    pub total_units_after: u64,
    pub total_transferred: u64,
    pub total_emitted_units: u64,
    pub total_emitted_value: u64,
    pub conservation_gate: String,
    pub replay_gate: String,
    pub determinism_gate: String,
    pub compact_record_gate: String,
    pub cpu_current_us: u64,
    pub gpu_compact_warm_mean_us: u64,
    pub gpu_compact_warm_min_us: u64,
    pub gpu_compact_warm_max_us: u64,
    pub gpu_full_warm_mean_us: u64,
    pub gpu_full_warm_min_us: u64,
    pub gpu_full_warm_max_us: u64,
    pub compact_record_bytes: usize,
    pub full_record_bytes: usize,
    pub compact_vs_full_record_ratio: f32,
    pub speedup_gpu_compact_vs_cpu: f32,
    pub final_state_matches_cpu: bool,
    pub replay_from_compact_matches_gpu: bool,
    pub repeated_gpu_outputs_identical: bool,
    pub interpretation: String,
    pub timing_note: String,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ResidentParams {
    pub n_items: u32,
    pub tick_index: u32,
    pub record_stride: u32,
    pub write_per_item_records: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct TickSummaryReadback {
    pub total_transferred: u32,
    pub total_emitted_units: u32,
    pub active_count: u32,
    pub reserved: u32,
}

#[derive(Debug, Clone)]
pub struct MultiTargetResidentResult {
    pub final_states: Vec<QueueState>,
    pub summaries: Vec<TickSummaryReadback>,
    pub compact_records: Vec<CompactDeltaRecord>,
    pub submit_us: u64,
    pub total_validation_us: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MultiTargetResidentReport {
    pub scenario_name: String,
    pub n_items: usize,
    pub active_count: usize,
    pub ticks: u32,
    pub record_ticks: u32,
    pub records_memory_fallback: bool,
    pub cpu_n_ticks_us: u64,
    pub gpu_resident_summary_submit_mean_us: u64,
    pub gpu_resident_summary_submit_min_us: u64,
    pub gpu_resident_summary_submit_max_us: u64,
    pub gpu_resident_summary_total_mean_us: u64,
    pub gpu_resident_summary_total_min_us: u64,
    pub gpu_resident_summary_total_max_us: u64,
    pub gpu_resident_records_submit_mean_us: u64,
    pub gpu_resident_records_submit_min_us: u64,
    pub gpu_resident_records_submit_max_us: u64,
    pub gpu_resident_records_total_mean_us: u64,
    pub gpu_resident_records_total_min_us: u64,
    pub gpu_resident_records_total_max_us: u64,
    pub gpu_resident_summary_total_per_tick_us: f32,
    pub gpu_resident_records_total_per_tick_us: f32,
    pub cpu_per_tick_us: f32,
    pub final_state_matches_cpu_summary_mode: bool,
    pub final_state_matches_cpu_records_mode: bool,
    pub summary_matches_cpu: bool,
    pub replay_from_records_matches_gpu: bool,
    pub resident_summary_speedup_vs_cpu: f32,
    pub resident_records_speedup_vs_cpu: f32,
    pub compact_record_bytes_total: usize,
    pub summary_bytes_total: usize,
    pub conservation_gate: String,
    pub replay_gate: String,
    pub determinism_gate: String,
    pub resident_performance_gate: String,
    pub interpretation: String,
    pub timing_note: String,
}

fn resolve_item(
    state: QueueState,
    params: QueueParams,
    item: u32,
) -> (QueueState, CompactDeltaRecord, FullDeltaRecord) {
    if params.is_active == 0 || params.unit_cost == 0 {
        let compact = CompactDeltaRecord {
            item,
            transfer_amount: 0,
            emit_count: 0,
            is_active: params.is_active,
        };
        let full = FullDeltaRecord {
            item,
            source_before: state.source_pool,
            queue_before: state.queue_accum,
            units_before: state.units,
            source_after: state.source_pool,
            queue_after: state.queue_accum,
            units_after: state.units,
            transfer_amount: 0,
            emit_count: 0,
            is_active: params.is_active,
            _pad0: 0,
        };
        return (state, compact, full);
    }

    let transfer = state.source_pool.min(params.daily_request);
    let source_after = state.source_pool - transfer;
    let queue_pre_emit = state.queue_accum + transfer;
    let emit_count = queue_pre_emit / params.unit_cost;
    let queue_after = queue_pre_emit - emit_count * params.unit_cost;
    let units_after = state.units + emit_count;

    let final_state = QueueState {
        source_pool: source_after,
        queue_accum: queue_after,
        units: units_after,
        _pad0: 0,
    };
    let compact = CompactDeltaRecord {
        item,
        transfer_amount: transfer,
        emit_count,
        is_active: params.is_active,
    };
    let full = FullDeltaRecord {
        item,
        source_before: state.source_pool,
        queue_before: state.queue_accum,
        units_before: state.units,
        source_after,
        queue_after,
        units_after,
        transfer_amount: transfer,
        emit_count,
        is_active: params.is_active,
        _pad0: 0,
    };
    (final_state, compact, full)
}

pub fn resolve_cpu_current(
    scenario: &MultiTargetScenario,
    full_records: bool,
) -> (
    Vec<QueueState>,
    Vec<CompactDeltaRecord>,
    Vec<FullDeltaRecord>,
) {
    let n_items = scenario.states.len();
    let mut final_states = Vec::with_capacity(n_items);
    let mut compact_records = Vec::with_capacity(n_items);
    let mut full_out = Vec::with_capacity(n_items);

    for item in 0..n_items {
        let (final_state, compact, full) =
            resolve_item(scenario.states[item], scenario.params[item], item as u32);
        final_states.push(final_state);
        compact_records.push(compact);
        if full_records {
            full_out.push(full);
        }
    }

    (final_states, compact_records, full_out)
}

pub fn replay_from_compact_records(
    initial: &[QueueState],
    params: &[QueueParams],
    records: &[CompactDeltaRecord],
) -> Result<Vec<QueueState>> {
    if initial.len() != params.len() {
        bail!("initial/params length mismatch");
    }
    if records.len() != initial.len() {
        bail!("records length mismatch");
    }

    let mut out = initial.to_vec();
    for record in records {
        let idx = record.item as usize;
        if idx >= out.len() {
            bail!("record item {} out of range", record.item);
        }
        if records[record.item as usize].item != record.item {
            bail!("record at slot {} has mismatched item id", record.item);
        }

        let p = params[idx];
        let before = out[idx];

        if p.is_active == 0 || p.unit_cost == 0 {
            if record.transfer_amount != 0 || record.emit_count != 0 {
                bail!("inactive record {} has non-zero deltas", record.item);
            }
            continue;
        }

        if record.transfer_amount > before.source_pool {
            bail!(
                "record {} transfer {} exceeds source {}",
                record.item,
                record.transfer_amount,
                before.source_pool
            );
        }

        let queue_pre_emit = before.queue_accum + record.transfer_amount;
        let emitted_value = record.emit_count as u64 * p.unit_cost as u64;
        if queue_pre_emit as u64 * 1 < emitted_value {
            bail!(
                "record {} emit value {} exceeds queue pre-emit {}",
                record.item,
                emitted_value,
                queue_pre_emit
            );
        }

        out[idx] = QueueState {
            source_pool: before.source_pool - record.transfer_amount,
            queue_accum: queue_pre_emit - record.emit_count * p.unit_cost,
            units: before.units + record.emit_count,
            _pad0: 0,
        };
    }
    Ok(out)
}

fn apply_compact_record(
    out: &mut [QueueState],
    params: &[QueueParams],
    record: CompactDeltaRecord,
) -> Result<()> {
    let idx = record.item as usize;
    if idx >= out.len() {
        bail!("record item {} out of range", record.item);
    }

    let p = params[idx];
    let before = out[idx];

    if p.is_active == 0 || p.unit_cost == 0 {
        if record.transfer_amount != 0 || record.emit_count != 0 {
            bail!("inactive record {} has non-zero deltas", record.item);
        }
        return Ok(());
    }

    if record.transfer_amount > before.source_pool {
        bail!(
            "record {} transfer {} exceeds source {}",
            record.item,
            record.transfer_amount,
            before.source_pool
        );
    }

    let queue_pre_emit = before.queue_accum + record.transfer_amount;
    let emitted_value = record.emit_count as u64 * p.unit_cost as u64;
    if (queue_pre_emit as u64) < emitted_value {
        bail!(
            "record {} emit value {} exceeds queue pre-emit {}",
            record.item,
            emitted_value,
            queue_pre_emit
        );
    }

    out[idx] = QueueState {
        source_pool: before.source_pool - record.transfer_amount,
        queue_accum: queue_pre_emit - record.emit_count * p.unit_cost,
        units: before.units + record.emit_count,
        _pad0: 0,
    };
    Ok(())
}

pub fn resolve_cpu_current_n_ticks(
    scenario: &MultiTargetScenario,
    ticks: u32,
) -> (
    Vec<QueueState>,
    Vec<TickSummaryReadback>,
    Vec<CompactDeltaRecord>,
) {
    let n_items = scenario.states.len();
    let mut states = scenario.states.clone();
    let mut summaries = Vec::with_capacity(ticks as usize);
    let mut all_records = Vec::with_capacity(ticks as usize * n_items);

    for _tick in 0..ticks {
        let mut tick_transfer = 0u32;
        let mut tick_emit = 0u32;
        let mut tick_active = 0u32;

        for item in 0..n_items {
            let (new_state, compact, _) =
                resolve_item(states[item], scenario.params[item], item as u32);
            states[item] = new_state;
            tick_transfer = tick_transfer.saturating_add(compact.transfer_amount);
            tick_emit = tick_emit.saturating_add(compact.emit_count);
            if scenario.params[item].is_active != 0 {
                tick_active += 1;
            }
            all_records.push(compact);
        }

        summaries.push(TickSummaryReadback {
            total_transferred: tick_transfer,
            total_emitted_units: tick_emit,
            active_count: tick_active,
            reserved: 0,
        });
    }

    (states, summaries, all_records)
}

pub fn replay_from_compact_records_n_ticks(
    initial: &[QueueState],
    params: &[QueueParams],
    records: &[CompactDeltaRecord],
    ticks: u32,
    n_items: usize,
) -> Result<Vec<QueueState>> {
    if initial.len() != params.len() {
        bail!("initial/params length mismatch");
    }
    if records.len() != ticks as usize * n_items {
        bail!(
            "records length {} expected {}",
            records.len(),
            ticks as usize * n_items
        );
    }

    let mut out = initial.to_vec();
    for tick in 0..ticks as usize {
        for item in 0..n_items {
            let idx = tick * n_items + item;
            let record = records[idx];
            if record.item as usize != item {
                bail!(
                    "record at tick {} slot {} has item id {}",
                    tick,
                    item,
                    record.item
                );
            }
            apply_compact_record(&mut out, params, record)?;
        }
    }
    Ok(out)
}

pub fn conservation_check(
    initial: &[QueueState],
    final_state: &[QueueState],
    params: &[QueueParams],
    records: &[CompactDeltaRecord],
) -> bool {
    if initial.len() != final_state.len()
        || initial.len() != params.len()
        || initial.len() != records.len()
    {
        return false;
    }

    let mut global_before = 0u64;
    let mut global_after = 0u64;

    for idx in 0..initial.len() {
        let before = initial[idx];
        let after = final_state[idx];
        let p = params[idx];
        let record = records[idx];

        if record.item as usize != idx {
            return false;
        }

        if p.is_active == 0 || p.unit_cost == 0 {
            if record.transfer_amount != 0 || record.emit_count != 0 {
                return false;
            }
            if before != after {
                return false;
            }
            global_before += before.source_pool as u64 + before.queue_accum as u64;
            global_after += after.source_pool as u64 + after.queue_accum as u64;
            continue;
        }

        if before.source_pool.saturating_sub(after.source_pool) != record.transfer_amount {
            return false;
        }
        if after.queue_accum
            != before
                .queue_accum
                .saturating_add(record.transfer_amount)
                .saturating_sub(record.emit_count.saturating_mul(p.unit_cost))
        {
            return false;
        }
        if after.units != before.units + record.emit_count {
            return false;
        }

        let emitted_value = record.emit_count as u64 * p.unit_cost as u64;
        let lhs = after.source_pool as u64 + after.queue_accum as u64 + emitted_value;
        let rhs = before.source_pool as u64 + before.queue_accum as u64;
        if lhs != rhs {
            return false;
        }

        global_before += before.source_pool as u64 + before.queue_accum as u64;
        global_after += after.source_pool as u64 + after.queue_accum as u64 + emitted_value;
    }

    global_before == global_after
}

pub fn make_manual_edge_case_scenario() -> MultiTargetScenario {
    let cases: [(QueueState, QueueParams); 10] = [
        (
            QueueState {
                source_pool: 0,
                queue_accum: 50,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 100,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 500,
                queue_accum: 10,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 0,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 100,
                queue_accum: 49,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 10,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 100,
                queue_accum: 50,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 10,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 100,
                queue_accum: 150,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 50,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 200,
                queue_accum: 40,
                units: 1,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 120,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 500,
                queue_accum: 0,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 250,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 1000,
                queue_accum: 0,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 500,
                unit_cost: 1,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 10,
                queue_accum: 0,
                units: 0,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 500,
                unit_cost: 50,
                is_active: 1,
                _pad0: 0,
            },
        ),
        (
            QueueState {
                source_pool: 100,
                queue_accum: 25,
                units: 3,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 50,
                unit_cost: 50,
                is_active: 0,
                _pad0: 0,
            },
        ),
    ];

    let mut states = Vec::new();
    let mut params = Vec::new();
    for (state, param) in cases {
        states.push(state);
        params.push(param);
    }

    MultiTargetScenario {
        name: "multitarget_manual_edge_cases".to_string(),
        states,
        params,
    }
}

pub fn make_depletion_n_tick_scenario() -> MultiTargetScenario {
    MultiTargetScenario {
        name: "multitarget_depletion_manual".to_string(),
        states: vec![
            QueueState {
                source_pool: 25,
                queue_accum: 0,
                units: 0,
                _pad0: 0,
            },
            QueueState {
                source_pool: 100,
                queue_accum: 0,
                units: 0,
                _pad0: 0,
            },
        ],
        params: vec![
            QueueParams {
                daily_request: 10,
                unit_cost: 3,
                is_active: 1,
                _pad0: 0,
            },
            QueueParams {
                daily_request: 50,
                unit_cost: 5,
                is_active: 0,
                _pad0: 0,
            },
        ],
    }
}

fn mix_u32(seed: u32) -> u32 {
    seed.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

pub fn make_multitarget_scenario(
    name: &str,
    n_items: usize,
    active_ratio: f32,
    bursty: bool,
) -> MultiTargetScenario {
    assert!(n_items > 0, "n_items must be > 0");
    let active_ratio = active_ratio.clamp(0.0, 1.0);

    let mut states = Vec::with_capacity(n_items);
    let mut params = Vec::with_capacity(n_items);

    for idx in 0..n_items {
        let h = mix_u32(idx as u32 + 1);
        let unit_cost = 10 + (h % 191);
        let source_pool = (h % 10_001) as u32;
        let queue_accum = ((h / 7) % (unit_cost.saturating_mul(3).max(1) + 1)) as u32;
        let units = ((h / 13) % 1_001) as u32;
        let daily_request = if bursty {
            ((h / 3) % 501) as u32
        } else {
            ((h / 5) % 501) as u32
        };
        let is_active = if (idx as f32) / (n_items as f32) < active_ratio {
            1
        } else {
            0
        };

        states.push(QueueState {
            source_pool,
            queue_accum,
            units,
            _pad0: 0,
        });
        params.push(QueueParams {
            daily_request,
            unit_cost,
            is_active,
            _pad0: 0,
        });
    }

    let edge = make_manual_edge_case_scenario();
    let edge_count = edge.states.len().min(n_items);
    for idx in 0..edge_count {
        states[idx] = edge.states[idx];
        params[idx] = edge.params[idx];
    }

    MultiTargetScenario {
        name: name.to_string(),
        states,
        params,
    }
}

fn pad_storage_bytes<T: Pod>(data: &[T]) -> Vec<u8> {
    let elem_size = std::mem::size_of::<T>();
    if data.is_empty() {
        vec![0u8; elem_size.max(4)]
    } else {
        bytemuck::cast_slice(data).to_vec()
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

pub struct MultiTargetReplayHarness {
    device: Device,
    queue: Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    resident_pipeline: wgpu::ComputePipeline,
    resident_bind_group_layout: wgpu::BindGroupLayout,
}

impl MultiTargetReplayHarness {
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
                    label: Some("simthing-workshop multitarget_replay"),
                    required_features: Features::empty(),
                    required_limits: limits,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("multitarget_replay"),
            source: wgpu::ShaderSource::Wgsl(include_str!("multitarget_replay_gpu.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("multitarget_replay_layout"),
            entries: &[
                storage_entry(0, true),
                storage_entry(1, true),
                storage_entry(2, false),
                storage_entry(3, false),
                storage_entry(4, false),
                uniform_entry(5),
            ],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("multitarget_replay_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("multitarget_replay_pl"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "resolve_multitarget",
            compilation_options: Default::default(),
            cache: None,
        });

        let resident_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("multitarget_resident_layout"),
                entries: &[
                    storage_entry(0, false),
                    storage_entry(1, true),
                    storage_entry(2, false),
                    storage_entry(3, false),
                    uniform_entry(4),
                ],
            });

        let resident_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("multitarget_resident_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("multitarget_resident_pl"),
                bind_group_layouts: &[&resident_bind_group_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "resolve_multitarget_resident_tick",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
            resident_pipeline,
            resident_bind_group_layout,
        })
    }

    pub fn run_gpu(
        &self,
        scenario: &MultiTargetScenario,
        replay_mode: u32,
    ) -> Result<(
        Vec<QueueState>,
        Vec<CompactDeltaRecord>,
        Vec<FullDeltaRecord>,
    )> {
        let n_items = scenario.states.len();
        let initial_bytes = pad_storage_bytes(&scenario.states);
        let params_bytes = pad_storage_bytes(&scenario.params);

        let initial_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_initial"),
                contents: &initial_bytes,
                usage: BufferUsages::STORAGE,
            });
        let params_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_params"),
                contents: &params_bytes,
                usage: BufferUsages::STORAGE,
            });

        let final_size = (n_items * std::mem::size_of::<QueueState>()) as u64;
        let compact_size = (n_items * std::mem::size_of::<CompactDeltaRecord>()) as u64;
        let full_size = (n_items * std::mem::size_of::<FullDeltaRecord>()).max(4) as u64;

        let final_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_final"),
            size: final_size.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let compact_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_compact"),
            size: compact_size.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let full_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_full"),
            size: full_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let gpu_params = MultiTargetParams {
            n_items: n_items as u32,
            replay_mode,
            _pad0: 0,
            _pad1: 0,
        };
        let uniform_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_uniform"),
                contents: bytemuck::bytes_of(&gpu_params),
                usage: BufferUsages::UNIFORM,
            });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("multitarget_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: initial_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: final_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: compact_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: full_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        let workgroups = ((n_items as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let final_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_final_readback"),
            size: final_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let compact_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_compact_readback"),
            size: compact_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let full_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_full_readback"),
            size: full_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("multitarget_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("multitarget_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&final_buffer, 0, &final_readback, 0, final_size.max(4));
        encoder.copy_buffer_to_buffer(
            &compact_buffer,
            0,
            &compact_readback,
            0,
            compact_size.max(4),
        );
        if replay_mode == REPLAY_MODE_FULL {
            encoder.copy_buffer_to_buffer(&full_buffer, 0, &full_readback, 0, full_size);
        }
        self.queue.submit(Some(encoder.finish()));

        let final_states = map_readback_pod::<QueueState>(&self.device, &final_readback, n_items)?;
        let compact_records =
            map_readback_pod::<CompactDeltaRecord>(&self.device, &compact_readback, n_items)?;
        let full_records = if replay_mode == REPLAY_MODE_FULL {
            map_readback_pod::<FullDeltaRecord>(&self.device, &full_readback, n_items)?
        } else {
            Vec::new()
        };

        Ok((final_states, compact_records, full_records))
    }

    pub fn run_gpu_resident(
        &self,
        scenario: &MultiTargetScenario,
        ticks: u32,
        write_per_item_records: bool,
    ) -> Result<MultiTargetResidentResult> {
        let t_total = Instant::now();
        let n_items = scenario.states.len();
        let states_bytes = pad_storage_bytes(&scenario.states);
        let params_bytes = pad_storage_bytes(&scenario.params);

        let states_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_resident_states"),
                contents: &states_bytes,
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            });
        let params_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_resident_params"),
                contents: &params_bytes,
                usage: BufferUsages::STORAGE,
            });

        let summary_u32s = (ticks as usize) * 4;
        let summary_zeros = vec![0u32; summary_u32s];
        let summary_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("multitarget_resident_summary"),
                contents: bytemuck::cast_slice(&summary_zeros),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            });

        let record_count = if write_per_item_records {
            ticks as usize * n_items
        } else {
            0
        };
        let compact_size = if write_per_item_records {
            (record_count * std::mem::size_of::<CompactDeltaRecord>()) as u64
        } else {
            std::mem::size_of::<CompactDeltaRecord>() as u64
        };
        let compact_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_resident_compact"),
            size: compact_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let workgroups = ((n_items as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let write_flag = if write_per_item_records {
            RESIDENT_MODE_RECORDS
        } else {
            RESIDENT_MODE_SUMMARY_ONLY
        };

        let mut uniform_buffers = Vec::with_capacity(ticks as usize);
        let mut bind_groups = Vec::with_capacity(ticks as usize);
        for tick in 0..ticks {
            let gpu_params = ResidentParams {
                n_items: n_items as u32,
                tick_index: tick,
                record_stride: n_items as u32,
                write_per_item_records: write_flag,
            };
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("multitarget_resident_uniform"),
                        contents: bytemuck::bytes_of(&gpu_params),
                        usage: BufferUsages::UNIFORM,
                    });
            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("multitarget_resident_bg"),
                layout: &self.resident_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: states_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: params_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: compact_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: summary_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            });
            uniform_buffers.push(uniform_buffer);
            bind_groups.push(bind_group);
        }

        let states_size = (n_items * std::mem::size_of::<QueueState>()) as u64;
        let summary_size = (summary_u32s * std::mem::size_of::<u32>()) as u64;
        let states_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_resident_states_readback"),
            size: states_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let summary_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_resident_summary_readback"),
            size: summary_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let compact_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("multitarget_resident_compact_readback"),
            size: compact_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let t_submit = Instant::now();
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("multitarget_resident_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("multitarget_resident_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.resident_pipeline);
            for bind_group in &bind_groups {
                pass.set_bind_group(0, bind_group, &[]);
                pass.dispatch_workgroups(workgroups, 1, 1);
            }
        }
        encoder.copy_buffer_to_buffer(&states_buffer, 0, &states_readback, 0, states_size.max(4));
        encoder.copy_buffer_to_buffer(
            &summary_buffer,
            0,
            &summary_readback,
            0,
            summary_size.max(4),
        );
        if write_per_item_records {
            encoder.copy_buffer_to_buffer(&compact_buffer, 0, &compact_readback, 0, compact_size);
        }
        self.queue.submit(Some(encoder.finish()));
        let submit_us = t_submit.elapsed().as_micros() as u64;

        let final_states = map_readback_pod::<QueueState>(&self.device, &states_readback, n_items)?;
        let summary_raw = map_readback_pod::<u32>(&self.device, &summary_readback, summary_u32s)?;
        let summaries = parse_tick_summaries(&summary_raw, ticks);
        let compact_records = if write_per_item_records {
            map_readback_pod::<CompactDeltaRecord>(&self.device, &compact_readback, record_count)?
        } else {
            Vec::new()
        };
        let total_validation_us = t_total.elapsed().as_micros() as u64;

        let _ = uniform_buffers;

        Ok(MultiTargetResidentResult {
            final_states,
            summaries,
            compact_records,
            submit_us,
            total_validation_us,
        })
    }
}

fn parse_tick_summaries(raw: &[u32], ticks: u32) -> Vec<TickSummaryReadback> {
    (0..ticks as usize)
        .map(|t| {
            let base = t * 4;
            TickSummaryReadback {
                total_transferred: raw[base],
                total_emitted_units: raw[base + 1],
                active_count: raw[base + 2],
                reserved: raw[base + 3],
            }
        })
        .collect()
}

fn map_readback_pod<T: Pod>(device: &Device, readback: &Buffer, len: usize) -> Result<Vec<T>> {
    let byte_len = len * std::mem::size_of::<T>();
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
    let out: Vec<T> = bytemuck::cast_slice(&mapped[..byte_len]).to_vec();
    drop(mapped);
    readback.unmap();
    Ok(out)
}

fn states_equal(a: &[QueueState], b: &[QueueState]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn warm_runs_for(n_items: usize) -> usize {
    if n_items >= 1_000_000 {
        3
    } else if n_items >= 100_000 {
        3
    } else {
        WARM_RUNS
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

fn scenario_totals(
    initial: &[QueueState],
    final_state: &[QueueState],
    params: &[QueueParams],
    records: &[CompactDeltaRecord],
) -> (u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut total_source_before = 0u64;
    let mut total_queue_before = 0u64;
    let mut total_units_before = 0u64;
    let mut total_source_after = 0u64;
    let mut total_queue_after = 0u64;
    let mut total_units_after = 0u64;
    let mut total_transferred = 0u64;
    let mut total_emitted_units = 0u64;
    let mut total_emitted_value = 0u64;

    for idx in 0..initial.len() {
        total_source_before += initial[idx].source_pool as u64;
        total_queue_before += initial[idx].queue_accum as u64;
        total_units_before += initial[idx].units as u64;
        total_source_after += final_state[idx].source_pool as u64;
        total_queue_after += final_state[idx].queue_accum as u64;
        total_units_after += final_state[idx].units as u64;
        total_transferred += records[idx].transfer_amount as u64;
        total_emitted_units += records[idx].emit_count as u64;
        total_emitted_value += records[idx].emit_count as u64 * params[idx].unit_cost as u64;
    }

    (
        total_source_before,
        total_queue_before,
        total_units_before,
        total_source_after,
        total_queue_after,
        total_units_after,
        total_transferred,
        total_emitted_units,
        total_emitted_value,
    )
}

fn interpretation_string(
    conservation_ok: bool,
    replay_ok: bool,
    deterministic: bool,
    final_matches: bool,
    compact_gate: bool,
    speedup: f32,
    ratio: f32,
) -> String {
    if !conservation_ok || !replay_ok || !deterministic || !final_matches {
        return "FAIL: GPU multi-target settlement or compact replay records are insufficient."
            .to_string();
    }
    if compact_gate && speedup >= 1.0 {
        format!(
            "STRONG_PASS: GPU compact settlement matches CPU in upload/readback envelope mode, replay reconstructs final state, conservation holds, compact records are {:.1}% of full size, GPU compact {:.2}x vs CPU.",
            ratio * 100.0,
            speedup
        )
    } else if compact_gate {
        format!(
            "WEAK_PASS: Correct and deterministic in upload/readback envelope mode, but GPU compact {:.2}x vs CPU; compact records {:.1}% of full size. Envelope timing is not representative of GPU-resident pivot performance.",
            speedup,
            ratio * 100.0
        )
    } else {
        format!(
            "WEAK_PASS: Correct and deterministic in upload/readback envelope mode, but compact record savings are weak ({:.1}% of full).",
            ratio * 100.0
        )
    }
}

fn record_ticks_for(n_items: usize, ticks: u32) -> (u32, bool) {
    let bytes = n_items as u64 * ticks as u64 * std::mem::size_of::<CompactDeltaRecord>() as u64;
    if bytes > 100 * 1024 * 1024 {
        (8, true)
    } else {
        (ticks, false)
    }
}

fn summaries_equal(a: &[TickSummaryReadback], b: &[TickSummaryReadback]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn resident_interpretation(
    conservation_ok: bool,
    replay_ok: bool,
    deterministic: bool,
    final_summary: bool,
    final_records: bool,
    summary_match: bool,
    summary_speedup: f32,
    records_speedup: f32,
    records_fallback: bool,
) -> (String, String) {
    if !conservation_ok || !replay_ok || !deterministic || !final_summary || !summary_match {
        return (
            "FAIL: GPU-resident multi-target settlement or replay validation failed.".to_string(),
            "FAIL".to_string(),
        );
    }

    let perf_gate = if summary_speedup >= 1.0 {
        if final_records && records_speedup >= 1.0 {
            "STRONG_PASS".to_string()
        } else if final_records {
            "WEAK_PASS".to_string()
        } else {
            "STRONG_PASS".to_string()
        }
    } else {
        "WEAK_PASS".to_string()
    };

    let mut interpretation = if summary_speedup >= 1.0 {
        format!(
            "STRONG_PASS: GPU-resident summary mode matches CPU over N ticks ({:.2}x vs CPU on total_validation timing). Semantic and summary gates pass.",
            summary_speedup
        )
    } else {
        format!(
            "WEAK_PASS: GPU-resident correctness passes but summary mode {:.2}x vs CPU on total_validation timing; perf unresolved.",
            summary_speedup
        )
    };

    if final_records {
        if records_speedup >= 1.0 {
            interpretation.push_str(&format!(
                " Records mode {:.2}x vs CPU on total_validation timing.",
                records_speedup
            ));
        } else {
            interpretation.push_str(&format!(
                " Records mode slower ({:.2}x vs CPU on total_validation timing); log volume pressure.",
                records_speedup
            ));
        }
    }

    if records_fallback {
        interpretation.push_str(
            " Records mode used 8-tick fallback due to ~102MB compact record allocation limit.",
        );
    }

    interpretation.push_str(
        " Workshop-local; not production AccumulatorOp; no contention; not pure shader timing.",
    );

    (interpretation, perf_gate)
}

pub fn compare_multitarget_resident_rich(
    scenario: &MultiTargetScenario,
    ticks: u32,
) -> Result<MultiTargetResidentReport> {
    let harness = MultiTargetReplayHarness::new()?;
    compare_multitarget_resident_rich_with_harness(&harness, scenario, ticks)
}

pub fn compare_multitarget_resident_rich_with_harness(
    harness: &MultiTargetReplayHarness,
    scenario: &MultiTargetScenario,
    ticks: u32,
) -> Result<MultiTargetResidentReport> {
    let n_items = scenario.states.len();
    let active_count = scenario.params.iter().filter(|p| p.is_active != 0).count();
    let (record_ticks, records_memory_fallback) = record_ticks_for(n_items, ticks);

    let t0 = Instant::now();
    let (cpu_final, cpu_summaries, _cpu_records) = resolve_cpu_current_n_ticks(scenario, ticks);
    let cpu_n_ticks_us = t0.elapsed().as_micros() as u64;

    let (cpu_final_records, cpu_summaries_records, _cpu_records_subset) = if record_ticks < ticks {
        let (f, s, r) = resolve_cpu_current_n_ticks(scenario, record_ticks);
        (f, s, r)
    } else {
        (
            cpu_final.clone(),
            cpu_summaries.clone(),
            _cpu_records.clone(),
        )
    };

    let t_records = Instant::now();
    let _ = resolve_cpu_current_n_ticks(scenario, record_ticks);
    let cpu_records_ticks_us = t_records.elapsed().as_micros() as u64;

    let _ = harness.run_gpu_resident(scenario, ticks, false)?;

    let mut summary_submit_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut summary_total_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut records_submit_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut records_total_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut gpu_summary_base: Option<MultiTargetResidentResult> = None;
    let mut gpu_records_base: Option<MultiTargetResidentResult> = None;
    let mut deterministic = true;

    for _ in 0..RESIDENT_WARM_RUNS {
        let gpu_summary = harness.run_gpu_resident(scenario, ticks, false)?;
        summary_submit_warm.push(gpu_summary.submit_us);
        summary_total_warm.push(gpu_summary.total_validation_us);

        match &gpu_summary_base {
            None => gpu_summary_base = Some(gpu_summary),
            Some(base) => {
                if base.final_states != gpu_summary.final_states
                    || base.summaries != gpu_summary.summaries
                {
                    deterministic = false;
                }
            }
        }

        let gpu_records = harness.run_gpu_resident(scenario, record_ticks, true)?;
        records_submit_warm.push(gpu_records.submit_us);
        records_total_warm.push(gpu_records.total_validation_us);

        match &gpu_records_base {
            None => gpu_records_base = Some(gpu_records),
            Some(base) => {
                if base.final_states != gpu_records.final_states
                    || base.compact_records != gpu_records.compact_records
                {
                    deterministic = false;
                }
            }
        }
    }

    let gpu_summary = gpu_summary_base.unwrap();
    let gpu_records = gpu_records_base.unwrap();

    let final_state_matches_cpu_summary_mode = states_equal(&cpu_final, &gpu_summary.final_states);
    let final_state_matches_cpu_records_mode =
        states_equal(&cpu_final_records, &gpu_records.final_states);
    let summary_matches_cpu = summaries_equal(&cpu_summaries, &gpu_summary.summaries);

    let replayed = replay_from_compact_records_n_ticks(
        &scenario.states,
        &scenario.params,
        &gpu_records.compact_records,
        record_ticks,
        n_items,
    )?;
    let replay_from_records_matches_gpu = states_equal(&replayed, &gpu_records.final_states);

    let records_summaries_match = if record_ticks < ticks {
        summaries_equal(&cpu_summaries_records, &gpu_records.summaries)
    } else {
        true
    };

    let conservation_gate = if final_state_matches_cpu_summary_mode && summary_matches_cpu {
        "PASS"
    } else {
        "FAIL"
    };
    let replay_gate = if replay_from_records_matches_gpu
        && final_state_matches_cpu_records_mode
        && records_summaries_match
    {
        "PASS"
    } else {
        "FAIL"
    };
    let determinism_gate = if deterministic { "PASS" } else { "FAIL" };

    let (summary_submit_mean, summary_submit_min, summary_submit_max) =
        warm_stats(&summary_submit_warm);
    let (summary_total_mean, summary_total_min, summary_total_max) =
        warm_stats(&summary_total_warm);
    let (records_submit_mean, records_submit_min, records_submit_max) =
        warm_stats(&records_submit_warm);
    let (records_total_mean, records_total_min, records_total_max) =
        warm_stats(&records_total_warm);

    let cpu_per_tick = cpu_n_ticks_us as f32 / ticks as f32;
    let summary_total_per_tick = summary_total_mean as f32 / ticks as f32;
    let records_total_per_tick = records_total_mean as f32 / record_ticks as f32;

    let summary_speedup = if summary_total_mean > 0 {
        cpu_n_ticks_us as f32 / summary_total_mean as f32
    } else {
        0.0
    };
    let records_speedup = if records_total_mean > 0 {
        cpu_records_ticks_us as f32 / records_total_mean as f32
    } else {
        0.0
    };

    let summary_bytes_total = ticks as usize * 4 * std::mem::size_of::<u32>();
    let compact_record_bytes_total =
        record_ticks as usize * n_items * std::mem::size_of::<CompactDeltaRecord>();

    let (interpretation, resident_performance_gate) = resident_interpretation(
        conservation_gate == "PASS",
        replay_gate == "PASS",
        determinism_gate == "PASS",
        final_state_matches_cpu_summary_mode,
        final_state_matches_cpu_records_mode,
        summary_matches_cpu,
        summary_speedup,
        records_speedup,
        records_memory_fallback,
    );

    let _ = (
        summary_submit_min,
        summary_submit_max,
        summary_total_min,
        summary_total_max,
        records_submit_min,
        records_submit_max,
        records_total_min,
        records_total_max,
    );

    Ok(MultiTargetResidentReport {
        scenario_name: scenario.name.clone(),
        n_items,
        active_count,
        ticks,
        record_ticks,
        records_memory_fallback,
        cpu_n_ticks_us,
        gpu_resident_summary_submit_mean_us: summary_submit_mean,
        gpu_resident_summary_submit_min_us: summary_submit_min,
        gpu_resident_summary_submit_max_us: summary_submit_max,
        gpu_resident_summary_total_mean_us: summary_total_mean,
        gpu_resident_summary_total_min_us: summary_total_min,
        gpu_resident_summary_total_max_us: summary_total_max,
        gpu_resident_records_submit_mean_us: records_submit_mean,
        gpu_resident_records_submit_min_us: records_submit_min,
        gpu_resident_records_submit_max_us: records_submit_max,
        gpu_resident_records_total_mean_us: records_total_mean,
        gpu_resident_records_total_min_us: records_total_min,
        gpu_resident_records_total_max_us: records_total_max,
        gpu_resident_summary_total_per_tick_us: summary_total_per_tick,
        gpu_resident_records_total_per_tick_us: records_total_per_tick,
        cpu_per_tick_us: cpu_per_tick,
        final_state_matches_cpu_summary_mode,
        final_state_matches_cpu_records_mode,
        summary_matches_cpu,
        replay_from_records_matches_gpu,
        resident_summary_speedup_vs_cpu: summary_speedup,
        resident_records_speedup_vs_cpu: records_speedup,
        compact_record_bytes_total,
        summary_bytes_total,
        conservation_gate: conservation_gate.to_string(),
        replay_gate: replay_gate.to_string(),
        determinism_gate: determinism_gate.to_string(),
        resident_performance_gate,
        interpretation,
        timing_note: RESIDENT_TIMING_NOTE.to_string(),
    })
}

pub fn format_multitarget_resident_report(report: &MultiTargetResidentReport) -> String {
    crate::multitarget_replay_report::format_multitarget_resident_report(report)
}

pub fn write_multitarget_resident_reports(report: &MultiTargetResidentReport) -> Result<()> {
    crate::multitarget_replay_report::write_multitarget_resident_reports(report).map_err(Into::into)
}

pub fn write_multitarget_resident_reports_bundle(
    reports: &[MultiTargetResidentReport],
) -> Result<()> {
    crate::multitarget_replay_report::write_multitarget_resident_reports_bundle(reports)
        .map_err(Into::into)
}

pub fn compare_multitarget_replay_rich(
    scenario: &MultiTargetScenario,
) -> Result<MultiTargetReplayReport> {
    let harness = MultiTargetReplayHarness::new()?;
    compare_multitarget_replay_rich_with_harness(&harness, scenario)
}

pub fn compare_multitarget_replay_rich_with_harness(
    harness: &MultiTargetReplayHarness,
    scenario: &MultiTargetScenario,
) -> Result<MultiTargetReplayReport> {
    let n_items = scenario.states.len();
    let warm_runs = warm_runs_for(n_items);

    let t0 = Instant::now();
    let (cpu_final, _cpu_compact, _cpu_full) = resolve_cpu_current(scenario, false);
    let cpu_current_us = t0.elapsed().as_micros() as u64;

    let _ = harness.run_gpu(scenario, REPLAY_MODE_COMPACT)?;

    let mut compact_warm = Vec::with_capacity(warm_runs);
    let mut full_warm = Vec::with_capacity(warm_runs);
    let mut gpu_final_base: Option<Vec<QueueState>> = None;
    let mut gpu_compact_base: Option<Vec<CompactDeltaRecord>> = None;
    let mut deterministic = true;

    for _ in 0..warm_runs {
        let t = Instant::now();
        let (gpu_final, gpu_compact, _) = harness.run_gpu(scenario, REPLAY_MODE_COMPACT)?;
        compact_warm.push(t.elapsed().as_micros() as u64);

        match (&gpu_final_base, &gpu_compact_base) {
            (None, None) => {
                gpu_final_base = Some(gpu_final);
                gpu_compact_base = Some(gpu_compact);
            }
            (Some(base_final), Some(base_compact)) => {
                if !states_equal(&gpu_final, base_final) || gpu_compact != *base_compact {
                    deterministic = false;
                }
            }
            _ => deterministic = false,
        }

        let t = Instant::now();
        let _ = harness.run_gpu(scenario, REPLAY_MODE_FULL)?;
        full_warm.push(t.elapsed().as_micros() as u64);
    }

    let gpu_final = gpu_final_base.unwrap_or_default();
    let gpu_compact = gpu_compact_base.unwrap_or_default();

    let replayed = replay_from_compact_records(&scenario.states, &scenario.params, &gpu_compact)?;
    let final_state_matches_cpu = states_equal(&cpu_final, &gpu_final);
    let replay_from_compact_matches_gpu = states_equal(&replayed, &gpu_final);
    let conservation_ok =
        conservation_check(&scenario.states, &gpu_final, &scenario.params, &gpu_compact);

    let compact_record_bytes = n_items * std::mem::size_of::<CompactDeltaRecord>();
    let full_record_bytes = n_items * std::mem::size_of::<FullDeltaRecord>();
    let compact_vs_full_record_ratio = if full_record_bytes > 0 {
        compact_record_bytes as f32 / full_record_bytes as f32
    } else {
        1.0
    };
    let compact_record_gate = compact_vs_full_record_ratio <= 0.75;

    let conservation_gate = if conservation_ok {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };
    let replay_gate = if replay_from_compact_matches_gpu {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };
    let determinism_gate = if deterministic {
        "PASS".to_string()
    } else {
        "FAIL".to_string()
    };
    let compact_gate_str = if compact_record_gate {
        "PASS".to_string()
    } else {
        "WEAK".to_string()
    };

    let (compact_mean, compact_min, compact_max) = warm_stats(&compact_warm);
    let (full_mean, full_min, full_max) = warm_stats(&full_warm);
    let speedup = if compact_mean > 0 {
        cpu_current_us as f32 / compact_mean as f32
    } else {
        0.0
    };

    let active_count = scenario.params.iter().filter(|p| p.is_active != 0).count();
    let unit_cost_min = scenario
        .params
        .iter()
        .map(|p| p.unit_cost)
        .min()
        .unwrap_or(0);
    let unit_cost_max = scenario
        .params
        .iter()
        .map(|p| p.unit_cost)
        .max()
        .unwrap_or(0);

    let (
        total_source_before,
        total_queue_before,
        total_units_before,
        total_source_after,
        total_queue_after,
        total_units_after,
        total_transferred,
        total_emitted_units,
        total_emitted_value,
    ) = scenario_totals(&scenario.states, &gpu_final, &scenario.params, &gpu_compact);

    let interpretation = interpretation_string(
        conservation_ok,
        replay_from_compact_matches_gpu,
        deterministic,
        final_state_matches_cpu,
        compact_record_gate,
        speedup,
        compact_vs_full_record_ratio,
    );

    Ok(MultiTargetReplayReport {
        scenario_name: scenario.name.clone(),
        n_items,
        active_count,
        unit_cost_min,
        unit_cost_max,
        total_source_before,
        total_queue_before,
        total_units_before,
        total_source_after,
        total_queue_after,
        total_units_after,
        total_transferred,
        total_emitted_units,
        total_emitted_value,
        conservation_gate,
        replay_gate,
        determinism_gate,
        compact_record_gate: compact_gate_str,
        cpu_current_us,
        gpu_compact_warm_mean_us: compact_mean,
        gpu_compact_warm_min_us: compact_min,
        gpu_compact_warm_max_us: compact_max,
        gpu_full_warm_mean_us: full_mean,
        gpu_full_warm_min_us: full_min,
        gpu_full_warm_max_us: full_max,
        compact_record_bytes,
        full_record_bytes,
        compact_vs_full_record_ratio,
        speedup_gpu_compact_vs_cpu: speedup,
        final_state_matches_cpu,
        replay_from_compact_matches_gpu,
        repeated_gpu_outputs_identical: deterministic,
        interpretation,
        timing_note: TIMING_NOTE.to_string(),
    })
}

pub fn format_multitarget_replay_report(report: &MultiTargetReplayReport) -> String {
    crate::multitarget_replay_report::format_multitarget_replay_report(report)
}

pub fn write_multitarget_replay_reports(report: &MultiTargetReplayReport) -> Result<()> {
    crate::multitarget_replay_report::write_multitarget_replay_reports(report).map_err(Into::into)
}

pub fn write_multitarget_replay_reports_bundle(reports: &[MultiTargetReplayReport]) -> Result<()> {
    crate::multitarget_replay_report::write_multitarget_replay_reports_bundle(reports)
        .map_err(Into::into)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

}
