# 0088-UI-PROTOTYPE-0 — Leaf A native presentation

PROBATION / proof-present / OPEN / UNMERGED to Orchestration only.
Board dispatch 5648994332 resumes Leaf A after the canonical HD repair.
Base `76c9cd43637dedaaca7199c1fcc3080bdf60b485`; branch
`codex/0088-native-ui-prototype`. Exact final head, hosted artifact, clearance,
complete INSPECT accounting and tested-code binding are carried in the PR/Board return.
HD-RECEIPT `47010c34edf2`; coding ORIENT-RECEIPT `28f56884d309`;
orientation rule stamp `73e54d6b56b7266b`. Novelty claim: NO.

## Seam inventory before edits

| Seam | Existing owner and disposition |
| --- | --- |
| Egui context memory | `app/ui.rs` contains temporary Generate, save/load, candidate, Clause picker and refresh flags. The selected clock workflow uses none of them; no flag extraction is necessary. |
| Commands and numeric draft | `StudioAppState.sim_clock_transport` owns the sole clock, TPS draft and validation error. Both presentations call its existing `apply` / `apply_max_tps_draft` methods synchronously. |
| Session | `StudioAppState.session` and the NonSend `StudioLiveSessionBridge` retain admission, execution and replacement. No native session, command queue or observer is created. |
| Scenario and save/load | `StudioScenarioLibraryModel` owns modal pause, load-attempt tokens and source-to-ready provenance; the existing scenario document/candidate save-load routes remain the acceptance path. |
| Selection | Native rows call `selection::apply_star_click` on the same `StudioAppState.selection` used by map picking, highlight, nameplates and egui inspector. Rows project current `view_model.stars`; no structural edit is made. |
| Observation | Native text calls `build_studio_live_observation_readout` and projects `live_bridge_readout.field_accretion_samples` with their existing tick/generation labels, as egui does. Replacement-pending snapshots are withheld. |
| Focus/input | Bevy UI layout provides physical node/clip bounds. Raw Bevy keyboard/wheel/button input is routed before egui BeginPass and map Update; only native-owned events are removed from egui's translated buffer. The existing EguiClipboard resource supplies OS clipboard access. |

## Implementation and coexistence

`rehearsal_studio_native_ui.rs` mounts one native Bevy UI pane on the existing
primary Camera3d. No additional camera, render resource, dependency, feature,
Cargo or lockfile edit. Native state in `StudioAppState` contains presentation
enable/focus/caret/list-scroll/input ownership only. Disabled is the startup
default; F8 or the egui checkbox toggles it, and its own Disable button exits.

The pane operates Pause/Play, 1x/2x/4x, the shared Max TPS draft and validation,
a three-row scrolling window over the actual loaded system list, and live
resident property samples. Tab/Shift-Tab navigate controls; Enter activates;
Up/Down select adjacent systems; Escape releases native focus. Numeric editing
supports character entry, caret arrows/Home/End, Backspace/Delete and
Ctrl-A/C/X/V. It is a bounded numeric field: no general code editor, IME widget,
partial-range mouse selection, or migration-grade accessibility claim.

Egui floating areas/windows take pointer priority; its empty full-viewport
background does not. Pointer drags keep their origin through release, including
native-to-map and map-to-native crossings. Native key holds remain owned until
release. Camera movement/hotkeys, selection Escape, picking and wheel/orbit
consult the same per-frame decision; blocked wheel/motion events are drained.
Scenario/settings/telemetry/generation/warning dialogs and loading cover suspend
the native pane. The existing library pause gate remains authoritative.

Deletion endpoint if migration is declined: remove the rehearsal module and its
lib/plugin/state/egui-toggle mounts, plus native input gates in camera/picking.
No serialized config or scenario data needs conversion. Egui remains supported.

## Focused semantic proof

`cargo check -p simthing-mapeditor`: PASS. The new owning integration battery
`rehearsal_studio_native_ui` passes 3/3 on Windows, with lawful AUDIT inventory
rows, boundary `0088-UI-PROTOTYPE-0`, birth track 0.0.8.8, DSU survivals 0.
It executes the production dispatcher and Bevy text update systems with input
events, tests egui's actual background registration, validates rejection of
TPS 0 and acceptance of 12.5 through the existing owner, focus-event consumption,
both drag origins, modal pause, disable fallback and physical hit coordinates
at a simulated Windows scale change to 1.5.

