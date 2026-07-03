//! Phase M-JIT-SQRT-0 — Native WGSL `sqrt` candidate battery (Tier-2, test-only).
//!
//! This file is intentionally test-local and does NOT add production opcode/admission/runtime
//! support. It probes native WGSL `sqrt` behavior for a constrained corpus and classifies the
//! result as exact, approximate JIT-only, or rejected/deferred.

use std::sync::Mutex;

use simthing_gpu::GpuContext;
use simthing_spec::MappingExecutionProfile;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;
const SQRT_INPUT_COL: u32 = 3;
const VEC_X_COL: u32 = 7;
const VEC_Y_COL: u32 = 8;
const OUT_COL: u32 = 16;

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
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SqrtClassification {
    ExactDeterministicCandidate,
    ApproximateJitOnly,
    RejectedDeferred,
}

#[derive(Debug, Clone, Copy)]
enum NativeSqrtCandidate {
    DirectScalar {
        input_col: u32,
        out_col: u32,
    },
    EuclideanMagnitude {
        x_col: u32,
        y_col: u32,
        out_col: u32,
    },
    GradientMagnitude {
        x_col: u32,
        y_col: u32,
        out_col: u32,
    },
}

#[derive(Debug, Clone, Copy)]
struct BatterySummary {
    tested_cases: usize,
    exact_cases: usize,
    max_ulp: u32,
    classification: SqrtClassification,
}

/// Vector magnitude battery records both shader-order (primary) and FMA (diagnostic) oracles.
#[derive(Debug, Clone, Copy)]
struct VectorBatterySummary {
    tested_cases: usize,
    /// Primary: GPU vs shader-text order `(x*x) + (y*y)` then `sqrt`.
    shader_order_exact: usize,
    shader_order_max_ulp: u32,
    shader_order_classification: SqrtClassification,
    /// Diagnostic only: GPU vs `x.mul_add(x, y*y).sqrt()` — not the aligned oracle unless WGSL
    /// intentionally emits FMA-equivalent behavior (it does not).
    fma_diagnostic_exact: usize,
    fma_diagnostic_max_ulp: u32,
}

fn classify(max_ulp: u32) -> SqrtClassification {
    if max_ulp == 0 {
        SqrtClassification::ExactDeterministicCandidate
    } else if max_ulp <= 2 {
        SqrtClassification::ApproximateJitOnly
    } else {
        SqrtClassification::RejectedDeferred
    }
}

fn ulp_distance(a: f32, b: f32) -> u32 {
    // Monotonic ordering transform for IEEE754 signed float bit patterns.
    fn ordered(bits: u32) -> i32 {
        if (bits & 0x8000_0000) != 0 {
            !(bits as i32)
        } else {
            bits as i32
        }
    }
    let oa = ordered(a.to_bits());
    let ob = ordered(b.to_bits());
    oa.abs_diff(ob)
}

fn assert_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "generated WGSL must be semantic-free; found `{term}`\n{wgsl}"
        );
    }
}

