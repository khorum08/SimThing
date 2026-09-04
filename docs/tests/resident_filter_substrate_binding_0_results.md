# RESIDENT-FILTER-SUBSTRATE-BINDING-0 — implementation evidence

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED / NO 15.7**.

Authority: handoff `handoffs/RESIDENT-FILTER-SUBSTRATE-BINDING-0.hd.md` (`HD-RECEIPT: d665dff5c374`), resume/remand `5536145216`, binding E5 DA Option B ruling `5536135335`, exact-arithmetic remand `5540959678`, and exact-authority remand `5542040712`. Coding orientation is `ORIENT-RECEIPT: 39716239b6b2` under rule stamp `a7f6c618858f2ed5`.

## Result

The resident exact projection is now homed on the ordinary admitted RF arena. It binds the arena's actual participant topology, property columns, live `AllocatedFlow`, descriptor integration band, and the session's authoritative `WorldGpuState`. The former private continuous world, host market vector, host-authored weight, and per-dispatch allocation-plan compile/upload have been removed from the production executor.

The continuous plane remains smooth policy authority. Market admission now carries an integer `ResidentExactBasisIdentity` and seals it into `ResidentMarketQualification`: `NeutralRequest` restores the exact request, while `LiveAllocatedFlow` converts the actual binary32 cell losslessly to Q149 and applies the exact integer cap there. Dispatch rows carry no basis-mode field, so a caller cannot alter Q under an unchanged qualification. Neither CPU nor WGSL projects `requested` to `f32`, and no comparison between claims or policy cells creates an exact equality band. The production GPU and its CPU mirror continue through the one Q149 projection and emit the one canonical `T_s`.

Sessions do not acquire an RF property or extra dimensions merely by opening. A scenario that explicitly carries the canonical RF property gets the default recursive arena; an authored spec gets its compiled arena. First admission and topology rebind both consume a freshly sealed borrowing view of the authoritative runtime tree. Empty growth batches need no executor; an actual growth batch without a completely admitted resident market fails closed.

## Archaeology map and production call graph

| Input / lifetime | Before 15.6 | Bound production source |
|---|---|---|
| topology | synthetic flat-star over semantic lanes | `ArenaRegistry` descriptors and participant rows admitted by ordinary scenario/spec installation |
| born P/F | private `AccumulatorOpSession` and private values plane | ordinary resource-flow band execution in the session `WorldGpuState` |
| `AllocatorWeight` | host `continuous_weight` field | live property column selected from the admitted arena layout |
| `AllocatedFlow` | private per-market-dispatch output vector | live ordinary RF cell addressed by admitted participant slot and column |
| precedence | caller integer | unchanged exact claim field; orders feasible work and reserves nothing |
| exact projection | plan compiled after manufacturing private continuous state | canonical semantic plan and exact-basis identity sealed at admission/rebind; dispatch binds live cells and submits Q without selecting basis mode |
| semantic rows | fixed synthetic resource/scope/Draw constants | digests and dictionaries derived from admitted market identity plus every admitted tree scope/lane |
| topology identity | absent from consumer token | selected arena, participant topology digest, registry generation, and registry layout digest |
| plan upload lifetime | every market dispatch | resource-flow plan at install/topology refresh; resident semantic plan at admission/topology rebind |

```text
Scenario property or authored GameModeSpec
  -> ordinary DimensionRegistry + ArenaRegistry admission
  -> BoundaryProtocol authoritative tree/registry/SlotAllocator
  -> initial_gpu_sync / ordinary RF plan sync
  -> freshly sealed TreeExecutionBinding
  -> ResidentRfArenaBinding (arena layout + participant slots + live columns)
  -> ResidentMarketQualification + sealed exact-basis identity + one resident semantic plan

generation dispatch
  -> canonical u32 request + precedence + rf_participant (no host basis-mode field)
  -> exact basis identity recovered from the admitted market lowering
  -> shared WorldGpuState[participant slot, live AllocatedFlow column]
  -> existing resident exact Q149 apportionment
  -> immutable canonical T_s in the one bounded live head
```

## Mechanical absence proof

The focused source referee scans the production resident owner and the real fission rebind call site. Its transcript is:

```text
15.6 ZERO-USE PASS private-world=0 host-vector=0 per-dispatch-plan=0 host-weight=0 E7-fresh-executor=0
```

