---
rung: RESOLUTION-SITE-SPLIT-0
kind: rung
track: 0.0.8.7
base_sha: eda4a775
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "DA ruling 5174090192 graduates 5.10 and returns the pointer to 6.2b. Standing delegation authorizes orchestration to author this handoff because exit-proof coverage does not flag 6.2b; #1601 already widened its proof. This is PRE-DISPATCH only: draft, self-clear, relay to DA; no coder implementation until DA reviews/merges and orchestration explicitly dispatches. Sol is rate-limited; coder identity is resolved only at dispatch."
surfaces: ["crates/simthing-core/src/automaton.rs", "crates/simthing-core/src/overlay.rs", "crates/simthing-core/tests", "crates/simthing-sim/src/boundary.rs", "crates/simthing-sim/tests", "crates/simthing-driver/src/session.rs", "crates/simthing-driver/src/spec_session.rs", "crates/simthing-driver/tests", "crates/simthing-kernel/src", "crates/simthing-kernel/tests", "scripts/ci/test_inventory.tsv", "docs/tests", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md"]
forbidden: ["a second execution model, second semantic vocabulary, or CPU-vs-closed-loop fork in meaning; resolution site is placement only", "in-shader SimThingId lookup/use, synthesized/default Overlay.origin, or any slot->id fallback; closed-loop origin stays in slot space and total re-attachment happens only at the barrier", "moving fission/fusion/pre-grow/structural allocation into shader space, modifying the 13-stage allocation pipeline, or bypassing barrier allocation", "new transport/queue/replay recorder, direct affects attachment, identity inference, or weakening 6.0b/6.1/6.1b/6.2 laws", "flag-day conversion, scenario/domain witnesses, gate/invariant weakening, 6.3 soak/staleness-field work, movement/contention/downstream implementation"]
required_checks: ["fresh coding ORIENT-RECEIPT + current ANCHOR-ACKs + HD-RECEIPT quoted before edits", "BEFORE resolution-site code: run representative existing telemetry soak and record readback/total upper bound plus allocation-stage floor as evidence; no performance threshold invented", "convert semantics incrementally and prove identical crossings yield BIT-IDENTICAL BoundaryRequest streams at both resolution sites, with a planted defect per converted semantic", "closed-loop overlays carry origin in slot space; barrier slot->SimThingId re-attachment is total/fail-closed, and a missing/unadmitted slot planted mutant REDs", "prove no in-shader path requires SimThingId; FissionTrigger and all allocation/structural stages remain barrier-only and the 13-stage pipeline is unmodified", "reception works at both sites; synthetic-only affected crate batteries + inventory/residue/detachability/lifecycle/orientation/anchor gates green"]
stop_conditions: ["any closed-loop semantic requires SimThingId before the barrier rather than admitted slot identity", "slot-space origin cannot re-attach totally/fail-closed without a synthesized/default identity or second identity authority", "bit-identical BoundaryRequest parity requires changing semantics rather than relocating identity re-attachment", "completion requires moving allocation into shader space, changing the 13-stage pipeline, adding a second transport/model, weakening inherited law, or beginning 6.3/downstream work"]
---
## BUILD
- Make closed-loop resolution the default SimThing execution placement; retain CPU-authoritative resolution as the vendorized barrier placement of the SAME model, not a second system.
- Measure first: land existing per-stage telemetry evidence before resolution-site code. Record readback/total as the removable upper bound and allocation stages as the barrier-only floor; architecture does not depend on a speedup threshold.
- Convert boundary semantics incrementally. Where CPU dispatch only re-attaches identity, keep closed-loop execution in slot space and let that CPU arm evaporate; keep genuine allocation (notably FissionTrigger pre-grow and structural stages) at the barrier in both modes.
- Carry Overlay origin in slot space inside the loop; re-attach required SimThingId at the barrier through the admitted slot map. Reception/routing semantics from 6.0b remain identical at both sites.
## FENCES
- One model, two resolution sites; same field math, crossings, CostBands, BoundaryRequest vocabulary and barrier allocation.
- No SimThingId is needed in-shader. Slot->id at the barrier is total and fail-closed; never synthesize/default origin.
- Preserve 6.0b/6.1/6.1b/6.2. No second queue/recorder/transport, no 13-stage pipeline rewrite, no 6.3 or downstream work.
## EXIT-PROOF
- Prerequisite telemetry evidence exists before resolution-site implementation.
- For each converted semantic, identical crossings produce BIT-IDENTICAL BoundaryRequest streams in closed-loop and CPU-authoritative placements; planted semantic divergence REDs.
- Closed-loop origin remains slot-space until the barrier; missing/unadmitted slot re-attachment REDs instead of producing a default origin; grep/proof shows no in-shader SimThingId path.
- The 13-stage pipeline is unmodified and allocation remains barrier-only in both modes; conversion is incremental, synthetic-only, and returns PROBATION / DA-review-pending. Coder does not `/clearance`, merge, move pointer, or begin 6.3.
