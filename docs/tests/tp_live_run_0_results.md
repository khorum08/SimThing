# TP-LIVE-RUN-0 Results

## Status

**PROBATION / proof-present / clearance-pending.** Implementation complete; orchestrator/DA review required before merge.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-LIVE-RUN-0` |
| Branch | `codex/tp-live-run-0` |
| Source scenario | `terran_pirate_galaxy.clause` (TP-FULL-TRANSPILE-0 fixture) |
| Helper | `crates/simthing-workshop/src/live_run_post_hydration.rs` |
| Test | `crates/simthing-workshop/tests/tp_live_run_0.rs` |
| birth_track | `0.0.8.5-terran-pirate` |

## Mechanism

1. Parse/hydrate the full transpile fixture.
2. Workshop `apply_live_run_post_hydration` chains commitments → fleet-movement → fronts+PALMA on a **7×7** contested border theater.
3. Placement re-bind table maps `tp_base::…` embedded targets + authority `SimThingId` → theater install keys.
4. Multi-tick STEAD field ticks on a single opened mapping session (front pressure shifts).
5. STEAD commitment threshold fires Terran reinforce from L3 urgency (GPU threshold events).
6. Combat HP multi-tick CPU transfer oracle + GPU==CPU one-step transfer parity on a real adapter.

## Scope Ledger

| In | Out |
|---|---|
| workshop live-run helper + one integration test | TP-DA-CLOSEOUT-0 |
| inventory/boundary + results | full-galaxy dense MF / atlas |
| real-adapter GPU for field + combat | kernel/WGSL/orientation |

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

Asserted in test: rebind non-empty, both factions present, `tp_base::` stems, authority id resolves in tree.

## Live/headless run proof

- Theater `SimSession::open_from_spec` on re-bound cells.
- Combat `SimSession::open_from_spec` on combat enrollments.
- Real GPU adapter required (`GpuContext::new_blocking`).
- Studio load path: canonical ScenarioSpec/install via driver session (same authority path as TP-FULL-TRANSPILE-0 + session open).

## Front-shift proof

- ≥3 field ticks with light→heavy pressure seeds.
- L2 `reduction_parent_value` changes across ticks (pre/post asserted).

## Combat-resolution proof

- ≥3 CPU transfer ticks change hull columns.
- GPU transfer one-step hull bits match first CPU oracle tick (real adapter).

## STEAD commitment proof

- Heavy pressure seeds → L3 urgency exceeds Terran reinforce threshold.
- Threshold event kind `TP_TERRAN_REINFORCE_EVENT_KIND` present.
- Fired via mapping `tick_with_commitment_spec_fixture` (GPU threshold path), not CPU planner emit.

## No CPU planner / no route solver proof

Source scan of live-run helper + test forbids:

`cpu_planner`, `planner_commitment`, `route_solver`, `path_solver`, `predecessor_map`, `per_tick_device_create`, `per_tick_buffer_create`.

## No per-tick device/buffer creation proof

- Theater mapping session opened once and reused across field/commitment ticks.
- Combat transfer plan synced once on isolated GPU state for parity step.
- No `GpuContext::new` inside tick loops for field ticks.

## Homing / substrate boundary

- Workshop-only Mechanism B composition (scenario envelope).
- No kernel / WGSL / orientation / atlas scheduler edits.

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

## Graduation routing

| Field | Value |
|---|---|
| Risk class | semantic + gpu (real adapter) |
| Recommended posture | PROBATION → clearance router |
| Falsification | drop rebind / zero pressure change / no commitment event / no hull change → FAIL |
