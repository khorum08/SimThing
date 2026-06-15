# BH3 Closeout PR8 Driver/GPU Sample Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR8 driver admit/install + GPU exercise proof; folded
> into [`clausething_closeout_results.md`](clausething_closeout_results.md)). The driver test
> guardrail is LIVE_GUARDRAIL.

> **DA REVIEW: APPROVED (2026-06-13, Opus / Design Authority).** Driver test executed and passed on a
> real GPU adapter after two test-only bugs were found and fixed during DA review (see "DA review
> findings" below).

## Verdict

**PASS — driver test executed successfully.** Canonical sample `ct_bh3_closeout_sample` admits through
existing generic driver install surfaces, honors default-off posture, and exercises bounded
GPU-resident SaturatingFlux + commitment + PALMA W/D paths with compact evidence only. No new
`simthing-sim` semantics, GPU kernels, movement, pathfinding, routes, predecessors, border, or
frontline services were added.

`cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver` → **2 passed; 0 failed** on a
machine with a real GPU adapter. The Windows `os error 740` seen during initial PR8 implementation was a UAC
installer-detection heuristic triggered by the former `..._install` binary name (fixed in PR8-WIN-HYGIENE by
renaming to `ct_bh3_closeout_sample_driver`). The test was **not** merged on a conditional/pending basis —
it ran and passed before approval.

## DA review findings (remediation applied)

DA refused to accept the original "PASS (pending CI)" verdict and ran the blocked test. It **failed**,
confirming the reviewer's concern. Two test-only defects were found and fixed in this PR (no production
code changed):

1. **W-compose column aliasing.** The test bridge set `choke_b_col = choke_a + 1`, which collided with
   the PALMA `w_output_col`. Fixed via `spare_choke_b_col`, choosing the smallest column not claimed by
   source / authored choke / W / D outputs as the generic operator's null second choke input.
2. **Duplicate property registration.** `scenario_from_pack` pre-registered the game-mode properties
   into the base scenario registry, then `install_atomic` registered them again → `DuplicateProperty`.
   Fixed by following the `open_from_spec` convention (placeholder-only base registry; install registers
   spec properties).

## Files changed

| Area | Path |
|---|---|
| Driver closeout tests | `crates/simthing-driver/tests/ct_bh3_closeout_sample_driver.rs` |
| Closeout ladder | `docs/design_0_0_8_2_clausething_closeout_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| Border hack track | `docs/design_0_0_8_1_border_hack_track.md` |
| PALMA integration guide | `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` |
| ClauseThing spec | `docs/clausething/ClauseThing_Spec.md` |
| PR8 result report | `docs/tests/bh3_closeout_pr8_driver_gpu_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/bh3_closeout_pr2..pr6_*` | ARCHIVE | Moved to `docs/archive/superseded_tests/` by PR9 |
| `docs/tests/bh3_closeout_pr7_sample_import_results.md` | CURRENT_EVIDENCE | Promoted by PR9 for PR10 citation |
| Fable/BH2 review packets | CURRENT_EVIDENCE | Retained |
| `ct_scenario_container.rs` canonical sample tests | LIVE_GUARDRAIL | Unchanged (45 tests) |
| `ct_bh3_closeout_sample_driver.rs` | LIVE_GUARDRAIL | New fast driver closeout guardrail |
| Scratch logs / duplicate reports / `target/` / worktrees | DELETE | None found |

## Deleted/superseded artifacts

None in PR8. PR9 archived PR2–PR6 per-PR reports and PR8-WIN-HYGIENE hygiene note (see
`bh3_closeout_pr9_test_battery_results.md`).

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
| `cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver` | **2 passed; 0 failed** (real GPU adapter; PR8-WIN-HYGIENE renamed binary — no UAC workaround required) |
| `cargo check -p simthing-driver --tests` | pass |
| `cargo fmt --all -- --check` | pass |
| `git diff --check` | pass |

## GPU availability / skip status

- Test A admits/installs via the CPU `install_atomic` path; the optional `open_from_spec` session check
  is gated on adapter presence
- Test B uses `GpuContext::new_blocking()` early return with a stderr skip message when no adapter exists
- DA review machine had a real GPU adapter: Test B's GPU-resident SaturatingFlux + commitment + PALMA
  min-plus + compact D probe all executed (not skipped)

## Docs updated

- `docs/design_0_0_8_2_clausething_closeout_ladder.md` — PR8 status + census row
- `docs/design_0_0_8_1_clausething_production_track.md` — PR8 addendum
- `docs/design_0_0_8_1_border_hack_track.md` — PR8 addendum
- `docs/design_0_0_8_1_palma_pathfinding_integration_guide.md` — PR8 addendum
- `docs/clausething/ClauseThing_Spec.md` — §3.5 driver closure note

## DA review status

**APPROVED (2026-06-13, Opus / Design Authority).** DA verified the full change scope is one test file
plus docs — no `simthing-sim`, `simthing-gpu` kernel, or driver `src/` changes. Checklist confirmed:

1. No new `simthing-sim` semantics — change scope excludes the crate entirely
2. No new runtime noun engine
3. No new GPU kernels — reuses `WImpedanceComposeOp`, `MinPlusTraversalFieldOp`,
   `MinPlusTraversalDProbeOp`, `FirstSliceMappingSession`
4. No CPU planner logic
5. No full-field CPU decision readback — asserts `field_values.is_none()`,
   `reduction_parent_value.is_none()`, `eml_output.is_none()`, traversal `values.is_none()`; only a
   compact EML scalar, one threshold-event row, and one compact D-probe value are read
6. No movement/pathfinding/route/predecessor/border/frontline semantics (forbidden-token grep enforced)
7. PALMA remains W/D feedstock
8. SaturatingFlux remains generic field math
9. FIELD_POLICY remains threshold/commitment feedstock
10. Candidate F untouched — min-plus is additive cost relaxation, not Euclidean magnitude; no
    sqrt/length/distance/normalize/hypot in the exercised bridge
11. Proof/test lifecycle regime followed
12. The driver test actually runs and passes (2 passed; 0 failed)

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
| `crates/simthing-driver/tests/ct_bh3_closeout_sample_driver.rs` | LIVE_GUARDRAIL |
