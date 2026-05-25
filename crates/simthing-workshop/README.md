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

The test uses one GPU invocation per parent and loops each parent's child range in canonical order. The workshop WGSL loop mirrors production `reduction.wgsl` WeightedMean: first child initializes weighted_sum/weight_sum, loop starts at i=1, zero total weight returns 0. It intentionally avoids atomics so that WeightedMean parity can be tested independently from transfer/emission contention.

Production reduction currently expects deterministic child-order semantics. This spike distinguishes bit-exact, strict tolerance, and loose tolerance outcomes. **Loose tolerance is not a production parity claim.**

| `parity_classification` | `accumulatorop_weightedmean_gate` | Meaning |
|-------------------------|-----------------------------------|---------|
| **BIT_EXACT** | STRONG_PASS | Strong candidate for AccumulatorOp v2 WeightedMean |
| **STRICT_TOLERANCE** | WEAK_PASS | Likely acceptable; production ADR must define tolerance policy |
| **LOOSE_TOLERANCE** | WEAK_PASS_REQUIRES_ADR | Exploratory pass only; do not claim production parity without ADR or fix |
| **FAIL** | FAIL | Retain specialized WeightedMean reduction path |

If this spike remains non-bit-exact, the AccumulatorOp ADR must either accept a tolerance policy or retain specialized WeightedMean reduction.

The 100k test writes `target/workshop/weighted_mean_parity_report_100k.md` (not committed).

## WeightedMean current-vs-pivot performance spike

This benchmark compares a workshop-local current-shaped broad reduction path against a targeted AccumulatorOp-style WeightedMean path.

The current-shaped path intentionally models current-system costs:

- overlay materialization before reduction;
- broad `n_dims` column sweep;
- column-rule table;
- full parent output vector;
- work on non-target columns.

The pivot-shaped path (P1: overlay then targeted op) computes only requested WeightedMean aggregate outputs.

This is **not** a production pipeline benchmark. It is a workload-shape benchmark designed to answer when targeted AccumulatorOp WeightedMean might be faster than the current broad reduction paradigm. Warm timings include upload, dispatch, wait, and readback — not pure shader time.

Run: `cargo test -p simthing-workshop weighted_mean_perf -- --nocapture`

The sparse 100k case writes `target/workshop/weighted_mean_perf_report.md` and `.json` (not committed).

## Overlay order-band semantics spike

This spike tests whether Add, Multiply, and Set overlays can be compiled into deterministic order bands for an AccumulatorOp-style pivot.

The current-shaped path applies raw overlays in canonical order and intentionally exposes overlay clutter under stress.

The pivot-shaped path compiles compatible contiguous overlay runs into order-band operations:

- Add runs become Sum;
- Multiply runs become Product;
- Set runs become LastByPriority / LastByOrder.

The compiler is intentionally conservative and must not group across op-kind or order-band boundaries. This test is meant to reveal whether overlay semantics can move from Partial/Risky to Clean-candidate under AccumulatorOp v2.

Run: `cargo test -p simthing-workshop overlay_order -- --nocapture`

The report bundle test writes `tests/overlay_order_semantics_reports.txt` (committed). Per-run markdown/json still go to `target/workshop/overlay_order_report.md` when invoked individually.

## Multi-target replay / delta logging spike

This spike tests whether a GPU-side AccumulatorOp-style operation can write multiple related targets and emit compact replay records sufficient to reconstruct final state.

Representative operation:

```text
source_pool -> queue_accum -> emitted_units
```

The current-shaped baseline resolves the operation on CPU, representing day-boundary settlement and explicit record construction. The pivot-shaped path resolves on GPU and emits compact delta records; replay reconstructs final state from initial state, params, and compact records only.

Run: `cargo test -p simthing-workshop multitarget -- --nocapture`

The report bundle test writes `tests/multitarget_replay_reports.txt` (committed). Bursty 100k also writes `target/workshop/multitarget_replay_report.md` and `.json`.
