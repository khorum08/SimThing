# M-4A — Algebraic Tile-Local Atlas Masking (Candidate Notes)

**Status:** Sandbox probe complete; candidate preserved; runtime reverted to parked state.  
**Evidence:** [`../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md`](../tests/mapping_atlas_algebraic_mask_sandbox_test_results.md)

## Hypothesis

Homogeneous square RegionField tiles can be flush-packed (`G=0`) if the atlas stencil kernel applies a **tile-local valid-neighbor mask**:

```text
contribution = neighbor_value * valid_tile_local_neighbor
```

Foreign-tile reads are annihilated algebraically before accumulation. Global buffer bounds are still checked before any load — no unsafe out-of-buffer reads.

## Preserved artifacts

| File | Role |
|------|------|
| [`mapping_atlas_algebraic_mask_sandbox_code_preserve.rs`](mapping_atlas_algebraic_mask_sandbox_code_preserve.rs) | CPU oracle + GPU runner |
| [`structured_field_stencil_atlas_mask_candidate.wgsl`](structured_field_stencil_atlas_mask_candidate.wgsl) | Prototype WGSL kernel |

## Key findings

1. **Correctness:** G=0 tile-local mask matches per-tile standalone protocol oracle across N∈{5,10,20,32}, H∈{1,4,8}, Normalized and SourceCappedNormalized (max full-tile error ≤ 0.000031).
2. **Failure control:** G=0 without mask diverges massively (max error 458–500 at H=8).
3. **VRAM:** Multiplier **1.0×** vs **6.76×** for physical G=H at N=10, H=8.
4. **Speed:** Sandbox wall_ms competitive; at 64 tiles algebraic (1.69 ms) beat physical gutter (4.11 ms). Coordinate derivation uses per-cell division — production should prefer tile-local dispatch layout where cheap.
5. **Normalization:** Fixed-denominator zero-boundary preferred over valid-neighbor renormalization (edge amplification in renorm variant).
6. **Source protocol:** Per-tile seed identity clear still required; column-wide `source_col` zero remains banned.

## M-4 amendment (ratified Opus 2026-05-28)

> Originally proposed pending sign-off; **ratified by Opus 2026-05-28 under human
> delegation** — see [`../reviews/m4_m4a_first_slice_oversight_opus_review.md`](../reviews/m4_m4a_first_slice_oversight_opus_review.md).
> Ratifies the isolation *policy* only; atlas implementation remains gated on a §11-gate-passing PR.

Short-term atlas isolation for **homogeneous square batches** may use either:

- **A.** Physical gutter `G >= H` (fallback / conservative path)
- **B.** Algebraic tile-local mask with `G=0` flush pack and protocol-oracle parity

Physical gutter is no longer the only short-term path for homogeneous square batches, but remains required when algebraic masking is not configured or not admitted.

## Guardrail relocation

Expressive policy (tile size homogeneity, operator admission, formula class, atlas isolation mode admission) stays at RON/spec + future packer policy. Runtime safety (buffer bounds, horizon caps, finite coefficients, default-off execution) stays in runtime/GPU.

## Not authorized

- Production atlas packer
- Production mapping runtime
- Pass graph wiring
- simthing-sim map awareness
- WGSL map/faction/AI semantics
