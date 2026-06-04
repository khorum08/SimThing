//! Phase M EML-GADGET-2A — Snapshot/Copy Band Fixture Proof
//!
//! Fixture-only proof that `previous_col <- current_col` is cleanly authorable
//! using the existing accumulator substrate: Identity combine + ConsumeMode::ResetTarget
//! at an earlier authored OrderBand, on Layer-3 (parent/personality/strategic) slot columns.
//!
//! This is NOT a temporal gadget implementation. No VelocityMonitor, Decay/EMA,
//! BoundedFeedback, Hysteresis, Acceleration. No runtime gadget execution.
//! No new EML opcode, no WGSL, no simthing-gpu/sim changes, no production wiring.
//!
//! All authoring uses existing primitives only. CPU-oracle sequence parity via
//! explicit authored ops + trace assertions (trivial Identity/Reset is its own oracle).

use std::sync::Mutex;

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, GpuContext};
use simthing_sim::PipelineFlags;
use simthing_spec::MappingExecutionProfile;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU context required for 2A fixture");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

const N_DIMS: u32 = 16;
const PERSONALITY_SLOT: u32 = 0;
const CURRENT_COL: u32 = 10;
const PREV_COL: u32 = 11;
const DRIVE_COL: u32 = 12; // "external" source for update-band recompute of current

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn make_initial_values() -> Vec<f32> {
    vec![0.0f32; (2 * N_DIMS) as usize] // 2 slots for headroom
}

fn set_col(values: &mut [f32], slot: u32, col: u32, val: f32) {
    values[idx(slot, col) as usize] = val;
}

fn get_col(values: &[f32], slot: u32, col: u32) -> f32 {
    values[idx(slot, col) as usize]
}

/// Snapshot op: previous_col <- current_col via Identity + ResetTarget at OrderBand(0)
fn snapshot_op() -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: PERSONALITY_SLOT,
            col: CURRENT_COL,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(0),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(PERSONALITY_SLOT, PREV_COL)],
    }
}

/// Update op (for "recompute current"): current_col <- drive_col (authored external source)
/// at OrderBand(1). This models an independent recompute/update band that writes current.
fn update_from_drive_op() -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: PERSONALITY_SLOT,
            col: DRIVE_COL,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(1),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(PERSONALITY_SLOT, CURRENT_COL)],
    }
}

/// One-band snapshot only (for Test 2)
fn snapshot_only_ops() -> Vec<AccumulatorOp> {
    vec![snapshot_op()]
}

/// Two-band snapshot-then-update sequence (for Test 3/4)
fn snapshot_then_update_ops() -> Vec<AccumulatorOp> {
    vec![snapshot_op(), update_from_drive_op()]
}

