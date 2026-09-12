# 0088-STUDIO-SLICE-0 C1a — source-to-ready instrument

## Status

PROBATION / proof-present / OPEN / UNMERGED. Return to Orchestration only.
Board dispatch 5647569540 accepts the instrument gap in STOP 5647547283 and
defines this implementation of the existing M15 denominator. No novelty claim.

## PR / branch / merge

- Branch: `codex/0088-studio-source-to-ready`.
- Base: `6f74d12014e4af4c7cb1e4c92c7d58bc3cfb68bf` (#2048 merged).
- Final tested head, hosted scan and fresh router verdict are recorded in the PR
  body and Board coding return. No self-merge or rung graduation.
- HD: `handoffs/0088-STUDIO-SLICE-0.hd.md`, receipt `b22c5f735522`.
- Carried ORIENT-RECEIPT: `28f56884d309`; required anchor payloads unchanged.

## What changed

The accepted attempt's existing `begin_load_attempt` call now retains one
`Instant` and the selected source path with the existing token. The ordinary
loader's final scene-batch branch uses `try_adopt_clause_load_attempt`, which
checks that token, performs the existing real resident-admission transaction,
then immediately captures the ready endpoint. Publication consumes the pending
timestamp, so an attempt produces at most one successful sample.

The duration includes worker work, worker-to-main-thread delay, scene staging and
the actual resident admission. It excludes the later reveal frame / first
display. It is not a sum of stage durations. Existing SceneAdopt timing is
unchanged; Studio_ops explicitly labels its exclusion of resident admission and
reveal.

`StudioScenarioLibraryModel.source_to_ready` exposes the completed duration,
retained process-local monotonic start, attempt token, captured source path, bound source/profile identities and
dependency identities copied from the admitted result's existing provenance.
Unavailable provenance stays unavailable; no replacement digest is computed.
Studio_ops displays the duration/token/source/source identity/profile identity.
The completed record survives normal reveal/closure until a new attempt or
library opening resets it. Pre-admission cancellation/failure publishes nothing;
superseded and repeated completion cannot publish or replace a current sample.

## Load-bearing proofs

### Falsifier first

At the unmodified base, a disposable initial version of the owning test selected
the shipped source, called `begin_load_attempt`, executed the ordinary picker,
and called the existing `try_adopt_loaded_scenario_session`. It verified a real
attached resident at G0 and the still-current attempt, then inspected the
existing loader readout for a completed source-to-ready sample. RED:

```text
real admission succeeded but no completed source-to-ready duration exists
test result: FAILED. 0 passed; 1 failed
```

The final owning test is
`rehearsal_studio_source_to_ready_tracks_successful_resident_admission` in
`crates/simthing-mapeditor/tests/rehearsal_studio_source_to_ready.rs`.
It uses the ordinary staged picker on a worker and the same token-bound
admission endpoint the UI calls. Two 25 ms delays exist only in the test, before
main-thread admission and before subsequent attempt finish/close. There is no
production delay hook or substitute Bevy application.

| Obligation | Owning evidence |
| --- | --- |
| Only real successful resident admission publishes | No sample after worker completion; successful transaction installs a real G0 bridge, publishes one record, and that resident can execute a tick. |
| Worker/queue/staging interval included; reveal excluded | The retained start must lie inside the original accepted-attempt bracket, so slow admission cannot hide a restarted timer. Monotonic brackets also bound the elapsed duration; the deliberate pre-admission delay is included. Sample exists while attempt remains active, and a delayed finish/close does not change it. The test exercises the controller boundary, not a rendered first-present claim. |
| Failure preserves prior valid state | A cloned invalid candidate targets `missing::resource`; admission refuses, no timing appears, and prior document, settings, execution identity, anchor rows, cached readout and tick count remain unchanged. |
| Correct token / exactly once | Cancelled, superseded and duplicate attempts return without adoption or timing publication. An older completion before or after a newer success cannot change the newer record. |
| Monotonic, bound provenance | `Duration` is positive in this real load and lies between monotonic brackets; source/profile identities match the shipped pin and dependency map matches the native cache. |
| No workload or stage reinterpretation | All three shipped bundle files remain byte-identical; admitted authored profile and historical stage records remain unchanged. |

The invalid candidate is an isolated refusal fixture. The shipped workload is
never edited. The test's printed duration is diagnostic evidence of endpoint
wiring, **not a release Studio baseline sample**.

### Endpoint mutation check

A disposable mutant moved `record_source_to_ready` before the real admission
call and projected provenance from the candidate. The owning test became RED at
`failed admission must never publish success` (0 passed / 1 failed). The exact
good UI source was restored from its saved copy and checked by SHA256 before
final verification. This falsifier distinguishes successful resident readiness
from merely reaching the admission call; there is no rollback that hides an
early success publication.


The first post-restore run still executed the mutant artifact: `Copy-Item` had
preserved the older source mtime, despite a byte-identical good-source restore.
That RED is retained separately. Advancing only the restored file's mtime forced
Cargo to rebuild the good owner before the final referee run; source bytes and
workload were unchanged. No stale-build run is counted as final verification.
### Verification

Final local verification PASS: cargo check and formatting; owning instrument referee 1 PASS, prior profile-identity referee 1 PASS, existing focused Mapeditor ingress battery 9 PASS (11 total, 0 failures). All five prior-profile opens reproduce the exact source/profile pin. Local and hosted scan details and the fresh router verdict are bound to the final head in the PR/Board return.

```text
cargo fmt -p simthing-mapeditor -- --check
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test rehearsal_studio_source_to_ready -- --nocapture
cargo test -p simthing-mapeditor --test rehearsal_ingress_profile_identity -- --nocapture
cargo test -p simthing-mapeditor --test rehearsal_ingress_native_rf --test rehearsal_ingress_source_cache --test rehearsal_ingress_literal_successor --test rehearsal_ingress_fidelity
git diff --check
bash scripts/ci/agent_scan.sh --base 6f74d12014e4af4c7cb1e4c92c7d58bc3cfb68bf --head <final-head>
```

Owning test and existing profile referee use the same source identity
`fnv1a64:ee4e4df9e8c9fbd9:5798` and profile identity
`fnv1a64:bfcbc44323b304bf:23530`. Existing dependency identities remain
`fnv1a64:c49f9ca3c8c75e77:20370` and `fnv1a64:2f064bfb3e043aa0:72`.
Owner host: i9-13980HX / RTX 4080 Laptop GPU, Windows 11; rustc 1.95.0,
`x86_64-pc-windows-msvc`. Referee runs use the existing dev/test profile. C1
measurement will separately qualify the release workload and report its full
measurement tuple after Orchestration merges this instrument leaf.

## Scope Ledger / Conformance

- Production owners: `src/app/ui.rs`, `src/studio_scenario_library_ui.rs` in
  Mapeditor. One owning integration test, this packet, and one test-inventory row.
- Expected route: `ORCHESTRATOR-CLEARABLE(rehearsal-studio-presentation)`.
- Test lifecycle: AUDIT / behavior-regression / ledger-only, born in
  `0.0.8.8-integrated-rehearsal`, DSU 0. It catches the escaped absence and
  premature publication of a timing endpoint that no type or admission refusal
  previously checked. Default deletion at track closeout; no permanent proof.
- No scenario, engine/spec/ClauseThing semantics, persistence, scheduling,
  observation authority, Cargo, class/gate/census/anchor or sealed/E8 change.
- No measurement denominator or service-line change; no frame/input/freshness
  instrument and no generic telemetry framework.

## Known gaps

C1 baseline collection remains separate work. First presented display, raw
frame/input p95 and wall-clock observation age remain unavailable through the
current instruments. Generation-labelled readouts and existing stage timings
keep their existing meanings. Generation-N persistence RED remains carried to
2.3, outside this leaf.

## Graduation routing

Risk class: semantic (presentation telemetry). Falsification: base absence and
premature endpoint mutation, followed by final owning/ingress/profile referees.
Final local/hosted scan, complete INSPECT list and fresh `/clearance` verdict
are supplied in the PR body and Board return. Recommended posture: Orchestration
review of this bounded instrument repair; PROBATION, OPEN, UNMERGED.


