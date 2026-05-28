# SEAD field-intelligence feasibility probe — test results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `50dfcc0` — revert RegionCell verification re-run (PR #199)  
**Final commit SHA:** _(pending merge)_  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — all 13 sandbox tests executed on GPU path.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test sead_field_intelligence_sandbox -- --nocapture` | **PASS** — 13/13 |
| `git rev-parse HEAD` | PASS — `50dfcc0` (pre-sandbox) |

**Full log:** [`sead_field_intelligence_sandbox_full.log`](sead_field_intelligence_sandbox_full.log)

---

## Stage 0 — Substrate capability probe

| Capability | Result |
|------------|--------|
| EvalEML registration | **YES** |
| EvalEML GPU execution | **YES** |
| AddToTarget one-hop propagation | **YES** |
| Propagation staging | **later-band-cascade** (same-band chain does not advance; sequential OrderBands within one tick do) |
| Decay without erasure | **YES** — `ConsumeMode::ScaleTarget` × 0.8 |
| Previous-values read | **NO** — no EML opcode for previous buffer; `PARAM` is tick `dt` |
| Whitelist `propagation_formula` | **rejected** |
| Whitelist `urgency_formula` | **rejected** |
| Whitelist `conversion_rate` alias | **accepted** (legacy `register()`) |
| Direct `register_formula` bypasses legacy whitelist | **YES** (sandbox uses C-8 path, not legacy class names) |

---

## Raw test output (key numerics)

```
Test0 eval_eml_register=true eval_eml_gpu=true add_to_target=true propagation_staging=later-band-cascade decay_without_erasure=true previous_values=NO
Test1 urgency_cpu_oracle[0.2]=1.3 urgency_cpu_oracle[0.9]=4.8 urgency_gpu_eml[0.2]=1.3 urgency_gpu_eml[0.9]=4.8
Test2 cell_0_threat_after=10 cell_1_threat_after=8 cell_2_threat_after=0 locality_direct_skip_confirmed=YES
Test3 cell_1=8.0000 cell_2=6.4000 propagation_staging=later-band-cascade
Test4 decay_model=ScaleTarget*0.8 initial_max=10.0000 tick5_max=3.2768 tick10_max=1.0737 tick20_max=0.1153 monotone=YES
Test5 center=164 north=80 south=80 east=80 west=80 corner_min=0 corner_max=0 gradient_magnitude=40.0000 gradient_structured=YES
Test6 sum_reduction=10.00 urgency_gpu_eml[0.5]=5.0000 urgency_gpu_eml[0.1]=1.0000
Test7 DEFERRED previous_values_supported=NO
Test8 cells=196 mean_tick_ms=0.966 projected_30k_ms=147.9 projected_100k_ms=493.0
Test8 cells=1024 mean_tick_ms=1.446 projected_30k_ms=42.4 projected_100k_ms=141.2
Test9 threat[4][4]=0.00 gradient_at_4_4=(x=0, y=0, mag=0) gradient_direction_correct=PARTIAL
Test10 finding=B (conversion_rate alias admissible; custom names rejected; C-8 register_formula bypasses legacy whitelist)
```

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key numeric output |
|---|---|---|---|
| 0 | Substrate staging | PASS | staging=later-band-cascade, EvalEML=YES, decay=YES |
| 1 | P2 formula logic | PASS | urgency[0.2]=1.3 urgency[0.9]=4.8 (GPU EvalEML bit-exact) |
| 2 | P1 one-hop locality | PASS | cell_1=8 cell_2=0 |
| 3 | P1 two-hop locality | PASS | cell_1=8 cell_2=6.4 staging=later-band-cascade |
| 4 | P3 dissipation | PASS | tick10_max=1.07 tick20_max=0.12 monotone=YES |
| 5 | 3x3 field structure | PASS | gradient_magnitude=40 structured=YES |
| 6 | faction reduction + EML | PASS | sum=10 urgency_hi=5 urgency_lo=1 |
| 7 | velocity probe | DEFERRED | previous_values=NO velocity=DEFERRED |
| 8 | scale/cost | PASS | cells=196 mean_tick_ms=0.966 projected_30k=147.9 |
| 9 | gradient quality | PARTIAL | direction=PARTIAL mag=0 at [4,4] |
| 10 | whitelist | PASS (record) | finding=B |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
Core AccumulatorOp substrate is substrate-real for: (1) later-band-cascade local propagation via AddToTarget + ScaleSpec falloff, (2) GPU EvalEML personality-weighted urgency, (3) ScaleTarget dissipation without reset erasure, and (4) SlotRange Sum faction reduction. Gaps remain: (a) temporal velocity requires previous-value read not present in EML, (b) corridor gradient at distance on 10×10 did not receive sufficient signal under staged multi-band model without source re-injection artifacts — propagation radius/design work remains, (c) production legacy whitelist rejects custom formula class names (`propagation_formula`, `urgency_formula`) though C-8 `register_formula` bypasses legacy class gate in sandbox. Not YES because long-range gradient quality and velocity are unresolved; not NO because P1/P2/P3 primitives exist without new WGSL.
```

### Scale projection

```text
Best measured size: 196 cells (14×14 grid)
Mean tick time: 0.966 ms (prop-only follow-up ticks)
Values bytes: 4704
Readback bytes: 4704
AccumulatorOps: 1457 (initial setup incl. propagation graph)
EML trees: 2
EML nodes: 12
Projected 30,000 cells: 147.9 ms/tick (rough linear extrapolation)
Projected 100,000 cells: 493.0 ms/tick (rough linear extrapolation)
Projection caveat: rough linear extrapolation only; small-N GPU dispatch overhead may distort scaling.
1024-cell secondary probe: mean_tick_ms=1.446 projected_30k_ms=42.4 (better scaling at larger N — treat as advisory only).
```

Budget label:

```text
MARGINAL — 196-cell primary probe under 10 ms/tick (encouraging); 30k linear projection ~148 ms/tick needs profiling/batching before production budget claim.
```

### Gradient quality verdict

```text
Gradient direction at corridor was: partial (signal did not reach [4,4]; magnitude=0)
Gradient magnitude: 0 at [4,4]; 40 on 3×3 local grid
AI utility: weak for long-range heatmaps under current staged propagation pass count; local 3×3 structure is actionable
```

### Whitelist finding

```text
Production use requires: either map SEAD formulas to whitelisted legacy class aliases (e.g. conversion_rate) or extend whitelist/policy for field-intelligence formula classes; direct C-8 register_formula works in sandbox but production authoring path must be verified separately.
```

### Remaining gap outside this sandbox

AI decision quality cannot be fully tested programmatically. Whether the formula shapes produce strategically interesting and balanced behavior requires gameplay evaluation with realistic scenarios.

This sandbox establishes technical viability only.

---

## Preserved artifacts

- Source: [`docs/workshop/sead_sandbox_code_preserve.rs`](../workshop/sead_sandbox_code_preserve.rs)
- This report + full log (this directory)

Production test file `crates/simthing-driver/tests/sead_field_intelligence_sandbox.rs` is removed on revert to parked state.
