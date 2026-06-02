# DEFAULT-SCHEDULE-0080-1-IMPL-0 Results

Verdict: **IMPLEMENTED / PASS** - scenario-scoped Nested Starmap SEAD-sourced schedule/movement.

## Scope

- Implemented `run_default_schedule_0080_1` / `replay_default_schedule_0080_1` in `crates/simthing-driver`.
- Explicit opt-in/default-off. Disabled input is a no-op and does not run `PRODUCTION-PATH-0080-1`.
- Consumes `run_production_path_0080_1`; rejects disabled, rejected, or non-admitted production-path posture.
- Uses read-only `ProductionPath0081SeadCompositeGapTerms` as a deterministic threshold source.
- Converts accepted threshold decisions into event emission and materialized `BoundaryRequest` records only.
- Routes accepted requests through the existing mobility/transfer substrate posture; no direct movement command.
- Bounded to three deterministic scenario-local steps for this implementation slice.

## Canonical Movement Table

| Step | Mover | Faction | Start starsystem | End starsystem | Threshold accepted | Event emitted | BoundaryRequest materialized | Identity preserved | Owner overlay preserved | Membership updated |
|---:|---:|---|---:|---:|---|---|---|---|---|---|
| 0 | 80301 | Terran | 0 | 1 | true | true | true | true | true | true |
| 1 | 80401 | Pirate | 6 | 2 | true | true | true | true | true | true |
| 2 | n/a | n/a | n/a | n/a | true | true | true | n/a | n/a | n/a |

The Terran move uses the contended Terran-owned ECON clearing posture. The Pirate move uses the neutral
Pirate-present ECON clearing posture. Both movement outcomes preserve ship identity and owner overlay, update
location membership without reparenting, keep owner simthings as non-spatial session siblings, and reject
capture-as-reparenting.

## Consumed Reports

- Atlas sparse residency: consumed from the production-path report and reflected in schedule diagnostics.
- Faction-index ECON: consumed from the production-path report, including Terran/Pirate fixed-set clearing.
- Pirate full economy: preserved as an adversarial economy participant, not merely a disruptor.
- Replay: deterministic checksum over production-path checksum, step decisions, movement outcomes, and guardrails.

## Guardrails Confirmed

No observation/control/demo, direct movement command, external `BoundaryRequest`, CPU planner/urgency/commitment,
default session pass-graph wiring, global default schedule, realtime loop/UI, semantic/raw WGSL, new shader or GPU
kernel, hard currency/markets/trade/`ai_budget`, nested Resource Flow, unbounded factions, owner spatial parent,
capture-as-reparenting, ClauseThing dependency, invariant edit, passive proof wrapper, or general scheduler was added.

## Files Touched

- `crates/simthing-driver/src/default_schedule_0080_1.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/default_schedule_0080_1.rs`
- `docs/tests/phase_default_schedule_0080_1_impl_results.md`
- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/production_paths/default_schedule_0080_1_opening_spec.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`

## Tests

Commands run:

```powershell
cargo test -p simthing-driver --test default_schedule_0080_1
cargo test -p simthing-driver --test production_path_0080_1
cargo test -p simthing-driver --test econ_scale_0080_0
cargo test -p simthing-driver --test atlas_0080_0
cargo test -p simthing-driver --test demo_0080_0
cargo test -p simthing-driver --test control_0080_0
cargo test -p simthing-driver --test gameplay_0080_0
cargo test -p simthing-driver --test default_schedule_0080_0
cargo test -p simthing-driver --test production_path_0080_0
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test mobility_econ0_substrate
cargo test -p simthing-spec --test mobility_owner0_substrate
cargo test -p simthing-spec --test mobility_runtime0_composition
cargo test -p simthing-spec --test mobility_runtime1_production_fixture
cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture
cargo test -p simthing-driver --test phase_m_sead_obs4_threshold_event
cargo test -p simthing-driver --test phase_m_sead_event0_compaction
cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline
cargo test -p simthing-spec --test sead_obs0_overlay_score_admission
cargo check --workspace
```

Result: PASS. No skipped tests.
