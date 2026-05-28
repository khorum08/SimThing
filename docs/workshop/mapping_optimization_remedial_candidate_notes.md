# Mapping Optimization Remedial Probe — Candidate Notes

Sandbox remedial probe resolving PARTIAL results from the Mapping Optimization Toolkit probe.

## Scope

1. Atlas isolation sweep with parameterized gutter (G ∈ {0,1,2,4,8,9}).
2. Gutter VRAM tax quantification.
3. Isolation policy comparison (gutter vs local bounds — modeled).
4. Caller-managed source policy baseline.
5. Behavioral source policy options (CPU models only; WGSL deferred).
6. Combined stack with safe gutter (G=H=8).
7. Active halo re-test on safe atlas.
8. Updated cost/VRAM projections.
9. ADR adoption update.

## Key corrections vs prior probe

- **Per-tile seed clearing:** Atlas runs must clear seed identity in every packed tile after step 0, not only at atlas origin.
- **Leak metric:** Cross-tile coupling measured at corridor t44 (primary tactical metric), not full-tile max (dominated by source_col / boundary semantics).
- **Source policy:** Column-wide `source_col` zero after step 0 is unsafe when propagated state occupies non-seed cells in that column. Generic `source_mask` or separate seed buffer required before behavioral shader policy.

## Findings

### Atlas isolation (Test 1)
- **t44 cross-tile leak:** NO for all gutters tested (0–9) with per-tile seed clearing; max_t44_error 0.006–0.016.
- **Full-tile max error:** Remains high (~409) vs standalone — atlas global zero boundary vs per-tile standalone oracle; ADR should not use full-tile L∞ as sole gate when source_col semantics differ.
- **Conservative ADR policy:** G ≥ H (8) recommended despite t44 passing at G=0 on this fixture — production must guard worst-case stencil reach and packing layouts.
- **VRAM tax at G=H on 10×10:** pitch=26, overhead 576%, multiplier 6.76×.

### Source policy (Tests 4–5)
- Caller-managed seed-only clear: growth_ratio≈2.13 (uncleared pumps to cap=500).
- Seed-buffer and source-mask CPU models match caller-managed output.
- Column-wide zero corrupts propagation when source_col holds non-seed state (demo err=235 vs 0 for seed-only).
- **Behavioral WGSL:** DEFERRED pending generic source_mask/seed buffer API.

### Combined stack (Test 6)
- Safe gutter G=8, 25% dirty, H-hop halo: max_error_vs_oracle=0.003, speedup≈18× vs standalone, **PASS**.

### Active halo (Test 7)
- On safe atlas, H-hop halo t44_error≈0.005 within masked tile-0 region; active-only fails.
- Speedup modest (~1.5–1.6×) on small 4-tile atlas.

### Projections (Test 8)
- safe_gutter_atlas_30k ≈ 37 ms (up from ~60 ms with G=1 atlas — larger atlas footprint).
- safe_gutter_combined_stack_30k ≈ 5.1 ms (still ≪ accumulator 3236.6 ms).
- VRAM multiplier 6.76× for 10×10 H=8 G=8.

## Verdict

**PARTIAL+** — corrected stack fixes combined-stack correctness (PASS) but VRAM tax and behavioral source policy deferral keep full YES pending Mapping ADR API design.

## Production constraints unchanged

- No mapping runtime. No production pass graph wiring.
- V7.6 `StructuredFieldStencilOp` remains live, opt-in, hardened, inert by default.
