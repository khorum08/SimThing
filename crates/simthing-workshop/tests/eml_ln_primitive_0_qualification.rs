//! EML-LN-PRIMITIVE-0 — edge-first battery, mutation referees, and the LOCAL
//! exhaustive admitted-domain qualification for the standalone LNDS candidate
//! (double-single, table-driven; DA 5186693435).
//!
//! Candidate-F discipline: the candidate executes as its OWN frozen WGSL
//! artifact (`crates/simthing-driver/tests/wgsl/eml_ln_ds_candidate.wgsl`)
//! against the CPU twin `simthing_core::eml_ln::eml_ln_pinned_bits`, with
//! edges BEFORE the sweep and promotion strictly after exhaustive green.
//! Domain: positive normals `0x00800000..=0x7F7FFFFF` (2,130,706,432).
//!
//! Exhaustive tests are `#[ignore]` local phase-boundary acts:
//! `cargo test -p simthing-workshop --test eml_ln_primitive_0_qualification -- --ignored --nocapture`

use simthing_core::eml_ln::{
    eml_ln_pinned_bits, EML_LN_ALGORITHM_IDENTITY, EML_LN_DOMAIN_MAX_BITS, EML_LN_DOMAIN_MIN_BITS,
    EML_LN_DOMAIN_SIZE, EML_LN_TABLE,
};
use simthing_gpu::GpuContext;
use wgpu::util::DeviceExt;

const CANDIDATE_WGSL: &str =
    include_str!("../../simthing-driver/tests/wgsl/eml_ln_ds_candidate.wgsl");
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const CHUNK: usize = 1 << 21;

fn fnv_fold(mut hash: u64, bits: u32) -> u64 {
    for byte in bits.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Live-tuple gate (5.11 comparator; the certified tuple roster is shared
/// hardware — the LN qualification module pins LN digests at promotion).
fn certified_context() -> Option<GpuContext> {
    let ctx = GpuContext::new_blocking().ok()?;
    let live =
        simthing_kernel::eml_exp_qualification::EmlExpLiveToolchainIdentity::from_context(&ctx);
    simthing_kernel::eml_exp_qualification::require_certified_toolchain(&live)
        .expect("live GPU tuple must be in the certified toolchain roster");
    eprintln!(
        "EML_LN_TOOLCHAIN live tuple CERTIFIED adapter={:?} backend={:?} driver={:?}",
        live.adapter, live.backend, live.driver
    );
    Some(ctx)
}

struct CandidateHarness {
    pipeline: wgpu::ComputePipeline,
    input: wgpu::Buffer,
    output: wgpu::Buffer,
    staging: wgpu::Buffer,
    capacity: usize,
}

impl CandidateHarness {
    fn new(ctx: &GpuContext, capacity: usize) -> Self {
        let module = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("eml-ln-ds-candidate"),
                source: wgpu::ShaderSource::Wgsl(CANDIDATE_WGSL.into()),
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: "main",
                compilation_options: Default::default(),
                cache: None,
            });
        let bytes = (capacity * 4) as u64;
        let mk = |usage| {
            ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: bytes,
                usage,
                mapped_at_creation: false,
            })
        };
        Self {
            pipeline,
            input: mk(wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST),
            output: mk(wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC),
            staging: mk(wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ),
            capacity,
        }
    }

    fn run(&self, ctx: &GpuContext, inputs: &[u32]) -> Vec<u32> {
        assert!(inputs.len() <= self.capacity);
        ctx.queue
            .write_buffer(&self.input, 0, bytemuck::cast_slice(inputs));
        let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.input.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.output.as_entire_binding(),
                },
            ],
        });
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind, &[]);
            pass.dispatch_workgroups(((self.capacity + 63) / 64) as u32, 1, 1);
        }
        let bytes = (inputs.len() * 4) as u64;
        encoder.copy_buffer_to_buffer(&self.output, 0, &self.staging, 0, bytes);
        ctx.queue.submit([encoder.finish()]);
        let slice = self.staging.slice(..bytes);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        ctx.device.poll(wgpu::Maintain::Wait);
        let out = {
            let data = slice.get_mapped_range();
            bytemuck::cast_slice::<u8, u32>(&data)[..inputs.len()].to_vec()
        };
        self.staging.unmap();
        out
    }
}

