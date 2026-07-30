# TP-PURGE-0 — Stage B results (REPLACE) — Remand 2 STOP

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **REMANDED / CLASSIFICATION-CONFLICT STOP** (Remand `5135691949`)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36`
- Board dispatch: `5133929991` · Remand 1: `5134500978` · Remand 2: `5135691949`

## Remand 2 finding (accepted)

The Remand-1 replacement map was complete by row count but false by mechanism:
cross-mechanism relabeling mapped distinct GPU / replay / Studio / map-gen / typeface
defects onto `s6`, threshold-upload packing, or ColumnIndex admission survivors that do
not bite those substrates.

## Stage B contract re-executed (mechanism-honest)

Rebuild from production substrate mechanisms. For each of the 222
`REPLACE-INLINE-INVARIANT` rows:

`old identity → invariant → mechanism → disposition → survivor (if any) → planted defect`

| metric | value |
|---|---|
| REPLACE-INLINE targets | **222** |
| Honest `SURVIVOR` mappings | **4** |
| `CLASSIFICATION-CONFLICT` (STOP) | **218** |
| Distinct invariant × mechanism families | **37** |
| Families with honest inline survivor | **3** (composite-conservation, allocator-conservation, column-index-residency) |

Map: `docs/tests/tp_purge_0_stage_b_replacement_map.tsv`

### Honest survivors retained (mechanism bite only)

| invariant × mechanism | survivor | planted defect |
|---|---|---|
| conservation × composite-conservation | `rf_conservation_oracle::composite_bite_nonconservative_fails_conservative_passes` | non-conserving composite RF step fails |
| conservation × allocator-conservation | `rf_conservation_oracle::allocator_broken_disburse_exceeding_eps_bound_fails` | disburse beyond O(eps·n) fails |
| residency-typing × column-index-residency | `column_index::gpu_round_trip_door_preserves_column_bits` | round-trip bit drop fails |

### Survivors present but **not** claimed over the 222-row set

These remain valid for their own mechanisms; Remand 2 forbids using them as catch-alls:

- `s6_threshold_events_match_cpu_golden` — threshold-event CPU/GPU parity only (no pure s6 row among the 222)
- `determinism_packed_threshold_upload_byte_identical_twice` — threshold-upload POD packing only
- `pack_cardinality_distinguishes_registration_count` — threshold-upload cardinality only (renamed; former `cpu_gpu_parity_inline_pack_matches_registration_count` must not appear as a parity claim)
- `boundedness_allocator_residual_beyond_eps_bound_fails` — allocator residual bound only (studio UI clamps are a different mechanism)
- `authored_admit_door_rejects_out_of_range_and_preserves_in_range` — ColumnIndex admission only (Studio typed observation is a different mechanism)

### Top CONFLICT families (need DA / genuine inline families)

| n | invariant | mechanism |
|---|---|---|
| 34 | determinism | replay-determinism |
| 30 | cpu-gpu-parity | mobility-gpu |
| 27 | cpu-gpu-parity | eml-intensity |
| 22 | determinism | mobility-dispatch-determinism |
| 11 | determinism | map-generation-determinism |
| 10 | determinism | canonical-serialization |
| 9 | cpu-gpu-parity | atlas-gpu-parity |
| 8 | residency-typing | studio-typed-observation |
| … | … | (full census in map TSV) |

Per handoff stop condition and Remand 2 §7: do **not** invent mappings, restore
corpus-coupled referees, or silently drop rows. **STOP for orchestration/DA.**

## Fences observed

- No rename-as-replace
- No corpus edit to make proofs pass (combat property removal is Stage C corpus-block delete)
- No expiry extension
- No 5.4–5.8 work
