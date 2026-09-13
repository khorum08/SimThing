# M16 B1 UI comparison — evidence collection in progress

Status: PROBATION / physical comparison pending / OPEN / UNMERGED.
Dispatch: Board 5654699942; DA endpoint ruling 5651557467; Owner loop 5653843173.
Base and frozen runtime: `a8c117313dbe3796aa160ba12cad1cd677bdc5e6`.
Branch: `codex/0088-m16-ui-comparison`. HD-RECEIPT: 47010c34edf2.
Carried coding ORIENT-RECEIPT: 28f56884d309.

This draft freezes collection and analysis before Owner data. It is not a
completed comparison, proof-present PR, budget pass or migration recommendation.
The runtime contains the merged Leaf A and B0a instruments. No Mapeditor
production code, renderer, dependency or simulation authority changes are made.

## Frozen matrix and quantity boundaries

The same Meridian source is used for both clients: source
`fnv1a64:ee4e4df9e8c9fbd9:5798`, profile `fnv1a64:bfcbc44323b304bf:23530`,
base JSON `fnv1a64:c49f9ca3c8c75e77:20370`, dependency manifest
`fnv1a64:2f064bfb3e043aa0:72`. Seven systems, six links, two owners, two sites,
four cohorts per site, thirteen energy RF participants, two recipes and four
material loci. No workload variant or large-list scaling claim is introduced.

Five conditions each have three paired repetitions (30 captures). Pair order
is Egui/Native, Native/Egui, Egui/Native. Captures target at least 15 seconds;
the fixed minimum usable duration is 10 seconds. All raw samples and rejected
attempts remain. Owner waits at least five seconds after setup before F9;
that warmup is operator-qualified, not independently timestamped.

| Condition | Fixed semantic operation | Physical capture status |
| --- | --- | --- |
| Paused | Pinned G0, 1x/TPS10, steady visible pane | PENDING, 0/6 |
| Running | Reload G0 before each repetition; Play 1x/TPS10; warmup; F9; F10 before Pause | PENDING, 0/6 |
| Numeric editing | Paused G0; three Apply TPS pairs 12.5 then 10 through the selected client | PENDING, 0/6 |
| Resize | Paused G0; three OS maximize/restore cycles with the same starting/ending size | PENDING, 0/6 |
| List interaction | Same selected-system sequence 1 through 7; map picking for Egui, native list/Down for Native | PENDING, 0/6 |

The Owner launcher uses the existing Windowed setting and 1600x900 startup
size in a copied, process-scoped settings directory. Other copied presentation
settings remain unchanged; runtime physical size and scale are recorded. The
resize condition changes geometry through existing OS controls and requires
the same observed size sequence on each side. The list group uses the same
Overhead view on both sides. These are named B1 presentation conditions;
they are not falsely described as the 1920x1080 B0b capture's window condition.

Native-on retains the egui acceptance pane. Whole-frame/process measurements
characterize coexistence overhead, not an isolated replacement client. Map
picking and a native scrolling list are different adapters over the same
selection owner; no equal-widget construction-cost claim is made.

| Quantity | Instrument and reporting rule | Current evidence |
| --- | --- | --- |
| Raw full-frame cadence/tails | Merged B0a `Time<Real>` unsmoothed intervals; all raw samples; p50/p95/p99/max; nearest rank; p95 <=16.7 ms characterization | PENDING |
| Shared CPU projection | Same `shared_clock_and_observation_projection` computation; host wall elapsed, not OS CPU accounting | PENDING |
| Adapter CPU | Each named native/egui scope separately with its existing boundaries; never sum nested scopes | PENDING |
| Observation freshness | Publication to first matching client consumption, full scene/resident/generation stamp; pre-capture publication ages retained separately | PENDING |
| Process memory | Read-only PID private bytes and working set immediately after durable raw export, before analyzer; process total including capture/export allocation history | PENDING |
| Input latency | INSTRUMENT-INVALID / UNAVAILABLE; no qualified endpoint; p95 <=100 ms UNEVALUATED | Existing B0b/R1 failure proof |
| First actual display | No qualified matching displayed endpoint | UNAVAILABLE |
| Direct render/GPU cost | No directly admitted per-client timing instrument | UNAVAILABLE |
| Separate simulation/presentation GPU memory | No direct separately measured allocation pair | UNAVAILABLE |