/// The mandatory edge corpus, ordered per the dispatch's standing order.
fn edge_corpus() -> Vec<u32> {
    let mut rows = vec![
        0x3F80_0000, // 1.0 -> must be exactly +0.0
        EML_LN_DOMAIN_MIN_BITS,
        EML_LN_DOMAIN_MAX_BITS,
        0x3F80_0001, // 1 + ulp
        0x3F7F_FFFF, // 1 - ulp
        0x4000_0000, // 2.0
        0x3F00_0000, // 0.5
    ];
    // Every binade boundary (all 254 positive-normal exponents).
    for e in 1..255u32 {
        rows.push(e << 23);
        rows.push((e << 23) | 0x7F_FFFF); // binade top
    }
    // Every table-cell seam at k in {-1, 0, 1}: the folded-binade cell edges.
    for j in 0..128u32 {
        let seam = 0x3F33_0000 + (j << 16);
        for delta in [0u32, 0xFFFF] {
            let b = seam + delta;
            for k in [-1i32, 0, 1] {
                let shifted = (b as i64 + ((k as i64) << 23)) as u32;
                if (EML_LN_DOMAIN_MIN_BITS..=EML_LN_DOMAIN_MAX_BITS).contains(&shifted) {
                    rows.push(shifted);
                }
            }
        }
    }
    // Identity-cell neighborhood: 256 ULPs around 1.0.
    for d in 1..=256u32 {
        rows.push(0x3F80_0000 + d);
        rows.push(0x3F80_0000 - d);
    }
    rows
}

// ── Fast battery (edges FIRST; any divergence is a STOP) ─────────────────────

#[test]
fn eml_ln_primitive_0_edge_battery_is_bit_exact_on_the_standalone_candidate() {
    // CPU-side hard pins first (cheap, run even without a GPU).
    assert_eq!(
        eml_ln_pinned_bits(0x3F80_0000),
        0,
        "ln(1.0) must be exactly +0.0"
    );
    assert_eq!(
        eml_ln_pinned_bits(0x0080_0000),
        0xC2AE_AC50,
        "min_normal pinned bits"
    );
    assert_eq!(
        eml_ln_pinned_bits(0x7F7F_FFFF),
        0x42B1_7218,
        "f32::MAX pinned bits"
    );
    let Some(ctx) = certified_context() else {
        return;
    };
    let corpus = edge_corpus();
    let harness = CandidateHarness::new(&ctx, corpus.len());
    let gpu = harness.run(&ctx, &corpus);
    for (index, bits) in corpus.iter().enumerate() {
        let want = eml_ln_pinned_bits(*bits);
        assert_eq!(
            gpu[index], want,
            "EDGE STOP: candidate diverges at input {bits:#010X} (gpu {:#010X} cpu {want:#010X})",
            gpu[index]
        );
    }
    assert_eq!(gpu[0], 0, "ln(1.0) on the candidate must be exactly +0.0");
    eprintln!(
        "EML_LN_EDGE battery green: {} rows bit-exact (incl. min_normal, 1.0->+0.0, MAX, 254 binades, 128 table seams x3 binades, near-1 512)",
        corpus.len()
    );
}

