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

## The law: a policy deforms its host's rolled-up total exactly once

An owner is a **seat**. It holds no spatial participants. Its sites live beside it under the root
and are its RF children through resource-parent edges, because resource parentage is not
containment. The overlay pass applies a standing policy by tree position, to its host and the host's
physical subtree. A seat's policy therefore reaches none of its RF children, and inheritance alone
would lose it. So the law is placed where the policy's reach ends.

- **Seat (no RF child is a physical descendant).** The host's standing Multiply/Add literals compose,
  in its own stack order, into one deformation `scale × total + offset`. The planner applies it with
  admitted ops only:
  - the reset band seeds the host's `weight` with `offset`;
  - the host's upsweep band writes the plain child sum to `weight_sum` (its own child-share
    denominator, unchanged);
  - the same band adds `scale ×` that sum onto the seeded `weight`, which is its participation
    upward. This is an OrderBand-gated `AddToTarget`, a single-writer atomic add.

  The two writes to `weight` sit in different bands, so the bootstrap contention law is untouched.
  The deformation is re-applied to a fresh total each generation and never compounds.
- **Contained (every RF child is a physical descendant).** A Multiply is already carried by the
  inheriting leaves, since `factor × Σ = Σ factor ×`. The host adds nothing.
- **An inherited Add refuses typed: `InheritedAddWeightPolicy`.** Inheritance would add `a` to every
  participant (`total + n·a`), and the aggregate reading adds it once (`total + a`). The two meanings
  disagree, and choosing one needs an Owner ruling or an overlay-pass change.
- **Split reach refuses typed: `InteriorWeightPolicyReachSplit`.** Here some RF children descend
  physically and some do not, so the policy cannot be applied exactly once.
- **A replacing policy keeps its host's own value upward.** This covers a Set, a non-literal program,
  a routed instruction, and a need binding. Set still replaces.
- **Neutral trees are untouched.** An empty deformation map is the historical plan, bit-identical.

A composed stack is deterministic, and it is exact whenever the composition is exact (for example,
dyadic literals). `[×k₁, +a, ×k₂]` evaluates as `(k₁k₂)·Σ + a·k₂`, which can differ from the
sequential form in the last ulp. The CPU oracle executes the same ops, so parity is unaffected.

Code:
- `SimRuntimeTree` gains two observation-only queries:
  - `overlay_transforms_by_host`: hosts only, in stack order; its census row is RESIDUE;
  - `physical_descendants`.
- `overlay_transform_targets` takes the caller's predicate.
- The driver decides the law in `collect_weight_host_policies` and the per-arena
  `host_deformations`.
- The planner applies it in `plan_arena_allocation_with_policies`.
- No kernel change.

## Witnesses

`crates/simthing-driver/tests/rf_interior_policy_composition_0.rs` runs the ORDINARY resident path. A
root injects `R` over two owner seats. The small seat owns 1 site of 4 cohorts and the large seat
owns 2, so the plain totals are 4 and 8. Every base weight is 1. The witness checks seat
AllocatedFlow in two table-driven tests.

| case | policies (large seat unless noted) | lawful split | the #2035/#2036 law |
|---|---|---|---|
| seat multiply | small ×1, large ×1.5, R=4 | 1.0 / 3.0, both generations | **1.6 / 2.4** |
| seat add | +4, R=4 | 1.0 / 3.0 (4 : 12), both generations | pinned at base + 4 |
| seat stack | ×0.5 then +8, R=6 | 1.5 / 4.5 (0.5·8 + 8 = 12) | pinned |
| seat stack | +8 then ×0.5, R=6 | 2.0 / 4.0 ((8 + 8)·0.5 = 8) | pinned |
| contained multiply | small ×1, large ×1.5, sites inside their seats | 1.0 / 3.0, generation 1 | a second host application: 0.73 / 3.27 |
| seat set | =2, R=6 | 4.0 / 2.0, both generations | 4.0 / 2.0 |
| neutral | none, R=6 | 2.0 / 4.0, both generations | 2.0 / 4.0 |
| contained add / split reach | +1 / one site inside, one by edge | typed refusal at open | silently pinned |

The classification-only interim, which relied on inheritance alone, lost the seat policy entirely:
1.33 / 2.67.

