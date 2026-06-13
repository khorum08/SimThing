# Fable review result — 0.0.8.1 / BH-2 closure

> Independent review gate executed 2026-06-12 by the Fable executive design authority over the
> FABLE-REVIEW-FREEZE packet ([`fable_review_bh2_track_packet.md`](fable_review_bh2_track_packet.md)).
> Docs + evidence + targeted code spot-checks; no runtime edits.

## 1. Verdict

```
ACCEPT_WITH_NOTES
```

The BH/BH-2 closure is constitutionally clean and the production spine is coherent. One
packet-accuracy defect (§5 checksum-scope wording) is corrected in this pass; it tripped no stop
condition. Proceed after the minor doc correction already applied here.

## 2. Scope reviewed

- **Docs:** the packet; `design_0_0_8_1.md` §0.7; `design_0_0_8_1_border_hack_track.md`;
  `design_0_0_8_1_palma_pathfinding_integration_guide.md`; `bh2d_ct4b_100tick_scenario_observations.md`;
  `r1_default_workspace_purge_results.md`; `simthing_core_design.md` (Candidate-F absence check).
- **Code spot-checked:** `simthing-gpu/src/structured_field_stencil.rs`
  (`cpu_saturating_flux_step`, `cpu_compute_c_at`, variant table, CFL admission) and its inline
  `shaders/structured_field_stencil.wgsl` (variant-7 branch); `min_plus_stencil.rs` +
  `shaders/min_plus_stencil.wgsl`; `min_plus_traversal_d_probe.rs`; `w_impedance_compose.rs`,
  `stress_compose.rs`, `saturating_flux_choke_threshold.rs` (sqrt sweep); `simthing-sim/src/*`
  (semantic-noun sweep); the R1/rr/r0/r2 test inventory.
- **Commands run:** `cargo fmt --all -- --check` → **PASS** (exit 0). `cargo test --workspace`
  **not run** (docs/review pass; no runtime/test files touched).

## 3. Constitutional checklist

| Check | Result | Evidence |
|---|---|---|
| No privileged combat/economy/AI/movement/pathfinding subsystem | ✓ | No such crate/module; min-plus is a numeric utility |
| No semantic WGSL | ✓ | Shaders carry `u_sat`/`chi`/`d`/`w` floats; only `candidate_f_magnitude.wgsl` has sqrt (the exact artifact path itself) |
| No border/frontline service | ✓ | Choke is a field column readout; no service/object |
| No movement engine | ✓ | Candidate sampler is test-only stimulus (obs report §1–2) |
| No pathfinding engine | ✓ | min-plus = `D = W + min(N4 D)` relaxation, no traversal planner |
| No route object | ✓ | grep: zero route objects; in-code disclaimers present |
| No predecessor table | ✓ | grep: no predecessor/backtrack/came_from in min-plus |
| No full-field CPU readback for production decisions | ✓ | Compact-only readbacks (choke 4-float; D compact probe); full readback is diagnostic/oracle |
| No faction-specific runtime behavior | ✓ | `Faction`/`Fleet` hits are pre-existing generic `SimThingKind` labels in test fixtures |
| No simthing-sim semantic leakage | ✓ | sim hits are kind labels + "route"=overlay-batch dispatch routing |
| No ClauseThing runtime reopening | ✓ | No CT runtime changes in BH track |
| Universal SimThing loop intact | ✓ | accumulate→reduce→mask→disburse→threshold unchanged |
| FIELD_POLICY threshold/feedstock doctrine intact | ✓ | Threshold scan GPU-side; feedstock columns generic |
| GPU-resident W/D/choke/stress remain numeric fields | ✓ | All are flat f32 columns; no semantic tagging |

## 4. Candidate F check

| Check | Result |
|---|---|
| Candidate F rule in `docs/design_0_0_8_1.md` | ✓ §0.7 "Exact numeric authority for decision gates" |
| In the 0.0.8.1 transient constitution section | ✓ §0.7, declared the carry-forward constitutional rule |
| **Not** moved into `simthing_core_design.md` | ✓ grep for Candidate F / hash / `m_jit_mag_f` in core design → **no matches** |
| Native sqrt-like ops diagnostic-only, may not gate commitments | ✓ §0.7: WGSL `sqrt`/`length`/`distance`/`normalize`/`hypot`/native sqrt are `ApproximateDiagnostic` only |
| GPU-resident sqrt/mag/distance/norm/threshold routes through Candidate F | ✓ §0.7 states it; the only sqrt-bearing shader is the artifact path `candidate_f_magnitude.wgsl` |

Chain verified verbatim in §0.7: fixed-point dx/dy → `m_jit_mag2_fixed_exact`/`ExactFixedPointDxDy`
→ Candidate F `m_jit_mag_f_from_exact_mag2` (hash `59ab4b2892e3c690`, LF-canonical re-pin
2026-06-11, `SQRT-REPIN-0`) → exact Euclidean magnitude → threshold. **No Candidate-F violation
found** — BH-0…BH-2S hot paths use linear arithmetic / `clamp` / products only (sqrt sweep of the
BH/W/stress production files returned nothing).

## 5. Live surface review

