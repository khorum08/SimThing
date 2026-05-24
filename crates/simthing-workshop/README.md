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

### Spike versions

**v1 (correctness gate)** proved CPU/GPU parity and determinism. GPU timings were setup-inclusive because each eval recreated adapter/device/pipeline.

**v2 (rich harness)** reuses GPU device/queue/pipelines across warm runs, adds a hardcoded intensity shader baseline, fixes empty-input behavior, validates node metadata, and emits richer correctness/determinism/timing reports.

Warm GPU timings still include buffer upload, dispatch, wait, and readback — not pure shader time. Dispatch-only timing is not reported (`wgpu` timestamp queries are not implemented in this spike).

The hardcoded baseline answers: *how much overhead does the tiny EML evaluator add over a bespoke shader?*

### Interpretation

| Outcome | Meaning |
|---------|---------|
| Correctness + determinism pass | EvalEML remains viable semantically for small combine functions |
| Warm EML much slower than warm hardcoded | Viable semantically; retained intensity pass likely wins on performance until further profiling |
| Warm EML close to hardcoded | Strong candidate for AccumulatorWrite v2 combine path |
| Correctness fails | Keep intensity as a retained specialized pass |

This still does not prove arbitrary GPU EML readiness (no parser, no `exp(x)-log(y)` Sheffer lowering, no production integration).

### Known limitations

- Max 32 expression nodes (spike-only bound).
- Iterative WGSL evaluator over a topologically sorted node array.
- GPU cold timing includes full harness init; warm timing includes per-run buffer alloc/upload/readback.
- Requires a working wgpu adapter (same as `simthing-gpu` tests).

### Run

```powershell
cargo check -p simthing-workshop
cargo test -p simthing-workshop -- --nocapture
```

The 100k test writes `target/workshop/eml_phase5_rich_report_100k.md` (not committed).

### Expected gates

**Correctness (hard fail):**

- EML vs CPU: `max_abs_error <= 1e-4`, `mean_abs_error <= 1e-5`
- Hardcoded vs CPU: same thresholds
- EML vs hardcoded: `max_abs_error <= 1e-4`
- Warm repeated EML and hardcoded outputs identical

**Performance (informative only):**

- Compare `gpu_eml_warm_mean_us` vs `gpu_hardcoded_warm_mean_us` under the same upload/readback conditions

## WeightedMean AccumulatorOp parity spike

This spike tests whether a workshop-local gather/combine/scatter kernel can compute WeightedMean over parent child-ranges with the same semantics as a CPU oracle.

It is not production AccumulatorOp.

The test uses one GPU invocation per parent and loops each parent's child range in canonical order. It intentionally avoids atomics so that WeightedMean parity can be tested independently from transfer/emission contention.

| `parity_classification` | Meaning |
|-------------------------|---------|
| **BIT_EXACT** | Strong AccumulatorOp v2 candidate for WeightedMean |
| **TOLERANCE_EXACT** | Passed weakly; production needs explicit tolerance ADR |
| **FAIL** | Retain specialized WeightedMean reduction path |

The 100k test writes `target/workshop/weighted_mean_parity_report_100k.md` (not committed).
