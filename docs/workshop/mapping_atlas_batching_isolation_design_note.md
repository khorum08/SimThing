# Phase M-4 Design Note: Atlas Batching Isolation + VRAM Accounting

**Status:** Design note / Opus-gated prerequisite — **isolation policy ratified (Opus, 2026-05-28); implementation still gated.**  
**This document does not implement atlas batching and does not authorize implementation.**  
**Atlas batching remains provisional and unimplemented.**  
**This document does not authorize production mapping runtime.**

> **Ratification (Opus, 2026-05-28, under human delegation — `../reviews/m4_m4a_first_slice_oversight_opus_review.md`):**
> For homogeneous square batches, **algebraic tile-local mask (G=0) is the preferred
> isolation candidate**; **physical gutter (G≥H) is the fallback**; mixed-size
> **local-bounds metadata remains deferred**. The §11 checklist is now a **binding
> acceptance gate**. Ratification chooses the isolation *design* an M-4 implementation PR
> pursues first — it does **not** authorize implementation. Atlas stays Provisional, and
> `request_atlas_batching` stays rejected at admission, until an M-4 PR passes that gate.

**Related:** [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md) (Mapping ADR — atlas classified **Provisional**), [`../design_v7_7.md`](../design_v7_7.md), [`mapping_current_guidance.md`](mapping_current_guidance.md), Phase M-3 admission [`../tests/phase_m3_region_field_spec_admission_test_results.md`](../tests/phase_m3_region_field_spec_admission_test_results.md).

**Evidence (M-4A sandbox):**

