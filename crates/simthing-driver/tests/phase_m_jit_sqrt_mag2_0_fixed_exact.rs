//! SQRT-MAG2-0 — Exact fixed-point pre-sqrt mag2 construction for gradient magnitude hot path.
//!
//! Q16.16 signed fixed-point `dx_fixed`/`dy_fixed` → integer `dx²+dy²` → pinned f32 mag2 bits
//! → artifact-backed Candidate F sqrt → exact Euclidean magnitude.
//! No production wiring, no semantic WGSL, no default mapping session wiring.

use std::sync::Mutex;
use std::time::Instant;

use simthing_gpu::GpuContext;
use simthing_spec::{
    is_exact_mag2_fixed_descriptor, landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, KernelDescriptorSpec, Mag2SourceContract,
    MappingExecutionProfile, OutputAuthority, SpecError, MAG2_FIXED_DESCRIPTOR_ID,
    MAG2_Q16_COMPONENT_MAX, MAG2_Q16_FRAC_BITS, MAG2_Q16_SCALE, MAG2_Q16_SCALE_SQ,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());
const SQRT_CR_F_WGSL: &str = include_str!("wgsl/sqrt_cr_f_candidate.wgsl");

/// Row stride: dx_fixed, dy_fixed, mag2_sum_lo, mag2_sum_hi, mag2_bits, mag_bits
const ROW_STRIDE: u32 = 6;

fn mag2_fixed_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == MAG2_FIXED_DESCRIPTOR_ID)
        .expect("mag2 fixed exact descriptor")
}

fn grad0_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_grad_0_observer")
        .expect("grad0 descriptor")
}

fn dxdy_probe_descriptor() -> KernelDescriptorSpec {
    landed_jit_kernel_descriptors()
        .into_iter()
        .find(|desc| desc.id == "m_jit_mag_f_from_dxdy_probe")
        .expect("dx/dy probe descriptor")
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn f32_to_q16(v: f32) -> i32 {
    (v * MAG2_Q16_SCALE as f32).round() as i32
}

fn cpu_mag2_sum(dx_fixed: i32, dy_fixed: i32) -> u64 {
    let dx = i64::from(dx_fixed);
    let dy = i64::from(dy_fixed);
    (dx * dx + dy * dy) as u64
}

/// Pinned Q16.16 mag2 integer sum → f32 bits (`scale_sq = 2^32`).
fn mag2_u64_q16_to_f32_bits(sum: u64) -> u32 {
    let lo = sum as u32;
    let hi = (sum >> 32) as u32;
    (hi as f32 + lo as f32 / 4294967296.0).to_bits()
}

fn cpu_mag2_bits_from_fixed(dx_fixed: i32, dy_fixed: i32) -> u32 {
    mag2_u64_q16_to_f32_bits(cpu_mag2_sum(dx_fixed, dy_fixed))
}

fn cpu_mag_bits_from_fixed(dx_fixed: i32, dy_fixed: i32) -> u32 {
    f32::from_bits(cpu_mag2_bits_from_fixed(dx_fixed, dy_fixed))
        .sqrt()
        .to_bits()
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
        return 1;
    }
    (diff / f32::EPSILON / scale).ceil() as u32
}

