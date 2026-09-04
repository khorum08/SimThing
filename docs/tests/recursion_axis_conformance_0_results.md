# RECURSION-AXIS-CONFORMANCE-0 — full remediation evidence

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED / NO 15.6**.

Authority: dispatch `5532779733`, semantic-scope binding `5529092629`, E6 Owner-stop packet `5533226409`, binding Owner ruling `5533372979`, resume remand `5533564241`, and mixed-equality-band remand `5534307164`. The handoff is `handoffs/RECURSION-AXIS-CONFORMANCE-0.hd.md` (`HD-RECEIPT: bbecf03b4d88`). Coding orientation is `ORIENT-RECEIPT: 4266e4870c67` under rule stamp `240c816e9fe71dc1`.

## Result

The production resident port now implements two distinct views of one immutable canonical `T_s`:

- Spatial recursion reads exactly parent `T_s.G` as child supply at the same generation, then clears under a changed child granter and the child's separately admitted semantic-row bank over actual descendants.
- Temporal recursion reads exactly parent `T_s.U` inside the existing Current-to-Next once-mint, combines it with independently authored N+1 demand as `d_effective(N+1) = d_authored(N+1) + f(U(N))`, and emits ordinary demand rather than a product-shaped pseudo-`T_s`.

Immediate flow is work-conserving under the Owner's E6 ruling. Precedence orders feasible work but consumes no capacity on a zero basis, including when a zero-basis request shares an equality band with a serviceable sibling. Each band's executable ceiling includes only non-zero-basis rows, and later bands subtract the resulting exact prior grants, never all requests in a band. Reservation is supplied only through the existing conserved `ResidencyCapacityPartition` in-flight holding lifecycle; there is still one exact projection.

## Production call graphs

Spatial:

```text
dispatch(root, N)
  -> one continuous AllocatedFlow producer
  -> one exact resident apportionment
  -> immutable T_s append
dispatch_spatial(parent_ticket, child_granter=v, N, scope=v)
  -> validate v is a parent product and each claim is a descendant of v
  -> exact shader reads parent product G directly from the resident segment
  -> child exact apportionment over v's separately admitted semantic rows
  -> immutable child T_s append
```

Temporal:

```text
immutable T_s(N)
  -> prepare_temporal_demands(N+1, authored demand)
  -> one resident demand mint reads U, evaluates optional admitted f(U), and emits 4-word ordinary demand
sealed/authored/born N+1 datum arrives
  -> dispatch_temporal(prepared demand, N+1 supply/precedence/weight)
  -> the same exact resident apportionment
  -> immutable T_s(N+1) append
```

Preparation submits no N+1 economics. The N+1 execution API requires the prepared demand ticket and separately supplied N+1-authoritative execution data.

## Canonical cross-product and parity transcripts

The production real-GPU referee constructs `T_s(N=50)` from request 10 and supply 4, producing `G=4, U=6`. A child market at the same generation changes granter/scope from root 7 to child 8; descendant source 9 receives exact supply 4 from the immutable parent product and produces its own semantic identity. Temporal preparation adds authored demand 2 to U 6 and emits ordinary demand 8. N+1 execution with N+1-authoritative supply 5 produces `G=5, U=3`.

```text
15.5 CROSS-PRODUCT PASS T_s=(G4,U6,N50)
child=(granter8,source9,G4,N50)
authored_N1=2 effective_N1=8 executes_with_N1_supply=5
```

The CPU once-mint and resident demand mint are bit-exact for both mandatory cases:

| Case | T_s input | Authored N+1 | Effective CPU demand | Effective resident demand |
|---|---:|---:|---:|---:|
| identity | `G=4, U=6` | 2 | 8 | 8 |
| admitted half deformation | `G=4, U=6`, `f(U)=U/2` | 2 | 5 | 5 |

The frozen 15.2 production decay referee also remains green through explicit prepare/execute phases: `100 -> 80 -> 64` without host reinjection.

## E6 transcript