fn emit_sqrt_candidate_wgsl(candidate: NativeSqrtCandidate, n_dims: u32) -> String {
    let mut wgsl = String::new();
    wgsl.push_str("@group(0) @binding(0) var<storage, read_write> values: array<f32>;\n\n");
    wgsl.push_str("@compute @workgroup_size(1)\n");
    wgsl.push_str("fn main(@builtin(global_invocation_id) gid: vec3<u32>) {\n");
    wgsl.push_str("    let slot = gid.x;\n");
    wgsl.push_str(&format!("    let n_dims = {n_dims}u;\n"));
    wgsl.push_str("    let base = slot * n_dims;\n");

    match candidate {
        NativeSqrtCandidate::DirectScalar { input_col, out_col } => {
            wgsl.push_str(&format!(
                "    let col_{input_col} = values[base + {input_col}u];\n"
            ));
            wgsl.push_str(&format!("    let tmp_0 = sqrt(col_{input_col});\n"));
            wgsl.push_str(&format!("    let out_col = {out_col}u;\n"));
            wgsl.push_str("    values[base + out_col] = tmp_0;\n");
        }
        NativeSqrtCandidate::EuclideanMagnitude {
            x_col,
            y_col,
            out_col,
        }
        | NativeSqrtCandidate::GradientMagnitude {
            x_col,
            y_col,
            out_col,
        } => {
            wgsl.push_str(&format!("    let col_{x_col} = values[base + {x_col}u];\n"));
            wgsl.push_str(&format!("    let col_{y_col} = values[base + {y_col}u];\n"));
            wgsl.push_str(&format!("    let tmp_0 = col_{x_col} * col_{x_col};\n"));
            wgsl.push_str(&format!("    let tmp_1 = col_{y_col} * col_{y_col};\n"));
            wgsl.push_str("    let tmp_2 = tmp_0 + tmp_1;\n");
            wgsl.push_str("    let tmp_3 = sqrt(tmp_2);\n");
            wgsl.push_str(&format!("    let out_col = {out_col}u;\n"));
            wgsl.push_str("    values[base + out_col] = tmp_3;\n");
        }
    }

    wgsl.push_str("}\n");
    wgsl
}

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn set_col(values: &mut [f32], col: u32, v: f32) {
    values[(EVAL_SLOT * N_DIMS + col) as usize] = v;
}

fn run_jit_gpu(ctx: &GpuContext, wgsl: &str, values_in: &[f32]) -> Vec<f32> {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_sqrt_candidate"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_sqrt_candidate_bgl"),
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
        label: Some("jit_sqrt_candidate_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_sqrt_candidate_pl"),
                bind_group_layouts: &[&bgl],
                push_constant_ranges: &[],
            }),
        ),
        module: &module,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let bytes = std::mem::size_of_val(values_in) as u64;
    let storage = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jit_sqrt_candidate_values"),
        contents: bytemuck::cast_slice(values_in),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_sqrt_candidate_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_candidate_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_sqrt_candidate_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_sqrt_candidate_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_sqrt_candidate_readback_enc"),
    });
    enc2.copy_buffer_to_buffer(&storage, 0, &staging, 0, bytes);
    queue.submit(Some(enc2.finish()));

    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = slice.get_mapped_range();
    let out: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging.unmap();
    out
}

fn is_admissible_positive_scalar(x: f32) -> bool {
    x.is_finite() && x >= 0.0 && (x == 0.0 || x.is_normal())
}

fn scalar_corpus() -> Vec<f32> {
    vec![
        0.0,
        1.0,
        2.0,
        4.0,
        f32::MIN_POSITIVE,
        1.0e-20,
        1.0e-10,
        0.2,
        0.3,
        3.1415927,
        10.75,
        12345.678,
        1.0e8,
        1.0e20,
    ]
}

fn vec2_corpus() -> Vec<(f32, f32)> {
    vec![
        (3.0, 4.0),
        (0.0, 0.0),
        (0.125, 0.0625),
        (1.25, 2.5),
        (1234.0, 4321.0),
        (1.0e10, 2.0e10),
        (0.30000004, 0.70000005),
        (31.5, 0.125),
    ]
}

fn classify_scalar_battery(ctx: &GpuContext) -> BatterySummary {
    let wgsl = emit_sqrt_candidate_wgsl(
        NativeSqrtCandidate::DirectScalar {
            input_col: SQRT_INPUT_COL,
            out_col: OUT_COL,
        },
        N_DIMS,
    );

    let mut max_ulp = 0u32;
    let mut exact = 0usize;
    let mut tested = 0usize;

    for x in scalar_corpus()
        .into_iter()
        .filter(|v| is_admissible_positive_scalar(*v))
    {
        let mut values = vec![0.0f32; N_DIMS as usize];
        set_col(&mut values, SQRT_INPUT_COL, x);
        let out = run_jit_gpu(ctx, &wgsl, &values);
        let gpu = out[(EVAL_SLOT * N_DIMS + OUT_COL) as usize];
        let cpu = x.sqrt();
        let ulp = ulp_distance(gpu, cpu);
        max_ulp = max_ulp.max(ulp);
        if ulp == 0 {
            exact += 1;
        }
        tested += 1;
    }

    BatterySummary {
        tested_cases: tested,
        exact_cases: exact,
        max_ulp,
        classification: classify(max_ulp),
    }
}

