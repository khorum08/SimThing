# RUNTIME-0080-0-R2 Results

> ## ⚠ DEVIATION RECORD (required by `invariants.md` → Specification Fidelity & Anti-Ceremony; constitution §0.6)
>
> **Added 2026-06-07 to bring this closure into compliance with the Specification Fidelity invariant.**
> The R6C/R2 rehearsal **flattened the specified recursive structure** of
> [`scenario_0080_2_dress_rehearsal_spec.md`](../scenarios/scenario_0080_2_dress_rehearsal_spec.md)
> into a single galactic tier. This deviation was **not** recorded at original closure; it is recorded
> now. The 100-tick GPU-forward runtime proof (Tier-A residency, journal, structural substrate, GPU
> ZeroCohort, R6C checksum-equivalence) **stands**; what is corrected is the honest scope claim.
>
> ### Specified vs Implemented ledger
> | Specified element (dress-rehearsal spec) | Status | Note |
> |---|---|---|
> | Galaxy starmap, 20×20 gridcells | `implemented` | flat galactic grid; matches |
> | Star system as a **10×10 subgrid** (×13, one per galactic cell) | **`proxied`** | systems are **flat galactic cells**; no 10×10 system subgrid is run |
> | Planet (Location) per system | **not implemented** | no planet entity in the running world |
> | Planet **surface 10×10** map | **not implemented** | no surface tier exists |
> | Factory district (surface-cell building) | **not implemented** | no surface buildings |
> | Pop cohort (surface-cell building) | **not implemented** | no population entity |
> | Planet-surface **labor economy** (pop +10 labor/tick → factory) | **`proxied`** | economy collapsed to per-owner galactic `stockpiles` + system-level `construction_progress` (starport proxy) |
> | Ship-cohort combat / movement / disruption / blockade at galactic tier | `implemented` | the proven flat-tier rehearsal |
>
> **Consumer impact:** the multi-theater nested residency (M-4A territory) and the planet-surface
> economy were never exercised — this is precisely why M-4A and system→planet recursion remained
> theoretical. The recursive structure is now scheduled for real implementation under
> [`runtime_0080_recursive_rehearsal_opening.md`](../production_paths/runtime_0080_recursive_rehearsal_opening.md),
> which **supersedes** the mis-scoped M-4A "parallel theaters" opening.
>
> **Closure status (amended):** RUNTIME-0080-0 remains CLOSED **only as a flat galactic-tier 100-tick
> runtime proof** — NOT as an implementation of the recursive galaxy→system→planet-surface dress-rehearsal
> spec. The recursive spec is OPEN and unbuilt.

Status: IMPLEMENTED / PASS — stable 100-tick GPU-forward rehearsal (flat galactic tier; see Deviation Record above)
Verdict: PASS
Primitive: `STABLE-100-TICK-GPU-FORWARD-REHEARSAL-0`
Rung: `RUNTIME-0080-0-R2`
Scope: stable 100-tick GPU-forward rehearsal over R1a–R1c-f
Stable report checksum: `73d818417f5b98bf`

## Adapter
- adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
- selected_discrete_gpu: true
- backend: Vulkan

## Rehearsal runner
- runs_100_ticks: true
- tick_count: 100
- wall_time_r2_test_harness: ~2.24s (OnceLock single run; gameplay-representative)

## Resident stack consumed
| Rung | Consumed | Evidence |
|------|----------|----------|
| R1a | yes | Tier-A GPU next-tick; tick-100 matches R6C oracle trajectory |
| R1b | yes | Resident event journal per tick; full journal parity |
| R1c-a | yes | Free-list marks from rehearsal journal (11 rows) |
| R1c-b | yes | Allocation from marks + LocalBirthRequest (4 rows) |
| R1c-c | yes | Membership apply (426 delta rows) |
| R1c-d | yes | Compaction + lineage staging (16 + 26 rows) |
| R1c-e | yes | Compacted-view apply (16 remap, 17 compacted, 426 membership remap) |
| R1c-f | yes | GPU-decided ZeroCohort from resident `num_ships` (1 row) |

## ZeroCohort
- gpu_decided: true
- cpu_witness_decides_zero_cohort: false
- zero_cohort_row_count: 1
- structural_decisions_gpu_emitted_zero_cohort: true
- structural_decisions_gpu_emitted (umbrella): false

