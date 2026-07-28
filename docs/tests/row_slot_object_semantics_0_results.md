# ROW-SLOT-OBJECT-SEMANTICS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 4.1)
- Status: **COMPLETE — DA-GRADUATED / merged #1475 @ 272f82e2** (DA reproduced full corpus green incl. driver 119/0/13 on RTX 4080/Vulkan)
- HD-RECEIPT: `f7797b9aa308`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- orientation digest: `501b55a749e135ba75a82fa8d085ec0b88217b1570a4ccfa7a77db029aae16dd`
- base_sha: `c962187975fc6bce31f7f0e7e1da948ecb001890`
- expected_route: `DA-RESERVE(gate-wiring)`; coding leaves the draft PR unmerged for DA clearance

## Anchor receipts

- `simthing-0087-pillars@42b6ba6442aa`
- `simthing-0087-binding-laws@91270dd77e96`
- `rf-arena-substrate@17b5f1e5c2ba`
- `structural-execution-convergence@17fa0732f44d`
- `eml-extension-ladder@7755bc72ffbe`
- `spec-fidelity-anti-ceremony@add4dbbc267a`

## What landed

- `SimThing` emits a sealed `ObjectResidencyRequest`; `SlotAllocator` alone executes it into an authoritative `ObjectResidency`.
- Initial population, AddChild, fission and cloned subtrees, replay, remove/tombstone, and reparent all route through that door. Reparent changes the object relation while preserving the stable row.
- Parent-child topology and dense property projection require allocator-minted residency, so a raw sidecar slot cannot join either production structure.
- CPU reduction judges moved from `reduction.rs` to `cpu_oracle.rs`. Raw WGSL upload encoding moved from `world_state.rs` into the single fenced `wgsl_encode.rs` boundary.
- Execution-status census now records `reduction.rs` as compile-plan, `world_state.rs` as executed, and `wgsl_encode.rs` as compile-plan; `mixed_ruled=0`.

## Permanent referees

| Referee | Regression caught |
|---|---|
| `row_slot_object_semantics_layout_identity_oracle_parity` | Canonical or uneven-fixture DFS row order, slot identity, capacity, or count changes |
| `row_slot_object_semantics_sidecar_cannot_project_or_join_topology` | Raw sidecar mint becomes valid topology/projection residency |
| `row_slot_unattached_child_cannot_mint_or_leak_residency` | Unattached child mints residency/topology/projection, or detached relationless row survives release |
| `row_slot_object_semantics_structural_routes_preserve_one_authority` | AddChild/reparent/tombstone bypasses the door or destabilizes a row |
| `row_slot_object_semantics_fission_clone_routes_all_rows_through_relations` | Fission or cloned descendants mint outside object-issued parent relations |

The focused kernel profile passes 3/3, including the remand escaped-bug referee; the two focused sim structural/fission referees also pass. The execution-status census reports 124 classified modules: executed=57, oracle=6, rehearsal=14, compile-plan=47, mixed_ruled=0, non_execution=26.

## Remand closure

- Child request minting checks exact object identity against the parent's current direct-child vector. A same-id clone or unattached object cannot emit the request.
- AddChild, fission (including capability clones), replay add/fission, and initial population attach first and admit the complete subtree transactionally. Partial allocator admission rolls back, and caller-side attachment rolls back on rejection.
- Reparent and replay reparent attach before relation rebind and restore the exact old attachment when rebind cannot commit.
- `alloc_for_oracle_or_rehearsal`, `try_alloc_contiguous_after`, `claim_exclusive_slot`, and direct allocator `tombstone` are absent from the production Rust surface. The only unbound-row injector is private and `cfg(test)` inside the escaped-bug referee module.
- Detached subtree release defensively retires an old/test-injected relationless row as well as ordinary relation-backed residency, preventing unreachable live rows.
- Production census over core/kernel/sim/feeder/driver finds zero row-allocation callers bypassing the object/attachment residency route.

## Proof battery

| Check | Result |
|---|---|
| `cargo build --workspace` | PASS |
| `cargo check -p simthing-kernel -p simthing-sim` | PASS |
| `cargo test -p simthing-core` | PASS — 36 passed, 0 failed, 0 ignored across 2 harnesses (14 unit + 22 documentation) |
| live-pinned `cargo test -p simthing-kernel` | PASS — 78 passed, 0 failed, 0 ignored across 2 harnesses (40 unit + 38 documentation) |
| live-pinned `cargo test -p simthing-sim` | PASS — 35 passed, 0 failed, 0 ignored across 10 unit/integration/documentation harnesses |
| live-pinned `cargo test -p simthing-driver` | PASS — 119 passed, 0 failed, 13 intentionally ignored across 63 unit/integration/documentation harnesses |
| GPU selection | PASS — `NVIDIA GeForce RTX 4080 Laptop GPU`, backend `Vulkan`, device type `DiscreteGpu` |
| `agent_scan.sh` | PASS — 0 hard failures, 0 delta INSPECT |
| `gen_digest.sh --check` | PASS |
| `gen_orientation.sh --check` | PASS |
| `doc_budget_check.sh --check` | PASS |
| `test_inventory_drift_check.sh` | PASS — rows=1620, discovered=1618, unledgered=0, stale=0 |
| `execution_status_census.py` | PASS — classified=124, `mixed_ruled=0` |
| `anchor_check.sh` | PASS |

All live GPU runs set `WGPU_BACKEND=vulkan`, `SIMTHING_GPU_ADAPTER_CONTAINS=4080`, and `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`; adapter fallback therefore could not satisfy the proof. Existing compiler deprecation and unused-code warnings remain outside this rung.

## Scope ledger

- In scope: typed object-to-allocator residency derivation; structural route convergence; topology/projection proof consumption; two ruled file relocations; census; referees; rung/posture stamps.
- Incidental governed classification: the pre-existing driver `order_directive` module was added to the executed census when the whole-tree census exposed it.
- Doctrine co-evolution: the new sealed `ObjectResidency` token and fenced `wgsl_encode` module have conforming kernel-surface records; no scanner exclusion or weakening was added.
- Not changed: authored/wire state, serialized slot values, column-index plan structs, shader semantics, EvalEML opcode vocabulary, value behavior, or a second allocator/topology authority.

## EML / WGSL reach disposition

The EML gadget/JIT documents were reviewed as requested. A sealed stack or opcode addition was not necessary for this derivation-only rung, and the handoff expressly forbids WGSL semantic edits. No speculative shader or EvalEML experiment was taken. The new `wgsl_encode` raw-drop door carries `PLAN-STRUCT-TYPING-0` as its 4.2 promotion blocker, making the future typed path explicit without widening this rung.

## Graduation routing

Rung 4.1 is stamped PROBATION and the active posture advances to `PLAN-STRUCT-TYPING-0`. DA must verify the branch tree and issue final clearance; coding does not merge or dispatch the next rung.
