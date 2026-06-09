# SlotSummary Readback Protocol (PR B-4 design memo)

**Author:** Opus 4.7
**Date:** 2026-05-25
**Gate for:** Implementation PR — `feat(gpu): implement B-4 AccumulatorOp summary protocol`
**Status:** Accepted (design); implementation PR follows separately
**Companion:** `docs/adr_accumulator_op_v2.md`, `docs/design_v7.md` §4 + §6, `docs/accumulator_op_v2_production_plan.md` PR B-4

---

## TL;DR

> **Adopt Design B (column-group checksums) with two small additions:** a
> whole-slot checksum for fast equality and a reserved `flags` word for
> forward extensibility. No coarse semantic values. No GPU-side
> previous-summary comparison. `simthing-gpu` stays semantically generic;
> CPU does dirty-mask diffing against a cached previous summary.
>
> ```rust
> #[repr(C)]
> pub struct SlotSummaryGpu {
>     pub slot:            u32,
>     pub flags:           u32,
>     pub checksum_all:    u32,
>     pub _pad:            u32,
>     pub group_checksums: [u32; 4],
> }
> ```
>
> 32 B/slot (vs current 8 B). The summary tier supports dirty detection
> at slot and column-group granularity. It does **not** drive boundary
> skip alone, replace emission records, replace targeted full readback,
> or carry semantic values.

---

## 1. What is the production `SlotSummary` shape?

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct SlotSummaryGpu {
    /// Slot index; redundant with array position but lets CPU code do
    /// `summaries.iter().filter(|s| live_slots.contains(&s.slot))` without
    /// re-indexing. Matches current B-1 layout.
    pub slot:            u32,

    /// Reserved generic flag word. B-4 stakes no bit semantics. Future
    /// migration PRs may stake bits (e.g. `OVERFLOWED`, `EMITTED_FROM_SLOT`)
    /// in PR-specific docs. Reading code MUST mask only bits it knows;
    /// unknown bits are forward-compat reserved.
    pub flags:           u32,

    /// Bit-pattern sum of all f32 columns in this slot. Identical to the
    /// B-1/B-2 provisional `checksum`. Use for cheap "did anything change
    /// in this slot" equality against the cached previous summary.
    pub checksum_all:    u32,

    /// Padding for 16-byte alignment of `group_checksums`.
    pub _pad:            u32,

    /// Per-group checksum. The kernel divides `n_dims` into exactly four
    /// equal-or-near-equal contiguous groups in ascending column order;
    /// `group_size = ceil(n_dims / 4)` for groups 0..3, with group 3
    /// covering the remainder. `n_dims < 4` is supported by leaving
    /// trailing groups at 0. Use for narrower dirty detection and
    /// targeted column-range readback.
    pub group_checksums: [u32; 4],
}
```

CPU-side mirror:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotSummary {
    pub slot:            u32,
    pub flags:           u32,
    pub checksum_all:    u32,
    pub group_checksums: [u32; 4],
}
```

The CPU type drops `_pad` and exposes the same data. `bytemuck::cast_slice`
remains the readback path.

**Why 32 bytes:** quadruples current 8 B but stays cheap. At 100k slots
that is 3.2 MB readback per tick — well under 1 ms at typical PCIe
bandwidth (16 GB/s+). Full values at the same scale with n_dims=16 is
6.4 MB; with n_dims=64 it is 25.6 MB. Summary remains the lightweight
path.

**Why exactly 4 groups:** size-stable, vectorises cleanly in WGSL,
matches typical authored property layouts (Amount + Velocity + Intensity
+ up to one Named group), and avoids the "configurable group count"
trap that would force callers to declare a layout to consume summaries.
Designers needing finer granularity perform targeted column readback
after a group flags a change.

**Why a uniform `n_dims/4` grouping rather than role-aware groupings:**
keeping the GPU semantically generic. Role mapping (Amount-columns vs
Named-columns) is a `DimensionRegistry` concern and lives CPU-side. If
a future PR wants role-aware groups, it can add a `ColumnGroupLayout`
sidecar in `simthing-core` that maps group index → role; the GPU
contract does not change.

