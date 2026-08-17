# DERIVED-SPAN-PROJECTION-INVALIDATION-0 (7.8a) evidence

Status: **PROBATION — proof present; DA review pending**

## Receipts and scope

- Implementation base: `75e50454ba2622fc0cdfda3dc77cc8975d1620af`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- orientation digest: `1eb98b1bd147714ee421cdc5f43cf892b8b65038c9b72b7b65a5e6cf72cf5100`
- HD-RECEIPT: `713dc041018b`
- Orchestrator handoff: Board comment `5311539724`
- Expected route: `DA-RESERVE(gate-wiring)`

Every rendered REQUIRED-ANCHOR was queried before governed edits. The generated reach log carries the complete 56-anchor projection. Load-bearing ACKs include `overlay-scale-laws@c2ffb2826df7`, `overlay-germ@f0c8d2ebade9`, `overlay-promoted-laws@248c7893b462`, `overlay-closure-thesis@241cc54c5706`, `overlay-designer-closure@4a047b29243d`, `core-overlays@f95c9376ee06`, `stemthing-slot-identity-ruling@02c87b9126e1`, `core-gpu-residency@eea1356db087`, `evaluation-identity-invariants@64ad30392930`, `one-tree-owners-never-spatial@a8689d4344f9`, `actionband-determinism-lifecycle@6306c484732c`, `actionband-performance-model@8d93f06d4bae`, `actionband-eml-payload-purity@2a1d981f3958`, and `actionband-native-authority-table@541a03cb00a1`.

No CI implementation, workflow, scan, allowlist, binding-condition, orientation, ladder, pointer, 7.9/8.x, Vector CostBand, or StemThing-B implementation was edited. `OVERLAY-PEER-AUTHORITY` remains live. Coding does not invoke `/clearance`.

## Production shape and replaced route

The former production path was:

`BoundaryProtocol::execute` → `sync_gpu_buffers` → `build_overlay_deltas` → recursive `build_node` → clone the ancestor transform vector at every descendant → emit one dense physical-row range per allocated object.

That is the current O(depth × descendants) path replaced by this rung. `build_overlay_deltas` remains only as the CPU parity oracle. The disabled AccumulatorOp route now REDs immediately and cannot invoke that recursive builder.

The production path is now:

`BoundaryProtocol::execute` collects exact overlay/property host changes and the existing `GenerationStamp` → `sync_gpu_buffers` retains one `OverlaySpanProjection` in `OverlayCompileCache` → session/topology admission builds the logical preorder range directory and sparse routed-target index once → already-composed standing/routed overlay behavior interns into `EffectiveProfileId` descriptors and maximal homogeneous `EffectiveSpan`s → an ordinary lifecycle/property boundary emits source-blind `ChangedLocus { logical_id, PropertyId, role, optional narrowing }` → frozen `DerivedDependencyIndex` resolves dependent span roots, STEAD/PALMA/Gu-Yang registrations, and derived work → only intersecting spans split/remap and receive the existing generation stamp → the separate dense materializer resolves current columns/slots and feeds the existing overlay OrderBand ABI.

The projection has no writer/source discriminant, mutable registry/service, listener, second generation/epoch authority, owner GPU identity plane, or universal value resolver. Overlay composition remains owned by the graduated overlay law. `resolve_owner()` and `resolve_owners_in_order()` are unchanged CPU query/oracle surfaces.

## Scale and invalidation measurements

The compact million-row fixture admits one logical subtree range without allocating a million descendants:

| Fixture | Logical rows | Profiles | Spans | Invalidation member rows scanned |
|---|---:|---:|---:|---:|
| homogeneous | 1,000,000 | 1 | 1 | 0 |
| one-row local divergence | 1,000,000 | 2 | 3 | 0 |

Homogeneous adjacent duplicates fail admission as `DescendantScaleProfileExplosion`. Profile descriptors are actually interned by semantic identity; a profile-id collision with unequal semantics fails closed. Runtime invalidation examines the span partition, not member rows. Sparse routed influence is indexed by active overlay host and route host, so a changed host does not rediscover routes by scanning every descendant.

