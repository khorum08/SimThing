# FIELD-ADJACENCY-GENERATORS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.6)
- Status: **PROBATION / proof-present / DA-review-pending**
- ORIENT-RECEIPT: `de6ca1a521b4` (fresh after anchor resync)
- HD-RECEIPT: `cdb153dca91e`
- Canonical handoff: `handoffs/FIELD-ADJACENCY-GENERATORS-0.hd.md`
- Remand authority: Board comments `5143977793` and resume `5144176590`
- Expected route: `DA-RESERVE(gate-wiring)`
- Pointer: no pointer advancement or graduation authority is claimed.

The exact reviewed head and hosted run identities are bound by the PR and Board relay because this
file cannot self-hash.

## Contract discharge

| Requirement | Result |
|---|---|
| Weighted GridOffsets | PASS — N4, explicitly weighted N8 diagonals, arbitrary authored offsets, and Chebyshev-radius shell weights lower to canonical input-list rows |
| LinkGraph | PASS — rows reuse the existing scenario-link compiler's sorted, deduplicated, undirected projection; asymmetric, duplicate, unsorted, self, and out-of-range rows reject |
| Conductance proof | PASS — conservative registration requires an exact-adjacency certificate proving `chi_i * sum_j(abs(c_ij)) <= bound` independently for every node |
| SaturatingFlux isolation | PASS — conductance writes a sealed kernel-private transient lane; compatible reads require a producer certificate and initialized shared session; no unrelated matrix column is borrowed |
| Scheduling separation | PASS — degree-homogeneous buckets contain only target-slot schedules; each row's authored gather/fold order remains immutable and fingerprint-bound |
| Execution parity | PASS — N4, N8, radius-r, and LinkGraph are full-buffer bit-exact across natural-order CPU, bucketed CPU, and adapter-pinned GPU execution |
| Sparse scale separation | PASS — the single LinkGraph parity fixture has 1,025 slots, above `REGION_FIELD_MAX_CELL_COUNT`, with no dense-cap reach in adjacency admission |
| Emergence falsifier | PASS — identical authored map/fold/post EML yields an N4 diamond, weighted-N8 octagonal front, and remote topology-following LinkGraph front; a planted N8→N4 alias flips the verdict red |
| Production lowering | PASS — mapping-plan and first-slice field execution use generic registrations/sessions; PALMA, Gu-Yang, structured fields, and W composition lower through the generic executor |
| Resident execution | PASS — compatible registration chains retain transient state and values inside one resident GPU session without host readback between passes |
| Legacy callers | PASS — permanent `FIELD-SWEEP-LEGACY-CALLERS` census finds zero compiled production reaches to the seven legacy operators; its planted production caller flips red |
| Dense-cap ratchet | PASS — permanent `FIELD-SWEEP-DENSE-CAP-CROSSING` census excludes all three `REGION_FIELD_*` dense caps from the generic/sparse path; its planted crossing flips red |
| Shader path | PASS — no new field-law WGSL was added; the kernel-owned generic field interpreter remains the production shader |
| Corpus Boundary | PASS — canonical LinkGraph compiler proof uses a minimal inline synthetic scenario; no game-corpus prerequisite remains |
| Typed negatives | PASS — invalid weights, non-canonical/asymmetric LinkGraph rows, over-bound conductance, certificate/adjacency mismatch, missing transient proof, and uninitialized transient reads reject |
| STEAD convergence | PASS — contract §10 binds weighted grid/LinkGraph adjacency, per-node conductance admission, and the no-borrow transient rule |
| Test lifecycle | seven new tests: five `AUDIT`/ledger-only falsifiers and two TIER7 oracle-parity `KEEP` rows |

## Focused proof

```text
field_adjacency_generators_0: 7 passed; 0 failed
field_sweep_n4_parity_0: 3 passed; 0 failed
stead_spatial_contract_guards: 3 passed; 0 failed
terran_pirate_mapping_atlas_scheduler: 1 passed; 0 failed
N4/N8/radius-r/LinkGraph-1025: full-buffer bit-exact CPU/GPU parity on Vulkan
PALMA + Gu-Yang generic production lowerings: full-buffer bit-exact CPU/GPU parity on Vulkan
FIELD-SWEEP-LEGACY-CALLERS planted violation: PASS (gate flipped red)
FIELD-SWEEP-DENSE-CAP-CROSSING planted violation: PASS (gate flipped red)
detachability: PASS (production_coupling=0, proof_coupling=0)
test_inventory_drift_check: PASS (1024 rows, 1024 discovered, 0 unledgered)
```

The workspace-wide `cargo check --workspace --all-targets` remains red only in untouched
ClauseThing test targets (`ct_2a_intrinsic_flow`, `ct_2c_category_economy`, and
`specialization_protocol_0`) due to pre-existing `ColumnIndex` migration and
`PropertySpec::admission_disposition` drift. The core/kernel/GPU/driver package suites and all
rung-owned focused targets above pass.

## Review posture

The change expands sealed field registration and permanent gate wiring, so DA review remains
required. Legacy implementations remain bounded migration oracles; their deletion is reserved to the
authorized removal rung. No resource/performance verdict or later-rung work is claimed.