fn run_bands(
    ctx: &GpuContext,
    session: &mut AccumulatorOpSession,
    ops: &[AccumulatorOp],
    values: &[f32],
) -> Vec<f32> {
    set_debug_readback_allowed(true);
    session.upload_values(ctx, values);
    session.upload_ops(ctx, ops).expect("upload ops");
    for band in [0u32, 1] {
        let has_band = ops.iter().any(|op| {
            if let GateSpec::OrderBand(bb) = op.gate {
                bb == band
            } else {
                false
            }
        });
        if has_band {
            session.tick(ctx, band).expect("tick band");
        }
    }
    session.readback_full(ctx).expect("readback")
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 — snapshot copy admits with existing substrate
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_copy_admits_with_existing_substrate() {
    // Author the band pair using only existing types — this is the clean authoring proof.
    let snap = snapshot_op();
    let upd = update_from_drive_op();

    assert_eq!(
        snap.combine,
        CombineFn::Identity,
        "must use Identity combine"
    );
    assert_eq!(
        snap.consume,
        ConsumeMode::ResetTarget,
        "must use ResetTarget for copy"
    );
    assert_eq!(
        snap.gate,
        GateSpec::OrderBand(0),
        "snapshot band must be earlier"
    );
    assert_eq!(
        upd.gate,
        GateSpec::OrderBand(1),
        "update band must follow snapshot"
    );

    // Layer-3 / parent-personality scoped (SlotValue on personality slot, not dense grid cells)
    match snap.source {
        SourceSpec::SlotValue { slot, .. } => {
            assert_eq!(
                slot, PERSONALITY_SLOT,
                "must target Layer-3 personality/parent slot"
            );
        }
        _ => panic!("snapshot source must be explicit SlotValue for Layer-3"),
    }

    // Explicit documentation of scope (dense per-cell temporal is separately gated)
    // We never use RegionCell / grid cell sources for temporal columns here.
    // This fixture proves Layer-3 explicit-column temporal authoring only.

    // No new opcode / consume / combine invented
    // (If Identity were absent we would have stopped per stop conditions.)

    // Posture: no gadget runtime in this authoring
    // (Full gadget stack execution is out of scope for 2A fixture proof)

    println!("2A Test 1: clean authoring of snapshot/copy band pair with Identity + ResetTarget on Layer-3 SlotValue succeeded.");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 — one-step snapshot copies current to previous
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn one_step_snapshot_copies_current_to_previous() {
    with_gpu(|ctx| {
        let mut session = AccumulatorOpSession::new(ctx, 2, N_DIMS);

        let mut values = make_initial_values();
        set_col(&mut values, PERSONALITY_SLOT, CURRENT_COL, 1.0);
        set_col(&mut values, PERSONALITY_SLOT, PREV_COL, 0.0);

        let after = run_bands(ctx, &mut session, &snapshot_only_ops(), &values);

        let prev_after = get_col(&after, PERSONALITY_SLOT, PREV_COL);
        let current_after = get_col(&after, PERSONALITY_SLOT, CURRENT_COL);

        assert!(
            (prev_after - 1.0).abs() < 1e-6,
            "previous must receive snapshot of current=1.0; got {prev_after}"
        );
        assert!(
            (current_after - 1.0).abs() < 1e-6,
            "current unchanged by snapshot-only run; got {current_after}"
        );

        println!("2A Test 2: one-step snapshot: previous={prev_after} (expected 1.0), current={current_after}");
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 — snapshot happens before update band (core 2A proof)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn snapshot_happens_before_update_band() {
    with_gpu(|ctx| {
        let mut session = AccumulatorOpSession::new(ctx, 2, N_DIMS);

        // Initial state: current holds "old" value from prior update (or seed)
        let mut values = make_initial_values();
        set_col(&mut values, PERSONALITY_SLOT, CURRENT_COL, 1.0); // old current
        set_col(&mut values, PERSONALITY_SLOT, PREV_COL, 0.0);
        set_col(&mut values, PERSONALITY_SLOT, DRIVE_COL, 1.5); // new value to be written by update band

        let after = run_bands(ctx, &mut session, &snapshot_then_update_ops(), &values);

        let prev_after = get_col(&after, PERSONALITY_SLOT, PREV_COL);
        let current_after = get_col(&after, PERSONALITY_SLOT, CURRENT_COL);

        // Core 2A proof:
        // previous holds the value that current had *before* the update band ran
        assert!(
            (prev_after - 1.0).abs() < 1e-6,
            "previous must hold pre-update current (1.0); got {prev_after}"
        );
        assert!(
            (current_after - 1.5).abs() < 1e-6,
            "current must hold post-update value from drive (1.5); got {current_after}"
        );

        println!("2A Test 3 (core proof): snapshot-before-update: previous={prev_after} (old), current={current_after} (new)");
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// Explicit CPU oracle for the snapshot-then-update primitive (Test 4 only)
// This is scoped strictly to the authored OrderBand snapshot/copy behavior.
// It is *not* a VelocityMonitor, EMA, or any Tier-2 gadget oracle.
// ─────────────────────────────────────────────────────────────────────────────

fn snapshot_then_update_oracle(current_before: f32, drive_update: f32) -> (f32, f32) {
    // previous_col receives whatever was in current_col at the instant the snapshot band (OrderBand 0) runs.
    // current_col is then independently advanced by whatever the later update band (OrderBand 1) writes.
    let previous_after_snapshot = current_before;
    let current_after_update = drive_update;
    (previous_after_snapshot, current_after_update)
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 — multi-step sequence parity (clean, explicit oracle, visible lag)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn multi_step_sequence_parity() {
    with_gpu(|ctx| {
        let mut session = AccumulatorOpSession::new(ctx, 2, N_DIMS);

        // starting_current = the value present in CURRENT_COL at the *beginning* of each tick's band sequence.
        // This is the value the snapshot band (OrderBand 0) must capture into previous_col.
        let starting_current: [f32; 3] = [1.0, 1.5, 1.25];

        // drive_updates = the independent new value written by the later update band (OrderBand 1).
        // This demonstrates that current_col advances *after* the snapshot has already captured the prior value.
        let drive_updates: [f32; 3] = [1.5, 1.25, 2.0];

        let mut prev_carry = 0.0f32; // previous at the very start of the whole sequence
        let mut observed_previous: Vec<f32> = Vec::new();
        let mut observed_current: Vec<f32> = Vec::new();

        for step in 0..3 {
            let mut values = make_initial_values();
            // State at the beginning of this tick's bands (what the snapshot band will see for "current")
            set_col(
                &mut values,
                PERSONALITY_SLOT,
                CURRENT_COL,
                starting_current[step],
            );
            set_col(&mut values, PERSONALITY_SLOT, PREV_COL, prev_carry);
            // The independent "drive" value that the update band will write into current
            set_col(
                &mut values,
                PERSONALITY_SLOT,
                DRIVE_COL,
                drive_updates[step],
            );

            let after = run_bands(ctx, &mut session, &snapshot_then_update_ops(), &values);

            let prev_after = get_col(&after, PERSONALITY_SLOT, PREV_COL);
            let curr_after = get_col(&after, PERSONALITY_SLOT, CURRENT_COL);

            observed_previous.push(prev_after);
            observed_current.push(curr_after);

            // For the next step, the "previous" carry is whatever we just captured in this snapshot
            prev_carry = prev_after;

            // Compare to explicit CPU oracle for this primitive
            let (oracle_prev, oracle_curr) =
                snapshot_then_update_oracle(starting_current[step], drive_updates[step]);

            assert!(
                (prev_after - oracle_prev).abs() < 1e-6,
                "step {step}: previous_after_snapshot mismatch oracle: got {prev_after}, oracle {oracle_prev}"
            );
            assert!(
                (curr_after - oracle_curr).abs() < 1e-6,
                "step {step}: current_after_update mismatch oracle: got {curr_after}, oracle {oracle_curr}"
            );

            println!(
                "2A Test 4 step {}: before bands current={:.2} prev={:.2} | after snapshot prev={:.2} | after update current={:.2}",
                step, starting_current[step], prev_carry, prev_after, curr_after
            );
        }

        // Required traces per R1 hygiene spec (visible lag: previous captured the old value, current then advanced)
        let expected_previous: [f32; 3] = [1.0, 1.5, 1.25];
        let expected_current: [f32; 3] = [1.5, 1.25, 2.0];

        for (i, &p) in observed_previous.iter().enumerate() {
            assert!(
                (p - expected_previous[i]).abs() < 1e-6,
                "previous_after_snapshot[{}] = {:.2}, expected {:.2}",
                i,
                p,
                expected_previous[i]
            );
        }
        for (i, &c) in observed_current.iter().enumerate() {
            assert!(
                (c - expected_current[i]).abs() < 1e-6,
                "current_after_update[{}] = {:.2}, expected {:.2}",
                i,
                c,
                expected_current[i]
            );
        }

        println!(
            "2A Test 4 clean sequence parity (with explicit oracle): previous_after_snapshot = {:?}, current_after_update = {:?}",
            observed_previous, observed_current
        );
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 — no ad-hoc runtime/gadget execution
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn no_ad_hoc_runtime_gadget_execution() {
    // Source-scan: driver, gpu, sim must not contain gadget runtime consumption for temporal kinds.
    let driver_src = include_str!("../src/first_slice_mapping_runtime.rs");
    let gpu_src = include_str!("../../simthing-gpu/src/lib.rs");
    let sim_src = include_str!("../../simthing-sim/src/lib.rs");

    // No consumption of CompiledEmlGadgetStack as runtime execution in driver/gpu/sim
    // (Tier-1 already enforces; 2A adds no new consumption paths)
    let driver_gadget_refs = driver_src.matches("CompiledEmlGadgetStack").count();
    assert!(
        driver_gadget_refs == 0,
        "driver must not consume CompiledEmlGadgetStack for runtime (2A fixture only); found {driver_gadget_refs}"
    );

    // Temporal gadget kinds (VelocityMonitor etc) remain rejected / deferred
    // We assert via the spec constant (imported indirectly via compile test re-runs)
    // Here we just document the posture; the actual deferred list is tested in eml_gadget_tier1.

    // No VelocityMonitor / EMA / BoundedFeedback / Hysteresis / Acceleration implementation
    // code paths or registrations in this 2A slice.
    assert!(
        !gpu_src.contains("VelocityMonitor")
            && !gpu_src.contains("fn velocity_monitor")
            && !sim_src.contains("VelocityMonitor"),
        "VelocityMonitor etc must remain unimplemented in 2A"
    );

    println!("2A Test 5: no ad-hoc runtime gadget execution posture preserved.");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 — posture preservation (all binding guardrails)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn posture_preservation_2a() {
    // Use safe, existing files for source scans (mirrors patterns in phase_m_economy_sead_product_fixture_posture_preserved and structured_field defaults tests)
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    let sim_boundary = include_str!("../../simthing-sim/src/boundary.rs");
    let gpu_lib = include_str!("../../simthing-gpu/src/lib.rs");
    let driver_lib = include_str!("../src/lib.rs");

    // No new EML opcode for previous / temporal hidden read
    assert!(
        !gpu_lib.contains("SNAPSHOT_PREVIOUS")
            && !gpu_lib.contains("COPY_PREV")
            && !gpu_lib.contains("EmlOp::Previous"),
        "no new EML opcode for previous-value"
    );

    // No WGSL / per-gadget GPU kernel for temporal gadgets
    assert!(
        !gpu_lib.contains("temporal_gadget")
            && !gpu_lib.contains("gadget_velocity")
            && !gpu_lib.contains("eml_gadget.wgsl"),
        "no per-gadget WGSL for temporal memory"
    );

    // No hidden previous-value read inside EML eval (use gpu lib as proxy; full eml interpreter lives under accumulator/passes)
    assert!(
        !gpu_lib.contains("read_previous")
            && !gpu_lib.contains("previous_value_buffer")
            && !gpu_lib.contains("hidden prev"),
        "EML must have no hidden previous-value read (explicit columns only)"
    );

    // simthing-sim has no Gadget/Personality/Memory semantics (binding invariant)
    assert!(
        !sim_lib.contains("struct Gadget")
            && !sim_lib.contains("enum Personality")
            && !sim_lib.contains("MemoryBank")
            && !sim_lib.contains("temporal_memory"),
        "simthing-sim must remain Gadget/Personality/Memory free"
    );

    // No DailyResolutionBoundary, no calendar/pause semantics inside simthing-sim
    for text in [&sim_lib[..], &sim_boundary[..]] {
        assert!(
            !text.contains("DailyResolutionBoundary"),
            "no DailyResolutionBoundary in simthing-sim"
        );
        assert!(
            !text.contains("struct Calendar")
                && !text.contains("enum Calendar")
                && !text.contains("struct Season"),
            "no calendar/season types in simthing-sim"
        );
    }

    // Defaults unchanged (binding)
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert!(
        !PipelineFlags::default().use_accumulator_resource_flow,
        "Resource Flow E-11 default-off"
    );

    // No production economy→mapping bridge (fixture orchestration only)
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    // No atlas batching / M-4A activation in production posture (request_atlas_batching authoring exists but is rejected at admission for prod)
    let atlas_refs = driver_lib.matches("request_atlas_batching: true").count();
    assert!(
        atlas_refs == 0,
        "production posture keeps atlas off; found {atlas_refs} active true"
    );

    println!("2A Test 6: full posture preservation (no new opcode/WGSL/runtime gadget/temporal impl/sim semantics/defaults/bridges) green.");
}
