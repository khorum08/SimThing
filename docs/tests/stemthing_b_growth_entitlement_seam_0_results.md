# STEMTHING-B-GROWTH-ENTITLEMENT-SEAM-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2c
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `39f98302bb8ca0b856a0c3d44addf30ace8d2f14`
- DA provenance-remand reviewed head: `e896f07080425f18a88a43fe9a7757521602de67`
- Remand tested-code checkpoint: `0596dd19a914ca94b3c024cc9a6f3013f405333d`
- Branch: `codex/stemthing-b-growth-entitlement-seam-0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- HD-RECEIPT: `7d5052c34f5d`
- Original handoff: Board comment `5417954669`
- Provenance remand: Board comment `5419014773`; governing DA ruling `5419004038`
- Install-door remand: Board comment `5425378390`; governing DA ruling `5419722895`
- Expected route: `DA-RESERVE(gate-wiring)` / `DEEP-TREE`
- Pointer movement: none
- Structural certificate baseline at `e896f070`: 119 suites / 453 passed / 7 failed
  (three known ClauseThing reds plus four install-door regressions)

## Pre-edit authority map

| Required link | Existing authority used |
|---|---|
| complete ordinary candidate batch | boundary lifecycle, fission preparation, and the one drained `BoundaryRequest::AddChild` batch |
| entitlement decision | 11.2a `AdmittedSpecializationFlowMarket`, `clear_constrained_claims_at_generation`, and real `MarketGrantRecord` |
| physical realization | 11.2b `ProvisionalResidencyEntitlement`, level-local placement book, and placement oracle |
| final structural mutation | existing fission fusion resolver and `apply_structural_mutations` AddChild door |
| canonical record | existing 6.1 `IntegrationSchedule`, extended by one typed refusal kind, plus existing delta log |
| authoritative replay | existing replay boundary; recorded growth commits are realized directly and re-clearing is forbidden |
| initial population | `install_initial_tree` permits initial bulk or continuation over the same admitted structural root; presenting a different subtree/root is a typed attached-growth-bypass refusal |

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

## Install-door bounded remand

The certificate found that global allocator emptiness was not the install law:
driver admission legitimately performs several pre-run installation passes over
the same structural root. `install_initial_tree` now checks root identity
continuity instead. No admitted root means initial bulk; the same admitted root
means a lawful installation continuation even when prior rows exist; a different
requested root means an ordinary attached subtree is trying to enter through the
install exception and returns
`SlotAllocError::InstallInitialTreeAttachedGrowthBypass`. The error names the
install-only door, the admitted/requested roots, and the verified ordinary-growth
precondition. Population failures also roll back and return `SlotAllocError`
rather than panicking.

The typed result propagates through driver install, session open, and replay
snapshot construction. Replay's public constructor now returns `ReplayError` and
its production callers propagate or render that error. No population door,
manager, mode registry, policy plane, or history surface was added.

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
| compile/admission seal | low-level residency execution is crate-private; ordinary mutation accepts only `VerifiedGrowthResidencyCommit`; clearing/grant provenance rejects public-field mutation; the old grantless subtree population entry does not exist; initial install permits only initial bulk/same-root continuation and types a different-root bypass; replay is explicitly named |
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
| `install_initial_tree_continues_same_root_and_types_attached_growth_bypass` | global-empty regression, panic-on-door-misuse, failure to continue a legitimate same-root install, or grantless admission of an attached subtree as a second root |
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
cargo test -p simthing-clausething --test capability_effect_host_admission_0 -- --nocapture
cargo test -p simthing-clausething --test capability_prereq_dag_admission_0 -- --nocapture
cargo test -p simthing-driver --lib install::tests::overlay_effect_host_admission_accepts_and_transforms_correct_host -- --nocapture
cargo test -p simthing-kernel --lib install_initial_tree_continues_same_root_and_types_attached_growth_bypass
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

All named commands exit zero locally. The four unmodified certificate regressions
are green: capability-effect host admission is `3 passed; 0 failed`, capability
prerequisite DAG admission is `2 passed; 0 failed`, and the focused driver-lib
test is `1 passed; 0 failed`. The direct typed-door falsifier is `1 passed; 0
failed`. The primary witness is `5 passed; 0 failed` on
the real GPU path; the existing market germ is `4 passed; 0 failed`; existing
VRAM residency is `1 passed; 0 failed`; GPU lifecycle parity is `1 passed; 0
failed`; protected-representative restore is `2 passed; 0 failed`; kernel
rustdoc is `48 passed; 0 failed`. Kernel, sim, and driver test targets compile;
core/kernel/feeder/sim library suites pass. Inventory is exact at `1335/1335`,
and `AGENT-SCAN-VERDICT: PASS` reports zero hard failures and zero inspect flags.
The standalone whole-tree doctrine census reports its pre-existing heuristic
`INSPECT` set (`419`, zero hard failures); the required PR-delta doctrine run
inside Agent scan is `PASS` with zero inspect flags at the tested-code SHA.
Exact tested/head SHAs, PR, and hosted workflow run IDs are carried by the
coding Board return.

## Fences retained

- No allocator-policy retirement (11.2d), Vendor Door work (11.2e), 11.3+, or
  pointer movement.
- No second market, clearing engine, placement manager, schedule, replay recorder,
  telemetry plane, generation authority, retry, or convergence loop.
- Physical free-range/index machinery remains downstream of market entitlement.
- Coding returns at PROBATION; DA alone reviews, graduates, merges, and certifies.
