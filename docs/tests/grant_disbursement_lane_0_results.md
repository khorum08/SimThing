# GRANT-DISBURSEMENT-LANE-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2f
- Status: **PROBATION / proof-present / DA-review-pending**
- Exact implementation base: `54f7e952c2eb9af2f1f5fa73407f9967087d5f91`
- Branch: `codex/grant-disbursement-lane-0`
- Pull request: #1852
- tested_code_sha: `5b741a199cf1f1a235baddd2ff80a28edebbb6f9`
- Original dispatch: Board #1332 comment `5434146794`
- DA A2 / resume: Board #1332 comments `5434301294` and `5434562484`
- DA remand / ingress-seal repair: Board #1332 comment `5439031875`
- HD-RECEIPT: `29ad3459ddbd`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- orientation_digest_sha: `fd3af7f2803d257945d77841b7c61266f4882bd56009e5259b3e294c4918d350`
- Expected route: `DA-RESERVE(gate-wiring)`
- Structural-certificate baseline inherited for DA: 120 suites / 461 passed / 3 failed / 14 ignored

## Canonical fact and schedule vocabulary

The real `MarketGrantRecord` lifecycle doors project into one serde-capable
`GrantLifecycleFact`. Each fact carries its generation, stable relationship
provenance, exact before/after relationship quantities, granter and grantee
identities, every affected relationship, and an optional typed release cause.
Partition and transfer each remain one atomic multi-node fact.

Exactly six matching `IntegrationScheduleRowKind` variants were added because
all six lifecycle transitions must be scheduled for N+1 publication without
re-clearing: `GrantAccepted`, `GrantRenewed`, `GrantRevoked`,
`GrantPartitioned`, `GrantTransferred`, and `GrantReleased`. The append map is
one-to-one:

| Real lifecycle door | Typed fact / row | Recorded relationship change |
|---|---|---|
| `record_cleared_grant` | Accepted / `GrantAccepted` | one relationship `0 -> granted` |
| `renew_from_clearance` | Renewed / `GrantRenewed` | one relationship `q -> q + renewal` |
| `revoke` | Revoked / `GrantRevoked` | one relationship `q -> q - revoked`, cause `Revocation` |
| `partition_for_fission` | Partitioned / `GrantPartitioned` | one source relationship to every successor relationship in one fact |
| `transfer_for_fusion` | Transferred / `GrantTransferred` | every input relationship to one fused relationship in one fact |
| `terminate` | Released / `GrantReleased` | one relationship `q -> 0`, typed death/dissolution/explicit cause |

Rows are append-only events, not a uniqueness map. Two lawful renewals of the
same relationship in the same generation therefore retain two ordered
`GrantRenewed` facts (`2 -> 3`, then `3 -> 4`) with the same provenance. Their
single N+1 boundary batch aggregates to the exact final lane state while replay
retains and consumes both facts in recorded order.

The lifecycle signatures gained only the market, generation, and mutable
`IntegrationSchedule` inputs required to compute and append that canonical
fact. `SimSession` supplies its existing generation and its one schedule. No
market, clearing, generation, history, telemetry, manager, dispatcher, peer
executor, or rebind authority was added.

## Sparse lane birth and sole publication path

`simthing/grant-disbursement-capacity` is an ordinary Anchored property with
four exact named scalar roles: `free`, `in_flight`, `occupied`, and `capacity`.
The registry schema is optional; only authored granting-active nodes receive a
property value and one initial `Infrastructure/System` state overlay before
session open. An inactive sibling proves that neither property presence nor an
overlay is synthesized globally. A scenario with no such authored property is
inert even when its canonical schedule contains lifecycle facts.

The only live causal path is:

`real lifecycle door -> typed IntegrationSchedule fact at N -> BoundaryProtocol at N+1 -> schedule-derived SuspendOverlay/AttachOverlay requests -> existing boundary activation/GPU sync -> existing hot overlay writer and threshold scan`

The publisher reads the GPU-fresh boundary shadow, rehydrates the existing
`ResidencyCapacityPartition` exact judge, validates the complete fact batch,
then replaces the active full-state overlay through ordinary boundary requests.
Beside those exact requests it mints a boundary-local, non-public capability
covering the exact old overlay suspension and exact new overlay attachment.
The public `apply_structural_mutations` door has an empty capability; only the
ordinary `BoundaryProtocol` call carrying the schedule publisher's capability
can consume the protected transitions. It creates no numeric setter and no
side ledger. Generic feeder patches, player intents, AI intents, public
`FeederSender::submit_boundary`, and direct `TransformPatcher::apply_one` calls
all reject this property as schedule-owned. Persistent `Infrastructure` state
uses `UntilDissolved` and is explicitly suspended by the next canonical fact,
so it adds no post-open lifecycle-catalogue row and no O(capacity) structure.

## Exact six-kind conservation proof

The integrated witness starts every granting participant at
`[free, in_flight, occupied, capacity] = [20, 0, 0, 20]` and drives the real
doors in order:

| Fact | Authoritative published lanes after N+1 |
|---|---|
| Accepted `source 0 -> 6` | source `[14, 0, 6, 20]` |
| Renewed `source 6 -> 8` | source `[12, 0, 8, 20]` |
| Revoked `source 8 -> 7` | source `[13, 0, 7, 20]` |
| Partitioned `source 7 -> {left 3, right 4}` | source `[20, 0, 0, 20]`; left `[17, 0, 3, 20]`; right `[16, 0, 4, 20]` |
| Transferred `{left 3, right 4} -> fused 7` | left/right `[20, 0, 0, 20]`; fused `[13, 0, 7, 20]` |
| Released `fused 7 -> 0` | fused `[20, 0, 0, 20]` |

