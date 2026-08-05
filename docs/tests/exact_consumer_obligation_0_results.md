# EXACT-CONSUMER-OBLIGATION-0 results

- Track: 0.0.8.7 (rung 5.13, Phase 5 reopened) · Status: **PROBATION / proof-present / DA-review-pending**
- Base: `f7544feb` (exact) · Branch: `fable/exact-consumer-obligation-0`
- ORIENT-RECEIPT: `b780ce1ca97e` (rule stamp `45e629f979c3f629`) · HD-RECEIPT: `e4e6e5def6ac`
- Dispatch: Board `5187606339`; authority DA `5187245896` + #1642 corrections
- ANCHOR-ACK: all 28 projected anchors — ACK (ingested via the coding projection before edits)

## The receiving half of the door

`ExactBearingConsumerDeclaration` + `ExactConsumerDigestEvidence` +
`admit_exact_bearing_consumer` (kernel gate): an exact-bearing consumer declares its primitive,
its own probe domain, and its **derived, justified execution-arm set** (never a fixed count),
and must present a digest per arm, all bit-identical. Missing arm evidence, a zero digest, an
empty arm set, or an arm mismatch is a **production admission hard-error**
(`ExactBearingConsumerWithoutDigestEvidence` / `ExactConsumerArmDigestMismatch`). No waiver, no
second channel, no inheritance — §4 Exact-Value Provenance Law mechanized. Planted defects
(missing ssa-jit evidence; zero digest; mismatched jit digest — the pre-repair seam shape;
empty arm set) all RED in `exact_consumer_obligation_0_admission_hard_errors_without_evidence`.

## SEAM LAW — the STEAD falloff map/fold repair (in this rung, as ordered)

Measured basis: the certified tuple contracts the unfused canonical-Sum seam regardless of
source form (an `fma(a,b,-0.0)` probe was algebraically re-simplified and re-contracted). The
repair **specifies the seam fused on every arm**: when a map ends in `MUL` and the fold is the
canonical Sum, `acc = fma(a, b, acc)` in one rounding — CPU twin (prefix evaluator +
`mul_add`), interpreted arm (registration-derived uniform `params` flag + pair evaluator +
`fma`; no data-dependent branch — the flag is uniform), SSA-JIT (generated fused fold).
Exact-by-construction semantics; no opcode, no 5.10 widening, no escape hatch.

- Witness tightened from ≤1 ULP to **bit-identity**: EXP-free seam witness **0/32 drift**
  (pre-repair: 3/32); STEAD falloff consumer **0/32** (pre-repair: 2/32). The pre-repair RED
  is carried by the admission mismatch planted defect (jit digest ≠ cpu digest hard-errors) and
  the two now-hard witness asserts, which fail on the unfused emission by construction.
- Inherited: three-way JIT census green; N4 parity battery green (3/3 — includes Gu-Yang flux,
  which shares the seam shape; all arms agree under the law).
- EXP/LN exhaustive JIT replays re-run under the SEAM LAW: pinned digests hold (their
  qualification programs use trivial folds; see relay for the re-run lines).

## Consumer digests (each over its OWN probe domain; arms derived + justified)

| Consumer | Primitive | Arms (justification) | Digest |
|---|---|---|---|
| stead-exponential-falloff | EXP | cpu-twin, interpreted-gpu, ssa-jit (field Matrix registration; fused-transient unreachable — Matrix output) | `0x3d60d7448fca13f8` ALL-IDENTICAL |
| log-accumulate | LN | cpu-twin, interpreted-gpu, ssa-jit (field Matrix registration; fused-transient unreachable) | `0x84cb4316b67aeedb` ALL-IDENTICAL |
| logistic-steering | EXP | cpu-twin, interpreted-gpu (ordinary AO EvalEML; field JIT never compiles AO programs) | `0x9203d12c4ca28325` |
| softmax-weight | EXP | cpu-twin, interpreted-gpu (AO) | `0xfb0a55b88473185f` |
| power-law | EXP+LN | cpu-twin, interpreted-gpu (AO) | `0xaf86f64c3b5adb9d` |
| eml-operator | EXP+LN | cpu-twin, interpreted-gpu (AO) | `0xee9ddc3c4fda1d38` |
| entropy-term | LN | cpu-twin, interpreted-gpu (AO) | `0x237fd8a4e6e9c53a` |

AO-surface interpreted-arm equality is carried by the standing C-8 AO parity pair (the AO
interpreter ↔ shared CPU stack machine) and each consumer's 5.11/5.12 oracle-parity battery;
the declarations record that justification rather than hedging.

## Reconciliation — type-walk vs raw usage (the count is a result)

Type-walk (registered rows): the 7 consumers above. Independent raw sweep
(`EML_OP_EXP|EML_OP_LN|OP_EXP|OP_LN|eml_exp|eml_ln|opcode::EXP|opcode::LN|power_law|eml_operator|PowerLaw`
over `crates/**/*.{rs,wgsl}`) returned 25 files; every hit closes as:

| Class | Files | Disposition |
|---|---|---|
| Registered consumers | `field_sweep_compile.rs` (falloff), `property.rs` (logistic), `eml_opcode_gate.rs` (softmax + LN gadget builders) | rows above |
| Primitive definitions | `eml_exp.rs`, `eml_ln.rs`, `eml_ln_ds_candidate.wgsl` | non-consuming: the primitives themselves |
| Execution substrate | `eml_nodes.rs`, `eml_registry.rs`, `intensity_eml.rs`, `cpu_oracle.rs`, `eml_resource_class.rs`, `field_sweep.rs`, `field_sweep.wgsl`, `accumulator_op.wgsl`, `eml_gadget.rs`, core/kernel `lib.rs` | non-consuming: interpreter/JIT arms and exports carry values, they do not consume exactness |
| Qualification / referees | `eml_exp_qualification.rs`, `eml_ln_qualification.rs`, workshop `eml_*_qualification.rs`, `eml_*_cost_evidence.rs`, `eml_exp_consumer_0.rs`, `exact_consumer_obligation_0.rs` | non-consuming: referees and pinned artifacts, not production consumers |

No unregistered consumer found; **7 is the result, not a target**.

## Verification

```text
cargo test -p simthing-workshop --test exact_consumer_obligation_0: 4 passed (hard-errors ×4 planted,
  falloff 3-arm, AO ×5, log-accumulate 3-arm)
cargo test -p simthing-workshop --test eml_exp_primitive_0_qualification: 5 passed (witness now
  bit-identity, 0/32; falloff 0/32)
cargo test -p simthing-workshop --test eml_resource_class_jit_parity_0: 1 passed
cargo test -p simthing-driver --test field_sweep_n4_parity_0: 3 passed
EXP/LN exhaustive JIT replays under SEAM LAW: pinned digests hold (see relay)
bash scripts/ci/agent_scan.sh --base f7544feb: see relay
```

## Posture

No `/clearance`, merge, pointer movement, 6.4/StemThing-A, or successor work. 5.12 untouched.
