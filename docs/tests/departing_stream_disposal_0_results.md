# DEPARTING-STREAM-DISPOSAL-0 — implementation evidence

Status: PROBATION / proof-in-progress / DA-review-pending / OPEN / UNMERGED.

Dispatch: Board comment 5569664053; Owner mint 5569511979. Canonical handoff:
`handoffs/DEPARTING-STREAM-DISPOSAL-0.hd.md`. Implementation base:
`339d688a23f2817da39ce0fc1abd21f7460d2163` (PR #1994); handoff mint base:
`141ce47874a9894879ed6a8800dadda9cfd92236`.

HD-RECEIPT: 893598d4d3df
ORIENT-RECEIPT: a9d2086a0dd2
role: coding
orientation_rule_stamp: e6709eccedfe50cb
orientation_digest_sha: 4cb2adf3f0ec65fb163dc27005deb14360b30cb0a4541993c82aac997570f39f

The fresh orientation was explicitly dispatched. The renderer requires 63 anchors
(the Board's stated count of 57 is stale); all 63 were queried against the admitted
surfaces before implementation. The existing reach ledger records this query.

## Archaeology before remedies

| Existing seam | Production path and constraint |
| --- | --- |
| 15.2 authoring/admission | ClauseThing `hydrate_shipsize_decoder.rs`: `compile_persistence_deformation_script_value` and the shared EML value lowerer. Spec `constrained_clearing.rs`: `PersistenceDeformationBinding` and `PersistenceDeformationBindings::admit`, keyed by full scope and claimant. Session freezes the installed bindings before execution. |
| 15.3 consequence ingress | ClauseThing `compile_persistence_consequence_script_value` produces `AuthoredPersistenceValuation`. Driver `submit_authored_persistence_consequence` calls Spec `fund_unresolved_persistence` (later generation, valuation, existing CostBand, authored `PersistenceOverlayBinding`), then `RoutedOverlayDelivery::admit(...).submit_boundary(...)`. Consequence has no demand output. |
| 15.8 complete-set once-mint | Driver `growth_entitlement.rs::settle_boundary_claims` retains one continuation. Resident `prepare_temporal_demands` checks the complete sorted source set before the device mint; CPU checks the complete source set before `produce_runtime_rf_next_generation_demands_for_tick`, whose Spec authority atomically consumes the sole Current-to-Next mint. Existing direct doors must retain default refusal. |
| 15.11 neutral observation | Empty authorized membership reads the resident continuation's already-materialized history span or CPU continuation's already-born final products. `IntegrationSchedule::record_neutral_stream_termination` sorts full claimant products and appends the neutral history row; continuation becomes Empty. Zero-valued present claimants remain members. |
| 15.10 effect ordering | `TreeGenerationPermit::authorize_economics` precedes termination, continuation take, mint and clear. Both session loop bodies finish the existing lease only after boundary effects and economy sync. A touched failed generation faults on permit drop; retry cannot begin another hot cycle. |
| Existing routed lifecycle admission | Session boundary stages Overlay attachment through the frozen lifecycle catalogue before tree mutation; standalone ingress submission is not evidence of session attachment. Final disposition proof must establish actual session admission and activation. |
| E8 | Existing GPU `build.rs` component list includes ClauseThing lowerer, Spec constrained clearing, Driver growth/session/resident runtime, Kernel temporal transform Rust/WGSL and GPU clearing plan. Any semantic edits require old-pin refusal and the admitted final two-literal roll; no component-list change is authorized. |

## Preserved RED-A — real authoring door

Before production edits, the new actual-session witness
`authored_departure_binding_reaches_the_existing_consequence_ingress` runs both
ResidentRequired and CpuVendorizedOracle. Each establishes authored demand 10,
available supply 4, and then removes the claimant's demand property through the
existing admitted runtime-tree swap. N2 records canonical final N1 G4/U6 and no
automatic Overlay. A plain authored valuation compiles, and the existing 15.3
ingress consumes that recorded U, funds CostBand n=3 and queues exactly one routed
Overlay with authored target/lifecycle and generation 2. This explicit ingress
control is not automatic disposition consumption.

The same real ClauseScript compiler rejects the formula carrying authored
departure scope/claimant/destination/lifecycle metadata:
`unsupported script_value field departure` (token 12), in both postures. The
test's final assertion fails on these two authoring errors. This is a runtime
authoring refusal, not a source-string absence or compile-failure seam.

Command: `cargo test -p simthing-workshop --test departing_stream_disposal_0 authored_departure_binding_reaches_the_existing_consequence_ingress -- --exact --nocapture --test-threads=1`.
Result: exit 101; 0 passed, 1 failed; both real-session and ingress controls pass.
The first RED commit contains this witness before any remedy. RED-B and remedies
remain pending at this evidence stage.
