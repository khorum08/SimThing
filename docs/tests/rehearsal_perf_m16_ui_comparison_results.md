# M16 B1 UI comparison — RETAIN-EGUI

Status: **PROBATION / proof-present / OPEN / UNMERGED to ORCHESTRATION ONLY**.
Rung: `0088-UI-PROTOTYPE-0`. Dispatch: Board **5654699942**; final qualification
and artifact disposition: **5671634736**; OVL addendum: **5653843173**.
HD-RECEIPT: **47010c34edf2**. Carried coding ORIENT-RECEIPT: **28f56884d309**.
Frozen runtime/base: **`a8c117313dbe3796aa160ba12cad1cd677bdc5e6`**.
Branch: `codex/0088-m16-ui-comparison`.

**Decision: RETAIN-EGUI.** The prototype operates the existing shared command
and observation paths, and Native consumes fresh observations earlier at its
instrumented endpoint. However, every primary capture misses the 16.7 ms frame
p95 reference line, required displayed input latency is unavailable, and
coexistence/process-total measurements do not establish an isolated replacement
benefit. Consumption is not display. No migration, dependency, Bevy upgrade,
prototype deletion or tuning is implemented or admitted by this leaf.

Orchestration has qualified the actual Paused setup, resize pacing and Running
durations. No further Owner capture is required. This packet records a decision
and measurement constraints; it does not grant a budget pass or rung graduation.
Final exact-head hosted certificates, complete INSPECT inventory and fresh
Clearance are bound in the PR and Board return, rather than inferred from this
document or an old certificate.

## Matrix selection and actual conditions

The primary matrix has **30 captures / 15 pairs / three paired repetitions for
each of five conditions**. The first chronologically completed Paused run is
selected by disposition, not by performance; the second is independent
replication. The six other valid exports remain unpaired historical evidence.
All seven rejected captures and all interrupted runs remain indexed and their
raw samples are committed as well. No run or slow tail is discarded.

Counterbalanced order within each complete group is Egui/Native, Native/Egui,
Egui/Native. A fresh Studio process starts each group, with copied isolated
settings and existing OS/driver caches. Native-on retains the Egui pane.
Whole-frame/process measurements therefore characterize coexistence, not an
isolated replacement client. The same seven-system workload is used throughout.

| Condition | Primary run | Qualified actual procedure |
| --- | --- | --- |
| Fresh-load Paused | `b1-20260913-132828-6183d8fe` | Reload before each capture; G0, paused, 1x/TPS10; steady visible pane. Not relabeled single-resident. |
| Numeric editing | `b1-20260913-221404-2c9b6380` | One loaded resident; G0; exactly 12.5,10,12.5,10,12.5,10 through each client. |
| Resize | `b1-20260913-224522-352c334b` | One resident, G0/TPS10; exactly three 1600x900 → 1920x1032 → 1600x900 cycles at actual pacing. |
| List interaction | `b1-20260913-233134-7564dae1` | One resident, clear initial selection; A,B,C,D,E,G,F / IDs 1,2,3,4,5,7,6 through map picks or Native list/Down. |
| Running | `b1-20260914-151919-2cb3ba3d` | Reload before each capture; Play1x/TPS10, observed running; F10 before Pause; actual generation phase retained. |

The 15-second duration and approximately two-second resize dwell were procedure
targets, not graduation minima or new service lines. Frozen usable minimum:
10 seconds. All accepted captures exceed it. Group 3 has five sub-target
durations and 28/36 observed geometry dwells below two seconds; Group 5 has
five sub-target durations. These are qualified at their actual values by
5671634736. No padding, trimming, threshold edit or exact two-second claim.
Manual warmup is at least five seconds as instructed, Owner-qualified rather
than independently timestamped. Actual capture lengths and runtime transitions
are retained. Generation phases are not normalized across re-admissions.

## Workload, machine and exact procedure

Source: `scenarios/stellaristhing_base.clause`; seed 0. Source identity
`fnv1a64:ee4e4df9e8c9fbd9:5798`; profile `fnv1a64:bfcbc44323b304bf:23530`;
base JSON `fnv1a64:c49f9ca3c8c75e77:20370`; dependencies
`fnv1a64:2f064bfb3e043aa0:72`. Source/base/dependency SHA-256 respectively:
`5821be96f0bfb439ce8717dba7dd280e994ae6a1113bff42d3137766031df328`,
`2b207255ac213398cc2eecb21ae70f2411af82f535eb1f7f86fad83032d21360`,
`7c626d606cf8059513d21549a5e682d8243f2112b5df586bed08fa2edadfbf87`.

