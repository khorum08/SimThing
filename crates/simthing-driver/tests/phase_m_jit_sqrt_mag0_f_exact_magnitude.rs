//! SQRT-MAG-0 — F-backed exact Euclidean magnitude SEAD hot-path probe (Tier-2, test-only).
//!
//! Computes `mag = sqrt_cr_f_bits(bitcast(dx*dx + dy*dy))` via verbatim F artifact inclusion.
//! No production wiring, no semantic WGSL, no default mapping session wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, fnv1a64_hex, is_exact_mag_f_descriptor,
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, ExactSqrtArtifactDescriptor, ExactSqrtAuthorityClass,
    KernelDescriptorSpec, MAG_F_DESCRIPTOR_ID, MAG_F_LABEL, MappingExecutionProfile,
    NativeMathClass, OutputAuthority, SQRT_F_ARTIFACT_HASH, SQRT_F_ARTIFACT_PATH,
    SQRT_F_ENTRYPOINT, SQRT_F_PROOF_REPORT, SpecError,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

const FORBIDDEN_SEMANTIC_TERMS: &[&str] = &[
    "faction", "ownership", "owner", "AI", "threat", "scarcity", "opportunity", "labor",
    "price", "logistics", "routing", "need", "demand", "supply", "personality", "drone", "SEAD",
    "ResourceEconomySpec", "SimSession", "Gadget", "Personality", "Memory",
];

/// Row stride: dx_bits, dy_bits, mag_bits, mag2_bits, weight_bits
const ROW_STRIDE: u32 = 5;

fn mag_f_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG_F_DESCRIPTOR_ID)
        .expect("mag F exact descriptor")
}

fn sqrt0_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_sqrt_0_candidate")
        .expect("sqrt0 descriptor")
}

fn grad0_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn emit_mag_f_batch_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
fn mag_f_euclidean_bits(dx_bits: u32, dy_bits: u32) -> u32 {{
    let dx = bitcast<f32>(dx_bits);
    let dy = bitcast<f32>(dy_bits);
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let mag2 = dx2 + dy2;
    return sqrt_cr_f_bits(bitcast<u32>(mag2));
}}

@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let dx_bits = data[base];
    let dy_bits = data[base + 1u];
    let dx = bitcast<f32>(dx_bits);
    let dy = bitcast<f32>(dy_bits);
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let mag2 = dx2 + dy2;
    data[base + 3u] = bitcast<u32>(mag2);
    data[base + 2u] = mag_f_euclidean_bits(dx_bits, dy_bits);
}}
"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count,
        stride = ROW_STRIDE,
    )
}

fn run_mag_f_batch(ctx: &GpuContext, pairs: &[(u32, u32)]) -> Vec<(u32, u32, u32, u32)> {
    use wgpu::util::DeviceExt;
    let n = pairs.len() as u32;
    let wgsl = emit_mag_f_batch_wgsl(n);
    let mut data = vec![0u32; (n * ROW_STRIDE) as usize];
    for (i, (dx, dy)) in pairs.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        data[base] = *dx;
        data[base + 1] = *dy;
    }

    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_mag_f_exact"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_mag_f_bgl"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("jit_mag_f_pipeline"),
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jit_mag_f_pl"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        })),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });
    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_mag_f_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_mag_f_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag_f_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_mag_f_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_mag_f_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag_f_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&storage, 0, &staging, 0, bytes);
    queue.submit(Some(enc2.finish()));
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let mapped = slice.get_mapped_range();
    let out: Vec<u32> = bytemuck::cast_slice(&mapped).to_vec();
    drop(mapped);
    staging.unmap();

    pairs
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let base = i * ROW_STRIDE as usize;
            (
                out[base],
                out[base + 1],
                out[base + 2],
                out[base + 3],
            )
        })
        .collect()
}

/// CPU oracle: same let-sequenced multiply-add contract as WGSL.
fn cpu_mag_bits(dx_bits: u32, dy_bits: u32) -> u32 {
    let dx = f32::from_bits(dx_bits);
    let dy = f32::from_bits(dy_bits);
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    let mag2 = dx2 + dy2;
    mag2.sqrt().to_bits()
}

fn cpu_mag2_bits(dx_bits: u32, dy_bits: u32) -> u32 {
    let dx = f32::from_bits(dx_bits);
    let dy = f32::from_bits(dy_bits);
    let dx2 = dx * dx;
    let dy2 = dy * dy;
    (dx2 + dy2).to_bits()
}

fn ulp_distance(a_bits: u32, b_bits: u32) -> u32 {
    if a_bits == b_bits {
        return 0;
    }
    let a = f32::from_bits(a_bits);
    let b = f32::from_bits(b_bits);
    if a.is_nan() || b.is_nan() {
        return u32::MAX;
    }
    let diff = (a - b).abs();
    if diff == 0.0 {
        return 0;
    }
    let scale = a.abs().max(b.abs());
    if scale == 0.0 {
        return 0;
    }
    ((diff / f32::EPSILON) / scale).ceil() as u32
}