/// Mutation referee 1 — planted table drift must change outputs (the digest
/// referee bites on the table, not only the arithmetic).
#[test]
fn eml_ln_primitive_0_planted_table_drift_mutant_reds_the_digest() {
    fn ln_with_table_drift(x_bits: u32) -> u32 {
        // The full pinned v4 sequence with TBL[j].inv_c perturbed by one ULP.
        let t = x_bits.wrapping_sub(0x3F33_0000);
        let k = (t as i32) >> 23;
        let m_bits = x_bits.wrapping_sub((k as u32) << 23);
        let j = ((t >> 16) & 0x7F) as usize;
        let m = f32::from_bits(m_bits);
        let inv = f32::from_bits(EML_LN_TABLE[j][0] + 1); // PLANTED DEFECT
        let lnc_hi = f32::from_bits(EML_LN_TABLE[j][1]);
        let lnc_mid = f32::from_bits(EML_LN_TABLE[j][2]);
        let p_hi = m * inv;
        let p_err = m.mul_add(inv, -p_hi);
        let s = p_hi - 1.0;
        let s_lo = p_err;
        let poly = s.mul_add(0.2, -0.25);
        let poly = s.mul_add(poly, f32::from_bits(0x3EAA_AAAB));
        let z = s * s;
        let sp = s * poly;
        let r1 = z.mul_add(sp, -0.5 * z);
        let slo_term = (-s_lo).mul_add(s, s_lo);
        let kf = k as f32;
        let t_hi = kf.mul_add(simthing_core::eml_ln::EML_LN_LN2_HI, lnc_hi);
        let mid = kf.mul_add(simthing_core::eml_ln::EML_LN_LN2_MID, lnc_mid);
        let low = mid + (slo_term + r1);
        let g1 = low + s;
        (t_hi + g1).to_bits()
    }
    // Honest digest form: fold both functions over a j=0 probe stratum.
    let mut reference = FNV_OFFSET;
    let mut mutant = FNV_OFFSET;
    for i in 0..4096u32 {
        let bits = 0x3F33_0000 + (i << 2); // j=0 cell rows
        reference = fnv_fold(reference, eml_ln_pinned_bits(bits));
        mutant = fnv_fold(mutant, ln_with_table_drift(bits));
    }
    assert_ne!(reference, mutant, "planted table drift must RED the digest");
    assert_ne!(EML_LN_ALGORITHM_IDENTITY, 0);
}

/// Mutation referee 2 — the two-sum error lanes are load-bearing: collapsing
/// them (the compiler-fast-math shape) must RED the digest.
#[test]
fn eml_ln_primitive_0_planted_two_sum_collapse_mutant_reds_the_digest() {
    fn ln_collapsed(x_bits: u32) -> u32 {
        // PLANTED DEFECT: the single-f32 reconstruction family (every lo lane
        // and the grid split dropped) — the shape that failed pre-5.12.
        let t = x_bits.wrapping_sub(0x3F33_0000);
        let k = (t as i32) >> 23;
        let m_bits = x_bits.wrapping_sub((k as u32) << 23);
        let j = ((t >> 16) & 0x7F) as usize;
        let m = f32::from_bits(m_bits);
        let inv = f32::from_bits(EML_LN_TABLE[j][0]);
        let lnc_hi = f32::from_bits(EML_LN_TABLE[j][1]);
        let s = m.mul_add(inv, -1.0);
        let poly = s.mul_add(0.2, -0.25);
        let poly = s.mul_add(poly, f32::from_bits(0x3EAA_AAAB));
        let z = s * s;
        let l = s - 0.5 * z + (z * s) * poly;
        let kf = k as f32;
        ((kf * 0.693_147_18_f32 + lnc_hi) + l).to_bits()
    }
    let mut reference = FNV_OFFSET;
    let mut mutant = FNV_OFFSET;
    let mut state = 0x243F_6A88_85A3_08D3u64;
    for _ in 0..65_536 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let span = u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS);
        let bits = EML_LN_DOMAIN_MIN_BITS + ((state >> 16) % (span + 1)) as u32;
        reference = fnv_fold(reference, eml_ln_pinned_bits(bits));
        mutant = fnv_fold(mutant, ln_collapsed(bits));
    }
    assert_ne!(
        reference, mutant,
        "planted two-sum/lo-lane collapse must RED the digest"
    );
}

