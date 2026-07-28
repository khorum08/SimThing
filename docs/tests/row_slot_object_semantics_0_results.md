# ROW-SLOT-OBJECT-SEMANTICS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 4.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `f7797b9aa308`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- orientation digest: `501b55a749e135ba75a82fa8d085ec0b88217b1570a4ccfa7a77db029aae16dd`
- base_sha (handoff): `310242c34b7c10178a1d8423a5fc38a3d29d4693`
- expected_route: `DA-RESERVE(unclassified-scope)`; coding leaves the draft PR unmerged for DA clearance

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
| `row_slot_object_semantics_structural_routes_preserve_one_authority` | AddChild/reparent/tombstone bypasses the door or destabilizes a row |
| `row_slot_object_semantics_fission_clone_routes_all_rows_through_relations` | Fission or cloned descendants mint outside object-issued parent relations |

The focused kernel and sim referee profiles pass 2/2 each. The execution-status census reports 124 classified modules: executed=57, oracle=6, rehearsal=14, compile-plan=47, mixed_ruled=0, non_execution=26.

## Proof battery

| Check | Result |
|---|---|
| `cargo build --workspace` | PASS |
| `cargo check -p simthing-kernel -p simthing-sim` | PASS |
| `cargo test -p simthing-core` | PASS — 14 unit and 22 documentation tests |
| live-pinned `cargo test -p simthing-kernel` | PASS — 39 unit and 38 documentation tests |
| live-pinned `cargo test -p simthing-sim` | PASS — every unit, integration, and documentation harness |
| live-pinned `cargo test -p simthing-driver` | PASS — every runnable harness; final documentation harness 5 passed, 1 intentionally ignored |
| GPU selection | PASS — Vulkan adapter match required; NVIDIA GeForce RTX 4080 selected |
| `agent_scan.sh` | PASS — 0 hard failures, 0 delta INSPECT |
| `gen_digest.sh --check` | PASS |
| `gen_orientation.sh --check` | PASS |
| `doc_budget_check.sh --check` | PASS |
| `test_inventory_drift_check.sh` | PASS — rows=1619, discovered=1617, unledgered=0, stale=0 |
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
