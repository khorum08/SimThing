# DEPARTING-STREAM-DISPOSAL-0 — implementation evidence

Status: STOP / PROBATION / partial proof present / DA-review-pending / OPEN / UNMERGED.

The candidate does not satisfy the zero-RED exit proof. The post-roll failures and
scope gap below require orchestration/DA disposition before implementation resumes.

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

## Implementation candidate

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
temporal WGSL are unchanged. Semantic commit `f02e217619dff463df5edda3aa43c6c1005e2dda`
precedes final E8 commit `b86dea52050f44f95ca67add69ebf6e4799e727f`.

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

## Final E8 evidence and source freeze

The old fixed pin `0x64c8_2fb4_de76_90ac` refused the final semantic candidate at
`SimSession::open`, before a hot/economic cycle, with `UnqualifiedAdapter`:
required `7262106853007855788`, observed `6628835086103067003`.

Commit `b86dea52050f44f95ca67add69ebf6e4799e727f` changes exactly the existing GPU
qualification literal and workshop parity literal to `0x5bfe_5a63_c5d1_b97b`.
No source, shader, component list or `build.rs` changed in that commit.
The four qualification mutants pass: ABI `3d03935abae26f5a`, child-share
`943734a4e965fa10`, planner `f9802e5b7220eca4`, temporal `7cb0e0e031825516`;
semantic bundle hash `9d5e270534c8f08f`.

No sealed source was edited after the roll. Subsequent work is limited to results,
a diagnostic assertion message in the new workshop witness, and the expressly
admitted constitutional census data. There was no second pin roll.

## STOP findings

1. **Mixed membership numerical disagreement.** The first resident mixed case
   (Step, normal source/arena order, supply4, departed10, survivor10, factor1,
   fresh entrant5) records the expected departure and passes the effective-demand
   assertions: survivor G+U=18 and entrant G+U=5. It then fails the unchanged
   grant assertion: resident survivor G=2, expected G=3. Preparatory CPU execution
   passed all 36 matrix cases with that expectation. The focused run stops this
   matrix at its first resident case; it does not establish the remaining resident
   cases or current-head CPU parity. The cause has not been adjudicated as fixture,
   substrate basis, or membership implementation. No arithmetic or golden was
   changed to erase the failure. A production correction would reopen sealed
   source after the final E8 roll and requires a remedial dispatch.
2. **Frozen single-port census.** Existing
   `persistence_deformation_port_0::structural_census_has_no_second_lane_or_consequence_reinjection`
   fails at line418. Its `single_port` predicate requires one call to
   `carry_unresolved_demand_to_next_generation` in `runtime_rf_tick.rs`; the unchanged
   ordinary door at line245 and the new typed survivor extension at line372 now
   produce two calls. The recurrence implementation is still singular, and both
   original recurrence/mint bodies are byte-unchanged, but this is a real frozen
   witness failure. Neither its count nor planted second-lane mutant was weakened.
   DA must adjudicate whether to restructure the extension or explicitly amend the
   witness while preserving the no-second-lane law.
3. **Overlay census scope gap.** `overlay_germ_archaeology_census_check.sh` fails:
   `UNJUSTIFIED-BROAD: driver:session.rs:admit_departure_lifecycle_catalogue spec:spec/constrained_clearing.rs:overlay`.
   Reconciliation reports routes77/discovery73/residue87/unclassified0/open0 before
   these two unclassified broad hits. The existing checker expects classification
   in `scripts/ci/overlay_germ_archaeology_census.tsv`, which the HD surface list
   omits. The first symbol admits existing lifecycle shapes during install; the
   second is a read-only accessor for the sealed authored Overlay binding. Their
   classification is for orchestration/DA; no census row, exception, checker,
   universe pin or source name was changed to bypass the finding.

4. **Generated digest scope gap.** After the permitted constitutional census
   update, `gen_digest.sh --check` fails because `docs/sanctioned_surface.md`
   still contains the old ledger hash. Its expected diff is one generated hash:
   `86d0e9a8d38665fa0a89bba1acd4cad874e84898d0644dc25ab9716b722db028`
   to `b7041f54ae6649090a36b3c830a5e18789f08b57174a22c351f570ac3f2b4e3b`.
   This generated file is also outside HD surfaces. Hosted Doctrine Scan run
   `34125622770` fails its actual sanctioned-surface freshness step; all later
   doctrine scan/census steps are skipped, so it provides no passing scan proof.
   The uploaded `doctrine-scan-reports` artifact contains the same digest refusal.
   No generated doctrine file was edited outside scope.