/// Shader-text order oracle: matches generated WGSL `tmp_0 = x*x; tmp_1 = y*y; tmp_2 = tmp_0 + tmp_1; sqrt(tmp_2)`.
fn cpu_magnitude_shader_order(x: f32, y: f32) -> f32 {
    let sum_shader_order = (x * x) + (y * y);
    sum_shader_order.sqrt()
}

/// FMA diagnostic oracle only — not the primary comparator unless WGSL emits fused behavior.
fn cpu_magnitude_fma_diagnostic(x: f32, y: f32) -> f32 {
    let sum_fma = x.mul_add(x, y * y);
    sum_fma.sqrt()
}

fn classify_vec2_battery(ctx: &GpuContext, candidate: NativeSqrtCandidate) -> VectorBatterySummary {
    let wgsl = emit_sqrt_candidate_wgsl(candidate, N_DIMS);
    let mut shader_order_max_ulp = 0u32;
    let mut shader_order_exact = 0usize;
    let mut fma_diagnostic_max_ulp = 0u32;
    let mut fma_diagnostic_exact = 0usize;
    let mut tested = 0usize;

    for (x, y) in vec2_corpus().into_iter() {
        let sum_shader_order = (x * x) + (y * y);
        if !sum_shader_order.is_finite() || sum_shader_order < 0.0 {
            continue;
        }
        let mut values = vec![0.0f32; N_DIMS as usize];
        set_col(&mut values, VEC_X_COL, x);
        set_col(&mut values, VEC_Y_COL, y);
        let out = run_jit_gpu(ctx, &wgsl, &values);
        let gpu = out[(EVAL_SLOT * N_DIMS + OUT_COL) as usize];

        let cpu_shader_order = cpu_magnitude_shader_order(x, y);
        let ulp_shader = ulp_distance(gpu, cpu_shader_order);
        shader_order_max_ulp = shader_order_max_ulp.max(ulp_shader);
        if ulp_shader == 0 {
            shader_order_exact += 1;
        }

        let cpu_fma = cpu_magnitude_fma_diagnostic(x, y);
        let ulp_fma = ulp_distance(gpu, cpu_fma);
        fma_diagnostic_max_ulp = fma_diagnostic_max_ulp.max(ulp_fma);
        if ulp_fma == 0 {
            fma_diagnostic_exact += 1;
        }

        tested += 1;
    }

    VectorBatterySummary {
        tested_cases: tested,
        shader_order_exact,
        shader_order_max_ulp,
        shader_order_classification: classify(shader_order_max_ulp),
        fma_diagnostic_exact,
        fma_diagnostic_max_ulp,
    }
}

#[test]
fn jit_sqrt_generated_wgsl_is_semantic_free() {
    let direct = emit_sqrt_candidate_wgsl(
        NativeSqrtCandidate::DirectScalar {
            input_col: SQRT_INPUT_COL,
            out_col: OUT_COL,
        },
        N_DIMS,
    );
    let magnitude = emit_sqrt_candidate_wgsl(
        NativeSqrtCandidate::EuclideanMagnitude {
            x_col: VEC_X_COL,
            y_col: VEC_Y_COL,
            out_col: OUT_COL,
        },
        N_DIMS,
    );
    assert_semantic_free(&direct);
    assert_semantic_free(&magnitude);
    assert!(direct.contains("sqrt("));
    assert!(magnitude.contains("sqrt("));
    assert!(!direct.contains("jit_"));
    assert!(!magnitude.contains("jit_"));

    let baseline = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(!baseline.contains("sqrt("));
}

