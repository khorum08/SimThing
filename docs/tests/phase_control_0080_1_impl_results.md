# CONTROL-0080-1-IMPL-0 Results

Verdict: **PASS**.

Implementation scope: `CONTROL-0080-1` bounded Nested Starmap command admission only.

## What Landed

- Added `crates/simthing-driver/src/control_0080_1.rs`.
- Exported the gate from `crates/simthing-driver/src/lib.rs`.
- Added `crates/simthing-driver/tests/control_0080_1.rs` with the 20 required opening-spec tests.

The gate is explicit opt-in/default-off. It accepts the narrow command vocabulary:

`set_step_count`, `set_terran_threshold`, `set_pirate_threshold`, `set_terran_source_starsystem`,
`set_terran_candidate_starsystem`, `set_pirate_source_starsystem`,
`set_pirate_candidate_starsystem`, `set_supply_security_gap`, `set_bilateral_relational_gap`,
`set_composite_gap_sum`, `run_observed_scenario`, and `export_transcript`.

Commands write only existing bounded schedule values or local bounded control config:

- `DefaultSchedule0081Input.step_count`, bounded to `0..=3`.
- `DefaultSchedule0081Input.movement_threshold`, derived from admitted Terran/Pirate thresholds.
- Bounded Nested Starmap control config values, including source/candidate starsystem selectors and SEAD gap terms.

Observed runs use only the existing path:

`CONTROL-0080-1` -> `DEFAULT-SCHEDULE-0080-1` -> `GAMEPLAY-0080-1`

The command layer does not directly move ships, emit external `BoundaryRequest` records, bypass SEAD,
add a CPU planner, create a player command loop, add UI/realtime/demo packaging, register a global
default schedule, add semantic/raw WGSL, add a shader/kernel, add hard currency, add nested Resource
Flow, alter `simthing-spec`, edit invariants, or implement ClauseThing.

## Transcript Shape

Each admitted report includes deterministic command transcript rows with:

| Field | Meaning |
|---|---|
| `command_index` | Stable position in the submitted command batch |
| `command` | Canonical command name |
| `accepted` | Whether admission accepted the command |
| `target_bounded_field` | Existing bounded schedule field or bounded control config field |
| `old_value` / `new_value` | Deterministic value transition |
| `run_observed` | Whether the row invokes `GAMEPLAY-0080-1` observation |
| `export_produced` | Whether the row requests text export |
| `replay_checksum` | Final deterministic admission report checksum |

## Required Tests

Passed:

- `cargo test -p simthing-driver --test control_0080_1`

Coverage includes:

- opt-in/default-off behavior;
- bounded command admission and schedule/config value writes;
- observed scenario invocation through `DEFAULT-SCHEDULE-0080-1` -> `GAMEPLAY-0080-1`;
- deterministic transcript export and replay checksum;
- rejection of direct Terran/Pirate movement, external boundary requests, SEAD bypass, CPU planner,
  player command loop, UI framework, realtime loop, global default schedule, semantic/raw WGSL,
  new shader/GPU kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow, and
  ClauseThing dependency.

## Required Regression Status

Full required regression list was run in the implementation branch after this report was authored.
See the PR validation summary for final command output. Pre-existing warnings only.