The frozen referee `rehearsal_lifecycle_interior_policy_overlay_0` was corrected to the ruled law at
greater strength. Its test keeps its name and now proves:

- a Set classifies its host and the nodes it affects;
- standing Multiply and Add classify nothing;
- a routed instruction classifies its origin and target;
- wrong property and wrong role never classify;
- the raw query carries no law of its own.

`rf_conservation_any_shape_0` still holds on every generated shape. Its generated trees are
physical, so their interior policies are drawn from the admitted contained set (×2, ×0.5, =2). Its
shipped owner economy uses the seat topology, so the independent RF-1 check judges the seeded and
accumulated seat ops.

## Found, not fixed: standing Multiply/Add compound on persistent cells

The overlay OrderBand pass re-applies a standing Multiply or Add to its targets' weight cells every
tick. Those cells persist. A throwaway DA probe measured a root injecting 4 over two leaves, one
carrying the policy, through the ordinary resident session. The probe was never committed.

| policy | leaf weight, generations 1–4 | policy leaf's AllocatedFlow |
|---|---|---|
| ×1.5 | 1.5, 2.25, 3.375, 5.0625 | 2.40, 2.77, 3.09, 3.34 |
| +1 | 2, 3, 4, 5 | 2.67, 3.00, 3.20, 3.33 |
| =2 | 2, 2, 2, 2 | 2.67 every generation |

Every cohort that inherits a policy compounds the same way, which is why the contained witness stops
at generation 1. Under #2036, master left a policy-bearing seat's weight cell to the overlay pass, so
shipped seat policies compounded too. That was latent only because the root injects nothing today.
The seat path is now immune, because the reset band reseeds the seat's weight each generation.

The CPU `Evaluator` is a one-step oracle: it clones current state, integrates, and applies the stack.
So per-tick parity holds, and parity cannot see the compounding. Overlay scale law §15 defines a
standing overlay by the effective value it projects. The defect predates this law and reaches leaf
policies alike. It is reported to the Owner for its own ruling.

## E8 roll

`arena_allocation_plan.rs`, `arena_allocation_sync.rs`, `session.rs` and `sim_runtime_tree.rs` are
sealed components. This is the twentieth roll.

- **Pin chain:** master `0xef92_b0f2_866c_ef5d` → `0x2308_1112_d5c9_8f38`.
- **Old pin RED.** Each design's referee refused the pin before it:
  - the classification-only draft observed `0x8dbf_1614_9872_6591`;
  - the Multiply-only host scale observed `0x6d9c_0759_624b_83ca` (clean-checkout proven at `dc936f81`, never merged);
  - the affine law observed `0x1642_af70_961e_e400` against `0x6d9c…`;
  - after a rustfmt-only edit of the sync file, the final bytes observed `0x2308_1112_d5c9_8f38` against `0x1642…`.
- **Roll.** Both literals now read `0x2308_1112_d5c9_8f38`:
  - `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` in `simthing-gpu/src/resident_clearing_runtime.rs`;
  - `QUALIFIED_RECORD_FINGERPRINT` in `simthing-workshop/tests/resident_clearing_parity_0.rs`.
- **Battery** on NVIDIA 616.92, `--features simthing-gpu/eml-resource-profiling`:

| step | result |
|---|---|
| parity referee | 1/1, `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 23081112d5c98f38` |
| score and bands | 3/3 |
| runtime (ABI, child-share, planner and temporal-15.2 mutants) | 4/4 |
| witnesses (composition 2, conservation 2, frozen referee 1) | 5/5 |
| full suites | sim 26, gpu 7, driver 58 (1 ignored), workshop 16, clausething 47 (2 ignored), mapeditor 25; 0 failed |

The full suites include every test that loads the two shipped policy scenarios: 8 files, among them
the 1.1 witness `rehearsal_ingress_native_rf`. mapeditor was run on its own: after clausething in one
batch it hits a local rlib-format cache artifact before any test runs.

**Clean-checkout proof: PASS.** The proof used a fresh clone from GitHub at the roll commit
`5ef4e080`; later commits touch docs only.
- autocrlf is `false`, with 0 dirty paths and 0 CRLF files.
- Both literals read `0x2308_1112_d5c9_8f38`.
- The parity referee passes 1/1, observing `23081112d5c98f38`.
