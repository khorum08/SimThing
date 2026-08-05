# SLOT-LOGICAL-IDENTITY-0 — witness results

Rung: 6.4 `SLOT-LOGICAL-IDENTITY-0` · dispatch `5194769530` (resume after accepted
FIRST-STEP STOP `5194589941`) · authority DA `5194703997` + `5193918971` ·
HD-RECEIPT `afee4a04800d` · base `600a40df`.

## What landed

**The ruled logical/physical split (StemThing §3.1 shape (a); Tier-2 law).**
`SlotIndex` is stable logical identity — stable within an epoch, rebindable only
at a recorded boundary remap on the ONE binding table
(`SlotAllocator::epoch_rebind`: capacity-preserving, reserved-gap-refusing,
row-churn-refusing) through the ONE `AnchorLocusRemap` history. Between epochs
there is zero per-access indirection: `apply_epoch_rebind_to_values` bakes the
recorded permutation into the slot-major plane at the boundary, and every
slot-bearing artifact (values, reduction CSR, OrderBand plan, anchor table) is
rebuilt from the post-rebind table.

**The typed history extension (DA `5194703997`, all three constraints).**
`AnchorLocusRemap.subject: RemapSubject::{PropertyLocus{property_id,from_col,to_col} | ObjectRow}` —
slots record-level, `ObjectRow` columnless by construction, no
`Option<SimPropertyId>` / sentinel / parallel-Options encoding expressible.
`AnchorRemapOperation::EpochRebind` (rebind is the event) derives demand from
pre/post **binding-table** snapshots: exactly one `ObjectRow` per moved live
row — zero-anchor rows included — with `PropertyLocus` rows only for loci whose
column binding changed. Both over-emission REDs are planted and green:
duplicate `ObjectRow` → `"duplicate Anchored remap keys"`; unchanged-column
`PropertyLocus` → `"PropertyLocus emitted for unchanged column binding"`.

**Order-bearing paths are authored-key derived.** `TopologyState::build` no
longer sorts child blocks by slot — canonical reduction order is AUTHORED TREE
ORDER (walk/attach order), invariant under rebinding; `add_child` appends
(attach order) and no longer relies on monotone slot minting. The OrderBand
planner's `SLOT_RANGE` contiguity check fail-closes on any placement whose
physical ascending order diverges from authored order — physically-ascending
GPU execution is reachable only when it coincides with authored order.

**Replay + K3.** `SpecDelta::ScriptedInstanceSlotChanged` and
`ScriptedEventInstance.current_slot` retyped `u32 → SlotIndex` (wire-transparent);
`apply_spec_delta` resolves pre-remap slot records against post-remap state via
`resolve_slot_through_chain` over the one canonical chain. Dense cell-space is
named `CellSpaceIndex` (authored-coordinate mint `from_authored_grid`,
`as_eml_literal`), transposition with `SlotIndex` uncompilable in both
directions; the min-plus destination literal is minted through it.

## Witnesses (crates/simthing-driver/tests/slot_logical_identity_0.rs — 3/3 green)

1. **Forced scramble bit-identity.** Physical rows scrambled with logical ids
   fixed at a mid-run barrier; 3 generations with id-keyed feedback evolution.
   CPU oracle (interior tree fixture, order-sensitive Sum triple `1e8, 1.0,
   -1e8`) and the REAL OrderBand GPU path (flat family through
   `plan_reduction_orderband` + `upload_reduction_soft_ops_with_bands` +
   `run_tick_pipeline_with_accumulators`, the dispatcher wiring) are
   **bit-identical per id per generation** to the never-scrambled run, and CPU
   == GPU bit-exactly. Baking == post-table layout is asserted directly.
