# EML-ARITHMETIC-SEMANTICS-0 — contraction uniqueness census (DISCHARGED)

**Status:** STOP discharged by DA uniqueness ruling `5192270934` / RESUME `5192307920`.
Two-or-more-MUL→one-ADD/SUB shapes are lawfully **UNFUSED (`U`)** — no tie-break; proceed.

| Field | Value |
| --- | --- |
| rung | `EML-ARITHMETIC-SEMANTICS-0` (5.14) |
| HD-RECEIPT | `b9070974440b` |
| ORIENT-RECEIPT | `fc5773df281f` |
| dispatch | `5191921308` → STOP `5192130398` → DA `5192270934` → RESUME `5192307920` |
| DA orders | `5191578752`, amendments `5191860082`, uniqueness `5192270934` |
| base_sha | `98180a4a4e7334fa9476c74170d995b5028202dc` |
| harness | `cargo test -p simthing-workshop --test eml_arithmetic_contraction_ambiguity_census_0 -- --nocapture` |

## Uniqueness rule (binding)

- Exactly one `MUL` → consuming `ADD`/`SUB` → **FUSED** (one rounding).
- Two or more `MUL`s → same `ADD`/`SUB` → **UNFUSED (`U`)**.
- No tie-break, ever. SEAM LAW is an instance at map/fold, not a peer law.

## Census summary

- Programs walked: **17**
- ADD hits (two MUL → one ADD): **5** — all measured **matches-U** on CPU + interpreted WGSL
- SUB hits (two MUL → one SUB): **0** (recorded for the ruling; uniqueness already decides)
- SSA-JIT: not-an-execution-arm for OrdinaryAccumulatorEvalEml hits

## Hits (all lawfully U)

1. WeightedAccumulator-n2 — ADD@6 ← MUL@2, MUL@5 — U on cpu/interpreted
2. WeightedAccumulator-n3 — ADD@9 ← MUL@5, MUL@8 — U
3. Ema — ADD@6 ← MUL@2, MUL@5 — U
4. BoundedFeedback — ADD@6 ← MUL@2, MUL@5 — U
5. need-binding-weighted-n2 — ADD@6 ← MUL@2, MUL@5 — U

Arms must **prove** they lower to `U` (authored meaning), not rely on interpreter accident.
