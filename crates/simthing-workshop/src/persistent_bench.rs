//! Persistent-buffer / timestamp benchmark — CPU boundary vs GPU envelope vs GPU-resident pivot.

use std::time::Instant;

use anyhow::{bail, Context, Result};
use bytemuck::{Pod, Zeroable};
use serde::Serialize;
use wgpu::util::DeviceExt;
use wgpu::{
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, Device,
    DeviceDescriptor, Features, Instance, InstanceDescriptor, Maintain, MapMode, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, QuerySetDescriptor, QueryType, Queue,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderStages,
};

pub const WORKGROUP_SIZE: u32 = 64;
pub const DEFAULT_TICKS: u32 = 64;
pub const WARM_RUNS: usize = 5;

pub const LOG_MODE_SUMMARY_ONLY: u32 = 0;
pub const LOG_MODE_COMPACT_RECORDS: u32 = 1;

pub const TIMING_NOTE: &str =
    "total_validation includes upload, encode, submit, GPU wait, and readback. Timestamp queries measure GPU dispatch block only when supported.";

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct BenchPool {
    pub amount: u32,
    pub regen_per_tick: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct BenchQueue {
    pub pool: u32,
    pub accum: u32,
    pub units: u32,
    pub unit_cost: u32,
    pub request_per_tick: u32,
    pub priority: u32,
    pub is_active: u32,
    pub _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct BenchPoolRange {
    pub start: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct BenchTickSummary {
    pub tick: u32,
    pub total_pool_before: u32,
    pub total_pool_after: u32,
    pub total_allocated: u32,
    pub total_emitted_units: u32,
    pub active_queues: u32,
    pub _pad0: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, PartialEq, Eq)]
pub struct BenchRecord {
    pub tick: u32,
    pub queue: u32,
    pub pool: u32,
    pub requested: u32,
    pub allocated: u32,
    pub emitted_units: u32,
    pub is_active: u32,
    pub _pad0: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BenchParams {
    pub n_pools: u32,
    pub n_queues: u32,
    pub tick_index: u32,
    pub log_mode: u32,
    pub record_stride: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}

#[derive(Debug, Clone)]
pub struct PersistentBenchScenario {
    pub name: String,
    pub pools: Vec<BenchPool>,
    pub queues: Vec<BenchQueue>,
    pub pool_ranges: Vec<BenchPoolRange>,
}

#[derive(Debug, Clone)]
pub struct PersistentGpuResult {
    pub final_pools: Vec<BenchPool>,
    pub final_queues: Vec<BenchQueue>,
    pub summaries: Vec<BenchTickSummary>,
    pub records: Vec<BenchRecord>,
    pub total_validation_us: u64,
    pub submit_us: u64,
    pub timestamp_total_ns: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PersistentBenchReport {
    pub scenario_name: String,
    pub n_pools: usize,
    pub n_queues: usize,
    pub ticks: u32,
    pub log_mode: String,
    pub timestamp_supported: bool,
    pub timestamp_period_ns: f32,
    pub correctness_gate: String,
    pub conservation_gate: String,
    pub determinism_gate: String,
    pub timestamp_gate: String,
    pub performance_gate: String,
    pub cpu_total_us: u64,
    pub cpu_per_tick_us: f32,
    pub current_gpu_envelope_total_us: u64,
    pub current_gpu_envelope_per_tick_us: f32,
    pub pivot_total_validation_mean_us: u64,
    pub pivot_total_validation_min_us: u64,
    pub pivot_total_validation_max_us: u64,
    pub pivot_total_validation_per_tick_us: f32,
    pub pivot_timestamp_total_ns: u64,
    pub pivot_timestamp_per_tick_ns: f32,
    pub current_gpu_speedup_vs_cpu: f32,
    pub pivot_validation_speedup_vs_cpu: f32,
    pub pivot_validation_speedup_vs_current_gpu: f32,
    pub final_state_matches_cpu: bool,
    pub summaries_match_cpu: bool,
    pub repeated_pivot_outputs_identical: bool,
    pub summary_bytes_total: usize,
    pub record_bytes_total: usize,
    pub final_state_bytes: usize,
    pub interpretation: String,
    pub timing_note: String,
}

fn mix_u32(seed: u32) -> u32 {
    seed.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

pub fn sort_queues_and_build_ranges(queues: &mut [BenchQueue], n_pools: usize) -> Vec<BenchPoolRange> {
    let mut indexed: Vec<(usize, BenchQueue)> = queues
        .iter()
        .copied()
        .enumerate()
        .map(|(i, q)| (i, q))
        .collect();
    indexed.sort_by(|(ai, a), (bi, b)| {
        a.pool
            .cmp(&b.pool)
            .then(a.priority.cmp(&b.priority))
            .then((*ai as u32).cmp(&(*bi as u32)))
    });
    for (i, (_, q)) in indexed.iter().enumerate() {
        queues[i] = *q;
    }

    let mut ranges = vec![BenchPoolRange { start: 0, count: 0 }; n_pools];
    for (idx, q) in queues.iter().enumerate() {
        let pool = q.pool as usize;
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

pub fn resolve_cpu_persistent_bench(
    scenario: &PersistentBenchScenario,
    ticks: u32,
    log_records: bool,
) -> (
    Vec<BenchPool>,
    Vec<BenchQueue>,
    Vec<BenchTickSummary>,
    Vec<BenchRecord>,
) {
    let n_queues = scenario.queues.len();
    let mut pools = scenario.pools.clone();
    let mut queues = scenario.queues.clone();
    let mut summaries = Vec::with_capacity(ticks as usize);
    let mut records = if log_records {
        vec![
            BenchRecord {
                tick: 0,
                queue: 0,
                pool: 0,
                requested: 0,
                allocated: 0,
                emitted_units: 0,
                is_active: 0,
                _pad0: 0,
            };
            ticks as usize * n_queues
        ]
    } else {
        Vec::new()
    };

    for tick in 0..ticks {
        let mut tick_pool_before = 0u32;
        let mut tick_pool_after = 0u32;
        let mut tick_allocated = 0u32;
        let mut tick_emitted = 0u32;
        let mut tick_active = 0u32;

        for pool_idx in 0..scenario.pools.len() {
            let before = pools[pool_idx].amount;
            pools[pool_idx].amount = pools[pool_idx].amount.saturating_add(pools[pool_idx].regen_per_tick);
            let range = scenario.pool_ranges[pool_idx];

            for i in 0..range.count {
                let q_idx = (range.start + i) as usize;
                let mut q = queues[q_idx];

                let mut allocated = 0u32;
                let mut emitted = 0u32;
                let mut active = 0u32;

                if q.is_active != 0 && q.unit_cost != 0 {
                    active = 1;
                    tick_active += 1;
                    allocated = pools[pool_idx].amount.min(q.request_per_tick);
                    pools[pool_idx].amount -= allocated;

                    let pre_emit = q.accum.saturating_add(allocated);
                    emitted = pre_emit / q.unit_cost;
                    q.accum = pre_emit - emitted * q.unit_cost;
                    q.units += emitted;
                    queues[q_idx] = q;

                    tick_allocated = tick_allocated.saturating_add(allocated);
                    tick_emitted = tick_emitted.saturating_add(emitted);
                }

                if log_records {
                    records[tick as usize * n_queues + q_idx] = BenchRecord {
                        tick,
                        queue: q_idx as u32,
                        pool: pool_idx as u32,
                        requested: q.request_per_tick,
                        allocated,
                        emitted_units: emitted,
                        is_active: active,
                        _pad0: 0,
                    };
                }
            }

            tick_pool_before = tick_pool_before.saturating_add(before);
            tick_pool_after = tick_pool_after.saturating_add(pools[pool_idx].amount);
        }

        summaries.push(BenchTickSummary {
            tick,
            total_pool_before: tick_pool_before,
            total_pool_after: tick_pool_after,
            total_allocated: tick_allocated,
            total_emitted_units: tick_emitted,
            active_queues: tick_active,
            _pad0: 0,
            _pad1: 0,
        });
    }

    (pools, queues, summaries, records)
}

pub fn replay_bench_records(
    initial_pools: &[BenchPool],
    initial_queues: &[BenchQueue],
    records: &[BenchRecord],
    ticks: u32,
    n_queues: usize,
) -> Result<(Vec<BenchPool>, Vec<BenchQueue>)> {
    if records.len() != ticks as usize * n_queues {
        bail!("record length mismatch");
    }

    let mut pools = initial_pools.to_vec();
    let mut queues = initial_queues.to_vec();

    for tick in 0..ticks as usize {
        for pool in pools.iter_mut() {
            pool.amount = pool.amount.saturating_add(pool.regen_per_tick);
        }
        for q_idx in 0..n_queues {
            let rec = records[tick * n_queues + q_idx];
            if rec.queue as usize != q_idx {
                bail!("record queue mismatch");
            }
            if rec.is_active == 0 {
                continue;
            }
            let pool_idx = rec.pool as usize;
            if rec.allocated > pools[pool_idx].amount {
                bail!("allocated exceeds pool");
            }
            pools[pool_idx].amount -= rec.allocated;
            let q = &mut queues[q_idx];
            let pre_emit = q.accum.saturating_add(rec.allocated);
            q.accum = pre_emit - rec.emitted_units * q.unit_cost;
            q.units += rec.emitted_units;
        }
    }

    Ok((pools, queues))
}

pub fn conservation_check(
    initial_pools: &[BenchPool],
    final_pools: &[BenchPool],
    initial_queues: &[BenchQueue],
    final_queues: &[BenchQueue],
    ticks: u32,
) -> bool {
    let pool_before: u64 = initial_pools.iter().map(|p| p.amount as u64).sum();
    let pool_after: u64 = final_pools.iter().map(|p| p.amount as u64).sum();
    let accum_before: u64 = initial_queues.iter().map(|q| q.accum as u64).sum();
    let accum_after: u64 = final_queues.iter().map(|q| q.accum as u64).sum();

    let regen_total: u64 = initial_pools
        .iter()
        .map(|p| p.regen_per_tick as u64 * ticks as u64)
        .sum();

    let mut emitted_value = 0u64;
    for (q0, q1) in initial_queues.iter().zip(final_queues.iter()) {
        if q1.units >= q0.units {
            emitted_value += (q1.units - q0.units) as u64 * q0.unit_cost as u64;
        }
    }

    pool_before + regen_total + accum_before
        == pool_after + accum_after + emitted_value
}

pub fn make_persistent_bench_scenario(
    name: &str,
    n_pools: usize,
    n_queues: usize,
    active_ratio: f32,
    hotspot: bool,
    regen: bool,
) -> PersistentBenchScenario {
    assert!(n_pools > 0 && n_queues > 0);
    let active_ratio = active_ratio.clamp(0.0, 1.0);
    let hot_pools = n_pools.min(16);

    let mut pools = Vec::with_capacity(n_pools);
    for pool_idx in 0..n_pools {
        let h = mix_u32(pool_idx as u32 + 1);
        let amount = if hotspot && pool_idx < hot_pools {
            (h % 500) + 500
        } else {
            (h % 4501) + 500
        };
        let regen_per_tick = if regen {
            if hotspot && pool_idx < hot_pools {
                (h % 901) + 100
            } else {
                (h % 901) + 100
            }
        } else {
            0
        };
        pools.push(BenchPool {
            amount,
            regen_per_tick,
            _pad0: 0,
            _pad1: 0,
        });
    }

    let mut queues = Vec::with_capacity(n_queues);
    for queue_idx in 0..n_queues {
        let h = mix_u32(queue_idx as u32 + 17);
        let pool = if hotspot {
            (queue_idx % hot_pools) as u32
        } else {
            (queue_idx % n_pools) as u32
        };
        let unit_cost = 10 + (h % 191);
        let is_active = if (queue_idx as f32) / (n_queues as f32) < active_ratio {
            1
        } else {
            0
        };
        queues.push(BenchQueue {
            pool,
            accum: (h / 7) % (unit_cost.saturating_mul(2).max(1) + 1),
            units: (h / 13) % 1001,
            unit_cost,
            request_per_tick: (h / 3) % 501,
            priority: (h / 11) % 101,
            is_active,
            _pad0: 0,
        });
    }

    let pool_ranges = sort_queues_and_build_ranges(&mut queues, n_pools);

    PersistentBenchScenario {
        name: name.to_string(),
        pools,
        queues,
        pool_ranges,
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

fn parse_summaries(raw: &[u32], ticks: u32) -> Vec<BenchTickSummary> {
    (0..ticks as usize)
        .map(|t| {
            let base = t * 6;
            BenchTickSummary {
                tick: t as u32,
                total_pool_before: raw[base],
                total_pool_after: raw[base + 1],
                total_allocated: raw[base + 2],
                total_emitted_units: raw[base + 3],
                active_queues: raw[base + 4],
                _pad0: 0,
                _pad1: 0,
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

fn pools_equal(a: &[BenchPool], b: &[BenchPool]) -> bool {
    a == b
}

fn queues_equal(a: &[BenchQueue], b: &[BenchQueue]) -> bool {
    a == b
}

fn summaries_equal(a: &[BenchTickSummary], b: &[BenchTickSummary]) -> bool {
    a == b
}

fn warm_stats(samples: &[u64]) -> (u64, u64, u64) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let sum: u64 = samples.iter().sum();
    (sum / samples.len() as u64, *samples.iter().min().unwrap(), *samples.iter().max().unwrap())
}

struct GpuTickBuffers {
    pools_buffer: Buffer,
    queues_buffer: Buffer,
    ranges_buffer: Buffer,
    accum_buffer: Buffer,
    records_buffer: Buffer,
    pools_readback: Buffer,
    queues_readback: Buffer,
    accum_readback: Buffer,
    records_readback: Buffer,
    record_count: usize,
    records_size: u64,
}

pub struct PersistentBenchHarness {
    device: Device,
    queue: Queue,
    pipeline: wgpu::ComputePipeline,
    layout: wgpu::BindGroupLayout,
    timestamp_supported: bool,
    timestamp_period_ns: f32,
    query_set: Option<wgpu::QuerySet>,
    timestamp_resolve: Option<Buffer>,
    timestamp_readback: Option<Buffer>,
}

impl PersistentBenchHarness {
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

        let timestamp_supported = adapter.features().contains(Features::TIMESTAMP_QUERY);
        let required_features = if timestamp_supported {
            Features::TIMESTAMP_QUERY
        } else {
            Features::empty()
        };

        let limits = adapter.limits();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("simthing-workshop persistent_bench"),
                    required_features,
                    required_limits: limits,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let timestamp_period_ns = queue.get_timestamp_period();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("persistent_bench"),
            source: wgpu::ShaderSource::Wgsl(include_str!("persistent_bench_gpu.wgsl").into()),
        });

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("persistent_bench_layout"),
            entries: &[
                storage_entry(0, false),
                storage_entry(1, false),
                storage_entry(2, true),
                storage_entry(3, false),
                storage_entry(4, false),
                uniform_entry(5),
            ],
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("persistent_bench_pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("persistent_bench_pl"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            })),
            module: &shader,
            entry_point: "persistent_tick",
            compilation_options: Default::default(),
            cache: None,
        });

        let (query_set, timestamp_resolve, timestamp_readback) = if timestamp_supported {
            let query_set = device.create_query_set(&QuerySetDescriptor {
                label: Some("persistent_bench_ts"),
                ty: QueryType::Timestamp,
                count: 2,
            });
            let timestamp_resolve = device.create_buffer(&BufferDescriptor {
                label: Some("persistent_bench_ts_resolve"),
                size: 16,
                usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
            let timestamp_readback = device.create_buffer(&BufferDescriptor {
                label: Some("persistent_bench_ts_readback"),
                size: 16,
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            (Some(query_set), Some(timestamp_resolve), Some(timestamp_readback))
        } else {
            (None, None, None)
        };

        Ok(Self {
            device,
            queue,
            pipeline,
            layout,
            timestamp_supported,
            timestamp_period_ns,
            query_set,
            timestamp_resolve,
            timestamp_readback,
        })
    }

    fn create_tick_buffers(
        &self,
        scenario: &PersistentBenchScenario,
        ticks: u32,
        log_records: bool,
        initial_pools: &[BenchPool],
        initial_queues: &[BenchQueue],
    ) -> Result<GpuTickBuffers> {
        let n_pools = scenario.pools.len();
        let n_queues = scenario.queues.len();
        let record_count = if log_records {
            ticks as usize * n_queues
        } else {
            0
        };
        let records_size = if log_records {
            (record_count * std::mem::size_of::<BenchRecord>()) as u64
        } else {
            std::mem::size_of::<BenchRecord>() as u64
        };

        let pools_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("pb_pools"),
            contents: &pad_storage_bytes(initial_pools),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        });
        let queues_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("pb_queues"),
            contents: &pad_storage_bytes(initial_queues),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        });
        let ranges_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("pb_ranges"),
            contents: &pad_storage_bytes(&scenario.pool_ranges),
            usage: BufferUsages::STORAGE,
        });

        let accum_u32s = (ticks as usize) * 6;
        let accum_zeros = vec![0u32; accum_u32s];
        let accum_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("pb_accum"),
            contents: bytemuck::cast_slice(&accum_zeros),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        });

        let records_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("pb_records"),
            size: records_size.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let pools_size = (n_pools * std::mem::size_of::<BenchPool>()) as u64;
        let queues_size = (n_queues * std::mem::size_of::<BenchQueue>()) as u64;
        let accum_size = (accum_u32s * std::mem::size_of::<u32>()) as u64;

        Ok(GpuTickBuffers {
            pools_buffer,
            queues_buffer,
            ranges_buffer,
            accum_buffer,
            records_buffer,
            pools_readback: self.device.create_buffer(&BufferDescriptor {
                label: Some("pb_pools_rb"),
                size: pools_size.max(4),
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            queues_readback: self.device.create_buffer(&BufferDescriptor {
                label: Some("pb_queues_rb"),
                size: queues_size.max(4),
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            accum_readback: self.device.create_buffer(&BufferDescriptor {
                label: Some("pb_accum_rb"),
                size: accum_size.max(4),
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            records_readback: self.device.create_buffer(&BufferDescriptor {
                label: Some("pb_records_rb"),
                size: records_size.max(4),
                usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            record_count,
            records_size,
        })
    }

    fn dispatch_ticks(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bufs: &GpuTickBuffers,
        scenario: &PersistentBenchScenario,
        ticks: u32,
        log_records: bool,
        timestamp_writes: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) -> Result<()> {
        let n_pools = scenario.pools.len();
        let n_queues = scenario.queues.len();
        let workgroups = ((n_pools as u32) + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
        let log_mode = if log_records {
            LOG_MODE_COMPACT_RECORDS
        } else {
            LOG_MODE_SUMMARY_ONLY
        };

        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("pb_pass"),
            timestamp_writes,
        });

        for tick in 0..ticks {
            let gpu_params = BenchParams {
                n_pools: n_pools as u32,
                n_queues: n_queues as u32,
                tick_index: tick,
                log_mode,
                record_stride: n_queues as u32,
                _pad0: 0,
                _pad1: 0,
                _pad2: 0,
            };
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("pb_uniform"),
                        contents: bytemuck::bytes_of(&gpu_params),
                        usage: BufferUsages::UNIFORM,
                    });
            let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
                label: Some("pb_bg"),
                layout: &self.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: bufs.pools_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: bufs.queues_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: bufs.ranges_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: bufs.accum_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: bufs.records_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 5,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }
        drop(pass);
        Ok(())
    }

    fn readback_results(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bufs: &GpuTickBuffers,
        n_pools: usize,
        n_queues: usize,
        ticks: u32,
        log_records: bool,
    ) -> Result<()> {
        let pools_size = (n_pools * std::mem::size_of::<BenchPool>()) as u64;
        let queues_size = (n_queues * std::mem::size_of::<BenchQueue>()) as u64;
        let accum_size = (ticks as usize * 6 * std::mem::size_of::<u32>()) as u64;

        encoder.copy_buffer_to_buffer(&bufs.pools_buffer, 0, &bufs.pools_readback, 0, pools_size.max(4));
        encoder.copy_buffer_to_buffer(
            &bufs.queues_buffer,
            0,
            &bufs.queues_readback,
            0,
            queues_size.max(4),
        );
        encoder.copy_buffer_to_buffer(
            &bufs.accum_buffer,
            0,
            &bufs.accum_readback,
            0,
            accum_size.max(4),
        );
        if log_records {
            encoder.copy_buffer_to_buffer(
                &bufs.records_buffer,
                0,
                &bufs.records_readback,
                0,
                bufs.records_size,
            );
        }
        Ok(())
    }

    fn finish_readback(
        &self,
        bufs: &GpuTickBuffers,
        n_pools: usize,
        n_queues: usize,
        ticks: u32,
        log_records: bool,
    ) -> Result<(Vec<BenchPool>, Vec<BenchQueue>, Vec<BenchTickSummary>, Vec<BenchRecord>)> {
        let final_pools = map_readback_pod::<BenchPool>(&self.device, &bufs.pools_readback, n_pools)?;
        let final_queues =
            map_readback_pod::<BenchQueue>(&self.device, &bufs.queues_readback, n_queues)?;
        let accum_raw =
            map_readback_pod::<u32>(&self.device, &bufs.accum_readback, ticks as usize * 6)?;
        let summaries = parse_summaries(&accum_raw, ticks);
        let records = if log_records {
            map_readback_pod::<BenchRecord>(
                &self.device,
                &bufs.records_readback,
                bufs.record_count,
            )?
        } else {
            Vec::new()
        };
        Ok((final_pools, final_queues, summaries, records))
    }

    pub fn run_current_gpu_envelope(
        &self,
        scenario: &PersistentBenchScenario,
        ticks: u32,
        log_records: bool,
    ) -> Result<(Vec<BenchPool>, Vec<BenchQueue>, Vec<BenchTickSummary>, Vec<BenchRecord>, u64)> {
        let t_total = Instant::now();
        let n_pools = scenario.pools.len();
        let n_queues = scenario.queues.len();

        let mut pools = scenario.pools.clone();
        let mut queues = scenario.queues.clone();
        let mut all_summaries = Vec::with_capacity(ticks as usize);
        let mut all_records = if log_records {
            vec![
                BenchRecord {
                    tick: 0,
                    queue: 0,
                    pool: 0,
                    requested: 0,
                    allocated: 0,
                    emitted_units: 0,
                    is_active: 0,
                    _pad0: 0,
                };
                ticks as usize * n_queues
            ]
        } else {
            Vec::new()
        };

        for tick in 0..ticks {
            let bufs = self.create_tick_buffers(scenario, 1, log_records, &pools, &queues)?;
            let mut encoder = self
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("pb_envelope_enc"),
                });
            self.dispatch_ticks(&mut encoder, &bufs, scenario, 1, log_records, None)?;
            self.readback_results(&mut encoder, &bufs, n_pools, n_queues, 1, log_records)?;
            self.queue.submit(Some(encoder.finish()));

            let (new_pools, new_queues, summaries, records) =
                self.finish_readback(&bufs, n_pools, n_queues, 1, log_records)?;
            pools = new_pools;
            queues = new_queues;
            all_summaries.push(BenchTickSummary {
                tick,
                ..summaries[0]
            });
            if log_records {
                for q in 0..n_queues {
                    all_records[tick as usize * n_queues + q] = records[q];
                }
            }
        }

        let elapsed = t_total.elapsed().as_micros() as u64;
        Ok((pools, queues, all_summaries, all_records, elapsed))
    }

    pub fn run_pivot_gpu_resident(
        &self,
        scenario: &PersistentBenchScenario,
        ticks: u32,
        log_records: bool,
        use_timestamps: bool,
    ) -> Result<PersistentGpuResult> {
        let t_total = Instant::now();
        let n_pools = scenario.pools.len();
        let n_queues = scenario.queues.len();

        let bufs = self.create_tick_buffers(
            scenario,
            ticks,
            log_records,
            &scenario.pools,
            &scenario.queues,
        )?;

        let timestamp_writes = if use_timestamps && self.timestamp_supported {
            Some(wgpu::ComputePassTimestampWrites {
                query_set: self.query_set.as_ref().unwrap(),
                beginning_of_pass_write_index: Some(0),
                end_of_pass_write_index: Some(1),
            })
        } else {
            None
        };

        let t_submit = Instant::now();
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("pb_pivot_enc"),
            });
        self.dispatch_ticks(
            &mut encoder,
            &bufs,
            scenario,
            ticks,
            log_records,
            timestamp_writes,
        )?;
        self.readback_results(&mut encoder, &bufs, n_pools, n_queues, ticks, log_records)?;

        if use_timestamps && self.timestamp_supported {
            let query_set = self.query_set.as_ref().unwrap();
            let resolve = self.timestamp_resolve.as_ref().unwrap();
            let readback = self.timestamp_readback.as_ref().unwrap();
            encoder.resolve_query_set(query_set, 0..2, resolve, 0);
            encoder.copy_buffer_to_buffer(resolve, 0, readback, 0, 16);
        }

        self.queue.submit(Some(encoder.finish()));
        let submit_us = t_submit.elapsed().as_micros() as u64;

        let (final_pools, final_queues, summaries, records) =
            self.finish_readback(&bufs, n_pools, n_queues, ticks, log_records)?;

        let timestamp_total_ns = if use_timestamps && self.timestamp_supported {
            let readback = self.timestamp_readback.as_ref().unwrap();
            let stamps = map_readback_pod::<u64>(&self.device, readback, 2)?;
            if stamps.len() >= 2 && stamps[1] >= stamps[0] {
                Some(((stamps[1] - stamps[0]) as f32 * self.timestamp_period_ns) as u64)
            } else {
                None
            }
        } else {
            None
        };

        let total_validation_us = t_total.elapsed().as_micros() as u64;

        Ok(PersistentGpuResult {
            final_pools,
            final_queues,
            summaries,
            records,
            total_validation_us,
            submit_us,
            timestamp_total_ns,
        })
    }
}

fn interpretation_string(
    correctness_ok: bool,
    conservation_ok: bool,
    deterministic: bool,
    pivot_vs_cpu: f32,
    pivot_vs_envelope: f32,
    timestamp_supported: bool,
    timestamp_gate: &str,
) -> (String, String) {
    if !correctness_ok || !conservation_ok || !deterministic {
        return (
            "FAIL: persistent benchmark correctness, conservation, or determinism failed.".to_string(),
            "FAIL".to_string(),
        );
    }

    let perf = if pivot_vs_envelope >= 1.0 && pivot_vs_cpu >= 0.5 {
        "STRONG_PASS"
    } else if pivot_vs_envelope >= 1.0 {
        "WEAK_PASS"
    } else {
        "WEAK_PASS"
    };

    let mut interpretation = format!(
        "Correctness and conservation pass. Pivot resident {:.2}x vs current GPU envelope, {:.2}x vs CPU on total_validation.",
        pivot_vs_envelope, pivot_vs_cpu
    );
    if timestamp_supported {
        interpretation.push_str(&format!(" timestamp_gate: {}.", timestamp_gate));
    } else {
        interpretation.push_str(" timestamp queries unavailable; timestamp_gate SKIPPED.");
    }
    interpretation.push_str(
        " Workshop-local; not production AccumulatorOp; one-invocation-per-pool; hotspot caveat applies.",
    );
    (interpretation, perf.to_string())
}

pub fn compare_persistent_bench_rich(
    scenario: &PersistentBenchScenario,
    ticks: u32,
    log_records: bool,
) -> Result<PersistentBenchReport> {
    let harness = PersistentBenchHarness::new()?;
    compare_persistent_bench_rich_with_harness(&harness, scenario, ticks, log_records)
}

pub fn compare_persistent_bench_rich_with_harness(
    harness: &PersistentBenchHarness,
    scenario: &PersistentBenchScenario,
    ticks: u32,
    log_records: bool,
) -> Result<PersistentBenchReport> {
    let n_pools = scenario.pools.len();
    let n_queues = scenario.queues.len();
    let log_mode = if log_records {
        "compact_records"
    } else {
        "summary_only"
    };

    let t0 = Instant::now();
    let (cpu_pools, cpu_queues, cpu_summaries, _cpu_records) =
        resolve_cpu_persistent_bench(scenario, ticks, log_records);
    let cpu_total_us = t0.elapsed().as_micros() as u64;

    let (env_pools, env_queues, env_summaries, _, envelope_us) =
        harness.run_current_gpu_envelope(scenario, ticks, log_records)?;

    let _ = harness.run_pivot_gpu_resident(scenario, ticks, log_records, harness.timestamp_supported)?;

    let mut pivot_warm = Vec::with_capacity(WARM_RUNS);
    let mut pivot_base: Option<PersistentGpuResult> = None;
    let mut deterministic = true;

    for _ in 0..WARM_RUNS {
        let pivot = harness.run_pivot_gpu_resident(
            scenario,
            ticks,
            log_records,
            harness.timestamp_supported,
        )?;
        pivot_warm.push(pivot.total_validation_us);
        match &pivot_base {
            None => pivot_base = Some(pivot),
            Some(base) => {
                if !pools_equal(&base.final_pools, &pivot.final_pools)
                    || !queues_equal(&base.final_queues, &pivot.final_queues)
                    || !summaries_equal(&base.summaries, &pivot.summaries)
                {
                    deterministic = false;
                }
            }
        }
    }

    let pivot = pivot_base.unwrap();
    let final_state_matches_cpu =
        pools_equal(&cpu_pools, &pivot.final_pools) && queues_equal(&cpu_queues, &pivot.final_queues);
    let summaries_match_cpu = summaries_equal(&cpu_summaries, &pivot.summaries);
    let envelope_matches = pools_equal(&cpu_pools, &env_pools)
        && queues_equal(&cpu_queues, &env_queues)
        && summaries_equal(&cpu_summaries, &env_summaries);

    let conservation_ok = conservation_check(
        &scenario.pools,
        &pivot.final_pools,
        &scenario.queues,
        &pivot.final_queues,
        ticks,
    );

    let correctness_gate = if final_state_matches_cpu && summaries_match_cpu && envelope_matches {
        "PASS"
    } else {
        "FAIL"
    };
    let conservation_gate = if conservation_ok { "PASS" } else { "FAIL" };
    let determinism_gate = if deterministic { "PASS" } else { "FAIL" };

    let timestamp_gate = if harness.timestamp_supported {
        if pivot.timestamp_total_ns.is_some() {
            "PASS"
        } else {
            "WEAK_PASS"
        }
    } else {
        "SKIPPED"
    };

    let (pivot_mean, pivot_min, pivot_max) = warm_stats(&pivot_warm);
    let cpu_per_tick = cpu_total_us as f32 / ticks as f32;
    let envelope_per_tick = envelope_us as f32 / ticks as f32;
    let pivot_per_tick = pivot_mean as f32 / ticks as f32;

    let current_gpu_speedup_vs_cpu = if envelope_us > 0 {
        cpu_total_us as f32 / envelope_us as f32
    } else {
        0.0
    };
    let pivot_validation_speedup_vs_cpu = if pivot_mean > 0 {
        cpu_total_us as f32 / pivot_mean as f32
    } else {
        0.0
    };
    let pivot_validation_speedup_vs_current_gpu = if pivot_mean > 0 {
        envelope_us as f32 / pivot_mean as f32
    } else {
        0.0
    };

    let pivot_timestamp_total_ns = pivot.timestamp_total_ns.unwrap_or(0);
    let pivot_timestamp_per_tick_ns = if ticks > 0 {
        pivot_timestamp_total_ns as f32 / ticks as f32
    } else {
        0.0
    };

    let summary_bytes_total = ticks as usize * std::mem::size_of::<BenchTickSummary>();
    let record_bytes_total = if log_records {
        ticks as usize * n_queues * std::mem::size_of::<BenchRecord>()
    } else {
        0
    };
    let final_state_bytes =
        n_pools * std::mem::size_of::<BenchPool>() + n_queues * std::mem::size_of::<BenchQueue>();

    let (interpretation, performance_gate) = interpretation_string(
        correctness_gate == "PASS",
        conservation_gate == "PASS",
        determinism_gate == "PASS",
        pivot_validation_speedup_vs_cpu,
        pivot_validation_speedup_vs_current_gpu,
        harness.timestamp_supported,
        timestamp_gate,
    );

    Ok(PersistentBenchReport {
        scenario_name: scenario.name.clone(),
        n_pools,
        n_queues,
        ticks,
        log_mode: log_mode.to_string(),
        timestamp_supported: harness.timestamp_supported,
        timestamp_period_ns: harness.timestamp_period_ns,
        correctness_gate: correctness_gate.to_string(),
        conservation_gate: conservation_gate.to_string(),
        determinism_gate: determinism_gate.to_string(),
        timestamp_gate: timestamp_gate.to_string(),
        performance_gate,
        cpu_total_us,
        cpu_per_tick_us: cpu_per_tick,
        current_gpu_envelope_total_us: envelope_us,
        current_gpu_envelope_per_tick_us: envelope_per_tick,
        pivot_total_validation_mean_us: pivot_mean,
        pivot_total_validation_min_us: pivot_min,
        pivot_total_validation_max_us: pivot_max,
        pivot_total_validation_per_tick_us: pivot_per_tick,
        pivot_timestamp_total_ns,
        pivot_timestamp_per_tick_ns,
        current_gpu_speedup_vs_cpu,
        pivot_validation_speedup_vs_cpu,
        pivot_validation_speedup_vs_current_gpu,
        final_state_matches_cpu,
        summaries_match_cpu,
        repeated_pivot_outputs_identical: deterministic,
        summary_bytes_total,
        record_bytes_total,
        final_state_bytes,
        interpretation,
        timing_note: TIMING_NOTE.to_string(),
    })
}

pub fn format_persistent_bench_report(report: &PersistentBenchReport) -> String {
    crate::persistent_bench_report::format_persistent_bench_report(report)
}

pub fn write_persistent_bench_reports(report: &PersistentBenchReport) -> Result<()> {
    crate::persistent_bench_report::write_persistent_bench_reports(report).map_err(Into::into)
}

pub fn write_persistent_bench_reports_bundle(reports: &[PersistentBenchReport]) -> Result<()> {
    crate::persistent_bench_report::write_persistent_bench_reports_bundle(reports).map_err(Into::into)
}