/// Mutation referee 3 — the final rounding is hi+lo, not hi alone: truncating
/// the lo lane at the final add must RED.
#[test]
fn eml_ln_primitive_0_planted_final_rounding_truncation_mutant_reds_the_digest() {
    // Deterministic stratum; compare pinned vs pinned-with-lo-dropped by
    // reconstructing the final step: any input whose v_lo influences RN.
    let mut reds = 0u64;
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..262_144 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let span = u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS);
        let bits = EML_LN_DOMAIN_MIN_BITS + ((state >> 16) % (span + 1)) as u32;
        let full = eml_ln_pinned_bits(bits);
        let truncated = eml_ln_hi_only(bits);
        if full != truncated {
            reds += 1;
        }
    }
    assert!(
        reds > 0,
        "the final lo lane must be observable (truncation mutant never RED = lo lane dead)"
    );
    eprintln!("EML_LN_FINAL_ROUNDING lo-lane observable on {reds}/262144 probe rows");
}

fn eml_ln_hi_only(x_bits: u32) -> u32 {
    // Pinned v4 sequence with the FINAL lo add dropped (t_hi alone).
    let t = x_bits.wrapping_sub(0x3F33_0000);
    let k = (t as i32) >> 23;
    let m_bits = x_bits.wrapping_sub((k as u32) << 23);
    let j = ((t >> 16) & 0x7F) as usize;
    let m = f32::from_bits(m_bits);
    let inv = f32::from_bits(EML_LN_TABLE[j][0]);
    let lnc_hi = f32::from_bits(EML_LN_TABLE[j][1]);
    let lnc_mid = f32::from_bits(EML_LN_TABLE[j][2]);
    let p_hi = m * inv;
    let p_err = m.mul_add(inv, -p_hi);
    let s = p_hi - 1.0;
    let s_lo = p_err;
    let poly = s.mul_add(0.2, -0.25);
    let poly = s.mul_add(poly, f32::from_bits(0x3EAA_AAAB));
    let z = s * s;
    let sp = s * poly;
    let r1 = z.mul_add(sp, -0.5 * z);
    let slo_term = (-s_lo).mul_add(s, s_lo);
    let kf = k as f32;
    let t_hi = kf.mul_add(simthing_core::eml_ln::EML_LN_LN2_HI, lnc_hi);
    let mid = kf.mul_add(simthing_core::eml_ln::EML_LN_LN2_MID, lnc_mid);
    let low = mid + (slo_term + r1);
    let _g1 = low + s;
    t_hi.to_bits() // PLANTED DEFECT: final lo add dropped
}

// ── Local phase-boundary acts (never standing CI) ────────────────────────────