- [`../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md)
- [`archive/mapping/mapping_atlas_algebraic_mask_candidate_notes.md`](archive/mapping/mapping_atlas_algebraic_mask_candidate_notes.md)

**Evidence (archived sandbox):**

- [`../tests/mapping_optimization_toolkit_sandbox_test_results.md`](../tests/mapping_optimization_toolkit_sandbox_test_results.md)
- [`../tests/mapping_optimization_remedial_sandbox_test_results.md`](../tests/mapping_optimization_remedial_sandbox_test_results.md)
- [`archive/mapping/mapping_optimization_remedial_candidate_notes.md`](archive/mapping/mapping_optimization_remedial_candidate_notes.md)

---

## 1. Status

| Item | Posture |
|------|---------|
| Atlas batching in Mapping ADR | **Provisional** — not production-authorized without this contract |
| This design note | Specifies the API contract and binding acceptance gate only |
| Isolation policy sign-off | **Completed by Opus 2026-05-28** (`../reviews/m4_m4a_first_slice_oversight_opus_review.md`) — algebraic tile-local mask G=0 preferred for homogeneous square batches; physical gutter fallback; local-bounds deferred |
| Implementation | **Blocked** — isolation-policy sign-off is done, but atlas implementation stays blocked until a named multi-theater scenario needs batching, an approved VRAM budget exists, **and** an M-4 PR satisfies the §11 binding acceptance gate. Atlas batching remains **Provisional and unimplemented**; this document does not itself implement atlas batching; `request_atlas_batching` remains rejected at admission |
| Production mapping runtime | **Not authorized** by this note |
| `StructuredFieldStencilOp` | Unchanged — live, opt-in, hardened, GPU-resident by default |
| `RegionFieldSpec` (M-3) | Designer/spec structure only; `request_atlas_batching` rejected at admission until M-4 implementation lands |

---

## 2. Evidence summary

### Atlas batching performance (toolkit sandbox)

Independent 10×10 regions batched into one atlas dispatch showed large speedups:

| N regions | Standalone ms | Atlas ms | Speedup |
|-----------|---------------|----------|---------|
| 4 | 18.1 | 2.5 | **7.3×** |
| 16 | 31.6 | 2.1 | **15.4×** |
| 64 | 116.6 | 2.0 | **59.6×** |

Source: toolkit Test 2 (`gutter=1`, 30.6% pitch overhead on 12×12 packed tiles).

### Correctness issue (toolkit sandbox)

Naive/low-gutter atlas batching (`gutter=1` at `H=8`) produced **cross-tile coupling** and protocol mismatch:

- Toolkit Test 1: **PARTIAL** — `max_error=3.46–5.27`; `cross_tile_leak=YES` (gutter=1 insufficient at H=8).
- Combined stack inherited atlas coupling error (toolkit Test 8: `max_error=3.46`).

### Remedial result (safe-gutter stack)

With **G=H=8**, per-tile seed clearing, dirty scheduling, and H-hop halo:

| Metric | Value |
|--------|-------|
| `max_error_vs_standalone_oracle` | **0.003** (Test 6 PASS) |
| Speedup vs standalone | **~18×** |
| Cross-tile t44 leak | **NO** (t44 ≤ 0.016 on all gutters with correct seed protocol) |
| Full-tile L∞ vs naive standalone | **~409** (boundary/source_col semantics — not t44 coupling) |

Source: remedial Tests 1, 6, 8.

### VRAM tax (remedial Test 2)

For **10×10 tiles, H=8, G=8**:

| Field | Value |
|-------|-------|
| `tile_pitch` | **26** (= 10 + 2×8) |
| `vram_multiplier` | **6.76×** |
| `vram_overhead_percent` | **576%** |

Reference table (same H=8, G=8):

| Tile | Pitch | Multiplier | Overhead |
|------|-------|------------|----------|
| 10×10 | 26 | 6.76× | 576% |
| 16×16 | 32 | 4.00× | 300% |
| 32×32 | 48 | 2.25× | 125% |

### Production implication

Atlas batching is **promising** for dispatch amortization but **must**:

1. Report VRAM multiplier before/at pack time.
2. Prove **full-tile parity** against a **protocol-faithful per-tile CPU oracle** (not corridor/t44 alone).
3. Refuse unsafe packing when isolation policy is missing or under-specified.

---

## 3. Atlas isolation contract (short-term)

### Rule

For production atlas batching **without** local-bounds metadata, short-term isolation uses
**exactly one** of the following two admitted policies (local-bounds metadata is the deferred
third policy — see "Tile isolation requirement"):

**A. Algebraic tile-local mask (preferred isolation candidate for homogeneous square batches — M-4A evidence, ratified Opus 2026-05-28)**

```text
gutter = 0 (flush-packed tiles)
shader applies tile-local valid-neighbor mask
foreign-tile reads annihilated before accumulation
fixed-denominator zero-boundary normalization
safe atlas-global bounds before any neighbor load
full-tile protocol-oracle parity required
```

**B. Physical gutter (fallback / conservative)**

```text
gutter >= effective_horizon
```

Physical gutter is required when algebraic masking is not configured or not admitted, or when
the layout is not homogeneous-square.

Where:

```text
effective_horizon = executed horizon for the atlas dispatch
```

(i.e., the configured stencil hop count actually run in that atlas batch, not a larger authored cap unless explicitly executed.)

### Tile isolation requirement

Every packed tile must declare **exactly one** admitted isolation policy:

```text
1. AlgebraicTileLocalMask:
   preferred candidate for homogeneous square batches
   gutter may be 0 (flush-packed tiles)
   tile-local valid-neighbor mask nullifies invalid cross-boundary contributions
   fixed-denominator zero-boundary normalization
   safe atlas-global bounds checked before any neighbor load
   full-tile protocol-oracle parity required

2. PhysicalGutter:
   fallback / conservative path
   gutter >= effective_horizon (zeroed/isolated on all sides)

3. LocalBoundsMetadata:
   deferred future path
   not authorized until a separate ADR/implementation PR
```

### Rationale

- Toolkit probe: `gutter=1` at `H=8` leaked across tiles.
- Remedial probe: t44 passed at `G=0` on a narrow fixture with per-tile seed clearing, but conservative production policy still requires **G ≥ H** to guard worst-case stencil reach and packing layouts.
- **M-4A probe (2026-05-19):** G=0 algebraic tile-local mask achieved full-tile protocol-oracle parity (max error ≤ 0.000031) with **1.0× VRAM** vs **6.76×** for G=H at N=10, H=8. Candidate preserved; runtime reverted. See [`../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md).
- Mapping ADR already classifies atlas as **Provisional** with this short-term policy.

