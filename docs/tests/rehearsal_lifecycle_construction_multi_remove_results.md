# Construction Leaf A — interim local canonical multi-remove defect

**LOCAL-ONLY / INTERIM / BLOCKED. Model 1 UNDECIDED / Model 2 CLOSED.**
No terminal semantic verdict, merge, PR rebase, hosted proof or final clearance.

Authority: [5706526833](https://github.com/khorum08/SimThing/pull/2062#issuecomment-5706526833),
Board [5706524771](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5706524771),
DA ruling 5706504123. Local composition is retained Leaf-A head
`1cd598bd74d51ee298a0259c541f494646a62640` plus exact #2068 head
`8a4165cb6747f7e3c9172822704d7c38f6dc8599`, local merge
`93b2e1efa87deca76c37aa8b03f8f7d212ab6316`.

Worktree `C:/Users/mvorm/SimThing-0088-construction-local-2068`, branch
`codex/0088-construction-local-2068`, with its own `target` directory. The retained
#2062 branch/worktree and the concurrent expiry-disposal agent's checkout remain
untouched. No local composition is pushed. The eventual retained-PR rebase target
is the payer merge SHA, not #2068; all final proofs remain due at that future head.
Ambient expiry RED is not a construction failure.

## Repaired cancellation passes unchanged

The original `funded_product_cancellation_must_release_placement_and_continue`
ran first, byte-unchanged: **1 passed / 0 failed / 0 ignored / 4 filtered**, 11.20s.
All eight funded/control × P/Q × order sessions retire the exact placement at G4;
slots/relations return to baseline and G5/G6 remain healthy. #2068 closes the prior
placement-retention and canonical-shrink failures. Actual freed-capacity reuse was
part of the next attempted lifecycle; do not mislabel the old witness as testing reuse.

## New concrete prerequisite defect

**Multiple canonical Remove requests in the same boundary resolve later requests
through stale sibling indices.** They can silently remove an unrequested node,
leave the requested node alive, or panic out of the boundary. This is reproduced
without ActionBand, so it is not evidence that #2068's scoped repair failed.

Minimal fixture: ordinary P/Q resource recipe session plus five inert siblings
`A,B,C,D,E` under the same root. Root/P/Q occupy the first three rows. Submit two
identity-addressed `BoundaryRequest::Remove` operations before one `step_once`.
All requests are fixed authored inputs; no readback selects the actions.

| Request order | Observed disposition | Required result |
|---|---|---|
| Remove A, then C | Returns Ok; removes **A and D**, leaves C alive | Remove exactly A and C |
| Remove C, then A | Removes C and A, healthy boundary | Control PASS |
| Remove D, then E | Removes D, then panics at `tree_index.rs:50` with removal index 6 / length 6; E survives | Remove exactly D and E |
| Remove E, then D | Removes E and D, healthy boundary | Control PASS |

The panic case's three retries return
`ExecutionIdentity("generation GenerationStamp(1) is faulted after an unfinished economic authorization")`.
Resident values, tick/day, integration schedule length, binding table and remaining
root-child identities do not change across the retries. This is genuine fail-stop
on that path. The silent wrong-sibling case returns success, so fail-stop cannot
protect it.

The new test checks tree membership AND original identity/slot correspondence for
every initial row. It collects the panic only to run all controls and verify
fail-stop; the final required-success assertion remains RED for every unexpected
identity disposition and the panic. No expected-failure pass, ignore, descending-order
workaround or source-only proof replaces the failing obligation.

Source chain on the exact local composition:

- `crates/simthing-sim/src/boundary.rs` constructs `structural_paths` once before
  applying the complete mutation batch and passes the same map through that batch.
- `crates/simthing-sim/src/tree_mutation.rs::apply_remove` retrieves the original
  path for the requested ID and calls `detach_at_path` after earlier mutations
  have already shifted the sibling vector.
- `crates/simthing-sim/src/tree_index.rs::detach_at_path` directly calls
  `parent.children.remove(idx)`, without resolving the current target identity
  or rejecting an obsolete last index.

A repair belongs to the existing generic structural-mutation identity/lifecycle
boundary, with current identity resolution across sequential mutations and typed,
nonfabricating behavior for invalidated targets. Sorting this caller's requests,
patching indices in the test, or splitting the intended boundary would mask the
production defect. No production repair is made here; any sealed-source/E8 effects
belong to a separately authorized repair.

## Where continuation stopped

The exploratory placement/restart script used authored removals to create two
separated free rows. It reached a full quantity-2 market grant followed by typed
`OrdinaryGrowthRefusalReason::Placement(NoContiguousExtent)` at G3, with no product
or placement and unchanged residual P/Q stock. Restart then panicked during a
three-sibling removal batch at G4. The minimal reproof revealed that the earlier
G1 batch had already removed the wrong identity. Therefore that exploratory G3
refusal is **an observation, not an accepted clean placement-refusal proof**.
Its raw output is preserved with that explicit qualification.

The unpublished future sequence was not retained as claimed coverage or a large
unexecuted test matrix. The committed addition is the small production-boundary
reproducer. Its code is appended to the existing recipe file; all eleven prior
Leaf-A tests remain byte-preserved.

Full placement refusal/disposition, authored funded restart, physical capacity
reuse, two fresh successful completions with all property/overlay/membership/
observation/child/binding checks and complete structural determinism remain **unproved**.
No semantic insufficiency of ordinary WIP has been established. No Model-2 packet
or capability is invented.

## Validation and scope

Focused new witness: **0 passed / 1 failed / 0 ignored / 5 filtered**, exit101,
4.68s. Both descending-order controls pass; ascending interior removal is wrong,
and ascending tail removal panics and then remains fail-stop. Exact committed-head
ordered suite results and local INSPECT accounting are in the interim Board return.
[Raw local output](rehearsal_lifecycle_construction_multi_remove_raw_results.md).

New local delta only: appended required-success test, its normal AUDIT inventory
row, and this evidence pair. No production-source/Cargo/kernel/WGSL/ClauseThing/
Studio/scenario/gate/class/anchor/router edit. No reinstall, binding refresh,
CPU economic authority, refund or second stock ledger. #2068 production changes
are imported authority, not this agent's edits. Nothing is pushed or finalized.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6

Receipt source TSVs unchanged; carried from this session. Construction contract
re-resolved unchanged: ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2.