/// Characterization vs f64 ln: envelope over boundaries + strata (no
/// correct-rounding claim; the pinned sequence is the bit law).
#[test]
#[ignore = "local phase-boundary characterization: envelope vs f64 ln reference"]
fn eml_ln_primitive_0_numerical_characterization() {
    fn ulp_distance(a: f32, b: f32) -> i64 {
        fn key(x: f32) -> i64 {
            let b = i64::from(x.to_bits());
            if b & 0x8000_0000 != 0 {
                0x8000_0000_i64 - b
            } else {
                b + 0x8000_0000_i64
            }
        }
        (key(a) - key(b)).abs()
    }
    let mut max_ulp = 0i64;
    let mut max_at = 0u32;
    let mut over1 = 0u64;
    let mut nonmono = 0u64;
    let mut checked = 0u64;
    let mut prev = f32::NEG_INFINITY;
    // stratified: every 64th pattern across the whole domain (33.3M rows), in
    // ascending order so monotonicity is also swept.
    let mut bits = EML_LN_DOMAIN_MIN_BITS;
    loop {
        let got = f32::from_bits(eml_ln_pinned_bits(bits));
        let want = (f64::from(f32::from_bits(bits))).ln() as f32;
        let d = ulp_distance(got, want);
        if d > max_ulp {
            max_ulp = d;
            max_at = bits;
        }
        if d > 1 {
            over1 += 1;
        }
        if got < prev {
            nonmono += 1;
        }
        prev = got;
        checked += 1;
        if bits > EML_LN_DOMAIN_MAX_BITS - 64 {
            break;
        }
        bits += 64;
    }
    eprintln!(
        "EML_LN_CHARACTERIZE checked={checked} max_ulp={max_ulp} at_bits={max_at:#010X} over1={over1} nonmono={nonmono}"
    );
    assert_eq!(over1, 0, "envelope must stay <=1 ULP vs the f64 reference");
    assert_eq!(nonmono, 0, "sampled ascending sweep must stay monotone");
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive admitted-domain CPU reference"]
fn eml_ln_primitive_0_exhaustive_cpu_reference() {
    let mut digest = FNV_OFFSET;
    let mut tested = 0u64;
    let mut bits = EML_LN_DOMAIN_MIN_BITS;
    loop {
        digest = fnv_fold(digest, eml_ln_pinned_bits(bits));
        tested += 1;
        if bits == EML_LN_DOMAIN_MAX_BITS {
            break;
        }
        bits += 1;
    }
    assert_eq!(tested, EML_LN_DOMAIN_SIZE);
    eprintln!(
        "EML_LN_QUALIFY arm=cpu-reference tested={tested} digest={digest:#018x} algorithm={EML_LN_ALGORITHM_IDENTITY:#018x}"
    );
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive standalone-candidate GPU replay"]
fn eml_ln_primitive_0_exhaustive_standalone_gpu_replay() {
    let ctx = certified_context().expect("exhaustive qualification requires the certified GPU");
    let harness = CandidateHarness::new(&ctx, CHUNK);
    let mut digest = FNV_OFFSET;
    let mut tested = 0u64;
    let mut inputs = vec![0u32; CHUNK];
    let mut bits = EML_LN_DOMAIN_MIN_BITS as u64;
    while bits <= EML_LN_DOMAIN_MAX_BITS as u64 {
        let n = usize::min(CHUNK, (EML_LN_DOMAIN_MAX_BITS as u64 - bits + 1) as usize);
        for (i, slot) in inputs[..n].iter_mut().enumerate() {
            *slot = (bits as u32) + i as u32;
        }
        for slot in inputs[n..].iter_mut() {
            *slot = 0x3F80_0000;
        }
        let out = harness.run(&ctx, &inputs);
        for (i, got) in out[..n].iter().enumerate() {
            let want = eml_ln_pinned_bits(inputs[i]);
            assert_eq!(
                *got, want,
                "candidate first divergence at input {:#010X}",
                inputs[i]
            );
            digest = fnv_fold(digest, *got);
        }
        tested += n as u64;
        bits += n as u64;
        if (bits & 0x0FFF_FFFF) < CHUNK as u64 {
            eprintln!(
                "EML_LN_QUALIFY arm=standalone-gpu progress bits={bits:#010x} tested={tested}"
            );
        }
    }
    assert_eq!(tested, EML_LN_DOMAIN_SIZE);
    eprintln!("EML_LN_QUALIFY arm=standalone-gpu tested={tested} digest={digest:#018x}");
}

// ── Three-arm EML-path exhaustive (post-promotion; the production arms) ──────

use simthing_core::{eml_opcode, ColumnIndex, EmlNodeGpu};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, FieldAdjacency, FieldLawProof, FieldSweepOutput,
    FieldSweepRegistration, FieldSweepRegistrationRequest, FieldSweepSession,
};

fn node(opcode: u32, a: u32, b: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b,
        c: 0,
        d: 0,
    }
}