Unchanged Meridian: seven systems, six links, two owners, two sites, four
cohorts at each site, thirteen energy RF participants, two recipes and four
material loci. No workload/stress variant. Existing resident subscriptions
and command/observation owners; no new observer or simulation authority.

Owner Windows 11 reference machine: Intel Core i9-13980HX; NVIDIA GeForce RTX
4080 Laptop GPU; Vulkan; driver 595.79 / 32.0.15.9579; default Fifo Bevy Window.
Primary runtime geometry is 1600x900 physical, scale1, except the explicit
resize cycles. Original 1920x1080 B0b geometry remains separately named.
Compiler: rustc 1.95.0 (59807616e, 2026-04-14), MSVC, LLVM22.1.2; Bevy0.16.1,
bevy_egui0.36.0, egui0.32.3. No Cargo or toolchain mutation.

Build: `cargo build -j 1 --release -p simthing-mapeditor --bin simthing-studio`,
`CARGO_TARGET_DIR=C:/Users/mvorm/SimThing/target`, unchanged release profile with
thin LTO. Studio SHA-256:
`0c43fd4ae548a18b5b18752fefad7daf6ae2031b9da74c4a79eb0df31682dad8`.
Build log SHA-256: `b7f7424a627ce6d4b3962ae5a051e75d8a48cf22d8713325224f8cd986cc7bdd`.
Instrument revision: `m16-b0a-v1`; B1 final bundle v6 / protocol v5, with each
older capture bound to its own original version. Python3.13.2 standard library.

Owner launcher: `C:/Users/mvorm/SimThing-0088-OVL-B1/Start-B1-OVL.cmd`, delegating
to `Start-B1-OVL.ps1 -Group <paused|numeric_edit|resize|list_interaction|running>`.
The child is that bundle's `simthing-studio.exe`, with process-local
`SIMTHING_M16_CONFIG=<run>/current-config.json` and isolated copied
`APPDATA=<run>/appdata`. Select the card's explicit Native checkbox state with
the mouse; F9 once starts silently; F10 once stops before export. Remain focused
without console clicks/screenshots during recording. Read the saved confirmation
before the next card; screenshots follow export. Normal user settings remain
untouched. Exact per-capture commands, full metadata, plans and original cards
are committed in the attempt-index and analyzer/protocol shards.

Nearest-rank p50/p95/p99 are `sorted[ceil(p*N)-1]`; include min/max/mean/count,
with no clipping or smoothing. Integer nanoseconds in the raw shards retain
original sample order. Full-frame cadence is Bevy `Time<Real>` main-loop
intervals, not scanout/compositor delivery. The first interval straddling F9 is
excluded by the already-merged instrument, not by retrospective trimming.

## Primary full-frame cadence

All **46,298 primary frame intervals** are retained. Primary p95 range: **38.7780–42.2687 ms**, all above 16.7 ms. Capture IDs join every table to the raw shards.