### Future packer obligation

A future generic atlas packer (driver, behind mapping profile) must refuse to pack unless
**exactly one** admitted isolation policy is configured, and must record the chosen policy in
debug/report output:

```text
For AlgebraicTileLocalMask:
  require homogeneous square batch
  require safe atlas-global bounds
  require fixed-denominator zero-boundary normalization
  require full-tile protocol-oracle parity
  require VRAM accounting
  reject mixed-size batches

For PhysicalGutter:
  require gutter >= effective_horizon

For LocalBoundsMetadata:
  reject until separately admitted
```

### M-4 implementation posture (isolation ratified; implementation still gated)

```text
Isolation policy: RATIFIED (Opus 2026-05-28) — preferred candidate is
  homogeneous square atlas batches
  G=0 AlgebraicTileLocalMask
  fixed-denominator zero-boundary normalization
  full-tile protocol-oracle parity
  PhysicalGutter fallback
Mixed-size atlas metadata remains deferred.

M-4 implementation remains BLOCKED. Ratifying the isolation policy is not
authorization to implement the packer. An M-4 implementation PR is admissible
only after (a) a named multi-theater scenario that needs batching, (b) an
approved VRAM budget for it, and (c) the PR satisfying the §11 binding gate.
Until then atlas stays Provisional and request_atlas_batching stays rejected
at admission. The named next mapping step is the first-slice product scenario
fixture (single grid, no atlas) — not atlas implementation.
```

---

## 4. Architectural Implications of Algebraic Tile-Local Masking

M-4A began as sandbox evidence and is now **ratified by Opus 2026-05-28** as the preferred
isolation candidate for homogeneous square atlas batches. This ratifies the isolation **design
only**; it does **not** mark atlas Adopted, does **not** implement atlas, and does **not**
authorize atlas execution. The probe supports a broader SimThing design principle:

```text
Topology, separation, boundaries, and validity can often be represented as generic
algebraic masks/gates over flat GPU fields, rather than as physical padding, CPU
branching, semantic runtime objects, or map-specific WGSL.
```

This section captures Opus-facing implications. It does **not** authorize atlas implementation, production mapping runtime, semantic WGSL, or ADR reclassification.

### 4.1 Structural separation does not always require physical separation

M-4A proves that independent simulation regions do **not** always require physical separation in memory. The physical gutter model isolates packed tiles by allocating unused distance between them. **Algebraic tile-local masking** instead packs tiles flush and nullifies invalid cross-boundary contributions before they enter the accumulator.

This means semantic/topological separation can be represented as **algebra over flat buffers**, provided runtime safety is preserved and the mask is generic.

| Isolation mode | Correctness (M-4A) | VRAM multiplier (10×10, H=8) |
|---|---|---|
| **G=0 algebraic tile-local mask** | Full-tile protocol-oracle parity; WGSL masked error 0.0 in tested cases | **1.0×** |
| **Physical G≥H gutter** | Correct fallback (remedial + M-4 contract) | **6.76×** |

Physical gutter remains the conservative fallback when algebraic masking is not configured or not admitted.

### 4.2 General SimThing design pattern

This is a **design pattern**, not a new primitive:

```text
1. Pack state densely in flat GPU buffers.
2. Author legal relationships, boundaries, and visibility at the RON/spec layer.
3. Compile those relationships into generic masks, gates, or field coefficients.
4. Run semantic-free GPU transforms.
5. Reduce summaries upward through hierarchy.
6. Interpret summaries with EML.
```

