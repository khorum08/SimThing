//! Transfer / emission contention spike — CPU boundary allocator vs GPU-resident pivot.

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
pub const RESIDENT_TICKS: u32 = 64;
pub const RESIDENT_WARM_RUNS: usize = 5;

pub const RECORD_MODE_SUMMARY_ONLY: u32 = 0;
pub const RECORD_MODE_COMPACT: u32 = 1;

pub const TIMING_NOTE: &str =
    "Resident total_validation timings include initial buffer upload, encode, submit, GPU wait, and final readback. submit timings cover encode+submit only. Neither is pure shader timestamp time.";

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct PoolState {
    pub amount: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct QueueState {
    pub accum: u32,
    pub units: u32,
    pub unit_cost: u32,
    pub is_active: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct TransferRequest {
    pub pool: u32,
    pub queue: u32,
    pub amount_requested: u32,
    pub priority_band: u32,
    pub priority: u32,
    pub authored_order: u32,
    pub is_active: u32,
    pub _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct PoolRequestRange {
    pub start: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct TransferDeltaRecord {
    pub tick: u32,
    pub request: u32,
    pub pool: u32,
    pub queue: u32,
    pub requested: u32,
    pub allocated: u32,
    pub emitted_units: u32,
    pub is_active: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct PoolTickSummary {
    pub pool: u32,
    pub tick: u32,
    pub amount_before: u32,
    pub amount_after: u32,
    pub total_requested: u32,
    pub total_allocated: u32,
    pub total_emitted_units: u32,
    pub active_requests: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferResidentParams {
    pub n_pools: u32,
    pub n_queues: u32,
    pub n_requests: u32,
    pub tick_index: u32,
    pub record_stride: u32,
    pub summary_stride: u32,
    pub write_records: u32,
    pub _pad0: u32,
}

#[derive(Debug, Clone)]
pub struct TransferContentionScenario {
    pub name: String,
    pub pools: Vec<PoolState>,
    pub queues: Vec<QueueState>,
    pub requests: Vec<TransferRequest>,
    pub pool_ranges: Vec<PoolRequestRange>,
    pub queue_cross_pool_contention: bool,
}

#[derive(Debug, Clone)]
pub struct TransferResidentResult {
    pub final_pools: Vec<PoolState>,
    pub final_queues: Vec<QueueState>,
    pub summaries: Vec<PoolTickSummary>,
    pub records: Vec<TransferDeltaRecord>,
    pub submit_us: u64,
    pub total_validation_us: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferContentionReport {
    pub scenario_name: String,
    pub n_pools: usize,
    pub n_queues: usize,
    pub n_requests: usize,
    pub ticks: u32,
    pub record_ticks: u32,
    pub records_memory_fallback: bool,
    pub queue_cross_pool_contention: bool,
    pub total_pool_before: u64,
    pub total_pool_after: u64,
    pub total_queue_accum_before: u64,
    pub total_queue_accum_after: u64,
    pub total_units_before: u64,
    pub total_units_after: u64,
    pub total_requested: u64,
    pub total_allocated: u64,
    pub total_emitted_value: u64,
    pub cpu_n_ticks_us: u64,
    pub gpu_summary_submit_mean_us: u64,
    pub gpu_summary_total_mean_us: u64,
    pub gpu_summary_total_per_tick_us: f32,
    pub gpu_records_submit_mean_us: u64,
    pub gpu_records_total_mean_us: u64,
    pub gpu_records_total_per_tick_us: f32,
    pub cpu_per_tick_us: f32,
    pub summary_speedup_vs_cpu: f32,
    pub records_speedup_vs_cpu: f32,
    pub final_state_matches_cpu_summary_mode: bool,
    pub final_state_matches_cpu_records_mode: bool,
    pub summaries_match_cpu: bool,
    pub replay_from_records_matches_gpu: bool,
    pub conservation_gate: String,
    pub priority_gate: String,
    pub replay_gate: String,
    pub determinism_gate: String,
    pub performance_gate: String,
    pub summary_bytes_total: usize,
    pub compact_record_bytes_total: usize,
    pub interpretation: String,
    pub timing_note: String,
}

fn canonical_cmp(
    a: &TransferRequest,
    b: &TransferRequest,
    a_idx: usize,
    b_idx: usize,
) -> std::cmp::Ordering {
    a.pool
        .cmp(&b.pool)
        .then(a.priority_band.cmp(&b.priority_band))
        .then(a.priority.cmp(&b.priority))
        .then(a.authored_order.cmp(&b.authored_order))
        .then((a_idx as u32).cmp(&(b_idx as u32)))
}

pub fn sort_requests_and_build_ranges(
    requests: &mut [TransferRequest],
    n_pools: usize,
) -> Vec<PoolRequestRange> {
    let mut indexed: Vec<(usize, TransferRequest)> = requests
        .iter()
        .copied()
        .enumerate()
        .map(|(i, r)| (i, r))
        .collect();
    indexed.sort_by(|(ai, a), (bi, b)| canonical_cmp(a, b, *ai, *bi));
    for (i, (_, req)) in indexed.iter().enumerate() {
        requests[i] = *req;
    }

    let mut ranges = vec![PoolRequestRange { start: 0, count: 0 }; n_pools];
    for (idx, req) in requests.iter().enumerate() {
        let pool = req.pool as usize;
        if pool >= n_pools {
            continue;
        }
        if ranges[pool].count == 0 {
            ranges[pool].start = idx as u32;
            ranges[pool].count = 1;
        } else {
            ranges[pool].count += 1;
        }
    }
    ranges
}

pub fn resolve_cpu_contention_n_ticks(
    scenario: &TransferContentionScenario,
    ticks: u32,
) -> (
    Vec<PoolState>,
    Vec<QueueState>,
    Vec<PoolTickSummary>,
    Vec<TransferDeltaRecord>,
) {
    let n_pools = scenario.pools.len();
    let n_requests = scenario.requests.len();
    let mut pools = scenario.pools.clone();
    let mut queues = scenario.queues.clone();
    let mut summaries = Vec::with_capacity(ticks as usize * n_pools);
    let mut records = vec![
        TransferDeltaRecord {
            tick: 0,
            request: 0,
            pool: 0,
            queue: 0,
            requested: 0,
            allocated: 0,
            emitted_units: 0,
            is_active: 0,
        };
        ticks as usize * n_requests
    ];

    for tick in 0..ticks {
        for pool_idx in 0..n_pools {
            let before = pools[pool_idx].amount;
            let mut remaining = before;
            let range = scenario.pool_ranges[pool_idx];

            let mut total_requested = 0u32;
            let mut total_allocated = 0u32;
            let mut total_emitted_units = 0u32;
            let mut active_requests = 0u32;

            for i in 0..range.count {
                let request_idx = (range.start + i) as usize;
                let req = scenario.requests[request_idx];

                let mut allocated = 0u32;
                let mut emitted = 0u32;
                let mut active = 0u32;

                if req.is_active != 0 && (req.queue as usize) < queues.len() {
                    let q = queues[req.queue as usize];
                    if q.is_active != 0 && q.unit_cost != 0 {
                        active = 1;
                        active_requests += 1;
                        total_requested = total_requested.saturating_add(req.amount_requested);

                        allocated = remaining.min(req.amount_requested);
                        remaining -= allocated;

                        let pre_emit = q.accum.saturating_add(allocated);
                        emitted = pre_emit / q.unit_cost;
                        let accum_after = pre_emit - emitted * q.unit_cost;
                        let units_after = q.units + emitted;

                        queues[req.queue as usize] = QueueState {
                            accum: accum_after,
                            units: units_after,
                            unit_cost: q.unit_cost,
                            is_active: q.is_active,
                        };

                        total_allocated = total_allocated.saturating_add(allocated);
                        total_emitted_units = total_emitted_units.saturating_add(emitted);
                    }
                }

                records[tick as usize * n_requests + request_idx] = TransferDeltaRecord {
                    tick,
                    request: request_idx as u32,
                    pool: pool_idx as u32,
                    queue: req.queue,
                    requested: req.amount_requested,
                    allocated,
                    emitted_units: emitted,
                    is_active: active,
                };
            }

            pools[pool_idx].amount = remaining;
            summaries.push(PoolTickSummary {
                pool: pool_idx as u32,
                tick,
                amount_before: before,
                amount_after: remaining,
                total_requested,
                total_allocated,
                total_emitted_units,
                active_requests,
            });
        }
    }

    (pools, queues, summaries, records)
}

pub fn replay_transfer_records_n_ticks(
    initial_pools: &[PoolState],
    initial_queues: &[QueueState],
    requests: &[TransferRequest],
    records: &[TransferDeltaRecord],
    ticks: u32,
    n_requests: usize,
) -> Result<(Vec<PoolState>, Vec<QueueState>)> {
    if records.len() != ticks as usize * n_requests {
        bail!(
            "records length {} expected {}",
            records.len(),
            ticks as usize * n_requests
        );
    }

    let mut pools = initial_pools.to_vec();
    let mut queues = initial_queues.to_vec();

    for tick in 0..ticks as usize {
        for request_idx in 0..n_requests {
            let idx = tick * n_requests + request_idx;
            let record = records[idx];
            let req = requests[request_idx];

            if record.request as usize != request_idx {
                bail!(
                    "record at tick {} slot {} has request id {}",
                    tick,
                    request_idx,
                    record.request
                );
            }
            if record.pool != req.pool || record.queue != req.queue {
                bail!(
                    "record {} pool/queue mismatch req ({},{}) record ({},{})",
                    request_idx,
                    req.pool,
                    req.queue,
                    record.pool,
                    record.queue
                );
            }

            if record.is_active == 0 {
                if record.allocated != 0 || record.emitted_units != 0 {
                    bail!("inactive record {} has non-zero deltas", request_idx);
                }
                continue;
            }

            if record.allocated > record.requested {
                bail!("record {} allocated exceeds requested", request_idx);
            }
            if record.allocated > pools[req.pool as usize].amount {
                bail!("record {} allocated exceeds pool amount", request_idx);
            }

            let q = queues[req.queue as usize];
            let pre_emit = q.accum.saturating_add(record.allocated);
            let emitted_value = record.emitted_units as u64 * q.unit_cost as u64;
            if (pre_emit as u64) < emitted_value {
                bail!("record {} emit value exceeds queue pre-emit", request_idx);
            }

            pools[req.pool as usize].amount -= record.allocated;
            queues[req.queue as usize] = QueueState {
                accum: pre_emit - record.emitted_units * q.unit_cost,
                units: q.units + record.emitted_units,
                unit_cost: q.unit_cost,
                is_active: q.is_active,
            };
        }
    }

    Ok((pools, queues))
}

pub fn conservation_check_contention_records(
    initial_pools: &[PoolState],
    final_pools: &[PoolState],
    initial_queues: &[QueueState],
    final_queues: &[QueueState],
    queues: &[QueueState],
    records: &[TransferDeltaRecord],
) -> bool {
    let mut pool_before = 0u64;
    let mut pool_after = 0u64;
    let mut accum_before = 0u64;
    let mut accum_after = 0u64;
    let mut emitted_value = 0u64;
    let mut total_allocated = 0u64;

    for p in initial_pools {
        pool_before += p.amount as u64;
    }
    for p in final_pools {
        pool_after += p.amount as u64;
    }
    for q in initial_queues {
        accum_before += q.accum as u64;
    }
    for q in final_queues {
        accum_after += q.accum as u64;
    }

    for record in records {
        if record.is_active == 0 {
            continue;
        }
        total_allocated += record.allocated as u64;
        let q = queues[record.queue as usize];
        emitted_value += record.emitted_units as u64 * q.unit_cost as u64;
    }

    let pool_depleted = pool_before.saturating_sub(pool_after);
    if total_allocated != pool_depleted {
        return false;
    }

    pool_before + accum_before == pool_after + accum_after + emitted_value
}

pub fn priority_allocation_check(
    scenario: &TransferContentionScenario,
    initial_pools: &[PoolState],
    records: &[TransferDeltaRecord],
    ticks: u32,
) -> bool {
    let n_requests = scenario.requests.len();
    let mut pools = initial_pools.to_vec();

    for tick in 0..ticks as usize {
        for pool_idx in 0..scenario.pools.len() {
            let range = scenario.pool_ranges[pool_idx];
            let mut remaining = pools[pool_idx].amount;

            for i in 0..range.count {
                let request_idx = (range.start + i) as usize;
                let rec = records[tick * n_requests + request_idx];
                let req = scenario.requests[request_idx];

                if rec.is_active == 0 {
                    if rec.allocated != 0 {
                        return false;
                    }
                    continue;
                }

                let expected = remaining.min(req.amount_requested);
                if rec.allocated != expected {
                    return false;
                }
                remaining = remaining.saturating_sub(rec.allocated);
            }

            pools[pool_idx].amount = remaining;
        }
    }

    true
}

fn mix_u32(seed: u32) -> u32 {
    seed.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

pub fn make_transfer_contention_scenario(
    name: &str,
    n_pools: usize,
    n_queues: usize,
    active_ratio: f32,
    hotspot: bool,
    bursty: bool,
) -> TransferContentionScenario {
    assert!(n_pools > 0 && n_queues > 0);
    let active_ratio = active_ratio.clamp(0.0, 1.0);
    let hot_pools = n_pools.min(16);

    let mut pools = Vec::with_capacity(n_pools);
    for pool_idx in 0..n_pools {
        let h = mix_u32(pool_idx as u32 + 1);
        let amount = if hotspot && pool_idx < hot_pools {
            (h % 100) + 10
        } else if bursty {
            (h % 500) + 50
        } else {
            (h % 5000) + 500
        };
        pools.push(PoolState {
            amount,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        });
    }

    let mut queues = Vec::with_capacity(n_queues);
    let mut requests = Vec::with_capacity(n_queues);

    for queue_idx in 0..n_queues {
        let h = mix_u32(queue_idx as u32 + 7);
        let pool = if hotspot {
            (queue_idx % hot_pools) as u32
        } else {
            (queue_idx % n_pools) as u32
        };
        let unit_cost = 10 + (h % 191);
        let accum = (h / 7) % (unit_cost.saturating_mul(3).max(1) + 1);
        let units = (h / 13) % 1001;
        let is_active = if (queue_idx as f32) / (n_queues as f32) < active_ratio {
            1
        } else {
            0
        };

        queues.push(QueueState {
            accum,
            units,
            unit_cost,
            is_active,
        });

        requests.push(TransferRequest {
            pool,
            queue: queue_idx as u32,
            amount_requested: (h / 3) % 501,
            priority_band: (h / 11) % 5,
            priority: (h / 17) % 101,
            authored_order: queue_idx as u32,
            is_active,
            _pad0: 0,
        });
    }

    let pool_ranges = sort_requests_and_build_ranges(&mut requests, n_pools);

    TransferContentionScenario {
        name: name.to_string(),
        pools,
        queues,
        requests,
        pool_ranges,
        queue_cross_pool_contention: false,
    }
}

pub fn make_manual_priority_edge_scenario() -> TransferContentionScenario {
    let pools = vec![PoolState {
        amount: 100,
        _pad0: 0,
        _pad1: 0,
        _pad2: 0,
    }];

    let queues = vec![
        QueueState {
            accum: 0,
            units: 0,
            unit_cost: 10,
            is_active: 1,
        },
        QueueState {
            accum: 0,
            units: 0,
            unit_cost: 10,
            is_active: 1,
        },
        QueueState {
            accum: 0,
            units: 0,
            unit_cost: 10,
            is_active: 1,
        },
    ];

    let mut requests = vec![
        TransferRequest {
            pool: 0,
            queue: 0,
            amount_requested: 30,
            priority_band: 0,
            priority: 0,
            authored_order: 0,
            is_active: 1,
            _pad0: 0,
        },
        TransferRequest {
            pool: 0,
            queue: 1,
            amount_requested: 80,
            priority_band: 0,
            priority: 1,
            authored_order: 1,
            is_active: 1,
            _pad0: 0,
        },
        TransferRequest {
            pool: 0,
            queue: 2,
            amount_requested: 10,
            priority_band: 0,
            priority: 2,
            authored_order: 2,
            is_active: 1,
            _pad0: 0,
        },
    ];

    let pool_ranges = sort_requests_and_build_ranges(&mut requests, 1);

    TransferContentionScenario {
        name: "transfer_priority_edge".to_string(),
        pools,
        queues,
        requests,
        pool_ranges,
        queue_cross_pool_contention: false,
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

pub struct TransferContentionHarness {
    device: Device,
    queue: Queue,
    resident_pipeline: wgpu::ComputePipeline,
    resident_layout: wgpu::BindGroupLayout,
}

impl TransferContentionHarness {
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
                    label: Some("simthing-workshop transfer_contention"),
                    required_features: Features::empty(),
                    required_limits: limits,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("transfer_contention"),
            source: wgpu::ShaderSource::Wgsl(include_str!("transfer_contention_gpu.wgsl").into()),
        });

        let resident_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("transfer_contention_layout"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, false),
                storage_entry(2, true),
                storage_entry(3, true),
                storage_entry(4, false),
                storage_entry(5, false),
                uniform_entry(6),
            ],
        });

        let resident_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("transfer_contention_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("transfer_contention_pl"),
                bind_group_layouts: &[&resident_layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "resolve_transfer_contention_tick",
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            resident_pipeline,
            resident_layout,
        })
    }

    pub fn run_gpu_resident(
        &self,
        scenario: &TransferContentionScenario,
        ticks: u32,
        write_records: bool,
    ) -> Result<TransferResidentResult> {
        let t_total = Instant::now();
        let n_pools = scenario.pools.len();
        let n_queues = scenario.queues.len();
        let n_requests = scenario.requests.len();

        let pools_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("tc_pools"),
                contents: &pad_storage_bytes(&scenario.pools),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            });
        let queues_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("tc_queues"),
                contents: &pad_storage_bytes(&scenario.queues),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            });
        let requests_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("tc_requests"),
                contents: &pad_storage_bytes(&scenario.requests),
                usage: BufferUsages::STORAGE,
            });
        let ranges_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("tc_ranges"),
                contents: &pad_storage_bytes(&scenario.pool_ranges),
                usage: BufferUsages::STORAGE,
            });

        let summary_count = ticks as usize * n_pools;
        let summary_zeros = vec![
            PoolTickSummary {
                pool: 0,
                tick: 0,
                amount_before: 0,
                amount_after: 0,
                total_requested: 0,
                total_allocated: 0,
                total_emitted_units: 0,
                active_requests: 0,
            };
            summary_count
        ];
        let summaries_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("tc_summaries"),
                contents: bytemuck::cast_slice(&summary_zeros),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            });

        let record_count = if write_records {
            ticks as usize * n_requests
        } else {
            0
        };
        let records_size = if write_records {
            record_count * std::mem::size_of::<TransferDeltaRecord>()
        } else {
            std::mem::size_of::<TransferDeltaRecord>()
        };
        let records_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("tc_records"),
            size: records_size.max(4) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let workgroups = ((n_pools as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let write_flag = if write_records {
            RECORD_MODE_COMPACT
        } else {
            RECORD_MODE_SUMMARY_ONLY
        };

        let mut uniform_buffers = Vec::with_capacity(ticks as usize);
        let mut bind_groups = Vec::with_capacity(ticks as usize);
        for tick in 0..ticks {
            let gpu_params = TransferResidentParams {
                n_pools: n_pools as u32,
                n_queues: n_queues as u32,
                n_requests: n_requests as u32,
                tick_index: tick,
                record_stride: n_requests as u32,
                summary_stride: n_pools as u32,
                write_records: write_flag,
                _pad0: 0,
            };
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("tc_uniform"),
                        contents: bytemuck::bytes_of(&gpu_params),
                        usage: BufferUsages::UNIFORM,
                    });
            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("tc_bg"),
                layout: &self.resident_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: pools_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: queues_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: requests_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: ranges_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: summaries_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 5,
                        resource: records_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 6,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            });
            uniform_buffers.push(uniform_buffer);
            bind_groups.push(bind_group);
        }

        let pools_size = (n_pools * std::mem::size_of::<PoolState>()) as u64;
        let queues_size = (n_queues * std::mem::size_of::<QueueState>()) as u64;
        let summaries_size = (summary_count * std::mem::size_of::<PoolTickSummary>()) as u64;
        let records_size_u64 = records_size as u64;

        let pools_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("tc_pools_rb"),
            size: pools_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let queues_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("tc_queues_rb"),
            size: queues_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let summaries_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("tc_summaries_rb"),
            size: summaries_size.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let records_readback = self.device.create_buffer(&BufferDescriptor {
            label: Some("tc_records_rb"),
            size: records_size_u64.max(4),
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let t_submit = Instant::now();
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("tc_enc"),
            });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("tc_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.resident_pipeline);
            for bind_group in &bind_groups {
                pass.set_bind_group(0, bind_group, &[]);
                pass.dispatch_workgroups(workgroups, 1, 1);
            }
        }
        encoder.copy_buffer_to_buffer(&pools_buffer, 0, &pools_readback, 0, pools_size.max(4));
        encoder.copy_buffer_to_buffer(&queues_buffer, 0, &queues_readback, 0, queues_size.max(4));
        encoder.copy_buffer_to_buffer(
            &summaries_buffer,
            0,
            &summaries_readback,
            0,
            summaries_size.max(4),
        );
        if write_records {
            encoder.copy_buffer_to_buffer(
                &records_buffer,
                0,
                &records_readback,
                0,
                records_size_u64,
            );
        }
        self.queue.submit(Some(encoder.finish()));
        let submit_us = t_submit.elapsed().as_micros() as u64;

        let final_pools = map_readback_pod::<PoolState>(&self.device, &pools_readback, n_pools)?;
        let final_queues =
            map_readback_pod::<QueueState>(&self.device, &queues_readback, n_queues)?;
        let summaries =
            map_readback_pod::<PoolTickSummary>(&self.device, &summaries_readback, summary_count)?;
        let records = if write_records {
            map_readback_pod::<TransferDeltaRecord>(&self.device, &records_readback, record_count)?
        } else {
            Vec::new()
        };
        let total_validation_us = t_total.elapsed().as_micros() as u64;

        let _ = uniform_buffers;

        Ok(TransferResidentResult {
            final_pools,
            final_queues,
            summaries,
            records,
            submit_us,
            total_validation_us,
        })
    }
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

