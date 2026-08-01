# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `b17e36045daf`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA seam ruling: Board `5153818317`
- Orchestrator remand: Board `5153845512`
- Adapter: `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan`

## Seams (DA-dissolved S1/S2; granted S3)

| Seam | Implementation |
|---|---|
| **S1** LinkGraph transport | Neighbor rows captured at the same site as `FieldAdjacency::link_graph(...)` (5.8 `neighbor_slots` seam). Grid uses public offsets helper. No LinkGraph accessor, no reconstruction, no kernel door. |
| **S2** Role identity | Read public `FieldSweepRegistration::output() == Matrix(col)`. Emitters = remaining registrations in **authored order**. `class_id` = admitted column identity (`col.raw_u32()+1`). No MIN/MUL/conservative heuristics. |
| **S3** Ordinary install product | `FieldPlanAdmissionReport` delivered on `SpecSessionState` by ordinary `compile_and_install` when `Scenario.field_plan_admission` carries the already-admitted product (like `property_admission` delivery). **No** `compile_and_install_with_field_plan` side-door. Default birth invokes 5.8 `admit_comparative_projections`. |

## Focused proof

```text
comparative_default_birth_0: 10 passed; 0 failed
COMPARATIVE-DEFAULT-BIRTH link adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
```

1. `ordinary_install_default_births_two_emitters`
2. `emitter_counts_1_2_3_many_fixed_census`
3. `authored_opt_out_visible`
4. `default_matches_explicit_grid_bit_for_bit`
5. `authored_order_invariant_under_registration_vector_reversal`
6. `planted_grid_topology_substitute_rejected`
7. `planted_link_neighbor_substitute_rejected_by_length_and_identity`
8. `link_default_matches_explicit_and_gpu`
9. `install_without_field_plan_no_invent`
10. `adjacency_mismatch_on_emitter_reg_fails_closed`

## Fences held

No kernel/allowlist widen. No second topology authority. No role enum/string namespace. 5.8 math untouched. TP void. No 6.1+ / pointer move / `/clearance`.

## Posture

Return **PROBATION / proof-present / DA-review-pending**.