| Surface | Class | I/O shape | CPU oracle | Readback | Leakage |
|---|---|---|---|---|---|
| `SaturatingFlux` | LIVE_API | flat f32 field col in→out | yes (`cpu_saturating_flux_step`, bit-exact) | GPU-resident | none |
| `ChokeReadout` | LIVE_API (opt `choke_output_col`) | writes `1−C/χ`∈[0,1] | yes (`cpu_compute_choke_at`) | GPU-resident | none |
| `ChokeThresholdConsumer` | LIVE_API | compact 4-float | yes (test) | **compact only** | none |
| `WImpedanceComposeOp` | LIVE_API | W profile cols | `cpu_w_impedance_compose_oracle` (test) | GPU-resident | none; **no sqrt** |
| `StressComposeOp` | LIVE_API | overlap/mismatch/velocity cols | `cpu_stress_compose_oracle` (test) | GPU-resident | none |
| FIELD_POLICY feedstock | LIVE_API (admission) | spec compile | admission preview | n/a | none |
| Composed W → PALMA `GpuInterleavedW` | LIVE_API | W in → resident D | PALMA test oracles | GPU-resident; compact D probe | none |
| Resident D compact probe | LIVE_API | compact probe | diagnostic/test parity | **compact only** | none |
| Fast R1* sentinels | TEST_ONLY (default gate) | tiny fixture | tiny oracles | none | none |

**Verified separation:** I confirmed the symmetric pairwise flux `((C_i+C_j)*0.5)·(u_j−u_i)` in
**both** the WGSL variant-7 branch and `cpu_saturating_flux_step` (NSEW order, `(1−σ)` product
for `C`, zero-flux boundary via in-bounds guards). The BH-track voiding tripwire (non-symmetric
`C_i`-only weighting) is **not** tripped — conservation-by-antisymmetry holds. The 2-hop diamond
is correctly implicit (each neighbor's `C` reads its own neighbors from the input buffer; no
scratch column, no second pass).

## 6. Test/proof scaffolding review

| Item | Finding |
|---|---|
| CT-4b fixture | TEST_ONLY; not imported by `simthing-driver/src/` |
| 100-tick dynamic observation runner | TEST_ONLY; report header disclaims movement/pathfinding/route/border |
| Candidate sampler displacement | TEST_ONLY stimulus ("move to lowest-D N4 neighbor (test-only)"), **not** production movement |
| CPU oracles | TEST_ONLY (`cpu_*`) |
| R1* proof-ledger/checksum batteries | **Removed** — r1a/r1b/r1c/r1c_a..f checksum binaries deleted; only `runtime_0080_0_r1_gate.rs` (2 fast tests) + `r1_fast_*`/`r1c_fast_*` unit sentinels remain. Verified: zero `*_report_checksum_stable` test fns in any `r1*` test file. |
| Default workspace residue | **NOTE** — 8 `*_report_checksum_stable` tests remain in non-R1 series (`r0`, `r2`, `rr_0..4`, `gpu_measure_0080_0`), 0 `#[ignore]`, CPU-deterministic. **Out of R1-TEST-PURGE scope, pre-existing**, not introduced by this track. |
| Ambiguous proof scaffolding in production paths | None found |

## 7. Evidence quality review

Canonical evidence (keep): the packet; `bh2d_ct4b_100tick_scenario_observations.md`;
`r1_default_workspace_purge_results.md`; the per-rung BH closure reports; the border hack track
ledger; §0.7 in the constitution. All present and internally consistent. Superseded items
(`r1c_default_gate_cleanup_results.md`, archived `runtime_0080_0_r1*` markdown, the deleted
`phase_m_jit_sqrt_exact5f_exhaustive_batches.log`) are already correctly dispositioned in the
packet §6. No stale/duplicate report required deletion this pass; tree is clean.

## 8. Risk register

| Risk | Severity | Evidence | Required follow-up |
|---|---|---|---|
| Packet §5 claims "`*_report_checksum_stable` in default gate: None" — inaccurate in absolute terms | Low | 8 non-R1 checksum tests (`r0`/`r2`/`rr_0..4`/`gpu_measure`) run by default | **Done in this PR**: §5 wording scoped to R1*; residual non-R1 batteries noted as out-of-scope/pre-existing |
| Non-R1 0080-series checksum batteries (r0/r2/rr/gpu_measure) still in default gate | Informational | CPU-deterministic, fast; never in R1 purge scope | Optional future hygiene rung if workspace time matters; **not** a BH/0.0.8.1 blocker |

No constitutional blocker. No Candidate-F violation. No semantic leakage. No
pathfinding/movement/route/predecessor/border service in production paths.

## 9. Recommendation

```
Proceed after minor doc cleanup.
```

The doc cleanup (packet §5 scoping) is applied in this same PR, so the repo is review-clean and
ready for the next production track. The natural next consumer remains BH-3 (ClauseThing
authoring surface for the operator) or a named FIELD_POLICY movement-policy consumer — both
consumer-pulled, neither opened here.

## 10. Stop-condition audit (all clear)

1. Candidate F in transient constitution — present (§0.7). 2. Not moved to core design —
confirmed. 3. R1* proof-ledger default tests — removed (residue is non-R1). 4. Packet vs docs —
one §5 wording defect, corrected. 5. Production depends on scaffolding — no. 6. Semantic runtime
leakage — none. 7. Route/pathfinding/movement/predecessor/border service in production — none.
8. Native sqrt gating decisions outside Candidate F — none. 9. Workspace bog on R1* — N/A
(not run; R1* heavy binaries removed). 10. Live vs test-only distinguishable — yes (packet §2 +
verified). **No PARTIAL trigger.**
