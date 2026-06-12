# Fable review packet — BH / BH-2 track (0.0.8.1)

> **Status: FABLE-PREP / PASS (2026-06-11).** Track closed through BH-2D-OBS-100R; R1C-GATE cleanup
> complete; Candidate F §0.7 elevated; full workspace gate passed. Ready for Fable review.

---

## A. Track closure summary

The Border Hack (`BH-`) track implements a semantic-free GPU saturating-flux stencil operator and
downstream field-composition feedstock for PALMA min-plus traversal. Borders and choke topology are
**field readouts only** — no border service, frontline service, pathfinding engine, movement engine,
route objects, or predecessor tables.

| Rung | What landed |
|---|---|
| **BH-0** | Generic GPU `SaturatingFlux` stencil (register-transient C, symmetric flux, zero-flux boundaries) |
| **BH-1** | Optional GPU-resident choke readout column `1 − C/χ` in same dispatch |
| **BH-1R** | Compact `SaturatingFluxChokeThresholdOp` (4-float readback) |
| **BH-1R-SCALE** | Staged parallel GPU reduction (256-thread pass 1 + partial fold pass 2) |
| **BH-2A** | Named consumer contract `CT-4b_Local_Automata_W_Feedstock` opens BH-2 |
| **BH-2B** | `WImpedanceComposeOp` — linear W from choke columns per admitted profile |
| **BH-2S** | `StressComposeOp` — overlap/mismatch/weighted/velocity stress algebra |
| **BH-2S-API-DOC** | Consumer service-surface documentation (§11 border hack track) |
| **BH-2C** | Composed W → PALMA `GpuInterleavedW` → resident D + compact probe |
| **BH-2D** | CT-4b 200×200 fixture proof over full resident feedstock chain |
| **BH-2D-OBS-100R** | Dynamic 100-tick observation report (test-only stimulus) |

**Explicitly deferred / not implemented:**

- **BH-3** — ClauseThing authoring (consumer-pulled)
- Production movement policy, pathfinding engine, route/predecessor objects
- Border/frontline service or semantic WGSL
- Full-field CPU readback for production decisions
- Local automata **movement** as a production rung (probe-implied tendencies observed in tests only)

---

## B. Specified vs Implemented ledger

| Rung | Specified | Implemented | Evidence | Deferred / Not implemented |
|---|---|---|---|---|
| BH-0 | SaturatingFlux operator + CPU oracle parity | Yes | [`bh0_saturating_flux_results.md`](bh0_saturating_flux_results.md) | — |
| BH-1 | Choke readout column in flux dispatch | Yes | [`bh1_choke_readout_results.md`](bh1_choke_readout_results.md) | — |
| BH-1R | Compact choke threshold consumer | Yes | [`bh1r_choke_consumption_results.md`](bh1r_choke_consumption_results.md) | — |
| BH-1R-SCALE | Staged parallel GPU reduction | Yes | [`bh1r_scale_parallel_reduction_results.md`](bh1r_scale_parallel_reduction_results.md) | — |
| BH-2A | Named consumer opens BH-2 | Yes | [`design_0_0_8_1_border_hack_track.md`](../design_0_0_8_1_border_hack_track.md) §9 | — |
| BH-2B | W composition kernel + bridge | Yes | [`bh2_w_composition_results.md`](bh2_w_composition_results.md) | — |
| BH-2S | Overlap/mismatch/velocity stress | Yes | [`bh2s_overlap_stress_results.md`](bh2s_overlap_stress_results.md) | — |
| BH-2S-API-DOC | Service surface docs | Yes | border hack track §11 | — |
| BH-2C | Composed W feeds PALMA resident D | Yes | [`bh2c_palma_feedstock_results.md`](bh2c_palma_feedstock_results.md) | — |
| BH-2D | CT-4b 200×200 fixture proof | Yes | [`bh2d_ct4b_fixture_results.md`](bh2d_ct4b_fixture_results.md) | — |
| BH-2D-OBS-100R | Dynamic 100-tick observation | Yes | [`bh2d_ct4b_100tick_scenario_observations.md`](bh2d_ct4b_100tick_scenario_observations.md) | — |
| BH-3 | ClauseThing authoring | No | — | Consumer-pulled; not opened |
| Movement policy | Local automata fleet AI | No | OBS report probe-implied only | Future consumer work |
| Pathfinding engine | Route planning service | No | — | Not in scope |
| Border service | Frontline/border objects | No | — | Explicitly forbidden |

