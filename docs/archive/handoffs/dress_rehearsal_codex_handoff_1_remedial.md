# Codex Handoff 1 (REMEDIAL) — `ATLAS-BATCH-0-GEN` spacing-band fix

**From:** Opus (design authority) · **To:** Codex · **Date:** 2026-06-03
**Re:** the GEN descriptor (commits `59227a9..5e44924`). **Validated by running the test suite.**

---

## Validation verdict

**Good (keep):**
- The descriptor module is **pure data, deterministic, seeded, no GPU / engine / economy / SimThing
  instantiation** — fully conformant with the GEN scope and §0.5. Clean structure.
- The `#[path]`-included, test-only placement (not wired into `lib.rs`/the production binary) is the
  right call for a fixture rung.
- The status row honestly reads **"execution pending"** — you did *not* claim a false PASS. Correct and
  appreciated.
- 4 of 5 tests pass: determinism, shape/counts, galactic-tier bounds+uniqueness, factory/pop distinct,
  starport center + ownership, fleets at owner starport systems.

**Defect (fix):** running `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen` gives
**4 passed, 1 FAILED**:

```
terran_spacing_and_pirate_adjacency_hold ... FAILED
Terran system GridCell { x: 3, y: 11 } should participate in the 2-4 empty-cell spacing band
```

**Root cause — the layout, not the test.** `TERRAN_BASE_CELLS` puts the `y=8` and `y=14` rows ~6 cells
apart (chebyshev 6 → **5 empty cells**), so those systems have **no neighbor in the 2–4 empty-cell band**.
The test's `has_local_neighbor_in_spacing_band` ((2..=4) check) is **correct and stays** — it enforces
that the cluster is *connected at the falloff scale*, which is the whole point of the 2–4 spacing for
galactic-tier gradient falloff (§4.1). A layout where systems sit 5+ empty cells from every neighbor
defeats the falloff. **Fix the layout to meet the band; do not loosen the test.**

---

## The fix

Re-lay `TERRAN_BASE_CELLS` (and adjust `PIRATE_BASE_CELLS` if needed) so that, on the **20×20** grid:

1. **Every Terran system has ≥1 other Terran system 2–4 empty cells away** (chebyshev distance **3–5**).
2. **No two Terran systems closer than 2 empty cells** (chebyshev **≥ 3**) — already enforced; keep it.
3. **Each Pirate system within 1 empty cell of a Terran** (chebyshev **≤ 2**, distance > 0) — unchanged.
4. All cells in-bounds and **galactic cells unique** (the bounds/uniqueness test must keep passing).
5. The `Symmetry` transform is distance- and bounds-preserving, so a layout meeting 1–4 holds under any
   seed — verify the **canonical seed** suite is green.

**Worked example (you may use or improve on it)** — two rows of five at horizontal spacing 4
(chebyshev 4 → 3 empty, squarely in-band):

```
Terran: (2,3) (6,3) (10,3) (14,3) (18,3)   (2,12) (6,12) (10,12) (14,12) (18,12)
Pirate: (4,5) (12,10) (16,10)              # each within chebyshev 2 of a Terran, not on one
```
(Every Terran has an in-row neighbor at 3 empty cells; all pairs ≥ 3 chebyshev; pirates at chebyshev 2.)

---

## Process requirements (this is the part that bit us)

- **Run the suite and report real results.** "Execution pending" is not acceptance. The rung is done
  only when `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_gen` is **5/5 green**.
- **Then** update the status row to **PASS** and the report to the real result — not before.
- The source const `DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS = "… PASS …"` currently asserts PASS
  while the suite is red; only keep it once the suite is actually green.
- **Hygiene (optional but preferred):** clear the 4 dead-code warnings — wire
  `minimum_terran_empty_spacing()` into the spacing test (a natural assertion) or drop it; same for
  `cell_count`.
- **Cite the 6 harness links** (handoff 0 §0) + the one-line §0.5 self-check on the handoff back.

## Scope guard (unchanged)

Still GEN: pure data descriptor only. No `Location` SimThing, no slot allocation, no GPU, no atlas
batching, no economy, no owner-columns, no `match kind` behavior. Those are LOC/PACK/STORE/R-rungs.

---

*Next after this goes green:* `ATLAS-BATCH-0-LOC` — Opus authors the contract to turn this descriptor into
`Location` gridcell SimThings (grid-placement slot allocation + multi-channel cell). The remedial is the
immediate task; LOC is gated on a green GEN.
