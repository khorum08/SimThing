# RESIDENT-SESSION-INTEGRATION-CONFORMANCE-0 — implementation evidence

Status: **PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED**. This is the coding return, not a graduation or pointer change. Hosted identifiers and the immutable return head are recorded in the PR/board relay.

## Authority and ingress

- Base: `79f0f6c84a5b0d66367edf9d253c6e7b9904e5fc`.
- Branch: `codex/resident-session-integration-conformance-0`.
- Original dispatch: [5549356007](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549356007).
- Conforming archaeology STOP: [5549449875](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549449875).
- Binding DA ruling: [5549583264](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549583264), rulings 1–7.
- Resumed implementation dispatch: [5549605185](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549605185).
- Same handoff: `handoffs/RESIDENT-SESSION-INTEGRATION-CONFORMANCE-0.hd.md`.
- `HD-RECEIPT: cdb03115f4b9`.
- Coding `ORIENT-RECEIPT: 8a4a068b3fe7`; rule stamp `1e2dcea8714428d2`.
- Orientation digest: `cefb68681891d3fafdc03ccdae0c16c7b224f83a34cf759bfe289d17c3d6e1e4`.
- All 63 machine REQUIRED-ANCHOR acknowledgments and original archaeology are retained in [the resolved STOP packet](resident_session_integration_conformance_0_stop_packet.md). The resumed session did not reorient.

## Ordinary execution, from authorship to settlement

The existing admitted market remains `GrowthEntitlementMarketBinding`. Its name predates recurring flow; it now composes the two adjudicated claim lifecycles over that same market. There is no second admitted market or numerical operator.

```text
SimSession::step_once / run / record_to_path
  ordinary hot cycle -> real arena RF bands -> boundary N
  session.execution_lease.begin_generation(N)
  BoundaryProtocol::execute_with_boundary_hook_and_growth
    existing spec boundary handlers
    authorize_current_flow(kind-free current runtime-tree view)
      admitted scope boundary + persistent claimant + inherited owner
      OWNER_FLOW_DEMAND_PROPERTY_ID + OWNER_FLOW_PRIORITY_PROPERTY_ID
      RuntimeOwnerSiloDemandBucket
      authorize_demand -> admitted market.authorize_draw
      ConstrainedClaim from the existing authorized demand
    settle_boundary_claims, under that same N permit
      if a prior flow ticket exists:
        prepare_temporal_demands(previous ticket, Current N authored demand)
          existing resident once-mint reads previous T_s.U on-device
          canonical resident T_d = authored + f(U)
      existing structural resolve_batch_resident, with only structural candidates
      dispatch / dispatch_temporal for the recurring exact-flow claims
        real arena allocated-flow cells -> existing exact projection
        canonical T_s -> same resident live-head append
      materialize current flow product to schedule (observation only)
      retain resident ticket, not host U
  existing structural rebind when necessary, preserving executor/live head
  session.execution_lease.finish_generation(N, N+1)
```

The unregistered authored metadata is read through `SimRuntimeTree::property_on_node`; ownership uses the existing `resolve_owner` through a kind-free accessor. The driver sees no raw semantic-kind tree. The scope comes from the admitted market, and the source is the demand-bearing node's persistent id. An admitted owner-silo's existing `OWNER_SILO_CURRENT_PROPERTY_ID` supplies the exact current `ConstrainedSupply` amount; it is not the structural subtree quantity or a host reconstruction of U. The existing live RF allocated-flow column supplies the resident exact basis.

The source/scope key remains `(claimant, owner_ref|resource_key|scope_id)`. The ordinary session never maps previous claimants to new identities. The graduated temporal source matcher remains unchanged. No partial-set matcher or tombstone row is introduced. An empty current stream drops its continuation; partial source-set changes still reach the existing typed refusal. The exit referee has one continuous claimant, including its mixed structural test (the newly placed child has no owner-flow demand).

`authorize_demand` is the single Draw composition used by recurring flow before posture selection and by the existing CPU structural oracle. Recurring CPU-oracle continuation calls the existing driver `produce_runtime_rf_next_generation_demands_for_tick` door. Its replay inputs remain in the explicitly selected CPU posture; they never supply resident economics. There is no fallback from resident failure to CPU.

## Current/next authority table

| Point | Authored datum | U authority | Permit / effect |
|---|---|---|---|
| Before first boundary | Claimant authored 10 | No preceding flow ticket | No temporal economics |
| Seal Current N | Draw reads and authorizes 10 | Exact resident settlement creates G4/U6 | Session's N permit; one canonical append |
| After finish N | Claimant is authored to 2 through the existing tree property authoring door | N ticket retains resident provenance | N is consumed; no N+1 prepare or execution |
| Seal Current N+1 | Same Draw reads and authorizes 2 | N ticket/live-head U6 | Distinct N+1 permit authorizes late once-mint |
| N+1 allocation | Existing T_d holds 8 or 5 | Existing resident mint applied identity or half | Same N+1 permit executes actual exact-flow settlement |

The direct-runtime preparation-at-N API remains usable by the unchanged frozen referees. Permit validation still refuses consumed, wrong, foreign, and stale capabilities. Ordinary session code always calls the late mint with the current N+1 permit; it never saves or reuses the N permit.

## Shared live-head ordering and structural isolation

Observer materialization releases an epoch's live-head reservation. Consequently, merely retaining an observed ticket while allowing another batch to overwrite its segment would corrupt temporal provenance. The ordinary boundary therefore submits the previous-flow mint **before** any structural append, then settles structural candidates, and appends the new recurring-flow product last. Queue ordering protects the already-minted resident demand from subsequent segment reuse. Rebind retains that live head.

