# BAND-QUANTIZED-DRAW-0 results

Rung: `BAND-QUANTIZED-DRAW-0` (6.1b)  
HD-RECEIPT: `d7cf7f107500`  
Branch: `grok/band-quantized-draw-0`  
Status: **PROBATION / proof-present / DA-review-pending** (Remand 2)

## Remand 2 closure notes

| blocker | disposition |
|---|---|
| A — ThresholdBuilder admission | `VelocityAlertRegistration` / `AggregateAlertRegistration` carry `CostBandSemantic`; builder uses `push_with_cost_band`; throttle copied from stored `ConjunctiveRecipeRegistration.throttle_hint_max_per_tick`. Real `BoundaryProtocol::rebuild_threshold_registry_from_builder` + `resolve_production_cost_band_draws` referees. |
| B — singular TransformOp type | `TransformOp` is an opaque admitted EML program struct (not an enum). Constructors `set`/`add`/`multiply`/`admit_eml` mint programs. Shape queries are not a second value form. compile_fail proves enum-variant match is impossible. |
| C — anchor hash drift | `anchor_check.sh --resync` updated `stead-spatial-contract-core`; coding orientation re-run after rule-source change. |
| Performance STOP | Preserved. Fair black-box after Remand 2: see measurement. No absolute waiver. |

## Measurement — one-node degenerate `LITERAL_F32` / `TransformOp::set`

**Methodology.** `#[inline(never)]` black-box loops; median of 7 × 500_000 iters; noise band ratio ≤ 1.5 else STOP.

**Recorded (Remand 2, Windows test profile):**

```
BAND-QUANTIZED-DRAW-0 STOP (handoff performance): measurable per-overlay regression
after fair black-box benchmark. EML_med=2.490ns/op prejoin_med=0.197ns/op ratio=12.65
samples=7 iters=500000. No absolute-ns waiver applied. DA ruling required.
```

Bit-identical asserted. STOP semantics retained for DA.

## Proof inventory (focused)

| proof | location |
|---|---|
| ThresholdBuilder admits recipe throttle sink | `band_quantized_draw_production_0` |
| BoundaryProtocol resolve door live-wiring | `boundary_resolve_door_must_be_the_execute_path` |
| Singular TransformOp + compile_fail | `property.rs` doctests |
| One-node Set; EML door; N-dependent overlay | `band_quantized_draw_0` |
| Off-by-one N oracle; conservation | `cost_band` |
| Anchor resync | `stead-spatial-contract-core` |

## Non-goals / frozen

- Zero WGSL diff; zero `ThresholdRegistration` layout change; no allowlist widening.
- Stage 2 recipe / `output_scale` re-expression deferred.
- No 6.2+ work.
- Performance STOP reserved for DA.
