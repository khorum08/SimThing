# RESIDENT-CLEARING-CUTOVER-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED**. Coding has
not graduated Phase 14, moved the active pointer, stamped completion, merged,
or begun a 14.7 rung.

## Provenance and scope

- handoff: `handoffs/RESIDENT-CLEARING-CUTOVER-0.hd.md`
- handoff base: `8466434814f6b3a9ca16fea2287744a222a5d7e3`
- implementation base / orientation head: `a369472ae81b1b87012a0b370d4de95074b0ccc3`
  (the handoff base is its ancestor; `a369472a` carries the handoff)
- `HD-RECEIPT: 23255c21e9db`
- `ORIENT-RECEIPT: b33f21e9a53a`
- rule stamp: `4823f68eb8020637`
- exact reviewed head: the immutable PR/Board packet records the final branch
  head after this report is committed
- inherited production qualification fingerprint:
  `0x73ae5e621b3e5021`

This is a cutover over the frozen 14.2–14.5 substrate. It introduces no score,
pressure, recurrence, exact-apportionment, product, or feedback law.

## Archaeology and five-door disposition

| 14.1 symbol-keyed door | Pre-cutover role/caller | 14.6 disposition | Mechanical protection |
|---|---|---|---|
| `clear_constrained_claims_at_generation` | ordinary CPU clearer called by growth entitlement | vendorized CPU oracle only; production session caller removed | `CPU-CLEARING-ORACLE-DOORS=5`, `CPU-CLEARING-ORACLE-CALL-SITES=2`; no symbol in resident runtime |
| `clear_reduced_owner_channels` | generationless compatibility used by frozen tests | retained only for frozen CPU-oracle compatibility; no production caller | exact five-door census plus production-call-site census |
| `clear_reduced_owner_channels_at_generation` | CPU generation wrapper | vendorized CPU oracle wrapper only | exact five-door census |
| `clear_stamped_owner_channels` | stamped CPU binding used by germ/oracle witnesses | vendorized CPU oracle wrapper only | exact five-door census |
| `produce_runtime_rf_next_generation_demands` | CPU recurrence fixture/tooling door | vendorized CPU oracle tooling only; driver wrapper has no ordinary session caller | exact five-door and two-call-site censuses |

Archaeology classified the authority cutover, schedule live head, posture split,
CPU rehome, structural projection, and canonization as **existing substrate plus
wiring**. It found no semantic MISSING item.

## Production authority and posture matrix

The ordinary call graph is:

`SimSession::open` → `ClearingExecutionPosture::ResidentRequired` admission →
`SimSession::{step_once_into_summary,record_to_path}` → boundary growth closure →
`GrowthEntitlementMarketBinding::resolve_batch_resident` →
`ResidentClearingRuntime::dispatch` → graduated exact GPU apportionment → direct
GPU copies of the identical canonical `T_s` bytes to the resident schedule live
head and N+1 intake → queue submit → asynchronous `materialize` → sparse CPU
structural grant/`BoundaryRequest` consequence.

The alternate graph exists only after explicit selection:

`ClearingExecutionPosture::CpuVendorizedOracle` →
`resolve_batch_cpu_vendorized_oracle` → the unchanged frozen CPU clearer.
There is no error/failure edge from the resident graph to the CPU graph.

| Scheduling posture | ResidentRequired | CpuVendorizedOracle |
|---|---|---|
| `Paced` | lawful, default production | lawful, explicit oracle |
| `Continuous` | lawful production | lawful, explicit oracle |

The core and integration posture tests prove all four cells. Resident admission
captures the exact adapter/runtime/compiler/shader/workgroup/ABI tuple; a
mutated tuple returns typed `UnqualifiedAdapter` before dispatch. A custom
growth market lacking the resident qualification is rejected at install.

## No-readback, bounded replay, and structural boundary

`ResidentClearingRuntime::dispatch` encodes exact settlement and copies each
graduated `ResidentConstrainedProduct` from GPU scratch into both the reserved
resident live-head segment and the identical N+1 intake, then submits the queue
before returning a ticket. No buffer map, CPU grant reconstruction, schedule
`Vec` append, or replay drain occurs on that path.

The production referee dispatches tree A at generations 11 and 12 before it
materializes either batch. Their tickets name N+1 generations 12 and 13 while
the schedule's host entries remain empty. Multiple reservations coexist until
the admitted four-row segment fills. A further reservation against a three-row
fixture returns typed `ReplayEgressExhausted { requested: 4, capacity: 3 }`;
out-of-order observation returns `ReservationMismatch`; FIFO materialization
does not recycle the segment until all pending rows drain. The observer reads
the live-head copy, not reusable exact scratch. There is one
`IntegrationSchedule`, one resident live head, no drop/coalesce/overwrite, and
no second authoritative CPU history.

Economic continuation consumes the canonical resident product directly.
`record_resident_structural_grant` is a checked, one-way projection only for a
genuine sparse structural consequence after N+1 has already been submitted.
Structural tree mutation remains CPU-bound through the existing sealed
`BoundaryRequest` path; no new structural GPU executor exists.

## Independence and seam transcripts

