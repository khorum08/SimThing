# MapGen PR4 Resource Flow Enrollment Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (PR4 RF enrollment report; DA-approved after a targeted repair).

## Verdict

**PASS / DA-APPROVED after a targeted DA repair (2026-06-14, Opus / Design Authority)** — PR3 lattice
hierarchy generates bounded Resource Flow enrollment with explicit selector admission, arena caps, deposit
intrinsic-flow feedstock, suppression/disruption arena, expansion report, and no
Movement-Front/PALMA/FIELD_POLICY/hyperlane/runtime leakage. **One DA finding was raised and fixed before
merge** — see "DA review & repair" below.

## DA review & repair (Opus / Design Authority, 2026-06-14)

Genuine pre-merge DA audit against ladder §0/§3/§6 PR4, ADR-RF, ADR-MAP, core §7 — verified independently
against the branch source, not the PR body. RF participation is explicit via `explicit_participants` (the
admission compile `resource_flow_admission.rs` reads **only** `explicit_participants`;
`validate_no_property_possession_admission` is enforced); every arena declares
`max_participants`/`max_coupling_fanout`/`max_orderband_depth`; `FissionPolicy::Reject`;
`ResourceFlowOptInMode::Disabled`; a single `OneTickDelay` coupling (fanout 1 ≤ 4, no all-`Algebraic`
cycle); bounded expansion report; no Movement-Front/SaturatingFlux/PALMA/FIELD_POLICY/hyperlane output
(`assert_no_deferred_pr4_surfaces`); no new `SimThingKind` (`assert_allowed_simthing_kinds`); no Candidate F
(minerals rate parsed from inert metadata, no distance/sqrt). `lib.rs` is pure module wiring; no
driver/GPU/sim files touched.

**Finding (DA SIGN-OFF: REQUEST_CHANGES → fixed before merge).** The deposit arena originally authored
`enrollment = InstallTarget(ScenarioListed { deposits[0] })` while `explicit_participants` listed *every*
deposit — a selector naming only the first deposit, harmless for the single-deposit pentad fixture but a
**latent multi-deposit generalization bug** (the exact ambiguity the handoff flagged for DA attention).
**DA-applied targeted repair:** the deposit arena now enrolls via `EnrollmentSelectorSpec::ExplicitOnly`
(matching the suppression arena and the authoritative participant list), removing the `deposits[0]`
dependency entirely; the test now asserts **both** arenas use `ExplicitOnly`.

**Battery reran green after the repair:** `cargo fmt --check` clean, `git diff --check` clean,
`mapgen_resource_flow` 16, `mapgen_lattice_hierarchy` 10, `mapgen_neutral_ast_parse` 8,
`ct_scenario_container` 45. `mapgen_resource_flow` promoted to **LIVE_GUARDRAIL**. **PR5 may proceed** under
its own DA-review gate — only the Design Authority writes the sign-off (governance rule carried from PR3).

## Track scope

0.0.8.2.5 MapGen PR4: bounded RF enrollment/feedstock from PR3 hierarchy (M3). **Do not merge until
DA review.**

PR4 enrolls RF arena/feedstock only. PR4 does not implement Movement-Front. PR4 does not implement
SaturatingFlux. PR4 does not implement PALMA W/D. PR4 does not implement FIELD_POLICY commitment. PR4
does not implement hyperlane lane coupling. PR4 does not touch runtime/GPU/driver/simthing-sim. PR4 does
not import the whole Stellaris corpus.

## Binding read-order recorded

1. `docs/invariants.md`
2. `docs/simthing_core_design.md` §1.1 and §7
3. `docs/adr/mapping_sparse_regioncell.md`
4. `docs/adr/resource_flow_substrate.md`
5. `docs/design_0_0_8_2_5_mapgen_ladder.md` §0, §3, §6 PR4, §9
6. `docs/clausething/mapgen_corpus_manifest.md`
7. `docs/clausething/MapGenThing.md`
8. `docs/clausething/ct_vertical_consumer_contract.md`
9. `docs/clausething/ct_2c_economic_category_memo.md`
10. `docs/clausething/ct_3b_4a_movement_front_heatmap_memo.md`
11. `docs/tests/mapgen_pr1_corpus_manifest_results.md`
12. `docs/tests/mapgen_pr2_neutral_ast_results.md`
13. `docs/tests/mapgen_pr3_lattice_hierarchy_results.md`
14. `docs/tests/mapgen_pr3_da_audit_results.md`
15. `docs/tests/clausething_closeout_results.md`

