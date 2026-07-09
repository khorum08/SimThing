# STUDIO-SIM-CLOCK-0 Results

## Status

**PROBATION / proof-present** — Studio sim clock substrate landed with headless tests. Transport UI and live SimSession bridge deferred to 9.2 / 9.3. Rolled-in 9.0 readiness status/orientation update is in this PR.

## Identity

| Field | Value |
|---|---|
| Rung | `STUDIO-SIM-CLOCK-0` |
| Track | `0.0.8.6-studio-live-ops` |
| Kind | mechanical implementation + tests + rolled-in 9.0 docs/status |
| Prior rung | `STUDIO-LIVE-OPS-READINESS-0` merged [#1257](https://github.com/khorum08/SimThing/pull/1257) @ `4f8c250c` |

## Changed files

| Path | Role |
|---|---|
| `crates/simthing-mapeditor/src/studio_sim_clock.rs` | Clock substrate |
| `crates/simthing-mapeditor/src/lib.rs` | Module + re-exports |
| `crates/simthing-mapeditor/tests/studio_sim_clock_0.rs` | Headless proofs |
| `docs/tests/studio_sim_clock_0_results.md` | This evidence |
| `docs/design_0_0_8_6_studio_live_ops.md` | 9.0 graduated; 9.1 exit proof; pointer hygiene |
| `docs/tests/studio_live_ops_readiness_0_results.md` | 9.0 COMPLETE stamp |
| `docs/orchestrator_orientation.md` | Regenerated |
| `scripts/ci/test_inventory.tsv` | Four KEEP rows, `birth_track=0.0.8.6-studio-live-ops` |

## Clock API summary

```text
StudioSimClock::new()          // default paused, Rate1x, max_tps=10
play() / pause()
set_rate(Rate1x | Rate2x | Rate4x)
set_max_tps(f64) -> Result      // finite, > 0
advance(elapsed_secs) -> u64    // scheduled ticks this step
tick_index() / scheduled_tick_count()
is_paused() / rate() / max_tps() / effective_tps()
```

Scheduling (deterministic): while playing, demand accumulates
`elapsed * rate.multiplier() * max_tps`; whole ticks emit and advance `tick_index`.
Paused: `advance` returns 0 and freezes `tick_index`.

Documented rate-ratio tolerance: `STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE = 0.05`.

Authority: mapeditor-owned transport substrate — **not** Bevy Time, egui state, or ScenarioSpec.

## Semantics proven

| Semantic | Proof |
|---|---|
| Pause freezes scheduling + tick index | `pause_freezes_tick_index` |
| Play resumes under current rate/TPS | same |
| 2× / 4× proportional vs 1× within tolerance | `rate_2x_4x_ratios` |
| max_tps caps demand (no tick storm) | `max_tps_cap_holds` |
| Clock does not mutate ScenarioSpec | `clock_does_not_mutate_scenario_spec` |
| Headless (no Bevy window / GPU) | integration test binary only |

## Test results

```text
cargo test -p simthing-mapeditor --test studio_sim_clock_0
running 4 tests
test clock_does_not_mutate_scenario_spec ... ok
test max_tps_cap_holds ... ok
test pause_freezes_tick_index ... ok
test rate_2x_4x_ratios ... ok
test result: ok. 4 passed; 0 failed
```

## Doctrine INSPECT triage

`TEST-BUDGET` HEURISTIC INSPECT (4 named `#[test]`s) triaged **green** in `scripts/ci/triage_log.tsv` on branch `studio-sim-clock-0`: distinct load-bearing semantics required by the handoff, not malformed-input enumeration theater.

## Known gaps

- Transport UI (Pause/Play/2×/4×/TPS readout) — **9.2**
- Wire scheduled counts into live `SimSession` / driver ticks — **9.3**
- Modal library ⇒ pause — **9.5** (needs clock first)
- Clearance class for live-ops shape — **9.7**

## Next recommended rung

Per design dependency `9.1 → (9.2 ∥ 9.3)`:

**`STUDIO-SIM-CLOCK-UI-0`** first (UI drives this substrate; programmatic hooks for CI), then or in parallel **`STUDIO-LIVE-SESSION-BRIDGE-0`**.

## Falsification held

- Bevy Time is not clock authority
- egui/UI state is not clock authority
- ScenarioSpec not mutated by scheduling
- No GameMode/RF attach
- No live SimSession bridge
- No library modal UI
- 9.0 status/orientation rolled into this PR (not split)
