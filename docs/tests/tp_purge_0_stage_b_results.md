# TP-PURGE-0 — Stage B results (REPLACE) — Remand 4 live-mechanism honesty

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **Stage B arithmetic preserved; Remand 4 mechanism honesty repaired** (`5136696481`)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36`
- Board dispatch: `5133929991` · Remand 3: `5136003644` · Continuation: `5136490881` · Remand 4: `5136696481`
- DA rulings: `5135942768`, `5136311181`

## Harnesses (exactly two; ten cases)

| harness | file | green | planted-defect red |
|---|---|---|---|
| `cpu_gpu_parity_matrix` | `crates/simthing-driver/tests/cpu_gpu_parity_matrix_0.rs` | `cpu_gpu_parity_matrix_cases_match` | `cpu_gpu_parity_matrix_planted_defects_fail` |
| `determinism_matrix` | `crates/simthing-driver/tests/determinism_matrix_0.rs` | `determinism_matrix_cases_match` | `determinism_matrix_planted_defects_fail` |

### Case table (`inline_unique_cases = 10`)

**Parity:** `mobility`, `eml-eval`, `accumulator` (subpath table), `rf-need-binding` (zero mapped antecedents; live `need_binding.rs`), `flux-choke`

**Determinism:** `replay` (live `ReplayDriver`), `ordering` (OrderBand + owner-silo presentation-order independence), `canonical-serialization` (inline packet), `mobility-dispatch`, `jit-artifact` (`compile_eml_gadget`)

### Closing arithmetic (unchanged)

| metric | value |
|---|---|
| `INLINE(...)` mapped rows | **145** |
| `REAP-NO-REPLACEMENT(n)` rows | **73** |
| `145 + 73` | **218** |
| Unclassified / `CLASSIFICATION-CONFLICT` | **0** |

Map: `docs/tests/tp_purge_0_stage_b_replacement_map.tsv`

## Remand 4 — per-case honesty report

### `cpu_gpu_parity_matrix`

| case | live path | inline input | green | planted defect | red |
|---|---|---|---|---|---|
| mobility | `cpu_scatter_indexed` / `IndexedScatterOp` | host src + entries | CPU==GPU | swap dst indices on GPU entries | FAIL |
| eml-eval | `eval_eml_cpu` + AO EvalEML tick | literal×slot nodes | CPU==GPU | corrupt GPU scale literal | FAIL |
| accumulator / transfer | `plan_transfer_ops` + `encode_transfer_plan` + AO tick | single-source max_transfer | CPU==GPU | corrupt GPU `scale_a` | FAIL |
| accumulator / emission | `plan_emission_ops` + `encode_emission_plan` + emission readback | Constant emit | reg/count match | corrupt GPU Constant source bits | FAIL |
| accumulator / intent | `PackedIntentUpload` + intent tick | affine delta | CPU==GPU | corrupt GPU add term | FAIL |
| accumulator / velocity | `plan_velocity_integration` + `encode_velocity_into` | amount/vel pair | CPU==GPU | wrong GPU dt | FAIL |
| accumulator / weighted-mean | WeightedMean SlotRange AO | 2 children + weights | CPU==GPU | corrupt GPU weight_col | FAIL |
| accumulator / owner-silo | ConjunctiveCrossing Sum (owner-silo GPU shape) | 2 participants → aggregate | CPU==GPU | drop participant on GPU | FAIL |
| accumulator / bh2 | `WImpedanceComposeOp` / `cpu_w_impedance_compose_oracle` | 2×2 W field | CPU==GPU | corrupt GPU `weight_a` | FAIL |
| rf-need-binding | `build_need_binding_ops` + EvalEML | staged cells + MUL tree | CPU==GPU | rewrite MUL→ADD on GPU nodes | FAIL |
| flux-choke | `StructuredFieldStencilOp` SaturatingFlux | seeded field | CPU==GPU | corrupt `u_sat` | FAIL |

### `determinism_matrix`

| case | live path | inline input | green | planted defect | red |
|---|---|---|---|---|---|
| replay | `ReplayDriver::from_snapshot` + `apply_frame` | OverlayAttached then Suspended | two applies identical lifecycle | reverse entry application order | FAIL |
| ordering / overlay | `plan_overlay_orderband` | two deltas | ops identical twice | swap delta order | FAIL |
| ordering / owner-silo | `apply_owner_silo_runtime_disburse_down_cpu` | equal-claim demands | orig **and reversed** presentation → same canonical alloc | presentation-order allocator | FAIL |
| canonical-serialization | mobility scenario0 RON ser→de→reser | inline packet (no factory) | digest stable | reverse `owner_columns` before serialize | FAIL |
| mobility-dispatch | `plan_mobility_alloc0` | two arrivals | assignment order stable | entity-id-desc dispatch ignores arrival_order | FAIL |
| jit-artifact | `compile_eml_gadget` SoftStep | SoftStep instance | nodes identical twice | reverse compiled node order | FAIL |

## Fences observed

- No eleventh case; no nine-case reduction; no reap-rule change
- No corpus/fixture/generator prerequisite for harness cases
- Stage A and Stage C not reopened
- No merge / pointer / expiry / 5.4–5.8 work
