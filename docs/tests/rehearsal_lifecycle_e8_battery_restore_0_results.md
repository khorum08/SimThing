# E8 qualification battery restored — correction of DA ruling 5723973467

DA correction carried on top of retention payer #2073 (relay `5725356465`). **The defect
originated in the DA ruling, not in the payer's execution** — Astra carried out ruling
`5723973467` faithfully.

## The defect

Ruling `5723973467` ordered "318 Rust proofs — REAP", reasoning that they had been retained
only as "potentially needed" for §5.3 and that #2062 consumed none of them. That test was
incomplete. It asked whether *§5.3* consumed each proof, not whether *anything* did. Eight of
the 318 are the **standing E8 resident-clearing seal qualification battery**, consumed by the
seal law on every pin roll — rolls 9 through 14 in this track alone:

| Battery | Tests | Home |
|---|---|---|
| parity + mutant matrix | 1 | `simthing-workshop/tests/resident_clearing_parity_0.rs` |
| score-and-bands | 3 | `simthing-workshop/tests/resident_clearing_score_and_bands_0.rs` |
| runtime qualification | 4 | `simthing-gpu/src/resident_clearing_runtime.rs` |

The payer reaped all eight. That removed `QUALIFIED_RECORD_FINGERPRINT` — the independent
half of the seal law's two-literal atomic cross-check — and left the production literal
with no referee. It is also why the payer could offer only "ingress 1/0/0/0" for its
fourteenth roll: the standing battery no longer existed to run.

They had sat in the pen since the 2026-09-10 ceremony disposition. Every roll this track
was a true consumption that should have renewed them with a structured `downstream-utility`
line; none was renewed. That lifecycle lapse is also the DA's.

## The correction

- restored `resident_clearing_parity_0.rs` and `resident_clearing_score_and_bands_0.rs`
  byte-identical from `9b6385ad`;
- restored the four runtime qualification tests in `resident_clearing_runtime.rs` —
  proven the file's only production delta versus the payer is the pin literal, so restoring
  from base and re-rolling the literal recovers exactly the four tests and nothing else;
- rolled the independent `QUALIFIED_RECORD_FINGERPRINT` to the payer's fourteenth pin, so
  **both literals now read `0xee0e_9ac0_af83_0bdf`** — the two-literal law restored;
- ledgered all eight as active `invariant-required` rows, each with its original birth
  provenance preserved (8/8) and the E8 seal law named as its standing consumer.

None of the three files is a sealed component, so restoring them moves no bundle byte and
the fourteenth pin stands.

## Fourteenth-roll qualification — the standing battery, now actually run

- parity: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ee0e9ac0af830bdf`, 1 passed
  (fingerprint equality + full mutant matrix) — independently confirming the payer's
  observed value;
- score-and-bands: 3 passed;
- runtime: 4 passed.

## Scope held

The remaining seal/parity-named reaps (`accumulator_convergence_seal_0`,
`cpu_gpu_parity_matrix_0`, `field_sweep_n4_parity_0`, `gpu_overlay_lifecycle_oracle_parity_0`,
the EML primitive qualification tests, `eml_resource_class_jit_parity_0`) **stay reaped**.
No recurring standing gate consumes them; restoring them would contradict the Owner's
explicit 2026-09-10 instruction to reap ceremony rather than risk zombie fixtures. Only the
battery a standing law invokes on every roll is restored.

Gates on the corrected tree: rungclose PASS (stamped + pointer-advanced + orientation-fresh +
reap-clear), artifact-expiry PASS 0/0/0, drift PASS unledgered=0, pen empty.