The real-adapter production referee interleaves two independent resident
executors:

- tree A: realm `0x1406a`, generations 11 and 12, N+1 12 and 13;
- tree B: realm `0x1406b`, generation 29, N+1 30;
- both trees use overlapping local SimThing ids 7 and 8;
- plan buffer owners, resident schedules, live heads, and allocated-flow planes
  are distinct; there is no global clearing singleton, host-wide tree lock,
  raw-id cross-tree comparison, or all-tree barrier;
- result: `two_tree=PASS`, materialized rows A=4 / B=2.

The seam fixture serializes a realm-qualified canonical resident plan, drops
the source authority/context, recreates the same semantic tree under a separate
execution incarnation and residency allocation, then replays with the bounded
consumer envelope. The receiver binds the identical canonical bytes and digest
`282d80600a3bf20083716e64720302ed`; no source pointer, physical row, GPU
handle, or translated economic payload crosses the seam.

## Extended constitutional census

The existing constitutional authority now pins:

- exactly one production resident authority: `ResidentClearingRuntime::dispatch`;
- exactly five frozen CPU-oracle doors and two admitted driver oracle call sites;
- zero resident-runtime economic adapters/seam translators;
- zero CPU PALMA/Gu-Yang/private-field/descendant-pressure solver or clearing cache;
- zero resident-to-CPU duplicate settlement/feedback caller;
- zero process-global clearing singleton/lock/barrier shape.

`constitutional_surface_check.sh --check` reports all new expected counts and
all four forbidden-shape counts at zero. Its selftest passes 12 plants plus the
existing binding/census plants.

## Frozen 14.1 comparator

Procedure-of-record, unchanged:

`cargo test -p simthing-workshop --test generation_critical_path_baseline_0 --offline -- --test-threads=1 --nocapture`

The four tests passed on 2026-09-02. The generated report was read and then
restored byte-for-byte; the baseline was not re-blessed. These are dated,
non-gating CPU-oracle instrument measurements, not a resident performance
benchmark. They separate the requested legs honestly:

| 1,000,000-claim median | Frozen 14.1 before (ns) | 14.6-head observation (ns) | Delta |
|---|---:|---:|---:|
| enclosing CPU clear | 1,905,226,300 | 1,618,446,800 | -15.05% |
| CPU schedule/replay recording | 337,641,600 | 286,231,700 | -15.23% |
| lawful structural consequence | 246,443,000 | 196,003,000 | -20.47% |
| N+1 launch-delay instrument | 100 | 300 | +200 ns (timer-floor noise) |
| next-generation host re-clear | 1,778,242,200 | 1,686,655,500 | -5.15% |
| end-to-end instrument | 10,524,813,600 | 9,345,863,000 | -11.20% |

The unchanged instrument explicitly times the CPU oracle and has zero GPU
upload/readback legs. The production cutover ordering and residency claims are
therefore established by the 14.6 executor referee, not inferred from this
comparator.

## Section-8 amendment and anchor closure

Canonical section 8.3 now states the complete Phase-14 responsibility split:
resident-primary qualified clearing, explicit CPU oracle, identical `T_s`
recursive ports, receive-not-recompute, one bounded resident schedule live
head, sparse CPU structural boundary, and binding falsifiers. The six expiring
design-authority anchors were homed and repointed as follows:

| Anchor | Canonical home | Lifecycle |
|---|---|---|
| `rf-market-mirror-cycle` | §8.3.1 | `canonical` |
| `rf-market-receive-not-recompute` | §8.3.2 | `canonical` |
| `rf-market-port-census` | §8.3.3 | `canonical` |
| `rf-market-settled-code-census` | §8.3.3 | `canonical` |
| `rf-market-candidate-laws` | §8.3.4 | `canonical` |
| `rf-market-falsifiers` | §8.3.5 | `canonical` |

None still points at `docs/workshop/SimThing_RF_Market_Core.md`.
`anchor_check --check` reports `healthy=0 orphaned=0 stale=0`, curation PASS,
and the six `rf-market-core` queries resolve to canonical §8.3 prose.

The workplan status is intentionally **CONFORMANCE-IN-FLIGHT / coding
PROBATION**. Only DA graduation may return it to complete.

## Verification packet

- touched production/workshop packages `cargo check --all-targets` — PASS;
- qualification mutation — PASS 1/1;
- frozen 14.2 plan — PASS 3/3;
- frozen 14.3 score/bands — PASS;
- frozen 14.4 apportionment — PASS;
- frozen 14.5 terminal parity — PASS 1/1;
- 14.6 cutover referee — PASS 3/3, real adapter;
- ordinary growth-entitlement session seam — PASS 5/5, default resident path;
- frozen 14.1 exact comparator — PASS 4/4;
- constitutional census/check/selftest — PASS;
- doctrine anchor check/selftest/query — PASS;
- lifecycle inventory/check — PASS after registering all five new tests;
- `git diff --check`, Agent Scan, hosted Doctrine Scan/Exec, exact-head
  clearance, and relay lint are recorded in the final PR/Board packet.

Structural certificate: **ZERO-RED locally; hosted exact-head confirmation and
DA deep-tree review pending.**
