# STEMTHING-B-ALLOCATOR-RETIREMENT-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2d
- Status: **PROBATION / proof-present / DA-review-pending**
- Exact implementation base: `8282f77903e9313d7417fb794a05a884d263efd3`
- Tested-code checkpoint: `e81d34620b296cef64d3348323e242e9cf912538`
- Branch: `codex/stemthing-b-allocator-retirement-0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- orientation digest at the exact base: `8f1b6d077c43141f8b7f62bceca53a35cc2f404164b0ceb4e30e605d0a0cf2ac`
- HD-RECEIPT: `106531617040`
- Handoff: Board #1332 comment `5430571776`
- Governing DA exit/unblock: Board #1332 comment `5426558048`
- Expected route: `DA-RESERVE(gate-wiring)`
- Pointer movement: none
- Predecessor certificate baseline: 119 suites / 458 passed / 3 failed / 14 ignored

## Pre-edit authority map and A1 dispositions

| Site | Final classification |
|---|---|
| session-open `install_initial_tree` | lawful initial-bulk or same-root continuation only; it realizes an already-authored tree, while a different-root attached-growth attempt remains a typed refusal |
| boundary fission | the complete candidate batch clears through the real 11.2a Draw/market binding before mutation; `VerifiedGrowthResidencyCommit` carries opaque provenance and 11.2b placement into spawn consumption |
| boundary AddChild | the complete drained batch follows the same market/provenance/placement chain before structural consumption; refusal attaches no child and mints no row |
| replay | consumes the recorded verified commit through the authoritative replay door; it has no clearing input and re-clear remains explicitly forbidden |
| relocation | retains the existing transactional epoch-rebind/remap route and stable logical identity |
| registry append / GPU sizing | downstream storage infrastructure after accepted placement; neither can select a recipient, admission result, or quantity |

The surviving chain is therefore: 11.2a market entitlement -> opaque
`MarketGrantResidencyProvenance` -> 11.2c `VerifiedGrowthResidencyCommit` ->
11.2b level-local extent placement -> physical slot/range machinery as geometry.
No ambiguous policy/geometry seam or STOP condition was found.

## Banked allocator disposition

`simthing-spec::designer_admission::mobility_alloc0::plan_mobility_alloc0`
constructed a `BTreeSet` of arriving entity identities and a `BTreeSet` of free
physical rows, then paired them with `pop_first()`. Under capacity pressure that
surface selected both the admitted recipients and their rows from collection
order, despite being test/designer substrate rather than production-wired.

That planner is deleted, not deprecated. Its block, boundary-event,
arrival-order, forbidden-request, assignment, plan-input, plan-report, allocator
identifier, and allocator-checksum APIs are gone from both module and crate
exports. The `ALLOC` stage and report are also removed from the mobility runtime
composition and its production-path fixture. `mobility_alloc0` retains only
passive parent-key and live-slice records because other mobility witnesses need
to name already-recorded residency.

The adjacent re-enrollment substrate is narrowed to structural movement of an
already-resident object. Its input contains no block capacity, free list,
arrival order, destination row, or admission contest. Every move carries the
object's existing stable logical slot unchanged; all valid moves commit after
canonical validation. This cannot choose WHO, WHETHER, or quantity from
physical order.

## Seal and standing falsifiers

| Proof | Biting failure |
|---|---|
| `mobility_alloc0` rustdoc compile-fail | restoration of the retired planner or allocation-shaped plan input |
| `mobility_reenroll0` rustdoc compile-fail | caller-proposed destination row or arrival-order authority |
| `reparenting_preserves_every_resident_slot_without_recipient_or_free_list_order` | input order changes the result, a move is dropped, or a stable logical slot is reallocated |
| `duplicate_live_logical_slot_is_a_typed_admission_red` | malformed recorded residency reaches movement rather than its named typed refusal |
| determinism-matrix planted defect | a renamed descending-identity/first-free allocator reproduces the live result instead of RED-diverging |
| frozen 11.2a/11.2b/11.2c witnesses | any deletion damages market grants, opaque provenance, verified growth, level-local placement, refusal U/revalue, exact subtree realization, or replay |

The renamed-equivalent planted defect deliberately reconstructs physical-row
assignment by descending entity identity. It diverges from the live
slot-preserving route. Together with the two missing input surfaces, the seal
is structural and typed; no new census, validation framework, or scan was
introduced.

## Deletion signal

Exact production/designer narrowing touches:

- `simthing-spec` mobility allocator, re-enrollment, runtime composition, and public exports;
- the driver production-path composition fixture; and
- the existing driver determinism witness plus the 34k projection support fixture.

Across production Rust sources (`crates/**/src/**/*.rs`), the delta is 128
insertions and 627 deletions: **net -499 production lines**. The new lines are
slot-preserving narrowing and documentation, not a replacement allocator,
manager, entitlement/provenance authority, policy registry, retry loop, or scan
surface. Kernel free-range/index physics, 11.2b placement, 11.2c provenance,
registry-column append, and GPU sizing remain intact.

## Focused evidence

```text
cargo test -p simthing-spec --test stemthing_b_allocator_retirement_0
cargo test -p simthing-spec --doc
cargo test -p simthing-driver --test determinism_matrix_0 --test cpu_gpu_parity_matrix_0
cargo test -p simthing-driver --test stemthing_b_flow_market_germ_0 --test stemthing_b_vram_residency_0 --test stemthing_b_growth_entitlement_seam_0 --test rf_column_mint_migrate_0
cargo test -p simthing-sim --test protected_representative_restore --test gpu_overlay_lifecycle_oracle_parity_0
cargo test -p simthing-kernel install_initial_tree_continues_same_root_and_types_attached_growth_bypass --lib
cargo check --workspace --all-targets
bash scripts/ci/test_inventory_check.sh
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/agent_scan.sh
```

The focused retirement suite is 2/2 green and both new rustdoc compile-fail
seals are green. The determinism and CPU/GPU parity matrices are each 2/2 green,
including the planted first-free divergence. The frozen market/growth/residency/registry batteries
are 11/11 green; GPU lifecycle parity is 1/1; representative restore is 2/2;
the typed install door is 1/1. Inventory and drift are exact at 1339/1339.
Workspace/all-targets and Agent scan are recorded at the tested checkpoint.
The production-deletion structural certificate remains intentionally owed to
DA at graduation.

## Fences retained

- No 11.2a, 11.2b, or 11.2c redesign; no physical placement/index deletion.
- No 11.2e Vendor Door, 11.3 implementation, 11.4, 12.x, Vector CostBand, or ClauseThing-red work.
- No replacement allocation, clearing, provenance, policy, retry, telemetry, history, generation, or scan authority.
- Coding returns at PROBATION; DA alone reviews, certifies, graduates, merges, and moves pointers.
