# CLAUSETHING-ADMISSION-CONVERGENCE-0 — hydration admission convergence

> **Status: PROBATION / proof-present / DA-review-pending.** Coding lane only;
> no merge, graduation, pointer movement, or 13.3+ work is claimed.

**Date:** 2026-08-29  
**Dispatch:** Board `5464509674`  
**Phase-13 authority:** Board `5462798074`  
**13.1 DA stamp authority:** Board `5464215228`  
**ORIENT-RECEIPT:** `21e642408895` (`orientation_rule_stamp=263e1a616adf5c77`)  
**HD-RECEIPT:** `540313f1bb0e`  
**Base:** `b705df0cc69d1ccefed8fb108bbd14b19079add0`  
**Tested code SHA:** `6e0a34d97a8f0f7e060f591398bca0b87cfd4f68`

## Result

ClauseThing now emits the already-canonical modern admission shape at its
existing `hydrate_*` boundary. The generic admission law is unchanged, no
lower crate was edited, and both sealed GPU oracles open the hydrated game mode
without mutating it into acceptability.

The single ordinary target route is:

```text
parse_raw_document
  -> RawDocument
  -> hydrate_category_economy_pack
  -> HydratedCategoryEconomyPack { scenario_registry, game_mode }
  -> Scenario
  -> SimSession::open_from_spec
```

The category lowerer now compiles the scenario registry in primary-production
order, emits native enrollment, balance bindings/roles, and suspended
zero-delta property-host overlays. The resource-flow lowerer emits native
cohort enrollment. The corpus run additionally forced three existing
application-boundary version-skew repairs: legacy daily recipes lower omitted
unit-output/order-band values to their historical defaults, scenario field
operators choose the admitted extended grid profile when their authored grid is
larger than 10, and shipsize gated rates lower to registered arenas/property
roles/standalone overlays with non-colliding capability progress properties.

## Five-shim census

Counting rule: one test-owned pre-open mutation or construction unit; a helper
body collapses to its call-site unit, while semantic scenario construction is
not a shim.

| Pre-change unit | Post-change disposition |
|---|---|
| Precompile hydrated flow properties into the scenario registry | Removed; hydration returns the admitted native registry |
| Materialize explicit RF participants | Removed; arenas carry native `AllOfKind` enrollment |
| Inject each arena's `balance_property` | Removed; hydration emits it |
| Inject `balance_rate` / `balance` helper lanes | Removed; hydration emits them |
| Clear hydrated game-mode properties before open | Removed; the lowerer separates registry sizing from executable bindings |

**Census:** `5 -> 0`. No equivalent target-oracle workaround moved to another
helper or test utility. The assertion bodies after session setup are unchanged.

## Pre-change typed refusals

| Oracle | `law_id` | `element_path` |
|---|---|---|
| `ct_2c_category_economy` | `base-flow-obligation-participant-property-live` | `resource_flow.base_obligations[id="farmer_settlement_food_produce"].participants[id=2].properties[key="simthing::settlement_food_flow"]` |
| `ct_3b_4a_gpu_projection` | `base-flow-obligation-participant-property-live` | `resource_flow.base_obligations[id="farmer_settlement_food_produce"].participants[id=2].properties[key="simthing::settlement_food_flow"]` |

The 13.1 typed error shape and the refusing admission predicate were not edited.

## Tree-derived corpus horizon

The integration witness discovers all direct `tests/fixtures/*.clause` files
and adds the canonical fixture-tree scenario. It asserts the standing census
before classifying by document structure:

| Class | Count | Result |
|---|---:|---|
| Direct ClauseThing fixtures | 41 | discovered |
| Canonical `tests/fixtures/scenario/terran_pirate_galaxy.clause` | 1 | discovered |
| Executable semantic positives | 18 | parse -> production hydrate -> ordinary `preview_install` PASS |
| Parser / expansion / scope stage positives | 19 | parse and stage contract PASS |
| Existing semantic negatives | 5 | exact reason preserved |

Negative reason table:

| Fixture | Preserved reason |
|---|---|
| `bh3_invalid_chi.clause` | `SaturatingFlux chi 0.5 exceeds CFL bound 0.25 (dt=1.0)` |
| `bh3_missing_u_sat.clause` | `missing required field` + authored field identity `u_sat` |
| `ct1a_unsupported_field.clause` | `unsupported entity field` + authored field identity `on_action` |
| `scope_malformed.clause` | `root..owner` and `.from` empty-dot-segment diagnostics |
| `scope_unknown_domain.clause` | `unknown domain scope fictitious_relay_scope (not in validation table)` |

The five excluded repository `.clause` files retain their distinct provenance:
two nested raw MapGen slices, the documentation sample, the shipped-scenario
copy outside the governed fixture tree, and the CI known-bad scanner fixture.
No `.clause` file or parser/grammar surface changed.

## Proof battery

| Proof | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| Corpus witness | PASS — 42 governed / 18 admitted / 19 stage-positive / 5 exact negatives |
| `ct_2c_category_economy` | PASS — real-adapter RF conservation oracle |
| `ct_3b_4a_gpu_projection` | PASS — real-adapter GPU projection oracle |
| Full ClauseThing test battery | All current binaries green except the authorized inherited `studio_star_naming_pass_0` stale golden |
| Test inventory drift/check | PASS — 1,355 discovered and ledgered; 0 missing/stale |
| `agent_scan.sh` | PASS — 0 hard failures, 0 INSPECT |
| `git diff --check` | PASS |

## Containment and debt

The implementation diff is confined to ClauseThing `hydrate_*` modules, the
two sealed-oracle setup sections, the corpus test, and proof ledgers/docs. There
is zero diff under `simthing-core`, `simthing-kernel`, `simthing-gpu`,
`simthing-sim`, `simthing-driver`, `simthing-spec`, or `simthing-embedder`; no
workflow or CI gate code changed.

Optional debt `5460089932` is carried unchanged. This hydration-only repair did
not naturally prove `open_replay_with_spec`, and the rung did not chase it into
a forbidden driver/embedder edit.
