# 0088-STUDIO-SLICE-0 C1b — Studio opening baseline

## Status

PROBATION / proof-present / OPEN / UNMERGED; return to Orchestration only.
This is the **1.2 opening baseline**, not Phase-3 characterization, a threshold
verdict, or tuning evidence. Dispatch: Board comment 5648327947. Route:
`rehearsal-measurement`; novelty_claim: NO. The accepted generation-N persistence
RED remains carried to 2.3; this run did not reopen that probe.

HD: `handoffs/0088-STUDIO-SLICE-0.hd.md`; HD-RECEIPT `b22c5f735522`.
ORIENT-RECEIPT `28f56884d309`; measurement anchor
`rehearsal-0088-measurement@8d639c581ade`.

## Revision and instrument

- Capture code and instrument revision / branch base:
  `1caf5219ded9e81947a3b880f9b5b748c2505a8b` (merged C1a #2049).
- Evidence branch: `codex/0088-studio-baseline-c1b`. Final evidence head, local
  checks, hosted Doctrine artifact and fresh router verdict are bound in the PR
  body and Board return. The evidence delta changes this Markdown packet only.
- Actual application: `C:\Users\mvorm\SimThing\target\release\simthing-studio.exe`.
  SHA256: `7215211f700d0e9795aceace538b55e5bbf02c80404979b0cd846e1a44cd7284`.
  The executable hash was identical before and after the series.
- Build command, from `C:\Users\mvorm\SimThing`:
  `cargo build --release -p simthing-mapeditor --bin simthing-studio`.
  PASS, 3m36s; 13 existing Mapeditor warnings. Studio itself reports
  `Build: release/optimized`. No test harness or substitute application was used.
- M15 source-to-ready: one monotonic interval from accepted ordinary Clause load
  attempt to successful real resident admission. Worker work, queue delay, scene
  staging and admission are inside; subsequent reveal/first display is outside.
  This is the merged instrument, not a sum of stage timers or UI automation time.
- Instrument owners: `crates/simthing-mapeditor/src/studio_scenario_library_ui.rs`
  (`begin_load_attempt`, `record_source_to_ready`) and `src/app/ui.rs`
  (`try_adopt_clause_load_attempt`, Studio_ops display). C1a's endpoint/refusal
  proofs remain in `rehearsal_studio_source_to_ready_0_results.md`.
- The available raw output is the displayed duration at 0.001 ms granularity.
  Internal `Duration` precision / effective clock resolution is not claimed.
  Procedure UTC timestamps below are not interval endpoints or latency samples.

## Bound workload and dependencies

Every repetition displayed the same source and authored profile, accepted a real
resident and produced a native loader cache with the same dependency map and an
empty operator resolver. No pin mismatch occurred. SHA256 checks before and after
the series confirmed all three shipped bundle files remained byte-identical.

| Identity | Value (all repetitions) |
| --- | --- |
| Source | `scenarios/stellaristhing_base.clause` |
| Source identity | `fnv1a64:ee4e4df9e8c9fbd9:5798` |
| Authored profile identity | `fnv1a64:bfcbc44323b304bf:23530` |
| `stellaristhing_base.base.json` dependency | `fnv1a64:c49f9ca3c8c75e77:20370` |
| `stellaristhing_base.dependencies.json` dependency | `fnv1a64:2f064bfb3e043aa0:72` |
| Clause SHA256 | `5821be96f0bfb439ce8717dba7dd280e994ae6a1113bff42d3137766031df328` |
| Base JSON SHA256 | `2b207255ac213398cc2eecb21ae70f2411af82f535eb1f7f86fad83032d21360` |
| Dependency manifest SHA256 | `7c626d606cf8059513d21549a5e682d8243f2112b5df586bed08fa2edadfbf87` |
| Loader cache format / SHA256, identical for all three copies | `simthing-clause-cache-v1` / `5fcaf0f60e83370f73da4dcfd0852a09b956b1f2ebea5d4e0a5d45133d8ecae7` |

UI source path was the canonical Windows path
`\\?\C:\Users\mvorm\SimThing\scenarios\stellaristhing_base.clause`.
The ordinary loader persisted and admitted the sibling
`stellaristhing_base.from-clause.simthing-scenario.json`; this is the native
loader cache, not a separate authored scenario. The UI reported
`sibling/canonical source_base`, empty operator resolver and
`StructuralRebindReady session hydrate PASS`. Cache source_path was relative
`stellaristhing_base.clause`, with exactly the two dependencies above.

| Cardinality / workload property | Evidence and limits |
| --- | --- |
| Seed / placements / spatial extent | Authored static profile seed 0; 7 systems, 6 links; UI grid 6x2, 7 occupied. No MapGenerator execution. Saved generation controls mention seed 42 / 1500 stars; those controls were not invoked and are not this workload. |
| Ownership / sites / cohorts | UI: 2 Owners. Source: A1 and E1, 4 Cohorts each. Spatial residency, energy RF parentage and ownership are distinct authored relations. |
| Energy RF | UI: `ConvergedArenaResourceFlow / active`, arena `meridian_energy`, named child SIM-000256, real ancestor SIM-000280 / 2 siblings. Source: 13 energy participants (root + 2 Owners + 2 sites + 8 Cohorts). This authored count is not an independently exported runtime install/channel counter. |
| Material recipes / quantity loci | Source: 2 local mineral-to-alloy recipes and 4 named site/resource quantity loci (A1/E1 minerals/alloys). UI exposes all 4 generation-labelled quantities. |
| Distinct physical destination fields / admitted channels / target eligibility edges | UNAVAILABLE as runtime counters in this capture; authored loci/participants above are not substitutes. |
| Pending structural products / retained work-in-progress | No construction campaign was authored or initiated. UI construction need is unbound (no admitted GameMode binding), threshold/live values absent and crossings 0. Full runtime product/WIP cardinalities UNAVAILABLE; absence of a UI binding is not proof of global zero. |
| Active versus reserved instance rows | UNAVAILABLE as runtime counters. |
| Selection / subscriptions | No selected system; UI selected-system id `--`. Exact active subscription count UNAVAILABLE; the left live observation and telemetry panes were opened as described below. |
| Simulation versus presentation GPU memory | Separate measured totals UNAVAILABLE. Run-3 UI tracked-asset estimate: 4.4 MB textures, 0.0 MB meshes, 0.0 MB Studio-tracked buffers. Render targets/swapchain and bloom/postprocess intermediates explicitly untracked. This is not total VRAM or a simulation/presentation partition. |
| Other current UI state | STEAD valid, RF ready, GPU index ready; no fleet icons in live presence snapshot; decision events last/cumulative 0/0 at the recorded generation views. No movement/combat/scale campaign was run. |

## Reference machine, build and presentation qualification

Owner reference machine: MSI Vector 16 HX A13VHG; Intel Core i9-13980HX,
24 cores / 32 logical processors; 31,897,579,520 bytes physical RAM.
Windows 11 Home 10.0.26200, build 26200. Inventory captured 2026-09-12
20:03:58 UTC, before the series.

Run-3 Studio telemetry directly identifies NVIDIA GeForce RTX 4080 Laptop GPU,
DiscreteGpu, vendor/device `0x10de/0x27a0`, Vulkan; exact adapter policy satisfied.
Windows driver `32.0.15.9579`; `nvidia-smi` after the series reports driver
`595.79`, 12282 MiB device memory. Intel UHD driver `32.0.101.6129` and Parsec
virtual driver `0.45.0.0` were also installed. The selected adapter/backend was
photographed in run 3, not independently photographed in runs 1 and 2; all runs
used the same machine, executable and unchanged selection/presentation settings.

Rust: `rustc 1.95.0 (59807616e 2026-04-14)`, full commit
`59807616e1fa2540724bfbac14d7976d7e4a3860`; host
`x86_64-pc-windows-msvc`; LLVM 22.1.2. Cargo:
`1.95.0 (f2d3ce0bd 2026-03-21)`. Workspace release override `lto = "thin"`;
otherwise ordinary release profile, no added flags/features. No shell
RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, CARGO_PROFILE_RELEASE*, CARGO_BUILD*, WGPU*
or BEVY* overrides. Ambient shell `RUST_LOG=warn` was unchanged; the Sky launch
helper's inherited environment was not separately dumped. Shader compiler
revision inside the driver is UNAVAILABLE; driver/backend versions qualify it.

Actual UI: borderless fullscreen 1920x1080, render scale 1.00, Fifo present mode,
MSAA 8x, 3D, BloomStarburst, stars and hyperlanes enabled; no tuning/isolation
toggles changed. Initial camera yaw/pitch/distance 0.6/0.55/95.0, target (0,0,0).
Nameplate relative width/base transparency/falloff distance/falloff transparency:
40/60/53/0 percent. Persisted settings were retained locally. Window and pane
navigation is part of the procedure; there was no fixed pre-load dwell.

## Exact launch/load procedure and cache treatment

1. Build completed before measurements; no concurrent Cargo build or scan during
   the three loads. Ordinary desktop applications remained open. No claim of
   global process quiescence, controlled thermals or fixed power state.
2. For each repetition, verify the previous Studio process has exited, then call
   the native application launch API
   `sky.launch_app({app:'C:\\Users\\mvorm\\SimThing\\target\\release\\simthing-studio.exe'})`.
   No command-line arguments, custom application wrapper or alternate loader.
3. Observe empty Studio, paused at tick 0. Open Library, Select File, then select
   `stellaristhing_base.clause` in the ordinary native file picker. Verify the
   selected path and press Load exactly once. Each fresh process used attempt 1.
4. After successful load, open Telemetry / Show Studio_ops Telemetry and close
   the covering Performance pane to read the completed source-to-ready record.
   Verify source/profile, resident path and paused tick 0; retain the screenshot,
   every displayed sample and the ordinary loader cache. Close the process after
   repetitions 1 and 2. No failed/cancelled/stale load was counted as a sample.
5. Only after run 3's G0 sample was retained, press Play at 1x / Max TPS 10;
   retain a tick-2 view, then Pause (actual tick 2206 when acted upon). Capture
   generation-labelled resident state and GPU/performance context, then close
   Studio. This later simulation work is not included in the load samples.

Three fresh processes, three successful accepted attempts, **no discarded
warmup loads or samples**. App in-memory state is fresh per process. Before
run 1 the sibling generated loader cache did not exist. Before runs 2 and 3 it
existed with the previous successful run's bytes. The ordinary Clause path
parses, persists the cache and re-ingests it for each load; all three persisted
copies were identical. OS file, driver and shader caches were not flushed or
quantified: this is an ordered ordinary-cache series, not a cold-cache claim.
The first slower result is retained without attributing its cause. No settings
or scenario edits were made between repetitions. After capture the generated
untracked cache was removed only after verifying it matched the retained copy;
it is not part of the evidence PR's scenario surface.

## Raw source-to-ready samples

Times are the actual Studio_ops readouts in milliseconds. All source/profile
and dependency identities are the common tuple above, verified for each row.
Process starts are local UTC-05; action times are UTC on 2026-09-12 and identify
procedure order only. All three resident admissions succeeded on the production
`simthing_driver::SimSession::open_from_spec + step_once` path, Field-bearing,
path preference `auto`, Meridian Arm / `stellaristhing_base`.

| Order | PID | Process start (UTC-05) | Load action (UTC) | Attempt token | Source-to-ready ms | Resident at retained sample |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | 15424 | 15:07:18.473598 | 20:09:33.708 | 1 | **1142.018** | admitted, G0, paused, executed 0 |
| 2 | 36176 | 15:11:51.367891 | 20:13:04.662 | 1 | **398.252** | admitted, G0, paused, executed 0 |
| 3 | 38476 | 15:14:35.807330 | 20:15:50.460 | 1 | **371.545** | admitted, G0, paused, executed 0 |

Additional descriptive summary, n=3: mean **637.272 ms**, median **398.252 ms**,
minimum **371.545 ms**, maximum **1142.018 ms**, sample standard deviation
**437.327 ms** (n-1). These describe these three retained observations only;
no tail-latency/service-line verdict or causal cache comparison follows.

Historical stage displays retained for completeness, all ms and all `passed`:

| Stage | Run 1 | Run 2 | Run 3 |
| --- | --- | --- | --- |
| Resolve | 0.069 | 0.062 | 0.066 |
| Parse | 0.151 | 0.133 | 0.114 |
| Hydrate | 0.623 | 0.422 | 0.413 |
| Rebind | 0.043 | 0.037 | 0.033 |
| Persist | 1.238 | 1.209 | 1.715 |
| Session build | 1.560 | 1.384 | 1.544 |
| Projection | 0.973 | 0.912 | 0.833 |
| Scene adopt | 80.224 | 79.865 | 77.838 |

These are diagnostic stage readouts, not a replacement denominator. Scene adopt
explicitly excludes resident admission and reveal. No overlapping stage sum,
subtracted residual, or conversion to first-present time is used.

## Separate running-resident observations and unavailable instruments

| Generation | Transport | A1 alloys | A1 minerals | E1 alloys | E1 minerals |
| --- | --- | --- | --- | --- | --- |
| 0, all three retained load samples | paused | 4.000 | 3.000 | 3.000 | 3.000 |
| 2, later run-3 view | playing | 7.000 | 3.000 | 6.000 | 3.000 |
| 2206, later run-3 view | paused | 3313.000 | 3.000 | 3312.000 | 3.000 |

Clock/bridge/executed generations agree in those views. These establish a real
advancing resident with generation-labelled material observations. They do not
measure simulation throughput, wall-clock observation age or first-useful latency.

| Quantity | Current instrument status / limit |
| --- | --- |
| Accepted source load to resident ready | AVAILABLE, merged C1a interval; all 3 raw success records above. |
| First presented display / first useful observation wall-clock latency | UNAVAILABLE: no current direct endpoint pair captured; neither source-to-ready nor screenshot save time substitutes. |
| Raw steady-frame p95 | UNAVAILABLE: `studio_frame_phase_gpu_telemetry.rs` reads smoothed diagnostics, then exposes last/running average. No raw frame series or steady window was captured; no verdict against 16.7 ms. |
| Input-event-to-response p95 | UNAVAILABLE: measured system execution costs are not event/response latency; no verdict against 100 ms. |
| Wall-clock observation age / freshness | UNAVAILABLE: `studio_live_observe.rs` generation labels lack paired publication/observation wall-clock timestamps. |
| Host submission / GPU interval / synchronization / readback / signed samplewise residual | UNAVAILABLE as separate, paired causal measurements here. UI reports render sub-app timing unavailable in this build; the existing clamped unexplained-frame estimate is not a signed samplewise residual. |

For instrument-status transparency only, the later paused run-3 screenshot
displayed FPS 149.6, Frame total 38.89 / 26.35 ms, Main Update 0.10 / 0.38 ms,
Egui/UI 0.14 / 0.21 ms, instrumented render-loop last 0.02 ms, unexplained frame
estimate 38.63 ms. These are independently updated diagnostic readouts; no
reciprocal-FPS calculation, steady-state assertion, stage sum or service-line
comparison is made from this snapshot.

## Six-field measurement status (§5.7)

| Row | Instrument validity | Current-path characterization | Historical comparison | Candidate release | Candidate execution and result | Inherited budget/debt disposition |
| --- | --- | --- | --- | --- | --- | --- |
| Source-to-ready | PASS for merged C1a denominator; all pins and successful admissions verified | 1.2 opening baseline, n=3; Phase-3 characterization remains open | NOT PERFORMED; old SceneAdopt has a different denominator | None proposed | No candidate executed; baseline raw results above | No source-ready threshold invented; later application characterization remains with Phase 3 |
| First display / useful observation | UNAVAILABLE direct instrument | UNAVAILABLE wall-clock characterization | NOT PERFORMED | None proposed | No candidate executed | Instrument gap recorded; no new instrument in C1b |
| Frame p95 | UNAVAILABLE raw distribution; existing smoothed display has narrower validity | UNAVAILABLE steady p95 | NOT PERFORMED | None proposed | No candidate executed | 16.7 ms reference receives no verdict; remains future direct measurement |
| Input response p95 | UNAVAILABLE direct latency instrument | UNAVAILABLE | NOT PERFORMED | None proposed | No candidate executed | 100 ms reference receives no verdict; remains future direct measurement |
| Observation age | UNAVAILABLE paired wall-clock timestamps; generation observations valid as labelled | Generation change observed, wall-clock freshness UNAVAILABLE | NOT PERFORMED | None proposed | No candidate executed | No generation-to-ms relabelling; future freshness characterization remains open |

The unavailable rows are honest 1.2 instrument-status evidence, not failed 1.2
obligations or authorization to widen this leaf. No inherited Phase-14/15/other
performance debt is retired by this small opening workload.

## Retained evidence, lifecycle and verification binding

The raw values and qualification above are committed in full. Supplemental
unedited screenshots and native cache copies remain on the Owner machine under
`C:\Users\mvorm\SimThing\.git\0088-studio-c1b-captures\`; they are local
corroboration, not remotely hosted attachments. SHA256s:

| Local capture | SHA256 |
| --- | --- |
| `run1-ready.png` | `e451f5cc5d0d918f796f3896ab08e33de476b7d39a7a6e440762730de4523edd` |
| `run2-ready.png` | `ea5426fddbbde2b426f4eabf853523b116f512c05e8b9cf6bc91a2daa61543ab` |
| `run3-ready.png` | `e352d5bfcbc479a5503a05468338b504927b90bb7caaa12af396e505b4c2b091` |
| `run3-playing-tick2.png` | `8fe0ea7acb91ab6db93242942169620d277bf2ebdb45396687e22ff7430ca957` |
| `run3-paused-tick2206.png` | `40fd8613047c20095c79e4069a32e5271cba362dd60629394ce06e0cf2828063` |
| `run3-scenario-context.png` | `fd391a38eb5cf33a4ecd08cac0cd5241921608f801923efbde4fbc7239f354fe` |
| `run3-gpu-context.png` | `c0ba275999557b1c43297343250dc47491340032172e34d66e3b8c08fb460706` |
| `run3-frame-instrument-status.png` | `b35a400dfd936b3b7861b8bb6cfbe7c648a8aa460e7372e3e31e0b2f324f22ac` |
| `samples.json` | `cf91b23d47797dc681ff977a93f020319fb51a3167fd78a8d9bded44f75851ea` |

The three `runN-loader-cache.json` copies share the cache SHA256 above. Build
log, machine/process inventories, initial user settings, bundle before/after
hashes and executable hash are retained in `.git/0088-studio-c1b-*` files.

This is a mortal 0.0.8.8-integrated-rehearsal results document, subject to ordinary
track closeout. It adds no executable test identity, fixture, leased code or
permanent proof; no test-inventory row or artifact lease is invented for it.
Applicable evidence/lifecycle gates and local `agent_scan` are run on the final
evidence delta. Hosted Doctrine report/step conclusions and fresh `/clearance`
are recorded in the PR body and Board return, including the complete INSPECT
list. Orchestration owns review, merge and any 1.2 reconciliation/graduation.