Same-publication freshness residuals and, when both calls fall in one actual
recorded frame with unchanged publication and runtime state, shared CPU
projection residuals use Native minus Egui and retain negative values. Other
capture statistics/memory are paired by condition/repetition. Any chronological
frame-ordinal pairs are explicitly descriptive independent-run pairs, with
unmatched tails retained, never a causal frame association or latency join.
Within-capture publication freshness is separate from the age of a paused
publication that predates F9. No smoothed FPS, residual GPU estimate or
submission/prepare timestamp substitutes for a requested quantity.

## Required unavailable input-latency evidence

DA 5651557467 defines app input ingestion QPC to the first matching present
reported DISPLAYED by Windows. The latency quantity remains required. The
physical run `ovl-20260913-163144-1436aab9` had six inputs per client and the
unchanged 0,4,0,4,0,4 sequence. Containment found zero joins among 2,481 Egui
and 1,427 Native app spans (Board 5654603968).

R1, authorized by Board 5654621290 and returned in 5654672574, tested strictly
`S_i.end_qpc < QPCTime < S_(i+1).begin_qpc` on unchanged raw evidence:

| R1 diagnostic | Egui | Native |
| --- | ---: | ---: |
| Adjacent span pairs | 2,480 | 1,426 |
| Empty / unique / ambiguous windows | 1,240 / 620 / 620 | 713 / 143 / 570 |
| Reused rows | 0 | 0 |
| Unmatched in-domain rows | 1,860 | 1,283 |
| Credited response endpoints | 0/6 | 0/6 |

PID, chain and runtime were consistent. Egui's six local response candidates
did not establish the full bijection. Five Native response windows were empty;
the other had two candidates. Final spans were uncredited. K falsifier NOT RUN
because the endpoint prerequisite failed. No additional join heuristic or
rerun of the invalid instrument is part of B1.

| Preserved artifact | SHA-256 |
| --- | --- |
| PresentMon 2.5.1 x64 tool | `9bec3083069f58f911e6a512f4806db51a27bd096103087bc1d05ef54c80a191` |
| B0b process-filtered CSV | `9a4cc51f441f483951f8786cd4247eb8c6b54de4faf72ddd268fc1d801c54e51` |
| B0b Egui raw JSON | `1a89f58a9e88c447e5ef51c00c65325e0c686531ef360c5fd9c86b84bd54e058` |
| B0b Native raw JSON | `5e6db8cac454944d9f9d6dc0ce2cae6b081fe61556e6f4c31c9d38882dc03dbb` |
| R1 detailed diagnostic | `cf07561c1387d730bb107cf64fdfda05e1cefed61ed7dea62be6d23a2c6a412a` |
| R1 diagnostic script | `a1741218e9e73feafe90d4fb619c95dde5981096689eb75a5a5164178f46fedc` |

Original evidence remains in `SimThing-0088-OVL-B0b/runs/ovl-20260913-163144-1436aab9`
and the common gitdir's `0088-m16-b0b-r1-causal-successor` packet. All original
31 raw-index entries were verified in R1. PresentMon source is the Intel
GameTechDev/PresentMon v2.5.1 release. B1 does not extend that failed run's
result to a newly tested window geometry; latency stays unavailable/unevaluated.

## Behavior, decision and remaining work

Leaf A's bounded numeric editing and shared command/observation ownership are
carried as established semantic/desktop evidence. Native desktop Down,
seven-row traversal and the named maximize/restore cycle are explicit Owner
checks here. General text accessibility, arbitrary dragging/resizing, large
lists, changed DPI and multiple monitors remain unqualified unless separately
demonstrated. Synthetic dispatcher tests are not physical qualification.

No migration decision is issued in this pending-data draft. Required latency
is unavailable, so a positive migration recommendation cannot be supported
by treating it as passed or optional. The final single decision will bind the
collected behavior/cost results and that uncertainty. No dependency, Bevy
upgrade, present-path change or UI migration is implemented.

The external Owner bundle, exact executable hash, compiler/build log, machine
qualification, case configs, raw JSON, memory snapshots and per-capture reports
will bind into the completed packet. Current preparation checks exercise the
external analyzer's refusal semantics and process/logging/serialization behavior;
they are not physical M16 results. Local proof, hosted Doctrine, complete INSPECT
and fresh Clearance on the final evidence head remain required before handback.
All collection/reporting scaffolding is rung-local and retires with the 1.3
experiment. No protected test, lifecycle row, gate, census or class is edited.
