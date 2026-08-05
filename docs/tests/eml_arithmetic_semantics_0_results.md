# EML-ARITHMETIC-SEMANTICS-0 results

- Track: 0.0.8.7 (rung 5.14) · Status: **PROBATION / proof-present** (coding return)
- Base: `98180a4a4e7334fa9476c74170d995b5028202dc`
- Branch: `coding/eml-arithmetic-semantics-0`
- HD-RECEIPT: `b9070974440b` · ORIENT-RECEIPT: `fc5773df281f`
- Authority: DA `5191578752` + amendments `5191860082` + uniqueness `5192270934` · RESUME `5192307920`

## Delivered

1. **Author-facing arithmetic** in `docs/full_eml_unification.md` §4.1 and design §4 uniqueness
   re-expression: ADD/SUB/MUL/DIV single-rounding/no reassociation; MIN/MAX/CLAMP exact;
   EXP/LN pinned; uniqueness contraction; SEAM LAW = uniqueness instance.
2. **Faithful fused lowerings** with production plants that RED falloff per arm:
   - CPU: `mul_add` → `*+` plant
   - Interpreted: pad1 fuse flag cleared
   - SSA-JIT: emit `(lhs*rhs)+acc` instead of `fma`
3. **5.13 evidence plumbing deleted**; language witnesses in `exact_consumer_obligation_0.rs`.
4. **New EXP consumer** `soft-tail-exp` — zero ExactBearing / declaration / census.
5. **Census** discharged: five ADD hits are lawful `U`; SUB hits = 0.

## Frozen digests (unchanged)

- EXP exhaustive: `0x7875a45ba919d588`
- LN exhaustive: `0x196aced82d03f378`

## Line-count / grep

- `crates/simthing-kernel/src` Rust lines at head: **21870** (base ~22375; decreased)
- Deleted symbols absent from kernel `src`: `ExactBearingEvidence`, `derive_consumer_arms`,
  `ExactConsumerArm`, `ExactConsumerDigestEvidence`, `ExactConsumerExecutionShape`,
  `ExactConsumerShapeBinding`, `FieldConsumerShapeProof`, `exact_consumer_shape_proof`