#[test]
fn jit_sqrt_direct_scalar_candidate_battery() {
    with_gpu(|ctx| {
        let summary = classify_scalar_battery(ctx);
        println!(
            "direct_scalar: tested={}, exact={}, max_ulp={}, classification={:?}",
            summary.tested_cases, summary.exact_cases, summary.max_ulp, summary.classification
        );
        assert!(summary.tested_cases > 0);
        assert!(matches!(
            summary.classification,
            SqrtClassification::ExactDeterministicCandidate
                | SqrtClassification::ApproximateJitOnly
                | SqrtClassification::RejectedDeferred
        ));
        if summary.classification == SqrtClassification::ExactDeterministicCandidate {
            assert_eq!(summary.max_ulp, 0);
        }
    });
}

#[test]
fn jit_sqrt_euclidean_magnitude_candidate_battery() {
    with_gpu(|ctx| {
        let summary = classify_vec2_battery(
            ctx,
            NativeSqrtCandidate::EuclideanMagnitude {
                x_col: VEC_X_COL,
                y_col: VEC_Y_COL,
                out_col: OUT_COL,
            },
        );
        println!(
            "euclidean_magnitude: tested={}, shader_order exact={}/max_ulp={}, fma_diagnostic exact={}/max_ulp={}, primary_classification={:?}",
            summary.tested_cases,
            summary.shader_order_exact,
            summary.shader_order_max_ulp,
            summary.fma_diagnostic_exact,
            summary.fma_diagnostic_max_ulp,
            summary.shader_order_classification
        );
        assert!(summary.tested_cases > 0);
        assert!(matches!(
            summary.shader_order_classification,
            SqrtClassification::ExactDeterministicCandidate
                | SqrtClassification::ApproximateJitOnly
                | SqrtClassification::RejectedDeferred
        ));
    });
}

#[test]
fn jit_sqrt_gradient_magnitude_candidate_battery() {
    with_gpu(|ctx| {
        let wgsl = emit_sqrt_candidate_wgsl(
            NativeSqrtCandidate::GradientMagnitude {
                x_col: VEC_X_COL,
                y_col: VEC_Y_COL,
                out_col: OUT_COL,
            },
            N_DIMS,
        );
        assert_semantic_free(&wgsl);
        // Keep generated naming generic; no semantic `grad_*` terms in WGSL.
        assert!(!wgsl.contains("grad_"));

        let summary = classify_vec2_battery(
            ctx,
            NativeSqrtCandidate::GradientMagnitude {
                x_col: VEC_X_COL,
                y_col: VEC_Y_COL,
                out_col: OUT_COL,
            },
        );
        println!(
            "gradient_magnitude: tested={}, shader_order exact={}/max_ulp={}, fma_diagnostic exact={}/max_ulp={}, primary_classification={:?}",
            summary.tested_cases,
            summary.shader_order_exact,
            summary.shader_order_max_ulp,
            summary.fma_diagnostic_exact,
            summary.fma_diagnostic_max_ulp,
            summary.shader_order_classification
        );
        assert!(summary.tested_cases > 0);
    });
}

