# SimThing — ChatGPT Implementation Review
## Architectural Fidelity, Performance Impact, and Recommended Remediation

> **V6 addendum (2026-05-22):** Suspended overlays, `ActivateOverlay`/`SuspendOverlay`,
> and capability-subtree fission clone landed in `f39fe6d`. Static-boundary skip now
> treats suspended overlays as inert (no lifecycle work). GPU overlay prep skips
> suspended entries — no WGSL changes. Studio capability-tree semantics are documented
> in `capability_tree_v1.md`; simulation crates remain agnostic. Open V6 guardrails:
> see `docs/todo.md` Priorities 1–3.

---

## Executive Summary

The ChatGPT-implemented changes constitute a mixed result. Three changes are genuine wins that
align with or improve on the GPU-forward design intent. Three changes introduce architectural
debt that partially undermines it. One change contains a correctness subtlety that is currently
safe but structurally fragile.

The GPU-forward design intent is **not fundamentally compromised**, but the boundary layer
accumulated more CPU overhead than the design calls for, and the patcher has a per-tick
allocation pattern that will become visible under sustained load.

---

## The Three Genuine Wins

### 1. The Intent Delta Affine Fold — Correct and Well-Designed

**The change:** `apply_collected_as_intents` folds all tick-time transforms into per-cell
`IntentDelta { slot, col, mul, add }` records before GPU dispatch, eliminating the read-modify-
write problem for `Add`/`Multiply` operations without requiring a GPU readback.

**Assessment: Sound.** The fold algebra is correct:
- `Set(k)` → `(mul=0, add=k)` — resets accumulated state correctly
- `Add(a)` → `add += a` — commutes correctly with any pending mul
- `Multiply(m)` → `(mul *= m, add *= m)` — distributes correctly through any pending add
- Identity `(1.0, 0.0)` for an untouched cell — correct

The decision to apply intent deltas **before Pass 0** (snapshot) is the right call for the
design: intent-driven changes become part of the reference frame for threshold detection,
preventing phantom crossings from command submission. The comment explaining this is
accurate and complete.

The intentional asymmetry — that threshold crossing detection cannot detect same-tick
intent-delta-induced crossings — is correct per the overlay-as-standing-register model.
Intent deltas are not events; they are state updates. The threshold system watches for
integration-driven drift, not command execution.

**What it fixes:** eliminates all per-slot GPU row readbacks during normal tick execution.
The benchmark shows this correctly as 0 RMW readback bytes under normal operation.

---

### 2. The Static Boundary Fast-Path — Architecturally Correct

**The change:** `can_skip_empty_boundary` checks three conditions (no events, no pending
requests, no transient lifecycle work) and allows the entire `execute()` sequence to be
bypassed.

**Assessment: Sound.** The three conditions are necessary and sufficient:
- No threshold events → no fission/fusion/expiry triggers
- No pending requests → no structural mutations or overlay attaches
- No transient lifecycle work → no overlay expiry or CPU-decay properties

This is not a heuristic shortcut. The three conditions together guarantee that the GPU
state, tree structure, and shadow are already consistent and no action is required.
The static map benchmark result (~25 ms/day at 1M slots) validates the impact.

The implementation correctly propagates this to `record_to_path` as well, writing an
empty frame to the replay log rather than silently skipping it, which preserves replay
fidelity.

---

### 3. The Sparse Dirty-Row Tracker — Correct Micro-Optimization

**The change:** patcher tracks dirty slots in a separate `Vec<u32>` alongside the `Vec<bool>`
bitmap, avoiding an O(n_slots) bitmap scan to find dirty rows.

**Assessment: Sound.** This is a straightforward improvement. The dual-structure approach
(bitmap for O(1) dedup, slot list for O(dirty) iteration) is the standard pattern. The
`apply_collected_as_intents` hot path does **not** touch the dirty bitmap at all, meaning
the dirty mechanism is correctly isolated to the legacy/direct shadow path.

---

## The Three Problems

### Problem 1: Eight Separate O(n) Tree Traversals Per Non-Skipped Boundary

**What was implemented:** Each step of the boundary protocol is a separate module with its
own full tree traversal:

```
1. resolve_overlay_lifecycle          → full tree walk
2. resolve_property_expiry            → full tree walk
3. build_node_paths (fission)         → full tree walk
4. find_node_mut per request          → O(n) per structural mutation request
5. build_overlay_deltas (gpu_sync)    → full tree walk
6. ThresholdBuilder::build_with_lineage → full tree walk
7. build_topology (reduction)         → full tree walk
8. entries_from_outcome (delta_log)   → full tree walk
```

**What the design calls for:** the design document sketches a single-pass evaluation
algorithm. It never describes separate passes for each concern. The boundary protocol
in §10 lists sequential steps, but those steps are conceptually phases within one
traversal, not eight independent passes over 11,520 nodes.

