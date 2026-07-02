//! SQRT-MAG-0 / SQRT-MAG-0 R1 — F-backed Euclidean magnitude FIELD_POLICY hot-path probe (Tier-2, test-only).
//!
//! Raw dx/dy probe: `mag = sqrt_cr_f_bits(bitcast(dx*dx + dy*dy))` — approximate mag authority.
//! Exact mag path: F sqrt over already exact-authoritative `mag2` bits only (R1 contract).
//! No production wiring, no semantic WGSL, no default mapping session wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    exact_sqrt_f_artifact_descriptor, fnv1a64_hex, is_exact_mag_f_from_mag2_descriptor,
    is_mag_f_dxdy_probe_descriptor, landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, ExactPreSqrtInputContract, ExactSqrtArtifactDescriptor,
    ExactSqrtAuthorityClass, KernelDescriptorSpec, MappingExecutionProfile, NativeMathClass,
    OutputAuthority, SpecError, MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID, MAG_F_FROM_DXDY_PROBE_LABEL,
    MAG_F_FROM_MAG2_DESCRIPTOR_ID, MAG_F_FROM_MAG2_LABEL, SQRT_F_ARTIFACT_HASH,
    SQRT_F_ARTIFACT_PATH, SQRT_F_DESCRIPTOR_ID, SQRT_F_ENTRYPOINT, SQRT_F_PROOF_REPORT,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

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
    "ResourceEconomySpec",
    "SimSession",
    "Gadget",
    "Personality",
    "Memory",
];

/// Row stride: dx_bits, dy_bits, mag_bits, mag2_bits, weight_bits
const ROW_STRIDE: u32 = 5;

fn mag_f_from_mag2_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG_F_FROM_MAG2_DESCRIPTOR_ID)
        .expect("mag F from exact mag2 descriptor")
}

fn dxdy_probe_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID)
        .expect("mag F dx/dy probe descriptor")
}

fn sqrt_f_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == SQRT_F_DESCRIPTOR_ID)
        .expect("sqrt F exact descriptor")
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
        label: Some("jit_mag_f_dxdy_probe"),
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
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_mag_f_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
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
            (out[base], out[base + 1], out[base + 2], out[base + 3])
        })
        .collect()
}

fn emit_mag_from_mag2_batch_wgsl(batch_count: u32) -> String {
    format!(
        r#"{f}
@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * 2u;
    let mag2_bits = data[base];
    data[base + 1u] = sqrt_cr_f_bits(mag2_bits);
}}
"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count,
    )
}

fn run_mag_from_mag2_batch(ctx: &GpuContext, mag2_bits: &[u32]) -> Vec<(u32, u32)> {
    use wgpu::util::DeviceExt;
    let n = mag2_bits.len() as u32;
    let wgsl = emit_mag_from_mag2_batch_wgsl(n);
    let mut data = vec![0u32; (n * 2) as usize];
    for (i, bits) in mag2_bits.iter().enumerate() {
        data[i * 2] = *bits;
    }

    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_mag_f_from_mag2"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_mag_f_from_mag2_bgl"),
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
        label: Some("jit_mag_f_from_mag2_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_mag_f_from_mag2_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });
    let bytes = std::mem::size_of_val(data.as_slice()) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_mag_f_from_mag2_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_mag_f_from_mag2_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag_f_from_mag2_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_mag_f_from_mag2_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_mag_f_from_mag2_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag_f_from_mag2_readback_enc"),
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

    mag2_bits
        .iter()
        .enumerate()
        .map(|(i, mag2_in)| (out[i * 2], out[i * 2 + 1]))
        .zip(mag2_bits.iter())
        .map(|((mag2_out, mag_out), _)| (mag2_out, mag_out))
        .collect()
}

