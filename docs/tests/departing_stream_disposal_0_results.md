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
Preserved RED-A commit: `f724c465`. No production source was changed.

## Preserved RED-B — real partial-membership boundary

`established_partial_departure_terminates_before_survivor_carry` establishes two
ordinary claimants requesting 10 each. After N1 it removes only one claimant's
demand property. Both ResidentRequired and CpuVendorizedOracle reject N2 with
`TemporalSourceMismatch`, leave the complete history unchanged, and emit no
termination or survivor carry. This repeats with supply 4 (both prior U8) and 40
(both prior U0). The four baseline refusals fail the final positive assertion.
The desired path additionally checks a single N2 departing fact and the survivor's
N2 canonical product on N3 termination: G4/U14 with supply 4, G10/U0 with supply 40.

Command: `cargo test -p simthing-workshop --test departing_stream_disposal_0 established_partial_departure_terminates_before_survivor_carry -- --exact --nocapture --test-threads=1`.
Result: exit 101; 0 passed, 1 failed, 1 filtered; four actual session refusals.
The separate RED-B commit precedes all production remedies, independently of
RED-A. No new testing seam or production source was introduced for either RED.

Preserved RED-B commit: `1d2aaf26`.

## Implementation candidate before E8

The new ClauseThing metadata lowerer delegates its value formula to the unchanged
15.3 compiler. `DepartureDispositionBinding` seals the claimant/full-scope key,
existing `AuthoredPersistenceValuation`, destination transform and lifecycle.
`PersistenceDeformationBindings::with_departure_dispositions` admits these values
in the existing session binding vehicle; the deformation iterator cannot expose
them. Existing `install_spec_state` admits their lifecycle shapes into the existing
frozen catalogue after install-time accumulator rebuilding. Its semantic shadow
creates no live Overlay or funded consequence.

The ordinary session compares authorized membership with its one continuation.
It records missing claimants from the already-born history before minting. The
unbound all-depart case keeps the exact graduated aggregate neutral row. Partial
departure and all-depart with authored bindings record per-claimant neutral rows;
only explicitly bound claimants call the unchanged consequence ingress. The
result appends a separate `DepartureConsequence` observation to the same schedule,
carrying the complete originating fact and key, CostBand bits, generation and
Overlay identity. The neutral row remains immutable. Both history row kinds are
excluded from RF reduce-up and standing replay. All new optional serialization
fields disappear when absent; canonical product/status ABI is unchanged.

`SurvivorSubsetPermission` carries identities and full scope, never U. It requires
exact same-generation per-claimant termination facts for every missing source;
wrong scope/source/generation and duplicates refuse. The resident extension
consumes its non-Clone batch ticket, copies survivor products device-to-device,
calls the unchanged 1:1 mint on those selected rows, and combines its output with
fresh entrant rows before one existing exact clear. Selection and policy checks
live in the already-bundled Kernel temporal-transform component; the GPU wrapper
only passes its existing buffers. The permission implementation lives in the
already-bundled Core persistence component. No shader, executable component,
component-list entry or build-script change was introduced.

The CPU extension uses the existing atomic mint authority and full prior clear;
only survivor observations enter the unchanged recurrence. Entrants cannot enter
that door. Missing effective outputs can remain fresh only for explicitly proved
entrants, never by an inferred missing match. Both ordinary session loop bodies
use the same preparation and existing feeder.

Exact function-body comparison against the live base confirms five frozen doors
are byte-unchanged: Driver `prepare_temporal_demands`, Kernel temporal `encode`,
Spec `produce_runtime_rf_next_generation_demands`, Spec
`clear_constrained_claims_at_generation`, and Driver
`submit_authored_persistence_consequence`. The exact-apportionment Rust/WGSL and
temporal WGSL are unchanged. E8's two literals have not yet been rolled at this
candidate stage.

Owner-local prequalification CPU runs exercised 36 mixed cases (3 loops, 2 real
arena/source orders, 6 positive/zero/deformation cases), 27 all-depart cases
(3 loops, 3 authored-binding counts, positive U / satisfied U0 / canonical G0U0),
and 3 actual post-disposition late-refresh failures. Existing lifecycle publication
attaches at N3 and observes dissolution at N7 for authored AfterTicks=3. The
no-fact CPU door refuses before consuming authority; exact recorded permission
then admits survivor demand 18, and a repeated attempt refuses the second mint.
These are preparatory observations, not the final resident/workspace certificate.

The preserved 15.11 partial-departure test keeps its inventory identity. Its
Owner-superseded partial refusal is replaced by successful per-claimant termination;
its invalid Draw and subsequent empty-set fault fence remain actual-session
checks at the next generation. A new direct resident-axis witness separately
exercises both the frozen complete-set door and new subset door without a fact.

Final E8, both-posture focused, frozen, full-workspace, structural, committed-head
scan and hosted certificates remain pending at this candidate stage.
