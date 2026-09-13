# M16 B0b application probe and Owner Verification Loop

Status: PROBATION / validity proof pending / OPEN / UNMERGED.
Authority: Board 5653633248, DA 5651557467, OVL addendum 5653843173.
HD-RECEIPT: 47010c34edf2. Base: a8c117313dbe3796aa160ba12cad1cd677bdc5e6.

This leaf implements the default-off application half of the displayed-response
instrument. It does not establish valid measured latency, B1 comparison, or a
client migration decision. Owner-run raw evidence and the independent Workshop
join are required before any quantitative sample is credited.

## Instrument boundary

`SIMTHING_M16_LATENCY_CONFIG` enables a six-response probe, alternating K=0 and
K=4 three times. F6 starts, F11 ingests one response request, F7 stops and exports
exclusive raw JSON. The response identifier selects an injective opaque marker
colour. Until the designated main frame the previous marker remains visible.
Only this measurement marker is delayed. No generation, command, resident,
observer, renderer, dependency, or presentation mode changes are introduced.

The input endpoint is QPC at the first Studio-owned read of Bevy's raw
`KeyboardInput` event in PreUpdate, before `InputSystem` and both clients' button
processing. It is application ingestion, not a device interrupt, earlier Winit
receipt, or monitor pixel response timestamp. Repeat and foreign-window events
are refused. Actual scene, resident, source/profile/dependency, client, pause,
focus, modal, window and adapter conditions qualify the capture.

Existing Bevy `present_frames` tracing enter/exit hooks bound submission QPC.
These are correlation bounds, not an assertion that the frame was displayed.
Main frame/epoch/marker identity is extracted with the actual rendered world.
The instrument checks actual egui indexed geometry or native extracted UI nodes
and prepared pipeline identity before recording a response as prepared. Existing
rendering still performs all draw and present work. Exact submission QPC and the
DISPLAYED endpoint come only from the external PresentMon row contained uniquely
inside that bound; an ambiguous or missing row invalidates the offline join.

The read-only `Studio_ops Telemetry · M16 OVL` pane uses the existing egui pass.
It deliberately does not activate the full Studio_ops modal, which suspends the
native prototype. It shows PID, adapter/backend, epochs, K, input QPC, target
frame, marker identity, prepared frame/span, and raw export status. It labels
DISPLAYED as pending the offline join. This pane is evidence scaffolding and
has no simulation or observation authority.

## Verification and remaining proof

Local Windows checks: Mapeditor cargo check; six focused M16 unit tests (five new
probe cases and the existing egui consumption case); existing B0a capture (four),
native UI (three), and real source-to-ready (one) integration cases pass. A prior
parallel-link attempt exhausted Windows linker memory; serial execution passed.
Exact code/executable identity and final verification logs accompany the OVL
bundle and Board return. Hosted Doctrine, full INSPECT and fresh Clearance remain
required on the final PR head before the handoff is complete.

The new tests exercise delayed marker continuity, real event-reader ordering,
existing tracing-span capture and clock failure, real egui tessellation including
degenerate/clipped/wrong-colour geometry, and evidence-preserving export retries.
They do not substitute for an actual Windows DISPLAYED capture.

External tool: Intel PresentMon 2.5.1 x64, SHA-256
`9bec3083069f58f911e6a512f4806db51a27bd096103087bc1d05ef54c80a191`,
from https://github.com/GameTechDev/PresentMon/releases/tag/v2.5.1.
`--qpc_time --v1_metrics` selects integer PresentStartTime (`QPCTime`),
`msUntilDisplayed`, and retained `Dropped` outcomes; v2 CPUStartQPC and
DisplayedTime are different quantities and are not substituted.

The owner bundle freezes the executable and pinned source hashes, starts a
process-filtered trace using normal Owner-approved Windows elevation, retains
CSV/stdout/stderr and run qualification, and enables F6 only after the live trace
has produced raw rows. Screenshots are taken after F7 export, avoiding focus-loss
invalidation. The Owner reports visible behavior with OWNER-OVL: PASS/FAIL; the
agent harvests raw JSON/CSV and applies the Workshop falsifier itself.

## Lifecycle

All five new test rows are AUDIT / ledger-only, birth track
0.0.8.8-integrated-rehearsal, DSU 0, boundary 0088-UI-PROTOTYPE-0.
Delete the probe module, its log/plugin/UI hooks and proof scaffolding when the
1.3 experiment is retired. No protected test, gate, census, anchor or class row
is changed. Mapeditor presentation and Workshop measurement are independent
leaves of their existing rehearsal classes.
