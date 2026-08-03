# BAND-QUANTIZED-DRAW-0 results

Rung: `BAND-QUANTIZED-DRAW-0` (6.1b)  
HD-RECEIPT: `d7cf7f107500`  
Branch: `grok/band-quantized-draw-0`

## Measurement — one-node degenerate `LITERAL_F32` / `TransformOp::Set`

**Methodology.** Host wall-clock batch of 200 000 applies:

1. **EML path:** `TransformOp::Set(v).apply(current)` (always via `to_eml_nodes` + interpreter).
2. **Direct baseline:** write `v.to_bits()` (the pre-join arithmetic special case for Set).

Require **bit-identical** accumulated fingerprints and no pathological per-overlay regression
(EML ns/op within a generous CI-noise bound vs direct, or absolute EML &lt; 200 ns/op).

**Recorded on coder host (Windows, dev profile, 200 000 iters):**

```
BAND-QUANTIZED-DRAW-0 measurement: one-node Set EML=26.224ns/op direct=0.019ns/op ratio=1344.85 iters=200000
```

Interpretation: EML one-node Set is bit-identical and stays in the tens-of-nanoseconds class
(~26 ns/op absolute). The “direct” baseline is a pure bit-write with no call overhead, so the
ratio is not a hot-loop regression signal — absolute EML cost remains acceptable for overlay
application. No Static/Computed execution branch exists; all three constructors share the EML path.

## Proof inventory (focused)

| proof | location |
|---|---|
| Unmarked observation bit-identical | `band_quantized_draw_0.rs` / `cost_band` unit |
| `V = N*C + R` randomized + planted lost remainder / off-by-one | `cost_band` unit tests |
| Ambiguous marker hard-error; throttle cap | `cost_band` + integration |
| Boolean depth-1 same path; planted separate branch RED | `cost_band::planted_separate_boolean_branch_reds` |
| Static Set = one-node `LITERAL_F32`; same EML path for Add/Mul | `static_set_is_one_node_literal_f32_bit_identical` |
| Per-program cap admit + exceed RED | `per_program_eml_cap_*` |
| Runtime depth (N) mutates output without re-hydration | `runtime_depth_mutation_changes_output_without_rehydration` |
| `CostBand` spelling in `docs/stead_spatial_contract.md` §5.1 | doc obligation |
| Contention row for overlay EML | `scripts/ci/allow/contention_mechanisms.txt` |
| compile_fail no `TransformOp::Static` / `::Computed` | `property.rs` doctests |

## Non-goals / frozen

- Zero WGSL diff; zero `ThresholdRegistration` layout change; no allowlist widening.
- Stage 2 recipe / `output_scale` re-expression deferred.
- No 6.2+ work.