fn emit_fixed_mag2_batch_wgsl(batch_count: u32, with_sqrt: bool) -> String {
    let sqrt_body = if with_sqrt {
        "    data[base + 5u] = sqrt_cr_f_bits(mag2_bits);\n"
    } else {
        "    data[base + 5u] = 0u;\n"
    };
    format!(
        r#"{f}
fn abs_fixed(v: i32) -> u32 {{
    if (v < 0) {{ return u32(0u - bitcast<u32>(v)); }}
    return u32(v);
}}

fn mul_u32_wide(a: u32, b: u32) -> vec2<u32> {{
    let mask = 65535u;
    let a0 = a & mask;
    let a1 = a >> 16u;
    let b0 = b & mask;
    let b1 = b >> 16u;
    let z0 = a0 * b0;
    let z1 = a0 * b1;
    let z2 = a1 * b0;
    let z3 = a1 * b1;
    let t = (z0 >> 16u) + (z1 & mask) + (z2 & mask);
    let lo = (z0 & mask) | ((t & mask) << 16u);
    let hi = z3 + (z1 >> 16u) + (z2 >> 16u) + (t >> 16u);
    return vec2<u32>(lo, hi);
}}

fn add_u64_wide(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {{
    let lo = a.x + b.x;
    let carry = select(0u, 1u, lo < a.x);
    let hi = a.y + b.y + carry;
    return vec2<u32>(lo, hi);
}}

fn mag2_sum_q16(dx_fixed: i32, dy_fixed: i32) -> vec2<u32> {{
    let dx2 = mul_u32_wide(abs_fixed(dx_fixed), abs_fixed(dx_fixed));
    let dy2 = mul_u32_wide(abs_fixed(dy_fixed), abs_fixed(dy_fixed));
    return add_u64_wide(dx2, dy2);
}}

fn mag2_u64_q16_to_f32_bits(sum: vec2<u32>) -> u32 {{
    return bitcast<u32>(f32(sum.y) + f32(sum.x) / 4294967296.0);
}}

@group(0) @binding(0) var<storage, read_write> data: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
    let i = gid.x;
    if (i >= {batch_count}u) {{ return; }}
    let base = i * {stride}u;
    let dx_fixed = bitcast<i32>(data[base]);
    let dy_fixed = bitcast<i32>(data[base + 1u]);
    let sum = mag2_sum_q16(dx_fixed, dy_fixed);
    data[base + 2u] = sum.x;
    data[base + 3u] = sum.y;
    let mag2_bits = mag2_u64_q16_to_f32_bits(sum);
    data[base + 4u] = mag2_bits;
{sqrt_body}}}
"#,
        f = SQRT_CR_F_WGSL,
        batch_count = batch_count,
        stride = ROW_STRIDE,
        sqrt_body = sqrt_body,
    )
}

fn run_fixed_mag2_batch(
    ctx: &GpuContext,
    pairs: &[(i32, i32)],
    with_sqrt: bool,
) -> Vec<(u64, u32, u32, u32, u32)> {
    use wgpu::util::DeviceExt;
    let n = pairs.len() as u32;
    let wgsl = emit_fixed_mag2_batch_wgsl(n, with_sqrt);
    let mut data = vec![0u32; (n * ROW_STRIDE) as usize];
    for (i, (dx, dy)) in pairs.iter().enumerate() {
        let base = i * ROW_STRIDE as usize;
        data[base] = *dx as u32;
        data[base + 1] = *dy as u32;
    }

    let device = &ctx.device;
    let queue = &ctx.queue;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_mag2_fixed"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_mag2_fixed_bgl"),
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
        label: Some("jit_mag2_fixed_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_mag2_fixed_pl"),
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
        label: Some("jit_mag2_fixed_values"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_mag2_fixed_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag2_fixed_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_mag2_fixed_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_mag2_fixed_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_mag2_fixed_readback_enc"),
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
            let sum_lo = out[base + 2];
            let sum_hi = out[base + 3];
            let sum = u64::from(sum_lo) | (u64::from(sum_hi) << 32);
            (sum, out[base + 4], out[base + 5], sum_lo, sum_hi)
        })
        .collect()
}

fn gradient_samples() -> Vec<f32> {
    vec![
        0.0, 0.001, 0.002, -0.001, -0.002, 0.005, -0.005, 0.01, -0.01, 0.02, -0.02, 0.05, -0.05,
        0.1, -0.1, 0.25, -0.25, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 3.0, 4.0, 5.0, 8.0, 16.0,
    ]
}