Meaning remains in RON/spec/admission. The GPU sees generic fields, masks, coefficients, and bounds — not map/faction/AI semantics.

### 4.3 What this strengthens

**Flat matrix, authored meaning**

- Meaning remains in RON/spec/admission.
- GPU sees generic fields, masks, coefficients, bounds.

**RegionCell-as-SimThing**

- RegionCell grids become more practical because physical adjacency in VRAM does **not** imply semantic adjacency in simulation.

**V7.7 WGSL relaxation**

- New WGSL can be admissible when it is generic, bounded, opt-in, and semantic-free.
- Tile-local masking is not faction/map/AI logic; it is generic boundary math.

**FIELD_POLICY**

- AI/strategy can rely on field summaries and algebraic gating without CPU-side planning.

### 4.4 What this does not authorize

```text
M-4A does not authorize semantic WGSL.
M-4A does not authorize atlas implementation by itself.
M-4A does not authorize production mapping runtime.
M-4A does not authorize mixed-size atlas metadata.
M-4A does not authorize behavioral source policy.
M-4A does not authorize ActiveOnlyExperimentalNoHalo.
M-4A does not mean every subsystem should receive a custom mask shader.
```

**Avoid mask fever.**

“Mask fever” is the tendency to add special-purpose masks for every gameplay concept. Masks are admissible only when they remain **generic, bounded, opt-in, and designer/RON-governed**.

### 4.5 Candidate domains for future algebraic masks

| Domain | Possible algebraic expression | Status |
|---|---|---|
| Atlas tile boundaries | tile-local valid-neighbor mask | M-4A **ratified preferred candidate** (Opus 2026-05-28); implementation still gated |
| Fog/perception | perceived = true × visibility/confidence | Future mapping/perception work |
| Supply reach | field × passability/connectivity mask | Future candidate |
| Ownership/jurisdiction | influence × legal/control mask | Future candidate |
| Source identity | source_mask / seed buffer | Deferred M-5 |
| Active frontier | active mask + H-hop halo | Provisional; active-only banned |

### 4.6 Relationship to dirty skipping and map residency

Algebraic masking solves **wrong interaction between packed maps**. It prevents adjacent packed fields from contaminating each other.

Dirty/residual/cadence skipping solves **unnecessary execution**. It prevents quiet maps from consuming GPU ticks.

These are **complementary**. A skipped RegionField must still expose a valid summary state — fresh, cached, decayed, stale-with-confidence, or zero-if-empty — so hierarchy and parent EML can continue to inform strategic heatmaps without running every dense local map every tick.

M-4A does **not** solve global map residency or field-column budget. It removes gutter overhead for active batched maps. A later **map residency / summary policy** should decide which maps are always resident, event-resident, cached, or cold.

### 4.7 Opus decisions (recorded 2026-05-28)

Opus no longer needs to decide whether physical gutters are too expensive; M-4A demonstrates a better candidate for homogeneous square batches. The six open questions are now decided (full reasoning: `../reviews/m4_m4a_first_slice_oversight_opus_review.md`):

1. **AlgebraicTileLocalMask admissible as generic, semantic-free WGSL? — YES** (boundary algebra over flat buffers; no map/faction/AI/sim semantics).
2. **Preferred M-4 isolation policy for homogeneous square batches? — YES** (preferred candidate).
3. **PhysicalGutter remains fallback? — YES.**
4. **Mixed-size LocalBoundsMetadata remains deferred? — YES.**
5. **Modulo/division vs tile-local dispatch for the first implementation? — EITHER**, gated on the acceptance test rather than the coordinatization method; bounds-check every load; report the chosen method and prefer measuring against tile-local dispatch.
6. **Acceptance gate sufficient? — YES, ratified as binding** (§11), with fixed-denominator normalization, safe atlas-global bounds, caller-managed seed-only clearing, banned column-wide `source_col` zeroing, VRAM accounting, no semantic WGSL, no `simthing-sim` awareness, no default pass-graph wiring, and no t44-only acceptance.