| Condition / rep / client | Capture ID | Duration s | Frames | p50 / p95 / p99 / max ms |
| --- | --- | --- | --- | --- |
| paused / 1 / Egui | 3 | 21.7881759 | 1631 | 3.9573 / 39.8964 / 41.1951 / 43.5134 |
| paused / 1 / Native | 4 | 25.4363252 | 1906 | 3.9601 / 39.4535 / 40.6461 / 43.5939 |
| paused / 2 / Egui | 6 | 20.1162756 | 1505 | 3.5021 / 42.2687 / 43.3849 / 46.0210 |
| paused / 2 / Native | 5 | 17.9018305 | 1339 | 3.6920 / 41.3685 / 43.2638 / 46.8837 |
| paused / 3 / Egui | 7 | 23.4307746 | 1755 | 3.8452 / 39.8933 / 41.1163 / 43.3392 |
| paused / 3 / Native | 8 | 23.1470208 | 1731 | 3.7388 / 41.5156 / 42.7736 / 47.5055 |
| numeric_edit / 1 / Egui | 20 | 26.5656849 | 1989 | 3.9104 / 41.7662 / 42.7793 / 45.5682 |
| numeric_edit / 1 / Native | 21 | 34.3822358 | 2574 | 3.8281 / 40.7399 / 42.2896 / 45.2894 |
| numeric_edit / 2 / Egui | 23 | 26.8383319 | 2010 | 3.8084 / 41.8931 / 42.9368 / 47.3578 |
| numeric_edit / 2 / Native | 22 | 24.8906034 | 1864 | 3.7762 / 39.6973 / 41.8541 / 44.9998 |
| numeric_edit / 3 / Egui | 24 | 23.6243451 | 1767 | 3.9745 / 42.1089 / 43.1128 / 45.8320 |
| numeric_edit / 3 / Native | 25 | 25.4513520 | 1905 | 4.0177 / 42.1362 / 43.1970 / 45.7423 |
| resize / 1 / Egui | 27 | 18.0539349 | 1343 | 3.6724 / 41.7395 / 45.1395 / 110.9532 |
| resize / 1 / Native | 28 | 13.9745124 | 1038 | 3.7580 / 41.5607 / 45.8799 / 91.7699 |
| resize / 2 / Egui | 30 | 11.8803585 | 878 | 3.6034 / 41.7075 / 49.7139 / 91.4260 |
| resize / 2 / Native | 29 | 12.0982545 | 895 | 3.9303 / 41.8172 / 52.0408 / 89.0274 |
| resize / 3 / Egui | 31 | 10.9782936 | 812 | 3.3418 / 41.5791 / 50.5329 / 91.7625 |
| resize / 3 / Native | 32 | 14.7168723 | 1094 | 3.4374 / 41.9724 / 46.4529 / 89.4676 |
| list_interaction / 1 / Egui | 38 | 33.2872630 | 2493 | 5.3331 / 39.0517 / 40.1319 / 43.3875 |
| list_interaction / 1 / Native | 39 | 24.2521099 | 1815 | 4.6903 / 38.7780 / 39.7387 / 43.7813 |
| list_interaction / 2 / Egui | 41 | 24.3832933 | 1826 | 4.2027 / 41.4260 / 42.2326 / 44.8330 |
| list_interaction / 2 / Native | 40 | 26.1979215 | 1961 | 4.3113 / 39.8233 / 41.4031 / 45.4940 |
| list_interaction / 3 / Egui | 42 | 28.2292540 | 2116 | 4.4645 / 41.3200 / 42.1056 / 45.2264 |
| list_interaction / 3 / Native | 43 | 23.7835860 | 1779 | 4.1465 / 41.3264 / 42.4032 / 46.0822 |
| running / 1 / Egui | 44 | 14.1855132 | 1061 | 7.7692 / 39.3748 / 40.4548 / 44.2825 |
| running / 1 / Native | 45 | 16.0308548 | 1199 | 4.4286 / 39.2424 / 40.4286 / 41.9913 |
| running / 2 / Egui | 47 | 13.8180410 | 1035 | 8.0741 / 39.6096 / 40.8252 / 43.4775 |
| running / 2 / Native | 46 | 14.0862868 | 1053 | 3.7330 / 39.3082 / 40.2347 / 43.6898 |
| running / 3 / Egui | 48 | 13.0762348 | 977 | 3.5661 / 39.5837 / 41.0642 / 43.4016 |
| running / 3 / Native | 49 | 12.6714559 | 947 | 3.9822 / 39.3715 / 40.4923 / 43.1784 |

## Paired descriptive frame and process-memory differences

Native minus Egui by condition/repetition. These independent captures have different manual phases/durations. Ordinal pairing is chronological description only, never a causal frame association or latency join. All unmatched tails remain explicit blank-sided rows in the residual shard. Memory is PID/start-bound private bytes and working set immediately after durable raw export, before analysis, with original UTC/lag retained. It includes instrumentation/export allocation history, not isolated UI or GPU allocations.

| Condition / rep | Frame N-E p50 / p95 / p99 / max ms | Private bytes N-E | Working set bytes N-E | Ordinal pairs / unmatched |
| --- | --- | --- | --- | --- |
| paused / 1 | 0.0028 / -0.4429 / -0.5490 / 0.0805 | +28,397,568 | +30,216,192 | 1631 / 275 |
| paused / 2 | 0.1899 / -0.9002 / -0.1211 / 0.8627 | -4,231,168 | -4,808,704 | 1339 / 166 |
| paused / 3 | -0.1064 / 1.6223 / 1.6573 / 4.1663 | +1,761,280 | +864,256 | 1731 / 24 |
| numeric_edit / 1 | -0.0823 / -1.0263 / -0.4897 / -0.2788 | +11,882,496 | +11,390,976 | 1989 / 585 |
| numeric_edit / 2 | -0.0322 / -2.1958 / -1.0827 / -2.3580 | -663,552 | -675,840 | 1864 / 146 |
| numeric_edit / 3 | 0.0432 / 0.0273 / 0.0842 / -0.0897 | +1,118,208 | +856,064 | 1767 / 138 |
| resize / 1 | 0.0856 / -0.1788 / 0.7404 / -19.1833 | +21,512,192 | +12,550,144 | 1038 / 305 |
| resize / 2 | 0.3269 / 0.1097 / 2.3269 / -2.3986 | -6,729,728 | -864,256 | 878 / 17 |
| resize / 3 | 0.0956 / 0.3933 / -4.0800 / -2.2949 | -258,772,992 | +1,454,080 | 812 / 282 |
| list_interaction / 1 | -0.6428 / -0.2737 / -0.3932 / 0.3938 | +10,190,848 | +10,502,144 | 1815 / 678 |
| list_interaction / 2 | 0.1086 / -1.6027 / -0.8295 / 0.6610 | -8,192 | +397,312 | 1826 / 135 |
| list_interaction / 3 | -0.3180 / 0.0064 / 0.2976 / 0.8558 | +4,096 | +4,096 | 1779 / 337 |
| running / 1 | -3.3406 / -0.1324 / -0.0262 / -2.2912 | +99,090,432 | +95,694,848 | 1061 / 138 |
| running / 2 | -4.3411 / -0.3014 / -0.5905 / 0.2123 | -8,253,440 | -9,486,336 | 1035 / 18 |
| running / 3 | 0.4161 / -0.2122 / -0.5719 / -0.2232 | -823,296 | -40,493,056 | 947 / 30 |