The explicitly allowed `constitutional_surfaces.tsv` was updated only to admit the
new ordinary ClauseThing lowerer in `LEGACY-CLAUSETHING-HYDRATOR-SURFACES`. All other
columns and restrictions remain unchanged. This resolves the independent registry
addition failure; constitutional check and planted selftest both pass.

## Validation on the frozen semantic candidate

- All seven touched packages plus tests: `cargo check` PASS.
- E8: actual old-pin refusal, final two-literal commit, mutation4/4 PASS.
- Focused new referee: 6 passed / 1 failed. Authored binding/real consequence,
  established partial U-positive/U-zero, reinjection rejection, exact no-fact
  refusal, 54 all-depart cases, and 6 post-effect fault/retry cases pass. Mixed
  membership remains RED. Existing/new recursion-axis tests: 6/6 PASS.
- Frozen ten-target run: 37 passed / 1 failed. The only failure is the single-port
  census above. Actual persistence deformation identity/decay/saturation/expiry,
  exact cap dual digest, 338 no-collision cases, 3104 capped physical runs,
  apportionment, parity/zero retention, resident filter binding, all ten session
  integration tests, generation fault/seal and execution identity tests pass.
- Spec consequence-only compile-fail docs: 4/4 PASS, including both new binding
  E0308 negatives. Core private schedule construction: 1/1 compile-fail PASS.
- Structural gates: 13/15 PASS after the admitted constitutional ledger update.
  Inventory/drift, constitutional check/selftest, lifecycle schema/prove,
  detachability/check selftest, anchors/check selftest, plan/observation/slot
  censuses pass. Overlay archaeology and generated digest freshness remain RED
  as above. The earlier pre-ledger digest PASS is superseded by the final check.
- Full workspace/all-targets is also run with `--no-fail-fast -j 1 --quiet`;
  its completed totals and hosted step/artifact results are recorded against the
  final commit in the Board return and PR packet. Zero-RED is not claimed.
- Hosted Doctrine Exec run `34125622893` executes the `ci-b-webchat-smoke`
  profile successfully; actual execution and stale-report steps both pass, with
  plan-only mode skipped. This smoke result does not exercise the failing GPU
  matrix or replace the full workspace certificate. Final-head run bindings are
  carried in the Board/PR packet after the documentation update.
- Committed candidate Agent Scan at `f02e2176`: reliable hard failures0,
  `TEST-BUDGET` INSPECT for seven new test functions; anchor-reach gate-wiring
  notice. This is not final-head clearance. Final-head and hosted reports belong
  in the Board return and PR packet; no scan-id-bearing finding is self-triaged.

## Authority census and return boundary

One existing Draw authority admits the current claims. One resident constrained
clearer settles them. One `IntegrationSchedule` stores existing canonical history,
neutral terminations and optional consequence proofs. One continuation enum/lane
holds each stream. One `TreeExecutionLease` and private generation permit protects
termination, disposition, mint and clear. One unchanged consequence ingress funds
CostBand and routes the authored Overlay. The typed membership permission contains
identities and provenance permission only, not economic U or another executor.
The frozen single-port census and Overlay classification failures are unresolved
limits of this candidate, not a passed authority certificate.

No merge, self-triage, clearance/relay-lint, graduation/pointer movement, closeout,
engineering re-review, canon/Unification update or successor work was performed.
Return this head to orchestration/DA with all failures intact.


## Changed-file ledger

- `crates/simthing-clausething/src/hydrate_shipsize_decoder.rs`
- `crates/simthing-clausething/src/lib.rs`
- `crates/simthing-core/src/generation_stamp.rs`
- `crates/simthing-core/src/lib.rs`
- `crates/simthing-core/src/persistence_deformation.rs`
- `crates/simthing-driver/src/growth_entitlement.rs`
- `crates/simthing-driver/src/resident_clearing_runtime.rs`
- `crates/simthing-driver/src/session.rs`
- `crates/simthing-gpu/src/resident_clearing_runtime.rs`
- `crates/simthing-kernel/src/resident_recursive_intake_transform.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/src/spec/constrained_clearing.rs`
- `crates/simthing-spec/src/spec/mod.rs`
- `crates/simthing-spec/src/spec/owner_channel_rf.rs`
- `crates/simthing-spec/src/spec/runtime_rf_tick.rs`
- `crates/simthing-workshop/tests/departing_stream_disposal_0.rs`
- `crates/simthing-workshop/tests/recursion_axis_conformance_0.rs`
- `crates/simthing-workshop/tests/resident_clearing_parity_0.rs`
- `crates/simthing-workshop/tests/resident_session_integration_conformance_0.rs`
- `docs/tests/departing_stream_disposal_0_results.md`
- `scripts/ci/anchor_reach_log.tsv`
- `scripts/ci/constitutional_surfaces.tsv`
- `scripts/ci/test_inventory.tsv`
