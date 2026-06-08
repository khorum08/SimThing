# RUNTIME-0080-0-R1c Opening Handoff — resident structural decision authority (and the anti-drift contract)

**Intended recipient:** orchestration / next production implementation agent
**Authority:** RUNTIME-0080-0 production track after R1b
**Predecessor:** `RUNTIME-0080-0-R1b-IMPL-0` (resident event journal) — IMPLEMENTED / PARTIAL, full journal parity earned
**Report:** [`docs/tests/runtime_0080_0_r1b_resident_event_journal_results.md`](../tests/runtime_0080_0_r1b_resident_event_journal_results.md)

## 1. Why this handoff exists

R1b shipped a resident event journal that now matches the R6C CPU oracle **exactly** (all event kinds, all 100 ticks). Getting there exposed a structural-drift failure mode that the orchestration must not let recur:

> An over-eager "GPU-side per-tick authority" tried to **reconstruct** the CPU boundary witness every tick from **partial** GPU Tier-A readback. Tier-A columns are value-only (disruption, stockpiles, construction_progress, existing-slot `num_ships`, blockade). They do **not** carry structural state (births, removals, fusion lineage, per-fleet positions). Reconstructing structural state from value-only readback silently dropped it, the witness drifted from the oracle, and the two GPU-decided combat ticks produced nothing.

R1b fixed this by making the CPU decision/structural state **self-consistent** (it carries its own state forward and is never rebuilt from partial readback), while the GPU value loop runs **resident and independent**. R1c must preserve that separation as it moves the structural *decision* itself onto the GPU.

## 2. The CPU shadow is always a complete, serializable, pausable mirror (load-bearing invariant)

This is the invariant the drift violated, and it is **non-negotiable for every rung, including after the GPU becomes the authority**:

- **Complete.** The CPU keeps a *full* mirror of world state in CPU land — every Tier-A value column **and** all Tier-B structural state (membership, per-fleet positions, births, removals, fusion lineage, slot allocation). It is never reduced to a partial projection, and never reconstituted from a partial projection (e.g. value-only Tier-A readback). The GPU may hold the *authority*; the CPU still holds the *whole world*.
- **Serializable.** At any tick boundary the complete shadow can be written out as a self-contained save with no GPU residency required to interpret it. A save taken on a discrete-GPU host must reload and continue identically on a host with no discrete GPU (CPU-only replay), because the shadow alone is sufficient world state. No field may live *only* in a GPU buffer.
- **Pausable / resumable.** The simulation can stop at any tick boundary (the save/pause/stable-state point) and resume bit-identically from the serialized shadow. Resume seeds the GPU resident buffers *from* the complete shadow — never the reverse-only direction where the shadow depends on a live GPU to be whole.
- **Reconciled, not replaced.** GPU authority and the CPU shadow are reconciled at the tick boundary via the journal (GPU-decided → CPU-applied). The shadow is the durable system of record and the parity witness; the resident buffers are the fast path. Deleting or thinning the shadow to "save memory" is forbidden — it breaks save/pause, cross-adapter replay, and semantic-GPU parity.

Concretely for R1c: the resident structural tables are an *acceleration* of the shadow, not a replacement. Every tick the journal must be sufficient to bring the complete CPU shadow forward to match the resident state exactly, so that serialize-at-boundary always yields a whole, GPU-independent world.

## 3. The non-negotiable anti-drift contract (applies to R1c and every later rung)

1. **Authority is partitioned, never half-reconstructed.**
   - Tier-A *value* columns: GPU-resident authority (R1a).
   - Tier-B *structural* state (membership, positions, births, removals, fusion lineage, slot allocation): a single coherent authority that is **either** fully resident **or** fully CPU-shadow — never a per-tick rebuild of one from a partial projection of the other.
2. **No structural state may be reconstructed from a value-only projection.** If a consumer needs structural state, it reads the structural authority directly (resident structural tables in R1c, or the self-consistent shadow in R1b), not the Tier-A value columns.
3. **The journal is the only cross-tier structural hand-off.** Bounded boundary maintenance consumes journal rows and applies them to the complete CPU shadow; it must never re-derive decisions from value columns.
4. **Equivalence is a test, not a hope.** Keep an assertion that the complete CPU shadow equals the CPU oracle every tick (R1b proves this via exact per-tick journal parity). Any rung that adds structural residency must keep an equivalent per-tick structural-parity gate so drift is caught at the tick it starts, not 44 ticks later. Add a serialize→reload→continue round-trip test so the shadow's completeness is proven, not assumed.
5. **Journal fields must be representable.** The resident journal fill rejects non-finite f32. Never bit-cast a signed/large integer into a journal slot (negative ints and exponent-0xFF patterns become NaN/Inf and are rejected). Store signed deltas as exact f32 values; reserve bit-cast only for values known finite (small u32 ids, finite magnitude bit patterns). Add a debug assertion that every staged field `is_finite()`.

