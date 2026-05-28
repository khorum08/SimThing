# Mapping Optimization Toolkit — Candidate Notes

Sandbox probe for V7.6 `StructuredFieldStencilOp` macro-optimizations informing the Mapping ADR.

## Optimizations probed

1. **Atlas batching** — pack N×10×10 logical regions into one atlas with 1-cell gutter per tile.
2. **Cadence tiers** — tick-modulo scheduling fixture (EveryTick / 4 / 10 / 60 / EventTriggered).
3. **Dirty macro-region skipping** — driver-level skip when no dirty/residual/topology/cadence trigger.
4. **Active frontier + halo** — mask dilation (0 / 1 / H hops) on `ActiveOnlyExperimentalNoHalo`.

## Key findings (truthful)

### Atlas batching
- **Cost:** Strong dispatch amortization — speedup 7.3× (N=4), 15.4× (N=16), 59.6× (N=64) vs N standalone ops.
- **Correctness:** **PARTIAL** — gutter=1 insufficient for H=8 independence; cross-tile coupling detected (`max_region_error` 3.46–5.27 on t44). ADR must specify gutter width or per-tile isolation policy.

### Cadence tiers
- **Determinism:** PASS — exact update counts over 120 ticks; 1580 dispatches avoided vs every-tick for 20 mixed fields.
- **Quality:** Source-capped H=8 run saturates at cap=500 within one update cycle; slower cadences appear equivalent on this fixture. ADR should classify field types by cadence using uncapped or decaying fields for quality probes.

### Dirty macro-region skip
- **Correctness:** PASS — 62.5% skip ratio on 16-region scenario; zero false skips.
- **Layer:** driver/scheduler — not shader behavior.

### Active frontier + halo
- **Correctness:** H-hop halo (H=8) matches full-grid oracle (`max_error` 0.0026); active-only and 1-cell halo fail badly.
- **Cost:** Modest speedup (~1.9×) when H-hop halo covers 64% of 10×10 grid — sparse benefit requires larger grids / sparser activity.
- **ADR:** Active masks must not be production-authorized without halo semantics; recommend H-hop or per-hop frontier expansion contract.

### Combined stack
- **Cost:** 31× vs standalone per-region loop at 25% dirty ratio.
- **Correctness:** PARTIAL — inherits atlas coupling error; quality_label=stale until gutter/isolation fixed.

### Cost projection (rough)
- Accumulator per-edge 30k dirty-adjusted: **3236.6 ms** (prior probe baseline).
- Dense stencil 30k: **~125 ms**.
- Atlas 30k: **~60 ms**.
- Dirty atlas 10%: **~6 ms**.
- Combined stack 30k: **~8 ms** (rough; assumes multiplicative factors).

## Production constraints unchanged
- No mapping runtime.
- No production pass graph wiring.
- `StructuredFieldStencilOp` remains opt-in generic toolkit code.

## Recommended Mapping ADR stack (provisional)
```text
dirty macro-region scheduler → atlas batching (with ADR gutter policy) → H-hop halo mask → cadence tiers (RON/Designer)
```