**Impact:** At endgame scale (11,520 nodes), eight passes is ~92,160 node visits per
boundary. This is visible in the benchmark timing breakdowns: `boundary_structural_ms`,
`boundary_lifecycle_ms`, `boundary_expiry_ms`, `boundary_delta_log_ms` etc. are all
separate, additive costs. The worklog shows `fission_stress` boundary time at ~30 ms
even after the parent-lookup and delta-log indexing optimizations — the remaining CPU
cost is distributed across these passes.

**The inconsistency:** `fission.rs` received a `build_node_paths` one-pass index
(correctly) as a targeted optimization. `tree_mutation.rs` still uses `find_node_mut`
for each structural mutation request separately. This is architectural inconsistency,
not policy: the fission optimization was done reactively when the benchmark showed pain,
while the underlying pattern that produced the pain — multiple separate passes — was
left intact everywhere else.

**What should happen:** The boundary protocol should build one `SimThingId → (path, &node)`
index at the start and pass it to all downstream steps. This would reduce eight O(n) passes
to one O(n) indexing pass plus eight O(touched_nodes) passes. At static-tree scale the
difference is already meaningful; at fission-growth scale it is the dominant CPU cost.

---

### Problem 2: Full Shadow Readback + Full Shadow Upload Every Non-Skipped Boundary

**What was implemented:** `boundary.rs` step 0 reads the entire GPU `values` buffer back
to CPU (`coord.shadow = state.read_values()`). `gpu_sync.rs` step 3 unconditionally calls
`coord.upload_full_shadow(state)` which writes the entire shadow back to GPU.

**The justification given:** integration output lives only on the GPU; without the readback,
`upload_full_shadow` would overwrite it. This is correct.

**The problem:** the justification for readback is sound, but it does not justify the full
upload. After the readback:
- Overlay lifecycle writes to specific shadow rows (dissolved rows only)
- Property expiry tombstones specific rows
- Fission seeds specific new rows
- Structural mutations touch specific rows
- AddDimension wides specific columns

The upload could target only the rows and columns that were actually mutated during the
boundary. Instead, the full `n_slots × n_dims × 4B` shadow is uploaded unconditionally.
At endgame scale this is ~3 MB of upload every boundary regardless of activity.

**The comment is revealing:** the `gpu_sync.rs` comment says "Callers that only had dirty-row
patches can call upload_row individually; here we flush the full shadow to keep correctness
simple at boundary time." This is a correctness-simplicity tradeoff made at implementation
time — not a design decision. The phrase "keep correctness simple" is a red flag: it means
correctness was the priority and performance was accepted as a casualty.

**The design intent:** the design's explicit memory budget calculation showed that ~3 MB
per boundary was "negligible." That is true at human-playable speed (1 boundary/second).
At accelerated time (10+ boundaries/second in fast-forward), 3 MB × 10 = 30 MB/s of
unnecessary GPU upload bandwidth, plus the cost of the readback.

**What should happen:** The boundary should track which rows were actually written during
lifecycle/expiry/fission/structural passes and upload only those rows. The fission
pre-grow path already does amortized growth correctly; a similar dirty-row approach
at boundary time would complete the picture.

---

### Problem 3: Per-Tick HashMap Allocation in the Intent Delta Hot Path

**What was implemented:** `apply_collected_as_intents` allocates a fresh `HashMap<(u32, u32),
(f32, f32)>` and `Vec<(u32, u32)>` on every call, which is called every tick.

```rust
let mut order: Vec<(u32, u32)> = Vec::new();
let mut folded: HashMap<(u32, u32), (f32, f32)> = HashMap::new();
```

**Impact:** At `intent_stress` scale (10,000 patches per tick, 4 ticks/day), each tick
allocates and populates a HashMap with up to 10,000 entries, then drops it. The allocator
pressure from these short-lived HashMaps will be visible under sustained load, particularly
under the GC pressure of the backing allocator's free-list management.

**The contrast:** the dirty-row tracker correctly uses persistent structures with `clear()`
rather than re-allocate. The intent fold does not follow the same pattern.

**The existing patcher structure** already has fields that persist between drains
(`pending_boundary`, `pending_player_intents`, `dirty`, `dirty_slots`). There is no structural
reason the fold accumulator could not be a reusable field on `TransformPatcher` that gets
cleared at the start of each drain rather than reallocated.

**What should happen:**

```rust
pub struct TransformPatcher {
    // ... existing fields ...
    fold_order: Vec<(u32, u32)>,           // cleared and reused each tick
    fold_accum: HashMap<(u32, u32), (f32, f32)>,  // cleared and reused each tick
}
```

This is a small, non-breaking change with measurable benefit under `intent_stress` load.

---

## The Structural Fragility (Watch Item)

### The Dual Path Problem in TransformPatcher

