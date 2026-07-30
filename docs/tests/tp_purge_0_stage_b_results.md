# TP-PURGE-0 — Stage B results (REPLACE) — Remand 3 closed

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **Stage B arithmetic CLOSED** (continuation `5136490881`; DA remainder `5136311181`)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36`
- Board dispatch: `5133929991` · Remand 1: `5134500978` · Remand 2: `5135691949` · Remand 3: `5136003644` · Continuation: `5136490881`
- DA rulings: `5135942768`, `5136311181`

## Harnesses (exactly two; ten cases)

| harness | file | green | planted-defect red |
|---|---|---|---|
| `cpu_gpu_parity_matrix` | `crates/simthing-driver/tests/cpu_gpu_parity_matrix_0.rs` | `cpu_gpu_parity_matrix_cases_match` | `cpu_gpu_parity_matrix_planted_defects_fail` |
| `determinism_matrix` | `crates/simthing-driver/tests/determinism_matrix_0.rs` | `determinism_matrix_cases_match` | `determinism_matrix_planted_defects_fail` |

### Case table (`inline_unique_cases = 10`)

**Parity:** `mobility`, `eml-eval`, `accumulator`, `rf-need-binding` (zero mapped antecedents; live `need_binding.rs` path), `flux-choke`

**Determinism:** `replay`, `ordering` (overlay OrderBand + owner-silo equal-claim tie-break path), `canonical-serialization`, `mobility-dispatch`, `jit-artifact`

### Closing arithmetic

| metric | value |
|---|---|
| `INLINE(...)` mapped rows | **145** |
| `REAP-NO-REPLACEMENT(n)` rows | **73** |
| `145 + 73` | **218** |
| Unclassified / `CLASSIFICATION-CONFLICT` | **0** |
| Honest `SURVIVOR` (outside 218) | **4** |

### REAP-NO-REPLACEMENT by rule

| rule | rows |
|---|---|
| (1) non-engine crate | 49 |
| (2) peripheral/presentation | 14 |
| (3) authoring-layer | 5 |
| (4) not proof | 3 |
| (5) already covered | 1 |
| (6) superseded | 1 |
| **total** | **73** |

Map: `docs/tests/tp_purge_0_stage_b_replacement_map.tsv`

## Fences observed

- No eleventh case; no nine-case reduction; no reap-rule change
- No corpus/fixture/generator prerequisite for harness cases
- Stage A and Stage C not reopened
- No merge / pointer / expiry / 5.4–5.8 work
