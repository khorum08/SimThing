# Construction Leaf A — funded cancellation stops at two production seams

**PROBATION / proof-present / BLOCKED / OPEN / DRAFT / UNMERGED. ORCHESTRATION ONLY.**

Authority: Board [5705344661](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5705344661),
followup [5705345918](https://github.com/khorum08/SimThing/pull/2062#issuecomment-5705345918),
and DA repair ruling 5705326876. Same PR #2062 and branch
`codex/0088-construction-leaf-a`, merged with repaired master
`6dbab071be7afd3967c63663badcb9067e94ac22` (#2067); synchronization commit
`6244bfc1d0ab5333447591230ea0af38c7173ac5`.

**STOP: canonical Remove retires the funded product's tree/slots but retains its
committed placement. With ActionBand installed, that removal also fails
`ActionBandIngress(BindingTableStale)` after the structural change.** These are
concrete production/precondition findings, not semantic falsification of ordinary
WIP. **Model 1 UNDECIDED / Model 2 CLOSED.** No new allocator, refund, reinstall,
test-side baseline refresh, binding bypass, engine edit or Model-2 API.

## Prerequisite repair confirmed

Before adding the new witness, unchanged ingress (1), allocation (2), WIP (3),
and recipe/lifecycle (4) suites ran in that order: **10 passed / 0 failed / 0 ignored**.
The original recipe file and other prerequisite tests are byte-preserved. #2067
repairs first funded birth in both orders; G3, G4 and G5 now complete. Joint
material accounting, partial-WIP recovery-host cancellation/restart, and typed
missing/ambiguous-host negatives remain green. Historical RED evidence is retained.

Current canonical qualified ingress: required/observed `67ffbfdfe25a1d4e`, bundle
`754d2b205494c4e0` (the #2067 twelfth E8 roll). No Leaf-A E8 roll or sealed-source
change. GPU reference: NVIDIA GeForce RTX 4080 Laptop / Vulkan / NVIDIA 595.79;
rustc 1.95.0, LLVM 22.1.2; feature `simthing-gpu/eml-resource-profiling`.

## Executed cancellation witness

One appended test, `funded_product_cancellation_must_release_placement_and_continue`,
runs eight independent ordinary sessions: funded ActionBand vs external AddChild
control × P/Q product host × original/reversed child, arena and authored recipe order.
The external control isolates canonical removal; it is not offered as funded construction.
All actions and the scrap disposition are fixed before observation; GPU reads only assert.

| Generation | Required and observed behavior |
|---|---|
| G1 | Root supplies 4A/4B once; opposite allocation settles P(3,1,0), Q(1,3,0); no child or placement. |
| G2 | Both ordinary recipes consume their own 1A+1B; P(2,0,1), Q(0,2,1). The selected project's sealed threshold .5 queues its pre-admitted AddChild. No child/placement yet. |
| G3 | Boundary succeeds. Fresh product plus component attach; product has the admitted scalar property. Root grants placement `[3,5)`, quantity 2, carrying a market-grant key and generation 3. Two new slots; preallocated shape unchanged. |
| G4 | Fixed authored Remove scraps the acquired product subtree. Both nodes disappear, both residency relations disappear, live count returns 5→3, and the entire binding table returns exactly to its original contents (including unchanged project bindings). Residual stock remains P(2,0,1), Q(0,2,1). **Placement still reports the same committed quantity-2 grant.** |

Disposition is explicit: the two consumed units per material across P/Q stay
consumed. There is no refund for a scrapped completed product. Residual P/Q WIP
remains owned and unchanged. Scalar recipe output is an accounting receipt, not
a separate reservation ledger. The unreleased placement is an observed failure,
not an accepted retention policy. Reusable capacity and later placement reuse are
**not claimed proved** merely because the physical slots were freed.

- All four external controls complete G4 and healthy G5/G6, but retain the stale
  placement at G4. This isolates the placement lifecycle gap from ActionBand.
- All four funded cases fail G4 with `ActionBandIngress(BindingTableStale)` after
  the same removal. No enrolled project identity was removed or remapped.
- Three retries in each funded case fail with
  `ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")`.
  Resident values, tick/day, schedule length, binding table, ActionBand execution
  generation and remaining placement are unchanged across all **12 retries**.
- The final required-success assertion reports all eight retained placements and
  four boundary failures. It is intentionally a real failing obligation, with no
  ignore, expected-panic or acceptance of the errors as success.

## Production boundary diagnosis and routing

1. `crates/simthing-sim/src/tree_mutation.rs::apply_remove` detaches the subtree,
   calls `SlotAllocator::release_subtree`, clears released shadow rows and records
   tombstones. In `crates/simthing-kernel/src/slot.rs`, `release_residency` retires
   the relation and `tombstone_slot` frees the row. Neither retires the associated
   committed residency placement. The runtime API still returns its old grant.
   Adjudicate the canonical commitment/release disposition and ownership separately;
   this test cannot lawfully invent a placement release ledger or mutate private state.
2. `crates/simthing-driver/src/session.rs::adjudicate_action_band_boundary_binding_table`
   accepts #2067 additive growth, but requires every accepted binding row to remain.
   Removing the newly added, unenrolled product rows through canonical Remove thus
   returns BindingTableStale even though the original bindings are identical.
   Adjudicate canonical removal evidence/baseline succession without relaxing foreign
   drift, bound-identity remap/removal, registry/dimension fences or fail-stop.

These are separate observed seams. The second is outside #2067's additive-growth
contract, so this packet does not label #2067's scoped repair a failed repair.
A separately authorized production repair must own its tests, sealed-source/E8
accounting and governance. Coding has changed no protected source.

## Full remaining contract accounting

| §5.3 obligation | Current status |
|---|---|
| Ordinary ingress, opposing P/Q allocation, settled recoverable WIP, exactly-once recipe consumption | GREEN unchanged prerequisites and joint matrix. |
| Partial-WIP cancellation/disposition/restart | GREEN existing recovery-host policy; no acquired product commitment in that earlier case. |
| First funded structural completion and healthy post-birth continuation | GREEN unchanged test after #2067, original and reversed orders. |
| Funded cancellation after acquired placement/residency, release and usable continuation | RED here; slots/relations retire, placement persists, ActionBand session faults. |
| Placement-unavailable refusal and material/reservation timing | Not executed in the full funded lifecycle; no fabricated pass. |
| Authored restart after funded cancellation/refusal | Not proved; cancellation prerequisite fails. |
| At least two successful fresh completions in a continuing session, properties/overlays/membership/observation/children/bindings | Not proved. Independent first births do not satisfy this requirement. |
| Full structural determinism and typed foreign/stale/placement negatives | Incomplete. Prior economic permutations, insufficient-input and host negatives stay green; this matrix adds both hosts/orderings and actual fail-stop. |

The authorized response to a new concrete prerequisite defect is STOP, not
PASS-MODEL-1 or FAIL-MODEL-1. No Model-2 semantic delta is requested from these failures.

## Validation and scope

Exploratory new witness: **0 passed / 1 failed / 4 filtered / 0 ignored**, exit 101,
10.16 seconds. [Raw packet](rehearsal_lifecycle_construction_release_raw_results.md)
contains the executed output. Final exact-head ordered validation, local/hosted
Doctrine artifacts and actual fresh clearance verdict are bound in the PR body,
validation comment and Board return after these evidence files are committed.

Current change: appended required-success test, its ordinary inventory row,
two evidence documents and historical headers on the birth packet. Total Leaf-A
scope remains four rehearsal test files, twelve evidence documents and
`test_inventory.tsv` (11 admission-adjacent AUDIT rows, delete-at-closeout).
No production, Cargo, kernel/WGSL, ClauseThing, Studio, scenario, gate, anchor or
router edits. No original test or assertion weakened.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6

Construction contract re-resolved: ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2.
Governance sources unchanged by #2067; existing source-bound receipt carried.
