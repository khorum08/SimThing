//! FIELD_POLICY-EVENT-0 — GPU-resident event compaction from threshold event rows (Tier-2, test-only).
//!
//! Consumes exact OBS-4-style event input rows; compacts nonzero events via atomic counter.
//! Event ordering is UnspecifiedAtomicOrder; membership exact under capacity contract.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    is_field_policy_event0_compaction_descriptor, landed_jit_kernel_descriptors,
    validate_kernel_descriptor_admission, EventCompactionMembershipAuthority,
    EventCompactionOrderAuthority, MappingExecutionProfile, FIELD_POLICY_EVENT0_DESCRIPTOR_ID,
};

use simthing_spec::OutputAuthority;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const INPUT_STRIDE: u32 = 5;
const RECORD_STRIDE: u32 = 5;
const ORDERING_CLASS: &str = "UnspecifiedAtomicOrder";

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction",
    "ownership",
    "owner",
    "AI",
    "threat",
    "scarcity",
    "opportunity",
    "labor",
    "price",
    "logistics",
    "routing",
    "need",
    "demand",
    "supply",
    "personality",
    "drone",
    "FIELD_POLICY",
    "economy",
    "planner",
    "resource",
    "map",
    "urgency",
    "commitment",
    "order",
    "route",
];

