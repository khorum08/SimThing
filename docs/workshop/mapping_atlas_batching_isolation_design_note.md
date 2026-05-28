# Phase M-4 Design Note: Atlas Batching Isolation + VRAM Accounting

**Status:** Design note / Opus-gated prerequisite — **parked at decision gate** (pending human + Opus sign-off).  
**This document does not implement atlas batching and does not authorize implementation.**  
**Atlas batching remains provisional.**  
**This document does not authorize production mapping runtime.**

**Related:** [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md) (Mapping ADR — atlas classified **Provisional**), [`../design_v7_7.md`](../design_v7_7.md), [`mapping_current_guidance.md`](mapping_current_guidance.md), Phase M-3 admission [`../tests/phase_m3_region_field_spec_admission_test_results.md`](../tests/phase_m3_region_field_spec_admission_test_results.md).

**Evidence (archived sandbox):**

- [`../tests/mapping_optimization_toolkit_sandbox_test_results.md`](../tests/mapping_optimization_toolkit_sandbox_test_results.md)
- [`../tests/mapping_optimization_remedial_sandbox_test_results.md`](../tests/mapping_optimization_remedial_sandbox_test_results.md)
- [`archive/mapping/mapping_optimization_remedial_candidate_notes.md`](archive/mapping/mapping_optimization_remedial_candidate_notes.md)

---

## 1. Status

| Item | Posture |
|------|---------|
| Atlas batching in Mapping ADR | **Provisional** — not production-authorized without this contract |
| This design note | Specifies future API contracts and acceptance gates only |
| Implementation | **Blocked** until human + Opus sign-off on this note |
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

For production atlas batching **without** local-bounds metadata:

```text
gutter >= effective_horizon
```

Where:

```text
effective_horizon = executed horizon for the atlas dispatch
```

(i.e., the configured stencil hop count actually run in that atlas batch, not a larger authored cap unless explicitly executed.)

### Tile gutter requirement

Every packed tile must have a **zeroed/isolated gutter** of at least `effective_horizon` cells on **all sides**, unless a future local-bounds API provides equivalent isolation without full gutters.

### Rationale

- Toolkit probe: `gutter=1` at `H=8` leaked across tiles.
- Remedial probe: t44 passed at `G=0` on a narrow fixture with per-tile seed clearing, but conservative production policy still requires **G ≥ H** to guard worst-case stencil reach and packing layouts.
- Mapping ADR already classifies atlas as **Provisional** with this short-term policy.

### Future packer obligation

A future generic atlas packer (driver, behind mapping profile) must:

- Refuse to pack when `gutter < effective_horizon` and no local-bounds isolation is configured.
- Record the chosen isolation policy in debug/report output.

---

## 4. Per-tile seed-clearing protocol

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

## 5. VRAM accounting

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

## 6. CPU oracle acceptance gate

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

## 7. Local-bounds metadata future path

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

This design note does **not** authorize local-bounds implementation. Atlas implementation remains on the **gutter ≥ H** path until a separate API design is approved.

---

## 8. Interaction with designer-facing square grid sizes (M-3)

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

## 9. Interaction with active masks

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

## 10. Future implementation acceptance checklist

Required tests for a **future** M-4 implementation PR (names indicative):

| Test | Requirement |
|------|-------------|
| `atlas_vram_accounting_reports_multiplier` | Reports `vram_multiplier`, `vram_overhead_percent`, `estimated_atlas_bytes` |
| `atlas_refuses_without_isolation_policy` | Refuses pack when `gutter < effective_horizon` and no local-bounds config |
| `atlas_gutter_ge_h_prevents_cross_tile_sampling` | No cross-tile coupling at G ≥ H on standard fixture |
| `atlas_per_tile_seed_clear_protocol_matches_cpu_oracle` | Seed-clear semantics match protocol oracle |
| `atlas_full_tile_parity_against_protocol_oracle` | Full useful tile bit-exact (or approved tolerance) vs oracle |
| `atlas_rejects_t44_only_acceptance` | CI/docs gate — t44 pass alone cannot satisfy acceptance |
| `atlas_batch_homogeneous_square_tiles` | Rejects mixed grid_size in one batch (v1) |
| `atlas_no_semantic_wgsl` | No map/faction/AI semantics in shaders |
| `atlas_no_simthing_sim_awareness` | Packer lives in driver/substrate; sim remains map-free |
| `atlas_no_pass_graph_wiring_by_default` | Opt-in behind mapping profile; default pass graph unchanged |

---

## 11. Stop conditions for future implementation

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