**Constitutional note:** the algebraic-mask representation carries no constitutional risk; the only risk is *mask fever* (a bespoke mask per gameplay concept). A new algebraic mask is admissible only when generic, bounded, opt-in, designer/RON-governed, semantic-free, and protocol-oracle-parity-tested. M-4A clears that bar for tile boundaries only; §4.5 domains each remain separately gated.

Despite ratification of the isolation policy, atlas batching remains **provisional and unimplemented** — implementation is separately gated (see §M-4 implementation posture).

---

## 5. Per-tile seed-clearing protocol

Required caller/packer protocol for `CallerManagedOneShotSeedThenZero` (v1 source policy):

```text
1. Seed source identity cells inside each tile (per tile, not atlas-global only).
2. Run initial hop/setup if required by caller-managed source policy.
3. Clear only source identity cells — not the whole source column.
4. Run configured horizon (effective_horizon hops).
5. Never use column-wide source_col zeroing.
```

### Column-wide zeroing is banned

```text
Column-wide source_col zeroing is banned because propagated state may live in
non-seed cells in the same column.
```

Evidence (remedial Test 5):

- Seed-buffer and source-mask CPU models match caller-managed output.
- Column-wide zero: **reject** — corrupts propagation (`demo err=235` vs `0` for seed-only clear).
- Behavioral source WGSL: **DEFERRED** pending M-5 generic source-identity buffer.

Atlas runs must apply seed clearing **in every packed tile** after the initial hop, not only at atlas origin (remedial candidate notes correction vs toolkit probe).

---

## 6. VRAM accounting

### Required future debug/report fields

A future atlas packer and mapping debug surface must expose at minimum:

| Field | Description |
|-------|-------------|
| `tile_size` | Square edge length N of packed tiles |
| `tile_count` | Number of tiles in atlas |
| `horizon` | Effective horizon for this dispatch |
| `gutter` | Isolation gutter G used |
| `tile_pitch` | Packed tile pitch including gutters |
| `useful_cells` | Total useful (non-gutter) cells |
| `atlas_cells` | Total atlas buffer cells allocated |
| `vram_multiplier` | `atlas_cells / useful_cells` (per-tile basis aggregated) |
| `vram_overhead_percent` | `(vram_multiplier - 1) * 100` |
| `bytes_per_cell` | sizeof(f32) × n_dims (or packed layout bytes) |
| `estimated_atlas_bytes` | `atlas_cells × bytes_per_cell` |
| `scheduled_tile_count` | Tiles scheduled this tick |
| `dirty_ratio` | Fraction of tiles/regions dirty this tick |

### Required formulas

```text
tile_pitch = tile_size + 2 * gutter

useful_cells_per_tile = tile_size * tile_size
atlas_cells_per_tile = tile_pitch * tile_pitch

vram_multiplier = atlas_cells_per_tile / useful_cells_per_tile
vram_overhead_percent = (vram_multiplier - 1) * 100
```

For multi-tile atlases:

```text
useful_cells = tile_count * useful_cells_per_tile
atlas_cells  = tile_count * atlas_cells_per_tile   # non-overlapping tile slots
estimated_atlas_bytes = atlas_cells * bytes_per_cell
```

### Refusal rule

```text
A future atlas packer must refuse to pack without producing/reporting this accounting.
```

M-3 admission already rejects `request_atlas_batching: true` until M-4 implementation exists.

---

## 7. CPU oracle acceptance gate

### Production acceptance criterion

```text
Production atlas acceptance requires bit-exact or tolerance-approved full-tile parity
against an exact per-tile-protocol CPU oracle.
```

