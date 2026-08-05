# EML-ARITHMETIC-SEMANTICS-0 results

- Track: 0.0.8.7 (rung 5.14) · Status: **PROBATION / proof-present** (coding return)
- Base: `98180a4a4e7334fa9476c74170d995b5028202dc`
- Branch: `coding/eml-arithmetic-semantics-0`
- HD-RECEIPT: `b9070974440b` · ORIENT-RECEIPT: `fc5773df281f`
- Authority: DA `5191578752` + amendments `5191860082` + uniqueness `5192270934` · RESUME `5192307920` · REMAND `5192641222`

## Delivered

1. **Author-facing arithmetic** in `docs/full_eml_unification.md` §4.1 and design §4:
   ADD/SUB/MUL/DIV single-rounding/no reassociation; MIN/MAX/CLAMP exact; EXP/LN pinned;
   uniqueness contraction on **`ADD` only**; SEAM LAW = uniqueness instance.
2. **Standalone opcode faithfulness** (`eml_arithmetic_semantics_0_standalone_opcodes_match_ieee_on_derived_arms`):
   ADD/SUB/MUL/DIV match IEEE f32 bits on the arms that actually execute OrdinaryAccumulatorEvalEml
   (CPU twin + interpreted WGSL; SSA-JIT is not-an-arm for these AO programs).
3. **Faithful fused lowerings** with production plants that RED falloff:
   - CPU: `mul_add` → separate `*` then `+` — **REDs**
   - Interpreted: pad1 fused flag cleared (map MUL + Sum fold separate) — **REDs**
   - SSA-JIT: truthful plant emits ordinary Sum fold instead of fused `fma` fold body
     (MUL in `eval_map`, ADD in `eval_fold`). On the certified toolchain this is
     **re-contracted to the same bits as fused** — measured by
     `eml_arithmetic_semantics_0_jit_seam_separate_rounding_plant_is_recontracted`.
     A post-`fma` bit-flip plant was rejected by remand as not falsifying fusion.
     **BOUNDED STOP:** no lawful separate-rounding JIT mutant remains without an
     optimizer fence.
4. **5.13 evidence plumbing deleted**; language witnesses in `exact_consumer_obligation_0.rs`.
5. **New EXP consumer** soft-tail — zero ExactBearing / declaration / census.
6. **Census** discharged: five ADD hits are lawful `U` (ADD-only uniqueness).

## Frozen digests (unchanged)

- EXP exhaustive: `0x7875a45ba919d588`
- LN exhaustive: `0x196aced82d03f378`

## Line-count / grep

- `crates/simthing-kernel/src` Rust lines decreased vs base (~22375 → lower).
- Deleted symbols absent from kernel `src`: `ExactBearingEvidence`, `derive_consumer_arms`,
  `ExactConsumerArm`, `ExactConsumerDigestEvidence`, `ExactConsumerExecutionShape`,
  `ExactConsumerShapeBinding`, `FieldConsumerShapeProof`, `exact_consumer_shape_proof`