`TransformPatcher` now has two distinct execution paths:

- `apply_collected` (legacy/shadow path): mutates `coord.shadow`, marks dirty rows,
  skips `Add`/`Multiply` without `GpuSynced` freshness
- `apply_collected_as_intents` (hot path): produces `Vec<IntentDelta>`, does NOT
  touch shadow, does NOT mark dirty rows

These two paths share the same struct but have different side effects. The hot path is
called from `tick()`. The legacy path is called from `drain()`, which is not called
from the normal tick loop at all — it exists for tests and replay-style callers.

**Current state:** this divergence is intentional and documented in `state-authority.md`.
It is not a bug. The shadow catches up via the GPU readback at every boundary.

**The fragility:** a future feature that needs mid-tick shadow accuracy — such as an
AI system that queries `observe()` between ticks on a recently intent-patched entity —
will see stale shadow values without realizing it. `observe()` is documented as potentially
lagging, but the documentation does not make clear that *intent-patched rows are always
stale mid-tick*, unlike rows that received dirty-path patches.

The asymmetry is subtle: `observe()` after a `Set` via dirty path is accurate mid-tick.
`observe()` after a `Set` via intent path is stale mid-tick (shadow not updated).
Both look identical to the caller.

**What should happen:** `observe_live()` documentation should explicitly note that mid-tick
calls on intent-patched rows require the GPU-row fidelity path. Alternatively, `observe_live`
could be the *default* for any entity that received an intent delta this tick, with shadow
as a fallback for unpatched rows.

---

## GPU-Forward Design Intent: Preserved or Compromised?

**Preserved:** The GPU is still doing the simulation. Integration, intensity, overlay
application, reduction, and threshold detection are all GPU-native. The intent delta path
correctly pushes tick-time transforms onto the GPU without CPU round-trips. The static
boundary fast-path correctly avoids unnecessary GPU work. These are GPU-forward decisions.

**Partially compromised at the boundary:** The full shadow upload every non-skipped boundary
is CPU-forward thinking applied to a situation where dirty-row tracking would be more GPU-
forward. It uploads data the GPU already has, because it's simpler than tracking which rows
actually changed. This is a correctness-first, GPU-bandwidth-second decision.

**Not compromised architecturally:** the eight separate tree passes are CPU overhead, but
they are correctness-complete and all feed GPU buffer uploads. The boundary is intentionally
a CPU moment; the question is whether it should be *this much* CPU per node. The answer is
no, but the issue is throughput, not architectural direction.

---

## Summary Table

| Change | Fidelity | Performance | Verdict |
|---|---|---|---|
| Intent delta affine fold | ✅ GPU-forward | ✅ Eliminates RMW readbacks | **Keep as-is** |
| Static boundary fast-path | ✅ Correct guard | ✅ Zero cost for quiet days | **Keep as-is** |
| Sparse dirty-row tracker | ✅ Correct pattern | ✅ O(dirty) not O(n) | **Keep as-is** |
| Eight O(n) boundary passes | ⚠ More than design sketched | ⚠ Additive CPU cost at scale | **Consolidate into shared index** |
| Full shadow readback + upload | ⚠ Necessary read, unnecessary full write | ⚠ 3MB per boundary unconditionally | **Add boundary dirty-row tracking** |
| Per-tick HashMap allocation | ✅ Correct semantics | ⚠ Allocator pressure at intent_stress scale | **Reuse patcher-owned accumulators** |
| Dual apply_collected paths | ⚠ Structurally fragile | ✅ No current correctness issue | **Document mid-tick shadow staleness explicitly** |

---

## Recommended Action Order

1. **Move fold accumulators to TransformPatcher fields** (1 day, no risk) — eliminates per-tick
   allocation. Clear instead of reallocate. Directly measurable on `intent_stress`.

2. **Unify fission node-path index with tree_mutation** (1 day, low risk) — fission already
   builds the index; pass it to `apply_structural_mutations` rather than having it re-scan.
   Extend to overlay lifecycle and property expiry with the same index.

3. **Add boundary dirty-row tracking for the shadow upload** (2–3 days, moderate risk) —
   track which rows were written during lifecycle, expiry, fission-seed, and structural
   mutation phases; upload only those rows rather than the full shadow. The readback
   remains (it is necessary); only the upload becomes targeted.

4. **Consolidate the remaining boundary passes into a shared tree walk** (3–5 days,
   architecture work) — build one `BoundaryIndex` at boundary start, pass it through
   lifecycle → expiry → fission → mutation → gpu_sync → delta_log rather than having
   each module independently traverse the tree.

Item 4 is the most impactful at scale but the highest risk. Items 1–3 are unambiguous
improvements with minimal architectural disruption. The benchmark tooling already in place
(`simthing bench` with phase attribution) will directly measure the effect of each.
