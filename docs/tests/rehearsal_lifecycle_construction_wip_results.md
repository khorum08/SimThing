# Construction Leaf A — canonical leaf stock settlement blocker

**PROBATION / blocker-proof-present / BLOCKED / OPEN / UNMERGED.**
Recipient: **ORCHESTRATION ONLY**. Authority: Board
[5689832213](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5689832213)
and [5689833128](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5689833128).
Same branch `codex/0088-construction-leaf-a`, draft PR #2062; synchronized by
merge onto exact repaired master `72d3c338ce91a47af17cfed5420b316a2484ede4` (#2064).
Historical ingress and allocation RED packets remain intact beneath dated-stage
history notices. No production repair or qualification pin change is in this leaf.

**STOP: delivered RF leaf residual does not enter canonical owned Balance.**
This is a production settlement gap before the required WIP lifecycle, not a
semantic falsification of ordinary WIP. **Model 1 remains UNDECIDED; Model 2
remains CLOSED.** Neither PASS-MODEL-1 nor FAIL-MODEL-1 is warranted.

## Required sequence and executed results

1. Canonical ordinary ingress ran FIRST and passed: **1/0/0**, required and
   observed tenth-roll fingerprint `d5fcaf92eda4f724`, bundle `7c10fa28f2ca3aa1`.
2. The existing independent-resource discriminator ran with unchanged semantic
   expectations and passed **2/0/0**: A alone, B alone, both arena orders, both
   physical child orders, and ordinary `open_from_spec + step_once`.
3. The next owned-stock prerequisite now executes through the same ordinary
   entry. New stock tests report **1 passed / 2 failed / 0 ignored**, exit 101.
   All failures retain required-success assertions; no expected-panic or skip.

Reference tuple: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79;
rustc 1.95.0, LLVM 22.1.2; `simthing-gpu/eml-resource-profiling` enabled.

## Stock fixture and controls

World has two project children P/Q. A and B are distinct ordinary RF properties
with source flow 1 each, dt=1, and P:Q weights A=3:1, B=1:3. Both start with
zero balances and zero balance rates. Each property has a distinct ordinary
`balance_rate` subfield and a `Balance` governed by that rate. This matches the
shape in `hydrate_category_economy::{balance_rate_subfield,balance_subfield}`
and the existing ClauseThing `ct_2a_intrinsic_flow` fixture. Balance is not
aliased to AllocatedFlow, and allocations/grants are never renamed stock.

The law is explicit in anchor `rf-arena-allocation-invariants`, content hash
`06044c5b726f583bba326942d01a573058425db3be5516ab7e7a91f1bb6b5dd2`:
leaf residual, allocator rounding residual and zero-weight surplus integrate
into Balance through governed_by; Balance is the sole RF carryforward ledger.
The query was performed through `anchor_query.sh`, not a raw doctrine grep.

| Case | Observed allocation | Observed balance | Required balance |
|---|---|---|---|
| A leaf P/Q | (.75,.25) | (0,0) | (.75,.25) |
| B leaf P/Q | (.25,.75) | (0,0) | (.25,.75) |
| Parent with full disbursement | 0 incoming | 0 | 0 |
| Parent zero-weight surplus control | children (0,0) | parent 1 | parent 1 — PASS |
| Seeded leaf rate .5, standalone RF control | no supplied flow | .5 | .5 — PASS |
| Same seeded rate, one ordinary session step | no supplied flow | 1.5 | .5 — FAIL |

The first two rows hold individually and jointly for all 12 component cases
(three resource sets × two arena orders × two physical child orders). All four
joint ordinary-session permutations reproduce the zero balances. A/B's live
allocations are correct in every case; their leaf balance rates remain zero.
Logical IDs stay fixed under physical reversal. Observations are GPU readbacks
only; nothing derived from readback is written back as economic authority.

The parent-surplus control passes on both component and ordinary paths. The
seeded-rate component proves the admitted governed pair can integrate at a
leaf, but its ordinary counterpart also exposes over-integration. That control
uses an explicitly seeded initial diagnostic rate, never claims delivered
material, and keeps its required .5 expectation. The exact cause of the 1.5
ordinary result needs production-path investigation; it is not silently changed
to an expected value or used to fund a construction claim.

## Source cause and repair boundary

`arena_allocation_sync::sync_resource_flow_accumulator_with_options` appends
`append_residual_closure_ops` before upload. The latter, in
`crates/simthing-driver/src/arena_allocation_plan.rs`, restricts rate producers
to `node.is_interior()`. It seeds a parent's rate from intrinsic budget, adds
its incoming allocation, then subtracts allocations to children. Leaf nodes
are omitted, so their distinct balance-rate column stays at its initial zero
despite positive incoming allocation. Ordinary governed integration then has
no leaf residual to settle. The control excludes absent Balance admission,
an unsupported GPU, the repaired cross-resource policy defect, and a missing
governed pair as explanations for this zero-rate result.

For the over-integration diagnostic, audit the same driver's use of
`build_governed_pairs(registry)` for every arena and the ordinary hot-cycle
governed integration path. This packet reports the observed .5→1.5 result;
it does not establish whether one or several scheduling sites own the cause.

A separately admitted production repair should start at `arena_allocation_plan.rs`
and `arena_allocation_sync.rs`, with hierarchy band budgets and the ordinary
kernel/session integration call chain inspected as necessary. Repair the
canonical resource settlement law, not construction-specific allocation.
Any sealed-component touch must follow the standing E8 procedure separately.
This leaf edits none of these sources, no pin, and no gate.

Replacing the leaf's balance rate with AllocatedFlow aliases a mutable planner
working column; a discarded exploratory fixture produced duplicate parent
stock that way. It is not the retained reproduction and not a lawful remedy.
Adding artificial child nodes merely to force the interior-only branch, or
copying GPU allocations into balances on the host, would conceal this gap.

## Full lifecycle accounting

| §5.3 obligation | Current evidence |
|---|---|
| Qualified ordinary ingress | PASS, tenth roll. |
| P/Q opposite allocation for one A and one B | PASS, unchanged discriminator. |
| Canonical delivered WIP owned by project | Executed RED: leaf balances remain zero. |
| No irreversible credit from incomplete inputs | Not proved in this lifecycle; no stock was fabricated to continue. |
| Cancel P and separately Q, stock/capacity/placement disposition and restart | Not proved; blocked before delivered WIP. |
| Residency/material timing, unavailable placement and touched fail-stop | Not proved as a composed construction lifecycle. |
| Two fresh funded AddChild/Remove/Reparent creations with all bindings | Not proved; manual structural operations and scalar recipe counters are not substitutes. |
| Determinism | Allocation/stock prerequisite permutations executed; full lifecycle unproved. |
| Insufficiency, starvation and cancellation negatives | Full construction negatives unproved. |

The explicit dispatch fence requires STOP for another production-source defect.
Scope remains test/evidence/inventory only. After an admitted repair, rerun the
unchanged success assertions, then continue the full matrix on this same leaf.

## Reproduction and validation

```powershell
$env:CARGO_TARGET_DIR='C:/Users/mvorm/SimThing/target'
cargo check -p simthing-driver --features simthing-gpu/eml-resource-profiling
bash scripts/ci/agent_scan.sh --base 72d3c338ce91a47af17cfed5420b316a2484ede4
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_discriminator -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_wip -- --nocapture --test-threads=1
```

Raw observations: [WIP raw packet](rehearsal_lifecycle_construction_wip_raw_results.md).
Exact final head, local scan, hosted artifact and step conclusions, and the fresh
post-body clearance verdict belong to the PR body and Board return.
`coverage_basis: FAIL` and `ci_green: NO` retain the required local GPU failures;
hosted Doctrine green cannot graduate construction. No new INSPECT waiver.

ORIENT-RECEIPT: 28f56884d309. Governance unchanged by #2064; receipt carried.
The direct Board remand is authority; no construction HD receipt is invented.
