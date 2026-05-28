# SEAD operator toolkit probe — test results

**Date/time:** 2026-05-19  
**Base HEAD (before sandbox branch):** `7e52c94` — SEAD strategic horizon revert HEAD doc update  
**Sandbox merge SHA:** _(filled after merge)_  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — 11/11 sandbox tests PASS (run with `--test-threads=1`).

**Note:** Operator comparison uses **directed south/east propagation** after source zeroing (same stability posture as probes 1–2). Stabilized operators combine ScaleTarget decay bands with AddToTarget directed propagation where applicable. `max_saturating` is **DEFERRED** — no cross-slot max in one EvalEML write.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-driver --test sead_operator_toolkit_sandbox -- --nocapture --test-threads=1` | **PASS** — 11/11 |

**Full log:** [`sead_operator_toolkit_sandbox_full.log`](sead_operator_toolkit_sandbox_full.log)

---

## Decision Gate Summary

| Test | Area | PASS / PARTIAL / DEFERRED / FAIL | Key result |
|---|---|---|---|
| 0 | Capability sanity | PASS | EvalEML=YES, AddToTarget=YES, ScaleTarget_decay=YES, copy=YES, SlotRange_sum=YES |
| 1 | Stabilized operators | PARTIAL | best_operator=directed_decayed H=8; raw_additive blowup=YES at H=24; clamped_additive saturates at H≥8 |
| 2 | Dirty/frontier skip | PARTIAL | H=8 clean_skip_ratio=0.37 dirty_ratio=0.63; H=16 dirty_ratio=0.96 |
| 3 | Cadence + frontier | PARTIAL | full_H8 dir=correct; model=C direction=none; model=D/E direction=partial |
| 4 | Whitelist/admission | PARTIAL | finding=C — legacy whitelist rejects field_* classes; C-8 register_formula=YES |
| 5 | Hierarchy-first awareness | PASS | hierarchy_ms=1.45 lateral_H8_ms=21.09 faction_threat=512 |
| 6 | Hybrid model | PARTIAL | local_gradient=correct faction_urgency=5758 blowup=NO |
| 7 | PF/dirty comparison | PARTIAL | pf_added_value=PARTIAL; first_clean_candidate_tick=NONE |
| 8 | Cost projection | PARTIAL | projected_30k_dirty_adjusted=3236.6ms budget=OVER BUDGET |

### Overall verdict

```text
PARTIAL
```

Reason:

```text
The substrate supports a production toolkit pattern: directed_decayed propagation reaches [4][4] with correct gradient by H=8 without blowup; hierarchy Sum→faction→urgency EML is ~15× cheaper than full lateral H=8 for faction awareness; dirty/frontier skips ~37% of cells at H=8 on 10×10. Not YES because: (1) raw_additive and long-horizon operators amplify badly; clamped_additive loses gradient after saturation; (2) frontier-restricted multi-tick cadence (models C/D/E) fails to preserve directional signal at effective H=16; (3) dirty skip collapses at H=16 (96% dirty); (4) 30k/100k projections remain OVER BUDGET even with dirty ratio; (5) field formula classes need whitelist/admission policy work (finding C). Not NO because stabilized decay+propagate, dirty frontier, hierarchy reduction, and hybrid local+tactical/strategic split are all expressible on current AccumulatorOp v2 without new WGSL or primitives.
```

### Recommended ADR toolkit

```text
recommended_propagation_operator = directed_decayed (ScaleTarget decay band + directed SE AddToTarget; normalized_neighbor viable alternative with lower peak values)
recommended_cadence_model = full-grid H=8 one-shot for tactical corridor gradient; multi-tick frontier cadence deferred until frontier filter preserves direction at effective H=16
recommended_dirty_frontier_policy = first-line skip at short horizons (H≤8); expect frontier expansion at H≥16 — pair with hierarchy-first strategic refresh rather than full-grid long horizon
recommended_hierarchy_strategy = SlotRange Sum cell→faction + EvalEML urgency at parent for empire-scale awareness; lateral SEAD for local tactical gradient
recommended_velocity_model = explicit columns (validated in probe 2; not re-tested here)
recommended_pf_role = cooling classifier / observability only — not first-line skip; dirty bit handles cold empty regions
recommended_whitelist_action = policy-only: add field_propagation / bounded_field_update / field_pressure / field_decay to designer whitelist or map to existing aliases; runtime C-8 register_formula path already accepts them
```

### Guardrail placement finding

```text
runtime substrate — directed_decayed and hierarchy Sum reduction are substrate-real; no new opcodes required
spec/RON/designer policy — field formula class whitelist/admission (finding C); CLAMP_BOUNDED available but saturation kills gradient — authoring should cap source injection not rely on post-hoc clamp for long horizons
scenario authoring limits — source zeroing after cluster seed; directed SE vs full NSEW; horizon caps before blowup
production profiling — dirty_ratio and cadence model selection; 30k projection OVER BUDGET even with dirty skip — cadence + hierarchy mandatory at scale
```

---

## Test 1 — Stabilized operator comparison (detail)

| Operator | first_nonzero_H | first_directional_H | H=8 t44 | H=8 dir | H=16 blowup | Notes |
|---|---|---|---|---|---|---|
| raw_additive | 8 | 8 | 362.8 | correct | NO | H=24 blowup=YES |
| decayed_accumulate | 8 | 8 | 72.5 | correct | NO | amplifies at H≥16 |
| normalized_neighbor | 8 | 8 | 5.5 | correct | NO | lower peaks; H=24 partial |
| clamped_additive | 4 | 4 | 10000 | none | NO | saturates — gradient lost |
| max_saturating | — | — | — | — | — | **DEFERRED** |
| directed_decayed | 8 | 8 | 118.6 | correct | NO | **best candidate** |

Pass criteria assessment: **PARTIAL** — multiple operators reach [4][4] by H≤16 with correct gradient, but only decay-family operators remain stable; raw additive blows up at H=24; clamp loses direction after saturation.

---

## Test 2 — Dirty/frontier skip (detail)

```text
Dirty/frontier estimate (directed_decayed):
  10×10 H=8:  total=100 reached=63 dirty=63 clean_skip_ratio=0.370 frontier=28
  10×10 H=16: total=100 reached=96 dirty=96 clean_skip_ratio=0.040 frontier=35
  30×30 H=8:  total=900 reached=20 dirty=20 clean_skip_ratio=0.978 frontier=20
  30×30 H=16: total=900 reached=35 dirty=35 clean_skip_ratio=0.961 frontier=32
