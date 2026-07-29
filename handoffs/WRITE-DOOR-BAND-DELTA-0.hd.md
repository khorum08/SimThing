---
rung: WRITE-DOOR-BAND-DELTA-0
kind: rung
track: 0.0.8.7
base_sha: 77ea7f12a933b5f0362afdaa4edf6970b4339ffc
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-committed rung 5.2. Std lane: Grok CLI (grok-4.5 pinned). Orchestrator-authored and self-issued under DA ruling 5111074281. Build P0(e) fulcrum 2 only; 5.3 anchor-table consumers and 5.4+ Field Sweep work remain fenced."
surfaces: ["crates/simthing-core/src", "crates/simthing-kernel/src", "crates/simthing-gpu/src", "crates/simthing-sim/src", "crates/simthing-feeder/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["ANCHOR-TABLE-SURFACE-0 table construction or consumer repointing", "FIELD-SWEEP rungs 5.4-5.8", "CPU post-hoc band rescans or CPU decision branching on observed values", "second threshold/listener/observation authority", "band edges entering accumulation, falloff, or conservation math", "structural slot/column movement encoded without a complete anchor remap", "new raw SlotIndex/ColumnIndex/RoleOffset doors outside wgsl_encode"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "adapter-pinned cargo test -p simthing-kernel", "adapter-pinned cargo test -p simthing-sim", "adapter-pinned full simthing-driver battery", "band-delta CPU-oracle parity and structural-remap negative referees", "RF-1 and replay determinism", "agent-scan and doctrine-scan", "test-inventory and execution-status census", "orientation-check, doc-budget, anchor-check, clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "band impact cannot be derived inside the existing fused mutation pass", "stable anchor identity requires the 5.3 observation table", "remap enforcement requires a second structural authority"]
---
## BUILD
- Census every production mutation of an Anchored property store and every operation that can move or reallocate its row/column identity. Bind impact to the existing AccumulatorOp/fused write door and object-residency pathway; no bypass list may remain unexplained.
- Add a typed band-crossing delta contract derived from admitted anchor identity plus the existing ordered threshold registrations. In the same GPU mutation pass, compare pre/post values and emit every crossed edge with deterministic direction/order, including multi-edge jumps; no later CPU value scan may infer impact.
- Carry the sealed deltas through the existing emission/boundary/replay transport as write-impact evidence only. Do not construct the 5.3 GPU anchor table, expose a new read API, or repoint Studio/telemetry/field consumers.
- Add a typed anchor-remap section to structural encoding. Fission, fusion/removal, AddChild/import, dimension/table reallocation, and any equivalent slot/column-moving operation must fail before GPU encoding when an Anchored store lacks a complete remap; stable-slot reparent must prove that no remap is required.
- Add load-bearing referees: fused GPU deltas versus an independent CPU oracle for rising/falling, exact-edge, no-crossing, and multi-edge cases; remap-less structural negatives with operation/source context; slot-churn remap completeness; replay bit-exactness; and a census proving zero production CPU-rescan or remap-free relocation doors.
- Land one signal-only results doc; stamp 5.2 PROBATION and advance posture to `ANCHOR-TABLE-SURFACE-0`; regenerate orientation and any co-evolved doctrine/scan digest.
## FENCES
- Quantize the reading, never the field: threshold edges do not alter accumulation, falloff, RF conservation, or homogeneous lane storage. Reuse the admitted threshold/EML machinery; do not create a parallel ladder or listener framework.
- Anchor, object, slot, column, and role identities stay typed end-to-end. Raw POD conversion remains confined to the governed WGSL encode/decode boundary; oracle/rehearsal independence stays fenced.
- Fulcrum 2 only: no derived anchor table, no observation consumer migration, no domain-generic ladder expansion, and no Field Triad map/fold work.
## EXIT-PROOF
- Every production Anchored-store write derives correct deterministic band deltas inside the fused pass, with zero CPU post-hoc inference; the GPU result is bit-exact against the independent oracle.
- Every production slot/column-moving structural op carries a complete typed remap or hard-rejects before encoding; remap-less reallocation is spanned/contextual, stable-slot reparent remains churn-free, and replay reproduces deltas plus remaps bit-exactly.
- Workspace/core/kernel/sim/full adapter-pinned driver corpus, RF-1, replay, scans, censuses, inventory, orientation, doc-budget, anchors, and clearance are green; PROBATION and the 5.3 pointer land in-diff.
