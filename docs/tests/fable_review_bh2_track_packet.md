# Fable review packet — BH / BH-2 track (0.0.8.1)

> **Status: FABLE-REVIEW-FREEZE (2026-06-11).** BH-0…BH-2D-OBS-100R closed; R1-TEST-PURGE complete;
> Candidate F §0.7 in transient constitution; no runtime churn in this freeze pass.

This packet is the canonical handoff for Fable review. Production docs:
[`design_0_0_8_1.md`](../design_0_0_8_1.md),
[`design_0_0_8_1_border_hack_track.md`](../design_0_0_8_1_border_hack_track.md).

---

## 1. Track status

| Rung | Status | Live surface | Evidence | Remaining risk |
|---|---|---|---|---|
| **BH-0** | ACCEPTED | `SaturatingFlux` GPU stencil | [`bh0_saturating_flux_results.md`](bh0_saturating_flux_results.md) | None for this rung |
| **BH-1** | ACCEPTED | Choke readout column in flux dispatch | [`bh1_choke_readout_results.md`](bh1_choke_readout_results.md) | None for this rung |
| **BH-1R** | ACCEPTED | `SaturatingFluxChokeThresholdOp` (4-float compact readback) | [`bh1r_choke_consumption_results.md`](bh1r_choke_consumption_results.md) | Threshold consumer only; not movement |
| **BH-1R-SCALE** | ACCEPTED | Staged parallel GPU reduction (256 + fold) | [`bh1r_scale_parallel_reduction_results.md`](bh1r_scale_parallel_reduction_results.md) | Scale path; not default session wiring |
| **BH-2A** | ACCEPTED | Named consumer `CT-4b_Local_Automata_W_Feedstock` (docs contract) | border hack track §9 | Consumer name is docs/tests; production uses generic columns |
| **BH-2B** | ACCEPTED | `WImpedanceComposeOp` + bridge | [`bh2_w_composition_results.md`](bh2_w_composition_results.md) | Linear W only; no sqrt in hot path |
| **BH-2S** | ACCEPTED | `StressComposeOp` (overlap/mismatch/velocity) | [`bh2s_overlap_stress_results.md`](bh2s_overlap_stress_results.md) | Stress algebra only |
| **BH-2S-DOC** | ACCEPTED | Consumer service-surface documentation | border hack track §11 | Doc rung; no new GPU |
| **BH-2C** | ACCEPTED | Composed W → PALMA `GpuInterleavedW` → resident D | [`bh2c_palma_feedstock_results.md`](bh2c_palma_feedstock_results.md) | PALMA is utility, not pathfinding engine |
| **BH-2D** | ACCEPTED | CT-4b 200×200 fixture resident feedstock chain | [`bh2d_ct4b_fixture_results.md`](bh2d_ct4b_fixture_results.md) | Fixture proof; test harness quarantined |
| **BH-2D-OBS-100R** | ACCEPTED | Dynamic 100-tick observation (test-only stimulus) | [`bh2d_ct4b_100tick_scenario_observations.md`](bh2d_ct4b_100tick_scenario_observations.md) | Probe-implied movement-front only; not production movement |
| **BH-3-AUTHORING-0** | ACCEPTED | ClauseThing `hydrate_field_operator_pack` → generic spec + driver bridges | [`../archive/superseded_tests/bh3_authoring_0_results.md`](../archive/superseded_tests/bh3_authoring_0_results.md) | Superseded as primary proof by PR4/PR7/PR9 battery; historical only |
| **R1-TEST-PURGE** | ACCEPTED | Fast R1* sentinels only in default workspace | [`r1_default_workspace_purge_results.md`](r1_default_workspace_purge_results.md) | Do not reintroduce proof-ledger batteries |
| **CANDIDATE-F-DOC** | ACCEPTED | §0.7 transient constitution exact-sqrt rule | [`design_0_0_8_1.md`](../design_0_0_8_1.md) §0.7 | Artifact hash pinned; not in core design |

**Not opened:** production movement policy, pathfinding engine, border/frontline service.
**Opened (BH-3-AUTHORING-0):** ClauseThing authoring/lowering for field operators only — no runtime semantics.

---

## 2. Live production surfaces