## Shared CPU and separate adapter scopes

| Scope | Boundary / interpretation |
| --- | --- |
| `shared_clock_and_observation_projection` | Identical shared computation in both clients; comparable host-wall interval, including preemption. |
| `egui_clock_transport_including_observation` | Existing Egui transport UI work including nested live observation. |
| `egui_live_observation_adapter_including_projection` | Nested Egui observation adapter including shared projection. |
| `native_pane_sync_including_projection` | Existing Native pane sync/Text work including shared projection; not render/layout/GPU completion. |
| `native_input_router` | Existing Native input routing, also called with Native disabled. |

Adapter shapes differ; none is a whole-client equivalent, and nested scopes are never summed. Each raw start/end interval is committed. The portable replay emits complete per-scope distributions. The following p95 adapter columns retain those different boundaries; missing Native sync means disabled, not measured zero.

| Condition / rep / client | Shared projection p50 / p95 / p99 / max ns | Egui transport / observation p95 ns | Native sync / input p95 ns |
| --- | --- | --- | --- |
| paused / 1 / Egui | 600 / 1100 / 3700 / 29200 | 50700 / 13300 | disabled / 3900 |
| paused / 1 / Native | 600 / 1200 / 1800 / 61100 | 47600 / 12600 | 23200 / 3900 |
| paused / 2 / Egui | 500 / 1100 / 3600 / 9700 | 51500 / 13100 | disabled / 3900 |
| paused / 2 / Native | 600 / 1200 / 1700 / 7900 | 46100 / 12100 | 22700 / 3900 |
| paused / 3 / Egui | 600 / 1100 / 3800 / 88400 | 49500 / 13200 | disabled / 3900 |
| paused / 3 / Native | 700 / 1300 / 2300 / 8100 | 47700 / 12400 | 23700 / 4100 |
| numeric_edit / 1 / Egui | 500 / 1000 / 1900 / 6100 | 49200 / 11700 | disabled / 3400 |
| numeric_edit / 1 / Native | 600 / 1200 / 1700 / 13200 | 47000 / 12200 | 22600 / 4600 |
| numeric_edit / 2 / Egui | 600 / 1100 / 3800 / 7900 | 52500 / 12900 | disabled / 3500 |
| numeric_edit / 2 / Native | 700 / 1200 / 2000 / 9500 | 47200 / 12100 | 22800 / 4600 |
| numeric_edit / 3 / Egui | 500 / 1100 / 3600 / 5700 | 53000 / 12500 | disabled / 3000 |
| numeric_edit / 3 / Native | 600 / 1200 / 1700 / 7200 | 48400 / 12400 | 23300 / 4900 |
| resize / 1 / Egui | 600 / 1000 / 3000 / 31500 | 50300 / 12200 | disabled / 3500 |
| resize / 1 / Native | 600 / 1100 / 1500 / 7900 | 40600 / 10100 | 19900 / 4200 |
| resize / 2 / Egui | 600 / 1000 / 1900 / 41000 | 45100 / 11500 | disabled / 3400 |
| resize / 2 / Native | 600 / 1100 / 1600 / 13300 | 42000 / 10400 | 19200 / 4100 |
| resize / 3 / Egui | 600 / 900 / 1700 / 5600 | 43400 / 10900 | disabled / 3500 |
| resize / 3 / Native | 600 / 1100 / 1600 / 80300 | 43500 / 11200 | 20800 / 4300 |
| list_interaction / 1 / Egui | 600 / 900 / 2300 / 6700 | 40600 / 10500 | disabled / 3400 |
| list_interaction / 1 / Native | 600 / 1000 / 1600 / 13600 | 38400 / 9800 | 19200 / 3700 |
| list_interaction / 2 / Egui | 500 / 900 / 1500 / 6400 | 39700 / 10000 | disabled / 3300 |
| list_interaction / 2 / Native | 600 / 1100 / 1600 / 8300 | 39400 / 10000 | 19600 / 3900 |
| list_interaction / 3 / Egui | 600 / 1000 / 3600 / 31000 | 44100 / 10900 | disabled / 3400 |
| list_interaction / 3 / Native | 600 / 1100 / 1500 / 13600 | 40000 / 10100 | 20200 / 4100 |
| running / 1 / Egui | 600 / 1000 / 4000 / 410900 | 58100 / 16000 | disabled / 3800 |
| running / 1 / Native | 600 / 1200 / 1900 / 13600 | 54700 / 15300 | 23500 / 4600 |
| running / 2 / Egui | 600 / 1000 / 1800 / 7900 | 60200 / 16800 | disabled / 3800 |
| running / 2 / Native | 600 / 1100 / 1800 / 6000 | 53300 / 15100 | 23600 / 4600 |
| running / 3 / Egui | 600 / 1000 / 2600 / 6600 | 58900 / 16700 | disabled / 3300 |
| running / 3 / Native | 600 / 1200 / 1800 / 13100 | 55900 / 15700 | 24000 / 4600 |