Dense materialization remains explicit and separately reports `rows_materialized`; it is retained only for the current one-op-per-physical-row OrderBand upload ABI. Deleting `OverlayCompileCache`'s dense vectors leaves the profile/span authority intact. Rebuilding produces identical deltas/ranges, and a forced `SlotAllocator::epoch_rebind` changes physical slots while preserving the logical-id profile digest and per-logical-id operations.

## Five mandatory production falsifiers

1. **Descendant-scale profile/span explosion:** a planted adjacent split of the one-profile million-row span REDs `DescendantScaleProfileExplosion`; the admitted fixture stays one profile/one span.
2. **O(depth × descendants) generation rewalk:** source-blind invalidation of the million-row root examines one span and zero member rows; a one-row local change yields three total spans and dirties only the dependent span. Production GPU sync no longer calls the recursive builder, and disabling the projection REDs immediately.
3. **Writer-subsystem/source branch:** `ChangedLocus` uses `deny_unknown_fields`; injecting `change_source` REDs deserialization. Identical state changes therefore have identical invalidation keys.
4. **Runtime-mutable dependency registry/service:** `DerivedDependencyIndex::admit` consumes its rows and exposes reads only. The compile-fail proof REDs a planted `insert_runtime_dependency` call; exact admitted bindings route to the span plus all three existing Field-Triad authority tags and derived work.
5. **Dense materializer as semantic authority:** the test deletes the real `OverlayCompileCache` dense vectors, proves the semantic profile digest remains, rebuilds bit-identical dense output, forces a physical epoch rebind, deletes again, and rematerializes identical behavior by logical identity.

These falsifiers exercise production types and routes, not inert `refuse_*` helpers.

## StemThing-B forward-bind analysis

The §6.4/§6.5 partial-reconciliation forward-bind is expressible without B-specific widening. A B consumer can name its authoritative reconciliation locus with the existing consumer-neutral tuple `(logical identity, PropertyId, SubFieldRole, optional admitted binding/profile id)`, map it at session admission to a dependent logical span or derived work id, and rematerialize against the current physical binding after EpochRebind. No B type, B-specific field, owner interning, physical row identity, or consumer-specific combine law is required in the substrate. Therefore this rung does not STOP on an inexpressible locus and implements no StemThing-B semantics.

## Test budget and validation

Seven permanent inventory entries are admitted:

- `homogeneous_million_row_projection_rejects_descendant_scale_profile_explosion` — mandatory profile/span explosion falsifier and million-row scale proof.
- `invalidation_visits_spans_not_depth_times_descendants` — mandatory ancestor-rewalk falsifier plus local-divergence measurement.
- `changed_locus_rejects_writer_subsystem_discriminants` — mandatory source-blind-key falsifier.
- `dependency_index_is_frozen_and_routes_exact_span_field_and_work_targets` — exact dependency routing proof for the generic substrate.
- `dense_materialization_is_deletable_cache_and_remaps_by_logical_identity` — mandatory semantic-materializer falsifier plus EpochRebind parity.
- `standing_and_routed_projection_match_inheritance_oracle_after_local_split` — one retained-oracle parity proof for both graduated overlay modes.
- `compile_fail_line_151` — the only mechanically enforceable proof that the dependency index has no runtime mutation API.

Validation:

- focused 7.8a integration: 6 passed;
- `simthing-kernel` doctests: 44 passed, including the new compile-fail proof;
- owner-channel oracle: 11 passed;
- intrinsic overlay inheritance: 3 passed;
- driver RF reception/oracle: 2 passed;
- full `simthing-sim` package: PASS;
- `cargo check -p simthing-sim`: PASS;
- `TEST-INVENTORY-DRIFT-CHECK`: PASS (`rows=1269 discovered=1269 unledgered=0 stale=0`);
- overlay census: `RECONCILIATION: routes=77 discovery=73 residue=77 unclassified=0 open=0`; PASS; no universe delta or repin;
- `test_inventory_check.sh`: existing repository-wide INSPECT findings remain (172 mechanically enumerated cfg-test module rows plus pre-existing lifecycle judgments); the authoritative drift gate for this diff is clean.

Exact-head agent scan, doctrine scan, relay-lint, hosted checks, and PR identity are recorded in the orchestration relay after publication.
