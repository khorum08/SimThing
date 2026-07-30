# TP-PURGE-0 — Stage B results (REPLACE) — Remand 5 same-input mutants

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **Stage B arithmetic preserved; Remand 5 same-semantic-input mutants repaired** (`5137044659`)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36`
- Board dispatch: `5133929991` · Remand 3: `5136003644` · Continuation: `5136490881` · Remand 4: `5136696481` · Remand 5: `5137044659`
- DA rulings: `5135942768`, `5136311181`

## Harnesses (exactly two; ten cases)

| harness | file | green | planted-defect red |
|---|---|---|---|
| `cpu_gpu_parity_matrix` | `crates/simthing-driver/tests/cpu_gpu_parity_matrix_0.rs` | `cpu_gpu_parity_matrix_cases_match` | `cpu_gpu_parity_matrix_planted_defects_fail` |
| `determinism_matrix` | `crates/simthing-driver/tests/determinism_matrix_0.rs` | `determinism_matrix_cases_match` | `determinism_matrix_planted_defects_fail` |

### Case table (`inline_unique_cases = 10`)

**Parity:** `mobility`, `eml-eval`, `accumulator` (subpath table), `rf-need-binding` (zero mapped antecedents; live `need_binding.rs`), `flux-choke`

**Determinism:** `replay` (live `ReplayDriver` vs mutant reversed-application executor), `ordering` (OrderBand presentation-order contract + owner-silo presentation-order independence), `canonical-serialization` (inline packet; live pretty vs mutant compact), `mobility-dispatch`, `jit-artifact` (live vs mutant reverse-encode compiler)

### Closing arithmetic (unchanged)

| metric | value |
|---|---|
| `INLINE(...)` mapped rows | **145** |
| `REAP-NO-REPLACEMENT(n)` rows | **73** |
| `145 + 73` | **218** |
| Unclassified / `CLASSIFICATION-CONFLICT` | **0** |

Map: `docs/tests/tp_purge_0_stage_b_replacement_map.tsv`

## Remand 4 — preserved parity honesty (`cpu_gpu_parity_matrix`)

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

## Remand 5 — same-semantic-input determinism mutants

Contract for every red: green and red receive the **same** semantic input; defect is a **test-local mutant mechanism**, not changed input or post-return artifact mutation.

| case | unchanged semantic input | live green | mutant mechanism | demonstrated red |
|---|---|---|---|---|
| replay | same `ReplaySnapshot` + `ReplayFrame` (OverlayAttached then Suspended) | `live_replay_apply` twice → identical lifecycle fingerprint | `mutant_replay_apply_reversed_entries` misapplies unchanged frame in reverse entry order | FAIL (lifecycle diverges) |
| ordering / overlay | same deltas `[2.0, 1.0]` (presentation order meaningful) | `plan_overlay_orderband` twice → identical ops/bands | `mutant_plan_overlay_sort_by_value` sorts by value then bands | FAIL (ops diverge) |
| ordering / owner-silo | equal-claim demands | live disburse on orig **and reversed** presentation → same canonical alloc | presentation-order defective allocator | FAIL (alloc order diverges under reverse) |
| canonical-serialization | same inline `MobilityScenario0Packet` | live pretty RON twice + ser→de→reser digest stable | `mutant_serialize_mobility_scenario0_compact` (compact RON) | FAIL (digest diverges) |
| mobility-dispatch | same inline alloc input | `plan_mobility_alloc0` twice | entity-id-desc dispatch ignores arrival_order | FAIL |
| jit-artifact | same SoftStep instance + opts | `compile_eml_gadget` twice | `mutant_compile_eml_gadget_reverse_encode` seals reverse node order | FAIL (nodes diverge) |

## Fences observed

- No eleventh case; no nine-case reduction; no reap-rule change
- No corpus/fixture/generator prerequisite for harness cases
- Stage A and Stage C not reopened; Remand 4 parity work not reopened
- No merge / pointer / expiry / 5.4–5.8 work