**20,233 exact primary shared-CPU pairs** match one recorded frame and publication identity inside Native-on captures, with Runtime-change frames excluded by the original analyzer. Unpaired calls remain counted and raw. Negative residuals and large maxima are retained; no slow-sample removal.

| Condition / rep | Direct pairs | N-E min / p50 / p95 / p99 / max ns | Unpaired projection calls |
| --- | --- | --- | --- |
| paused / 1 | 1788 | -10600 / 100 / 600 / 1200 / 60300 | 242 |
| paused / 2 | 1253 | -13100 / 100 / 500 / 1000 / 6900 | 178 |
| paused / 3 | 1633 | -4500 / 100 / 700 / 1400 / 6200 | 200 |
| numeric_edit / 1 | 2168 | -3400 / 100 / 500 / 900 / 12800 | 816 |
| numeric_edit / 2 | 1641 | -4600 / 100 / 600 / 1300 / 8400 | 452 |
| numeric_edit / 3 | 1679 | -3100 / 100 / 600 / 1000 / 4300 | 458 |
| resize / 1 | 938 | -2100 / 100 / 600 / 1000 / 3700 | 206 |
| resize / 2 | 809 | -4300 / 100 / 500 / 900 / 12800 | 178 |
| resize / 3 | 973 | -2200 / 100 / 600 / 1300 / 79900 | 246 |
| list_interaction / 1 | 1624 | -31600 / 100 / 500 / 1000 / 13100 | 388 |
| list_interaction / 2 | 1810 | -115700 / 100 / 500 / 1100 / 7300 | 308 |
| list_interaction / 3 | 1681 | -70900 / 100 / 500 / 900 / 12800 | 200 |
| running / 1 | 845 | -3100 / 100 / 500 / 1300 / 13200 | 712 |
| running / 2 | 738 | -3500 / 100 / 500 / 900 / 4500 | 636 |
| running / 3 | 653 | -4900 / 100 / 500 / 900 / 2200 | 594 |

## Fresh observation delivery, distinct from display

Running yields **839 new publications**, all consumed by the expected pane. Including coexisting Egui gives **1,267 first-consumption samples** and **428 exact same-publication pairs** within Native-on captures. Full scene/resident/generation/publication-time identity, uniqueness and bounds verify; there are zero stale/rejected observations and no unconsumed in-capture publications for active panes. Paused/numeric/resize/list retain the G0 publication; their pre-capture age is not new delivery. All ages, actual generations and any rejected records remain in the raw event shard.

| Rep / capture client / measured pane | Fresh count | p50 / p95 / p99 / max ms |
| --- | --- | --- |
| 1 / Egui / Egui | 142 | 0.3352 / 0.5273 / 0.6671 / 0.7012 |
| 1 / Native / Native | 160 | 0.0269 / 0.0660 / 0.1391 / 0.1396 |
| 1 / Native / Egui | 160 | 0.3377 / 0.4900 / 0.6990 / 0.8362 |
| 2 / Egui / Egui | 138 | 0.3297 / 0.5520 / 0.9274 / 1.0413 |
| 2 / Native / Native | 141 | 0.0265 / 0.0648 / 0.0737 / 0.0778 |
| 2 / Native / Egui | 141 | 0.3295 / 0.4277 / 0.5554 / 0.6429 |
| 3 / Egui / Egui | 131 | 0.3419 / 0.6084 / 0.8912 / 1.1287 |
| 3 / Native / Native | 127 | 0.0271 / 0.0805 / 0.1311 / 0.2865 |
| 3 / Native / Egui | 127 | 0.3444 / 0.5910 / 0.9151 / 1.0005 |

