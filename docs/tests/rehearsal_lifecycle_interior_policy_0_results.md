# rehearsal_lifecycle_interior_policy_0 — INTERIOR-POLICY COMPOSITION LAW (DA admission + seventh E8 roll)

**Status: COMPLETE (DA capability increment, rung `0088-INGRESS-FIDELITY-0`).**
Orchestrator reserve relay `5625360554`; Astra first consumer return `5620986815`
(retained RED: beta policy 3→7 leaving beta born allocation `21.692308` unchanged while the
beta_3 flow discriminator 8→20 correctly moved born allocation `5.448718→17.448719`).

## The ruled law

For an interior arena participant, the recursive branch-pressure upsweep publishes the
direct-child Sum to the node's own child-share denominator (`weight_sum_col`) ALWAYS, and to
the node's upward participation weight (`weight_col`) ONLY when the node carries no authored
AllocatorWeight program. A node with an authored program KEEPS that authored value as its
participation identity at its parent's disbursement — the constitutional per-level
reduce → apply-policy → disburse order. Silent installation followed by erasure (the
confirmed defect: `sum_reduction_to_targets_ops` double-targeting `weight_col` +
`weight_sum_col` with `ResetTarget`) is outlawed. One canonical AllocatorWeight authority
(the resolved need-binding program set) — no parallel policy lane, no caller replay, no
per-leaf copying, no post-RF patch. Neutral trees degenerate to the historical plan
bit-for-bit. Deform-with-aggregate composition (price × pressure) remains expressible as a
FUTURE authored-vocabulary extension (a program reading its own aggregated cell); it is not
minted here.

## Witnesses (this increment)

- `arena_allocation_plan.rs::interior_authored_weight_survives_the_pressure_upsweep` — a
  policy-bearing interior's aggregate targets ONLY `weight_sum_col`; no plan op writes its
  `weight_col`. PASS.
- `arena_allocation_plan.rs::neutral_interior_pressure_carrier_is_bit_identical_to_history`
  — empty authored-weight set reproduces the historical op stream exactly. PASS.
- Existing `sparse_child_rows_compile_to_one_ordered_input_list_writer` (historical
  double-target shape on the neutral path) — PASS unchanged.
- Application first consumer: Astra's retained beta-policy discriminator (draft #2034) goes
  GREEN on rebase; that RED/GREEN pair is the ordinary-session witness and stays in the 1.1
  evidence set.

## Seventh E8 roll (first of 0.0.8.8)

The sealed semantic bundle includes `crates/simthing-driver/src/arena_allocation_plan.rs`
and `crates/simthing-driver/src/arena_allocation_sync.rs` (build.rs COMPONENTS); this
increment's semantic edits therefore roll the seal under the standing clean-checkout law.

- **Old pin refusal (RED, required first):**
  `resident_clearing_parity_terminal_referee` FAILED with observed record fingerprint
  `0x42a7_338e_0b42_b5be` vs pinned `0xb295_851d_f402_d50b` — the seal refusing the amended
  bundle before economics, as designed.
- **Roll:** exactly the two existing literals, identical observed value:
  `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` (crates/simthing-gpu/src/resident_clearing_runtime.rs)
  and `QUALIFIED_RECORD_FINGERPRINT` (crates/simthing-workshop/tests/resident_clearing_parity_0.rs)
  → `0x42a7_338e_0b42_b5be`. No third site, no comparator/ABI/component-list/build change.
- **Required tuple:** `simthing-gpu/eml-resource-profiling` (the workshop crate pins the
  feature; commands below are the explicit documented forms), reference machine, release
  test profile.
- **Post-roll qualification (all local, this machine's GPU):**
  - `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
    → `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 42a7338e0b42b5be`; 1 passed
    (fingerprint equality + full mutant matrix inside the referee).
  - `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_score_and_bands_0 -- --nocapture --test-threads=1` → 3 passed.
  - `cargo test -p simthing-gpu --features eml-resource-profiling resident_clearing_runtime -- --nocapture --test-threads=1` → 4 passed.
- **Clean-checkout reproducibility:** recorded below after the roll commit exists.

## Clean-checkout proof

- commit: `a5864f6b94dbaf07a9edd125c18cf6e1c08503f1`
- command: fresh `git clone` (canonical LF checkout) at that exact commit;
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 42a7338e0b42b5be`; referee
  `1 passed; 0 failed` — pinned fingerprint reproduced and the qualified witness admitted
  from the fresh canonical-LF clone (sibling directory, never %TEMP%).