---

## C. Production surfaces

| Surface | Owning crate / file | Input → output (semantic-free) | CPU oracle | Readback |
|---|---|---|---|---|
| **SaturatingFlux** | `simthing-gpu` structured field stencil + `saturating_flux` WGSL | Interleaved pressure column → evolved pressure (+ optional choke col) | Yes (`cpu_*` in tests) | GPU-resident; test oracle only |
| **ChokeReadout** | Same dispatch as BH-0 when `choke_output_col` set | Pressure → `1 − C/χ` column | Yes | GPU-resident column |
| **ChokeThresholdConsumer** | `simthing-gpu/saturating_flux_choke_threshold.rs` | Choke column → 4-float compact threshold readback | Yes (tests) | **Compact only** (4 floats) |
| **WComposition** | `simthing-gpu/w_impedance_compose.rs` + `simthing-driver/w_impedance_compose_bridge.rs` | `base_w`, `choke_a/b` → `output_w` profiles | Yes (`cpu_w_impedance_compose_oracle`, tests) | GPU-resident |
| **OverlapStressComposition** | `simthing-gpu/stress_compose.rs` + `simthing-driver/stress_compose_bridge.rs` | Choke/stress cols → stress profile cols | Yes (`cpu_stress_compose_oracle`, tests) | GPU-resident |
| **FIELD_POLICY feedstock** | Admission via `simthing-spec` (`WImpedanceComposeSpec`, `StressComposeSpec`, `RegionFieldOperatorSpec::SaturatingFlux`) | Spec → compiled GPU config | Admission preview only | N/A |
| **Composed W → PALMA bridge** | `simthing-driver/w_impedance_compose_bridge.rs` `composed_w_min_plus_stencil_config` | Composed W col on interleaved buffer → PALMA stencil | No (PALMA has own oracle in tests) | GPU-resident W/D; **compact D probe** for consumers |
| **PALMA resident traversal/probe** | `simthing-gpu` min-plus traversal (pre-existing PATH track) | `GpuInterleavedW` → resident D | Diagnostic only | **Compact probe readback** |

Production driver `src/` does **not** import CT-4b fixtures, observation runners, or CPU oracles.

---

## D. Test/proof scaffolding inventory

