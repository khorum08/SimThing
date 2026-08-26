# STEMTHING-B-GROWTH-ENTITLEMENT-SEAM-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2c
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `39f98302bb8ca0b856a0c3d44addf30ace8d2f14`
- DA-remand reviewed head: `78606e7a17d4b9ee00bb88b562ad5bf8e2af363b`
- Remand tested-code checkpoint: `TESTED_CODE_SHA_PENDING`
- Branch: `codex/stemthing-b-growth-entitlement-seam-0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- HD-RECEIPT: `7d5052c34f5d`
- Original handoff: Board comment `5417954669`
- Remand: Board comment `5419014773`; governing DA ruling `5419004038`
- Expected route: `DA-RESERVE(gate-wiring)` / `DEEP-TREE`
- Pointer movement: none
- Structural certificate: owed at graduation because the ordinary tick growth path changed

## Pre-edit authority map

| Required link | Existing authority used |
|---|---|
| complete ordinary candidate batch | boundary lifecycle, fission preparation, and the one drained `BoundaryRequest::AddChild` batch |
| entitlement decision | 11.2a `AdmittedSpecializationFlowMarket`, `clear_constrained_claims_at_generation`, and real `MarketGrantRecord` |
| physical realization | 11.2b `ProvisionalResidencyEntitlement`, level-local placement book, and placement oracle |
| final structural mutation | existing fission fusion resolver and `apply_structural_mutations` AddChild door |
| canonical record | existing 6.1 `IntegrationSchedule`, extended by one typed refusal kind, plus existing delta log |
| authoritative replay | existing replay boundary; recorded growth commits are realized directly and re-clearing is forbidden |
| initial population | one-shot, empty-allocator-only `install_initial_tree` |

No mapped link was replaced. The missing node was a typed, consumed product joining
the graduated grant and placement authorities to the ordinary structural door.

## Target 1 DA-remand closure

The remand found that a caller could construct a bare
`ProvisionalResidencyEntitlement` key, obtain a public 11.2b raw commit, and feed
that commit into ordinary mutation. The repaired authority chain is now:

`clear_constrained_claims_at_generation` -> sealed `ConstrainedGrant` ->
`MarketGrantRecord` -> opaque `MarketGrantResidencyProvenance` -> driver
entitlement decision -> boundary full-field provenance comparison -> 11.2b raw
placement commit -> `VerifiedGrowthResidencyCommit` -> ordinary fission/AddChild.

The constrained-clearing product retains its public observation fields for the
accepted 11.2a API, but carries a private snapshot seal over every field. Both
initial recording and renewal reject a cloned-and-mutated product as
`InvalidClearingSeal`. `MarketGrantResidencyProvenance` has private
representation and is projected only from the resulting private-field
`MarketGrantRecord`. The ordinary mutation signatures no longer accept a raw
`GrowthResidencyCommit`; their only production input is the private-field
verified wrapper minted by matching granter, grantee, stable grant key,
quantity, and generation against that opaque provenance. Initial install and
authoritative replay keep their already named, separate exceptions.

A provenance mismatch becomes typed U before placement: exactly one
`GrowthEntitlementRefusal` schedule row and one refusal fact, no attach, no
allocator slot, no committed residency row, no same-generation retry, and an
explicit next-generation revaluation. This closes only remanded Target 1;
accepted Targets 2-8 and their semantics remain frozen.

## Landed seam

`GrowthResidencyCommit` carries the structural parent, the real provisional
entitlement, and the committed 11.2b placement. The ordinary boundary now:

1. drains all AddChild requests and prepares all fission candidates without mutation;
2. sorts the complete mixed batch by stable logical identity and resolves it once;
3. clears authored claims through the installed 11.2a market and records ordinary U;
4. commits every accepted 11.2b placement before any structural attachment; and
5. consumes the commit while resolving fission or applying AddChild.

An unauthored session installs a degenerate root standing market through the same
11.2a admission and clearing code. It produces ordinary `MarketGrantRecord` values;
there is no default grant or placement bypass.

## Signal matrix

| Signal | Observed result |
|---|---|
| implicit AddChild success | accepted claim carries a real market key and committed extent; placement precedes attach; the child receives one row and GPU buffer sizing follows the committed extent |
| ordinary refusal | partial or zero clear records one refusal fact, preserves U, names the next generation, attaches nothing, and mints no row |
| real fission | rebellion prepares a candidate before mutation, then clears, places, and resolves with the exact consumed commit |
| mixed oversubscription | one fission plus one AddChild batch at insufficient capacity yields identical logical grants, refusals, placements, and schedule facts under reversed request order |
| compile/admission seal | low-level residency execution is crate-private; ordinary mutation accepts only `VerifiedGrowthResidencyCommit`; clearing/grant provenance rejects public-field mutation; the old grantless subtree population entry does not exist; initial install is one-shot; replay is explicitly named |
| replay | accepted recorded commits realize exact rows without clearing; recorded refusal reproduces exactly; a re-clear attempt returns `ReplayReclearForbidden` |

## Standing falsifiers

| Test | Biting failure |
|---|---|
| `implicit_root_market_add_child_refusal_and_replay_use_one_authority_chain` | implicit-market bypass, attach-before-placement, refusal row mint, lost U, same-generation retry, replay re-clear, or second schedule |
| `real_fission_clears_places_then_attaches_through_the_implicit_market` | grantless fission or loss of the real grant/placement chain |
| `oversubscribed_mixed_batch_is_permutation_independent_through_schedule_and_placement` | vector, event, free-list, or arrival-order authority over a mixed batch |
| `grantless_ordinary_add_child_is_rejected_at_the_only_structural_door` | structural attachment or row residency without the consumed commit |
| `fabricated_market_grant_key_is_typed_refusal_without_attach_row_or_retry_and_revalues_next_generation` | the exact remanded bare-key path reaching ordinary mutation, duplicate refusal facts, attach/row mint on refusal, same-generation retry, or lost next-generation revaluation |
| `cleared_entitlement_places_locally_refuses_to_u_then_revalues_and_relocates` | cloned-and-mutated `ConstrainedGrant` minting a market record/provenance, or any accepted 11.2b behavior changing |
| kernel rustdoc compile-fail seals | public low-level residency execution or restoration of grantless `populate_subtree` |

## Authority census

- Clearing remains `clear_constrained_claims_at_generation`; the binding invokes it
  once for the complete canonical batch.
- Grant identity remains `MarketGrantRecord`, created only from an actual cleared
  constrained grant.
- Placement remains the 11.2b level-local book/oracle; no global extent scan or
  retry loop was added.
- History remains the existing integration schedule, delta log, and replay door.
  Compatibility and lifecycle preflight schedules are discard-only scratch values;
  they are never retained, replayed, or consulted as an authority.
- Generation remains the existing session day/generation.
- Registry-column append and GPU buffer sizing remain infrastructure downstream of
  placement and do not decide entitlement.

## Focused evidence

```text
cargo test -p simthing-driver --test stemthing_b_growth_entitlement_seam_0
cargo test -p simthing-driver --test stemthing_b_flow_market_germ_0
cargo test -p simthing-driver --test stemthing_b_vram_residency_0
cargo test -p simthing-sim --lib
cargo test -p simthing-sim --test gpu_overlay_lifecycle_oracle_parity_0
cargo test -p simthing-sim --test protected_representative_restore
cargo test -p simthing-kernel --doc
cargo check -p simthing-spec
cargo check -p simthing-driver --tests
cargo test -p simthing-sim --tests --no-run
bash scripts/ci/test_inventory_check.sh
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/agent_scan.sh
bash scripts/ci/doctrine_scan.sh
```

All named commands pass locally. The primary witness is `5 passed; 0 failed` on
the real GPU path; the existing market germ is `4 passed; 0 failed`; existing
VRAM residency is `1 passed; 0 failed`; GPU lifecycle parity is `1 passed; 0
failed`; protected-representative restore is `2 passed; 0 failed`; kernel
rustdoc is `48 passed; 0 failed`. Kernel, sim, and driver test targets compile;
core/kernel/feeder/sim library suites pass. Inventory is exact at `1334/1334`,
and `AGENT-SCAN-VERDICT: PASS` reports zero hard failures and zero inspect flags.
Exact tested/head SHAs, PR, and hosted workflow run IDs are carried by the
coding Board return.

## Fences retained

- No allocator-policy retirement (11.2d), Vendor Door work (11.2e), 11.3+, or
  pointer movement.
- No second market, clearing engine, placement manager, schedule, replay recorder,
  telemetry plane, generation authority, retry, or convergence loop.
- Physical free-range/index machinery remains downstream of market entitlement.
- Coding returns at PROBATION; DA alone reviews, graduates, merges, and certifies.
