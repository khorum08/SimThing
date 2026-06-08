# FIELD_POLICY strategic horizon / velocity / PF-skip feasibility probe — test results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `8b05ca4` — FIELD_POLICY sandbox revert HEAD doc update  
**Sandbox merge SHA:** `0878c39` (PR #202)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 11/11 sandbox tests PASS (run with `--test-threads=1`).

**Note:** Horizon tests use **directed south/east propagation** after source zeroing to avoid bidirectional NSEW feedback blow-up observed in probe 1. Full NSEW remains substrate-available but unstable for long horizons without per-band decay.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test field_policy_strategic_horizon_sandbox -- --nocapture --test-threads=1` | **PASS** — 11/11 |

**Full log:** [`archive/field_policy/field_policy_strategic_horizon_sandbox_full.log`](archive/field_policy/field_policy_strategic_horizon_sandbox_full.log)

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key numeric output |
|---|---|---|---|
| 0 | Capability sanity | PASS | EvalEML=YES copy_col=YES |
| 1 | Strategic horizon sweep | PASS | first_nonzero_H=8 first_directional_H=8 |
| 2 | Horizon cost curve | PASS | H_directional=8 projected_30k=1555.7ms |
| 3 | Multi-cadence horizon | PARTIAL | model=C per_tick=1.76ms direction=partial at H=16 |
| 4 | Explicit velocity column | PASS | velocity_tick2=1.0 tick3=2.0 support=YES |
| 5 | Velocity overhead | PASS | overhead_percent=14.3 |
| 6 | PF convergence detection | PARTIAL | first_skip_candidate_tick=NONE (max=0.051 at tick32) |
| 7 | PF skip correctness simulation | PARTIAL | skip_would_be_safe=NO max_error=0.0106 |
| 8 | Horizon + decay interaction | PARTIAL | decay variant still reaches [4][4]=108565 |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
Strategic horizon IS substrate-real when propagation horizon ≥ 8 bands (directed SE model): [4][4] becomes nonzero with correct negative gradient by H=8. Multi-cadence amortization works (Model C: 1.76 ms/tick vs Model A: 19.63 ms/tick for 16 effective hops). Explicit-column velocity works on GPU via EvalEML SUB + ResetTarget copy (no previous-buffer opcode). PF convergence is measurable (ratio≈0.8) but skip-candidate threshold not reached by tick 32 and skip simulation barely fails epsilon (0.0106 > 0.01). Bidirectional NSEW long horizons amplify without decay — design constraint, not missing primitive. Not YES because PF skip safety and multi-cadence gradient direction remain partial; not NO because horizon, velocity, and decay metrics are expressible today.
```

### Strategic horizon verdict

```text
first_nonzero_horizon_at_4_4 = 8
first_directional_horizon_at_4_4 = 8 (grad_x=-400.1 grad_y=-400.1 mag=565.9 at H=8)
best_cadence_model = C_2bands_8ticks (mean 1.76 ms/tick vs 19.63 ms one-shot)
cost_at_best_horizon = 5.19 ms wall at H=8 one-tick
AI visibility verdict = sufficient at H≥8 with directed propagation; marginal at H=16+ due to field amplification
```

### Cost impact of widening horizon

```text
H=4 wall_ms=4.78 (no [4][4] signal — insufficient hops)
H=8 wall_ms=5.19 ms_per_hop=0.648
H=12 wall_ms=5.66
H=16 wall_ms=5.98
H=24 wall_ms=9.51
Projected 30k at best directional H=8 = 1555.7 ms/tick
Projected 100k at best directional H=8 = 5185.5 ms/tick
Budget label = MARGINAL — H=8 directional within single-tick budget at 100 cells; 30k projection needs cadence control/batching
```

### Velocity verdict

```text
explicit previous column works = YES
copy current→previous via ResetTarget SlotValue = YES
velocity GPU EvalEML SUB = YES (tick2=1.0 tick3=2.0 decay=0.0)
velocity overhead = 14.3% at 196 cells (WITHIN BUDGET <20%)
production recommendation = explicit col 6/7 column schedule viable; copy band before threat update band
```

### PF skipping verdict

```text
convergence detectable = YES (monotone decay ratio≈0.8)
skip candidate tick = NONE by tick 32 (max_abs_value=0.051 > epsilon 0.01)
skip correctness = PARTIAL (max_error=0.010634 after 8 ticks post-candidate; barely above epsilon)
production recommendation = convergence metrics usable for observability; skip threshold needs tuning (epsilon or tick count) before boundary skip authorization
```

### Remaining gap outside this sandbox

AI decision quality still requires gameplay evaluation with realistic scenarios. Directed vs NSEW propagation policy, cadence selection, and skip epsilon tuning are design work. This sandbox establishes technical and cost feasibility only.

---

## Raw test output (key numerics)

```
Test0 EvalEML_GPU=true AddToTarget=true ScaleTarget_decay=true copy_current_to_previous=true
Horizon H=8: threat[4][4]=352.32 grad=(-400.1,-400.1) direction=correct
Horizon H=4: threat[4][4]=0 (insufficient hops)
first_nonzero_horizon_at_4_4=8 first_directional_horizon_at_4_4=8
Model A 16bands/1tick: threat[4][4]=130612 grad partial total_ms=19.63
Model B 4bands/4ticks: threat[4][4]=74484 per_tick=3.59
Model C 2bands/8ticks: threat[4][4]=85544 per_tick=1.76
Test4 velocity_tick2=1.0 velocity_tick3=2.0 velocity_decay=0.0
Test5 overhead_percent=14.3 delta_ms=0.201
Test6 tick32 max=0.050706 first_skip_candidate=NONE ratio_mean=0.8
Test7 skip_would_be_safe=NO max_error=0.010634
Test8 H=16 decay threat[4][4]=108565; H=16 no_decay threat[4][4]=191195
```

---

## Preserved artifacts

- Source: [`docs/workshop/archive/field_policy/field_policy_strategic_horizon_sandbox_code_preserve.rs`](../workshop/archive/field_policy/field_policy_strategic_horizon_sandbox_code_preserve.rs)
- This report + full log

Production test file removed on revert to parked state.
