# BAND-QUANTIZED-DRAW-0 results

Rung: `BAND-QUANTIZED-DRAW-0` (6.1b)  
HD-RECEIPT: `d7cf7f107500`  
Branch: `grok/band-quantized-draw-0`  
Status: **PROBATION / proof-present / DA-review-pending** (Remand 1)

## Remand 1 closure notes

| blocker | disposition |
|---|---|
| 1 — production CostBand wiring | CostBand rides `ThresholdRegistry` `event_kind` semantics; boundary calls `resolve_cost_band_draws_for_deltas`; caller cannot supply `is_sink`/throttle ad hoc. Live-wiring counter REDs if door is bypassed. |
| 2 — one-node Set + admission cap | `Set` lowers to **one-node** `LITERAL_F32(v)`; `AdmittedEmlProgram` private field seals cap bypass; ordinary overlay N-dependent EML via `admit_eml` + `apply_to_data_with_n`; static-bypass planted RED via EML eval counter. Off-by-one N referee uses floor/throttle oracle (survives recomputed R). |
| 3 — performance STOP | Fair black-box median measurement after removing per-apply heap lower. **STOP retained for DA ruling** — see measurement below. No absolute-ns / large-ratio waiver. |
| 4 — sanctioned surface digest | Regenerated from retained `contention_mechanisms.txt` row (8 rows). |

## Measurement — one-node degenerate `LITERAL_F32` / `TransformOp::set`

**Methodology (Remand 1 fair apples-to-apples).**

1. Pre-join baseline: `#[inline(never)]` direct Set (`return v`).
2. EML path: `#[inline(never)]` `TransformOp::set(v).apply(current)` — singular interpreter; stack-local one-node buffer (no per-apply heap lower).
3. Both loops: `std::hint::black_box` on inputs/accumulator; **median of 7 samples × 500_000 iters**.
4. Criterion: ratio ≤ 1.5 ⇒ PASS; else **STOP** (handoff forbids absolute waiver / redefining acceptable).

**Recorded on coder host (Windows, `cargo test` / test profile):**

```
BAND-QUANTIZED-DRAW-0 STOP (handoff performance): measurable per-overlay regression
after fair black-box benchmark. EML_med=3.357ns/op prejoin_med=0.197ns/op ratio=17.00
samples=7 iters=500000. No absolute-ns waiver applied. DA ruling required —
do not code around this STOP.
```

Bit-identical results remain non-negotiable and are asserted. Per-apply heap lower was
removed (prior landing measured ~26 ns/op); residual interpreter tax vs a pure bit-write
still exceeds the 1.5× noise band. Per handoff stop_conditions: **STOP and report** —
not a coded redefinition of “acceptable.”

## Proof inventory (focused)

| proof | location |
|---|---|
| Unmarked observation bit-identical | `band_quantized_draw_0` + production registry tests |
| `V = N*C + R` randomized + planted lost remainder | `cost_band` unit |
| Off-by-one N fails oracle even with recomputed R | `cost_band::planted_off_by_one_n_reds` + integration |
| Ambiguous marker hard-error; throttle from admitted table | production + cost_band |
| Boolean depth-1 same path | `cost_band` + production |
| Static Set = **one-node** `LITERAL_F32` | `static_set_is_genuinely_one_node_literal_f32` |
| Production apply enters EML door; static bypass REDs | `production_apply_*` / `planted_static_bypass_*` |
| Ordinary N-dependent overlay same path | `ordinary_overlay_n_dependent_eml_same_path` |
| Per-program cap at admission; forge compile_fail | `per_program_cap_*` + `AdmittedEmlProgram` doctest |
| Production CostBand resolve door / live-wiring | `band_quantized_draw_production_0` |
| Runtime depth mutation w/o re-hydration | core + production |
| `CostBand` spelling in `docs/stead_spatial_contract.md` §5.1 | doc obligation |
| Contention row for overlay EML | `scripts/ci/allow/contention_mechanisms.txt` |
| compile_fail no `Static` / `Computed` / Eml forge | `property.rs` doctests |

## Non-goals / frozen

- Zero WGSL diff; zero `ThresholdRegistration` layout change; no allowlist widening.
- Stage 2 recipe / `output_scale` re-expression deferred.
- No 6.2+ work.