fn dense_fixed_pairs() -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for &dx in gradient_samples().iter() {
        for &dy in gradient_samples().iter() {
            out.push((f32_to_q16(dx), f32_to_q16(dy)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    out.dedup();
    out
}

fn mobile_simthing_fixed_pairs(count: usize) -> Vec<(i32, i32)> {
    let samples = gradient_samples();
    let mut out = Vec::with_capacity(count);
    let mut state = 0x5345_4144u32;
    for _ in 0..count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dx = samples[(state as usize) % samples.len()];
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let dy = samples[(state as usize) % samples.len()];
        out.push((f32_to_q16(dx), f32_to_q16(dy)));
    }
    out
}

fn edge_fixed_pairs() -> Vec<(i32, i32)> {
    let max_q = f32_to_q16(MAG2_Q16_COMPONENT_MAX);
    let min_q = f32_to_q16(-MAG2_Q16_COMPONENT_MAX);
    let tiny = f32_to_q16(0.001);
    vec![
        (0, 0),
        (f32_to_q16(3.0), 0),
        (0, f32_to_q16(4.0)),
        (f32_to_q16(3.0), f32_to_q16(4.0)),
        (max_q, 0),
        (0, max_q),
        (min_q, min_q),
        (tiny, tiny),
        (-tiny, tiny),
        (max_q, max_q),
    ]
}

#[test]
fn sqrt_mag2_0_descriptor_admits_fixed_exact_mag2() {
    let mag2 = mag2_fixed_descriptor();
    assert!(is_exact_mag2_fixed_descriptor(&mag2));
    validate_kernel_descriptor_admission(&mag2).expect("mag2 fixed admits");
    assert_eq!(
        mag2.mag2_source_contract,
        Some(Mag2SourceContract::ExactFixedPointDxDy {
            fraction_bits: MAG2_Q16_FRAC_BITS,
        })
    );
    let out = mag2
        .writes
        .iter()
        .find(|w| w.name == "mag2_bits")
        .expect("mag2_bits");
    assert_eq!(out.authority, OutputAuthority::ExactAuthoritative);
    validate_exact_kernel_inputs(&mag2, &["mag2_bits"]).expect("mag2_bits exact");

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

    println!(
        "sqrt_mag2_0_descriptor: id={MAG2_FIXED_DESCRIPTOR_ID} frac_bits={MAG2_Q16_FRAC_BITS}"
    );
}

#[test]
fn sqrt_mag2_0_fixed_mag2_edge_rows() {
    with_gpu(|ctx| {
        let rows = edge_fixed_pairs();
        let outputs = run_fixed_mag2_batch(ctx, &rows, false);
        let mut exact = 0usize;
        for ((sum_gpu, mag2_bits_gpu, _, _, _), (dx, dy)) in outputs.iter().zip(rows.iter()) {
            let sum_cpu = cpu_mag2_sum(*dx, *dy);
            assert_eq!(*sum_gpu, sum_cpu, "dx={dx} dy={dy}");
            assert_eq!(
                *mag2_bits_gpu,
                cpu_mag2_bits_from_fixed(*dx, *dy),
                "mag2_bits dx={dx} dy={dy}"
            );
            exact += 1;
        }
        println!(
            "sqrt_mag2_0_edge: tested={} exact={exact} overflow=0 max_int_error=0",
            rows.len()
        );
        assert_eq!(exact, rows.len());
    });
}

#[test]
fn sqrt_mag2_0_fixed_mag2_dense_corpus() {
    with_gpu(|ctx| {
        let rows = dense_fixed_pairs();
        let outputs = run_fixed_mag2_batch(ctx, &rows, false);
        let mut exact = 0usize;
        let mut worst: Option<(i32, i32, u64, u64)> = None;
        for ((sum_gpu, _, _, _, _), (dx, dy)) in outputs.iter().zip(rows.iter()) {
            let sum_cpu = cpu_mag2_sum(*dx, *dy);
            if *sum_gpu == sum_cpu {
                exact += 1;
            } else {
                worst = Some((*dx, *dy, *sum_gpu, sum_cpu));
            }
        }
        if let Some((dx, dy, gpu, cpu)) = worst {
            println!("sqrt_mag2_0_dense_worst: dx={dx} dy={dy} gpu_sum={gpu} cpu_sum={cpu}");
        }
        println!(
            "sqrt_mag2_0_dense: tested={} exact={exact} max_int_error=0 overflow=0",
            rows.len()
        );
        assert_eq!(exact, rows.len());
    });
}

#[test]
fn sqrt_mag2_0_fixed_mag2_feeds_f_sqrt_edge_rows() {
    with_gpu(|ctx| {
        let rows = edge_fixed_pairs();
        let outputs = run_fixed_mag2_batch(ctx, &rows, true);
        let mut max_ulp = 0u32;
        let mut exact = 0usize;
        for ((_, mag2_bits, mag_bits, _, _), (dx, dy)) in outputs.iter().zip(rows.iter()) {
            let cpu_mag2 = cpu_mag2_bits_from_fixed(*dx, *dy);
            assert_eq!(*mag2_bits, cpu_mag2);
            let cpu_mag = cpu_mag_bits_from_fixed(*dx, *dy);
            let ulp = ulp_distance(*mag_bits, cpu_mag);
            if ulp == 0 {
                exact += 1;
            }
            max_ulp = max_ulp.max(ulp);
        }
        println!(
            "sqrt_mag2_0_f_edge: tested={} exact={exact} max_ulp={max_ulp}",
            rows.len()
        );
        assert_eq!(max_ulp, 0);
        assert_eq!(exact, rows.len());
    });
}

#[test]
fn sqrt_mag2_0_fixed_mag2_feeds_f_sqrt_dense_corpus() {
    with_gpu(|ctx| {
        let rows = dense_fixed_pairs();
        let outputs = run_fixed_mag2_batch(ctx, &rows, true);
        let mut max_ulp = 0u32;
        let mut exact = 0usize;
        let mut worst: Option<(i32, i32, u32, u32, u32)> = None;
        for ((_, mag2_bits, mag_bits, _, _), (dx, dy)) in outputs.iter().zip(rows.iter()) {
            let cpu_mag2 = cpu_mag2_bits_from_fixed(*dx, *dy);
            assert_eq!(*mag2_bits, cpu_mag2);
            let cpu_mag = cpu_mag_bits_from_fixed(*dx, *dy);
            let ulp = ulp_distance(*mag_bits, cpu_mag);
            if ulp == 0 {
                exact += 1;
            }
            if ulp > max_ulp {
                max_ulp = ulp;
                worst = Some((*dx, *dy, *mag_bits, cpu_mag, ulp));
            }
        }
        if let Some((dx, dy, gpu, cpu, ulp)) = worst {
            println!(
                "sqrt_mag2_0_f_dense_worst: dx={dx} dy={dy} gpu={gpu:#x} cpu={cpu:#x} ulp={ulp}"
            );
        }
        println!(
            "sqrt_mag2_0_f_dense: tested={} exact={exact} max_ulp={max_ulp}",
            rows.len()
        );
        assert_eq!(max_ulp, 0);
        assert_eq!(exact, rows.len());
    });
}

#[test]
fn sqrt_mag2_0_no_default_runtime_wiring() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    let mag2 = mag2_fixed_descriptor();
    assert!(mag2.default_off);
    assert!(!mag2.production_wiring);

    let wgsl = emit_fixed_mag2_batch_wgsl(1, true);
    for forbidden in [
        "SimSession",
        "ResourceEconomySpec",
        "simthing-sim",
        "KernelCache",
        "cache.insert",
        "faction",
        "ownership",
    ] {
        assert!(
            !wgsl.contains(forbidden),
            "mag2 WGSL must not reference `{forbidden}`"
        );
    }
}
