# EXACT-CONSUMER-OBLIGATION-0 results

- Track: 0.0.8.7 (rung 5.13, Phase 5 reopened) · Status: **PROBATION / proof-present / DA-review-pending**
- Base: `f7544feb` (exact) · Branch: `fable/exact-consumer-obligation-0`
- ORIENT-RECEIPT: `b780ce1ca97e` (rule stamp `45e629f979c3f629`) · HD-RECEIPT: `e4e6e5def6ac`
- Dispatch: Board `5187606339`; authority DA `5187245896` + #1642 corrections
- ANCHOR-ACK: all 28 projected anchors — ACK (ingested via the coding projection before edits)

## The receiving half of the door (ONE channel — remand 5190634963 §1–2 applied)

The obligation is folded into the **existing** production consumer path: `ExactPrimitiveConsumerEvidence`
now carries `exact_bearing: ExactBearingEvidence` (`NonExactBearing` | `ExactBearing { consumer_id,
primitive, domain_note, shape, digests }`), and `ExactPrimitiveAdmissionDoor::verify_consumer` enforces
it. The parallel channel from the rejected head (`ExactBearingConsumerDeclaration` /
`ExactConsumerAdmission` / `admit_exact_bearing_consumer`) is **deleted** — there is no second lawful
door, and a production call that declares exact-bearing but omits exactness evidence hard-errors inside
`verify_consumer` itself.

The execution-arm set is **derived, never caller-authored**, and the shape itself is **bound to the
production consumer surface** (remand 5190934274): evidence carries an `ExactConsumerShapeBinding` —
`OrdinaryAccumulatorEvalEml` lawful only for the AO consumer variant, or
`FieldSweep(FieldConsumerShapeProof)` lawful only for `FieldSweepEvalEml`, where the sealed proof is
mintable only from an **admitted** `FieldSweepRegistration` (no free constructor exists):
`exact_consumer_shape_proof()` reads the registration's typed `output`/`transient_read_proof` — the
same fields `can_fuse_transient_pair` consumes — so Matrix-vs-TransientFusable (and with it
fused-arm reachability) is determined by the admitted registration, never selected by the caller. A
binding that does not belong to the evidence's consumer variant REDs
(`ExactConsumerShapeNotBoundToConsumer`) before any digest row is read. `derive_consumer_arms(shape)`
then resolves the complete obligation (`FieldSweepMatrix` → cpu-twin + interpreted-gpu + ssa-jit;
`FieldSweepTransientFusable` → + fused-transient; `OrdinaryAccumulatorEvalEml` → cpu-twin +
interpreted-gpu) and evidence is compared against that derived set in both directions. Planted
defects ×7 all RED in `exact_consumer_obligation_0_admission_hard_errors_without_evidence`:

1. **Omitted real arm** — authored evidence carries matching cpu+interpreted rows only; derivation
   still requires ssa-jit for a `FieldSweepMatrix` shape → `ExactBearingConsumerWithoutDigestEvidence`.
   A caller cannot shrink its own obligation (remand §2's required defect).
2. Exact-bearing declared with **no digest evidence at all** → RED at the first derived arm.
3. **Zero digest** → RED (no evidence).
4. **Arm digest mismatch** (the pre-repair seam shape: jit ≠ cpu/interpreted) →
   `ExactConsumerArmDigestMismatch`.
5. **Non-derived arm row** (an AO consumer presenting a field-JIT digest) →
   `ExactConsumerArmNotDerived` — the derivation is authoritative in both directions.
6. **The shape-binding bypass** (remand 5190934274's exact scenario): `FieldSweepEvalEml` + the AO
   shape + two internally consistent matching digests → `ExactConsumerShapeNotBoundToConsumer` —
   a field consumer cannot shed its SSA-JIT obligation via an authored classification.
7. **The inverse binding bypass**: an AO consumer presenting a field-sweep shape proof → same RED.

No waiver, no grandfather, no inheritance — §4 Exact-Value Provenance Law mechanized. The EXP and LN
qualification rituals themselves now admit through this folded evidence (their necessity consumers carry
`ExactBearing` digests measured by this rung's battery); the 5.10-era door-shape fixtures are
`NonExactBearing` and exercise the necessity gate only.

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

## Consumer digests (each over its OWN probe domain; every arm EXECUTED here)

Every digest below was produced by executing that arm in this battery — the AO interpreted-gpu
digests run the **real AO EvalEML GPU interpreter** per consumer (register → upload tree → one
EvalEML op per probe row → `tick_with_eml` → readback → hash in row order) and are compared to the
CPU twin as independently-produced numbers (remand §3). No digest is copied between arms, and no
inherited generic parity battery is cited as substitute evidence.

| Consumer | Primitive | Derived arms (from shape) | Digest |
|---|---|---|---|
| stead-exponential-falloff | EXP | FieldSweepMatrix → cpu-twin, interpreted-gpu, ssa-jit (fused-transient does not derive: fusion requires a Transient producer) | `0x3d60d7448fca13f8` ALL-IDENTICAL |
| log-accumulate | LN | FieldSweepMatrix → cpu-twin, interpreted-gpu, ssa-jit | `0x84cb4316b67aeedb` ALL-IDENTICAL |
| logistic-steering | EXP | OrdinaryAccumulatorEvalEml → cpu-twin, interpreted-gpu (field JIT never compiles AO programs) | cpu `0x9203d12c4ca28325` = interpreted-gpu `0x9203d12c4ca28325` INDEPENDENT+IDENTICAL |
| softmax-weight | EXP | OrdinaryAccumulatorEvalEml → cpu-twin, interpreted-gpu | cpu `0xfb0a55b88473185f` = interpreted-gpu `0xfb0a55b88473185f` INDEPENDENT+IDENTICAL |
| power-law | EXP+LN | OrdinaryAccumulatorEvalEml → cpu-twin, interpreted-gpu | cpu `0xaf86f64c3b5adb9d` = interpreted-gpu `0xaf86f64c3b5adb9d` INDEPENDENT+IDENTICAL |
| eml-operator | EXP+LN | OrdinaryAccumulatorEvalEml → cpu-twin, interpreted-gpu | cpu `0xee9ddc3c4fda1d38` = interpreted-gpu `0xee9ddc3c4fda1d38` INDEPENDENT+IDENTICAL |
| entropy-term | LN | OrdinaryAccumulatorEvalEml → cpu-twin, interpreted-gpu | cpu `0x237fd8a4e6e9c53a` = interpreted-gpu `0x237fd8a4e6e9c53a` INDEPENDENT+IDENTICAL |

## Reconciliation (remand §4 — census and sweep published separately)

**Tree-derived registration census (production types).** The production consumer identity type
`ExactPrimitiveConsumer` has exactly **2 variants** — `OrdinaryAccumulatorEvalEml` and
`FieldSweepEvalEml` — and the production shape type `ExactConsumerExecutionShape` has 3 variants
(`FieldSweepMatrix`, `FieldSweepTransientFusable`, `OrdinaryAccumulatorEvalEml`) whose derived arm
sets are enumerated in full above. That is the type tree; the 7 rows in the digest table are
**concrete consumers admitted through the one channel** (`verify_consumer` with `ExactBearing`
evidence), classified onto those 2 variants: 5 × `OrdinaryAccumulatorEvalEml` (logistic-steering,
softmax-weight, power-law, eml-operator, entropy-term) + 2 × `FieldSweepEvalEml`
(stead-exponential-falloff, log-accumulate). Test cases are not type-walk registrations; the type
census and the concrete-consumer roll are different facts and are published separately here.

**Independent raw usage sweep.** Grep
(`EML_OP_EXP|EML_OP_LN|OP_EXP|OP_LN|eml_exp|eml_ln|opcode::EXP|opcode::LN|power_law|eml_operator|PowerLaw`
over `crates/**/*.{rs,wgsl}`) returned 25 files; every hit closes through the one channel or is
recorded provably non-consuming with reason:

| Class | Files | Disposition |
|---|---|---|
| Concrete consumers (close through `verify_consumer` + `ExactBearing`) | `field_sweep_compile.rs` (falloff), `property.rs` (logistic), `eml_opcode_gate.rs` (softmax + LN gadget builders) | the 7 digest rows above |
| Primitive definitions | `eml_exp.rs`, `eml_ln.rs`, `eml_ln_ds_candidate.wgsl` | non-consuming: the primitives themselves |
| Execution substrate | `eml_nodes.rs`, `eml_registry.rs`, `intensity_eml.rs`, `cpu_oracle.rs`, `eml_resource_class.rs`, `field_sweep.rs`, `field_sweep.wgsl`, `accumulator_op.wgsl`, `eml_gadget.rs`, core/kernel `lib.rs` | non-consuming: interpreter/JIT arms and exports carry values, they do not consume exactness |
| Qualification / referees | `eml_exp_qualification.rs`, `eml_ln_qualification.rs`, workshop `eml_*_qualification.rs`, `eml_*_cost_evidence.rs`, `eml_exp_consumer_0.rs`, `exact_consumer_obligation_0.rs` | referees and pinned artifacts; the two ritual necessity-consumers admit through the one channel with `ExactBearing` digests (see above) |

No unregistered consumer found; **the counts (2 variants, 3 shapes, 7 concrete consumers) are
results, not targets**.

## Verification

```text
cargo test -p simthing-workshop --test exact_consumer_obligation_0: 4 passed (hard-errors ×7 planted
  incl. both shape-binding bypasses, falloff 3-arm, AO ×5 with independently-executed
  interpreted-gpu digests, log-accumulate 3-arm)
cargo test -p simthing-kernel --lib: 28 passed (folded verify_consumer gate + EXP/LN rituals
  admitting through the one channel with ExactBearing digests)
cargo test -p simthing-workshop --test eml_exp_primitive_0_qualification: 5 passed (witness now
  bit-identity, 0/32; falloff 0/32)
cargo test -p simthing-workshop --test eml_resource_class_jit_parity_0: 1 passed
cargo test -p simthing-driver --test field_sweep_n4_parity_0: 3 passed
EXP/LN exhaustive JIT replays under SEAM LAW: pinned digests hold (see relay)
bash scripts/ci/agent_scan.sh --base f7544feb: see relay
```

## Posture

No `/clearance`, merge, pointer movement, 6.4/StemThing-A, or successor work. 5.12 untouched.