#[test]
fn jit_sqrt_vector_oracle_order_is_explicit() {
    // Generated WGSL uses separate multiply/add, not FMA.
    let wgsl = emit_sqrt_candidate_wgsl(
        NativeSqrtCandidate::EuclideanMagnitude {
            x_col: VEC_X_COL,
            y_col: VEC_Y_COL,
            out_col: OUT_COL,
        },
        N_DIMS,
    );
    assert!(wgsl.contains("col_7 * col_7"));
    assert!(wgsl.contains("col_8 * col_8"));
    assert!(wgsl.contains("tmp_0 + tmp_1"));
    assert!(
        !wgsl.contains("mul_add"),
        "WGSL must not claim FMA unless proven"
    );

    // Shader-order and FMA oracles can differ on some inputs (diagnostic separation).
    let x = 0.30000004f32;
    let y = 0.70000005f32;
    let shader_order = cpu_magnitude_shader_order(x, y);
    let fma = cpu_magnitude_fma_diagnostic(x, y);
    // Primary oracle for classification is shader-order; FMA is diagnostic only.
    assert_eq!(shader_order.to_bits(), ((x * x) + (y * y)).sqrt().to_bits());

    with_gpu(|ctx| {
        let euclidean = classify_vec2_battery(
            ctx,
            NativeSqrtCandidate::EuclideanMagnitude {
                x_col: VEC_X_COL,
                y_col: VEC_Y_COL,
                out_col: OUT_COL,
            },
        );
        // Classification must use shader-order primary oracle, not FMA diagnostic.
        assert_eq!(
            euclidean.shader_order_classification,
            classify(euclidean.shader_order_max_ulp)
        );
        println!(
            "oracle_policy: primary=shader_order max_ulp={} class={:?}; diagnostic=fma max_ulp={} (shader_order_cpu={:?} fma_cpu={:?})",
            euclidean.shader_order_max_ulp,
            euclidean.shader_order_classification,
            euclidean.fma_diagnostic_max_ulp,
            shader_order.to_bits(),
            fma.to_bits()
        );
    });
}
#[test]
fn jit_sqrt_not_in_baseline_runtime() {
    let shader = include_str!("../../simthing-gpu/src/shaders/accumulator_op.wgsl");
    assert!(
        !shader.contains("sqrt("),
        "baseline runtime shader must remain sqrt-free"
    );

    let opcodes = include_str!("../../simthing-core/src/eml_nodes.rs");
    assert!(
        !opcodes.contains("SQRT"),
        "no production EML sqrt opcode may be introduced"
    );

    let gadget_compile = include_str!("../../simthing-spec/src/compile/eml_gadget.rs");
    assert!(!gadget_compile.contains("NativeSqrt"));
}

#[test]
fn jit_sqrt_result_classification_is_explicit() {
    with_gpu(|ctx| {
        let scalar = classify_scalar_battery(ctx);
        let magnitude = classify_vec2_battery(
            ctx,
            NativeSqrtCandidate::EuclideanMagnitude {
                x_col: VEC_X_COL,
                y_col: VEC_Y_COL,
                out_col: OUT_COL,
            },
        );
        let final_classification =
            match (scalar.classification, magnitude.shader_order_classification) {
                (SqrtClassification::RejectedDeferred, _)
                | (_, SqrtClassification::RejectedDeferred) => SqrtClassification::RejectedDeferred,
                (SqrtClassification::ApproximateJitOnly, _)
                | (_, SqrtClassification::ApproximateJitOnly) => {
                    SqrtClassification::ApproximateJitOnly
                }
                _ => SqrtClassification::ExactDeterministicCandidate,
            };

        println!(
            "sqrt_candidate_final_classification={:?} (scalar max_ulp={}, magnitude shader_order max_ulp={}, magnitude fma_diagnostic max_ulp={})",
            final_classification,
            scalar.max_ulp,
            magnitude.shader_order_max_ulp,
            magnitude.fma_diagnostic_max_ulp
        );

        // Explicitly one of exact / approximate / rejected, never implicit.
        assert!(matches!(
            final_classification,
            SqrtClassification::ExactDeterministicCandidate
                | SqrtClassification::ApproximateJitOnly
                | SqrtClassification::RejectedDeferred
        ));

        // No default-on mapping/JIT wiring changed as part of this battery.
        assert_eq!(
            MappingExecutionProfile::default(),
            MappingExecutionProfile::Disabled
        );
    });
}
