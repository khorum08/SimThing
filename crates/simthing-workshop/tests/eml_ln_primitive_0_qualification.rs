//! EML-LN-PRIMITIVE-0 — parity probes, mutation referees, and the LOCAL
//! exhaustive admitted-domain qualification for the admitted `LN` exact primitive.
//!
//! The exhaustive tests are `#[ignore]`: they are a **phase-boundary local
//! certification act** (Owner ruling; full_eml_unification §10.3), never
//! standing CI. Run them with:
//! `cargo test -p simthing-workshop --test eml_ln_primitive_0_qualification -- --ignored --nocapture`

use simthing_core::{
    eml_ln_pinned_f32, eml_opcode, ColumnIndex, EmlNodeGpu, EML_LN_DOMAIN_MAX_BITS,
    EML_LN_DOMAIN_MIN_BITS,
};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, FieldAdjacency, FieldLawProof, FieldSweepOutput,
    FieldSweepRegistration, FieldSweepRegistrationRequest, FieldSweepSession, GpuContext,
};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn certified_context() -> Option<GpuContext> {
    let ctx = GpuContext::new_blocking().ok()?;
    let live =
        simthing_kernel::eml_ln_qualification::EmlLnLiveToolchainIdentity::from_context(&ctx);
    let certified = simthing_kernel::eml_ln_qualification::require_certified_toolchain(&live)
        .expect("live GPU tuple must be in the certified LN toolchain roster");
    eprintln!(
        "EML_LN_TOOLCHAIN live tuple CERTIFIED adapter={:?} backend={:?} driver={:?} qualified_on={}",
        live.adapter, live.backend, live.driver, certified.qualified_on
    );
    Some(ctx)
}

const CHUNK_SLOTS: usize = 1 << 21;

fn fnv_fold(mut hash: u64, bits: u32) -> u64 {
    for byte in bits.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn domain_bit_range() -> (u32, u32) {
    (EML_LN_DOMAIN_MIN_BITS, EML_LN_DOMAIN_MAX_BITS)
}

fn domain_size() -> u64 {
    u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS) + 1
}

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

fn probe_corpus() -> Vec<f32> {
    let mut corpus = vec![
        f32::from_bits(EML_LN_DOMAIN_MIN_BITS),
        f32::from_bits(EML_LN_DOMAIN_MAX_BITS),
        1.0,
        2.0,
        0.5,
        4.0,
        0.25,
        f32::from_bits(0x3F80_0000), // 1.0 exact
        f32::from_bits(0x3F00_0000), // 0.5 exact
        // Prior magnets that are probe-stable under LN1C (exhaustive still finds
        // additional near-floor 1-ULP gaps — see results STOP note).
        f32::from_bits(0x0095_db87),
        f32::from_bits(0x7bf2_98ff),
        f32::from_bits(0x0b80_0000),
        f32::from_bits(0x3f7f_ff10),
    ];
    // Neighborhoods of 1.0 (±512 ULPs).
    let one = 1.0f32.to_bits();
    for delta in -512i32..=512 {
        let bits = (one as i64 + i64::from(delta)) as u32;
        if in_domain(bits) {
            corpus.push(f32::from_bits(bits));
        }
    }
    // Powers of two across the admitted domain.
    for exp in -125i32..=127 {
        let bits = ((exp + 127) as u32) << 23;
        if in_domain(bits) {
            corpus.push(f32::from_bits(bits));
        }
    }
    // Endpoint neighborhoods (512 ULPs inward).
    let mut bits = EML_LN_DOMAIN_MIN_BITS;
    for _ in 0..512 {
        corpus.push(f32::from_bits(bits));
        bits += 1;
    }
    let mut bits = EML_LN_DOMAIN_MAX_BITS;
    for _ in 0..512 {
        corpus.push(f32::from_bits(bits));
        bits -= 1;
    }
    // Deterministic LCG stratum.
    let mut state = 0x243F_6A88_85A3_08D3u64;
    for _ in 0..4096 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let unit = ((state >> 40) as f32) / (1u32 << 24) as f32;
        let min = f32::from_bits(EML_LN_DOMAIN_MIN_BITS);
        let max = f32::from_bits(EML_LN_DOMAIN_MAX_BITS);
        let x = min + (max - min) * unit;
        if in_domain(x.to_bits()) {
            corpus.push(x);
        }
    }
    corpus
}

