# PRODUCTION-PATH-0080-0 - Local Patrol Economy Production-Path Opening Spec

**Status:** `IMPLEMENTED / PASS - Local Patrol Economy opt-in production path`

| Field | Value |
|---|---|
| Production-path gate | `PRODUCTION-PATH-0080-0` |
| Scenario | `Local Patrol Economy` |
| Accepted scenario gate | `SCENARIO-0080-0` |
| Consumed substrate | `0.0.7.9 mobility/transfer substrate` |
| Decision source | Accepted GPU-resident FIELD_POLICY `Threshold` + `EmitEvent` -> `BoundaryRequest` |
| Runtime target | First non-test-support default `SimSession` path, scoped only to Local Patrol Economy |
| Implementation status | Implemented in `PRODUCTION-PATH-0080-0` implementation PR |

`SCENARIO-0080-0` is accepted. `PRODUCTION-PATH-0080-0` is implemented as a narrow,
opt-in/default-off Local Patrol Economy production path. The implementation report is
[`../tests/phase_production_path_0080_0_impl_results.md`](../tests/phase_production_path_0080_0_impl_results.md).
This update does not expand the opening scope.

## Implementation Result

The implementation adds a `simthing-driver` production-path surface for Local Patrol Economy. It
instantiates only under explicit opt-in, accepts a FIELD_POLICY `Threshold` + `EmitEvent` ->
`BoundaryRequest`, delegates relocation through the 0.0.7.9 mobility/transfer substrate, preserves
patrol identity and owner overlay continuity, and updates bounded local economy participation.
`DEFAULT-SCHEDULE-0080-0`, gameplay, semantic WGSL, ClauseThing/L3, hard currency, markets/trade,
`ai_budget`, nested Resource Flow, capture-as-reparenting, owner-entity-as-spatial-parent, invariant
edits, and passive proof wrappers remain closed.

## Product Scenario Summary

Local Patrol Economy is a small product scenario with two or a few local locations, one owner, and
one or a small fixed number of patrols. Each location has bounded local supply, maintenance, output,
security, and disruption values. Patrol relocation changes membership and local economy
participation while preserving entity identity, owner relation, and latched owner overlays.

## Production-Path Purpose

The production need is to move from test-support-only substrate proof to the first non-test-support
default `SimSession` path for this named scenario. The path is scoped only to Local Patrol Economy.
It does not authorize a general production mobility runtime, gameplay UI, semantic WGSL, markets,
hard currency, nested Resource Flow, or broad economy architecture.

## Minimum Implementation Slice To Authorize Next

The next PR may add a default-off / opt-in Local Patrol Economy production-path fixture or narrow
`SimSession` surface. It may instantiate the Local Patrol Economy scenario in a non-test-support
path. It may route accepted FIELD_POLICY `Threshold` + `EmitEvent` -> `BoundaryRequest` into the
mobility/transfer substrate, apply identity-preserving relocation, preserve owner overlay
continuity, and update bounded local economy participation after relocation.

The slice must remain reversible and scenario-scoped. It must not become a general mobility runtime.

## FIELD_POLICY Decision-Source Contract

Patrol relocation is not externally scripted and is not CPU-planned. The decision source is
GPU-resident FIELD_POLICY: a `disruption` / `local_security` threshold crossing emits an event, the event
materializes as a `BoundaryRequest`, and the mobility substrate consumes that request.

This does not open a new FIELD_POLICY substrate. It does not allow CPU planner, CPU urgency, or CPU
commitment emission.

## Mobility / Ownership / Flow Contract

ALLOC / REENROLL / IDROUTE provide deterministic movement, identity, and route handling. OWNER
preserves owner relation and latched overlays after relocation. ECON handles bounded local economy
reassociation: the source location stops counting patrol participation after the move, and the
destination location starts counting patrol participation after the move.

Owner-entity as spatial parent is not authorized. Capture-as-reparenting is not authorized.

## Basic Economy Bounds

Allowed local values only:

- `supply`
- `maintenance`
- `local_output`
- `local_security`
- `disruption`

Excluded:

- hard currency
- market
- trade
- nested Resource Flow
- multi-level fanout
- `ai_budget`
- policy overlays
- multi-faction economy
- Hybrid-Strata/faction-index scaling

## Default Behavior

The future implementation may be opt-in / default-off at first. This opening spec authorizes the
first non-test-support path only for Local Patrol Economy. It does not authorize a global default
schedule; `DEFAULT-SCHEDULE-0080-0` remains closed. Gameplay remains closed. Semantic WGSL remains
closed.

## Required Tests For The Future Implementation PR

- `production_path_0080_0_explicit_opt_in_only`
- `production_path_0080_0_no_global_default_schedule`
- `production_path_0080_0_instantiates_local_patrol_economy`
- `production_path_0080_0_field_policy_threshold_emits_boundary_request`
- `production_path_0080_0_no_cpu_planner_or_external_move_script`
- `production_path_0080_0_identity_preserved_after_relocation`
- `production_path_0080_0_source_membership_updates`
- `production_path_0080_0_destination_membership_updates`
- `production_path_0080_0_owner_overlay_persists_after_move`
- `production_path_0080_0_source_economy_stops_counting_patrol`
- `production_path_0080_0_destination_economy_starts_counting_patrol`
- `production_path_0080_0_bounded_local_economy_only`
- `production_path_0080_0_rejects_capture_as_reparenting`
- `production_path_0080_0_rejects_owner_entity_as_spatial_parent`
- `production_path_0080_0_rejects_nested_transfer`
- `production_path_0080_0_rejects_hard_currency_markets_trade_aibudget`
- `production_path_0080_0_no_semantic_or_raw_wgsl`
- `production_path_0080_0_no_gameplay_surface`
- `production_path_0080_0_no_clausething_dependency`
- `production_path_0080_0_replay_deterministic`
- `production_path_0080_0_docs_status_matches_gate`

## Future Implementation Command Guidance

Do not run these in this docs-only PR. The future implementation PR is expected to run:

- targeted new production-path tests
- relevant existing 0.0.7.9 mobility/transfer regression tests
- FIELD_POLICY threshold/event/boundary regression tests, if present
- `cargo check --workspace`

## Stop Conditions

The implementation gate must stop if it would require:

- global default schedule
- gameplay UI
- semantic/raw WGSL
- CPU planner / urgency / commitment emission
- hard currency
- markets/trade/`ai_budget`
- nested Resource Flow
- capture-as-reparenting
- owner-entity as spatial parent
- ClauseThing implementation
- `simthing-spec` alteration for ClauseThing
- invariant edits
- passive proof wrappers
- reopening closed ladders

## Exit Criteria For Opening Spec

This docs PR is complete when:

- opening spec exists
- scope is Local Patrol Economy only
- implementation slice is named but not implemented
- required future tests are named
- production track marks `PRODUCTION-PATH-0080-0` as implemented/pass with this spec and the
  implementation report linked
- mapping guidance and worklog are updated
- no code changed
