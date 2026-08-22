# EMBEDDER-INTERFACE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 11.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- Canonical base: `30491614bb0b4a1ada23a523dfe662cba267b4d1`
- HD-RECEIPT: `606500a6a787`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- Coding branch: `codex/embedder-interface-0`
- Tested code SHA: `466aa0ff`
- Expected route: `DA-RESERVE(gate-wiring)`
- Dispatch: board comment `5377904777`

## Archaeology and gap classification

The required first step completed before construction. No verb requires a new
runtime authority, stateful service, scheduler, evaluator, registry, history,
dispatcher, execution path, or admission vocabulary.

| Obligation | Graduated production mechanism | Gap classification |
|---|---|---|
| Specialist data | `SimThing::declared_specializations` + `derive_specializations` | None |
| Owner seats / neutral reservation | `make_owner_entity` + `OwnerRef::try_new_authored` + `unowned` | None |
| Owner × specialization | `resolve_owners_in_order` joined to `SpecializationReport` | Permitted pure query |
| Tree/RF population | `SimThing`, `GameModeSpec`, `SimSession::open_from_spec` / existing install admission | None |
| Intrinsic ownership | `bind_owner`; absence resolves through `resolve_owner` | Pure boundary-shape validation added |
| Queue shape/depth | `CostBandSemantic::admit_sink` + `ThresholdRegistry::resolve_cost_band_draw`; runtime `V→N` | None |
| Overlay origin/horizon | `resolve_owner` membership + `dispatch_until_dissolved` + `admit_dispatch_minted_overlay` | None |
| Band consequence/threshold | `compile_crossing_consequence_session`; session velocity/aggregate registration | None |
| CPU observation | existing coordinator shadow + slot allocator + production generation stamp | Permitted borrowed read-only view |
| Lifecycle/posture/history | `open_from_spec`, `set_execution_posture`, `step_once`, `record_to_path` | None |

One integration defect was exposed by the end-to-end proof: intrinsic metadata
properties had no `DimensionRegistry` column but dense projection indexed them
as dimensions. `project_tree_to_values` now consults the existing registry
authority with `try_column_range`; unregistered structural metadata stays out of
GPU values. No property authority or new projection path was added.

## Five verbs → mechanism table

| Verb | Vendor Door meaning | Delegated engine surface |
|---|---|---|
| Derive | Author/derive specialists and owner seats; query owner × specialization without kind | core specialization protocol, intrinsic owner resolution, canonical spec owner entity |
| Populate | Bind only owner crossings and author queued CostBand shape | core `bind_owner`/boundary validation; sim CostBand admission |
| Overlay | Require a supplied in-tree origin and non-empty authored horizon | core origin resolution + dispatch lifecycle admission |
| Bind | Bind ActionBand consequences/thresholds and borrow observation | driver ActionBand consequence compile; sim threshold builder; driver read-only shadow |
| Run | Initialize, select posture, tick, serialize | the existing `SimSession` lifecycle and replay writer |

## Public-function delegation table

| Public facade function | Graduated callee(s) |
|---|---|
| `derive::owner_seat` | `OwnerRef::try_new_authored` → `make_owner_entity` |
| `derive::specializations` | `derive_specializations` |
| `derive::owner_specializations` | `query_owner_specializations` |
| `derive::installed_owner_specializations` | `SimSession::owner_specializations` |
| `derive::reserved_unowned` | `unowned` |
| `populate::owner` | `bind_owner` |
| `populate::ownership` | `validate_owner_binding_boundaries` |
| `populate::queued_cost_band` | `cost_band_quantize` validation + `CostBandSemantic::admit_sink` |
| `overlay::authored` | `resolve_owner` → `dispatch_until_dissolved` → `admit_dispatch_minted_overlay` |
| `bind::action_band_commitments` | direct re-export of `compile_crossing_consequence_session` |
| `bind::velocity_threshold` | `BoundaryProtocol::register_velocity_alert` |
| `bind::aggregate_threshold` | `BoundaryProtocol::register_aggregate_alert` |
| `bind::queued_draw` | `ThresholdRegistry::resolve_cost_band_draw` |
| `bind::shadow` | `SimSession::shadow_view` |
| `run::initialize` | owner boundary validation → `SimSession::open_from_spec` |
| `run::start` | `SimSession::set_execution_posture` |
| `run::tick` | `SimSession::step_once` |
| `run::serialize` | `SimSession::record_to_path` |

