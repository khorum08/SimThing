# FIELD-ADJACENCY-GENERATORS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.6)
- Status: **PROBATION / proof-present / DA-review-pending**
- ORIENT-RECEIPT: `01a351001f4c`
- Handoff: Board comment `5143237424`
- Expected route: `DA-RESERVE(gate-wiring)`
- Pointer: no pointer advancement or graduation authority is claimed.

The exact reviewed head is bound by the PR and Board relay because this file cannot self-hash.

## Contract discharge

| Requirement | Result |
|---|---|
| Weighted GridOffsets | PASS — N4, explicitly weighted N8 diagonals, arbitrary authored offsets, and Chebyshev-radius shell weights lower to canonical input-list rows |
| LinkGraph | PASS — rows reuse the existing scenario-link compiler's sorted, deduplicated, undirected projection; asymmetric, duplicate, unsorted, self, and out-of-range rows reject |
| Conductance proof | PASS — conservative registration requires an exact-adjacency certificate proving `chi_i * sum_j(abs(c_ij)) <= bound` independently for every node |
| Scheduling separation | PASS — degree-homogeneous buckets contain only target-slot schedules; each row's authored gather/fold order remains immutable and fingerprint-bound |
| Emergence falsifier | PASS — identical authored map/fold/post EML yields an N4 diamond, weighted-N8 octagonal front, and remote topology-following LinkGraph front |
| Production lowering | PASS — mapping-plan and first-slice field execution use generic registrations/sessions; PALMA, Gu-Yang, structured fields, and W composition lower through the generic executor |
| Resident execution | PASS — chained registrations import/export resident GPU buffers without host readback between field passes |
| Legacy callers | PASS — compiled production roots contain zero constructor/dispatch callers of the seven legacy field operators; retained definitions and calls are migration referees or uncompiled fixture/orphan modules |
| Shader path | PASS — no new field-law WGSL was added; the kernel-owned generic field interpreter remains the production execution shader |
| Typed negatives | PASS — invalid weights, non-canonical/asymmetric LinkGraph rows, over-bound conductance, and certificate/adjacency mismatch reject before dispatch |
| STEAD convergence | PASS — contract §10 binds authored weighted grid and LinkGraph adjacency plus per-node conductance admission |
| Test lifecycle | six new tests: five `AUDIT`/ledger-only falsifiers and one TIER7 W-compose oracle-parity `KEEP` |

## Focused proof

```text
field_adjacency_generators_0: 6 passed; 0 failed
field_sweep_n4_parity_0: 3 passed; 0 failed
stead_spatial_contract_guards: 3 passed; 0 failed
terran_pirate_mapping_atlas_scheduler: 1 passed; 0 failed
PALMA + Gu-Yang generic production lowerings: bit-exact CPU/GPU parity on Vulkan
```

## Review posture

The change expands the sealed field-registration surface and production GPU routing, so DA review
remains required. Legacy operator implementations stay available only as bounded migration oracles;
their retirement remains governed by the later legacy-removal rung.
