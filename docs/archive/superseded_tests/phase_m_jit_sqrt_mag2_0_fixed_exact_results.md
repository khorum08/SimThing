# SQRT-MAG2-0 — Exact Fixed-Point Pre-Sqrt Mag2 Results

## Base HEAD

`51a0571551c95cfcf797ed1297930092fb29a0f1` (post-SQRT-MAG-0 R1 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_mag2_fixed_exact`, `Mag2SourceContract`, `validate_mag2_source_contract()` |
| `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` | `mag2_source_contract` field; mag2_bits exact-input validation |
| `crates/simthing-spec/src/compile/jit_kernel_graph_identity.rs` | `m2src_contract` in canonical identity |
| `crates/simthing-spec/src/lib.rs`, `compile/mod.rs` | Re-exports |
| `crates/simthing-spec/tests/jit_exact_sqrt_artifact_admission.rs` | SQRT-MAG2-0 admission test |
| `crates/simthing-driver/tests/phase_m_jit_sqrt_mag2_0_fixed_exact.rs` | **New** — GPU fixture + seven SQRT-MAG2-0 tests |
| `docs/invariants.md`, `mapping_current_guidance.md`, `accumulator_op_v2_production_plan.md`, `sqrt_candidates.md`, `worklog.md` | Active doc updates |

## Pre-edit evaluation summary

1. **Exactness gap after SQRT-MAG-0 R1:** F sqrt and exact mag-from-mag2 admission exist, but no pinned exact `mag2` construction — raw f32 `dx²+dy²` had 40/784 GPU/CPU bit mismatches.
2. **Why arbitrary raw f32 `dx²+dy²` is not accepted:** GPU/CPU f32 multiply-add ordering diverges; SQRT-MAG-0 proved this is pre-sqrt, not Candidate F failure.
3. **Fixed-point scale selected:** **Q16.16** (`fraction_bits=16`, scale=65536, `scale_sq=2^32`).
4. **Maximum safe range:** Components ±16.0 (FIELD_POLICY probe table); max `dx²+dy²` integer = 2×(16×65536)² ≈ 2.2×10¹² (u32 limb-pair, no overflow in corpus).
5. **Exact-authoritative `mag2_bits`:** Integer `dx_fixed²+dy_fixed²` as u32 lo/hi limbs; conversion `mag2_f32 = hi + lo/2^32` → f32 bits.
6. **F-backed magnitude feed:** `sqrt_cr_f_bits(mag2_bits)` in same WGSL kernel; feeds `m_jit_mag_f_from_exact_mag2` contract path.
7. **34k benchmark shape:** Deterministic mobile-simthing pairs (28-value gradient table, LCG seed `0x5345_4144`), Q16.16 quantization, one dispatch, readback included.

## Fixed-point representation

| Field | Value |
|---|---|
| Format | Q16.16 signed fixed-point |
| `fraction_bits` | 16 |
| Component range | ±16.0 (bounded FIELD_POLICY probe) |
| Quantization | Round-to-nearest: `(f32 * 65536).round() as i32` |
| Integer mag2 | `u64` sum via u32 limb multiply-add (portable WGSL, no int64) |
| Overflow | None in bounded FIELD_POLICY corpus; max sum fits u64 limbs |
| f32 conversion | `bitcast<f32>(f32(hi) + f32(lo) / 4294967296.0)` |

## Admission matrix

| Path | Exact? | Notes |
|---|---|---|
| Fixed-point mag2 (`m_jit_mag2_fixed_exact`) | **Yes** | `ExactFixedPointDxDy`, outputs `mag2_bits` |
| F sqrt over exact mag2 (`m_jit_mag_f_from_exact_mag2`) | **Yes** | Unchanged; F hash `e2e9e27601ee2e13` |
| Raw f32 dx/dy mag2 (`m_jit_mag_f_from_dxdy_probe`) | **No** | Benchmark probe |
| Diagnostic mag2 (`m_jit_grad_0_observer`) | **No** | Blocked as exact input |
| Native sqrt | **No** | `ApproximateJitOnly` |

## Correctness results

**Edge rows (10):** integer mag2 exact=10/10; F sqrt max_ulp=0.

**Dense corpus (784 pairs):** integer mag2 exact=784/784; F sqrt exact=784/784, max_ulp=0. (All 784 rows — including the 40 previously mismatched raw f32 mag2 rows.)

**34k benchmark:**

```text
inputs=34000 dispatches=1 includes_readback=true
elapsed_ms≈21.5 per_entity_us≈0.63 spot_max_ulp=0
path=fixed_q16_mag2_plus_F_sqrt
```

## Tests/scans run

```bash
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture   # 10 passed
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture        # 11 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_0_fixed_exact -- --nocapture  # 7 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude -- --nocapture  # 12 passed
cargo check --workspace  # ok
```

## Guardrail confirmations

- No scheduler/cache/default SimSession wiring
- No semantic WGSL
- No production economy→mapping bridge
- No `simthing-sim` semantic awareness
- F artifact hash verified: `e2e9e27601ee2e13`
- No Candidate C/f64

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/`.

## Final verdict

**PASS** — SQRT-MAG2-0 landed an exact fixed-point pre-sqrt mag2 construction for the FIELD_POLICY hot path; raw f32 dx/dy mag2 remains non-exact/probe, diagnostic mag2 remains blocked unless routed through the exact mag2 descriptor, F-backed exact sqrt consumes exact mag2 bits successfully, correctness and 34,000-row benchmark results were recorded, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