| Surface | Owner | Classification | Readback | CPU oracle |
|---|---|---|---|---|
| **SaturatingFlux** | `simthing-gpu` structured field / `saturating_flux` WGSL | LIVE_API | GPU-resident columns | Test/diagnostic only (`cpu_*` in tests) |
| **ChokeReadout** | Same BH-0 dispatch when `choke_output_col` set | LIVE_API | GPU-resident choke column | Test/diagnostic only |
| **ChokeThresholdConsumer** | `simthing-gpu/saturating_flux_choke_threshold.rs` | LIVE_API | **Compact only** (4 floats) | Test/diagnostic only |
| **WImpedanceComposeOp** | `simthing-gpu/w_impedance_compose.rs`, `simthing-driver/w_impedance_compose_bridge.rs` | LIVE_API | GPU-resident W profiles | `cpu_w_impedance_compose_oracle` — tests only |
| **StressComposeOp** | `simthing-gpu/stress_compose.rs`, `simthing-driver/stress_compose_bridge.rs` | LIVE_API | GPU-resident stress cols | `cpu_stress_compose_oracle` — tests only |
| **FIELD_POLICY feedstock** | `simthing-spec` admission (`WImpedanceComposeSpec`, `StressComposeSpec`, `RegionFieldOperatorSpec::SaturatingFlux`) | LIVE_API | N/A (compile/admission) | Admission preview only |
| **Composed W → PALMA bridge** | `simthing-driver/w_impedance_compose_bridge.rs` `composed_w_min_plus_stencil_config` | LIVE_API | GPU-resident W; **compact D probe** for downstream | PALMA test oracles only |
| **Resident D compact probe** | `simthing-gpu` min-plus traversal (PATH track) | LIVE_API | **Compact probe readback** | Diagnostic / test parity only |
| **Fast R1* sentinels** | `tests/runtime_0080_0_r1_gate.rs` + `r1_fast_*` / `r1c_fast_*` unit tests | TEST_ONLY (default gate) | None (no reports/checksums) | Tiny fixture oracles on live helpers |

Production `simthing-driver/src/` does **not** import CT-4b fixtures, observation runners, or CPU decision oracles.

---

## 3. Explicit non-surfaces

These are **not** production features in this track:

- border service
- frontline service
- pathfinding engine
- movement engine
- route object
- predecessor table
- fleet AI
- semantic WGSL
- faction-specific runtime behavior

**Movement language:** BH-2D-OBS-100R observed **probe-implied movement-front tendencies** and
test-only candidate sampler displacement in the 100-tick fixture. **Local automata movement is not
implemented** as a production rung.

---

## 4. Candidate F transient-constitution check

| Check | Result |
|---|---|
| Candidate F rule in `docs/design_0_0_8_1.md` §0.7 | ✓ Present |
| Artifact chain moved into `docs/simthing_core_design.md` | ✓ **Not moved** (principle-level doc unchanged) |
| Native sqrt-like ops in authoritative decision gates | ✓ Forbidden; diagnostic-only |
| Parity-sensitive paths route through Candidate F or admitted exact primitive | ✓ Stated in §0.7 |

**Exact-authoritative chain (preserve):**

```text
fixed-point dx/dy
→ exact pre-sqrt mag2 (`m_jit_mag2_fixed_exact` / `ExactFixedPointDxDy`)
→ Candidate F sqrt (`m_jit_mag_f_from_exact_mag2`, artifact hash `59ab4b2892e3c690`, LF-canonical re-pin 2026-06-11, `SQRT-REPIN-0`)
→ exact Euclidean magnitude
→ threshold
```

BH-0…BH-2S production hot paths use linear arithmetic, `clamp`, products, and abs — no native sqrt.
PALMA-adjacent paths follow PATH-track Candidate-F discipline.

---

## 5. R1* cleanup check

| Check | Result |
|---|---|
| R1* proof-ledger/report/checksum tests in default workspace | ✓ Removed (R1-TEST-PURGE) |
| Fast production-relevant R1* sentinels retained | ✓ `runtime_0080_0_r1_gate` + unit oracles |
| Historical R1* proof reports | ✓ Archived under `docs/archive/superseded_tests/` only |
| `*_report_checksum_stable` in **R1\*** default gate | ✓ None (r1a/r1b/r1c/r1c_a..f checksum binaries deleted; `r1_gate` sentinel has none) |
| Workspace R1* 60-second warnings | ✓ Eliminated (~95s workspace; zero R1* 60s warnings post-purge) |