**Why not include `coarse_amount` / `coarse_intensity`:** rejected per
production plan §"Key caveats" item 1. Coarse semantic values would
either require GPU-side knowledge of `SubFieldRole` (semantic
contamination) or require column indices baked into the summary uniform
(brittle when `DimensionRegistry` grows). The same information is
recoverable by reading the targeted column range after a group
checksum changes — at the cost of one extra readback for genuinely
hot slots. The cost is bounded by the dirty-mask filter and is the
right tradeoff for keeping `simthing-gpu` semantics-free.

---

## 2. What is the summary tier allowed to decide?

The summary tier provides **change detection** at two granularities:

1. **Slot-level dirty**: `summary[i].checksum_all != cached_prev[i].checksum_all`
2. **Group-level dirty**: `summary[i].group_checksums[g] != cached_prev[i].group_checksums[g]`

It supports the following decisions:

| Decision | How |
|---|---|
| "Did anything in slot S change this tick?" | `checksum_all` equality |
| "Which column-group changed in slot S?" | per-group checksum equality |
| "Which slots are candidates for full readback?" | iterate summaries, filter by dirty |
| "Did emission overflow this tick?" | not via summary — read `emission_count` |
| "What event fired this tick?" | not via summary — read emission records |

The summary tier is **one input** to a boundary-skip decision (see §4),
not the whole decision.

---

## 3. What is the summary tier explicitly NOT allowed to decide?

1. **It does not drive hard structural triggers.** Fission / fusion /
   property expiry / capability unlock continue to flow through Pass 7
   `ThresholdEvent` and the A-4 `assert_no_hard_trigger_on_soft_aggregate`
   guard. `SlotSummary` is a change-detection signal, not a threshold
   crossing.
2. **It does not replace emission records.** `EmissionRecord` is the
   exact sparse event stream for `EmitEvent`-consume ops. Summaries
   say "slot changed"; emissions say "registration N emitted K units."
3. **It does not replace targeted full readback.** When CPU code needs
   the actual f32 value (e.g. to drive a `BoundaryRequest`), it must
   perform a targeted slot-or-column readback. The summary tells it
   *which* slot or column to read, not *what value* is there.
4. **It does not carry semantic role information.** No `coarse_amount`,
   no `coarse_intensity`, no resource names, no Recipe IDs, no faction
   labels. Role/recipe semantics live in `simthing-core` and `simthing-spec`.
5. **It does not enforce exactness for authoritative resources.** The
   underlying `values_buffer` is f32 today; checksum equality on f32
   bit patterns detects any change, but it does not establish
   conservation. Per the A-4 soft-aggregate guard, exact economic
   conservation in v7 §5.4 is checked via emission records and
   targeted readback, not summary equality.
6. **It does not measure its own GPU cost.** B-3's
   `last_pass_time_us()` measures the `execute_ops` pass only. If
   summary-pass timing becomes load-bearing, a future PR adds a
   separate query pair around `write_summaries`. B-4 does not require
   summary-pass timestamps for correctness.

---

## 4. How does the summary tier support boundary skip?

Boundary skip is a **multi-input** decision — see existing
`simthing-driver::SpecSessionState::requires_boundary_tick` (B3, 6
conditions). Summary readback adds two more conditions, both of which
must be evaluated as **necessary but not sufficient**:

```
A boundary may be skipped only if ALL of:
  1. SpecSessionState::requires_boundary_tick(events, registry) == false
     (encapsulates: no queued selections, no live cooldowns, no
      Predicate triggers, no OnPrereqMet, no CapabilityUnlock events,
      no ScriptedEventTrigger events — the existing B3 conditions)
  2. No structural pending requests from feeder/spec hook
  3. session.readback_emissions().is_empty()   // no GPU-resolved emissions
  4. summaries == cached_prev_summaries        // OR a narrower group-mask
                                               //   check declared by the
                                               //   migration PR that
                                               //   integrates summary
                                               //   into the boundary
```

The summary tier's contribution is **condition 4**. Existing B3
conditions remain authoritative — adding summary equality lets us also
skip ticks where no AccumulatorOp registration produced any state
change (a strictly stronger filter than "no events" alone, since some
ops can write without crossing a threshold).