The deleted production members/routes are `continuous_plane`, `continuous_values`, `prepare_continuous_allocation`, and `continuous_weight`. `dispatch_market` now submits directly against the caller's authoritative session `WorldGpuState`; it neither compiles nor uploads an allocation plan. The fission enrollment route invokes identity-preserving bind/rebind and contains no fresh `ResidentClearingRuntime::admit` call.

## ResidentMarketQualification seal

`ResidentMarketQualification` is consumer-owned, has private fields, and is required at every dispatch, materialization, temporal-mint, and proof readback. It is evidence, not a boolean or eligibility enum.

| Sealed field | Bound fact | Invalidation witness |
|---|---|---|
| `market_semantic_digest` | authored market, offering/Draw envelope, resource/scope and policy identities | changing the authored Draw identity produces a distinct token |
| `resource_shape_digest` | resource identity plus selected flow property | covered by market/property binding |
| `scope_draw_shape_digest` | every admitted realm-qualified scope owner and lane Draw row | topology rebind changes the shape token |
| `arena_idx`, `flow_property_id` | selected admitted RF arena and property | incomplete/missing preferred arena typed-refuses |
| `topology_digest` | sorted admitted participant/slot/parent topology | 4-to-5 participant growth changes the digest; old token returns `StaleMarketQualification` |
| `registry_layout_digest`, `registry_generation` | property activation/layout/column ranges and arena registry generation | adding a registry property changes the layout digest |
| `precedence_digest` | bound precedence lowering identity | included in the sealed equality check |
| `continuous_policy_digest` | authored policy identity plus canonical child-share EML identity | included in the sealed equality check |
| `exact_projection_abi_digest` | `resident-q/u32-request+live-allocated-flow+exact-basis-identity/v3` | included in the sealed equality check |
| `exact_basis_identity` | immutable `NeutralRequest` or `LiveAllocatedFlow` selected by admitted market lowering | changing basis mode produces a distinct qualification; cross-mode token use returns `StaleMarketQualification` |
| private `seal` | digest of all fields above | `has_intact_seal` and exact token equality precede execution |

A genuinely authored non-implicit shipyard market qualifies and executes. Mutating live RF weights from `9:1` to `1:9` after admission flips the exact winner left-to-right, proving live arena cells defeat stale host assumptions. A market preferring an absent arena returns typed `MarketCannotLower` rather than growing a score system or alternate executor.

```text
15.6 AUTHORED-MARKET PASS live-AllocatedFlow-winners=left/right
market-mutation=INVALID registry-mutation=INVALID incomplete-lowering=TYPED-REFUSAL
```

## E5 Option B and exact-basis remand transcripts

The permanent falsifier uses requests `r1=16,777,217`, `r2=16,777,216`, and supply `S=1`. The two binary32 cells have identical bits, mechanically reproducing the boundary collapse. Before remand `5540959678`, a genuinely below-cap `16,777,216f32` for source 1000 was incorrectly raised to its exact request because the request projected to those same bits. The pre-remedy test was RED: CPU produced `{1000:(1,16777216),1001:(0,16777216)}` where the live-below-cap authority required `{1000:(0,16777217),1001:(1,16777215)}`.

After the remand, both the resident GPU and CPU resident mirror consume the same exact producer fact and distinguish all three meanings without float equality, epsilon, magnitude, or ordering authority:

| Case / carried identity | source 1000 `(G,U)` | source 1001 `(G,U)` | Result |
|---|---:|---:|---|
| neutral collapse / `NeutralRequest` | `(1, 16,777,216)` | `(0, 16,777,216)` | exact request recovered |
| genuine below cap / `LiveAllocatedFlow` | `(0, 16,777,217)` | `(1, 16,777,215)` | exact live value retained |
| genuine above cap / `LiveAllocatedFlow` | `(1, 16,777,216)` | `(0, 16,777,216)` | exact request cap applied |

Each row is tested through both the CPU mirror and a real GPU dispatch. The full admitted `u32` request domain remains intact. The continuous `f32` plane still owns smooth allocation policy; exact possession/request arithmetic remains integer-exact in the existing single Q/operator with no second clearer or response state.

