---
rung: ANCHOR-TABLE-SURFACE-0
kind: rung
track: 0.0.8.7
base_sha: 0c17998c7bcc08ce8056737fe0ec706ee1bfd4df
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-authorized rung 5.3. Std lane: Grok CLI (grok-4.5 pinned). Orchestrator-authored and self-issued under DA ruling 5118920013 plus Owner lane correction 5118942221. Build P0(e) fulcrum 3 only: one derived GPU anchor table and consumer migration; 5.4+ Field Sweep and Phase 6 transport remain fenced."
surfaces: ["crates/simthing-core/src", "crates/simthing-kernel/src", "crates/simthing-gpu/src", "crates/simthing-sim/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-mapeditor/src", "crates/simthing-mapeditor/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["FIELD-SWEEP rungs 5.4-5.8 or semiring/adjacency work", "Phase 6 event-generation stamps, async queues, rings, or backpressure redesign", "CPU reconstruction of bands, urgency, or crossings from raw values", "second listener, observation table, feature cache, or consumer-specific GPU index path", "CPU decision branching on anchor-table or field reads", "band edges entering field accumulation, falloff, or RF conservation math", "heterogeneous matrix cells or new raw SlotIndex/ColumnIndex/RoleOffset doors"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "adapter-pinned cargo test -p simthing-kernel", "adapter-pinned cargo test -p simthing-sim", "adapter-pinned full simthing-driver battery", "adapter-pinned simthing-mapeditor observation referees", "anchor-table GPU/oracle parity and remap referees", "RF-1 and replay determinism", "agent-scan and doctrine-scan", "observation-bypass census, inventory, orientation, doc-budget, anchors, clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "table state cannot be updated by admission, the fused write door, and typed remaps alone", "last-crossing generation requires Phase 6 transport redesign rather than table-local dispatch metadata", "consumer migration requires a second observation authority"]
---
## BUILD
- Census every admitted Anchored locus and every production observation consumer. Define one typed anchor-table schema carrying stable anchor identity, current slot/column/role locus, current band, last-crossing generation, urgency, and admitted falloff parameters; encode raw GPU POD only at the governed boundary.
- Build the GPU-resident table deterministically from canonical admission state: exactly one row per active Anchored locus, no row for `Unobserved`, stable ordering, and no caller enrollment. Structural operations apply the existing typed `AnchorRemapSection` so identity survives slot/column movement without duplicate, stale, or missing rows.
- Update dynamic table fields only inside the unified fused write/threshold path. Reuse the same ordered edge evidence that mints `BandCrossingDelta`; multi-edge jumps resolve deterministically, no-crossing writes preserve crossing generation, and current generation reaches the table as dispatch metadata only—not as a Phase 6 event-transport redesign.
- Expose one compact typed anchor-table readback/snapshot door. Repoint `hosted_property_observation`, Studio field/disruption readouts, and production telemetry/metrics to it; remove or proof-fence raw `GpuValuesSnapshot`/full-value observation paths. Internal boundary mutation/replay readback may remain, but cannot be a consumer observation authority.
- Add load-bearing referees: admission cardinality and `Unobserved` exclusion; rising/falling/multi-edge/no-crossing GPU parity; remap identity preservation under fission/fusion/AddChild/reallocation; Studio/telemetry exact reads from the table; and a census proving zero production consumer bypass or band reconstruction.
- Land one signal-only results doc; stamp 5.3 PROBATION and advance posture to `FIELD-SWEEP-IR-PROBE-0`; regenerate orientation and any co-evolved scan/allowlist digest.
## FENCES
- Quantize the reading, never the field: the table observes ordinary homogeneous-lane properties and must not alter field math, RF envelopes, threshold semantics, or replay authority.
- Admission, the fused write door, and typed remaps are the only table writers. No listener framework, CPU shadow table, consumer-specific cache, raw public matrix snapshot, or independently reconstructed band state.
- Fulcrum 3 only: no domain-generic ladder expansion beyond the existing ordered-scalar paths, no Field Triad map/fold implementation, and no Phase 6 egress/ingress transport work.
## EXIT-PROOF
- Canonical TP installation produces exactly the live Anchored inventory in one GPU table (`25` Anchored, `0` Unobserved at dispatch baseline); structural churn preserves stable identity and exact cardinality, while crossings update band/generation/urgency bit-exact against the independent oracle.
- Studio and telemetry read only the typed table; a production-path census finds zero raw-value observation, consumer-specific indexing, CPU band reconstruction, or second-table authority.
- Workspace/core/kernel/sim/full adapter-pinned driver and mapeditor observation corpus, RF-1, replay, scans, censuses, inventory, orientation, doc-budget, anchors, and clearance are green; PROBATION and the 5.4 pointer land in-diff.
