# rf_interior_policy_composition_0 — the interior-policy law completed

**Status: COMPLETE (DA intervention 2 of the 0.0.8.8 drift audit, live Board `5879126789`; completes
the INTERIOR-POLICY COMPOSITION LAW of #2035/#2036; rung `0088-ECONOMY-FLEET-0`, on HOLD).**

## The defect

#2035 ruled that a policy-bearing interior bids its OWN weight upward and discards its subtree
aggregate. #2036 counted every active weight overlay as policy-bearing. Together they contradicted
the foundation for Add and Multiply policies:

- **The recursive cycle** reduces a subtree UP, applies the overlay modifications, and disburses
  DOWN (0.0.8.7 P0; core §8.4.3's upward statistic `weight_sum`/`P_up`).
- **The RF invariant** says weight columns "default to Demand-proportional and are
  overlay-modifiable via existing Add/Multiply/Set OrderBands".

Shipped content hit it. `pirate_supply_policy` is a standing ×1.5 Multiply on an owner, and owners are
interior RF participants. So each faction's share of anything its root handed down was pinned at
1 : 1.5, whatever its subtree needed.

## The shipped shape decides the mechanism

An owner is a **seat**. It holds no spatial participants. Its sites live beside it under the root
and are its RF children through resource-parent edges, because resource parentage is not
containment. The overlay pass applies a standing policy by tree position, to its host and the host's
physical subtree. A seat's policy therefore reaches none of its RF children, and inheritance alone
would lose it. So the law is placed where the policy's reach ends.

- **Seat (no RF child is a physical descendant).** The host's upsweep writes two sums in one band,
  both reading only its RF children:
  - the plain sum, into `weight_sum`, which stays its own child-share denominator;
  - `ScaleSpec::Constant(factor)` × that sum, into `weight`, which is its participation upward.

  The factor is applied once per generation to a freshly reduced total, so it never compounds.
- **Contained (every RF child is a physical descendant).** Every inheriting leaf already carries the
  factor, and the host's plain total is already factor × total. The host adds nothing.
- **Split reach** (some RF children descend physically, some do not) cannot be applied exactly once.
  It refuses typed: `InteriorWeightPolicyReachSplit`.
- **A standing Add at an RF interior refuses typed:** `InteriorAddWeightPolicy`. Its lawful meaning,
  total + a, needs an affine stage after the reduction. No admitted op provides one: kernel scale
  kinds are Identity and Constant, and the affine-intent combine is ungated, so it is inadmissible in
  RF bands. The only alternative is a new band per level, which renumbers the band layout. Neither
  belongs to this intervention. An Add at a leaf is unchanged.
- **A replacing policy keeps its host's own value upward.** This covers a Set, a non-literal program,
  a routed instruction, and a need binding. Set still replaces.
- **Neutral trees are untouched.** An empty host-scale map is the historical plan, bit-identical.

Code:
- `SimRuntimeTree` gains two observation-only queries:
  - `overlay_transforms_by_host`: hosts only, in stack order;
  - `physical_descendants`.
- `overlay_transform_targets` takes the caller's predicate.
- The driver decides the law in `collect_weight_host_policies` and the per-arena `host_scales`.
- The planner applies it in `plan_arena_allocation_with_policies`.

## Witnesses

`crates/simthing-driver/tests/rf_interior_policy_composition_0.rs` runs the ORDINARY resident path. A
root injects `R` over two owner seats. Terran owns 1 site of 4 cohorts; Pirate owns 2. Every base
weight is 1. The witness checks owner AllocatedFlow.

| case | policies | lawful split | the #2035/#2036 law |
|---|---|---|---|
| seat multiply | Terran ×1, Pirate ×1.5, R=4 | 1.0 / 3.0 (4 : 12), both generations | **1.6 / 2.4** |
| contained multiply | same, sites inside their seats | 1.0 / 3.0, generation 1 | a second host application: 0.73 / 3.27 |
| seat set | Pirate =2, R=6 | 4.0 / 2.0, both generations | 4.0 / 2.0 |
| neutral | none, R=6 | 2.0 / 4.0, both generations | 2.0 / 4.0 |
| seat add / split reach | Pirate +1 / one site contained, one by edge | typed refusal at open | silently pinned |

The classification-only interim that inheritance alone would have given loses the seat policy
entirely: 1.33 / 2.67.

The frozen referee `rehearsal_lifecycle_interior_policy_overlay_0` was corrected to the ruled law at
greater strength. Its test keeps its name and now proves:

- a Set classifies its host and the nodes it affects;
- standing Multiply and Add classify nothing;
- a routed instruction classifies its origin and target;
- wrong property and wrong role never classify;
- the raw query carries no law of its own.

`rf_conservation_any_shape_0` still holds on every generated shape. Its interiors now carry only the
admitted policies (×2, ×0.5, =2), and its shipped owner economy uses the seat topology, so the
host-scale ops are judged by the independent RF-1 check:

```
coverage: 32 trees, depth histogram [7, 7, 6, 6, 6], 80 interiors owning intrinsic flow, 77 sinks, 47 interior weight policies, 3 generations each
test conservation_holds_for_every_generated_tree_shape ... ok
test conservation_holds_for_the_shipped_owner_economy_shape ... ok
```

## Found, not fixed: standing Multiply/Add compound on persistent cells

The overlay OrderBand pass re-applies a standing Multiply or Add to its targets' weight cells every
tick. Those cells persist. A leaf's ×1.5 policy is therefore ×1.5ⁿ by generation n, and so is every
cohort that inherits one. The contained witness stops at generation 1 for this reason. The seat path
is immune, because the reduction rewrites the seat's weight each generation.

The defect predates this law and reaches leaf policies alike. It is reported to the Owner for its
own ruling.

## E8 roll

`arena_allocation_plan.rs`, `arena_allocation_sync.rs`, `session.rs` and `sim_runtime_tree.rs` are
sealed components.

- **Old pin RED.** The first design's referee refused master's `0xef92_b0f2_866c_ef5d`, observing
  `0x8dbf_1614_9872_6591`. The redesign's referee then refused that interim pin, observing
  `0x6d9c_0759_624b_83ca`.
- **Roll.** Both literals now read `0x6d9c_0759_624b_83ca`:
  - `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` in `simthing-gpu/src/resident_clearing_runtime.rs`;
  - `QUALIFIED_RECORD_FINGERPRINT` in `simthing-workshop/tests/resident_clearing_parity_0.rs`.
- **Battery** on NVIDIA 616.92, `--features simthing-gpu/eml-resource-profiling`:

| step | result |
|---|---|
| parity referee | 1/1, `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 6d9c0759624b83ca` |
| score and bands | 3/3 |
| runtime (ABI, child-share, planner and temporal-15.2 mutants) | 4/4 |
| witnesses (composition 5, conservation 2, frozen referee 1) | 8/8 |

CLEAN_CHECKOUT_PROOF