fn cpu_sqrt_f_bits(mag2_bits: u32) -> u32 {
    f32::from_bits(mag2_bits).sqrt().to_bits()
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
    let from_mag2 = mag_f_from_mag2_descriptor();
    assert!(is_exact_mag_f_from_mag2_descriptor(&from_mag2));
    validate_kernel_descriptor_admission(&from_mag2).expect("mag from mag2 should admit");
    assert_eq!(
        from_mag2.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::ExactMag2Bits)
    );
    validate_exact_kernel_inputs(&from_mag2, &["mag"]).expect("exact mag from mag2");

    let probe = dxdy_probe_descriptor();
    assert!(is_mag_f_dxdy_probe_descriptor(&probe));
    validate_kernel_descriptor_admission(&probe).expect("dx/dy probe should admit");
    assert_eq!(
        probe.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::RawDxDyProbe)
    );
    let mag_out = probe
        .writes
        .iter()
        .find(|out| out.name == "mag")
        .expect("mag output");
    assert_eq!(mag_out.authority, OutputAuthority::ApproximateDiagnostic);
    assert!(matches!(
        validate_exact_kernel_inputs(&probe, &["mag"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

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

    let mut wrong_hash = from_mag2.clone();
    wrong_hash.exact_sqrt_artifact = Some(ExactSqrtArtifactDescriptor {
        artifact_hash_fnv1a64: "0000000000000000".into(),
        ..exact_sqrt_f_artifact_descriptor()
    });
    assert_admission_err(&wrong_hash, "hash mismatch");

    println!(
        "sqrt_mag0_descriptor: from_mag2={MAG_F_FROM_MAG2_DESCRIPTOR_ID} probe={MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID} hash={SQRT_F_ARTIFACT_HASH}"
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

fn field_policy_gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.001, 0.002, -0.001, -0.002, 0.005, -0.005, 0.01, -0.01, 0.02, -0.02, 0.05, -0.05,
        0.1, -0.1, 0.25, -0.25, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 3.0, 4.0, 5.0, 8.0, 16.0,
    ]
}

fn dense_corpus_pairs() -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for &dx in field_policy_gradient_samples().iter() {
        for &dy in field_policy_gradient_samples().iter() {
            out.push((dx.to_bits(), dy.to_bits()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    out.dedup();
    out
}

fn mobile_simthing_pairs(count: usize) -> Vec<(u32, u32)> {
    let samples = field_policy_gradient_samples();
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
            println!(
                "sqrt_mag0_dense_worst: dx={dx:#x} dy={dy:#x} gpu={gpu:#x} cpu={cpu:#x} ulp={ulp}"
            );
        }
        println!(
            "sqrt_mag0_dense: tested={} mag2_match={mag2_match} exact={exact} max_ulp={max_ulp}",
            rows.len()
        );
        assert!(
            mag2_match > 100,
            "need mag2-matched FIELD_POLICY corpus coverage"
        );
        assert_eq!(max_ulp, 0);
        assert_eq!(exact, mag2_match);
    });
}

#[test]
fn sqrt_mag0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let probe = dxdy_probe_descriptor();
    assert!(probe.default_off);
    assert!(!probe.production_wiring);

    let wgsl = emit_mag_f_batch_wgsl(1);
    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "cache.insert",
    ] {
        assert!(
            !wgsl.contains(forbidden),
            "mag0 WGSL must not reference production `{forbidden}`"
        );
    }
}

// --- SQRT-MAG-0 R1: pre-sqrt exactness contract ---

#[test]
fn sqrt_mag0_r1_accepts_f_sqrt_over_exact_mag2() {
    let from_mag2 = mag_f_from_mag2_descriptor();
    validate_kernel_descriptor_admission(&from_mag2).expect("from_mag2 admits");
    validate_exact_kernel_inputs(&from_mag2, &["mag"]).expect("exact mag from mag2");

    let binding = from_mag2.exact_sqrt_artifact.as_ref().expect("artifact");
    assert_eq!(binding.artifact_hash_fnv1a64, SQRT_F_ARTIFACT_HASH);
    assert_eq!(binding.proof_report, SQRT_F_PROOF_REPORT);

    with_gpu(|ctx| {
        let mag2_inputs = vec![
            0.0f32.to_bits(),
            25.0f32.to_bits(),
            2.0f32.to_bits(),
            0.125f32.to_bits(),
        ];
        let outputs = run_mag_from_mag2_batch(ctx, &mag2_inputs);
        for ((_, mag_out), mag2_in) in outputs.iter().zip(mag2_inputs.iter()) {
            let cpu = cpu_sqrt_f_bits(*mag2_in);
            assert_eq!(*mag_out, cpu, "mag2_in={mag2_in:#x}");
        }
        println!(
            "sqrt_mag0_r1_from_mag2: rows={} max_ulp=0",
            mag2_inputs.len()
        );
    });
}

#[test]
fn sqrt_mag0_r1_raw_dxdy_mag_requires_exact_mag2_contract() {
    let probe = dxdy_probe_descriptor();
    assert_eq!(
        probe.pre_sqrt_contract,
        Some(ExactPreSqrtInputContract::RawDxDyProbe)
    );
    assert!(matches!(
        validate_exact_kernel_inputs(&probe, &["mag"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    with_gpu(|ctx| {
        let rows = vec![(3.0f32.to_bits(), 4.0f32.to_bits())];
        let outputs = run_mag_f_batch(ctx, &rows);
        assert_eq!(outputs.len(), 1);
        println!(
            "sqrt_mag0_r1_raw_dxdy_probe: executes as benchmark probe, mag authority approximate"
        );
    });
}

#[test]
fn sqrt_mag0_r1_reproduces_mag2_mismatch_rows() {
    with_gpu(|ctx| {
        let rows = dense_corpus_pairs();
        let outputs = run_mag_f_batch(ctx, &rows);
        let mut mag2_match = 0usize;
        let mut mag2_mismatch = 0usize;
        let mut worst: Option<(u32, u32, u32, u32)> = None;
        for ((_, _, _, mag2_gpu), (dx, dy)) in outputs.iter().zip(rows.iter()) {
            let cpu_mag2 = cpu_mag2_bits(*dx, *dy);
            if *mag2_gpu == cpu_mag2 {
                mag2_match += 1;
            } else {
                mag2_mismatch += 1;
                worst = Some((*dx, *dy, *mag2_gpu, cpu_mag2));
            }
        }
        if let Some((dx, dy, gpu, cpu)) = worst {
            println!("sqrt_mag0_r1_worst_mag2: dx={dx:#x} dy={dy:#x} gpu_mag2={gpu:#x} cpu_mag2={cpu:#x}");
        }
        println!(
            "sqrt_mag0_r1_mag2_mismatch: total={} match={} mismatch={} cause=GPU/CPU f32 multiply-add bit divergence on dx2+dy2",
            rows.len(),
            mag2_match,
            mag2_mismatch
        );
        assert_eq!(rows.len(), 784);
        assert_eq!(mag2_match, 744);
        assert_eq!(mag2_mismatch, 40);
    });
}

#[test]
fn sqrt_mag0_r1_native_sqrt_and_diagnostic_mag2_still_reject() {
    let sqrt0 = sqrt0_descriptor();
    assert_eq!(sqrt0.native_math, NativeMathClass::ApproximateJitOnly);
    assert!(matches!(
        validate_exact_kernel_inputs(&sqrt0, &["sqrt_out"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    let grad0 = grad0_descriptor();
    assert!(matches!(
        validate_exact_kernel_inputs(&grad0, &["mag2"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));

    let probe = dxdy_probe_descriptor();
    assert!(matches!(
        validate_exact_kernel_inputs(&probe, &["mag"]),
        Err(SpecError::JitKernelDescriptorAdmission { .. })
    ));
}

#[test]
fn sqrt_mag0_r1_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    for desc in [
        mag_f_from_mag2_descriptor(),
        dxdy_probe_descriptor(),
        sqrt_f_descriptor(),
    ] {
        assert!(desc.default_off);
        assert!(!desc.production_wiring);
    }
}