| Rep / Native capture ID | Exact freshness pairs | Same-publication N-E min / p50 / p95 / p99 / max ms |
| --- | --- | --- |
| 1 / 45 | 160 | -0.7015 / -0.3083 / -0.2363 / -0.2218 / -0.2128 |
| 2 / 46 | 141 | -0.6021 / -0.2945 / -0.2366 / -0.2079 / -0.1857 |
| 3 / 49 | 127 | -0.9701 / -0.3081 / -0.2424 / -0.2142 / -0.2136 |

Native reaches the existing observation point earlier in all 428 pairs. This is a real bounded delivery result, not evidence of earlier DISPLAYED response or an input-latency pass. Signed residual p95 is the p95 of the residuals, not subtraction of two independent p95s. Cross-resident generations are not treated as the same publication.

## Exact procedure transitions and independent replication

| Resize capture ID | Observed geometry dwell seconds, first maximize through stop (six intervals) |
| --- | --- |
| 27 | 1.5789198 / 2.5029728 / 1.3890549 / 2.5060439 / 1.1767887 / 2.1818089 |
| 28 | 1.4518256 / 1.7763537 / 1.2417030 / 2.0646242 / 1.2704431 / 2.0619117 |
| 30 | 1.1277042 / 1.5390479 / 1.3623294 / 1.8650740 / 1.3114164 / 2.0633118 |
| 29 | 1.2283432 / 1.5421867 / 1.4270813 / 1.6683623 / 1.3384016 / 2.7025828 |
| 31 | 1.1780718 / 1.6887956 / 1.2821566 / 1.9076044 / 1.0142679 / 1.9569242 |
| 32 | 1.6566402 / 1.7977831 / 1.7488826 / 1.8543748 / 1.4388575 / 3.3485731 |

These are observation-state dwell intervals, not exact physical-key timestamps. All numeric/list transitions, including earlier failed sequences, are retained as Runtime event records. Source projection orders placements by (row,col,system_id), so the actual Native row order is A,B,C,D,E,G,F. The prospective Group 4 protocol correction applies only to new captures; original ascending-ID reports remain rejected under their original rules.

Independent Paused replication: **`b1-20260913-133909-27418d42`**, six fresh-load captures, three pairs, kept outside the primary matrix. Its shorter actual durations retain the same qualification; it does not replace the first completed run on performance grounds.

| Rep / client | Capture ID | Duration s | Frames | p50 / p95 / p99 / max ms |
| --- | --- | --- | --- | --- |
| 1 / Egui | 11 | 17.9792917 | 1345 | 3.4785 / 42.5438 / 43.5331 / 46.0232 |
| 1 / Native | 12 | 17.8728955 | 1337 | 3.7808 / 42.4652 / 43.8888 / 46.4679 |
| 2 / Native | 13 | 22.8399698 | 1709 | 3.6112 / 42.5901 / 43.5928 / 46.1050 |
| 2 / Egui | 14 | 12.3766909 | 926 | 3.4345 / 42.5901 / 43.5008 / 46.3145 |
| 3 / Egui | 15 | 12.9113857 | 965 | 3.9139 / 42.5011 / 43.5259 / 45.7152 |
| 3 / Native | 16 | 17.6350898 | 1319 | 3.8920 / 40.6004 / 42.7829 / 47.5320 |

The attempt-index shard lists all 18 runs and 49 exports with exact raw/report hashes, original results, roles and metadata. Committed raw samples include six valid unpaired exports and all seven rejected captures; none is promoted into a primary pair. The larger archive retains launcher failure diagnostics, original screenshots and B0b/R1. The prior chronological investigation is preserved at documentation commit `c7f8fb3b64bbfddb05fee21926377d6307f0d9fa`.

| Rejected capture ID / run | Original failures |
| --- | --- |
| 1 / `b1-20260913-130128-dab634ef` | Native projection appeared in the native-disabled baseline; Unexpected focused; check source/client/focus/setup; Unexpected native_enabled; check source/client/focus/setup |
| 17 / `b1-20260913-213947-06892d58` | Numeric edits must record exactly three 12.5 -> 10 pairs |
| 19 / `b1-20260913-215610-e01f7c05` | Numeric edits must record exactly three 12.5 -> 10 pairs |
| 26 / `b1-20260913-223431-74f244c0` | Resize must record three maximize/restore cycles ending at the initial size |
| 34 / `b1-20260913-230204-f9063a95` | Selection must start clear and visit systems 1 through 7 in order |
| 35 / `b1-20260913-232035-e26f7c3d` | Selection must start clear and visit IDs 1,2,3,4,5,7,6 in order |
| 37 / `b1-20260913-232212-fb723200` | Selection must start clear and visit IDs 1,2,3,4,5,7,6 in order |

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