```

Pass criteria assessment: **PARTIAL** — substantial skippable space at H=8 on 10×10 and on sparse 30×30; frontier grows quickly on 10×10 at H=16.

---

## Test 3 — Cadence + frontier (detail)

```text
Cadence/frontier:
  A full_H8:   total_ms=18.72 dir=correct
  B full_H16:  total_ms=34.96 dir=partial
  C 4×4:       mean_tick_ms=10.16 active_mean=18 t44=0.0 direction=none
  D 2×8:       mean_tick_ms=6.48  active_mean=35.6 t44=2552.7 direction=partial
  E 1×16:      mean_tick_ms=4.02  active_mean=62.3 t44=2859.3 direction=partial
```

Pass criteria assessment: **PARTIAL** — per-tick cost drops with frontier cadence but directional signal is lost (C) or only partial (D/E) at effective H=16.

---

## Test 4 — Whitelist/admission (detail)

```text
field_propagation      legacy=NO C8=YES finding=C
bounded_field_update   legacy=NO C8=YES finding=C
field_pressure         legacy=NO C8=YES finding=C
field_decay            legacy=NO C8=YES finding=C
conversion_rate        legacy=YES C8=YES finding=E
```

No whitelist modified in sandbox. GPU EvalEML executes for trees registered via C-8 path.

---

## Test 5–7 — Hierarchy / hybrid / PF (detail)

```text
Hierarchy-first: lateral_H8_ms=21.09 hierarchy_reduction_ms=1.45 faction_threat=512 faction_urgency=256 local_gradient=correct
Hybrid: local_gradient=correct faction_urgency=5758 mean_tick_ms=1.85 active_cells=63 field_max=440.4 blowup=NO
PF/dirty: first_clean_candidate_tick=NONE dirty_skip_cells=51 clean_skip_ratio=0.510 pf_applicable_cells=0 pf_added_value=PARTIAL
```

---

## Test 8 — Cost summary (detail)

```text
Cost summary:
  best_operator=directed_decayed
  best_cadence=full_H8 (tactical); frontier multi-tick marginal for direction
  dirty_ratio=0.620 (H=8 10×10)
  mean_tick_ms=17.40
  projected_30k_naive=5220.3ms
  projected_30k_dirty_adjusted=3236.6ms
  projected_100k_naive=17400.9ms
  projected_100k_dirty_adjusted=10788.6ms
  budget=OVER BUDGET
```

Projections use `measured_ms × (target_cells / measured_cells) × dirty_ratio` — rough estimates only.

---

## Raw test output (key numerics)

```
Test0 EvalEML_GPU=true AddToTarget=true ScaleTarget_decay=true copy_current_to_previous=true SlotRange_sum=true
best_operator_candidate=directed_decayed H=8
directed_decayed H=8 t44=118.57 grad=(-92.02,-93.44) mag=131.14 max=319.3 dir=correct
raw_additive H=24 blowup=YES max=58224836
max_saturating DEFERRED
Dirty/frontier H=8 clean_skip_ratio=0.370
Cadence A full_H8 dir=correct ms=18.72
Cadence C direction=none
Hierarchy hierarchy_ms=1.45 lateral_ms=21.09
Hybrid local_gradient=correct faction_urgency=5758.46
PF first_clean_candidate_tick=NONE clean_skip_ratio=0.510
Cost projected_30k_dirty_adjusted=3236.6ms budget=OVER BUDGET
```

---

## Preserved artifacts

- Source: [`docs/workshop/sead_operator_toolkit_sandbox_code_preserve.rs`](../workshop/sead_operator_toolkit_sandbox_code_preserve.rs)
- This report + full log

Production test file removed on revert to parked state.