Every row sums exactly to 20. Positive deltas traverse the existing exact
`issue` then `deliver` transition; negative deltas traverse `release`.
`in_flight` therefore remains zero in the authoritative boundary result because
one recorded accepted relationship is atomically issued and delivered by its
single N+1 realization. Partition and transfer validate their full three-node
sets before emitting any request, so no partial multi-node publication exists.

## ActionBand and replay seals

The standing witness
`six_real_doors_publish_conserved_sparse_lanes_and_cross_actionband_without_rebind`
installs the source property, its one initial state overlay, the occupied-lane
threshold, ActionBand template, EML consequence, and resident `PropertyNext`
destination before open. An accepted generation-0 grant leaves the lane and
crossing unchanged in generation 0; the N+1 publication moves occupied from 0
to 6 and the graduated GPU threshold/ActionBand path reports exactly one rising
crossing. The admitted binding generation stays `Some(0)` while execution
advances to `Some(1)`: no refresh, rebind, `LateInstall`, or ingress-shape
weakening occurs.

Each boundary outcome copies the same scheduled fact into a
`BoundaryDeltaEntry::GrantLifecycleFact` before its ordinary overlay entries.
`ReplayDriver::try_apply_frame` derives the exact expected lane state and old
overlay identity from each contiguous fact batch, admits one matching
System/Infrastructure attachment and suspension, and applies the frame to a
clone before committing it. A forged protected `OverlayAttached` without its
fact returns `GrantLaneCausalBypass` with day, facts, base property, and overlay
count unchanged. An accepted-only prefix reproduces `[14, 0, 6, 20]` and the
one sealed band-crossing delta; the complete log consumes all six facts. Replay
never evaluates the market. A `shadow_values` checkpoint without delta entries
can restore presentation bytes but leaves the canonical fact collection empty
and therefore cannot mint the causal history.

## Named REDs and implementation repair

- Live second writer: an actual public `FeederSender::submit_boundary` carrying
  a forged protected `AttachOverlay` increments
  `boundary_grant_lane_authority_rejections` and changes neither lane values nor
  overlay count. Protected public suspend/activate routes are closed by the same
  boundary-local capability.
- Replay second writer: a protected `BoundaryDeltaEntry::OverlayAttached`
  without a matching fact-derived plan returns `ReplayError::GrantLaneCausalBypass`
  atomically, with zero mutation.
- Legitimate multiplicity: two real same-kind, same-generation, same-provenance
  renewals both append, publish to `[16, 0, 4, 20]`, and replay in exact order.
- Direct writer: a generic direct patch of the protected property increments
  `protected_grant_lane_write_forbidden`, performs zero writes, and leaves the
  row bit-identical. The same guard covers generic feeder/player/AI ingress.
- Replay re-clear: `attempt_grant_lifecycle_reclear_forbidden` returns the
  grant-specific extension of `ReplayGrowthError`.
- Checkpoint shortcut: a replay frame containing only `shadow_values` consumes
  zero grant facts.
- Same-generation pacing: a malformed schedule row with fact N and boundary N
  returns `SameGenerationPublicationForbidden`.

The generation-paced ActionBand witness also exposed a pre-existing uniform
alias: accumulator `_pad1` held both AO band count / owning generation and the
compact-velocity execution-mode discriminator. Generation 1 therefore selected
compact-velocity mode. The uniform and WGSL now carry distinct `generation` and
`execute_mode` words; the frozen ActionBand GPU and kernel batteries prove both
paths remain intact.

## Focused and frozen evidence

| Command / battery | Result |
|---|---|
| `cargo test -p simthing-driver --test grant_disbursement_lane_0` | PASS — 3/3 |
| `cargo test -p simthing-core -p simthing-sim` | PASS — all unit, integration, and doctest harnesses; only pre-existing warnings/ignored perf witness |
| ActionBand GPU execution + overlay actuation | PASS — 5/5 + 2/2 |
| frozen 11.2a/b/c driver batteries | PASS — 4/4 + 1/1 + 5/5 |
| frozen 11.2d allocator retirement | PASS — 2/2 |
| full `simthing-embedder` / Vendor Door | PASS — 13 integration tests + 5 compile-fail doctests |
| deterministic matrix | PASS — 2/2 |
| sim pacing unit + protected representative restore | PASS — 1/1 + 2/2 |
| kernel `accumulator_op` focused units | PASS — 4/4 |
| `cargo check --workspace` | PASS |
| `cargo fmt --all -- --check` / `git diff --check` | PASS |
| inventory check / drift check | PASS — exact 1,347/1,347; zero unledgered, stale, or parked rows |
| lifecycle `--schema` | PASS — zero expired or audit candidates |
| `agent_scan.sh` exact PR delta | PASS — zero hard failures and zero inspect flags |
| whole-tree `doctrine_scan.sh` | exit 0 — zero hard failures; 417 pre-existing whole-tree heuristic INSPECT findings, none introduced by this delta |

Hosted workflow IDs and the exact final evidence head are returned on Board
#1332 after the final probation evidence commit is pushed and all jobs settle.

## Fences retained

- One `IntegrationSchedule`, one `BoundaryDeltaEntry` history, one session
  generation, and one schedule-minted boundary-local publication capability.
- No re-clear during replay, checkpoint-derived fact, direct setter, shadow
  ledger, peer writer, telemetry authority, or ActionBand rebind.
- 11.2a-e clearing, provenance, two-stage placement, allocator retirement,
  install-only door, and five-verb Vendor Door semantics remain frozen.
- No 11.3 implementation, 11.4, 12.x, Vector CostBand, or ClauseThing-red work.
- Pointer remains 11.3. Coding does not merge, graduate, or move it.
