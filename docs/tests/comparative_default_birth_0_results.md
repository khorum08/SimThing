# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `42c0ce43c22d` (amended handoff after DA `5154348081` / #1568)
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA ruling (scope): **`5154348081`** — Q1 class_id granted; Q2 triad OUT (narrowed)
- Orchestrator implement: `5154432508`
- **tested_code_sha / implementation:** PR-body-bound only (this file does not self-hash)
- **final_head_sha / clearance_pr_head:** PR-body-bound only
- PR: #1567 draft

## Scope (DA-narrowed)

| In | Out |
|---|---|
| Ordinary-install field-plan product from `GameModeSpec.region_fields` | Defaulting `palma_d_col` / `guyang_value_col` / `guyang_conductance_col` |
| Default emitters: `authored_order` = list position; `class_id = authored_order as f32` | `class_id` from name / column / hash / vec iterate |
| Same-authority sealed adjacency + neighbor rows | Scenario enrollment / parallel install door |
| Consume via 5.8 `admit_comparative_projections` with **explicit triad** | Producer discriminant / operator→role map |

## Implementation

- `crates/simthing-driver/src/comparative_default_birth.rs`
  - `SealedFieldTopology` — no independent adjacency+neighbor mint
  - `FieldPlanAdmissionReport` — topology + emitters (+ diagnostic names)
  - `admit_field_plan_from_region_fields` — ordinary product mint
  - `admit_comparative_from_field_plan` — default emitters + explicit triad
- `compile_and_install` lands product on `SpecSessionState.field_plan_admission` when `region_fields` non-empty

## Focused proof

```text
comparative_default_birth_0: 10 passed; 0 failed
```

1. `ordinary_install_mints_field_plan_from_region_fields`
2. `ordinary_install_without_region_fields_no_invent`
3. `emitter_counts_1_2_3_many_fixed_census`
4. `authored_opt_out_visible`
5. `default_emitters_match_explicit_with_same_triad`
6. `class_id_is_authored_order_as_f32_not_name`
7. `authored_order_invariant_under_registration_vector_reversal`
8. `sealed_topology_rejects_independent_same_length_link_substitution`
9. `link_default_emitters_cpu_oracle_and_gpu`
10. `grid_default_emitter_cpu_oracle_gpu_parity`

## Posture

**PROBATION / proof-present / DA-review-pending.** Draft #1567. No merge. No pointer move.
No 6.1+. No triad default. No Gu-Yang Transient→Matrix lowering.