/// Values chosen to exercise mantissa reduction and the Lg* polynomial — the
/// planted mutants below must diverge on at least one of these even when the
/// broader probe stratum happens to agree.
fn mutant_referee_corpus() -> Vec<f32> {
    let mut corpus = probe_corpus();
    corpus.extend([
        1.5f32,
        2.0,
        0.75,
        4.0,
        8.0,
        f32::from_bits(0x3fb5_04f4),
        f32::from_bits(0x3fc0_0000),
        f32::from_bits(0x4000_0000),
    ]);
    corpus
}

fn digest_reds_on_corpus(
    reference_fn: impl Fn(f32) -> f32,
    mutant_fn: impl Fn(f32) -> f32,
    corpus: &[f32],
    label: &str,
) {
    let (reference_digest, mutant_digest) = corpus.iter().fold(
        (FNV_OFFSET, FNV_OFFSET),
        |(reference_digest, mutant_digest), input| {
            (
                fnv_fold(reference_digest, reference_fn(*input).to_bits()),
                fnv_fold(mutant_digest, mutant_fn(*input).to_bits()),
            )
        },
    );
    assert_ne!(
        reference_digest, mutant_digest,
        "planted {label} defect must RED the digest referee"
    );
}

fn in_domain(bits: u32) -> bool {
    bits >= EML_LN_DOMAIN_MIN_BITS && bits <= EML_LN_DOMAIN_MAX_BITS
}

fn gpu_ln_outputs(
    ctx: &GpuContext,
    session: &mut FieldSweepSession,
    registration: &FieldSweepRegistration,
    inputs: &[f32],
) -> Vec<f32> {
    session.upload_values(ctx, inputs).expect("chunk upload");
    session
        .dispatch_chain(ctx, std::slice::from_ref(registration), 1)
        .expect("chunk dispatch");
    session.readback(ctx).expect("chunk readback")
}

fn run_gpu_arm(interpreted: bool, arm: &str) -> (u64, u64) {
    let ctx =
        certified_context().expect("exhaustive qualification requires the local certified GPU");
    let registration = ln_elementwise_registration(CHUNK_SLOTS as u32);
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
    let mut inputs = vec![0.0f32; CHUNK_SLOTS];
    let mut fill = 0usize;
    let (start, end) = domain_bit_range();
    let mut flush = |inputs: &mut Vec<f32>,
                     fill: &mut usize,
                     digest: &mut u64,
                     tested: &mut u64,
                     session: &mut FieldSweepSession| {
        if *fill == 0 {
            return;
        }
        for slot in inputs[*fill..].iter_mut() {
            *slot = 0.0;
        }
        let outputs = gpu_ln_outputs(&ctx, session, &registration, inputs);
        for (index, output) in outputs[..*fill].iter().enumerate() {
            let expected = eml_ln_pinned_f32(inputs[index]);
            assert_eq!(
                output.to_bits(),
                expected.to_bits(),
                "{arm}: first divergence at input bits {:#010x}",
                inputs[index].to_bits()
            );
            *digest = fnv_fold(*digest, output.to_bits());
        }
        *tested += *fill as u64;
        *fill = 0;
    };

    let mut bits = start;
    loop {
        inputs[fill] = f32::from_bits(bits);
        fill += 1;
        if fill == CHUNK_SLOTS {
            flush(&mut inputs, &mut fill, &mut digest, &mut tested, &mut session);
            eprintln!("EML_LN_QUALIFY arm={arm} progress bits={bits:#010x} tested={tested}");
        }
        if bits == end {
            break;
        }
        bits += 1;
    }
    flush(&mut inputs, &mut fill, &mut digest, &mut tested, &mut session);
    (digest, tested)
}