## Behavior qualification and six separate status fields

Leaf A establishes shared command/session/selection/observation ownership.
B1 adds Owner-qualified exact numeric editing, Down traversal with matching
selection state, actual maximize/restore cycles, and fresh running observations.
The final Native screenshot shows post-test G207 and scheduled tick1371, after
the last captured publication G187; it is context, not a measurement endpoint.
Screenshots have no run/time overlay and are bound as Owner-supplied context
with immutable hashes. Physical key/click details, camera pose and setup warmup
remain Owner-qualified. General text/accessibility, arbitrary resize, long-list
scaling, changed DPI and multiple monitors remain unqualified. No synthetic
proof is silently upgraded to desktop qualification.

`Release a8` below means the unchanged recorded release executable, not candidate
admission or a performance-budget pass. Historical timing is not pooled across
different instruments or window conditions.

| Quantity | Instrument validity | Current-path characterization | Historical comparison | Candidate release | Candidate execution/result | Inherited budget/debt disposition |
| --- | --- | --- | --- | --- | --- | --- |
| Full frame cadence | B0a valid | Five qualified actual conditions | No cross-instrument equivalence | Release a8 | Three Native/Egui pairs per condition; replication separate | Every primary p95 >16.7ms; constraint measured, tuning not authorized |
| Shared CPU | Valid identical scope | Exact frame/publication pairs; raw unpaired calls | No substituted baseline | Release a8 | Signed samplewise residuals retained | No new CPU service line; isolated client benefit unproven |
| Adapter CPU | Valid named host-wall scopes | Separate nested distributions | Unequal adapters not equivalents | Release a8 | Native and coexisting Egui scoped | No sum of nested scopes or GPU attribution |
| Freshness | Valid exact identity | 839 fresh publications; 428 exact pairs | Paused age not delivery baseline | Release a8 | Native earlier consumer in 428 pairs; zero Running rejections | Display/input debt unresolved |
| Process memory | Valid PID/start samples | Private bytes/working set after export | Allocation histories differ | Release a8 | Both conditions, signed differences | UI-only and GPU splits unavailable |
| Input latency | INSTRUMENT-INVALID / UNAVAILABLE | Containment and R1 failure preserved | Original B0b, no new measurement | Endpoint not qualified | 0/6 credited each; K not run | Required p95 <=100ms UNEVALUATED |
| First display | UNAVAILABLE | No matching DISPLAYED endpoint | No submission proxy | Not qualified | Not measured | No first-display benefit claim |
| Direct render/GPU cost | UNAVAILABLE | No directly admitted timing instrument | No residual proxy | Not qualified | Not measured | No GPU saving claim |
| Separate simulation/presentation GPU memory | UNAVAILABLE | Process totals cannot separate | No inferred baseline | Not qualified | Not measured | No allocation-split claim |

## Committed raw shards and reproducibility

All numeric evidence is committed within `docs/tests/rehearsal_perf_*_results.md`, as directed. The original captures reconstruct exactly in semantic JSON form, with original byte hashes separately retained. Eight shards preserve **81,398 frames, 404,254 CPU intervals, 1,339 consumptions, 839 publications and 172 Runtime records**, plus 34,436 exact CPU residuals, 428 exact freshness pairs and 29,159 ordinal rows including unmatched tails. Those totals cover all 49 exports, not just the 30-row primary matrix.