**Corridor/t44 agreement alone is not sufficient for production acceptance.**

### Oracle must replay

The CPU oracle must model the **same protocol** the GPU atlas uses:

| Parameter | Must match GPU atlas |
|-----------|---------------------|
| Tile size | ✓ |
| Gutter G | ✓ |
| Boundary mode | ✓ (e.g., Zero vs atlas-global edge) |
| Source seed layout | ✓ per tile |
| Per-tile seed-clear protocol | ✓ step-0 identity clear only |
| Horizon | ✓ effective_horizon |
| Operator | ✓ Normalized / SourceCappedNormalized |
| Source cap | ✓ if applicable |
| Cadence/dirty scheduling | ✓ if relevant to which tiles run |

### Why t44 is insufficient alone

- Remedial Test 1: t44 cross-tile leak ≤ 0.016 even at low gutters with correct seed protocol, while **full-tile L∞ ~409** vs a naive standalone oracle.
- t44 was the sandbox **tactical-signal** metric (corridor cell 44 on 10×10).
- Full-tile parity is only meaningful when the oracle replays atlas gutter + boundary + seed-clear semantics — not a per-tile standalone grid with different edge treatment.

### Tolerance policy (future implementation PR)

- Default gate: **bit-exact** f32 parity on all useful cells per tile after horizon completion.
- If float tolerance is ever approved, it must be explicit, documented, and narrower than t44 corridor checks alone.

---

## 8. Local-bounds metadata future path

### Long-term preferred design

Per-tile **local-bounds metadata** should eventually replace large gutters to avoid the quadratic gutter VRAM tax (remedial Test 3: local bounds modeled; minimal memory overhead vs 6.76× gutter tax on 10×10).

### Possible metadata (semantic-free)

| Field | Purpose |
|-------|---------|
| `tile_id` | Stable tile index in batch |
| `atlas_origin_x`, `atlas_origin_y` | Top-left of useful tile in atlas |
| `tile_size` | Square N |
| `local_min_x`, `local_max_x` | Inclusive useful bounds in atlas coords |
| `local_min_y`, `local_max_y` | Inclusive useful bounds in atlas coords |

WGSL would sample neighbors only within declared local bounds (or equivalent tile-rect clip), providing isolation without `2G` gutter cells.

### Constraints

- Must remain **semantic-free** — no map/faction/AI semantics.
- Must **not** require `simthing-sim` awareness.
- Must have **CPU oracle parity** using the same metadata protocol.

### Deferral

```text
Local-bounds metadata is deferred and requires implementation ADR/PR before use.
```

This design note does **not** authorize local-bounds implementation. Per the ratified
isolation policy (Opus 2026-05-28), atlas implementation candidates are
**AlgebraicTileLocalMask (G=0)** (preferred, homogeneous square batches) with
**PhysicalGutter (G≥H)** as fallback — **not** local-bounds metadata, which remains deferred.

---

## 9. Interaction with designer-facing square grid sizes (M-3)

M-3 admits designer-addressable `grid_size = N` (square only at spec admission).

### v1 atlas batching rule

```text
One atlas batch = same grid_size, same horizon/gutter policy, same operator family,
compatible cadence.
```

- Atlas batching should **initially assume homogeneous square tiles** within each batch.
- Homogeneous batching is a **driver/layout policy**, not a runtime law of `StructuredFieldStencilOp`.
- Mixed-size batching is **deferred** unless a future profile explicitly admits it.

### Admission linkage

- StandardSquare max N=10; ExtendedSquare max N=32 (M-3 caps).
- Atlas VRAM multiplier worsens on small tiles — budget review is mandatory before multi-theater atlas adoption (Mapping ADR first-slice guidance: single grid, no atlas).

---

## 10. Interaction with active masks

| Item | Posture |
|------|---------|
| `ActiveOnlyExperimentalNoHalo` | **Provisional / not production-authorized** |
| Atlas + active mask | **Not authorized** in M-4 implementation |
| If active masks used later | **H-hop or per-hop halo parity** still required |

