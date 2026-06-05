//! Phase M-JIT-0 — Generic EvalEML WGSL JIT prototype (Tier-2 GPU/JIT prototype gate).
//!
//! Test-only, opt-in proof that an admitted, compiled EML gadget node program for a narrow
//! supported subset (`WeightedAccumulator`, `Ema`) can be deterministically lowered into
//! straight-line, semantic-free WGSL, compiled through `wgpu`, executed, and matched against
//! the existing CPU/spec oracles.
//!
//! This module adds NO production JIT wiring, NO default mapping wiring, NO new EML opcode,
//! NO `sqrt`, NO chained scheduling, NO automatic snapshot/copy scheduling, and NO
//! `simthing-sim` Gadget/Personality/Memory semantics. The existing EvalEML interpreter
//! runtime path (see `phase_m_eml_gadget_runtime_execution_gate.rs`) is left unchanged.

use simthing_core::eml_opcode;
use simthing_spec::{
    compile_eml_gadget_stack, deserialize_eml_gadget_stack_ron, eval_eml_postfix, oracle_ema,
    oracle_weighted_accumulator, CompiledEmlGadget, EmlGadgetCompileOptions,
    MappingExecutionProfile,
};
use std::sync::Mutex;

// `eval_eml_cpu` is the GPU crate's interpreter oracle (same opcode semantics) and is reused
// here as a second CPU parity reference alongside the spec-layer `eval_eml_postfix`.
use simthing_gpu::{eval_eml_cpu, GpuContext};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const WEIGHTED_ACC_RON: &str = include_str!("fixtures/jit_weighted_accumulator.ron");
const EMA_RON: &str = include_str!("fixtures/jit_ema.ron");
const FIELD_SAMPLER_RON: &str = include_str!("fixtures/jit_field_sampler_unsupported.ron");

const N_DIMS: u32 = 64;
const EVAL_SLOT: u32 = 0;

// Semantic terms that must never appear in generated WGSL (designer/gameplay meaning stays at
// the RON/spec/admission layer). Matched case-sensitively, mirroring the required `rg` scan.
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

// ── Test-only generic WGSL emitter ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct JitEmitError(String);

/// Lower a compiled EvalEML postfix node program into a deterministic, semantic-free,
/// straight-line WGSL compute shader. Supports only the narrow opcode subset required for
/// `WeightedAccumulator` and `Ema`. Any other opcode is rejected with a structured error;
/// there is no fallback to unsafe interpretation inside the generated shader.
fn emit_evaleml_wgsl(
    nodes: &[simthing_core::EmlNodeGpu],
    output_col: u32,
    n_dims: u32,
) -> Result<String, JitEmitError> {
    let mut decls = String::new();
    let mut stack: Vec<String> = Vec::new();
    let mut tmp_counter: usize = 0;
    let mut emitted_cols: Vec<u32> = Vec::new();
    let mut result_var: Option<String> = None;

    for node in nodes {
        match node.opcode {
            eml_opcode::LITERAL_F32 => {
                // Exact-bit literal: bitcast the stored f32 bit pattern so generated WGSL is
                // bit-identical to the compiled node (no decimal round-trip risk).
                let name = format!("tmp_{tmp_counter}");
                tmp_counter += 1;
                decls.push_str(&format!("    let {name} = bitcast<f32>({}u);\n", node.a));
                stack.push(name);
            }
            eml_opcode::SLOT_VALUE => {
                let col = node.a;
                let name = format!("col_{col}");
                if !emitted_cols.contains(&col) {
                    decls.push_str(&format!("    let {name} = values[base + {col}u];\n"));
                    emitted_cols.push(col);
                }
                stack.push(name);
            }
            op @ (eml_opcode::ADD | eml_opcode::SUB | eml_opcode::MUL) => {
                let symbol = match op {
                    eml_opcode::ADD => "+",
                    eml_opcode::SUB => "-",
                    _ => "*",
                };
                let rhs = stack
                    .pop()
                    .ok_or_else(|| JitEmitError("binary op underflow (rhs)".into()))?;
                let lhs = stack
                    .pop()
                    .ok_or_else(|| JitEmitError("binary op underflow (lhs)".into()))?;
                let name = format!("tmp_{tmp_counter}");
                tmp_counter += 1;
                decls.push_str(&format!("    let {name} = {lhs} {symbol} {rhs};\n"));
                stack.push(name);
            }
            eml_opcode::RETURN_TOP => {
                result_var = Some(
                    stack
                        .last()
                        .cloned()
                        .ok_or_else(|| JitEmitError("RETURN_TOP on empty stack".into()))?,
                );
                break;
            }
            other => {
                return Err(JitEmitError(format!(
                    "unsupported opcode {other} for M-JIT-0 generic WGSL subset \
                     (only LITERAL_F32, SLOT_VALUE, ADD, SUB, MUL, RETURN_TOP)"
                )));
            }
        }
    }

    let result = match result_var {
        Some(r) => r,
        None => stack
            .last()
            .cloned()
            .ok_or_else(|| JitEmitError("program produced no result value".into()))?,
    };

    let mut wgsl = String::new();
    wgsl.push_str("@group(0) @binding(0) var<storage, read_write> values: array<f32>;\n\n");
    wgsl.push_str("@compute @workgroup_size(1)\n");
    wgsl.push_str("fn main(@builtin(global_invocation_id) gid: vec3<u32>) {\n");
    wgsl.push_str("    let slot = gid.x;\n");
    wgsl.push_str(&format!("    let n_dims = {n_dims}u;\n"));
    wgsl.push_str("    let base = slot * n_dims;\n");
    wgsl.push_str(&decls);
    wgsl.push_str(&format!("    let out_col = {output_col}u;\n"));
    wgsl.push_str(&format!("    values[base + out_col] = {result};\n"));
    wgsl.push_str("}\n");
    Ok(wgsl)
}

