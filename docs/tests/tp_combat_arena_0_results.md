# TP-COMBAT-ARENA-0 / 0R Results

## Status

**DA-GRADUATED (2026-07-05) — one-time, non-precedential Homing Boundary exception accepted.** The scenario-specific combat hydrator is homed in `simthing-clausething/src` by explicit owner clearance for this rung only (not precedent; Phase 5+ homes scenario candidate code in `simthing-workshop`). GPU==CPU bit-exact parity was accepted at graduation on **owner attestation**, and has since been **strengthened to a citable owner-local proof** — `docs/tests/tp_combat_arena_0_gpu_proof.md` (PR #1148), `DOCTRINE-TESTS-VERDICT: PASS`, all 3 tests on a real NVIDIA RTX 4080 adapter, no skip path, tested SHA `72dc4355` (DA-verified to contain byte-identical combat code to this merge). Evidence repaired in `TP-COMBAT-ARENA-0R`; classification table below is honest and complete. Superseded prior state: `PROBATION / DA-OWNER REVIEW`.

## Identity

| Field | Value |
|---|---|
| PR | [#1145](https://github.com/khorum08/SimThing/pull/1145) |
| Branch | `tp-combat-arena-0` |
| Base | `origin/master` |
| Rung | Phase 4.0 `TP-COMBAT-ARENA-0` + remedial `TP-COMBAT-ARENA-0R` |
| Head (0R proof) | `c9325a8774dabd5758cf0b69508c3f9dcf36281b` |

## One-time owner-cleared exception

Owner/Opus cleared the in-flight `simthing-clausething/src/hydrate_combat_arena.rs` combat hydrator for **this rung only**. This is **not precedent**. The code would not exist without the 0.0.8.5 Terran-Pirate scenario track and is classified as **scenario candidate code** homed in an engine crate by explicit exception. DA may merge with the exception, request move-to-workshop, or require further remediation. Phase 5 (`TP-DIPLOMACY-FLOW-0`) remains blocked until DA/Owner clearance.

## Homing Boundary Classification

Classifier: *Would this code exist if this scenario didn't?*

| Symbol / path | Would this exist without TP? | Classification | Action |
|---|---:|---|---|
| `mod hydrate_combat_arena` — `lib.rs` | no | owner-cleared exception | keep as one-time owner-cleared exception |
| `HydratedCombatArenaPayload` — `lib.rs` export | no | scenario candidate code | keep as one-time owner-cleared exception |
| `HydratedCombatShipEnrollment` — `lib.rs` export | no | scenario candidate code | keep as one-time owner-cleared exception |
| `HydratedCombatShipEnrollment` — `hydrate_combat_arena.rs` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `HydratedCombatArenaPayload` — `hydrate_combat_arena.rs` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `ParsedCombatArenaPayload` — `hydrate_combat_arena.rs` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `parse_combat_arena_payload` | no | scenario candidate code (combat_arena_payload grammar) | keep as one-time owner-cleared exception |
| `finalize_combat_arena_payload` | no | scenario candidate code (terran/pirate owner+fleet requirements) | keep as one-time owner-cleared exception |
| `attach_combat_contact_fleets` | no | scenario candidate code (fleet-contact placement) | keep as one-time owner-cleared exception |
| `place_combat_contact_fleet` | no | scenario candidate code (`Fleet`/`Cohort` construction for contact) | keep as one-time owner-cleared exception |
| `complete_combat_arena_payload` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `build_combat_transfers` | no | scenario candidate code (hostile hull damage transfer recipe) | keep as one-time owner-cleared exception |
| `apply_combat_arena_to_game_mode` | no | scenario candidate code (combat property/event/overlay lowering) | keep as one-time owner-cleared exception |
| `seed_combat_property_columns_on_tree` | no | scenario candidate code (per-ship combat column seeding) | keep as one-time owner-cleared exception |
| `combat_hull_property_spec` / `combat_weapon_property_spec` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `game_session_child` / `game_session_child_mut` | no | owner-cleared exception (`GameSession` kind read) | keep as one-time owner-cleared exception |
| `game_session_galaxy_map` / `game_session_galaxy_map_mut` | no | owner-cleared exception (galaxy-map tree walk) | keep as one-time owner-cleared exception |
| `system_matches_target` / `system_contact_target_id` | no | scenario candidate code (contact system selection) | keep as one-time owner-cleared exception |
| `find_simthing_mut` | no | scenario candidate code (combat-local tree helper) | keep as one-time owner-cleared exception |
| `parse_combat_modifier_entries` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `COMBAT_TRANSFER_CAP` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `HydratedScenarioPack.combat_arena_payload` — `hydrate_scenario.rs` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `combat_arena_payload` parse/hydrate loop — `hydrate_scenario.rs` | no | scenario candidate code | keep as one-time owner-cleared exception |
| `TP_FLEET_*` / `TP_SHIP_*` property IDs — `hydrate_scenario.rs` | no | scenario candidate code (TP fleet/ship metadata) | keep as one-time owner-cleared exception |

No symbol in this delta is classified as a genuinely generic ClauseScript surface reusable across scenarios without TP combat meaning.

## Corrected conformance statements

| Claim | Accurate status |
|---|---|
| No combat subsystem | yes — transfers + overlays + boundary removal only; no combat engine crate |
| No new opcode / WGSL / AccumulatorRole | yes |
| No net-new gameplay kind branching in `simthing-spec` | yes — `simthing-spec` production `src/**` untouched |
| No `SimThingKind` branching in all production `src/**` | **no — false; retracted in 0R** |
| ClauseThing production source kind exposure | **yes — owner-cleared:** `hydrate_combat_arena.rs` includes `GameSession` tree-walk kind reads (lines 598, 611) and direct `Fleet`/`Cohort` node construction for this rung. Classified as one-time Homing Boundary exception, not precedent |
| Homing Boundary clean by default | **no — owner-cleared by exception only** |
| Owner bonus via overlay path only | proven (`owner_weapon_damage_mult_changes_damage_via_overlay_only`) |
| GPU main parity test does not skip without adapter | yes — `require_gpu()` hard-fails |

## INSPECT / triage

### Delta-specific `SPEC-LOWERER-KIND-READ` (PR scope)

| Location | Hit | Classification | Triage |
|---|---|---|---|
| `hydrate_combat_arena.rs:598` | `child.kind == SimThingKind::GameSession` | owner-cleared, non-gameplay tree navigation | DA review required; not silently cleared |
| `hydrate_combat_arena.rs:611` | `child.kind == SimThingKind::GameSession` | owner-cleared, non-gameplay tree navigation | DA review required; not silently cleared |

### Whole-tree scan context

`bash scripts/ci/doctrine_scan.sh` reports `INSPECT failures=0 inspect=415`. The 415 count is predominantly pre-existing backlog (`mapgen_lattice.rs`, `jomini/errors.rs`, `simthing-spec` designer_admission, etc.). The **delta** above is the PR-relevant kind-read exposure and must not be dismissed by citing the whole-tree count.

### Other INSPECT

None with hard-failure status. No new triage rows added to `inspect_justifications.tsv` in 0R — delta hits are documented here for DA review.

## Rustification / lifecycle

| Item | Status |
|---|---|
| New tests inventoried | 3 rows in `scripts/ci/test_inventory.tsv` |
| `birth_track` | `0.0.8.5-terran-pirate` on all 3 tests |
| Per-rung lifecycle track | none — no `test_lifecycle_tracks.tsv` edit |
| Boundary rows | 3 rows in `scripts/ci/test_lifecycle_boundary_rows.tsv` |
| Drift gate | PASS — 0 unledgered runnable tests |
| Inventory check | INSPECT (exit 0) — expected promotion-target posture |

### Inventory rows

| Test | birth_track | class | verdict |
|---|---|---|---|
| `gpu_two_fleet_contact_matches_transfer_oracle` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `zero_hp_threshold_requests_boundary_removal` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |
| `owner_weapon_damage_mult_changes_damage_via_overlay_only` | `0.0.8.5-terran-pirate` | oracle-parity | KEEP |

## Implemented authored forms

| Form | Lowering |
|---|---|
| `combat_arena_payload` | Co-located hostile ship contact in a named `system_target`; one ship per side for oracle isolation |
| Hull damage | `SubtractFromSource` discrete transfers between opposing ship weapon/hull columns |
| HP recovery | Optional `governed_by` hull recovery per tick (authored; not exercised in main parity test) |
| Owner combat bonus | `ship_weapon_damage_mult` decoded via shipsize decoder → overlay on weapon column |
| Ship death | Zero-hull rising `Threshold` → `EffectSpec::Remove` via scripted boundary handler |

## Load-bearing proofs

| Proof | Test | What it catches |
|---|---|---|
| GPU==CPU transfer oracle bit-exact | `gpu_two_fleet_contact_matches_transfer_oracle` | Silent GPU transfer drift on hostile hull damage |
| Zero-HP boundary removal | `zero_hp_threshold_requests_boundary_removal` | Ships surviving past hull zero or missing slot tombstone |
| Overlay-only owner bonus | `owner_weapon_damage_mult_changes_damage_via_overlay_only` | Hardcoded owner logic smuggled into damage calc |

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-clausething --test tp_combat_arena_0 -- --nocapture` | PASS (3 passed) |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_boundary_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | INSPECT failures=0 inspect=415 (0R run) |
| `git diff --check origin/master...HEAD` | PASS |

## Scope ledger

| Item | Touched? |
|---|---|
| simthing-clausething | yes — `hydrate_combat_arena.rs`, `hydrate_scenario.rs`, `lib.rs`, tests + fixture |
| simthing-spec | no production edits |
| simthing-gpu/driver/sim/mapeditor/tools/workshop | no |
| workflows / scans / allowlists | no |
| `test_lifecycle_tracks.tsv` | no |
| workspace `cargo test` | no |

## Graduation routing

- **CI verdict:** doctrine_scan INSPECT with `failures=0`; delta kind-read hits documented above (accepted)
- **GPU proof basis:** **citable owner-local proof** (strengthened post-graduation) — `docs/tests/tp_combat_arena_0_gpu_proof.md` (PR #1148), `DOCTRINE-TESTS-VERDICT: PASS`, 3/3 tests on a real NVIDIA RTX 4080 adapter, `require_gpu()` panics on no-adapter (no skip path), `0 ignored / 0 filtered`, tested SHA `72dc4355` (DA-verified `--is-ancestor` of graduation, combat code byte-identical). Graduation was originally accepted on owner attestation per §6 waiver; this artifact supersedes that weaker posture.
- **Risk class:** Homing Boundary exception / semantic scenario-code-in-engine-crate — **accepted one-time, non-precedential**
- **DA disposition (2026-07-05):** **DA-GRADUATED.** `TP-COMBAT-ARENA-0` complete with the one-time exception. The exception is **not precedent**; the queued net-new-engine-symbol tripwire is the durable forward enforcement.
- **Phase 5:** `TP-DIPLOMACY-FLOW-0` **authorized to proceed** — its scenario candidate code (diplomacy hydrator) homes in `simthing-workshop`, not `clausething`, with no exception.