| Name | Path | Classification | Why it remains | Not production because |
|---|---|---|---|---|
| `Ct4bFixture` | `crates/simthing-driver/tests/support/ct4b_field_fixture.rs` | TEST_ONLY | BH-2D/BH-2D-OBS fixture shape | Never linked from `simthing-driver/src` |
| `ct4b_100tick_runner` | `crates/simthing-driver/tests/support/ct4b_100tick_runner.rs` | TEST_ONLY | 100-tick observation pass | Ignored integration test only |
| `DynamicObsState` | same file | TEST_ONLY | Mobile emitters + candidate displacement | Test-only dynamic stimulus |
| `source_pulse` / mobile emitter schedule | same file | TEST_ONLY | Deterministic observation dynamics | Not in production tick path |
| `render_observation_markdown` | same file | DIAGNOSTIC_ONLY | Writes `docs/tests` report | Report generation only |
| `readback_buffer` | BH-2D/OBS test modules | DIAGNOSTIC_ONLY | Test aggregate metrics / parity | Full-field readback quarantined to tests |
| `cpu_w_impedance_compose_oracle` | `simthing-gpu` (exported) | TEST_ONLY | BH-2B GPU/CPU parity | Used only from tests |
| `cpu_stress_compose_oracle` | `simthing-gpu` (exported) | TEST_ONLY | BH-2S GPU/CPU parity | Used only from tests |
| `cpu_min_plus_*` / PALMA oracles | test `support/palma_min_plus_oracle.rs` | TEST_ONLY | BH-2C/D probe parity | Test support only |
| `bh2d_ct4b_fixture.rs` | driver tests | TEST_ONLY | BH-2D proof suite | Integration test only |
| `bh2c_palma_w_feedstock.rs` | driver tests | TEST_ONLY | BH-2C proof suite | Integration test only |
| `bh2d_ct4b_100tick_observation.rs` | driver tests | TEST_ONLY | OBS-100R smoke + ignored full run | Integration test only |
| BH GPU test suites | `simthing-gpu/tests/bh*.rs` | TEST_ONLY | Per-rung regression gates | Not production binaries |
| BH admission tests | `simthing-spec/tests/bh*.rs` | TEST_ONLY | Admission validation | Spec compile tests only |
| Forbidden vocab lists (`Terran`, `Pirate`) | BH-2C/D test files | TEST_ONLY | Negative-control scans | Lists forbidden terms; production code does not use them |
| Canonical result reports | `docs/tests/bh*_*.md` | DIAGNOSTIC_ONLY | Rung closure evidence | Documentation only |
| `bh2d_ct4b_100tick_scenario_observations.md` | `docs/tests/` | DIAGNOSTIC_ONLY | Canonical dynamic observation | Generated by ignored test |
| R1C-B/C `OnceLock` proof wrappers (removed) | former `runtime_0080_0_r1c_b/c.rs` | **DELETE_PROOF_SCAFFOLD** | Bogged default workspace (60s+ per binary) | Superseded by gate sentinels + archive |
| `runtime_0080_0_r1c_gate.rs` | driver tests | **KEEP_FAST_SENTINEL** | Default opt-in + no-compaction sentinels | No GPU/report/checksum |
| `r1c_fast_allocation_selects_one_compatible_marked_slot` | `runtime_0080_0_r1c_b.rs` unit test | **KEEP_FAST_SENTINEL** | Lowest-mark oracle on tiny fixture | No GPU/report/checksum |
| `r1c_fast_membership_delta_applies_to_one_slot` | `runtime_0080_0_r1c_c.rs` unit test | **KEEP_FAST_SENTINEL** | Bounded delta oracle on tiny fixture | No GPU/report/checksum |
| R1C archived proof reports | `docs/archive/superseded_tests/runtime_0080_0_r1c_*` | DIAGNOSTIC_ONLY | Historical rung closure | Not default gate |
| Candidate F §0.7 | `design_0_0_8_1.md` §0.7 | **LIVE_API** | Transient constitution exact-sqrt rule | Artifact hash pinned; not in core design |

**R1C-GATE note:** Legacy R1C-B/C proof/report replay tests were removed from the default workspace gate.
Default workspace retains only fast production-relevant allocation/membership sentinels. Candidate F
exact-sqrt authority elevated into 0.0.8.1 §0.7. BH track independent — see
[`r1c_default_gate_cleanup_results.md`](r1c_default_gate_cleanup_results.md).

**Ambiguity check:** No production module calls `Ct4bFixture`, `run_observation_ticks`, or dynamic pulse schedules. Bridges are numeric column plumbing only.

---

## E. Candidate-F / native-sqrt audit

**Rule:** GPU-resident sqrt/magnitude/distance/norm paths must use `m_jit_sqrt_f_exact`. Native
`sqrt`, `length`, `distance`, `normalize`, `hypot`, magnitude, or norm forbidden in authoritative
BH/BH-2/PALMA-adjacent production paths.

**Audit result: PASS** (R1C-GATE cleanup does not touch BH/PALMA hot paths; Candidate F rule now in 0.0.8.1 §0.7)

