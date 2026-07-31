# FIELD-SWEEP-N4-PARITY-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.5)
- Status: **COMPLETE — DA-GRADUATED / merged #1529 @ 4ab19613**. DA reproduced on RTX 4080/Vulkan: parity 3/0 (incl. `palma_and_gu_yang_are_bit_exact_cpu_and_gpu`), STEAD guards 3/0, driver 48/0/1, gates 0/0/0. FIELD-SWEEP-SINGLE-PATH falsification-proven: a planted eighth bespoke field shader flips the SHADERS arm PASS->FAIL.
- HD-RECEIPT: `2539bdaf8933`
- ORIENT-RECEIPT: `01a351001f4c`
- Remand: Board comment `5139419605`
- expected route: `DA-RESERVE(gate-wiring)`
- Pointer: remains `FIELD-SWEEP-N4-PARITY-0`; no graduation or 5.6 authority is claimed.

The tested exact-head SHA is bound in the PR and board relay because this result file cannot
self-hash.

## Contract discharge

| Requirement | Result |
|---|---|
| One production field-sweep path | PASS — one kernel-owned WGSL interpreter over the existing `AccumulatorOp` input-list gather |
| Typed identity frontier | PASS — `ColumnIndex` / `SlotIndex` survive through registrations, PALMA/Gu-Yang specs, edge context, and `InputSpec` gather rows; raw POD encoding occurs only at the existing WGSL/GPU boundary |
| Algebra as data | PASS — authored EML map / fixed-linear-fold / post programs; no field/algebra enum, tag, or operator-identity dispatch |
| Edge context | PASS — target slot, neighbor slot, accumulator, edge scalar, and dt; `TARGET_VALUE` / `NEIGHBOR_VALUE` are field-context-only |
| Proof-present registration | PASS — canonical-order proof required; conservative folds additionally require adjacency-bound undirected-symmetry certificate |
| Resource class | PASS — admission accepts only the legacy fixed `32` stack / `32` program-node class until rung 5.7 |
| PALMA | PASS — authored WENS N4 min-plus instance with destination pinning authored in EML |
| Gu-Yang | PASS — authored NSEW N4 conductance + conservative flux registrations |
| Exact session binding | PASS — immutable adjacency, gather order, slot count, and dimension layout bind session creation; equal-length registrations with different authored order reject before queue writes; Gu-Yang's two passes share the exact binding |
| Parity-alongside | PASS — PALMA and Gu-Yang are bit-exact on CPU and GPU against the existing unmodified legacy paths |
| Adapter evidence | `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan` |
| Typed negatives | PASS — raw identities are compile-time rejected; forged out-of-range typed columns and destination slots reject during admission before upload/dispatch |
| STEAD convergence | PASS — contract §10 and `stead_spatial_contract_guards` bind both authored N4 instances to the generic registration |
| Permanent single-path tripwire | PASS — algebra/tag/operator scan plus exact canonical interpreter exemption across both GPU and kernel shader homes; planted eighth-shader self-tests fire in each home |
| Legacy migration oracles | PRESERVED — seven shaders catalogued with per-item rationale and rung-10.1 promotion blocker |
| Corpus / referees | UNMODIFIED |
| Test lifecycle | parity is a TIER7 CPU/GPU oracle `KEEP`; typed negatives are `AUDIT` / `ledger-only`; both are born on 0.0.8.7 with `dsu_survivals=0`; STEAD proof remains contract-required |

## Focused proof

```text
FIELD-SWEEP-N4-PARITY adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan PALMA=bit-exact Gu-Yang=bit-exact
test result: ok. 3 passed; 0 failed
```

## Preservation and next route

The same interpreter executes both authored algebras; float fold order is the registration-bound
canonical adjacency order; scheduling does not author physics; conservation is admitted rather than
inferred. N8, weighted adjacency, LinkGraph, resource-class specialization/JIT, comparative
projections, corpus changes, and legacy-shader retirement remain outside this rung.

DA review is required because this diff adds EML vocabulary and permanent gate wiring. The track
pointer remains on 5.5.