| Shard | UTF-8 LF bytes | SHA-256 |
| --- | --- | --- |
| [rehearsal_perf_m16_ui_comparison_raw_frames_results.md](rehearsal_perf_m16_ui_comparison_raw_frames_results.md) | 4206109 | `92149aba2badee660aeed7f9bd235e8f4435abcc17a98bbd11ae8a223fe3ab89` |
| [rehearsal_perf_m16_ui_comparison_raw_cpu_results.md](rehearsal_perf_m16_ui_comparison_raw_cpu_results.md) | 16620137 | `fc5caa4d3c4215c9bafa8766e799a89be506346d32379144342b4cf23992d0d1` |
| [rehearsal_perf_m16_ui_comparison_raw_freshness_results.md](rehearsal_perf_m16_ui_comparison_raw_freshness_results.md) | 492505 | `01e767d0bac5c7e09ac4a07a69fb8159f23134656fbf8c32187c12bfde7a5f81` |
| [rehearsal_perf_m16_ui_comparison_raw_memory_results.md](rehearsal_perf_m16_ui_comparison_raw_memory_results.md) | 32526 | `2fa577e87dc5b7f557569d0a09124a52792707540d59b8e2ea8134fe4cb84ec1` |
| [rehearsal_perf_m16_ui_comparison_raw_residuals_results.md](rehearsal_perf_m16_ui_comparison_raw_residuals_results.md) | 5300173 | `f38a0e30891b4527ee2b2e799e668d8cc14a77266a9d26ca5bbf6a8eaf5463c3` |
| [rehearsal_perf_m16_ui_comparison_analyzer_protocol_results.md](rehearsal_perf_m16_ui_comparison_analyzer_protocol_results.md) | 71661 | `083eb59583cd894dcbbba38212b5f7a2ac6590410a311b443bda00c1761265f4` |
| [rehearsal_perf_m16_ui_comparison_replay_results.md](rehearsal_perf_m16_ui_comparison_replay_results.md) | 13391 | `9a39ba03ebef5f0c9bbe739ae288a0c55f29607e1d7e3102856a43f2265c04b5` |
| [rehearsal_perf_m16_ui_comparison_attempt_index_results.md](rehearsal_perf_m16_ui_comparison_attempt_index_results.md) | 593116 | `e21caa9699cd53d9eac41940ba4bfcf27f095a80706c92529b85af518a91cd72` |

Copy the single Python block in the replay shard into a temporary `replay_shards.py`, then run:

```text
python replay_shards.py /path/to/checkout --selftest
```

Python3.13.2 standard library only; no archive, original C: paths, Studio, GPU
or network required. The verifier restores sample order and every raw capture,
checks original canonical hashes, uses each captured analyzer/protocol version,
reproduces every complete report, verifies exact residuals and every ordinal
tail, and recomputes all primary/replication tables. Markdown hash checks use
UTF-8 LF so normal Windows CRLF checkout conversion is supported. Original
artifact byte hashes and original version line endings are not normalized.

Local lossless replay PASS: **49 reports, 42 valid / 7 rejected**; role checks
confirm 30 primary, 6 independent replication, 6 valid unpaired, 7 rejected.
Falsification checks detect a changed final frame interval, dropped final
sample, and altered negative CPU residual. These verify artifact integrity,
not a new physical run or input-latency instrument.

The Python block was also extracted from a separate copy containing only the
eight shards, with every Markdown file converted to Windows CRLF. Replay and
all three falsification checks PASS there without the local ZIP or original
run directories. Derived output SHA-256:
`9f029f7cb4be4c1b150770b2736d5563c9195b2ed3fd24f7997ec73fc004c60a`.

The larger **823-file / 10,788,169-byte** archive remains at
`C:/Users/mvorm/SimThing/.git/0088-m16-b1-review/m16-b1-review-through-running.zip`.
ZIP SHA-256: `c0e21bcdf0d64f3980067464dc67d0230232d8ed7e801dc7d29c55077c87e56e`;
index SHA-256: `e8a41bf4532496d6249ea86752e8584f21d05f12924628340004e54092cea638`.
Its complete immutable file index is committed in the attempt-index shard,
including original screenshots, interrupted/rejected attempts, original
protocol/analyzer versions and B0b/R1 diagnostic hashes. Executables are omitted
with build/version hashes retained. Its separate `python replay.py` verifies
623 original B1 entries and all49 reports plus31 B0b entries; it does not rerun
B0b/R1 or invent another correlation heuristic. The archive is supplemental;
the committed shards independently support the measured B1 numeric rows.

All preparation bugs and failed attempts remain evidence. No extra Owner work
or DA relay is required by disposition 5671634736. This leaf has no production,
test regime, dependency, gate, class, census or protected fixture mutation.
Results/parser code blocks are rung-local replay documentation; no parser is
mounted into production. All experimental artifacts retire with the owning
1.3 rung. Prototype retirement is a separately admitted follow-up, not an
implicit action of this retain decision.

## Proof and routing

The exact raw runtime is the a8 release executable above. The final evidence
head is separately bound in the PR's `tested_code_sha` and current hosted
Doctrine/INSPECT/Clearance records. Local replay and targeted integrity
falsification are the semantic proof for these Markdown data changes; no Rust
implementation or redundant cargo battery is introduced. Local `agent_scan`,
hosted Doctrine step conclusions and full advisory inventory must be read at
the final head. Coding does not self-triage, merge, graduate or contact DA.
Return **PROBATION / proof-present / OPEN / UNMERGED to ORCHESTRATION ONLY**.
