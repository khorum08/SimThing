# TP-LIVE-RUN-0 Results

## Status

**PROBATION / proof-present / clearance-pending (0R2).** RF combat semantics tightened; orchestrator/DA review required before merge.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-LIVE-RUN-0` (+ 0R RF combat) |
| Branch | `codex/tp-live-run-0` |
| Source scenario | `terran_pirate_galaxy.clause` (TP-FULL-TRANSPILE-0 fixture) |
| Helper | `crates/simthing-workshop/src/live_run_post_hydration.rs` |
| Test | `crates/simthing-workshop/tests/tp_live_run_0.rs` |
| birth_track | `0.0.8.5-terran-pirate` |

## Doctrine ingestion

D1/D2 ingested: all conflict is RF accumulator economics and overlay filters. Combat is not a subsystem beside the tree. Hull HP is modeled as a damage-to-kill / hull-deficit emission band; incoming damage is the resource that fills the deficit; destroyed_ships is emitted by the RF flow and then depletes num_ships.

## Mechanism

1. Parse/hydrate the full transpile fixture.
2. Workshop `apply_live_run_post_hydration` chains commitments → fleet-movement → fronts+PALMA on a **7×7** contested border theater and composes RF combat economics over weapon→hull transfers.
3. Placement re-bind table maps `tp_base::…` embedded targets + authority `SimThingId` → theater install keys.
4. Multi-tick STEAD field ticks on a single opened mapping session (front pressure shifts).
5. STEAD commitment threshold fires Terran reinforce from L3 urgency; hard `BoundaryRequest::AttachOverlay` proof.
6. Combat: real-adapter multi-tick RF transfer fills hull-deficit band; RF emission-band law emits destroyed_ships and depletes num_ships; CPU oracle is parity-only.

## Scope Ledger

| In | Out |
|---|---|
| workshop live-run helper + one integration test | TP-DA-CLOSEOUT-0 |
| inventory/boundary + results | full-galaxy dense MF / atlas |
| real-adapter RF transfer + field GPU | kernel/WGSL/orientation / new AccumulatorRole |

## Theater selection proof

- Theater grid = 7×7 (`TP_LIVE_RUN_THEATER_GRID`), horizon-3 from fleet-movement chain.
- Contested Terran/Pirate border systems from ownership volumes on authority tree.
- Fixture id `terran_pirate_galaxy` (not a toy clause).

## Placement / link re-bind proof

**Mapping rule:**

1. Select contested border systems from ownership volumes on `authority_root`.
2. Assign each a `(theater_row, theater_col)` on the 7×7 grid; stamp `theater_target_id = "{embedded_target}@{r}_{c}"`.
3. Runtime install clones the authority system shell by `simthing_id` into the session root and registers `install_targets[theater_target_id] → shell.id`.
4. Embedded lattice remains STEAD feedstock; every theater cell carries a resolved authority `simthing_id` (no dangling producer-local id).

## Live/headless run proof

- Theater `SimSession::open_from_spec` on re-bound cells.
- Combat `SimSession::open_from_spec` on combat enrollments with RF combat columns.
- Real GPU adapter required (`GpuContext::new_blocking`).
- Studio load path: canonical ScenarioSpec/install via driver session.

## Front-shift proof

- ≥3 field ticks with light→heavy pressure seeds.
- L2 `reduction_parent_value` changes across ticks (pre/post asserted).

## Combat-resolution proof

TP-LIVE-RUN-0R2 proves RF-shaped combat inside the scenario envelope: real-adapter RF transfer fills the hull-deficit / damage-to-kill band, and workshop-homed emission-band settlement computes destroyed_ships and num_ships from that RF output. Generic on-device RF emission of destroyed_ships remains a named substrate opportunity, not claimed as completed substrate.

RF shape asserted:

- weapon / incoming damage column (stored damage budget; SubtractFromSource drains it)
- hull deficit / damage-to-kill band column
- `damage_to_kill_1_hull` price column
- `num_ships` column (per-enrollment)
- `destroyed_ships` column (workshop settlement writeback after RF fill)
- cross-opponent RF transfers: Terran weapon slot→Pirate hull slot and reverse (`source_slot != target_slot`)

Weapon multi-tick semantics: harness reinstalls per-tick damage production equal to DTK into the weapon source so multi-tick RF can fill the kill band. That reinstall is **test-harness production input**, not a claimed permanent RF production opcode.

## Overlay-filter proof / boundary

No combat modifier overlay is authored on the `terran_pirate_galaxy` combat path. Overlay-filter proof is **structural only**: composition restricts discovered combat-related overlays to weapon/hull RF columns, but non-vacuous modifier effect is **not** claimed.

## 0R2 semantic tightening

This PR proves RF-shaped combat in the scenario envelope. Real-adapter RF transfer fills the hull-deficit / damage-to-kill band. Workshop-homed emission-band settlement computes destroyed_ships and num_ships from that RF output. Generic on-device RF emission of destroyed_ships is not claimed and remains a named substrate opportunity.

## Accounting boundary

`num_ships` is the per combat enrollment ship-object count, seeded to 1.0. Fleet/cohort-scale aggregation of ship counts is not claimed unless the test asserts that aggregation (it does not).

## RF → STEAD coupling boundary

Combat RF and theater front RF are both derived from the full TP fixture and authority tree, but exercised in focused sessions. This PR does **not** prove casualty output reduces up into ArenaPressureBinding on the next STEAD tick. That coupling remains DA-accepted TP-LIVE-RUN-0 scope residue / closeout Deviation candidate.

## STEAD commitment proof

- Heavy pressure seeds → L3 urgency exceeds Terran reinforce threshold.
- Threshold event kind `TP_TERRAN_REINFORCE_EVENT_KIND` present.
- Marker property `tp_commitment::terran_commitment_marker` resolves.
- `BoundaryRequest::AttachOverlay` constructed with target / overlay.affects / property_id / sub_field_deltas asserted.
- No tautological effect fallback.

## No CPU planner / no route solver proof

Source scan of live-run helper + test forbids:

`cpu_planner`, `planner_commitment`, `route_solver`, `path_solver`, `predecessor_map`, `per_tick_device_create`, `per_tick_buffer_create`, `combat_engine`, `combat_resolver`, `combat_planner`, `manual_hull_resolver`, `manual_hp_subtract`, `bespoke_hp_resolver`, `zero_hp_removal_system`, `owner_bonus_combat`, `cpu_combat_loop`.

## No per-tick device/buffer creation proof

- Theater mapping session opened once and reused across field/commitment ticks.
- Combat transfer plan synced once on isolated GPU state for multi-tick RF transfer.
- No device create inside field tick loop.

## Homing / substrate boundary

- Workshop-only Mechanism B composition (scenario envelope).
- No kernel / WGSL / orientation / atlas scheduler / new AccumulatorRole.

## 0R / 0R2 notes

0R: refactored combat proof from CPU hull-change oracle into STEAD/RF conflict economics. Hull HP is treated as the damage-to-kill band; incoming damage fills that deficit; destroyed_ships is workshop-settled from RF output. BoundaryRequest proof tightened.

0R2: cross-opponent slot proof; non-vacuous destroyed_ships + num_ships depletion; accounting/overlay/RF→STEAD boundaries explicit; weapon source = budget + harness per-tick production reinstall documented; no overclaim of generic GPU emission.

## Rustification / lifecycle

| Field | Value |
|---|---|
| birth_track | `0.0.8.5-terran-pirate` |
| class | `oracle-parity` |
| verdict | `KEEP` |
| dsu_survivals | `0` |
| per-rung lifecycle track | **not** created |

## Load-bearing proofs

```text
cargo test -p simthing-workshop --test tp_live_run_0 -- --nocapture
  → terran_pirate_border_theater_live_run_multi_tick PASS
```

## Known gaps / next

TP-DA-CLOSEOUT-0 remains next: Scope Ledger over every §2 acceptance element, Deviation Records, corpus-abstraction candidate list, and DA sign-off.

Dense full-galaxy Movement-Front execution remains an atlas Deviation, not a silent failure.

Generic RF emission primitive that writes destroyed_ships directly from floor(hull/dtk) without workshop emission-band settlement remains a named substrate opportunity (not invented here — no new AccumulatorRole).

## Graduation routing

| Field | Value |
|---|---|
| Risk class | semantic + gpu (real adapter) + RF doctrine |
| Recommended posture | PROBATION → clearance router |
| Falsification | drop rebind / zero pressure change / no commitment BoundaryRequest / no RF hull-band fill / CPU-primary combat → FAIL |