## R6C CPU oracle comparison
- r6c_checksum_expected: `1bba891c779190a4`
- r6c_checksum_equivalent: `1bba891c779190a4`
- r6c_checksum_matches: true
- tier_a_tick100_matches_oracle: true
- event_journal_parity: true
- explanation: tier-A tick-100 + full journal parity against R6C oracle; equivalent to pinned R6C checksum

### Checksum claim boundary (R2-REVIEW-0)
The reported `1bba891c779190a4` is **checksum-equivalent**, not an independently recomputed
hash of R2's own runtime state. The R2 runner **assigns** the pinned R6C checksum when, and
only when, all of these equivalence conditions hold for the live run:
- 100 ticks completed;
- full event-journal parity against the R6C oracle;
- per-tick journal parity against the R6C oracle;
- Tier-A tick-100 parity against the R6C oracle trajectory;
- R1c-f GPU-decided ZeroCohort consumed.
If any condition fails, the runner instead reports the oracle's own `stable_checksum` and
`r6c_checksum_matches = false`. The equivalence is therefore earned by per-tick + endpoint
parity, but the literal 64-bit value is the pinned R6C constant, not a fresh R2-state hash.
See `runtime_0080_0_r2.rs` (`r6c_checksum_observed` assignment).

## Remaining CPU-decided structural classes (findings, not blockers)
- DamageDelta
- MoveRequest
- LocalBirthRequest
- FusionRequest
- ShipCountDelta
- OwnerCodeFlip
- remaining_class_blocked_run: false

## Anti-ceremony
- m4a_required: false
- multi_atlas_required: false
- new_copy_substrate_added: false
- report_only_aggregation: false (real 100-tick runner executed)

## Required commands (VERIFY-0, foreground PowerShell)

| Target | Result | Wall time |
|--------|--------|-----------|
| `runtime_0080_0_r2` | 18 passed; 0 failed | 2.21s |
| `runtime_0080_0_r1c_f` | (predecessor; unchanged) | — |
| `dress_rehearsal_r6c_integrated_run` | (oracle baseline) | — |
| `cargo build --workspace` | success (exit 0) | 0.21s |
| `cargo fmt --all -- --check` | success (exit 0) | 2.47s |
| `cargo check --workspace` | success (exit 0) | 1.86s |

No scratch logs committed.

## Design-authority ruling (RUNTIME-0080-0-R2-REVIEW-0)

Ruling: **A — ACCEPT / CLOSED AS STABLE 100-TICK GPU-FORWARD REHEARSAL**

Findings:
- R2 real runner: yes (live `for tick in 0..R6C_CANONICAL_TICK_COUNT` loop drives resident Tier-A readback, GPU ZeroCohort threshold/emission, Tier-A dispatch, journal commit, boundary apply; not report aggregation).
- 100 ticks executed: yes.
- ZeroCohort GPU-decided: yes (resident `num_ships` threshold/emission band, event kind 4; CPU witness excludes ZeroCohort).
- R1c substrates consumed: yes (R1c-a→e run from the rehearsal journal; R1c-f boundary preserved).
- checksum wording acceptable: corrected — relabelled **checksum-equivalent** with explicit claim boundary (see above); the value is the pinned R6C constant assigned on per-tick + endpoint parity, not a fresh R2-state hash.
- remaining CPU-decided classes are findings: yes (DamageDelta, MoveRequest, LocalBirthRequest, FusionRequest, ShipCountDelta, OwnerCodeFlip did not block the 100-tick run).
- M-4A remains parked: yes (single-theater, 1.23 MiB steady-state GPU footprint; no scale pressure).

Predecessor battery: full production-track battery was run in PR #550 immediately before R2; PR #551 changed no predecessor semantics (only `pub(crate)` helper visibility plus new R2 files). A verify-only rerun was judged ceremony and **not** ordered.

Next horizon: **no new rung until a consumer is chosen.** Do not continue R1c substrate rungs. The next consumer is selected only after the project decides among (A) class-by-class GPU decision conversion for remaining structural classes, (B) richer emergence scenario, (C) multi-theater / M-4A sparse residency, (D) multi-faction economy, (E) system→planet recursion.

Profiling capture (wall-clock / memory / CPU-vs-GPU): [`runtime_0080_0_r2_profiling_capture.md`](runtime_0080_0_r2_profiling_capture.md).