The real-adapter leg loads the unchanged shipped Clause source via the worker
picker and `try_adopt_clause_load_attempt`, verifies the profile pin, operates
the clock and selection owners, and checks actual Bevy Text against the bridge
snapshot. At resident G1 the four samples are A1 alloys 5, A1 minerals 4,
E1 alloys 4, E1 minerals 4. Presentation refresh leaves the resident generation
and execution identity unchanged. Stale rendered rows refuse after a scene
revision change. Source/base/dependency bytes remain unchanged.
The existing `rehearsal_studio_source_to_ready` admission regression passes 1/1.

## Reference Windows behavior witness

Executed the release binary built from code commit
`b1eecfbc1429d29463a3721940f34f43d148da3d` on 2026-09-12. Subsequent
results-only edits do not change this binary's sources. This is a functional
exercise, with no M16 timing or migration claim. Desktop procedure and observations:

| Action | Observed result |
| --- | --- |
| Launch Studio; F8; click Max TPS; paste `0`; Enter | Default-off pane mounts on the existing camera. Both clients show the owner's finite-and-positive refusal; effective Max TPS remains 10. |
| Ctrl-A; paste `12.5`; Tab; Shift-Tab; Ctrl-X; Ctrl-V | Both clients show 12.5. Tab focuses Apply TPS without toggling the 3D camera; Shift-Tab returns to numeric focus. Cut clears the shared draft and paste restores 12.5. |
| Open existing Library while native focus is active; Select File; choose the unchanged shipped Clause path; Load | Native pane suspends for the existing picker/modal and returns after admission. Seven real systems and resident G0 samples 4/3/3/3 appear. |
| Native Play, then native Pause | Both clients play; captured G2 samples are 7/3/6/3. Both pause at G109; samples 167/4/166/4 remain frozen during subsequent UI operations. Sample order is A1 alloys/minerals, then E1 alloys/minerals. |
| Wheel over the three-row native system list; click C; Tab; Enter; Escape | List changes A/B/C to B/C/D. Clicking C selects system 3 in both native text and the egui inspector. Tab/Enter selects D/system 4. Escape removes native focus while preserving selection 4. The paused resident stays G109. |
| Open/close existing Telemetry and Settings | Native pane suspends and resumes; egui dialogs remain operable. Settings confirms unchanged MSAA 8x. |
| Existing borderless/exclusive window controls; restart | The pane remains visible through the full-screen mode operation at 1920x1080. Restart is default-off. Reference borderless mode is restored. |
| With native disabled, select egui 2x; F8; native Disable; Tab | Native re-enable projects the same 2x owner. Disable removes the pane and clears the checkbox. Map Tab changes the camera to strategic overhead; the existing egui control restores 3D. |

Ten local proof screenshots are retained under the common git directory
`0088-ui-witness/` (01 Meridian G0, 02 shared refusal, 03 clipboard, 04 running,
05 paused G109, 06 keyboard selection, 07 modal priority, 08 settings,
09 egui fallback reflected, 10 disabled map hotkey). They are supporting local
captures; the committed procedure and owning executable tests are the portable proof.

Qualification limits: the direct desktop capture is 1920x1080; OS DPI was not
independently measured or changed. The 1.5 scale transition and RMB drag crossings
are production-dispatch test evidence, not a claim of physical multi-monitor
or held-RMB desktop qualification. The synthetic desktop Down key did not change
selection; its cause is unresolved. Actual Tab/Enter navigation passed, and raw
Bevy ArrowDown passes the resident integration test. Arbitrary desktop resize
and long-list stress are not qualified here (the pinned workload has seven rows;
the existing toolbar exposes fullscreen modes). Carry these limits into Leaf B
instead of inferring migration-grade parity from the prototype.

## Qualification and remaining leaf

Reference machine: Intel Core i9-13980HX, NVIDIA RTX 4080 Laptop GPU,
driver 32.0.15.9579; rustc 1.95.0 / LLVM 22.1.2, x86_64-pc-windows-msvc.
Build: `cargo build --release -p simthing-mapeditor --bin simthing-studio`.
Current Bevy 0.16.1 and bevy_egui 0.36 dependencies are unchanged.
Source `fnv1a64:ee4e4df9e8c9fbd9:5798`; profile
`fnv1a64:bfcbc44323b304bf:23530`; base
`fnv1a64:c49f9ca3c8c75e77:20370`; dependency manifest
`fnv1a64:2f064bfb3e043aa0:72`.

Leaf B owns M16 paused/running/editing/resize/list comparisons, raw repetitions,
instrument qualification, service-line characterization and the explicit
migration decision. Leaf A makes no comparative latency/frame/memory/GPU cost
claim and selects no migration winner. No engine/ClauseThing/spec/GPU gate,
census, authority, workload, measurement denominator or service line changes.
