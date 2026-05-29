# Phase M-JIT-0 — Generic EvalEML WGSL JIT Prototype — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `4985bfc02a0e179515914fca43ca0633dd23820a`  
**Final commit SHA:** `b7eda79` (implementation commit on `phase-m-jit0-evaleml-wgsl-prototype`; the squash/merge commit SHA is recorded by the PR merge)  
**Lane classification:** Tier-2 JIT prototype gate (V7.7 §5 / `docs/workshop/phase_m_gating_and_doc_policy.md`)  
**Decision:** **IMPLEMENTED — test-only generic WGSL emission prototype**  
**Verdict:** **PASS — M-JIT-0 landed**

---

## Pre-Edit Evaluation Summary

Inspected: `docs/invariants.md`, `docs/workshop/mapping_current_guidance.md`,
`docs/accumulator_op_v2_production_plan.md`, `docs/workshop/eml_gadget_library_design_note.md`,
`docs/tests/phase_m_eml_gadget_runtime_execution_gate_test_results.md`,
`crates/simthing-driver/tests/phase_m_eml_gadget_runtime_execution_gate.rs`,
`crates/simthing-spec/src/compile/eml_gadget.rs`, `crates/simthing-spec/src/compile/eml_registry.rs`,
`crates/simthing-gpu/src/accumulator_op/eml_program_table.rs`, the `eval_eml_postfix` /
`eval_eml_cpu` surfaces, and the existing `structured_field_stencil.rs` `wgpu`
shader-module/pipeline/readback test pattern.

| Question | Answer |
|---|---|
| **1. Safe compiled node/op subset for M-JIT-0?** | `LITERAL_F32`, `SLOT_VALUE`, `ADD`, `SUB`, `MUL`, `RETURN_TOP`. This exactly covers `WeightedAccumulator` (`SLOT/MUL/ADD/RETURN_TOP`) and `Ema` (`SLOT/LITERAL/MUL/ADD/RETURN_TOP`). `DIV`/`CLAMP`/`ABS`/`CMP_*`/`SELECT`/`PARAM` are out of scope and rejected. |
| **2. `WeightedAccumulator` lowerable without new opcode?** | **Yes.** Straight-line `values[base+input]*values[base+weight]` products summed via existing `MUL`/`ADD` lowering. No new opcode. |
| **3. `Ema` lowerable without hidden previous reads?** | **Yes.** `previous_col` is read explicitly from the same generic `values` buffer (`col_13`); `decay`/`1-decay` are exact-bit literals; output writes the explicit `output_col`. Single storage binding — no hidden previous buffer. |
| **4. Deterministic / snapshot-stable WGSL?** | **Yes.** Emission is a pure function of `(nodes, output_col, n_dims)`; node order and column-dedup order are deterministic; repeated emission is byte-identical (asserted). |
| **5. Generic names only?** | **Yes.** `values`, `slot`, `n_dims`, `base`, `col_<idx>`, `tmp_<n>`, `out_col`. No designer/gameplay identifiers; gadget RON ids do not leak. |
| **6. GPU vs `eval_eml_postfix` and named oracle?** | **Yes.** Bit-exact (`f32::to_bits`) against `eval_eml_postfix`, `eval_eml_cpu`, and `oracle_weighted_accumulator` / `oracle_ema`. |
| **7. Deferred for production JIT?** | Cohorting identical graphs, kernel caching/dispatch, multi-gadget chained-stack lowering, conditional (`CMP`/`SELECT`) lowering, batched multi-slot dispatch, and any default/production wiring. All remain separately gated. |

The prior EML-GADGET runtime gate proved the existing C-8a EvalEML interpreter executes these
compiled nodes with oracle parity; that path remains the interpreter substrate and is unchanged.
M-JIT-0 is the orthogonal proof that the same admitted graphs lower to straight-line WGSL.

---

## Supported Subset

| Formula | Fixture | Lowered WGSL math |
|---|---|---|
| WeightedAccumulator | `fixtures/jit_weighted_accumulator.ron` (in `[3,4]`, w `[20,21]`, out `16`) | `values[base+16] = col_3*col_20 + col_4*col_21` |
| Ema | `fixtures/jit_ema.ron` (in `3`, prev `13`, out `13`, decay `0.85`) | `values[base+13] = col_13*decay + col_3*(1-decay)` (decay literals via exact-bit `bitcast<f32>`) |

`SoftStep`/`Hysteresis`/`CMP`/`SELECT`/`DIV` conditional lowering deliberately **not** added
(deferred to M-JIT-1).

---

## Generated WGSL Safety Summary

- Single generic storage binding (`@group(0) @binding(0) values: array<f32>`); one bind-group entry.
- Straight-line `let` lowering; no loops, no recursion, no transcendental functions, no `sqrt`.
- Float literals emitted as exact-bit `bitcast<f32>(<u32 bits>)` — bit-identical to the compiled
  node, removing decimal round-trip risk and guaranteeing CPU/GPU parity.
- Only generic identifiers (`values`/`slot`/`n_dims`/`base`/`col_*`/`tmp_*`/`out_col`).
- No designer/gameplay semantics; no RON gadget id; no `simthing-sim`/`SimSession`/`economy`/
  `mapping`/`Personality`/`Memory`/`Gadget` tokens (asserted).

---

## Semantic-Free Scan Results

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD" \
   crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs crates/simthing-gpu/src/shaders \
   docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md