/// Elementwise guarded-LN post program (clamp is identity over the domain).
fn ln_elementwise_registration(slots: u32) -> FieldSweepRegistration {
    let col = ColumnIndex::try_from_admitted_authored(0, 1).expect("bounded column");
    let adjacency = FieldAdjacency::independent_slots(slots, col).expect("independent slots");
    let order = adjacency.apply_canonical_order_proof();
    apply_field_sweep_registration(FieldSweepRegistrationRequest {
        adjacency,
        n_dims: 1,
        output: FieldSweepOutput::Matrix(col),
        map_program: vec![
            node(eml_opcode::LITERAL_F32, 0.0f32.to_bits(), 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        fold_program: vec![
            node(eml_opcode::PARAM, field_param::ACCUMULATOR, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        identity_bits: 0.0f32.to_bits(),
        post_program: vec![
            node(eml_opcode::TARGET_VALUE, 0, 0),
            node(
                eml_opcode::CLAMP_BOUNDED,
                EML_LN_DOMAIN_MIN_BITS,
                EML_LN_DOMAIN_MAX_BITS,
            ),
            node(eml_opcode::LN, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ],
        field_law_proof: Some(FieldLawProof::apply_non_conservative()),
        transient_read_proof: None,
        canonical_order_proof: Some(order),
        dt: 1.0,
    })
    .expect("LN elementwise admission (guarded shape-2 call site)")
}

fn run_eml_arm(interpreted: bool, arm: &str) -> (u64, u64) {
    let ctx = certified_context().expect("exhaustive qualification requires the certified GPU");
    let registration = ln_elementwise_registration(CHUNK as u32);
    let class = registration.resource_class();
    let mut session = if interpreted {
        FieldSweepSession::new_interpreted_for_profiling(&ctx, &registration, class)
            .expect("interpreted session")
    } else {
        FieldSweepSession::new_with_profiling_resource_class(&ctx, &registration, class)
            .expect("generated JIT session")
    };
    let mut digest = FNV_OFFSET;
    let mut tested = 0u64;
    let mut inputs = vec![0.0f32; CHUNK];
    let mut bits = EML_LN_DOMAIN_MIN_BITS as u64;
    while bits <= EML_LN_DOMAIN_MAX_BITS as u64 {
        let n = usize::min(CHUNK, (EML_LN_DOMAIN_MAX_BITS as u64 - bits + 1) as usize);
        for (i, slot) in inputs[..n].iter_mut().enumerate() {
            *slot = f32::from_bits((bits as u32) + i as u32);
        }
        for slot in inputs[n..].iter_mut() {
            *slot = 1.0;
        }
        session.upload_values(&ctx, &inputs).expect("chunk upload");
        session
            .dispatch_chain(&ctx, std::slice::from_ref(&registration), 1)
            .expect("chunk dispatch");
        let out = session.readback(&ctx).expect("chunk readback");
        for (i, got) in out[..n].iter().enumerate() {
            let want = eml_ln_pinned_bits(inputs[i].to_bits());
            assert_eq!(
                got.to_bits(),
                want,
                "{arm}: first divergence at input {:#010X}",
                inputs[i].to_bits()
            );
            digest = fnv_fold(digest, got.to_bits());
        }
        tested += n as u64;
        bits += n as u64;
    }
    (digest, tested)
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive interpreted-arm replay"]
fn eml_ln_primitive_0_exhaustive_interpreted_replay() {
    let (digest, tested) = run_eml_arm(true, "interpreted");
    assert_eq!(tested, EML_LN_DOMAIN_SIZE);
    assert_eq!(
        digest,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_REFERENCE_DIGEST
    );
    eprintln!("EML_LN_QUALIFY arm=interpreted tested={tested} digest={digest:#018x}");
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive SSA-JIT-arm replay"]
fn eml_ln_primitive_0_exhaustive_jit_replay() {
    let (digest, tested) = run_eml_arm(false, "jit");
    assert_eq!(tested, EML_LN_DOMAIN_SIZE);
    assert_eq!(
        digest,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_REFERENCE_DIGEST
    );
    eprintln!("EML_LN_QUALIFY arm=jit tested={tested} digest={digest:#018x}");
}
