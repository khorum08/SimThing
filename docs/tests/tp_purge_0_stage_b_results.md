# TP-PURGE-0 — Stage B results (REPLACE)

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **Stage B complete** (separately relayable)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `0c1168be074cd145233f4ed2b55a9daaa8b5e613`
- Board dispatch: `5133929991`

## Stage B contract executed

Collapse `REPLACE-INLINE-INVARIANT` rows from
`docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv` to
**low-single-digit** inline proofs per (invariant × mechanism), then paired-reap
the old forms. Inputs constructed inline; no corpus/fixture/generator coupling.

| metric | value |
|---|---|
| REPLACE-INLINE targets | **222** |
| Inventory rows reaped (Stage B) | **222** |
| New inline integration proofs | **3** (`invariant_set_inline_0.rs`) |
| Restored conservation unit bites | **3** (`rf_conservation_oracle.rs`) |
| Mechanism coverage | determinism · boundedness/conservation · cpu/gpu pack cardinality (+ live s6 golden as primary parity referee) |

### Replacement suite

- `crates/simthing-driver/tests/invariant_set_inline_0.rs`
  - `determinism_packed_threshold_upload_byte_identical_twice`
  - `boundedness_allocator_residual_beyond_eps_bound_fails`
  - `cpu_gpu_parity_inline_pack_matches_registration_count`
- Unit conservation bites retained/restored in
  `crates/simthing-driver/src/rf_conservation_oracle.rs` with biting `catches:` notes
  under track `0.0.8.7-rf-arena-modernization`.

Tooling: `scripts/ci/tp_purge_stage_a_reap.py --stage-b`
Targets export: `docs/tests/tp_purge_0_stage_b_replace_targets.tsv`
Report: `docs/tests/tp_purge_0_stage_b_reap_report.tsv`

## Stage B gates

| check | result |
|---|---|
| focused inline + conservation unit tests | **PASS** |
| `cargo build --workspace` | **PASS** |
| `test_inventory_drift_check.sh` | **PASS** |

## Fences observed

- No rename-as-replace (replacement count is low-single-digit, not ~222)
- No corpus edit to make proofs pass
- No expiry extension
- No 5.4–5.8 work