fn cpu_reference_digest() -> (u64, u64) {
    let mut digest = FNV_OFFSET;
    let mut tested = 0u64;
    let (start, end) = domain_bit_range();
    let mut bits = start;
    loop {
        let output = eml_ln_pinned_f32(f32::from_bits(bits));
        digest = fnv_fold(digest, output.to_bits());
        tested += 1;
        if bits == end {
            break;
        }
        bits += 1;
    }
    (digest, tested)
}

#[test]
fn eml_ln_primitive_0_probe_corpus_is_three_way_bit_exact() {
    let Some(ctx) = certified_context() else {
        return;
    };
    let corpus = probe_corpus();
    let slots = corpus.len() as u32;
    let registration = ln_elementwise_registration(slots);
    let class = registration.resource_class();
    for (arm, interpreted) in [("jit", false), ("interpreted", true)] {
        let mut session = if interpreted {
            FieldSweepSession::new_interpreted_for_profiling(&ctx, &registration, class)
                .expect("interpreted session")
        } else {
            FieldSweepSession::new_with_profiling_resource_class(&ctx, &registration, class)
                .expect("generated JIT session")
        };
        let outputs = gpu_ln_outputs(&ctx, &mut session, &registration, &corpus);
        let mut mismatches = 0u64;
        let mut first = None;
        for (index, output) in outputs.iter().enumerate() {
            let expected = eml_ln_pinned_f32(corpus[index]);
            if output.to_bits() != expected.to_bits() {
                mismatches += 1;
                if first.is_none() {
                    first = Some((corpus[index].to_bits(), output.to_bits(), expected.to_bits()));
                }
            }
        }
        if let Some((in_bits, got, want)) = first {
            eprintln!(
                "EML_LN_PROBE arm={arm} corpus={} mismatches={mismatches} first_in={in_bits:#010x} got={got:#010x} want={want:#010x}",
                corpus.len()
            );
        } else {
            eprintln!(
                "EML_LN_PROBE arm={arm} corpus={} CPU/GPU=bit-exact",
                corpus.len()
            );
        }
        assert_eq!(mismatches, 0, "{arm}: probe must be bit-exact vs CPU twin");
    }
}

/// Shared LN1M skeleton used by planted mutants — mirrors `eml_ln_pinned_f32`
/// except for the single planted defect named by each test.
fn ln1m_skeleton(x: f32, pivot: u32, newton_iters: usize, lg1: f32) -> f32 {
    use simthing_core::eml_ln::{
        EML_LN_LG2, EML_LN_LG3, EML_LN_LG4, EML_LN_LN2, EML_LN_RECIP_MAGIC, EML_LN_THIRD,
    };
    let ix = x.to_bits();
    let mut k = ((ix >> 23) as i32) - 127;
    let mant = ix & 0x007f_ffff;
    let mut mx = mant | 0x3f80_0000;
    if mx > pivot {
        mx -= 0x0080_0000;
        k += 1;
    }
    let m = f32::from_bits(mx);
    let f = m - 1.0_f32;
    let dk = k as f32;
    let ln1p = if (0x007f_ffff & (0x8000 + mant)) < 0xc000 {
        if f == 0.0 {
            0.0
        } else {
            let inner = 0.5_f32 - EML_LN_THIRD * f;
            let f2 = f * f;
            let r = f2 * inner;
            f - r
        }
    } else {
        let y = 2.0_f32 + f;
        let mut r = f32::from_bits(EML_LN_RECIP_MAGIC.wrapping_sub(y.to_bits()));
        for _ in 0..newton_iters {
            r = r * (2.0_f32 - y * r);
        }
        let s = f * r;
        let z = s * s;
        let w = z * z;
        let t1 = w * w.mul_add(EML_LN_LG4, EML_LN_LG2);
        let t2 = z * w.mul_add(EML_LN_LG3, lg1);
        let poly = t2 + t1;
        let f2 = f * f;
        let hfsq = 0.5_f32 * f2;
        let hp = hfsq + poly;
        let s_term = s * hp;
        let mid = hfsq - s_term;
        f - mid
    };
    dk.mul_add(EML_LN_LN2, ln1p)
}

