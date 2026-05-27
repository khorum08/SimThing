# E-11 — Narrowed Implementation Handoff (Cursor binding)

**Status:** **Done (flat-star vertical slice)** — PR [#159](https://github.com/khorum08/SimThing/pull/159), commit `8a628ca`. **E-11R hardening** landed PR [#160](https://github.com/khorum08/SimThing/pull/160). Burn-in may proceed under default-off flag control.  
**Authority:** [`e11_hierarchical_allocation_design.md`](e11_hierarchical_allocation_design.md) (Opus v2)  
**Prerequisites:** E-10R, E-10R2, E-10R3, E-8R, E-7R — **landed** (PRs #155–#157).

**Do not re-implement prerequisites.** Build on the landed APIs below.

---

## What actually landed (honest scope)

E-11 landed **flat-star D=2 allocation execution** over AccumulatorOp v2:

- `build_execution_plan` materializes a **D=2 star** from arena participants (first sibling = root, remainder = leaves).
- GPU parity is proven for the flat-star path (`e11_single_level_positive_weights_cpu_gpu_parity`, session-path `e11_resource_flow_flag_uploads_and_dispatches_flat_star_ops`).
- **Nested hierarchy GPU execution is deferred** to E-11B / follow-up. CPU oracle covers custom nested layouts (`e11_multi_level_hierarchy_cpu_oracle_parity`); production materialization of nested trees is not wired.

`use_accumulator_resource_flow` remains **default false**. Do not begin burn-in until E-11R error propagation and scope/test hardening land.

---

## Landed E-11 modules

| Module | Purpose |
|--------|---------|
| `arena_hierarchy.rs` | Execution plan, tree layout, band math, column resolution |
| `arena_allocation_oracle.rs` | CPU oracle (phases 0–3) |
| `arena_allocation_plan.rs` | AccumulatorOp planner (reset, up-sweep, broadcast/disburse, E-7R integration) |
| `child_share_eml.rs` | `child_share_formula` EML registration |
| `arena_allocation_sync.rs` | Session sync behind `use_accumulator_resource_flow` (default **false**) |

**Substrate note:** `SourceSpec::SlotRange` now carries explicit `col` so up-sweep can sum child `intrinsic_flow` / `weight` into parent `intrinsic_flow_sum` / `weight_sum` columns.

**Constitution preserved:** no new WGSL; `simthing-sim` arena-ignorant; E-2B blocked unless enrollment compilation explicitly lands.

---

## `e11_*` test suite (14/14 green)

```
e11_single_level_positive_weights_cpu_gpu_parity
e11_zero_weight_sum_allocates_zero_no_nan
e11_multi_level_hierarchy_cpu_oracle_parity
e11_reserved_gap_fission_preserves_slotrange
e11_orderband_depth_budget_enforced
e11_balance_integrates_after_allocation_band
e11_rejects_missing_allocator_weight
e11_rejects_missing_allocated_flow
e11_no_new_wgsl
e11_no_simthing_sim_arena_imports
e11_allocated_flow_resets_each_tick
e11_integration_band_immediately_follows_deepest_disbursement
e11_no_nan_propagation_in_disbursement_path
e11_replay_bit_exact_across_two_runs
```

**Verification gate:** `cargo test -p simthing-driver e11 e11r`; prerequisite suites (`e10r*`, `e8r`, `e7r`, `accumulator_op`); `cargo check --workspace`; `cargo test --workspace`.

---

## E-11R remedial hardening (landed with this PR)

| Item | Status |
|------|--------|
| Sync errors | `ResourceFlowSyncError`; `install_spec_state` / `sync_resource_flow_if_enabled` propagate when flag enabled |
| Scope honesty | Flat-star D=2 GPU path documented; nested GPU deferred (E-11B) |
| Tests | `e11r_resource_flow_sync_error_is_reported_when_flag_enabled`, `e11_resource_flow_flag_uploads_and_dispatches_flat_star_ops`; renamed `e11_multi_level_hierarchy_cpu_oracle_parity` |

---

## Scope

Implement **E-11 allocation execution only**:

- Reset `allocated_flow` and per-tick internal columns
- Up-sweep `intrinsic_flow_sum` / `weight_sum` reductions (existing SlotRange + Sum substrate)
- Down-sweep propagated budget and weight-sum **broadcast**
- Per-child **EvalEML disbursement** (`child_share_formula`)
- **E-7R** governed integration after deepest disbursement band
- **CPU oracle parity**
- Feature flag `use_accumulator_resource_flow` (default `false`)

**Out of scope for E-11 PR:**

- New WGSL or kernel branches
- New `AccumulatorRole` variants
- `simthing-sim` importing `ArenaRegistry` or branching on arena semantics
- CPU production allocation fallback
- Boundary-time slot compaction
- Cross-arena tick-time coupling ops (memo §2.6 — boundary stage only for v1)
- Default-on flag flip (Step 10 — after CI burn-in)

---

## Landed APIs to use (do not duplicate)

| API | Crate | Purpose |
|-----|-------|---------|
| `materialize_arena_participants` | `simthing-driver` | Flat participant scaffold + gap pools |
| `ArenaParticipantScaffold`, `arena_participant_sibling_slots`, `slots_are_contiguous` | `simthing-driver` | Contiguity invariants |
| `try_alloc_participant_child_in_gap` | `simthing-driver` | Fission gap claim |
| `validate_resource_flow_preflight` | `simthing-driver` | Pre-materialize identity |
| `compile_and_materialize_resource_flow` | `simthing-driver` | `ArenaRegistry` |
| `expand_arena_internal_columns` | `simthing-core` | Column layout |
| `plan_reduction_orderband` | `simthing-gpu` | Up-sweep Sum reductions |
| `plan_governed_integration_at_band` | `simthing-gpu` | Phase 3 integration |
| `EmlExpressionRegistry::register_formula` | `simthing-core` | Disbursement tree |
| AccumulatorOp encode/upload/dispatch | `simthing-gpu` | Reset, broadcast, EvalEML, AddToTarget |

---

## Canonical band schedule (memo §2.3)

For arena depth `D` (root at 0, leaves at `D−1`):

| Phase | Bands | Ops |
|-------|-------|-----|
| **0 Reset** | `0` | `allocated_flow ← 0` on every participant |
| **1 Up-sweep** | `1 .. D−1` | Deepest interior first: `iF_sum`, `weight_sum` via SlotRange Sum |
| **2 Down-sweep** | per level `d ∈ [0, D−2]`: broadcast `D+2d`, disburse `D+2d+1` | Broadcast parent → child `pIF`/`pAF`/`pWS`; EvalEML disburse → child `aF` |
| **3 Integration** | `D + 2·(D−1) = 3·D − 1` | E-7R `plan_governed_integration_at_band(..., band, Some(participant_slots))` |

**Band count:** `total_bands(D) = 3·D − 1`  
**D = 1:** degenerate (reset + integration only; no allocation ops).

**Single-writer rule:** broadcast and disburse **must** be separate bands (memo §2.2).

---

## Column resolution

After `expand_arena_internal_columns`, resolve per participant slot via `PropertyLayout::offset_of`:

| Logical | Source |
|---------|--------|
| iF | sub-field with `AccumulatorRole::IntrinsicFlow` |
| aF | `AllocatedFlow { arena }` |
| weight | `AllocatorWeight { arena }` |
| balance | `Balance(_)` (optional for integration) |
| iF_sum, weight_sum, pIF, pAF, pWS | E-8R named internals |
| hosted_simthing_id | E-8R (observability; not used in allocation ops) |

Store resolved indices in `NodeColumnRefs` (memo §4.5).

---

## EML share formula (memo §7.1)

```
child_share = select(pWS > 0, (pIF + pAF) * w / pWS, 0)
```

- Register once per arena via `EmlExpressionRegistry::register_formula`
- Class: `ExactDeterministic`, `TransferConservation`
- Disburse: `EvalEML` + `AddToTarget` on child `aF` (single writer per disburse band)
- Root broadcast: `pAF = 0` at depth 0
- CPU oracle: `if pWS > 0.0 { ... } else { 0.0 }` (memo §9.1) — must match GPU bit-exact for zero-weight path

---

## Implementation sequence

### Step 1 — `arena_hierarchy.rs`

Build from `ArenaParticipantScaffold` + enrollment spec:

- `ArenaExecutionPlan`, `ArenaTreeLayout`, `HierarchyNode`, `NodeColumnRefs`, `ArenaBandLayout`
- Populate `arena_participant_index`, compute `max_depth`, `total_bands = 3·D − 1`
- **Reject** at plan build if `total_bands > arena.max_orderband_depth`

Start with flat explicit participants (`D = 2` vertical slice: root + leaves). Extend to nested hierarchy once oracle passes.

### Step 2 — `arena_allocation_oracle.rs`

CPU oracle from memo §9.1:

- Phase 0 reset, Phase 1 up-sweep, Phase 2 broadcast/disburse, Phase 3 balance integration
- Unit tests: 1-level, 2-level, 3-level before GPU wiring

### Step 3 — `arena_allocation_plan.rs`

Emit `Vec<AccumulatorOpGpu>` matching §2.3:

- Reset: Identity + Constant(0) + ResetTarget on `aF`
- Up-sweep: reuse `plan_reduction_orderband` patterns (SlotRange Sum, deepest-first bands)
- Broadcast: Identity + ResetTarget on child `pIF`/`pAF`/`pWS`
- Disburse: EvalEML + AddToTarget on child `aF`
- Integration: delegate to `plan_governed_integration_at_band` at `3·D − 1`

Topology for reductions must use **participant slot** child ranges from `ArenaExecutionPlan`, not hosted spatial tree.

### Step 4 — Register `child_share_formula`

Per memo §7.1 postfix tree (13 nodes). Bind column indices from `NodeColumnRefs`.

### Step 5 — Session sync wiring

- Add `use_accumulator_resource_flow: bool` (default `false`)
- Wire planner into accumulator session sync when flag enabled and `ArenaRegistry` non-empty
- Do **not** import arena types into `simthing-sim`

### Step 6 — Fission refresh (minimal)

On hosted SimThing fission with arena membership:

- `try_alloc_participant_child_in_gap` for slot
- Attach new `SimThingKind::ArenaParticipant` under parent participant in tree
- Reject per `FissionPolicy::Reject` when gap exhausted (log; do not break hosted fission)

Scaffold gap API is ready; tree attachment is new.

### Step 7 — Acceptance tests (binding names)

Primary suite: `crates/simthing-driver/tests/e11_arena_allocation.rs`

```rust
#[test] fn e11_single_level_positive_weights_cpu_gpu_parity()
#[test] fn e11_zero_weight_sum_allocates_zero_no_nan()
#[test] fn e11_multi_level_hierarchy_cpu_gpu_parity()
#[test] fn e11_reserved_gap_fission_preserves_slotrange()
#[test] fn e11_orderband_depth_budget_enforced()
#[test] fn e11_balance_integrates_after_allocation_band()
#[test] fn e11_rejects_missing_allocator_weight()
#[test] fn e11_rejects_missing_allocated_flow()
#[test] fn e11_no_new_wgsl()
#[test] fn e11_no_simthing_sim_arena_imports()
```

Additional tests from memo §11 (implement as needed for full coverage):

- `e11_allocated_flow_resets_each_tick`
- `e11_integration_band_immediately_follows_deepest_disbursement`
- `e11_no_nan_propagation_in_disbursement_path`
- `e11_replay_bit_exact_across_two_runs`

Participant scaffold tests (`e10r2_*`, `e10r3_*`) remain prerequisite regression — do not delete.

### Step 8 — Docs sync

Update `accumulator_op_v2_production_plan.md` E-11 row, `worklog.md`, `todo.md` when E-11 lands.

---

## STOP conditions (Cursor must report, not workaround)

Stop and request design review if any emerge during E-11 implementation:

1. **SlotRange cannot express a reduction group** without boundary compaction
2. **EvalEML share formula produces NaN** on zero-weight path despite `select` (memo §12.1)
3. **LIFO gap claim places fission child outside parent's reserved pool** when pool non-empty (memo §12.2)
4. **`NonContiguousChildren`** on arena participant hierarchy despite E-10R3 block layout (memo §12.3)
5. **`total_bands > max_orderband_depth`** for a valid enrolled arena at plan build
6. **E-7R cannot place integration** at `3·D − 1` with participant filter
7. **`simthing-sim` must import arena types** to complete the feature
8. **New WGSL or CPU production allocator** appears necessary

---

## Verification commands (E-11 PR gate)

```
cargo test -p simthing-driver e11 -- --nocapture
cargo test -p simthing-driver e10r2 e10r3 e10r -- --nocapture
cargo test -p simthing-core e8r -- --nocapture
cargo test -p simthing-gpu e7r -- --nocapture
cargo test -p simthing-gpu accumulator_op -- --nocapture
cargo check --workspace
cargo test --workspace
```

Prerequisite suites must stay green.

---

## First vertical slice (recommended)

1. **D = 2** arena fixture: one interior root participant + N leaf participants, all marked roles on flow property
2. CPU oracle parity for one tick, positive weights
3. GPU op plan dispatch at flag = true
4. `e11_single_level_positive_weights_cpu_gpu_parity` green
5. Then expand to multi-level, zero-weight, integration band, fission gap

---

## Bottom line

Prerequisites are **done**. Implement allocation execution per this handoff and Opus v2 §2–§9. Do not revisit E-10R/E-10R2/E-10R3 unless a STOP condition fires.
