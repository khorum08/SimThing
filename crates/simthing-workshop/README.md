# simthing-workshop

This crate contains isolated architectural spikes for SimThing.

**Note:** `simthing-workshop` holds experiments and viability gates. The deferred GUI crate is `simthing-studio` (not this crate).

## EML Phase 5 intensity spike

This spike tests whether a tiny GPU-side expression-tree evaluator can reproduce the intensity-update formula:

```text
new_intensity =
  clamp(
    if abs(velocity) > threshold:
      intensity + build * abs(velocity) * dt
    else:
      intensity - decay * intensity * dt,
    0.0,
    1.0
  )
```

This is not a general EML implementation. It uses a hand-authored expression node array and a fixed small opcode set.

A passing result supports the claim that EvalEML is viable for small combine functions in the future AccumulatorWrite v2 / AccumulatorOp model.

A failing result means intensity update should remain a retained specialized pass, and the full-pivot docs should not claim complete elegance through GPU EvalEML.

### Known limitations

- Max 32 expression nodes (spike-only bound).
- Iterative WGSL evaluator over a topologically sorted node array — no recursion, parser, or RON loader.
- GPU timing includes submit, wait, and readback — not pure shader time.
- Requires a working wgpu adapter (same as `simthing-gpu` tests).

### Run

```powershell
cargo check -p simthing-workshop
cargo test -p simthing-workshop -- --nocapture
```