#[test]
fn eml_ln_primitive_0_planted_one_newton_iteration_mutant_reds_the_digest() {
    fn mutant_one_newton(x: f32) -> f32 {
        // PLANTED DEFECT: one Newton iteration instead of two.
        ln1m_skeleton(
            x,
            simthing_core::eml_ln::EML_LN_SQRT2_BITS,
            1,
            simthing_core::eml_ln::EML_LN_LG1,
        )
    }
    digest_reds_on_corpus(
        eml_ln_pinned_f32,
        mutant_one_newton,
        &mutant_referee_corpus(),
        "one-Newton-iteration",
    );
}

#[test]
fn eml_ln_primitive_0_planted_wrong_sqrt2_pivot_mutant_reds_the_digest() {
    fn mutant_wrong_sqrt2(x: f32) -> f32 {
        // PLANTED DEFECT: pivot at 1.0 instead of √2 — wrong half-scale reduce.
        ln1m_skeleton(x, 0x3F80_0000, 2, simthing_core::eml_ln::EML_LN_LG1)
    }
    digest_reds_on_corpus(
        eml_ln_pinned_f32,
        mutant_wrong_sqrt2,
        &mutant_referee_corpus(),
        "wrong-sqrt2-pivot",
    );
}

#[test]
fn eml_ln_primitive_0_planted_wrong_poly_coeff_mutant_reds_the_digest() {
    fn mutant_wrong_lg1(x: f32) -> f32 {
        // PLANTED DEFECT: zero the primary odd coefficient.
        ln1m_skeleton(x, simthing_core::eml_ln::EML_LN_SQRT2_BITS, 2, 0.0)
    }
    digest_reds_on_corpus(
        eml_ln_pinned_f32,
        mutant_wrong_lg1,
        &mutant_referee_corpus(),
        "wrong-LG1-coefficient",
    );
}