The three-generation mixed referee inserts a real structural AddChild at N+1 and proves flow requested amounts `10, 8, 6`, with unresolved `6, 4, 2`, across real permits and topology rebind. This is an ordering repair within the existing buffers, not a new retained-U store or abort mechanism.

`record_resident_structural_grant`, `OrdinaryGrowthCandidate` quantities, all-or-nothing placement validation, and the placement/refusal lifecycle are unchanged. Recurring demand and temporal products never enter the structural recorder. The pure 15.8 transcript has no placement or growth-refusal schedule rows. Structural unresolved does not become a recurring demand.

## Persistence admission

```text
ClauseScript script_value
  compile_persistence_deformation_script_value
  existing PersistenceDeformationProgram
  PersistenceDeformationBinding(full scope, claimant, program)
  PersistenceDeformationBindings::admit
  SpecSessionState.persistence_deformations
  GrowthEntitlementMarketBinding::persistence_deformations (same full scope)
  SimSession::admit_resident_clearing_for_market
  existing admit_sealed_market_with_persistence_deformations
  existing plan-bound persistence port -> resident once-mint
```

The unconditional session `&[]` is gone. Tick-zero spec installation rebuilds admission before execution; later spec replacement cannot change persistence policy. Ordinary topology rebind does not rebuild the runtime. Absent bindings use the graduated bit-exact identity. The referee compares absence with an explicitly ClauseThing-compiled identity, then compiles `base = 0.5` through that same ClauseScript door. All three use the actual session constructor/admission path.

## Actual-session economic transcript

Each paired permutation uses a clone of the same authored scenario, including identical claimant id and full scope. Fresh sessions have distinct execution realms as required by 15.7. Within each session, realm/incarnation, claimant, and scope remain unchanged across N/N+1.

| Policy | N authored | N exact T_s | N+1 authored | Effective N+1 | N+1 exact T_s | A,B / B,A |
|---|---:|---|---:|---:|---|---|
| Absent | 10 | G4/U6 | 2 | 8 | G4/U4 | Same |
| Authored Clause identity | 10 | G4/U6 | 2 | 8 | G4/U4 | Same |
| Authored Clause half | 10 | G4/U6 | 2 | 5 | G4/U1 | Same |
| Recording path, Clause half | 10 | G4/U6 | 2 | 5 | G4/U1 | B,A also passes |

Every row crosses actual session generations 1 and 2. The N+1 ordinary RF allocated-flow cell is finite and positive (`1` in this fixture), and exact settlement consumes that real arena cell. Effective demand is verified by `G + U` from the canonical resident product, not a printed host arithmetic surrogate. The supply remains the authored current amount 4 in this witness.

Host-U economic-use count: **0**. Resident continuation holds only the existing dispatch ticket. The CPU observer return from `materialize` is discarded after schedule recording; it never populates authored demand, available supply, policy weight, a carry vector, or temporal execution rows. The resident mint reads the existing product segment on-device. The CPU oracle is explicitly separate and is not counted as resident production.

## Arena resolution

| Existing identity | Resolution | Failure |
|---|---|---|
| Explicit preferred arena | Exactly one admitted descriptor with that name | Zero/multiple matches typed-refuse |
| No preferred arena | Exactly one flow property's canonical `namespace::name` equal to the admitted market resource | Zero/multiple matches typed-refuse |

Physical indices are recovered only after the unique semantic match. No scorer, ranking, compatibility fallback, or index-zero default is present. The actual-session referee builds two valid arenas over independently registered flow properties, permutes their descriptors and participant indices, and proves the same market's results in both orders. An unmatched resource gets `MarketCannotLower` with zero matches in both orders.

## Authority, qualification, and validation

The unchanged constitutional census passes: resident production authority **1**, CPU oracle doors **5**, CPU oracle call-site families **2**, peer-runtime residue **2**, duplicate settlement **0**, economic adapter **0**, global coupling **0**, private field solver **0**. One resident runtime, one exact projection, one integration schedule/live head, one execution lease, and one whole-generation permit lifecycle remain.

Qualification includes the new boundary/source/persistence composition in the existing build-time component bundle. `crates/simthing-gpu/build.rs` additionally names the session, market binding, spec state, boundary, kind-free property/owner reader, existing Draw implementation, demand datum, binding table, owner resolver, and Clause persistence compiler. This build-file edit is a necessary companion to preserving the handoff's complete-bundle requirement; the original path list named GPU `src/**` but omitted `build.rs`. No CI/workflow gate code is changed. The observed qualified production fingerprint is `d06d2691e0a9d732`; the old seal failed closed after source changes, before execution. Runtime compiler/filesystem discovery remains absent.

Focused proof: new 15.8 referee **5 passed**; unchanged 15.5 **5 passed**, unchanged 15.6 **4 passed**, unchanged 15.7 **4 passed**. The final full-workspace, structural, committed-delta, and hosted certificate is recorded with the immutable implementation head in the PR/board return. No full-zero-red or hosted success is claimed before those runs finish.

Changed surfaces are the driver market/session/runtime/spec carrier, kind-free sim boundary/property readers, existing spec persistence table reader, GPU build qualification and pin, one workshop referee, this evidence and historical STOP packet, test inventory, and append-only anchor reach ledger. Test budget deltas and any other INSPECT results are returned for orchestration triage; coding does not author triage rows.

## Graduation routing

Expected route: **DA-RESERVE(binding)**. Orchestration independently reviews the substantive diff, handles INSPECT triage, and runs fresh exact-head `/clearance` and `/relay-lint` after this coding return. DA graduation remains pending. 15.9 cap redistribution, 15.10 abort safety, canon rewrite/adoption, marker deletion, pointer movement, graduation, and merge are outside this implementation.