// ── Test helpers ─────────────────────────────────────────────────────────────────────────────

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let ctx = GpuContext::new_blocking().expect("GPU required");
    f(&ctx);
}

fn set_col(values: &mut [f32], col: u32, v: f32) {
    values[(EVAL_SLOT * N_DIMS + col) as usize] = v;
}

fn compile_single_gadget(ron: &str) -> CompiledEmlGadget {
    let spec = deserialize_eml_gadget_stack_ron(ron).expect("gadget RON parse");
    let stack = compile_eml_gadget_stack(&spec, EmlGadgetCompileOptions::default())
        .expect("gadget stack compile");
    assert_eq!(stack.gadgets.len(), 1, "fixtures define exactly one gadget");
    stack.gadgets.into_iter().next().unwrap()
}

fn assert_semantic_free(wgsl: &str) {
    for term in FORBIDDEN_SEMANTIC_TERMS {
        assert!(
            !wgsl.contains(term),
            "generated WGSL must be semantic-free; found forbidden term `{term}`:\n{wgsl}"
        );
    }
}

/// Compile the generated WGSL through `wgpu`, run it over a single-slot `values` buffer, and
/// read the buffer back.
fn run_jit_gpu(ctx: &GpuContext, wgsl: &str, values_in: &[f32]) -> Vec<f32> {
    use wgpu::util::DeviceExt;
    let device = &ctx.device;
    let queue = &ctx.queue;

    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("jit_evaleml_prototype"),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("jit_evaleml_bgl"),
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
        label: Some("jit_evaleml_pipeline"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("jit_evaleml_pl"),
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
        label: Some("jit_evaleml_values"),
        contents: bytemuck::cast_slice(values_in),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
    });

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("jit_evaleml_bg"),
        layout: &bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_evaleml_enc"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("jit_evaleml_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    queue.submit(Some(encoder.finish()));

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("jit_evaleml_readback"),
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("jit_evaleml_readback_enc"),
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

// ── Test 1: WeightedAccumulator generates semantic-free, deterministic WGSL ───────────────────

#[test]
fn jit_weighted_accumulator_generates_semantic_free_wgsl() {
    let gadget = compile_single_gadget(WEIGHTED_ACC_RON);
    let out_col = gadget.output_col.expect("fixture sets output_col");

    let wgsl = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("WA lowers to WGSL");

    assert_semantic_free(&wgsl);
    // No RON gadget id leaks into the generated shader.
    assert!(
        !wgsl.contains(&gadget.id),
        "generated WGSL must not embed the authored gadget id `{}`",
        gadget.id
    );
    // Deterministic across repeated emission.
    let wgsl_again = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("repeat emit");
    assert_eq!(wgsl, wgsl_again, "generated WGSL must be deterministic");

    // Straight-line lowering shape: the weighted-sum body is generic column math only.
    assert!(wgsl.contains("values[base + 3u]"));
    assert!(wgsl.contains("values[base + 20u]"));
    assert!(wgsl.contains("let out_col = 16u;"));
}

// ── Test 2: WeightedAccumulator GPU output matches all oracles ────────────────────────────────

#[test]
fn jit_weighted_accumulator_gpu_matches_oracles() {
    let gadget = compile_single_gadget(WEIGHTED_ACC_RON);
    let out_col = gadget.output_col.expect("fixture sets output_col");
    let wgsl = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("WA lowers to WGSL");

    let mut values = vec![0.0f32; N_DIMS as usize];
    set_col(&mut values, 3, 12.5);
    set_col(&mut values, 4, -3.25);
    set_col(&mut values, 20, 0.75);
    set_col(&mut values, 21, 2.0);

    let cpu_postfix = eval_eml_postfix(&gadget.nodes, EVAL_SLOT, &values, N_DIMS);
    let cpu_interp = eval_eml_cpu(&gadget.nodes, EVAL_SLOT, &values, N_DIMS, [0.0; 4]);
    let named = oracle_weighted_accumulator(&[12.5, -3.25], &[0.75, 2.0]);

    with_gpu(|ctx| {
        let out = run_jit_gpu(ctx, &wgsl, &values);
        let gpu = out[(EVAL_SLOT * N_DIMS + out_col) as usize];
        assert_eq!(
            gpu.to_bits(),
            cpu_postfix.to_bits(),
            "GPU vs eval_eml_postfix"
        );
        assert_eq!(gpu.to_bits(), cpu_interp.to_bits(), "GPU vs eval_eml_cpu");
        assert_eq!(
            gpu.to_bits(),
            named.to_bits(),
            "GPU vs oracle_weighted_accumulator"
        );
    });
}

