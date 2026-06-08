# DEMO-0080-1-IMPL-0 Results

Verdict: **PASS**.

Implementation scope: `DEMO-0080-1` headless Nested Starmap demo/export library helper only.
**No CLI binary.** Pure read/orchestration over the existing `CONTROL-0080-1 → DEFAULT-SCHEDULE-0080-1 → GAMEPLAY-0080-1` path.

## What Landed

- Added `crates/simthing-driver/src/demo_0080_1.rs`.
- Exported the gate from `crates/simthing-driver/src/lib.rs`.
- Added `crates/simthing-driver/tests/demo_0080_1.rs` with the 24 required opening-spec tests.

The gate is explicit opt-in/default-off. It applies a canonical bounded `CONTROL-0080-1`
command batch (via `Control0081CommandBatch::canonical_run()`) and runs the existing
`control → DEFAULT-SCHEDULE-0080-1 → GAMEPLAY-0080-1` path via `admit_control_0080_1`.

The demo report includes:

- Atlas residency summary presence flag
- Faction-index ECON summary presence flag
- Owner-overlay inheritance summary presence flag
- Ownership up-aggregation summary presence flag
- FIELD_POLICY movement trace presence flag
- Terran and Pirate movement rows (step index, faction, start/end starsystem, threshold/event/boundary flags, identity preserved, owner overlay preserved, membership-without-reparenting)
- Command transcript (command index, command name, accepted, target bounded field, old/new value)
- Deterministic replay checksum (FNV-64 over applied command count, step count, movement counts, movement row flags)
- Full pipe-delimited text export (`DEMO-0080-1|...`, `CMD|...`, `MOVE|...` lines)

## What Was NOT Added

No CLI binary, no direct movement command, no external `BoundaryRequest`, no FIELD_POLICY bypass, no CPU
planner, no player command loop, no UI framework, no real-time loop, no global default schedule,
no semantic/raw WGSL, no new shader/GPU kernel, no hard currency/markets/trade/`ai_budget`, no
nested Resource Flow, no ClauseThing dependency, no `simthing-spec` alteration, no invariant edit,
no passive proof wrapper, no general command/demo framework. The demo is a pure
read/orchestration helper.

## Required Tests

Passed:

- `cargo test -p simthing-driver --test demo_0080_1` — **24/24 PASS**

Coverage includes:

- opt-in/default-off behavior (default session → disabled/no-op; `enabled_by_default = true` → rejected);
- canonical control batch admission and command transcript presence;
- existing control→schedule→observation path invocation;
- Nested Starmap export emission (contains `DEMO-0080-1`, `Nested Starmap`, `CMD|`, `MOVE|` lines);
- Terran and Pirate movement rows present;
- atlas residency, faction-index ECON, owner-overlay/up-aggregation summary flags;
- deterministic replay (two runs produce identical reports and checksums);
- rejection of CLI binary, direct movement command, external boundary request, FIELD_POLICY bypass, player
  command loop, UI framework, real-time loop, global default schedule, semantic/raw WGSL,
  new shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow, and
  ClauseThing dependency;
- docs status constant matches gate.

## Required Regression Status

Full required regression list run after implementation:

- `cargo test -p simthing-driver --test control_0080_1` — **20/20 PASS**
- `cargo test -p simthing-driver --test gameplay_0080_1` — **22/22 PASS**
- `cargo test -p simthing-driver --test demo_0080_0` — **18/18 PASS**
- `cargo check --workspace` — clean (pre-existing warnings only)