## 4. The "do not starve the GPU on the CPU" contract

The drift temptation came from coupling the GPU tick to the CPU boundary pass. Forbidden. The pipeline must be **decoupled** — and decoupling is what makes the complete shadow (§2) affordable without stalling the GPU:

1. **GPU value loop runs resident and ahead.** The Tier-A loop advances on its own resident buffers from per-tick derived inputs. It must not block on a GPU→CPU readback to make progress, and must not wait for the CPU boundary drain. (R1b removed the per-tick decision readback; keep it removed.)
2. **The journal is a double-buffered decoupling queue.** The GPU writes tick `N+1` structural decisions into journal buffer B while the CPU drains tick `N` from buffer A. Neither side stalls the other.
3. **CPU boundary maintenance runs at the boundary cadence** (save/pause/stable-state point), draining accumulated journal rows in bounded batches to bring the complete CPU shadow (§2) forward. It is a consumer, never a gate on GPU progress. The shadow is whole and serializable at each such boundary.
4. **Readbacks are batched and lagged, never per-tick-synchronous on the hot path.** Any remaining small readback (e.g. the R4 Candidate-F max-magnitude reduction) must be pipelined/batched so a single tick never round-trips GPU→CPU→GPU synchronously. Treat a per-tick synchronous readback in the decision path as a defect.
5. **Backpressure, not stalls.** If the CPU consumer falls behind, the journal grows (or applies a bounded ring with a documented high-water mark) — the GPU does not wait. Document the chosen policy. Note the resulting bound on how far the resident state may run ahead of the last fully-serializable shadow boundary.

## 5. R1c scope (what flips the PARTIAL to PASS)

R1b's only remaining gap is `structural_decisions_gpu_emitted = false`: the CPU decision witness still *decides* the structural events. R1c moves that decision authority resident:

- resident REENROLL / membership scatter for movement targets (the GPU chooses the destination, not the CPU);
- resident cohort birth / removal (zero-cohort) slot allocation;
- resident fusion lineage / compaction.

When the GPU emits these decisions into the journal (and the boundary pass still only *applies* them), set `structural_decisions_gpu_emitted = true`; the existing R1b PASS gate then yields PASS.

This sits **behind** the §11 / free-list-scatter stop-lines and the M-4A / multi-atlas gate. Do not pull those forward. Whatever resident structural tables R1c adds, they must remain an acceleration of the complete CPU shadow (§2), never its replacement.

## 6. Stop conditions (return to Opus)

Stop if R1c appears to require: a scenario-specific GPU compute pass; a semantic event kind tied only to 0080-2; a CPU planner or CPU redecision of structural events; M-4A / multi-atlas; pinned-number change; scenario reopen; `docs/invariants.md` edit; loosening the R4 f32 bound; or no discrete GPU in the expected environment. A generic, semantic-free substrate primitive remains admissible under the §2.4 / §4a gate with bit-exact CPU-oracle parity.

## 7. Read first

1. `crates/simthing-driver/src/runtime_0080_0_r1b.rs` (the decoupled loop + journal round-trip + boundary shadow)
2. `R1aBoundaryWitness::step_tick_capture_events` in `crates/simthing-driver/src/dress_rehearsal_r6c_integrated_run.rs` (the self-consistent CPU decision witness)
3. `crates/simthing-driver/src/runtime_0080_0_r1a.rs` (Tier-A resident value authority)
4. `docs/tests/runtime_0080_0_r1b_resident_event_journal_results.md`
5. `docs/production_paths/runtime_0080_0_r1_next_tick_authority_spec.md`

## 8. Terminology

Use PR #539 domain-neutral terms: `FieldPolicy`, `field_agent`, `selection`, `extraction`, `resident event journal`, `GPU-side structural event rows`, `disabled-transform parity check`. Do not reintroduce normalized terms.