- BH-0…BH-2S WGSL and Rust hot paths use linear arithmetic, `clamp`, products, and abs — no native sqrt.
- BH-2B/BH-2S tests include forbidden-token scans (`bh2_w_composition.rs`, `bh2s_overlap_stress.rs`, etc.).
- BH-2D/OBS tests scan hot-path strings for forbidden sqrt-like tokens.
- BH-2D-OBS-100R uses scalar W, D probes, choke, stress, Manhattan displacement — no sqrt introduced.
- PALMA min-plus traversal (BH-2C consumer) follows existing PATH-track Candidate-F discipline; constitutional rule now in 0.0.8.1 §0.7.

No native sqrt-like operation found in authoritative BH production paths requiring remediation.

---

## F. Constitutional checklist

| Requirement | Status |
|---|---|
| No border service | ✓ |
| No frontline service | ✓ |
| No pathfinding engine | ✓ |
| No movement engine | ✓ |
| No route object | ✓ |
| No predecessor table | ✓ |
| No CPU planner | ✓ |
| No semantic WGSL | ✓ |
| No faction-specific production code | ✓ |
| No full-field CPU readback for production decisions | ✓ |
| No `simthing-sim` semantic change from BH track | ✓ |
| No ClauseThing runtime change from BH track | ✓ |
| Resource-flow spine intact | ✓ |
| FIELD_POLICY threshold doctrine intact | ✓ |

---

## G. 100-tick scenario observations (canonical)

Reference: [`bh2d_ct4b_100tick_scenario_observations.md`](bh2d_ct4b_100tick_scenario_observations.md)

**Observed (live GPU, BH-2D-OBS-100R):**

- **Shifting pressure:** max `choke_a` 0.633 → 1.0; overlap 0.60 → 0.95; velocity stress active through tick 99 (0.976).
- **W-profile divergence:** anchor W profile0 1.99→2.43, profile1 7.73→10.43; D probes profile0 32.5→33.2, profile1 42.0→46.5.
- **Probe-implied movement-front:** profile-1 anchor neighbor rank 1→3; candidate mean displacement 0.03→1.12 Manhattan cells.
- **Not production movement:** candidate sampler displacement is test-only scaffolding; no movement policy implemented.

---

## H. Test runs

### Focused sanity gates (FABLE-PREP)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p simthing-driver --test bh2d_ct4b_100tick_observation -- bh2d_ct4b_100tick_observation_smoke` | PASS |
| `cargo test -p simthing-driver --test bh2d_ct4b_fixture` | PASS |
| `cargo test -p simthing-driver --test bh2c_palma_w_feedstock` | PASS |
| `cargo test -p simthing-gpu --test bh2_w_composition` | PASS |
| `cargo test -p simthing-gpu --test bh2s_overlap_stress` | PASS |

### Full workspace gate

| Command | Result | Duration |
|---|---|---|
| `cargo test --workspace` | PASS | see commit CI / local run |

*(Duration and exact test count recorded at PR merge time from the FABLE-PREP agent run.)*

---

## Cleanup actions (this rung)

| Item | Action |
|---|---|
| Stale "BH-2C/BH-2D deferred" in `bh2_w_composition_results.md` | Updated — rungs now closed |
| `bh2d_ct4b_fixture_results.md` | Kept — binary proof gates; OBS narrative superseded by 100-tick report |
| `bh2d_ct4b_100tick_scenario_observations.md` | Kept — canonical dynamic observation |
| BH result reports (`bh0`…`bh2c`) | Kept — referenced by production docs status ledger |
| CT-4b / OBS test scaffolding | Quarantined TEST_ONLY — not deleted (still required for regression) |
| `target/`, `.claude/worktrees/` | Not committed (gitignored) |
| `phase_m_jit_sqrt_exact5f_exhaustive_batches.log` | Out of BH scope — retained for Phase-M sqrt audit trail |

---

## Reviewer notes for Fable

1. **Movement language:** Say "probe-implied movement-front tendencies" or "test-only candidate sampler displacement" — not "movement implemented."
2. **Consumer contract:** `CT-4b_Local_Automata_W_Feedstock` is a docs/tests named consumer; production uses generic `field_a` / `field_b` columns.
3. **Next consumer work:** FIELD_POLICY feedstock surfaces are available; production movement policy remains a future consumer rung (BH-3 or CT vertical), not shipped here.