fn pools_equal(a: &[PoolState], b: &[PoolState]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn queues_equal(a: &[QueueState], b: &[QueueState]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn summaries_equal(a: &[PoolTickSummary], b: &[PoolTickSummary]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
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

fn record_ticks_for(n_requests: usize, ticks: u32) -> (u32, bool) {
    let bytes =
        n_requests as u64 * ticks as u64 * std::mem::size_of::<TransferDeltaRecord>() as u64;
    if bytes > 128 * 1024 * 1024 {
        (8, true)
    } else {
        (ticks, false)
    }
}

fn scenario_totals(
    scenario: &TransferContentionScenario,
    final_pools: &[PoolState],
    final_queues: &[QueueState],
    records: &[TransferDeltaRecord],
) -> (u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let total_pool_before: u64 = scenario.pools.iter().map(|p| p.amount as u64).sum();
    let total_pool_after: u64 = final_pools.iter().map(|p| p.amount as u64).sum();
    let total_queue_accum_before: u64 = scenario.queues.iter().map(|q| q.accum as u64).sum();
    let total_queue_accum_after: u64 = final_queues.iter().map(|q| q.accum as u64).sum();
    let total_units_before: u64 = scenario.queues.iter().map(|q| q.units as u64).sum();
    let total_units_after: u64 = final_queues.iter().map(|q| q.units as u64).sum();

    let mut total_requested = 0u64;
    let mut total_allocated = 0u64;
    let mut total_emitted_value = 0u64;
    for record in records {
        if record.is_active == 0 {
            continue;
        }
        total_requested += record.requested as u64;
        total_allocated += record.allocated as u64;
        let q = scenario.queues[record.queue as usize];
        total_emitted_value += record.emitted_units as u64 * q.unit_cost as u64;
    }

    (
        total_pool_before,
        total_pool_after,
        total_queue_accum_before,
        total_queue_accum_after,
        total_units_before,
        total_units_after,
        total_requested,
        total_allocated,
        total_emitted_value,
    )
}

fn interpretation_string(
    conservation_ok: bool,
    priority_ok: bool,
    replay_ok: bool,
    deterministic: bool,
    final_summary: bool,
    summary_speedup: f32,
    records_fallback: bool,
) -> (String, String) {
    if !conservation_ok || !priority_ok || !replay_ok || !deterministic || !final_summary {
        return (
            "FAIL: GPU-resident contested allocation or replay validation failed.".to_string(),
            "FAIL".to_string(),
        );
    }

    let perf_gate = if summary_speedup >= 1.0 {
        "STRONG_PASS".to_string()
    } else {
        "WEAK_PASS".to_string()
    };

    let mut interpretation = if summary_speedup >= 1.0 {
        format!(
            "STRONG_PASS: GPU-resident pool-contention matches CPU ({:.2}x vs CPU on total_validation). Conservation, priority, and replay gates pass.",
            summary_speedup
        )
    } else {
        format!(
            "WEAK_PASS: Correctness passes but summary mode {:.2}x vs CPU on total_validation; perf unresolved under hotspot/sparse loads.",
            summary_speedup
        )
    };

    if records_fallback {
        interpretation.push_str(
            " Records mode used 8-tick fallback due to ~205MB compact record allocation limit.",
        );
    }

    interpretation.push_str(
        " v1 is pool-contention only (queue_cross_pool_contention=false); CPU-compiled ordering; not production AccumulatorOp; no atomics; not pure shader timing.",
    );

    (interpretation, perf_gate)
}

pub fn compare_transfer_contention_rich(
    scenario: &TransferContentionScenario,
    ticks: u32,
) -> Result<TransferContentionReport> {
    let harness = TransferContentionHarness::new()?;
    compare_transfer_contention_rich_with_harness(&harness, scenario, ticks)
}

pub fn compare_transfer_contention_rich_with_harness(
    harness: &TransferContentionHarness,
    scenario: &TransferContentionScenario,
    ticks: u32,
) -> Result<TransferContentionReport> {
    let n_pools = scenario.pools.len();
    let n_queues = scenario.queues.len();
    let n_requests = scenario.requests.len();
    let (record_ticks, records_memory_fallback) = record_ticks_for(n_requests, ticks);

    let t0 = Instant::now();
    let (cpu_final_pools, cpu_final_queues, cpu_summaries, cpu_records) =
        resolve_cpu_contention_n_ticks(scenario, ticks);
    let cpu_n_ticks_us = t0.elapsed().as_micros() as u64;

    let (cpu_final_pools_records, cpu_final_queues_records, _, cpu_records_subset) =
        if record_ticks < ticks {
            let (p, q, s, r) = resolve_cpu_contention_n_ticks(scenario, record_ticks);
            (p, q, s, r)
        } else {
            (
                cpu_final_pools.clone(),
                cpu_final_queues.clone(),
                cpu_summaries.clone(),
                cpu_records.clone(),
            )
        };

    let t_rec = Instant::now();
    let _ = resolve_cpu_contention_n_ticks(scenario, record_ticks);
    let cpu_records_ticks_us = t_rec.elapsed().as_micros() as u64;

    let _ = harness.run_gpu_resident(scenario, ticks, false)?;

    let mut summary_submit_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut summary_total_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut records_submit_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut records_total_warm = Vec::with_capacity(RESIDENT_WARM_RUNS);
    let mut gpu_summary_base: Option<TransferResidentResult> = None;
    let mut gpu_records_base: Option<TransferResidentResult> = None;
    let mut deterministic = true;

    for _ in 0..RESIDENT_WARM_RUNS {
        let gpu_summary = harness.run_gpu_resident(scenario, ticks, false)?;
        summary_submit_warm.push(gpu_summary.submit_us);
        summary_total_warm.push(gpu_summary.total_validation_us);

        match &gpu_summary_base {
            None => gpu_summary_base = Some(gpu_summary),
            Some(base) => {
                if !pools_equal(&base.final_pools, &gpu_summary.final_pools)
                    || !queues_equal(&base.final_queues, &gpu_summary.final_queues)
                    || !summaries_equal(&base.summaries, &gpu_summary.summaries)
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
                if !pools_equal(&base.final_pools, &gpu_records.final_pools)
                    || !queues_equal(&base.final_queues, &gpu_records.final_queues)
                    || base.records != gpu_records.records
                {
                    deterministic = false;
                }
            }
        }
    }

    let gpu_summary = gpu_summary_base.unwrap();
    let gpu_records = gpu_records_base.unwrap();

    let final_state_matches_cpu_summary_mode =
        pools_equal(&cpu_final_pools, &gpu_summary.final_pools)
            && queues_equal(&cpu_final_queues, &gpu_summary.final_queues);
    let final_state_matches_cpu_records_mode =
        pools_equal(&cpu_final_pools_records, &gpu_records.final_pools)
            && queues_equal(&cpu_final_queues_records, &gpu_records.final_queues);
    let summaries_match_cpu = summaries_equal(&cpu_summaries, &gpu_summary.summaries);

    let replayed = replay_transfer_records_n_ticks(
        &scenario.pools,
        &scenario.queues,
        &scenario.requests,
        &gpu_records.records,
        record_ticks,
        n_requests,
    )?;
    let replay_from_records_matches_gpu = pools_equal(&replayed.0, &gpu_records.final_pools)
        && queues_equal(&replayed.1, &gpu_records.final_queues);

    let conservation_ok = conservation_check_contention_records(
        &scenario.pools,
        &gpu_summary.final_pools,
        &scenario.queues,
        &gpu_summary.final_queues,
        &scenario.queues,
        &cpu_records,
    );
    let priority_ok = priority_allocation_check(scenario, &scenario.pools, &cpu_records, ticks);

    let conservation_gate = if conservation_ok && final_state_matches_cpu_summary_mode {
        "PASS"
    } else {
        "FAIL"
    };
    let priority_gate = if priority_ok { "PASS" } else { "FAIL" };
    let replay_gate = if replay_from_records_matches_gpu && final_state_matches_cpu_records_mode {
        "PASS"
    } else {
        "FAIL"
    };
    let determinism_gate = if deterministic { "PASS" } else { "FAIL" };

    let (summary_submit_mean, _, _) = warm_stats(&summary_submit_warm);
    let (summary_total_mean, _, _) = warm_stats(&summary_total_warm);
    let (records_submit_mean, _, _) = warm_stats(&records_submit_warm);
    let (records_total_mean, _, _) = warm_stats(&records_total_warm);

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

    let summary_bytes_total = ticks as usize * n_pools * std::mem::size_of::<PoolTickSummary>();
    let compact_record_bytes_total =
        record_ticks as usize * n_requests * std::mem::size_of::<TransferDeltaRecord>();

    let (interpretation, performance_gate) = interpretation_string(
        conservation_gate == "PASS",
        priority_gate == "PASS",
        replay_gate == "PASS",
        determinism_gate == "PASS",
        final_state_matches_cpu_summary_mode,
        summary_speedup,
        records_memory_fallback,
    );

    let (
        total_pool_before,
        total_pool_after,
        total_queue_accum_before,
        total_queue_accum_after,
        total_units_before,
        total_units_after,
        total_requested,
        total_allocated,
        total_emitted_value,
    ) = scenario_totals(
        scenario,
        &gpu_summary.final_pools,
        &gpu_summary.final_queues,
        &cpu_records,
    );

    let _ = (cpu_records_subset, summary_submit_mean, records_submit_mean);

    Ok(TransferContentionReport {
        scenario_name: scenario.name.clone(),
        n_pools,
        n_queues,
        n_requests,
        ticks,
        record_ticks,
        records_memory_fallback,
        queue_cross_pool_contention: scenario.queue_cross_pool_contention,
        total_pool_before,
        total_pool_after,
        total_queue_accum_before,
        total_queue_accum_after,
        total_units_before,
        total_units_after,
        total_requested,
        total_allocated,
        total_emitted_value,
        cpu_n_ticks_us,
        gpu_summary_submit_mean_us: summary_submit_mean,
        gpu_summary_total_mean_us: summary_total_mean,
        gpu_summary_total_per_tick_us: summary_total_per_tick,
        gpu_records_submit_mean_us: records_submit_mean,
        gpu_records_total_mean_us: records_total_mean,
        gpu_records_total_per_tick_us: records_total_per_tick,
        cpu_per_tick_us: cpu_per_tick,
        summary_speedup_vs_cpu: summary_speedup,
        records_speedup_vs_cpu: records_speedup,
        final_state_matches_cpu_summary_mode,
        final_state_matches_cpu_records_mode,
        summaries_match_cpu,
        replay_from_records_matches_gpu,
        conservation_gate: conservation_gate.to_string(),
        priority_gate: priority_gate.to_string(),
        replay_gate: replay_gate.to_string(),
        determinism_gate: determinism_gate.to_string(),
        performance_gate,
        summary_bytes_total,
        compact_record_bytes_total,
        interpretation,
        timing_note: TIMING_NOTE.to_string(),
    })
}

pub fn format_transfer_contention_report(report: &TransferContentionReport) -> String {
    crate::transfer_contention_report::format_transfer_contention_report(report)
}

pub fn write_transfer_contention_reports(report: &TransferContentionReport) -> Result<()> {
    crate::transfer_contention_report::write_transfer_contention_reports(report).map_err(Into::into)
}

pub fn write_transfer_contention_reports_bundle(
    reports: &[TransferContentionReport],
) -> Result<()> {
    crate::transfer_contention_report::write_transfer_contention_reports_bundle(reports)
        .map_err(Into::into)
}