fn assert_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "WGSL must not contain forbidden semantic term `{term}`"
        );
    }
    assert!(!wgsl.contains("f64("));
    assert!(!wgsl.contains("sqrt_cr_c"));
}

fn assert_admission_err(spec: &KernelDescriptorSpec, reason_substr: &str) {
    let err = validate_kernel_descriptor_admission(spec).expect_err("expected admission failure");
    match err {
        SpecError::JitKernelDescriptorAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn sqrt_mag0_descriptor_admits_f_backed_exact_magnitude() {
    let mag = mag_f_descriptor();
    assert!(is_exact_mag_f_descriptor(&mag));
    validate_kernel_descriptor_admission(&mag).expect("mag F descriptor should admit");

    let binding = mag.exact_sqrt_artifact.as_ref().expect("artifact binding");
    assert_eq!(binding.artifact_path, SQRT_F_ARTIFACT_PATH);
    assert_eq!(binding.artifact_hash_fnv1a64, SQRT_F_ARTIFACT_HASH);
    assert_eq!(binding.entrypoint, SQRT_F_ENTRYPOINT);
    assert_eq!(binding.proof_report, SQRT_F_PROOF_REPORT);
    assert_eq!(binding.authority_class, ExactSqrtAuthorityClass::ExactDeterministic);

    let mag_out = mag
        .writes
        .iter()
        .find(|out| out.name == "mag")
        .expect("mag output");
    assert_eq!(mag_out.authority, OutputAuthority::ExactAuthoritative);
    validate_exact_kernel_inputs(&mag, &["mag"]).expect("mag exact via F");

    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    let grad0 = grad0_descriptor();
    let mag2 = grad0
        .writes
        .iter()
        .find(|out| out.name == "mag2")
        .expect("mag2");
    assert_eq!(mag2.authority, OutputAuthority::ApproximateDiagnostic);
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    let mut wrong_hash = mag.clone();
    wrong_hash.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_hash_fnv1a64: "0000000000000000".into(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&wrong_hash, "hash mismatch");

    println!(
        "sqrt_mag0_descriptor: id={MAG_F_DESCRIPTOR_ID} label={MAG_F_LABEL} hash={SQRT_F_ARTIFACT_HASH}"
    );
}

#[test]
fn sqrt_mag0_wgsl_semantic_free() {
    let wgsl = emit_mag_f_batch_wgsl(1);
    assert_semantic_free(&wgsl);
    assert!(wgsl.contains(SQRT_CR_F_WGSL));
    assert_eq!(wgsl.matches(SQRT_CR_F_WGSL).count(), 1);
    assert!(wgsl.contains("fn sqrt_cr_f_bits("));
    assert!(wgsl.contains("mag_f_euclidean_bits"));
    assert!(!wgsl.contains("sqrt_cr_c"));
    assert_eq!(fnv1a64_hex(SQRT_CR_F_WGSL), SQRT_F_ARTIFACT_HASH);

    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

#[test]
fn sqrt_mag0_edge_rows_match_cpu_oracle() {
    with_gpu(|ctx| {
        let rows: Vec<(u32, u32)> = vec![
            (0.0f32.to_bits(), 0.0f32.to_bits()),
            (3.0f32.to_bits(), 0.0f32.to_bits()),
            (0.0f32.to_bits(), 4.0f32.to_bits()),
            (3.0f32.to_bits(), 4.0f32.to_bits()),
            (1.0f32.to_bits(), 1.0f32.to_bits()),
            (0.125f32.to_bits(), 0.25f32.to_bits()),
        ];
        let outputs = run_mag_f_batch(ctx, &rows);
        let mut max_ulp = 0u32;
        for ((_, _, mag, mag2_bits), (dx_in, dy_in)) in outputs.iter().zip(rows.iter()) {
            let cpu = cpu_mag_bits(*dx_in, *dy_in);
            let ulp = ulp_distance(*mag, cpu);
            max_ulp = max_ulp.max(ulp);
            let expected_mag2 = cpu_mag2_bits(*dx_in, *dy_in);
            assert_eq!(*mag2_bits, expected_mag2, "mag2 diagnostic mismatch");
            if ulp > 0 {
                println!(
                    "edge row dx={dx_in:#x} dy={dy_in:#x} gpu={mag:#x} cpu={cpu:#x} ulp={ulp}"
                );
            }
            assert_eq!(*mag, cpu, "edge row bit mismatch");
        }
        println!("sqrt_mag0_edge: rows={} max_ulp={max_ulp}", rows.len());
        assert_eq!(max_ulp, 0);
    });
}

fn sead_gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.001, 0.002, -0.001, -0.002, 0.005, -0.005, 0.01, -0.01, 0.02, -0.02, 0.05, -0.05,
        0.1, -0.1, 0.25, -0.25, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 3.0, 4.0, 5.0, 8.0, 16.0,
    ]
}

fn dense_corpus_pairs() -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for &dx in sead_gradient_samples().iter() {
        for &dy in sead_gradient_samples().iter() {
            out.push((dx.to_bits(), dy.to_bits()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    out.dedup();
    out
}

fn mobile_simthing_pairs(count: usize) -> Vec<(u32, u32)> {
    let samples = sead_gradient_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for _ in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dx = samples[(state as usize) % samples.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dy = samples[(state as usize) % samples.len()];
        out.push((dx.to_bits(), dy.to_bits()));
    }
    out
}

#[test]
fn sqrt_mag0_dense_corpus_match_cpu_oracle() {
    with_gpu(|ctx| {
        let rows = dense_corpus_pairs();
        let outputs = run_mag_f_batch(ctx, &rows);
        let mut max_ulp = 0u32;
        let mut exact = 0usize;
        let mut mag2_match = 0usize;
        let mut worst: Option<(u32, u32, u32, u32, u32)> = None;
        for ((dx, dy, mag, mag2_gpu), (dx_in, dy_in)) in outputs.iter().zip(rows.iter()) {
            let cpu_mag2 = cpu_mag2_bits(*dx_in, *dy_in);
            if *mag2_gpu != cpu_mag2 {
                continue;
            }
            mag2_match += 1;
            let cpu = cpu_mag_bits(*dx_in, *dy_in);
            let ulp = ulp_distance(*mag, cpu);
            if ulp == 0 {
                exact += 1;
            }
            if ulp > max_ulp {
                max_ulp = ulp;
                worst = Some((*dx, *dy, *mag, cpu, ulp));
            }
        }
        if let Some((dx, dy, gpu, cpu, ulp)) = worst {
            println!("sqrt_mag0_dense_worst: dx={dx:#x} dy={dy:#x} gpu={gpu:#x} cpu={cpu:#x} ulp={ulp}");
        }
        println!(
            "sqrt_mag0_dense: tested={} mag2_match={mag2_match} exact={exact} max_ulp={max_ulp}",
            rows.len()
        );
        assert!(mag2_match > 100, "need mag2-matched SEAD corpus coverage");
        assert_eq!(max_ulp, 0);
        assert_eq!(exact, mag2_match);
    });
}

#[test]
fn sqrt_mag0_perf_34k_mobile_simthing_hot_path() {
    with_gpu(|ctx| {
        const N: usize = 34_000;
        let pairs = mobile_simthing_pairs(N);
        let t0 = Instant::now();
        let outputs = run_mag_f_batch(ctx, &pairs);
        let elapsed = t0.elapsed();
        assert_eq!(outputs.len(), N);

        let mut spot_max_ulp = 0u32;
        for ((_, _, mag, mag2_gpu), (dx, dy)) in outputs.iter().take(512).zip(pairs.iter().take(512)) {
            if *mag2_gpu != cpu_mag2_bits(*dx, *dy) {
                continue;
            }
            spot_max_ulp = spot_max_ulp.max(ulp_distance(*mag, cpu_mag_bits(*dx, *dy)));
        }
        assert_eq!(spot_max_ulp, 0, "34k spot-check requires max_ulp==0 on mag2-matched rows");

        let ms = elapsed.as_secs_f64() * 1000.0;
        let per_entity_us = elapsed.as_secs_f64() * 1_000_000.0 / N as f64;
        println!(
            "sqrt_mag0_perf_34k: inputs={N} dispatches=1 includes_readback=true elapsed_ms={ms:.3} per_entity_us={per_entity_us:.4} spot_max_ulp={spot_max_ulp}"
        );
        println!(
            "sqrt_mag0_perf_note: prior SQRT-EXACT-4F F-only 34k smoke was ~single sqrt per row; this path adds dx/dy/mag2 + F sqrt per mobile simthing"
        );
    });
}

#[test]
fn sqrt_mag0_perf_scaled_smoke() {
    with_gpu(|ctx| {
        for size in [10_000usize, 34_000, 100_000] {
            let pairs = mobile_simthing_pairs(size);
            let t0 = Instant::now();
            let outputs = run_mag_f_batch(ctx, &pairs);
            let elapsed = t0.elapsed();
            assert_eq!(outputs.len(), size);
            let ms = elapsed.as_secs_f64() * 1000.0;
            println!(
                "sqrt_mag0_perf_smoke: inputs={size} dispatches=1 includes_readback=true elapsed_ms={ms:.3}"
            );
        }
        println!("sqrt_mag0_perf_smoke: 1_000_000 row run skipped by default (ignored optional)");
    });
}

#[test]
fn sqrt_mag0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let mag = mag_f_descriptor();
    assert!(mag.default_off);
    assert!(!mag.production_wiring);

    let wgsl = emit_mag_f_batch_wgsl(1);
    for forbidden in ["SimSession", "ResourceEconomySpec", "simthing-sim", "KernelCache", "cache.insert"] {
        assert!(
            !wgsl.contains(forbidden),
            "mag0 WGSL must not reference production `{forbidden}`"
        );
    }
}