**Discipline:** Historical R1* proof batteries must not be reintroduced as default tests.

> **Fable-review correction (2026-06-12):** the checksum-purge claim is scoped to the **R1\***
> series. Eight `*_report_checksum_stable` tests remain in **non-R1** 0080-series binaries
> (`runtime_0080_0_r0`, `runtime_0080_0_r2`, `runtime_0080_rr_0..4`, `gpu_measure_0080_0`) and run
> in the default workspace; they are CPU-deterministic, pre-existing, and were never in
> R1-TEST-PURGE scope. Trimming them is an optional future hygiene rung, not a 0.0.8.1 blocker.
> See [`fable_review_0_0_8_1_result.md`](fable_review_0_0_8_1_result.md) §6/§8.

Sentinel inventory: [`r1_default_workspace_purge_results.md`](r1_default_workspace_purge_results.md).
Prior R1C-B/C-only note (superseded): [`r1c_default_gate_cleanup_results.md`](../archive/superseded_tests/r1c_default_gate_cleanup_results.md).

---

## 6. Canonical evidence (keep)

| Artifact | Role |
|---|---|
| [`bh2d_ct4b_100tick_scenario_observations.md`](bh2d_ct4b_100tick_scenario_observations.md) | **Canonical** dynamic 100-tick observation |
| **This packet** | **Canonical** Fable review handoff |
| [`r1_default_workspace_purge_results.md`](r1_default_workspace_purge_results.md) | **Canonical** R1 default-gate posture |
| [`bh0_saturating_flux_results.md`](bh0_saturating_flux_results.md) … [`bh2c_palma_feedstock_results.md`](bh2c_palma_feedstock_results.md) | BH rung closure (referenced by production docs) |
| [`bh2d_ct4b_fixture_results.md`](bh2d_ct4b_fixture_results.md) | BH-2D binary fixture proof |
| [`design_0_0_8_1_border_hack_track.md`](../design_0_0_8_1_border_hack_track.md) | Track ledger + API surfaces |

**Superseded / archived (not default-gate evidence):**

| Artifact | Disposition |
|---|---|
| [`r1c_default_gate_cleanup_results.md`](../archive/superseded_tests/r1c_default_gate_cleanup_results.md) | Superseded by full R1 purge report; archived under `docs/archive/superseded_tests/` |
| [`runtime_0080_0_r1a_next_tick_authority_results.md`](runtime_0080_0_r1a_next_tick_authority_results.md) | Historical R1a closure; proof integration tests deleted |
| `docs/archive/superseded_tests/runtime_0080_0_r1*` | Historical proof markdown only |
| `phase_m_jit_sqrt_exact5f_exhaustive_batches.log` | **Deleted** (scratch log; Phase-M sweep retained as `.md`) |

**Regression harness (TEST_ONLY, not deleted):** `ct4b_field_fixture`, `bh2d_ct4b_*`, `bh2c_palma_w_feedstock`, BH GPU test suites.

---

## 7. Constitutional checklist

| Requirement | Status |
|---|---|
| No border / frontline service | ✓ |
| No pathfinding / movement engine | ✓ |
| No route / predecessor table | ✓ |
| No CPU planner for production decisions | ✓ |
| No semantic WGSL | ✓ |
| No faction-specific production code | ✓ |
| No full-field CPU readback for production decisions | ✓ |
| Resource-flow spine intact | ✓ |
| FIELD_POLICY threshold doctrine intact | ✓ |

---

## 8. Freeze validation (this PR)

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | Docs-only freeze pass |
| `cargo test --workspace` | Not rerun | No test/harness/runtime changes |

---

## Reviewer notes

1. Say **probe-implied movement-front** or **test-only candidate displacement** — not "movement implemented."
2. `CT-4b_Local_Automata_W_Feedstock` is a named consumer in docs/tests; production uses generic field columns.
3. FIELD_POLICY feedstock is available; production movement policy is a **future consumer rung**, not shipped here.
4. Do not reopen R1* proof batteries or checksum/report replay tests in default workspace.
