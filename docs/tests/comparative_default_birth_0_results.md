# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `b17e36045daf`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA seam authorization: Board `5151136145`
- Pointer advance: Board `5153399789`
- Adapter: `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan`

## What landed

| Seam | Implementation |
|---|---|
| A — topology transport | `AdmittedFieldPlanBinding` carries exact `FieldAdjacency` + sealed neighbor rows + field-sweep registrations into `SpecSessionState.admitted_field_plan`. Install: `compile_and_install_with_field_plan`. No `sqrt(n_slots)`, no second authority. |
| B — role derivation | From registration structure only: conservative matrix → Gu-Yang U; non-conservative MUL fold → Gu-Yang C; non-conservative MIN fold → PALMA D; residual matrix outputs in **authored registration order** → emitters. Ambiguous/missing → fail closed. |
| Default birth | Invokes existing 5.8 `admit_comparative_projections` — settled math unchanged. |

## Focused proof

```text
comparative_default_birth_0: 6 passed; 0 failed
COMPARATIVE-DEFAULT-BIRTH link adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
```

1. `install_carries_admitted_field_plan_and_default_births`
2. `default_matches_explicit_5_8_on_dual_emitter_front`
3. `planted_topology_substitute_rejected`
4. `one_emitter_insufficient`
5. `install_without_field_plan_no_invent`
6. `link_graph_default_birth_oracle_and_gpu`

## Fences held

No string namespace / property-name grammar / role enum. No TP/corpus. No kernel/GPU public door widen (only proof accessors `FieldLawProof::is_conservative`, `FieldAdjacency::order_fingerprint`, `field_law_proof()`). No 6.1+. No pointer move by coder.

## Posture

Return **PROBATION**. Coding does not invoke `/clearance` or advance the pointer.
