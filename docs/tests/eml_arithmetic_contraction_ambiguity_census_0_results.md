# EML-ARITHMETIC-SEMANTICS-0 — contraction uniqueness census (DISCHARGED)

**Status:** STOP discharged by DA uniqueness ruling `5192270934` / RESUME `5192307920`.
Two-or-more-MUL→one-**ADD** shapes are lawfully **UNFUSED (`U`)** — no tie-break; proceed.
DA exit-proof `5193244394` / remand `5193312235`: MUL→SUB dataflows measured **NON-ZERO (4)**;
uniqueness extends to **`ADD` or `SUB`** (all four edges are unique → **FUSED** / fms).

| Field | Value |
| --- | --- |
| rung | `EML-ARITHMETIC-SEMANTICS-0` (5.14) |
| HD-RECEIPT | `b9070974440b` |
| ORIENT-RECEIPT | `bcef56bc23ce` |
| dispatch | `5191921308` → STOP `5192130398` → DA `5192270934` → RESUME `5192307920` → REMAND `5192641222` → EXIT `5193244394` → SUB service `5193312235` |
| base_sha | `98180a4a4e7334fa9476c74170d995b5028202dc` |
| harness | `cargo test -p simthing-workshop --test eml_arithmetic_contraction_ambiguity_census_0 -- --nocapture` |

## Uniqueness rule (binding — ADD or SUB)

- Exactly one `MUL` → consuming `ADD` or `SUB` → **FUSED** (one rounding; `SUB` = fms).
- Two or more `MUL`s → same `ADD`/`SUB` → **UNFUSED (`U`)**.
- No tie-break, ever. SEAM LAW is an instance at map/fold, not a peer law.

## Census summary

- Programs walked: **17**
- ADD hits (two MUL → one ADD): **5** — all measured **matches-U** on CPU + interpreted WGSL
- MUL→SUB dataflows: **4** — all unique (one MUL → that SUB) → **FUSED** under the generalized rule
- SSA-JIT: not-an-execution-arm for OrdinaryAccumulatorEvalEml hits

## MUL→SUB edges (all unique → FUSED)

1. logistic-steering — SUB@9 ← MUL@7
2. logistic-steering — SUB@15 ← MUL@14
3. logistic-steering — SUB@22 ← MUL@20
4. intensity-behavior — SUB@18 ← MUL@17

## Hits (all lawfully U)

1. WeightedAccumulator-n2 — ADD@6 ← MUL@2, MUL@5 — U on cpu/interpreted
2. WeightedAccumulator-n3 — ADD@9 ← MUL@5, MUL@8 — U
3. Ema — ADD@6 ← MUL@2, MUL@5 — U
4. BoundedFeedback — ADD@6 ← MUL@2, MUL@5 — U
5. need-binding-weighted-n2 — ADD@6 ← MUL@2, MUL@5 — U

Arms must **prove** they lower to authored meanings (U or FUSED), not rely on interpreter accident.