// ── Test 3: Ema generates semantic-free, deterministic WGSL ───────────────────────────────────

#[test]
fn jit_ema_generates_semantic_free_wgsl() {
    let gadget = compile_single_gadget(EMA_RON);
    let out_col = gadget.output_col.expect("fixture sets output_col");

    let wgsl = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("Ema lowers to WGSL");

    assert_semantic_free(&wgsl);
    assert!(
        !wgsl.contains(&gadget.id),
        "generated WGSL must not embed the authored gadget id `{}`",
        gadget.id
    );
    let wgsl_again = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("repeat emit");
    assert_eq!(wgsl, wgsl_again, "generated WGSL must be deterministic");

    // Explicit-column temporal memory: previous_col (13) is read explicitly; output is col 13.
    // No hidden previous-value buffer — only the generic `values` storage is bound.
    assert!(wgsl.contains("values[base + 13u]"));
    assert!(wgsl.contains("let out_col = 13u;"));
    assert!(
        wgsl.contains("bitcast<f32>("),
        "decay literals lowered via exact-bit bitcast"
    );
    assert_eq!(
        wgsl.matches("@binding").count(),
        1,
        "only one generic storage binding"
    );
}

// ── Test 4: Ema GPU output matches all oracles ────────────────────────────────────────────────

#[test]
fn jit_ema_gpu_matches_oracles() {
    let gadget = compile_single_gadget(EMA_RON);
    let out_col = gadget.output_col.expect("fixture sets output_col");
    let wgsl = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS).expect("Ema lowers to WGSL");

    let input = 4.0f32;
    let previous = 10.0f32;
    let decay = 0.85f32;

    let mut values = vec![0.0f32; N_DIMS as usize];
    set_col(&mut values, 3, input);
    set_col(&mut values, 13, previous);

    let cpu_postfix = eval_eml_postfix(&gadget.nodes, EVAL_SLOT, &values, N_DIMS);
    let cpu_interp = eval_eml_cpu(&gadget.nodes, EVAL_SLOT, &values, N_DIMS, [0.0; 4]);
    let named = oracle_ema(input, previous, decay);

    with_gpu(|ctx| {
        let out = run_jit_gpu(ctx, &wgsl, &values);
        let gpu = out[(EVAL_SLOT * N_DIMS + out_col) as usize];
        assert_eq!(
            gpu.to_bits(),
            cpu_postfix.to_bits(),
            "GPU vs eval_eml_postfix"
        );
        assert_eq!(gpu.to_bits(), cpu_interp.to_bits(), "GPU vs eval_eml_cpu");
        assert_eq!(gpu.to_bits(), named.to_bits(), "GPU vs oracle_ema");
    });
}

// ── Test 5: Unsupported opcode/shape rejects clearly ──────────────────────────────────────────

#[test]
fn jit_rejects_unsupported_opcode_or_shape() {
    // FieldSampler compiles to DIV + CLAMP_BOUNDED, which are outside the M-JIT-0 subset.
    let gadget = compile_single_gadget(FIELD_SAMPLER_RON);
    let out_col = gadget.output_col.expect("fixture sets output_col");

    let result = emit_evaleml_wgsl(&gadget.nodes, out_col, N_DIMS);
    let err = result.expect_err("unsupported opcode must reject");
    assert!(
        err.0.contains("unsupported opcode"),
        "structured rejection, got: {}",
        err.0
    );

    // Empty program (no result) also rejects, not silently emits a degenerate shader.
    let empty = emit_evaleml_wgsl(&[], 0, N_DIMS);
    assert!(empty.is_err(), "empty program must reject");
}

// ── Test 6: JIT path is test-only and default-off ─────────────────────────────────────────────

#[test]
fn jit_is_test_only_and_default_off() {
    // No default mapping wiring: mapping execution remains disabled by default.
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );

    // The generated shaders carry no production-coupling or simthing-sim semantics.
    let wa = compile_single_gadget(WEIGHTED_ACC_RON);
    let wa_wgsl = emit_evaleml_wgsl(&wa.nodes, wa.output_col.unwrap(), N_DIMS).expect("WA lowers");
    let ema = compile_single_gadget(EMA_RON);
    let ema_wgsl =
        emit_evaleml_wgsl(&ema.nodes, ema.output_col.unwrap(), N_DIMS).expect("Ema lowers");

    for wgsl in [&wa_wgsl, &ema_wgsl] {
        assert_semantic_free(wgsl);
        for token in [
            "simthing_sim",
            "simthing-sim",
            "SimSession",
            "ResourceEconomySpec",
            "mapping",
            "economy",
            "Personality",
            "Memory",
            "Gadget",
            "sqrt",
        ] {
            assert!(
                !wgsl.contains(token),
                "generated WGSL must not reference production/coupling token `{token}`"
            );
        }
    }
}