## Structural leaf certificate

`simthing-embedder` depends downward on core/spec/sim/driver. No engine crate
depends on it. Its source contains no static, mutex, cache, registry, scheduler
loop, runtime authority, owned session wrapper, evaluator, dispatcher, or
history. The only stateful value exposed is the existing engine-owned
`SimSession`; the facade never wraps or owns it. `SessionShadowView` borrows the
existing allocator and CPU shadow, carries the existing generation stamp, and
has no mutable values, evaluator, or submission door.

## Biting falsifiers

| Planted rival | Ordinary production-door RED |
|---|---|
| Kind-based owner/specialization answer | Changing `SimThingKind` while owner/profile facts stay fixed leaves the kind-free Derive query identical |
| Authored party named `unowned` | `derive::owner_seat` rejects through `try_new_authored` |
| Stamp owner on every node | Populate and Run reject a child binding equal to its inherited owner |
| Synthesized/foreign overlay origin | `overlay::authored` rejects a foreign `SimThing`; bare-id origin is compile-fail |
| Missing/default overlay horizon | `overlay::authored` rejects an empty condition list as `MissingDissolveCondition` |
| Queue re-hydration | One admitted event/CostBand shape resolves runtime depths 1 and 4 without rebuilding the registration |
| Writable/decision shadow | private borrowed storage is compile-fail writable and exposes observation methods only |
| Posture semantic fork | paced and continuous call the same `step_once` door and produce the same one-tick outcome |

## Evidence

| Command / proof | Result |
|---|---|
| `cargo check -p simthing-embedder` | PASS |
| `cargo test -p simthing-embedder -- --test-threads=1` | PASS — 6 integration, 1 compile-fail |
| `test_inventory_drift_check.sh` / inventory / lifecycle schema | PASS |
| Core owner-channel / kernel projection / driver doctests | PASS — 11 / 5 / 7 (+1 ignored) |
| Detachability / DOC-BUDGET / orientation / sanctioned digest | PASS |
| DEAD-EXPORT residue scan | INSPECT — 48 pre-existing test-support exports; no touched or embedder path |
| Local Doctrine Scan | INSPECT — zero hard failures; heuristic baseline findings only |
| Agent scan at `466aa0ff` | INSPECT — zero hard failures; one test-budget heuristic for the six required named production-door falsifiers |
| `cargo test --workspace --all-targets -j 1 --no-fail-fast -- --test-threads=1` at `466aa0ff` | Exit 1 — only the three handoff-declared baseline ClauseThing targets; every other target green |
| Baseline `ct_2c_category_economy` red | `gpu_category_micro_economy_matches_arena_allocation_oracle`: existing install validation failure |
| Baseline `ct_3b_4a_gpu_projection` red | `gpu_scatter_projection_matches_cpu_oracle_through_commitment`: existing install validation failure |
| Baseline `studio_star_naming_pass_0` red | `star_naming_canonical_tp_all_systems_have_display_names`: existing canonical naming mismatch |
| Hosted Doctrine Scan `32554842664` / job `96987273824` | PASS — every PR-applicable step inspected; PR delta has zero hard failures and the same one test-budget INSPECT; whole-tree step skipped by PR policy |

## Scope disposition

Return **PROBATION / proof-present / DA-review-pending**. Coding does not invoke
`/clearance` or `/relay-lint`, merge, graduate, move the pointer, or begin 11.2+.