2. **Production physical-row-order mutant REDs.** The planted defect is the
   exact line deleted from `TopologyState::build`
   (`for v in &mut per_slot_children { v.sort_unstable(); }`), executed by the
   UNMODIFIED production reducers: CPU sums `a`'s kids as authored `0.0` vs
   mutant `1.0`; on the GPU the planner FAIL-CLOSES on the authored
   non-monotone CSR (`NonContiguousChildren` — never silent reorder) while the
   slot-sorted mutant CSR plans and executes to `16.25` vs the authored oracle
   `15.25`. Divergence asserted on both arms.
3. **Pre-remap replay through the chain.** A `ScriptedInstanceSlotChanged`
   recorded before the rebind applies against post-rebind state and lands on
   the post-rebind row via the canonical chain (empty chain = identity);
   anchor-table `ObjectRow` transport moves whole rows on the CPU twin and the
   GPU `KIND_ROW_MOVE` arm with columns and row-count preserved.

### Source-level mutation run (DA `5193918971` amendment 2)

The defect was additionally planted INTO the production builder
(`crates/simthing-kernel/src/reduction.rs`, `TopologyState::build`):

```text
+        for v in &mut state.per_slot_children { v.sort_unstable(); } // MUTANT
```

Witness output with the mutant in production source (then reverted; battery
green again after revert):

```text
test slot_logical_identity_0_forced_epoch_rebind_is_bit_identical_cpu_gpu ... FAILED
test slot_logical_identity_0_production_row_order_mutant_reds ... FAILED
  assertion failed: generation 1: SimThingId(11) col 0 drifted under a pure physical scramble
    left: 1107853312  right: 1108246528
  assertion failed: authored order sums a's kids to exactly 0.0
    left: 1065353216  right: 0
```

## One-history / zero-indirection grep proof

- `AnchorRemapSection`/`AnchorLocusRemap` remain the ONLY remap/history record:
  producers/consumers are `anchor_remap.rs` (record + validation),
  `anchor_remap_encode.rs`/`boundary.rs` (derive/gate), `anchor_table.rs` +
  `world_state.rs` (apply, CPU/GPU), `slot.rs` (`epoch_rebind` mint),
  `delta_log.rs`/`replay.rs`/`spec_replay.rs` (transport + chain resolution).
  No other remap/epoch/sidecar record type exists (`grep -rn "remap"` over
  production crates closes onto this set; `fission.rs` and
  `scripted_event_definition.rs` reference the section/chain types only).
- The id→slot authority remains the ONE `SlotAllocator`
  (`slot_owners`/`by_id`); `binding_table_snapshot()` is a read-only
  projection, `epoch_rebind` the only mutator beyond the existing
  residency/tombstone doors. No second id→slot map exists (`by_id` grep closes
  onto `slot.rs`; other hits are unrelated local maps in clausething/anchor
  bookkeeping keyed by other identities).
- Zero per-access indirection between epochs: no production read path consults
  a logical→physical map at access time — bindings are baked at boundary
  upload (`apply_epoch_rebind_to_values`, topology/plan rebuild, anchor-table
  remap dispatch); `resolve_slot_through_chain` is invoked only at replay
  interpretation.

## Census / bookkeeping

- `stemthing_slot_census_check.sh --check`: rows=25 universe=51 assigned=51
  dup=0 missing=0 blockers=0 — **PASS** (zero BLOCKER, zero ORDER-PIN
  preserved). The four load-bearing rows close: C1 (binding table = the
  rebind home), P1-anchor-remap-record (extended in place — subject typed, no
  second record), P1-spec-replay-slot-delta (retyped + chain-resolved), K3
  (cell-space named, conflation uncompilable).
- Inventory +10 rows (3 driver witnesses, 3 allocator units, 4 anchor-remap
  units); TEST-BUDGET INSPECT justified (DA-fixed door semantics, one law per
  named RED); allowlist +1 conforming row (`apply_epoch_rebind_to_values` —
  the rung's boundary-baking surface); triage log row present.
- 9.2 `RF-COLUMN-MINT-MIGRATE-0` now **BINDS 6.4**: persistent column layout
  keys on stable logical identity — never physical row position or allocation
  order — echoed in the 9.2 exit-proof with its own planted-red obligation.
