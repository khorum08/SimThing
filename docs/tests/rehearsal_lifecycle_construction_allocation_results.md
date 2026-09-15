# Construction Leaf A — independent-resource binding blocker

Status: **PROBATION / blocker-proof-present / BLOCKED / OPEN / UNMERGED**.
Recipient: **ORCHESTRATION ONLY**. Resume authority:
[5687371290](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5687371290)
and [5687372904](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5687372904).
Base: `df5dd480b42dc224926c8bbab1d229b988b3f57c` (#2063).
The same branch/PR #2062 was synchronized by merge, retaining the pre-roll history.

**STOP: the executed allocation prerequisite violates independent resource policy.**
Ordinary resident ingress is now GREEN. A and B each allocate correctly alone,
but joint execution gives both the first arena's child weights. This is a current
production binding defect before lawful delivery of the discriminator's inputs.
It does not establish that recoverable ordinary WIP is semantically insufficient.
**Model 1 remains UNDECIDED; Model 2 remains CLOSED.** Neither requested semantic
verdict can truthfully be issued from this failure. In particular, atomic recipe
commitment would not repair a formula bound to the wrong resource's weight column.

## Executed reproduction

Two logical projects P/Q share a parent. Two distinct registered resource
properties A/B each supply flow 1 for one dt=1 generation. Authored weights are
A P:Q=3:1 and B P:Q=1:3. Expected allocations are A=(.75,.25), B=(.25,.75).
All quantities are exactly representable binary fractions. No flow is described
as owned WIP, no stock is minted from CPU readback, and no completion is claimed.

| Execution | A allocation (P,Q) | B allocation (P,Q) | Result |
|---|---|---|---|
| A alone, existing RF GPU planner | (.75,.25) | absent | positive control PASS |
| B alone, existing RF GPU planner | absent | (.25,.75) | positive control PASS |
| Joint GPU plan, arenas A then B | (.75,.25) | (.75,.25) | B wrong |
| Joint GPU plan, arenas B then A | (.25,.75) | (.25,.75) | A wrong |
| Same logical tree, physical children reversed | same as each joint row above | same | defect survives slot-order change |
| `SimSession::open_from_spec`, ordinary resident posture, one `step_once` | (.75,.25) | (.75,.25) | B wrong |
| Same ordinary scenario, physical children reversed | (.75,.25) | (.75,.25) | B wrong |

The component controls isolate the existing planner; the two ordinary-session
runs prove this is reachable through production ingress and a real generation.
Each session installs the existing resident RF substrate. Authored RF plan depth
is explicitly 16; this is a finite fixture budget, not a qualification bypass.
The test preserves required-success assertions: **0 passed, 2 failed, 0 ignored,
exit 101**. It does not mark the defect expected, ignore it, or substitute a CPU
allocation result. The first test runs both positive controls and all four
permutations before its final assertion, so failure does not hide later cases.

Canonical ingress command (run first after synchronization):

```powershell
$env:CARGO_TARGET_DIR='C:/Users/mvorm/SimThing/target'
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
```

Result: **1 passed, 0 failed, 0 ignored**. Required/observed fingerprint both
`6fe1d809c05ee0f4`; semantic bundle `c6b6a70c64f7ae81`. Reference tuple remains
NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79, rustc 1.95.0,
LLVM 22.1.2, `EML_RESOURCE_PROFILING`. No further pin or source-bundle mutation.

Focused reproduction and compile check:

```powershell
cargo check -p simthing-driver --features simthing-gpu/eml-resource-profiling
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_discriminator -- --nocapture --test-threads=1
```

Raw excerpts are in [the resumed raw packet](rehearsal_lifecycle_construction_allocation_raw_results.md).
Exact final head, hosted Doctrine/step conclusions, INSPECT accounting and fresh
clearance are carried by the PR body and Board return. `ci_green: NO` is required
while the local success assertions remain RED, even if hosted Doctrine is green.

## Source cause and repair boundary

The observed arena-order switch follows the actual production call chain:

1. `arena_allocation_sync::sync_resource_flow_accumulator_with_options` resolves
   columns for every arena into one `EmlExpressionRegistry` and calls
   `register_child_share_formula` for each.
2. `child_share_eml::compile_child_share_formula_nodes` embeds the arena's concrete
   child `weight_col` in a `SLOT_VALUE` node. The three parent operands are params;
   the child's weight is not a param.
3. `register_child_share_formula` uses the single `CHILD_SHARE_FORMULA_TREE_ID`
   and returns success immediately if that ID is already registered. Therefore
   the first resource's embedded weight column survives for all later resources.
4. `arena_allocation_plan::disburse_op` points every arena at that same tree ID.
   Its resource-specific parent budget/denominator remain correct; its child
   numerator comes from the first resource. The independent controls plus arena
   reversal isolate precisely that combination, not rounding or stock integration.
5. `WorldGpuState::sync_resource_flow_ops_from_cpu` also keeps existing formulas
   by ID when merging supplemental EML into the runtime registry. Any repair must
   cover rebind/layout changes as well as first upload.

Minimum repair candidates for a separate admitted production leaf:
`crates/simthing-driver/src/child_share_eml.rs`,
`crates/simthing-driver/src/arena_allocation_plan.rs`, and
`crates/simthing-driver/src/arena_allocation_sync.rs`; inspect the shared tree-ID
contract in `arena_hierarchy.rs` and runtime merge in
`crates/simthing-kernel/src/world_state.rs` before choosing the exact surface.
Either a correctly admitted per-resource formula binding or a column-independent
formula must preserve the existing allocation law and one runtime authority.
This packet does not select or implement that design. No new Model-2 API, Cargo
change, construction allocator, or scheduler is warranted by this evidence.
Some candidate owners are in the full-byte E8 bundle; orchestration must route
any resulting qualification debt under the existing rule, not copy a new pin.

The original dispatch's fence applies: **"If truthful proof requires [a production
source edit], STOP rather than widening scope."** Supplying manually corrected B
allocations, a test-only replacement formula, or a second allocator would conceal
this failure. No such workaround is included.

