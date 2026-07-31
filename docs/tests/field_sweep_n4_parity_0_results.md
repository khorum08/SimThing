# FIELD-SWEEP-N4-PARITY-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `2539bdaf8933`
- ORIENT-RECEIPT: `ea874cae36fb`
- expected route: `DA-RESERVE(gate-wiring)`
- Pointer: remains `FIELD-SWEEP-N4-PARITY-0`; no graduation or 5.6 authority is claimed.

The tested exact-head SHA is bound in the PR and board relay because this result file cannot
self-hash.

## Contract discharge

| Requirement | Result |
|---|---|
| One production field-sweep path | PASS — one kernel-owned WGSL interpreter over the existing `AccumulatorOp` input-list gather |
| Algebra as data | PASS — authored EML map / fixed-linear-fold / post programs; no field/algebra enum, tag, or operator-identity dispatch |
| Edge context | PASS — target slot, neighbor slot, accumulator, edge scalar, and dt; `TARGET_VALUE` / `NEIGHBOR_VALUE` are field-context-only |
| Proof-present registration | PASS — canonical-order proof required; conservative folds additionally require adjacency-bound undirected-symmetry certificate |
| Resource class | PASS — admission accepts only the legacy fixed `32` stack / `32` program-node class until rung 5.7 |
| PALMA | PASS — authored WENS N4 min-plus instance with destination pinning authored in EML |
| Gu-Yang | PASS — authored NSEW N4 conductance + conservative flux registrations |
| Parity-alongside | PASS — PALMA and Gu-Yang are bit-exact on CPU and GPU against the existing unmodified legacy paths |
| Adapter evidence | `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan` |
| Typed negatives | PASS — missing law, missing canonical order, mismatched symmetry certificate, nondefault resource class, and malformed field context reject before dispatch |
| STEAD convergence | PASS — contract §10 and `stead_spatial_contract_guards` bind both authored N4 instances to the generic registration |
| Permanent single-path tripwire | PASS — algebra/tag/operator scan plus retiring seven-shader allowlist; planted enum and eighth-shader self-tests fire |
| Legacy migration oracles | PRESERVED — seven shaders catalogued with per-item rationale and rung-10.1 promotion blocker |
| Corpus / referees | UNMODIFIED |
| Test lifecycle | parity is a TIER7 CPU/GPU oracle `KEEP`; typed negatives are `AUDIT` / `ledger-only`; both are born on 0.0.8.7 with `dsu_survivals=0`; STEAD proof remains contract-required |

## Focused proof

```text
FIELD-SWEEP-N4-PARITY adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan PALMA=bit-exact Gu-Yang=bit-exact
test result: ok. 2 passed; 0 failed
```

## Preservation and next route

The same interpreter executes both authored algebras; float fold order is the registration-bound
canonical adjacency order; scheduling does not author physics; conservation is admitted rather than
inferred. N8, weighted adjacency, LinkGraph, resource-class specialization/JIT, comparative
projections, corpus changes, and legacy-shader retirement remain outside this rung.

DA review is required because this diff adds EML vocabulary and permanent gate wiring. The track
pointer remains on 5.5.
