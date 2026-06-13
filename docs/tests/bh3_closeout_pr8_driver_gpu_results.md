# BH3 Closeout PR8 Driver/GPU Sample Results

> **Artifact lifecycle: PROBATION** (fresh PR8 driver admit/install + GPU-resident exercise proof;
> requires Design Authority review before merge; review for promotion, archive, or deletion by PR 9).

> **DA REVIEW REQUIRED BEFORE MERGE**

## Verdict

**PASS (pending CI driver test execution)** — canonical sample `ct_bh3_closeout_sample` admits through
existing generic driver install surfaces, honors default-off posture, and exercises bounded
GPU-resident SaturatingFlux + commitment + PALMA W/D paths with compact evidence only. No new
`simthing-sim` semantics, GPU kernels, movement, pathfinding, routes, predecessors, border, or
frontline services were added.

Local agent environment could not spawn `simthing-driver` integration test binaries (Windows os error
740 — elevation required for all driver test executables linking the GPU stack). Compile verified via
`cargo check -p simthing-driver --tests`; runtime gate deferred to CI for
`cargo test -p simthing-driver --test ct_bh3_closeout_sample_install`.

## Files changed

| Area | Path |
|---|---|
| Driver closeout tests | `crates/simthing-driver/tests/ct_bh3_closeout_sample_install.rs` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| Border hack track | `docs/design_0_0_8_1_border_hack_track.md` |
| PALMA integration guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| PR8 result report | `docs/tests/bh3_closeout_pr8_driver_gpu_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_closeout_pr2..pr7_*` | PROBATION | Retained; not superseded by PR8 |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs` canonical sample tests | LIVE_GUARDRAIL | Unchanged (45 tests) |
| `ct_bh3_closeout_sample_install.rs` | LIVE_GUARDRAIL | New fast driver closeout guardrail |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None. PR2–PR7 per-PR reports remain PROBATION evidence for PR9 promotion review.

## Admit/install shape

Chain exercised:

```text
ct_bh3_closeout_sample.clause
→ parse_raw_document / hydrate_scenario
→ HydratedScenarioPack + Scenario + GameModeSpec
→ compile_region_field_preview (SaturatingFlux + commitment)
→ compile_w_impedance_compose_preview from PALMA feedstock DTO (test bridge)
→ install_atomic (CPU install path)
→ SimSession::open_from_spec when GPU present (mapping None under Disabled)
```

## Installed surfaces

| Surface | Evidence |
|---|---|
| SaturatingFlux region field | `compile_region_field_preview` admits choke flux operator |
| PALMA W/D feedstock | `HydratedScenarioPalmaFeedstock` → W compose admission + min-plus stencil |
| FIELD_POLICY commitment | `FirstSliceCommitmentSpec` on region field + reduction/parent_formula |
| Default-off posture | `MappingExecutionProfile::Disabled`; session `mapping.is_none()` |
| Root scenario + locations | 3 location children + 2 N4 links preserved through install |
| Bounded grid link metadata | `pack.grid_metadata.links.len() == 2` |

## GPU-resident exercise description

**Test A** (`closeout_sample_admits_installs_and_honors_default_off`): CPU-first admit/install;
optional GPU session open confirms default-off wiring.

**Test B** (`closeout_sample_gpu_resident_path_exercises_compact_evidence`): GPU-gated; skips cleanly
when adapter unavailable. Explicit test-only opt-in via
`FirstSliceMappingSession::open(..., SparseRegionFieldV1, ...)` — canonical sample remains
default-off. Exercises:

1. Seeded first-slice mapping tick + commitment fixture scan
2. Compact diagnostic EML readback (`diagnostic_readback_reduction_eml`) crossing threshold 0.75
3. One journaled `threshold_events` row (event_kind 7)
4. BH-2C chain: `WImpedanceComposeOp` → `MinPlusTraversalFieldOp` (GpuResident) →
   `MinPlusTraversalDProbeOp` compact D probe at one cell

No full-field CPU decision readback. No canonical sample mutation to `enabled = true`.

## Compact evidence readout

- Mapping report: `reduction_stencil_readbacks == 0`, no full field values in report
- Commitment: finite threat/urgency scalars; `urgency > 0.75`; exactly one threshold event
- PALMA: one gathered D value + finite `min_d` from compact probe buffer
- Forbidden-token grep on `w_impedance_compose_bridge.rs` (no sqrt/distance/pathfinding vocabulary)

## Tests run

| Command | Result |
|---|---|
| `cargo test -p simthing-clausething --test ct_scenario_container` | 45 passed |
| `cargo check -p simthing-driver --tests` | pass (includes new test binary) |
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_install` | **not executed locally** (Windows os error 740 spawning driver test binary) |
| `cargo fmt --all -- --check` | pass |
| `git diff --check` | pass |

## GPU availability / skip status

- Test A runs without GPU (CPU install path); optional session check gated on adapter presence
- Test B uses `GpuContext::new_blocking()` early return with stderr skip message when adapter unavailable
- Local agent: driver test binary spawn blocked (elevation); GPU runtime not verified in-agent

## Docs updated

- `docs/design_0_0_8_2_clausething_closeout_ladder.md` — PR8 status + census row
- `docs/design_0_0_8_1_clausething_production_track.md` — PR8 addendum
- `docs/design_0_0_8_1_border_hack_track.md` — PR8 addendum
- `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` — PR8 addendum
- `docs/clausething/ClauseThing_Spec.md` — §3.5 driver closure note

## DA review status

**REQUIRED BEFORE MERGE.** DA must confirm: no new `simthing-sim` semantics; no new runtime noun
engine; no new GPU kernels; no CPU planner logic; no full-field CPU decision readback; no
movement/pathfinding/route/predecessor/border/frontline semantics; PALMA remains W/D feedstock;
SaturatingFlux remains field math; FIELD_POLICY remains threshold feedstock; Candidate F authority
intact; proof/test lifecycle regime followed.

## Remaining risks

- PR9 battery consolidation and PROBATION artifact promotion still pending
- Canonical sample has no authored `field_impedance` block; PR8 test derives W compose admission from
  PALMA feedstock column metadata (test-only bridge, not production API)
- Session-loop mapping for live play still requires explicit profile + pressure_binding beyond this
  closeout sample shape

## Lifecycle classification for new artifacts

| Artifact | Classification |
|---|---|
| `docs/tests/bh3_closeout_pr8_driver_gpu_results.md` | PROBATION |
| `crates/simthing-driver/tests/ct_bh3_closeout_sample_install.rs` | LIVE_GUARDRAIL |