## Full lifecycle accounting

The earlier [source inventory](rehearsal_lifecycle_construction_ingress_results.md)
still names the relevant stock/recipe/grant/placement/structural owners. The new
runtime evidence refines its material-delivery row only.

| Required item | Current evidence / remaining work |
|---|---|
| Ordinary qualified ingress | PASS after #2063; historical RED retained separately. |
| Two projects / two inputs / opposed allocation | Executed; independent-resource binding FAIL above. Required lawful starting allocation cannot be claimed. |
| Owned delivered WIP and no irreversible partial spend | NOT PROVED end to end; no fabricated corrected delivery fed to recipes. |
| Cancel P and Q, release/restart | NOT PROVED end to end. |
| Placement timing, unavailable placement and fail-stop | Source inventory only for acceptance; preliminary recipe/refusal probe is not a coupling proof. |
| Two fresh funded structural completions and full bindings | NOT PROVED. No fixed template, manual fresh AddChild controls or completion counter substituted. |
| Determinism/storage invariance | Executed diagnostic: physical child reversal leaves the defect; arena order changes which resource's policy controls both. Semantic allocation invariance fails. |
| Shortage, malformed ownership and other lifecycle negatives | Still required after independent allocation is repaired; no broad construction PASS inferred from this narrow blocker. |

The old exploratory four-case draft was attempted after ingress became green.
It exposed fixture issues (an unproduced leaf balance-rate, undersized default RF
band budget, and an unfunded AddChild control), plus one recipe-before-refusal
observation. Those draft assertions were replaced by this isolated reproduction;
they are not enrolled or claimed as construction acceptance. In particular,
recipe output retained as finished WIP could be lawful after placement refusal,
and the existence of a frozen structural consequence does not by itself prove a
need for atomic multi-lane commitment.

After admitted repair, rerun these unchanged success assertions, then resume the
full matrix on the same Leaf A. The requested PASS-MODEL-1 / FAIL-MODEL-1 decision
belongs after that executable lifecycle, not after this allocation defect.

## Changed surface and lifecycle ledger

Relative to repaired master, PR #2062 contains only:

- `crates/simthing-driver/tests/rehearsal_lifecycle_construction_ingress.rs` — existing provenance/ordinary-open guard, current header.
- `crates/simthing-driver/tests/rehearsal_lifecycle_construction_discriminator.rs` — two required-success allocation reproductions.
- `docs/tests/rehearsal_lifecycle_construction_ingress{,_raw}_results.md` — preserved pre-roll history, current pointer.
- `docs/tests/rehearsal_lifecycle_construction_allocation{,_raw}_results.md` — current blocker and raw evidence.
- `scripts/ci/test_inventory.tsv` — three Leaf-A-consumed admission-adjacent AUDIT rows, delete at closeout.

No production, scenario/UI, Cargo, sealed component, gate, anchor, or router edits.
No 2.1 graduation or independent DA relay. Board handoffs are direct authority;
no nonexistent HD receipt is fabricated.

ORIENT-RECEIPT: 28f56884d309
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6

Governance sources did not change in #2063; the session receipt is carried.
Construction-contract and charter anchors were re-resolved with unchanged hashes;
all 14 original ACKs remain in the historical raw packet.