const FORBIDDEN_EXACT_TERMS: &[&str] = &["f64", "F64RoundDown", "SHADER_F64", "sqrt_cr_c"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventInputRow {
    observer_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct EventRecord {
    source_index: u32,
    event_code: u32,
    state: u32,
    score_fixed: i32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CompactionParams {
    row_count: u32,
    capacity: u32,
    _pad: [u32; 2],
}

struct CompactionOutcome {
    event_count: u32,
    overflow: u32,
    records: Vec<EventRecord>,
    elapsed: std::time::Duration,
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn emit_compaction_wgsl() -> &'static str {
    r#"
const INPUT_STRIDE: u32 = 5u;
const RECORD_STRIDE: u32 = 5u;

struct Params {
    row_count: u32,
    capacity: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> inputs: array<u32>;
@group(0) @binding(1) var<storage, read_write> counters: array<atomic<u32>, 2>;
@group(0) @binding(2) var<storage, read_write> records: array<u32>;
@group(0) @binding(3) var<uniform> params: Params;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= params.row_count) { return; }
    let base = i * INPUT_STRIDE;
    let code = inputs[base + 1u];
    if (code == 0u) { return; }
    let slot = atomicAdd(&counters[0], 1u);
    if (slot >= params.capacity) {
        atomicStore(&counters[1], 1u);
        return;
    }
    let out_base = slot * RECORD_STRIDE;
    records[out_base] = inputs[base];
    records[out_base + 1u] = code;
    records[out_base + 2u] = inputs[base + 2u];
    records[out_base + 3u] = inputs[base + 3u];
    records[out_base + 4u] = 0u;
}
"#
}

fn pack_inputs(rows: &[EventInputRow]) -> Vec<u32> {
    let mut data = Vec::with_capacity(rows.len() * INPUT_STRIDE as usize);
    for row in rows {
        data.push(row.observer_index);
        data.push(row.event_code);
        data.push(row.state);
        data.push(bytemuck::cast(row.score_fixed));
        data.push(row.flags);
    }
    data
}

fn expected_nonzero(rows: &[EventInputRow]) -> Vec<EventRecord> {
    rows.iter()
        .filter(|row| row.event_code != 0)
        .map(|row| EventRecord {
            source_index: row.observer_index,
            event_code: row.event_code,
            state: row.state,
            score_fixed: row.score_fixed,
        })
        .collect()
}

fn sort_records(records: &mut [EventRecord]) {
    records.sort_by(|a, b| {
        (a.source_index, a.event_code, a.state, a.score_fixed).cmp(&(
            b.source_index,
            b.event_code,
            b.state,
            b.score_fixed,
        ))
    });
}

fn membership_exact(expected: &[EventRecord], got: &[EventRecord]) -> bool {
    if expected.len() != got.len() {
        return false;
    }
    let mut a = expected.to_vec();
    let mut b = got.to_vec();
    sort_records(&mut a);
    sort_records(&mut b);
    a == b
}

fn run_compaction(
    ctx: &GpuContext,
    rows: &[EventInputRow],
    capacity: u32,
    repeat_dispatches: u32,
    do_readback: bool,
) -> CompactionOutcome {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;
    let inputs = pack_inputs(rows);
    let params = CompactionParams {
        row_count: rows.len() as u32,
        capacity,
        _pad: [0, 0],
    };

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("field_policy_event0_compaction"),
        source: wgpu::ShaderSource::Wgsl(emit_compaction_wgsl().into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("field_policy_event0_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("field_policy_event0_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("field_policy_event0_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event0_inputs"),
        contents: bytemuck::cast_slice(&inputs),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let meta_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event0_meta"),
        contents: &[0u8; 8],
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let record_words = (capacity.max(1) * RECORD_STRIDE) as usize;
    let records_init = vec![0u32; record_words];
    let records_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event0_records"),
        contents: bytemuck::cast_slice(&records_init),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_policy_event0_params"),
        contents: bytemuck::bytes_of(&params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("field_policy_event0_bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: meta_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: records_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: params_buf.as_entire_binding(),
            },
        ],
    });

    let t0 = Instant::now();
    for _ in 0..repeat_dispatches {
        queue.write_buffer(&meta_buf, 0, &[0u8; 8]);
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("field_policy_event0_enc"),
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("field_policy_event0_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(rows.len().div_ceil(64) as u32, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
    let elapsed = t0.elapsed();

    if !do_readback {
        return CompactionOutcome {
            event_count: 0,
            overflow: 0,
            records: Vec::new(),
            elapsed,
        };
    }

    let meta_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_event0_meta_readback"),
        size: 8,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let records_bytes = (record_words * 4) as u64;
    let records_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("field_policy_event0_records_readback"),
        size: records_bytes.max(4),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("field_policy_event0_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&meta_buf, 0, &meta_staging, 0, 8);
    if records_bytes > 0 {
        enc2.copy_buffer_to_buffer(&records_buf, 0, &records_staging, 0, records_bytes);
    }
    queue.submit(Some(enc2.finish()));

    let meta_slice = meta_staging.slice(..);
    meta_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let meta_mapped = meta_slice.get_mapped_range();
    let meta: [u32; 2] = bytemuck::cast_slice(&meta_mapped)[..2].try_into().unwrap();
    drop(meta_mapped);
    meta_staging.unmap();

    let mut records = Vec::new();
    if records_bytes > 0 {
        let rec_slice = records_staging.slice(..);
        rec_slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);
        let rec_mapped = rec_slice.get_mapped_range();
        let rec_words: &[u32] = bytemuck::cast_slice(&rec_mapped);
        let written = meta[0].min(capacity) as usize;
        records.reserve(written);
        for slot in 0..written {
            let base = slot * RECORD_STRIDE as usize;
            records.push(EventRecord {
                source_index: rec_words[base],
                event_code: rec_words[base + 1],
                state: rec_words[base + 2],
                score_fixed: bytemuck::cast(rec_words[base + 3]),
            });
        }
        drop(rec_mapped);
        records_staging.unmap();
    }

    CompactionOutcome {
        event_count: meta[0],
        overflow: meta[1],
        records,
        elapsed,
    }
}

fn row(index: u32, code: u32, state: u32, score: i32) -> EventInputRow {
    EventInputRow {
        observer_index: index,
        event_code: code,
        state,
        score_fixed: score,
        flags: 0,
    }
}

fn edge_compaction_cases() -> Vec<(Vec<EventInputRow>, u32, &'static str)> {
    vec![
        (vec![row(0, 0, 0, 0), row(1, 0, 0, 100)], 8, "no_events"),
        (vec![row(3, 1, 1, 500)], 4, "single_event"),
        (
            (0..6)
                .map(|i| row(i, if i % 2 == 0 { 1 } else { 2 }, i % 2, i as i32 * 100))
                .collect(),
            8,
            "all_events",
        ),
        (
            vec![
                row(0, 1, 1, 10),
                row(1, 0, 0, 20),
                row(2, 2, 0, 30),
                row(3, 1, 1, 40),
            ],
            8,
            "mixed_codes",
        ),
        (
            (0..4).map(|i| row(i, 1, 1, i as i32)).collect(),
            4,
            "capacity_exact_full",
        ),
        (
            (0..6).map(|i| row(i, 1, 1, i as i32)).collect(),
            4,
            "capacity_overflow",
        ),
        (vec![row(0, 1, 1, 1)], 0, "zero_capacity"),
    ]
}

fn dense_event_rows() -> Vec<EventInputRow> {
    let mut out = Vec::new();
    for idx in 0..4096u32 {
        let code = match idx % 7 {
            0 => 0,
            1 | 2 => 1,
            3 | 4 => 2,
            _ => 0,
        };
        out.push(row(idx, code, idx % 2, (idx as i32).wrapping_mul(655)));
    }
    out
}

fn density_rows(count: usize, density_pct: u32) -> Vec<EventInputRow> {
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_5645u32;
    for idx in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let emit = (state % 100) < density_pct;
        let code = if emit {
            if state & 1 == 0 {
                1
            } else {
                2
            }
        } else {
            0
        };
        out.push(row(
            idx as u32,
            code,
            idx as u32 % 2,
            (idx as i32).wrapping_mul(17),
        ));
    }
    out
}

fn obs4_style_event_inputs(count: usize) -> Vec<EventInputRow> {
    density_rows(count, 51)
}

#[test]
fn field_policy_event0_wgsl_semantic_free() {
    let wgsl = emit_compaction_wgsl();
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "WGSL must not contain forbidden semantic term `{term}`"
        );
    }
    for term in FORBIDDEN_EXACT_TERMS {
        assert!(!wgsl.contains(term));
    }
    assert!(wgsl.contains("atomicAdd"));
    assert!(!wgsl.contains("planner"));
    assert!(!wgsl.contains("urgency"));
    assert!(!wgsl.contains("commitment"));
    assert!(!wgsl.contains("scheduler"));
    println!("field_policy_event0_wgsl: semantic_free=true ordering={ORDERING_CLASS}");
}

#[test]
fn field_policy_event0_compaction_edge_rows() {
    with_gpu(|ctx| {
        for (rows, capacity, label) in edge_compaction_cases() {
            let expected = expected_nonzero(&rows);
            let outcome = run_compaction(ctx, &rows, capacity, 1, true);
            let written = outcome.records.len() as u32;
            let overflow_expected = if expected.len() as u32 > capacity {
                1
            } else {
                0
            };
            assert_eq!(
                outcome.event_count,
                expected.len() as u32,
                "{label} event_count"
            );
            assert_eq!(outcome.overflow, overflow_expected, "{label} overflow");
            assert_eq!(
                written,
                outcome.event_count.min(capacity),
                "{label} written"
            );
            if overflow_expected == 0 {
                assert!(
                    membership_exact(&expected, &outcome.records),
                    "{label} membership"
                );
            }
            println!(
                "field_policy_event0_edge[{label}]: inputs={} nonzero={} capacity={capacity} event_count={} written={written} overflow={} ordering={ORDERING_CLASS}",
                rows.len(),
                expected.len(),
                outcome.event_count,
                outcome.overflow
            );
        }
    });
}

#[test]
fn field_policy_event0_compaction_dense_corpus() {
    with_gpu(|ctx| {
        let rows = dense_event_rows();
        let expected = expected_nonzero(&rows);
        let capacity = expected.len() as u32 + 16;
        let outcome = run_compaction(ctx, &rows, capacity, 1, true);
        assert_eq!(outcome.event_count, expected.len() as u32);
        assert_eq!(outcome.overflow, 0);
        assert!(membership_exact(&expected, &outcome.records));
        println!(
            "field_policy_event0_dense: rows={} nonzero={} event_count={} membership=exact_unordered ordering={ORDERING_CLASS}",
            rows.len(),
            expected.len(),
            outcome.event_count
        );
    });
}

#[test]
fn field_policy_event0_obs4_to_compaction_smoke() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let rows = obs4_style_event_inputs(N);
        let expected = expected_nonzero(&rows);
        let capacity = expected.len() as u32 + 64;
        let outcome = run_compaction(ctx, &rows, capacity, 1, true);
        assert_eq!(outcome.event_count, expected.len() as u32);
        assert_eq!(outcome.overflow, 0);
        assert!(membership_exact(&expected, &outcome.records));
        println!(
            "field_policy_event0_obs4_smoke: input_rows={N} nonzero={} compact_count={} overflow={} membership=exact_unordered",
            expected.len(),
            outcome.event_count,
            outcome.overflow
        );
    });
}

#[test]
fn field_policy_event0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let desc = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == FIELD_POLICY_EVENT0_DESCRIPTOR_ID)
        .expect("event0 descriptor");
    assert!(desc.default_off);
    assert!(!desc.production_wiring);
    assert!(is_field_policy_event0_compaction_descriptor(&desc));
    validate_kernel_descriptor_admission(&desc).expect("event0 admits");

    let wgsl = emit_compaction_wgsl();
    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "scheduler",
    ] {
        assert!(!wgsl.contains(forbidden));
    }

    let _ = EventCompactionMembershipAuthority::ExactAuthoritativeUnordered;
    let _ = EventCompactionOrderAuthority::UnspecifiedAtomicOrder;
    let event_count = desc
        .writes
        .iter()
        .find(|w| w.name == "event_count")
        .expect("event_count");
    assert_eq!(event_count.authority, OutputAuthority::ExactAuthoritative);
    println!("field_policy_event0_wiring: descriptor=landed no_cpu_planner no_bridge");
}