## Files changed

| Area | Path |
|---|---|
| RF enrollment generator | `crates/simthing-clausething/src/mapgen_resource_flow.rs` |
| Public exports | `crates/simthing-clausething/src/lib.rs` |
| RF enrollment tests | `crates/simthing-clausething/tests/mapgen_resource_flow.rs` |
| Fixture README | `crates/simthing-clausething/tests/fixtures/mapgen/README.md` |
| MapGen ladder | `docs/design_0_0_8_2_5_mapgen_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGen reference | `docs/clausething/MapGenThing.md` |
| PR4 report | `docs/tests/mapgen_pr4_resource_flow_results.md` |

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr1_corpus_manifest_results.md` | PROBATION | Unchanged |
| `docs/tests/mapgen_pr2_neutral_ast_results.md` | PROBATION | Unchanged |
| `docs/tests/mapgen_pr3_lattice_hierarchy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_pr3_da_audit_results.md` | CURRENT_EVIDENCE | Unchanged |
| `mapgen_neutral_ast_parse.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_lattice_hierarchy.rs` | LIVE_GUARDRAIL | Unchanged |
| `mapgen_resource_flow.rs` | CURRENT_EVIDENCE | New PR4 generator (DA repair applied) |
| `mapgen_resource_flow.rs` (tests) | LIVE_GUARDRAIL | New PR4 tests; promoted at DA approval |
| `ct_scenario_container` | LIVE_GUARDRAIL | Unchanged |
| `docs/tests/mapgen_pr4_resource_flow_results.md` | PROBATION | New PR4 report |
| Scratch logs / duplicates / worktrees | DELETE | None found |

## M3 / RF doctrine preserved

- Deposit minerals authored value → `BaseFlowObligationSpec` produce rate (feedstock only)
- Two bounded arenas: `mapgen_deposit_minerals` (1 participant) + `mapgen_suppression` (5 gridcells)
- Both arenas enroll via `EnrollmentSelectorSpec::ExplicitOnly` over their authoritative `explicit_participants` — multi-deposit-safe (DA repair; deposit arena no longer singles out `deposits[0]`)
- All arenas declare `max_participants`, `max_coupling_fanout`, `max_orderband_depth`
- Shallow coupling deposit → suppression (`OneTickDelay`); fanout capped
- `ResourceFlowOptInMode::Disabled` — no GPU execution in PR4
- Expansion report per arena with caps, enrolled properties, implicit-reject count
- Fixture-local 5 gridcells only — no 200×200 RF participant allocation

## Forbidden surfaces not touched

- No Movement-Front field_operator, PALMA feedstock, FIELD_POLICY commitment, hyperlane links
- No runtime/GPU/driver/simthing-sim changes
- No new `SimThingKind`, no movement/pathfinding/route/predecessor/border/frontline semantics
- No Candidate F Euclidean authority

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test ct_scenario_container
git diff --check
```

## Test results

| Command | Result |
|---|---|
| `mapgen_neutral_ast_parse` | 8 passed |
| `mapgen_lattice_hierarchy` | 10 passed |
| `mapgen_resource_flow` | 16 passed |
| `ct_scenario_container` | 45 passed |

## DA review checklist (verified 2026-06-14, Opus / Design Authority; deposit-selector finding fixed)

- [x] Resource Flow participation is explicit
- [x] Property possession alone does not admit a participant
- [x] Every arena has max_participants
- [x] Every arena has max_coupling_fanout
- [x] Every arena has max_orderband_depth
- [x] Expansion report exists and is bounded
- [x] No deep unbounded fanout
- [x] No runtime/GPU/driver/simthing-sim changes
- [x] No Movement-Front output
- [x] No SaturatingFlux field_operator output
- [x] No PALMA feedstock output
- [x] No FIELD_POLICY commitment output
- [x] No hyperlane lane coupling output
- [x] No pathfinding/movement/route/predecessor/border/frontline semantics
- [x] No new SimThingKind
- [x] No Candidate F implication
- [x] Proof/test lifecycle performed
- [x] Tests are focused and not proof theater
- [x] Deposit-arena enrollment selector is multi-deposit-safe (`ExplicitOnly`) — DA repair applied

## Constraints preserved

- 0.0.8.2 closeout closed — not reopened
- **FIELD-MOVIE-DATASET-0** / editor export deferred
- PR5 is hyperlane → bounded link (next generator rung)
