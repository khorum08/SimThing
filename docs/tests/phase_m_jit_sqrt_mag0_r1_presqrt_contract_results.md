# SQRT-MAG-0 R1 — Pre-Sqrt Exactness Contract Results

## Base HEAD

`81c77bd3e00b741edf9941ffdab4e13cbd97e591` (post-SQRT-MAG-0 merge)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | Split `m_jit_mag_f_exact` → `m_jit_mag_f_from_exact_mag2` + `m_jit_mag_f_from_dxdy_probe`; `ExactPreSqrtInputContract`; `validate_exact_pre_sqrt_contract()` |
| `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` | `pre_sqrt_contract` on `KernelDescriptorSpec`; exact `mag` requires `ExactMag2Bits`; landed registry updated |
| `crates/simthing-spec/src/compile/jit_kernel_graph_identity.rs` | Canonical identity includes `pre_sqrt_contract` |
| `crates/simthing-spec/src/lib.rs`, `compile/mod.rs` | Re-exports |
| `crates/simthing-spec/tests/jit_exact_sqrt_artifact_admission.rs` | Three R1 admission tests |
| `crates/simthing-spec/tests/jit_kernel_*` (6 files) | `pre_sqrt_contract: None` on manual descriptors |
| `crates/simthing-driver/tests/phase_m_jit_sqrt_mag0_f_exact_magnitude.rs` | R1 tests, mag-from-mag2 GPU path, probe labeling |
| `crates/simthing-driver/tests/phase_m_jit_*` (4 fixture files) | `pre_sqrt_contract: None` |
| `docs/invariants.md` | Pre-sqrt mag2 invariant |
| `docs/workshop/mapping_current_guidance.md` | SQRT-MAG-0 R1 row |
| `docs/accumulator_op_v2_production_plan.md` | R1 compact note |
| `docs/workshop/sqrt_candidates.md` | F vs mag2 construction clarification |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **Where SQRT-MAG-0 claimed exact authority for `mag`:** `m_jit_mag_f_exact` declared `mag` as `ExactAuthoritative` for raw `dx,dy` reads, implying full `dx,dy → mag` exactness.
2. **40 mag2 mismatch rows:** Recorded in [`phase_m_jit_sqrt_mag0_f_exact_magnitude_results.md`](phase_m_jit_sqrt_mag0_f_exact_magnitude_results.md) dense corpus — 784 total, 744 mag2-matched, 40 GPU/CPU `dx²+dy²` bit divergences; `max_ulp=0` only on mag2-matched rows.
3. **Descriptor surface before R1:** Did not distinguish “exact sqrt over exact mag2” from “raw dx/dy magnitude”.
4. **Narrowing without breaking F sqrt:** Yes — `m_jit_sqrt_f_exact` unchanged; magnitude split into exact-from-mag2 vs dx/dy probe.
5. **Exact mag2 construction required for full dx,dy → mag:** Pinned exact-authoritative `mag2` bits (precomputed column, integer/fixed-point construction, constrained SEAD ranges, or future artifact-backed mag2 kernel); squared-magnitude comparisons where sqrt is unnecessary.

## SQRT-MAG-0 exactness gap

SQRT-MAG-0 proved artifact-backed F sqrt is exact **when its input mag2 bits match the oracle**. It did not prove raw `dx,dy` f32 multiply/add produces identical mag2 bits on GPU and CPU. The 40 mismatch rows are pre-sqrt construction divergence, not Candidate F failure.

## Descriptor/admission changes

| Descriptor | Role | `pre_sqrt_contract` | `mag` authority |
|---|---|---|---|
| `m_jit_sqrt_f_exact` | Exact sqrt over non-negative input bits | `None` | N/A (`sqrt_out` exact) |
| `m_jit_mag_f_from_exact_mag2` | Exact mag when mag2 input is exact | `ExactMag2Bits` | `ExactAuthoritative` |
| `m_jit_mag_f_from_dxdy_probe` | Raw dx/dy benchmark probe | `RawDxDyProbe` | `ApproximateDiagnostic` |