#[test]
#[ignore = "local phase-boundary characterization: envelope vs f64 ln reference"]
fn eml_ln_primitive_0_numerical_characterization() {
    fn ulp_distance(a: f32, b: f32) -> i64 {
        fn key(x: f32) -> i64 {
            let bits = i64::from(x.to_bits());
            if bits & 0x8000_0000 != 0 {
                0x8000_0000_i64 - bits
            } else {
                bits + 0x8000_0000_i64
            }
        }
        (key(a) - key(b)).abs()
    }
    let mut max_ulp = 0i64;
    let mut max_at = 0u32;
    let mut max_rel = 0f64;
    let mut nonfinite = 0u64;
    let mut checked = 0u64;
    let mut check = |bits: u32,
                     max_ulp: &mut i64,
                     max_at: &mut u32,
                     max_rel: &mut f64,
                     nonfinite: &mut u64,
                     checked: &mut u64| {
        let x = f32::from_bits(bits);
        let got = eml_ln_pinned_f32(x);
        let reference = (f64::from(x)).ln();
        if !got.is_finite() {
            *nonfinite += 1;
        }
        let distance = ulp_distance(got, reference as f32);
        if distance > *max_ulp {
            *max_ulp = distance;
            *max_at = bits;
        }
        let relative = if reference == 0.0 {
            f64::from(got).abs()
        } else {
            ((f64::from(got) - reference) / reference).abs()
        };
        if relative > *max_rel {
            *max_rel = relative;
        }
        *checked += 1;
    };
    for endpoint in [EML_LN_DOMAIN_MIN_BITS, EML_LN_DOMAIN_MAX_BITS] {
        let mut bits = endpoint;
        for _ in 0..4096 {
            check(
                bits,
                &mut max_ulp,
                &mut max_at,
                &mut max_rel,
                &mut nonfinite,
                &mut checked,
            );
            if endpoint == EML_LN_DOMAIN_MIN_BITS {
                bits += 1;
            } else {
                bits -= 1;
            }
        }
    }
    let span = u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS);
    let samples = 1u64 << 20;
    for index in 0..=samples {
        let bits = EML_LN_DOMAIN_MIN_BITS + ((span * index) / samples) as u32;
        check(
            bits,
            &mut max_ulp,
            &mut max_at,
            &mut max_rel,
            &mut nonfinite,
            &mut checked,
        );
    }
    let mut monotonicity_violations = 0u64;
    for exp in -125i32..=127 {
        let bits = ((exp + 127) as u32) << 23;
        if !in_domain(bits) {
            continue;
        }
        let mut previous = eml_ln_pinned_f32(f32::from_bits(bits));
        let mut walk = bits;
        for _ in 0..10_000 {
            walk += 1;
            if !in_domain(walk) {
                break;
            }
            let value = eml_ln_pinned_f32(f32::from_bits(walk));
            if value < previous {
                monotonicity_violations += 1;
            }
            previous = value;
        }
    }
    eprintln!(
        "EML_LN_CHARACTERIZE checked={checked} max_ulp={max_ulp} at_bits={max_at:#010x} \
         max_rel={max_rel:.3e} nonfinite={nonfinite} monotonicity_violations={monotonicity_violations}"
    );
    assert_eq!(nonfinite, 0, "every in-domain output is finite");
    assert_eq!(
        monotonicity_violations, 0,
        "adjacent-input spot sweeps stay monotone on powers-of-two neighborhoods"
    );
    assert!(
        max_ulp <= 8,
        "observed envelope must stay in the low-ULP class (got {max_ulp})"
    );
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive admitted-domain CPU reference"]
fn eml_ln_primitive_0_exhaustive_cpu_reference() {
    let (digest, tested) = cpu_reference_digest();
    assert_eq!(tested, domain_size());
    assert_eq!(
        tested,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_DOMAIN_SIZE
    );
    assert_eq!(
        digest,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_REFERENCE_DIGEST,
        "CPU reference digest must match the pinned qualification artifact"
    );
    eprintln!(
        "EML_LN_QUALIFY arm=cpu-reference tested={tested} digest={digest:#018x} algorithm={:#018x}",
        simthing_core::eml_ln::EML_LN_ALGORITHM_IDENTITY
    );
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive interpreted-arm replay"]
fn eml_ln_primitive_0_exhaustive_interpreted_replay() {
    let (digest, tested) = run_gpu_arm(true, "interpreted");
    assert_eq!(tested, domain_size());
    assert_eq!(
        digest,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_REFERENCE_DIGEST,
        "interpreted replay digest must match the pinned reference"
    );
    eprintln!("EML_LN_QUALIFY arm=interpreted tested={tested} digest={digest:#018x}");
}

#[test]
#[ignore = "local phase-boundary certification: exhaustive SSA-JIT-arm replay"]
fn eml_ln_primitive_0_exhaustive_jit_replay() {
    let (digest, tested) = run_gpu_arm(false, "jit");
    assert_eq!(tested, domain_size());
    assert_eq!(
        digest,
        simthing_kernel::eml_ln_qualification::EML_LN_EXHAUSTIVE_REFERENCE_DIGEST,
        "SSA-JIT replay digest must match the pinned reference"
    );
    eprintln!("EML_LN_QUALIFY arm=jit tested={tested} digest={digest:#018x}");
}
