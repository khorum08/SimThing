# M-4A Sandbox — Algebraic Tile-Local Atlas Masking Test Results

**Date:** 2026-05-19  
**Branch probe:** M-4A sandbox (reverted to parked state after preservation)  
**Verdict:** **YES** — Algebraic tile-local masking is ready to become the preferred M-4 implementation path for homogeneous square atlas batches, pending human + Opus sign-off. Physical gutter remains fallback.

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / FAIL | Key result |
|---|---|---|---|
| 0 | Guardrail sanity | PASS | M-4 parked; atlas flag rejected; no runtime wiring; simthing-sim map-free |
| 1 | CPU oracle G=0 mask | PASS | Masked max error ≤ 0.000031 all N/H/operator; unmasked error 17–497 at H≥4 |
| 2 | WGSL parity G=0 mask | PASS | GPU masked error 0.0; unmasked diverges (458–500) |
| 3 | G=0 mask vs G=H gutter | PARTIAL | Correctness + VRAM 1.0× vs 6.76×; wall_ms mixed (alg faster at 64 tiles) |
| 4 | Coordinate cost | PARTIAL | Modulo/division plausible (~1.6–5.8 ms); recommend tile-local dispatch in production |
| 5 | Normalization semantics | PASS | Fixed denominator preferred; renorm amplifies edges (max err 321 vs oracle) |
| 6 | Source protocol | PASS | Uncleared growth 1.95×; column-wide zero err 256; seed-only clear required |
| 7 | Guardrail relocation | PASS | Expressive policy → RON/spec; runtime safety stays in runtime |
| 8 | ADR/M-4 recommendation | PASS | ADOPT provisionally for homogeneous square batches |

---

## Test 1 — CPU oracle (selected rows)

| N | H | operator | max_err_masked | max_err_unmasked | leak_unmasked |
|---|---|---|---|---|---|
| 5 | 8 | Normalized | 0.000015 | 496.79 | yes |
| 10 | 8 | SourceCapped | 0.000031 | 458.86 | no* |
| 32 | 8 | SourceCapped | 0.000031 | 458.86 | no* |

\*Probe-point leak false at large N/H but full-tile max error proves unmasked divergence.

All masked cases: max_err_masked ≤ 0.000031 across full N×H×operator matrix.

---

## Test 2 — WGSL parity

| N | tile_count | H | max_err_mask | max_err_unmask | wall_ms | dispatches |
|---|---|---|---|---|---|---|
| 10 | 4 | 8 | 0.0 | 458.86 | 4.62 | 9 |
| 10 | 16 | 8 | 0.0 | 500.0 | 2.15 | 9 |
| 5 | 4 | 8 | 0.0 | 473.17 | 1.62 | 9 |
| 20 | 4 | 8 | 0.0 | 458.86 | 1.66 | 9 |

---

## Test 3 — Isolation mode comparison (N=10, H=8, SourceCapped)

| mode | tile_count | VRAM mult | wall_ms | max_err_vs_oracle | dispatches |
|---|---|---|---|---|---|
| algebraic G=0 | 16 | 1.00 | 3.77 | 0.0 | 9 |
| physical G=H | 16 | 6.76 | 2.74 | (gutter baseline) | 9 |
| no mask G=0 | 16 | 1.00 | — | 500.0 | — |
| algebraic G=0 | 64 | 1.00 | 1.69 | 0.0 | 9 |
| physical G=H | 64 | 6.76 | 4.11 | (gutter baseline) | 9 |
| no mask G=0 | 64 | 1.00 | — | 500.0 | — |

Estimated bytes (N=10, 16 tiles, 4 dims): flush 40×40×4×4 = 25,600 B vs gutter 104×104×4×4 = 173,056 B (~6.76×).

---

## Test 4 — Coordinate derivation cost

| method | N | tile_count | wall_ms | notes |
|---|---|---|---|---|
| modulo_division (WGSL) | 10 | 16 | 3.86 | Per-cell tile_x/tile_y; plausible; production may use tile-local dispatch |

---

## Test 5 — Normalization semantics (N=10, H=8, 4 tiles)

| variant | max_value | L1 | max_err_vs_fixed_oracle | recommendation |
|---|---|---|---|---|
| fixed_denominator | 256.34 | 11728.59 | 0.000031 | **preferred** |
| valid_neighbor_renorm | 423.17 | 21695.00 | 321.73 | edge amplification — defer |

---

## Test 6 — Source-clearing protocol

| metric | value |
|---|---|
| growth_ratio_uncleared | 1.95 |
| error_column_wide_zero | 256.34 |
| seed_only_clear | matches oracle |
| column_wide_zero | banned |

---

## Test 7 — Guardrail relocation

| Guardrail | Current layer | Proposed layer | Safe to move? | Evidence |
|---|---|---|---|---|
| square grid size | RON/spec | RON/spec | yes | M-3 |
| homogeneous atlas tile size | future packer | RON/spec + packer | yes | M-4A |
| algebraic tile-local masking | deferred | generic WGSL opt-in | yes | M-4A |
| operator admission | RON/spec | RON/spec | yes | M-3 |
| source policy | RON/spec + runtime | keep | no | Test 6 |
| horizon caps | runtime + spec | keep both | no | safety |
| buffer bounds | runtime/GPU | keep | no | safety |
| formula class admission | RON/spec | RON/spec | yes | V7.6/M-3 |

---

## Test 8 — ADR/M-4 recommendation

| Isolation strategy | Correctness | VRAM | Speed | Complexity | Recommendation |
|---|---|---|---|---|---|
| Physical gutter G>=H | strong | poor (6.76× @10) | good | low | fallback |
| Algebraic tile-local mask G=0 | strong | 1.0× | good (scale) | medium | **preferred candidate** |
| Local-bounds metadata | deferred | best long-term | TBD | high | future |
| No mask G=0 | fails | 1.0× | fast | low | reject |

**Classification:** ADOPT provisionally for homogeneous square batches pending human + Opus sign-off.

**Proposed M-4 amendment:** Short-term isolation may use physical G>=H **or** algebraic tile-local mask with protocol-oracle parity. Physical gutter becomes fallback for homogeneous square batches.

---

## Posture preserved

- V7.7 Mapping ADR unchanged at architecture level
- M-4 remains parked; no atlas implementation landed
- No mapping runtime; no pass graph wiring
- StructuredFieldStencilOp unchanged in production
- simthing-sim map-free
- Candidate code preserved under `docs/workshop/`

Full log: [`mapping_atlas_algebraic_mask_sandbox_full.log`](mapping_atlas_algebraic_mask_sandbox_full.log)
