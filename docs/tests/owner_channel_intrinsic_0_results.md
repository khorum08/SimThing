# OWNER-CHANNEL-INTRINSIC-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.0)
- Status: **PROBATION / proof-present / DA-review-pending**
- ORIENT-RECEIPT: `4992234cbe01`
- HD-RECEIPT: `8a5a9c8b0f14`
- Authoritative DA intervention: Board `5151931092`
- Implementing handoff: Board `5152775529`

## Remaining deliverables (d)/(e)

| Contract | Evidence |
|---|---|
| Generalized reduce-up key | `OwnerChannelScopeKey { owner_ref, resource_key, scope_id }`; retired domain-shaped fields are absent |
| N-owner conservation | Three owners and two resources coexist under one synthetic container; input totals equal canonical bucket totals |
| No owner-equality aggregation branch | Ordered-map insertion by the full key performs segregation; owner comparison exists only inside the already-sanctioned crossing predicate |
| One boundary | Ownership crossing, reduce-up `ScopeId`, and AccumulatorOp execution plan use the same boundary-node identity |
| Bounded STEAD record | One owner-bearing record per crossing; identity edges retain none; ordinary own-aggregate rows carry no resolved owner or scope |
| Reconstruction | Crossing records plus own aggregates reconstruct the complete canonical RF bucket map |
| CPU/GPU parity | Every owner/resource/scope bucket is reduced through the existing `AccumulatorOp` sum path and compared bit-exactly after GPU readback |
| Synthetic witness | Inline arbitrary-tree proof plus the crate-local `specialist_citizens_minimal.clause`; no shipped asset or cross-crate fixture reach |

## Focused proof

```text
owner_channel_intrinsic_reduce_up_0: 3 passed; 0 failed
```

1. `n_owner_container_conserves_and_reconstructs_in_canonical_bucket_order`
2. `retained_owner_state_is_bounded_by_crossings_not_nodes_owners_or_resources`
3. `every_owner_resource_scope_bucket_is_bit_exact_on_cpu_and_gpu`

The boundedness falsifier uses 128 nodes, three owners, and two resources. It retains 256
ordinary node/resource own-aggregate rows but exactly two owner-bearing crossing rows,
demonstrating that owner state scales with crossings rather than nodes multiplied by
owners/resources.

## Verification

- `cargo check -p simthing-spec -p simthing-driver --all-targets`: PASS
- `cargo test -p simthing-spec --lib`: PASS (5/5 under the repository's active pare profile)
- `cargo test -p simthing-driver --test owner_channel_intrinsic_reduce_up_0`: PASS (3/3)
- GPU parity with `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`: PASS (real adapter required)
- Existing owner-channel core proof: PASS (9/9)
- Crate-local specialist fixture hydration/census proof: PASS
- Determinism matrix: PASS (2/2)
- Scenario residue: INSPECT with 0 scenario, 0 domain, and the same 20 advisory pre-existing dead exports; the new surfaces have live consumers
- Detachability: PASS (`production_coupling=0`, `proof_coupling=0`, `ceiling=0`)
- Generated digest/orientation and document budget: PASS
- Generalized-key retirement scan: PASS; retired planet/star scope fields are absent from spec/driver production and test surfaces
- Inventory/lifecycle: the three new rows are classified; the repository-wide checker retains its warned baseline (153 missing mechanical rows plus nine unrelated existing judgment errors), with no new owner-channel error

## Posture

This rung does not graduate itself, advance the pointer, or implement any downstream `BINDS 6.0`
consumer. The result remains **PROBATION / proof-present / DA-review-pending**.
