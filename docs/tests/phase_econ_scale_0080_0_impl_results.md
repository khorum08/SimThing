# ECON-SCALE-0080-0 Implementation Results

**Date:** 2026-06-02
**Verdict:** **PASS** — bounded faction-indexed contended ECON scaling for `SCENARIO-0080-1` (Nested Starmap).

## Files touched

- `crates/simthing-driver/src/econ_scale_0080_0.rs` (new — module)
- `crates/simthing-driver/tests/econ_scale_0080_0.rs` (new — 17 tests)
- `crates/simthing-driver/src/lib.rs` (module declaration + re-export)
- `docs/design_0_0_8_0_consumer_pulled_production_track.md` (ladder row → IMPLEMENTED/PASS)
- `docs/production_paths/econ_scale_0080_0_opening_spec.md` (impl-result note)
- `docs/workshop/mapping_current_guidance.md` (status line)
- `docs/worklog.md` (top entry)
- `docs/tests/phase_econ_scale_0080_0_impl_results.md` (this report)

## Implementation scope

`ECON-SCALE-0080-0` only. A narrow `simthing-driver` module exposing
`run_econ_scale_0080_0(&EconScale0080Input) -> EconScale0080RunReport` and `replay_econ_scale_0080_0()`,
plus the `EconScale0080{Gate,Surface,Scenario,Faction,FactionIndex,Participant,StarsystemEconomy,
ClearingInput,ClearingReport,RunReport,ForbiddenRequests}` types. No `PRODUCTION-PATH-0080-1`, schedule,
observation, control, or demo for `0080-1` was implemented. `ATLAS-0080-0` and Local Patrol Economy
`0080-0` were not altered.

## Confirmations

- **Explicit opt-in / default-off:** default surface returns a disabled no-op (no clearing); `enabled_by_default` is rejected; only `explicit_opt_in()` admits. PASS.
- **Default path single-owner unchanged:** the surface is opt-in/scenario-scoped; the default ECON path is never altered (`single_owner_default_unchanged = true`; disabled path performs no clearing). PASS.
- **Bounded fixed faction set:** Terran + Pirate (`faction_count == 2`); `EconScale0080FactionIndex::is_bounded()` enforces count + uniqueness; `unbounded_faction_fanout` and oversized participant lists are rejected; no dynamic faction registry. PASS.
- **Faction-indexed participation:** participants carry a bounded faction index (Terran=0, Pirate=1); clearing reports surface distinguishable `terran_extraction` / `pirate_extraction` and `faction_indices_present`. PASS.
- **Adversarial contended clearing:** deterministic bounded integer clearing over `supply`/`extraction`/`security`/`disruption`/`contention`; Terran extracts first (owner-priority via subsidiarity), the adversarial pirate contends over remaining supply, contention + disruption rise with pirate pressure, security falls. The contended starsystem shows `supply_after < supply_before`, `contention_after > before`, `disruption_after > before`, `pirate_extraction > 0`. PASS.
- **Parity / determinism:** an independent CPU oracle (`clearing_scalars_oracle`) re-derives every clearing scalar; `parity_bit_exact` is the equality of production vs oracle and holds for all reports. No GPU-resident field is touched; no semantic/raw WGSL; no semantically-named shader. Replay (`replay_econ_scale_0080_0`) produces identical reports + checksum. PASS.
- **Subsidiarity preserved:** the faction index layers onto the session-clearinghouse / FlatStar posture (`subsidiarity_preserved = true`, `flat_star_posture_preserved = true`); `replace_subsidiarity` is rejected; no nested Resource Flow depth introduced. PASS.
- **Pirate = full economy faction:** `pirate_is_full_economy_faction = true`; the pirate actually extracts (participates in resource flow) in both a contended Terran-owned starsystem and a neutral starsystem it entered — not merely a disruptor identity. PASS.
- **Guardrails (all rejected as diagnostics):** hard currency; markets/trade/`ai_budget`; nested Resource Flow; unbounded factions; replace-subsidiarity; CPU planner; semantic/raw WGSL; semantically-named shader; ClauseThing dependency; invariant edit; `PRODUCTION-PATH-0080-1`. PASS.

## Tests run (exact commands)

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test econ_scale_0080_0` | **17/17 PASS** |
| `cargo test -p simthing-driver --test atlas_0080_0` | 17 PASS |
| `cargo test -p simthing-driver --test demo_0080_0` | 18 PASS |
| `cargo test -p simthing-driver --test control_0080_0` | 18 PASS |
| `cargo test -p simthing-driver --test gameplay_0080_0` | 15 PASS |
| `cargo test -p simthing-driver --test default_schedule_0080_0` | 24 PASS |
| `cargo test -p simthing-driver --test production_path_0080_0` | 21 PASS |
| `cargo test -p simthing-spec --test mobility_alloc0_substrate` | 15 PASS |
| `cargo test -p simthing-spec --test mobility_reenroll0_substrate` | 16 PASS |
| `cargo test -p simthing-spec --test mobility_idroute0_substrate` | 20 PASS |
| `cargo test -p simthing-spec --test mobility_econ0_substrate` | 20 PASS |
| `cargo test -p simthing-spec --test mobility_owner0_substrate` | 24 PASS |
| `cargo test -p simthing-spec --test mobility_runtime0_composition` | 23 PASS |
| `cargo test -p simthing-spec --test mobility_runtime1_production_fixture` | 28 PASS |
| `cargo test -p simthing-driver --test mobility_runtime1a_runtime_fixture` | 21 PASS |
| `cargo test -p simthing-driver --test phase_m_sead_obs4_threshold_event` | 7 PASS |
| `cargo test -p simthing-driver --test phase_m_sead_event0_compaction` | 7 PASS |
| `cargo test -p simthing-driver --test phase_m_sead_pipe0_observer_event_pipeline` | 7 PASS |
| `cargo test -p simthing-spec --test sead_obs0_overlay_score_admission` | 29 PASS |
| `cargo check --workspace` | Finished; only pre-existing warnings (unchanged) |

## Skipped tests

None. All required commands ran with the listed target names.

## Negative confirmation

No `PRODUCTION-PATH-0080-1`, no schedule/observation/control/demo for `0080-1`, no hard currency, no
markets/trade/`ai_budget`, no nested Resource Flow, no unbounded faction fan-out, no CPU planner, no
semantic/raw WGSL, no semantically-named shader, no ClauseThing implementation, no `simthing-spec`
alteration, no invariant edit, no passive proof wrapper, no closed-ladder reopen. `ATLAS-0080-0` remains
IMPLEMENTED/PASS; Local Patrol Economy `0080-0` unchanged.