**Important non-promise:** the summary tier does not say "no
structural work was needed." It only says "no per-slot f32 changed."
The driver still has to consult the existing B3 surface for queued
selections, cooldowns, etc.

---

## 5. How does the summary tier support replay / delta logging?

Per `design_v7.md §6.2`, replay is anchored by:

- `SpecSnapshot` (initial spec session state, from O2 replay v3)
- `EmissionRecord` compact log per tick (Pass C)
- Summary checksums per tick (for verification, not reconstruction)

The summary tier's job in replay is **divergence detection**, not
state reconstruction:

```
Replay verification per tick:
  expected_summary = recorded_summary_checksums[tick]
  actual_summary   = session.readback_summary()
  for each live slot:
    if expected_summary[slot].checksum_all != actual_summary[slot].checksum_all:
      // Divergence — surface to replay diagnostic, optionally fall back
      // to full readback on this slot for debugging
```

This is sufficient because:

- Transfer / emission are reconstructed from `EmissionRecord`.
- Deterministic ops re-derive the same state from the same inputs.
- Summary mismatch flags a divergence; the per-group checksums localise
  the divergence to one of four column groups; the debug-only full
  readback resolves to actual f32 values.

**The summary tier does NOT carry compact per-column deltas.** If a
future PR wants per-column delta logging, it builds on `EmissionRecord`
(extending the record shape) rather than overloading `SlotSummary`.

**Storage cost in replay:** 32 B × n_slots per tick at full snapshot
cadence is too much. Recommend recording summary checksums **only at
checkpoint cadence** (every N ticks, e.g. 60), not every tick. Between
checkpoints, replay relies on emission records + determinism. The
checkpoint cadence is a logging-tier policy, not a `SlotSummary`
shape question.

---

## 6. How does the summary tier interact with emission records?

They are **complementary**, not overlapping:

| Buffer | Volume | Purpose | Source |
|---|---|---|---|
| `SlotSummary[n_slots]` | dense, 32 B × n_slots | "did anything change here" | `write_summaries` kernel reads `values_buffer` after `execute_ops` |
| `EmissionRecord[n_emitted]` | sparse, 8 B × n_emitted | "registration N emitted K units" | `execute_ops` kernel writes during `EmitEvent` consume |

Specific overlap rules:

- An `EmitEvent` op writes both a value change to its target slot AND
  an emission record. The summary will mark that slot dirty; the
  emission record carries the exact `(reg_idx, emit_count)`. The CPU
  needs both: summary to gate "any structural work to do," emission
  record to drive the actual emission-handling logic.
- A non-`EmitEvent` op (transfer, identity, sum) writes a value change
  but emits no record. Only the summary detects that this op ran.
- `emission_count == 0` does **not** imply `summary_dirty == false`. A
  silent transfer (faction pool → factory queue) produces no emission
  but produces a summary change.
- `summary_dirty == false` does **not** imply `emission_count == 0`. A
  `ConsumeMode::EmitEvent` op with `scale: Constant(0.0)` could
  theoretically write a zero, but the B-2 kernel skips emit-record
  writes when `emit_count == 0u`. So in practice the two are
  consistent, but neither is the other's proxy.

**Do not overload `SlotSummary` to carry emission counts.** The
emission buffer is the right place; the summary is a slot-level
fingerprint.

---

## 7. How does the summary tier avoid soft-aggregate hard-trigger violations?

A-4 established the invariant: hard structural triggers
(`FissionTrigger`, `FusionTrigger`, `PropertyExpiry`,
`CapabilityUnlock`) may not register on `THRESH_BUF_OUTPUT` (post-
reduction) without a `SoftAggregateGuard` on the relevant `SubFieldSpec`.

The B-4 summary tier preserves this invariant by **not introducing any
new threshold registration shape.** The summary kernel reads
`values_buffer` (pre-reduction post-AccumulatorOp state) and writes
checksums. Nothing in the summary path produces a `ThresholdEvent`,
nothing creates a new structural trigger source.

If C-5 (WeightedMean migration) later wires a soft-aggregate value
into a structural decision, A-4's
`assert_no_hard_trigger_on_soft_aggregate` validator catches it at
registration time. The summary tier neither enables nor disables that
gate.