```
**Result:** In the test file the only matches are the `FORBIDDEN_SEMANTIC_TERMS` negative-assertion
guardrail list (lines 37–38). No matches in `crates/simthing-gpu/src/shaders`. In this report the
terms appear only as forbidden/guardrail context. Generated WGSL is produced at runtime and asserted
semantic-free by the tests.

```
rg "sqrt|GradientXY|output_col_x|output_col_y|new EML opcode" crates docs
```
**Result:** In the new test file, `sqrt`/`new EML opcode` appear only in the header guardrail comment
and the test-6 forbidden-token list. No new math/opcode/output-contract work.

```
rg "default SimSession mapping|production economy→mapping bridge|ResourceEconomySpec.*mapping|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|...|atlas|M-4A|ActiveOnlyExperimentalNoHalo" crates docs
```
**Result:** In the new test file, `ResourceEconomySpec` appears only in the test-6 forbidden-token
list. No new prohibited implementation; existing matches elsewhere are guardrail/deferred context.

```
rg "JIT|generated WGSL|shader generation|semantic-free|eval_eml_postfix|eval_eml_cpu|oracle_weighted_accumulator|oracle_ema" crates docs
```
**Result:** The emitter symbol `emit_evaleml_wgsl` and JIT shader-generation appear only in the new
test file; oracle/parity surfaces are pre-existing in `simthing-spec`/`simthing-gpu`. No production
JIT wiring.

(Scans executed via the ripgrep-backed Grep tool; `rg` is not on the shell PATH in this environment.)

---

## CPU/Spec/GPU Oracle Parity Results

| Formula | GPU vs `eval_eml_postfix` | GPU vs `eval_eml_cpu` | GPU vs named oracle |
|---|---|---|---|
| WeightedAccumulator (`12.5,-3.25` × `0.75,2.0`) | bit-exact | bit-exact | bit-exact (`oracle_weighted_accumulator`) |
| Ema (`input 4.0`, `prev 10.0`, `decay 0.85`) | bit-exact | bit-exact | bit-exact (`oracle_ema`) |

Comparison via `f32::to_bits()` equality.

---

## Unsupported Opcode Rejection Result

`fixtures/jit_field_sampler_unsupported.ron` (FieldSampler → `DIV` + `CLAMP_BOUNDED`) returns a
structured `JitEmitError` containing `"unsupported opcode"`; no shader is produced and there is no
fallback to unsafe interpretation inside the generated shader. An empty node program is also
rejected.

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_evaleml_wgsl_prototype -- --nocapture
```
**Result:** **6 passed; 0 failed** (`jit_weighted_accumulator_generates_semantic_free_wgsl`,
`jit_weighted_accumulator_gpu_matches_oracles`, `jit_ema_generates_semantic_free_wgsl`,
`jit_ema_gpu_matches_oracles`, `jit_rejects_unsupported_opcode_or_shape`,
`jit_is_test_only_and_default_off`).

```
cargo test -p simthing-driver --test phase_m_eml_gadget_runtime_execution_gate -- --nocapture
```
**Result:** **6 passed; 0 failed** — existing EvalEML interpreter runtime fixture remains green.

```
cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture
```
**Result:** **14 passed; 0 failed**

```
cargo test -p simthing-spec --test eml_gadget_tier2_temporal -- --nocapture
```
**Result:** **10 passed; 0 failed**

```
cargo check --workspace
```
**Result:** **PASS** (only pre-existing unrelated warnings).

Broader (touched-surface-adjacent confidence):

```
cargo test -p simthing-spec --test eml_gadget_tier2_hysteresis -- --nocapture   # 16 passed
cargo test -p simthing-spec --test eml_gadget_tier2_acceleration -- --nocapture # 11 passed
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture # 28 passed
```

---

## Scans Run and Results

See **Semantic-Free Scan Results** above; all four required scans recorded. Summary:

- Generated WGSL fixtures contain no semantic terms.
- JIT/shader-generation appears only in the new test/report/docs.
- No new production wiring, no `sqrt`/`GradientXY`/new-opcode work.
- Prohibited-token matches are guardrail/deferred context only.

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files present; **no scratch/tmp artifacts found;
nothing deleted** (historical full logs preserved per policy).

---

## Files Changed

- `crates/simthing-driver/Cargo.toml` — test-only `wgpu`/`bytemuck`/`pollster` dev-dependencies
- `crates/simthing-driver/tests/fixtures/jit_weighted_accumulator.ron`
- `crates/simthing-driver/tests/fixtures/jit_ema.ron`
- `crates/simthing-driver/tests/fixtures/jit_field_sampler_unsupported.ron`
- `crates/simthing-driver/tests/phase_m_jit_evaleml_wgsl_prototype.rs`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`
- `docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md` — this report

---

## Posture Attestation

No semantic WGSL, no production JIT wiring, no default mapping wiring, no simthing-sim
Gadget/Personality/Memory semantics, no new EML opcode, no sqrt, no chained scheduling, no
automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production
economy→mapping bridge; M-JIT-0 is a test-only generic WGSL emission prototype with CPU/spec/GPU
oracle parity; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M-JIT-0 landed as a test-only generic EvalEML WGSL emission prototype for
WeightedAccumulator and Ema, with deterministic semantic-free generated WGSL, successful wgpu
compilation, CPU/spec/GPU oracle parity, explicit unsupported-op rejection, and the existing
EvalEML interpreter runtime fixture left green.