With total supply 4, a precedence-0 request of 4 with exact basis 0 consumes zero capacity; the later precedence-1 serviceable claim receives all 4. The remand's mixed-band falsifier was planted before the remedy and produced `ResidentProductFailure`: with supply 10, precedence-0 A asks 100 at basis 0, precedence-0 B asks 1 at positive basis, and precedence-1 C asks 9 at positive basis. After the remedy, the equality band grants exactly 1 to B, A grants 0, and all remaining 9 fall through to C. Supplying an actual conserved in-flight holding of 3 reduces free supply to 1 in the original two-row case, so the later claim receives 1. CPU and resident settlement are bit-identical on the mixed band and follow `S_(k+1) = S_k - sum(G_i)`.

```text
E6 PASS
no-commitment=[(8,G0,U4),(9,G4,U0)]
mixed-band=[(7,G0,U100),(8,G1,U0),(9,G9,U0)]
in_flight=3 reserved=[(8,G0,U4),(9,G1,U3)]
law=S_next=S-sum(G)
```

No request, precedence integer, or class label acts as reservation authority.

## Mutant matrix

| Planted forbidden shape | Mechanical RED boundary |
|---|---|
| temporal `G + f(U)` | ordinary demand layout has no G field; canonical referee expects 8/5, not 12/9 |
| spatial generation increment | `dispatch_spatial` returns typed `SpatialGenerationMismatch` unless child generation equals parent generation |
| spatial parent-granter retention | resident live head returns typed `SpatialGranterRetained` unless the child granter changes |
| temporal mutable pseudo-`T_s` | temporal mint output is a distinct 4-word `ResidentTemporalDemand`, not the 8-word product ABI; structural source assertions pin the absence of product copy/write paths |
| copied parent claim identity in child market | descendant membership rejects source 8 in scope 8; lawful child source 9 occupies a distinct semantic row and product identity |
| prepared input executes early | changing only N+1-authoritative supply from 1 to 6 changes the eventual result from `(G=1,U=7)` to `(G=6,U=2)`; preparation precedes and cannot observe either value |

Focused execution is 5/5 green, including all RED-mutant assertions.

## Archaeology and changed authority

| Surface | Before remediation | Bound result |
|---|---|---|
| CPU/WGSL exact precedence | later capacity subtracted prior requested quantity, then the first E6 remedy still counted zero-basis requests inside a partially serviceable equality band | each equality band bounds execution by non-zero-basis rows and carries forward its exact granted total; the 100@basis0 sibling consumes zero while the 1@positive-basis sibling grants 1 and 9 falls through |
| reservation | request-shaped behavior was implicit in precedence arithmetic | only actual `ResidencyCapacityPartition.in_flight` reduces free supply before the one exact path |
| spatial recursive intake | retained parent granter/scope/claim identity and advanced generation | same-generation product-G view, changed granter, changed semantic scope, own descendants/rows |
| temporal recursive intake | copied product, modified U, then interpreted `G+f(U)` as a new request | once-mint reads immutable U and emits independently typed ordinary N+1 demand |
| N+1 lifecycle | recursive dispatch both prepared and executed using an N template | preparation and execution are separate; execution requires N+1-authoritative data |
| continuous row storage | market lanes shared cells across scopes | every admitted semantic scope owns distinct continuous and exact rows |

## Scope ledger and conformance

The implementation changes the existing resident clearing runtime, exact CPU/resident precedence construction, resident shaders, world-state forwarding, and the existing temporal-intake module in place. It updates the production growth-entitlement caller, the 14.6/15.2 frozen witnesses, the 15.5 referee, qualification fingerprint, inventory ledger, this report, the current-evidence row, and append-only anchor reach records.

There is one immutable product ABI, one continuous allocation producer, one exact apportionment, one Current-to-Next mint, and one demand vocabulary. No adapter, second carry, second exact path, canon rewrite, workflow/CI implementation edit, pointer movement, 15.6/15.7 work, graduation, merge, compression, or closeout is present.

The production qualification tuple is pinned at `0xbfc8db391f8cd256` after the remanded exact shader change. The full `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet` suite passes with the mixed-band remedy present. Frozen resident causal/tree-isolation, persistence, exact parity, theorem/census, filter closure, consequence ingress, oracle quarantine, pacing, qualification, authority, inventory, lifecycle, anchor, detachability, Agent Scan, and hosted Doctrine evidence are recorded in the exact-head PR relay.