The only adjacent concern: a CPU consumer reading per-group checksums
might mistakenly conclude "group 2 changed → group 2's columns are
authoritative." That is wrong — `checksum != prev_checksum` says
"something here changed," not "this value is exact." Document this
explicitly in the CPU `SlotSummary` doc-comment to forestall the
"checksum-as-truth" reading.

---

## 8. How does the summary tier avoid smuggling economic semantics into `simthing-gpu`?

Three structural protections:

1. **Group count is fixed at 4, derived from `n_dims` only.** The kernel
   computes `group_size = (n_dims + 3) / 4` and partitions columns by
   ascending index. No `Amount-vs-Velocity-vs-Named` knowledge, no
   property-id input.
2. **No semantic fields.** No `iron_ore_amount`, no
   `factory_queue_debt`, no `population_growth`. The summary uniform
   carries only `n_slots`, `n_dims`. Adding semantic fields would
   require `DimensionRegistry` shape in the GPU uniform, which violates
   the `simthing-sim` ↔ `simthing-gpu` boundary.
3. **`flags` bits are reserved at B-4.** Future PRs may stake bits but
   each new bit must justify itself as generic (e.g. "emission produced
   from this slot" is generic; "iron pool depleted" is not). The
   reservation is documented in `flags`' doc-comment.

If a designer needs role-aware group readback in the future (e.g. "give
me the checksum of all Velocity columns across slot S"), the right
mechanism is a CPU-side `ColumnGroupLayout` mapping
`SubFieldRole → group_index_set` that filters the summary readback.
That layout lives in `simthing-core`; the GPU contract is unaffected.

---

## 9. What implementation PR should follow B-4?

**Proposed PR title:** `feat(gpu): implement B-4 AccumulatorOp summary protocol`

**Model:** Composer 2.5 (mechanical implementation of accepted design).

**Files to change:**

- `crates/simthing-gpu/src/accumulator_op/types.rs`
  - Replace `SlotSummary { slot, checksum }` with the 4-field shape.
  - Replace `SlotSummaryGpu` with the 5-field `#[repr(C)]` shape (incl. `_pad`).
  - Update `slot_checksum(values, slot, n_dims)` to also compute the four
    group checksums; rename to `slot_summary(values, slot, n_dims) -> SlotSummary`.
  - Update `summaries_from_values` to use the new shape.
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl`
  - Extend `SlotSummaryGpu` struct.
  - `write_summaries` kernel computes `checksum_all` (existing logic) plus
    four `group_checksums` (each group sums bit patterns over its column range).
  - `flags` is written as 0 at B-4; future PRs OR additional bits.
- `crates/simthing-gpu/src/accumulator_op/session.rs`
  - Update `readback_summary` mapping `SlotSummaryGpu` → `SlotSummary`.
  - Update `summary_buffer` size computation (`SlotSummaryGpu` is now 32 B).
- `crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs`
  - Mirror the per-group computation in the CPU oracle.
- `docs/design_v7.md` §6.1
  - Update the "Summary (default production)" entry to mention group
    granularity (volume now "32 B × n_slots per tick").
- `docs/accumulator_op_v2_production_plan.md` PR B-4
  - Mark the design half complete; reference this memo as the gate.
- `docs/agents.md`
  - One-line update to the summary tier description if applicable.

**Tests to add (per the production plan's test list):**

- `summary_format_roundtrip` — `SlotSummaryGpu` ↔ `SlotSummary` cast.
- `checksum_all_changes_when_one_column_changes` — single-column write
  flips `checksum_all` AND exactly one `group_checksum`; other three
  groups unchanged.
- `group_checksum_isolates_changed_group` — write to column in group 2
  flips `group_checksums[2]` only.
- `summary_size_stable_with_small_n_dims` — `n_dims < 4` does not panic;
  trailing groups remain 0.
- `summary_size_stable_with_large_n_dims` — `n_dims = 64` produces
  16-column groups.
- `emission_records_independent_of_summary` — `EmitEvent` op produces
  both a summary change AND an emission record; the two readbacks
  remain in their own buffers and don't cross-contaminate.
- `timestamp_query_path_still_works` — B-3 timestamps still cover
  `execute_ops` only after the summary kernel grows.
- `readback_full_remains_debug_only` — guarded readback warning still
  fires.
- `cpu_oracle_matches_gpu_per_group` — `summaries_from_values` CPU
  reference matches GPU output bit-exactly.
- `flags_field_zero_at_b4` — guard that no implementation accidentally
  stakes a flag bit before its PR documents it.

---

## 10. Decision rationale: why B (with two additions) and not A / C / D?

| Option | Verdict |
|---|---|
| **A** (checksum only) | Insufficient. Forces full-slot readback after any change. Workshop persistent-buffer numbers (design_v7 §8) depend on narrower readback for ≥10× wins; checksum-only forces 100% readback on dirty slots. |
| **B** (group checksums) | **Selected, with additions.** Strikes the bandwidth/specificity balance. Generic. Compatible with B3 boundary-skip and Replay v3 (O2). |
| **C** (dirty bitmask + checksum) | Rejected for B-4 implementation. Computing dirty mask on GPU requires `previous_summary` buffer or a per-tick memcpy. CPU-side dirty mask diffing (compare cached previous against current) achieves the same with simpler GPU state. Reconsider in a future PR if profiling shows CPU-side mask computation is a bottleneck. |
| **D** (hybrid with semantic flags) | Rejected as written — `soft_aggregate_touched`/`exact_column_touched` flags require GPU-side `SubFieldRole` knowledge. Generic version is included as the `flags: u32` field with B-4-reserved bits and future-PR stakes. |

**Two additions on top of Design B:**

1. **`checksum_all`** in addition to `group_checksums[4]` — costs 4 B per
   slot and saves the CPU from XOR-ing the four group checksums in the
   common "any change?" path. Net throughput improvement.
2. **`flags: u32`** — reserved at B-4 with no semantics. The cost is 4 B
   per slot. The benefit is that subsequent PRs (C-1 threshold migration,
   E-1 emit-on-threshold builder) can stake bits without a layout
   migration. Without this slot today, every future flag becomes a
   schema change.

---

## 11. Out of scope for B-4 (deferred to later PRs)

- **GPU-side previous-summary buffer + on-GPU dirty mask computation.**
  Deferred until profiling shows CPU-side diffing is a bottleneck.
- **Role-aware column groupings.** Requires
  `simthing-core::ColumnGroupLayout` design; CPU-side, doesn't change
  GPU contract.
- **Summary-pass timestamp queries.** B-3 covers `execute_ops` only.
  Add a second timestamp pair around `write_summaries` only when
  summary-pass timing becomes load-bearing.
- **Checkpoint cadence for replay summary recording.** Logging-tier
  policy decision; not a summary-shape decision.
- **BoundaryProtocol integration of `AccumulatorOpSession`.** That is
  the C-family migration scope. B-4 only specifies the readback
  contract such integrations consume.
- **Integer / fixed-point representation for exact-conservation columns.**
  Separate question, raised in the handoff caveat 5. Tracked but not
  resolved by B-4.

---

## 12. Sign-off checklist

Per PR B-4 acceptance:

- [x] Production `SlotSummary` shape specified (§1)
- [x] Allowed decisions enumerated (§2)
- [x] Disallowed decisions enumerated (§3)
- [x] Boundary-skip interaction defined (§4)
- [x] Replay / delta logging interaction defined (§5)
- [x] Emission-record interaction defined (§6)
- [x] Soft-aggregate hard-trigger preservation (§7)
- [x] Economic-semantics avoidance (§8)
- [x] Implementation PR scope defined (§9)
- [x] Decision rationale documented (§10)
- [ ] Human + Opus sign-off on this memo (this PR requests it)

---

## References

- `docs/adr_accumulator_op_v2.md` — ADR (semantic scope, soft-aggregate policy)
- `docs/design_v7.md` §4 (pipeline), §6 (logging tiers), §8 (performance model)
- `docs/accumulator_op_v2_production_plan.md` PR B-4
- `docs/workshop/archive/soft_aggregate_tolerance_audit.md` (PR A-4, archived)
- `crates/simthing-gpu/src/accumulator_op/types.rs` — current provisional `SlotSummary`
- `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` — current `write_summaries` kernel
- `crates/simthing-driver/src/spec_session.rs` — B3 `requires_boundary_tick`
