# CT vertical consumer contract — the closed CT-3b+4a production shape

> **Status: FROZEN (2026-06-12, CT-CONSUMER-0).** This page is the single entry point for
> consuming the closed ClauseThing headline vertical without re-auditing the ladder. It changes
> only when the production contract changes. Authority: the track ledger
> ([`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md)
> §11, **CLOSED**, every rung IMPLEMENTED / PASS) and the rung report
> ([`../tests/ct_3b_4a_impl_0a_results.md`](../tests/ct_3b_4a_impl_0a_results.md), including the
> Line 3R edge-crossing addendum and the authored-effects addendum).

## What runs, end to end, inside `SimSession::run()`

Per tick, under the explicit authored opt-in (`SparseRegionFieldV1` + `FlatStarOptIn`),
default-off, zero value readback on the runtime path:

```
ClauseScript fixture → hydration → GameModeSpec → open_from_spec install
RF arena bands (OrderBand 0 = effective-rate EvalEML band: folded static rates
                + trigger-gated terms + value: formula trees; then reduce/allocate)
→ on-device pressure scatter (session values buffer → stencil input buffer)
→ bounded stencil heatmap (seed-then-zero, horizon-capped)
→ Layer-2 Sum reduce → ai_will_do field_urgency EvalEML (authored weights)
→ GPU edge-detected commitment threshold scan (compact event readback only)
→ journaled crossings → authored CommitmentEffectSpec consequence as
  BoundaryRequest::AttachOverlay through the ordinary feeder channel (once-latched)
```

Canonical authored example: `crates/simthing-clausething/tests/fixtures/ct3b4a_headline.clause`.
Canonical proof: `crates/simthing-clausething/tests/ct_3b_4a_session_loop.rs` — the in-loop run
(one journaled crossing under sustained urgency, effect applied exactly once, alarm column
transformed on GPU), the default-off / half-authored-hard-error guards, and the Line 3R edge
test (rise/hold/hold = 1/0/0 crossings; decay below; re-cross fires once more).

## Production APIs (the supported surface)

| Surface | Contract |
|---|---|
| `SimSession::open_from_spec` | Installs `SessionMappingState` iff the game mode authors profile + exactly one region field + pressure binding + ai_will_do weights + commitment threshold. Anything half-authored is a hard open error naming the missing surface. |
| `SimSession::run` / `record_to_path` | Dispatch RF bands and the mapping chain per tick when opted in; journal `SimSession::mapping_commitments`; counters on `RunSummary` (`resource_flow_band_dispatches`, `mapping_ticks`, `mapping_commitment_events`, `mapping_commitment_effects_applied`). |
| `FirstSliceMappingSession::tick_with_commitment_spec` | **The production commitment tick.** Edge-detected: previous threshold state persists GPU-side (`AccumulatorOpSession::copy_values_to_previous` after every scan; zeroed once at first scan only). |
| `compile_arena_pressure_scatter` + `simthing_gpu::IndexedScatterOp` | On-device arena→cell projection (the runtime path). `PressureSourceSpec::Named { sub_field }` is the gadget composition hook: any flow-property column a session EML/gadget op writes is projectable heatmap feedstock. |
| `CommitmentEffectSpec` (v1, closed) | `{ target_id, targets_property, sub_field_deltas, lifecycle (Permanent), once (default true) }` — do not widen without a named consumer and design authority. |
| `ResourceFlowSpec.gated_rates` / `script_value` / `value:` refs | The CT-RF-EML-RATE-0 band: `intrinsic = (base + Σadd×gate)×(1 + Σmult×gate)` recomputed per tick from the immutable `rate_base` column; flat formulas only. |

## Fixture / diagnostic APIs (never call from production code)

| Surface | Why it is not production |
|---|---|
| `tick_with_commitment_spec_fixture`, `tick_with_commitment_threshold_fixture` | Reset the threshold baseline to zero **every** scan (level-triggered). Retained solely for the standing single-tick product fixtures. |
| `diagnostic_readback_reduction_eml`, `FirstSliceTickOptions::debug_readback` | Value readback — diagnostic/oracle seams only; runtime decisions never read values back. |
| `project_arena_pressure_seeds` | The CPU projection — the parity **oracle** for the on-device scatter, and boundary-time diagnostics. |
| `HydratedCategoryEconomyPack::contributions` | Diagnostic mirror of decoded rates; the install path consumes `base_obligations`/`gated_rates`, never this. |

## Standing guardrails (unchanged by closure)

Everything is a SimThing; no noun engines, no CPU planner, no movement/pathfinding/route
constructs; `simthing-sim` is ClauseThing-blind; PALMA/min-plus is a seated generic GPU utility
awaiting a named consumer; GPU sqrt claiming exactness routes through `m_jit_sqrt_f_exact`;
§6 backlog items (SPEC-SCOPE-1/2/3, EffectSpec vocabulary, iterators, control flow,
effect-ordering, ship grammars) open consumer-pulled only; the `modifiers.log` round-trip is the
admission bar for any future corpus-wide decoder claim.