Remand `5542040712` then exposed an authority defect above that arithmetic: the three public production row bindings still let a dispatch caller choose `LiveAllocatedFlow` or `NeutralRequest`. The demanded pre-remedy same-token mutant held qualification, market, arena, requests, and live cells fixed; flipping only that enum changed the full resident-runtime grant vector from `[0,1]` to `[1,0]`. After remedy, those row fields are absent. Two otherwise-identical admitted markets with different basis identities produce distinct sealed qualifications, and presenting the live token to the neutral runtime returns `StaleMarketQualification` before dispatch.

The focused referee also executes the three cases through `ResidentClearingRuntime`, the real admitted RF arena, and `ResidentMarketQualification`, not only through the low-level kernel session:

```text
15.6 E5 PRODUCTION PASS dispatch-tag=ABSENT cross-basis-token=TYPED-REFUSAL
neutral=source-8 below-cap=source-9 above-cap=source-8
```

## E7 identity-preserving rebind

Lawful runtime growth expands the actual RF participant topology from four rows to five while keeping the same executor. The old qualification is invalid immediately after the rebind.

| Required continuity | Witness |
|---|---|
| `TreeRealmId` | unchanged `0x1506` realm |
| `ExecutionIncarnation` | unchanged incarnation `1`; no migration |
| `GenerationStamp` | pending and post-rebind products remain generation `30` |
| live-head continuity | a ticket submitted before growth materializes successfully after rebind |
| semantic identity | an existing owner/resource/scope/Draw semantic row is unchanged |
| persistence deformation | admitted `f(U)=U/2` remains installed and mints demand `5` from U `10` |
| pending exact provenance | the pre-growth ticket/product remains the same live-head submission and semantic row |
| bounded shape | lane capacity remains `2`; only topology-dependent plan/buffers are rebound |

```text
15.6 E7 PASS topology-participants=4->5 lane-capacity=2
realm=PRESERVED incarnation=1 generation=30 pending-live-head=PRESERVED
persistence-demand=5 topology-token=INVALIDATED
```

## Frozen cross-product and authority closure

The graduated 15.5 five-test referee now constructs its continuous values through a real admitted recursive arena and the shared world state. The canonical combined object remains:

```text
T_s(N=50)=(G4,U6)
child=(changed granter 8, changed scope, descendant source 9, G4, N50)
authored_N1=2; identity demand=8; half-deformation demand=5
N+1 supply=5 -> (G5,U3)
```

The E6 mixed band remains work-conserving at `[0,1,9]`, and only a real conserved in-flight holding reserves supply. The 15.0 theorem/census and 15.1 unification referee remain green with the peer-authority census unchanged at two. No market, clearer, disburser, Q, score system, host policy authority, exact projection, or economic response state was added.

## Test certificate

- Focused 15.6 referee: 4/4, including qualification-bound basis identity and the full production real-arena E5 three-way witness.
- Frozen resident apportionment 14.5: 7/7, including permanent E5.
- Frozen cutover/causal/tree-isolation 14.6: 3/3; parity terminal: 1/1 with qualification fingerprint `0x810418ff57aa9b08`.
- Recursive-resource formalization/unification: 1/1 and 2/2.
- Persistence/consequence/oracle-quarantine/recursion-axis: 3/3, 2/2, 2/2, and 5/5.
- Driver growth entitlement: 5/5; participant elimination: 2/2; field-sweep session seam: 3/3.
- Previously failing capability-effect, capability-prerequisite, grant-disbursement, and unified-convergence targets rerun green after explicit arena admission.
- `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet`: exit `0`, 141 test-result groups, every group `ok`. A diagnostic case-insensitive search for `FAILED` also matched the literal text `0 failed`; it is not reported as a red count.

The exact committed head's inventory, drift, constitutional/census selftests, lifecycle, sanctioned-surface, anchor, detachability, Agent Scan, hosted Doctrine Scan/Exec, clearance, and relay-lint identifiers are carried in the PR/board relay after those head-bound gates run.

## Scope and fences

This rung changes the existing driver/GPU/kernel/sim resident path, existing frozen referees, the focused 15.6 referee, inventory, sanctioned surface, and evidence only. It also updates the inherited `need_binding` compile-fail snippet to construct admitted `ColumnIndex` values through the current public API; the negative type proof itself is unchanged. It does not edit workflow or CI implementation, canon, active pointer, compression, graduation, merge, or closeout state. 15.7 remains fenced.