Evidence:

- Toolkit Test 7: `active_only` **FAIL**; `halo_H8` max_error **0.0026** vs full grid.
- Remedial Test 7: on safe atlas, H-hop halo t44_error **≈0.005**; active-only fails; speedup modest (~1.5–1.6×) on small atlas.

Atlas batching **does not authorize** `ActiveOnlyExperimentalNoHalo`. Any future atlas + sparse execution must contract halo expansion per hop or equivalent frontier semantics before production acceptance.

---

## 11. Future implementation acceptance checklist — **BINDING GATE (Opus, 2026-05-28)**

This checklist is the **binding acceptance gate** for any future M-4 implementation PR. A
PR that does not satisfy **all** rows is not admissible. Required tests (names indicative):

| Test | Requirement |
|------|-------------|
| `atlas_vram_accounting_reports_multiplier` | Reports `vram_multiplier`, `vram_overhead_percent`, `estimated_atlas_bytes` |
| `atlas_refuses_without_isolation_policy` | Refuses pack unless exactly one admitted isolation policy is configured (AlgebraicTileLocalMask with full-tile parity + safe bounds + fixed-denominator normalization, or PhysicalGutter with `gutter ≥ effective_horizon`); LocalBoundsMetadata rejected until separately admitted |
| `atlas_gutter_ge_h_prevents_cross_tile_sampling` | No cross-tile coupling at G ≥ H on standard fixture |
| `atlas_per_tile_seed_clear_protocol_matches_cpu_oracle` | Seed-clear semantics match protocol oracle |
| `atlas_full_tile_parity_against_protocol_oracle` | Full useful tile bit-exact (or approved tolerance) vs oracle |
| `atlas_rejects_t44_only_acceptance` | CI/docs gate — t44 pass alone cannot satisfy acceptance |
| `atlas_batch_homogeneous_square_tiles` | Rejects mixed grid_size in one batch (v1) |
| `atlas_no_semantic_wgsl` | No map/faction/AI semantics in shaders |
| `atlas_no_simthing_sim_awareness` | Packer lives in driver/substrate; sim remains map-free |
| `atlas_no_pass_graph_wiring_by_default` | Opt-in behind mapping profile; default pass graph unchanged |

---

## 12. Stop conditions for future implementation

Stop and escalate (do not land) if:

- Atlas requires map/faction/AI semantics in WGSL.
- Atlas requires `simthing-sim` awareness.
- Atlas requires production mapping runtime / session-open wiring without separate gate.
- Atlas cannot report VRAM multiplier.
- Atlas cannot prove protocol-faithful full-tile parity.
- Atlas tries to use `ActiveOnlyExperimentalNoHalo`.
- Atlas requires behavioral source policy / source identity buffer before **M-5**.
- Atlas implies t44/corridor agreement alone is sufficient for production acceptance.
- Atlas permits column-wide `source_col` zeroing.
- Default `PipelineFlags` or Resource Flow posture changes without explicit approval.

---

## Next steps (after sign-off)

1. **Human + Opus sign-off** on this design note.
2. **Either:**
   - Implement generic atlas packer (Composer 2.5) behind `MappingExecutionProfile::SparseRegionFieldV1` with VRAM reporting and protocol oracle tests; **or**
   - Deliberately choose **first-slice runtime wiring that avoids atlas entirely** (single 10×10 grid, no atlas, no active mask — per Mapping ADR first-slice posture).

Until sign-off: **no atlas packer code**, **no new WGSL**, **no session execution wiring**.

---

## Amendment note

This document **supplements** the Mapping ADR atlas **Provisional** classification with implementation contracts. It does **not** reclassify atlas to **Adopted**. Mapping ADR amendment is required only if future work changes adopt/provisional/deferred classifications.