Key rule enforced in admission: **F sqrt exactness does not automatically make dx²+dy² exact.**

## Admission matrix

| Path | Exact? | Notes |
|---|---|---|
| F sqrt over exact mag2 (`m_jit_mag_f_from_exact_mag2`) | **Yes** | F hash pin + `ExactMag2Bits` contract |
| Raw dx/dy magnitude (`m_jit_mag_f_from_dxdy_probe`) | **No (probe)** | Executes as benchmark; mag approximate |
| Native sqrt | **No** | `ApproximateJitOnly` |
| Diagnostic `mag2` | **No** | `ApproximateDiagnostic`; blocked as exact input |
| Hash mismatch | **No** | Rejects |
| Arbitrary F text | **No** | Verbatim artifact only |

## Reproduced mag2 mismatch rows

```text
total=784 match=744 mismatch=40
worst: dx=0xbdcccccd dy=0xbd4ccccd gpu_mag2=0x3c4ccccd cpu_mag2=0x3c4cccce
cause: GPU/CPU f32 multiply-add bit divergence on dx2+dy2 (operation order/backend arithmetic)
```

Test: `sqrt_mag0_r1_reproduces_mag2_mismatch_rows`

## 34k benchmark (probe classification)

```text
inputs=34000 dispatches=1 includes_readback=true
elapsed_ms≈5.4 per_entity_us≈0.16 spot_max_ulp=0 (512-row mag2-matched spot check)
classification=raw_dxdy_F_backed_magnitude_probe
```

Not fully exact-authoritative until mag2 construction is pinned/proven.

## Future exact mag2 construction options (design note only)

1. Exact-authoritative precomputed `mag2` column
2. Squared-magnitude comparisons where sqrt is not needed
3. Fixed-point/integer vector components for exact `dx²+dy²`
4. Constrained SEAD gradient ranges where `dx²+dy²` is proven stable
5. Future artifact-backed exact `mag2` kernel

## Tests/scans run

```bash
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture   # 9 passed
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture        # 11 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude -- --nocapture  # 12 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture  # 35 passed, 3 ignored
cargo check --workspace  # ok
```

R1 tests added:

- `sqrt_mag0_r1_accepts_f_sqrt_over_exact_mag2`
- `sqrt_mag0_r1_raw_dxdy_mag_requires_exact_mag2_contract`
- `sqrt_mag0_r1_reproduces_mag2_mismatch_rows`
- `sqrt_mag0_r1_native_sqrt_and_diagnostic_mag2_still_reject`
- `sqrt_mag0_r1_no_default_runtime_wiring`

## Guardrail confirmations

- `MappingExecutionProfile::default() == Disabled`
- No scheduler/cache/default SimSession mapping wiring
- No production economy→mapping bridge
- No semantic WGSL
- No `simthing-sim` semantic awareness
- No E-phase/E11 evidence touched
- F artifact hash verified: `e2e9e27601ee2e13`
- `m_jit_sqrt_f_exact` remains `ExactDeterministic`
- No Candidate C/f64 implementation

## Transient cleanup

No scratch/tmp artifacts deleted under `docs/tests/` — only retained proof logs present (e.g. `phase_m_jit_sqrt_exact5f_exhaustive_batches.log`).

## Final verdict

**PASS** — SQRT-MAG-0 R1 clarified and enforced the pre-sqrt exactness contract: artifact-backed Candidate F sqrt remains exact-authoritative, but Euclidean magnitude is exact-authoritative only when its pre-sqrt mag2 input is exact-authoritative/pinned; raw dx/dy f32 multiply-add magnitude remains a test-only/probe path until exact mag2 construction is proven; native/raw sqrt and diagnostic mag2 remain blocked as exact inputs; no scheduler/cache/default wiring/semantic WGSL/economy bridge was added; active docs and production plan were updated; tests and cargo check are green; V7.7 / Mapping ADR / SEAD posture remains intact.
