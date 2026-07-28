---
rung: PLAN-STRUCT-TYPING-0
kind: rung
track: 0.0.8.7
base_sha: 0342a28cce8ca891bc283e8ad88d1264d7eee2ba
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-approved rung 4.2. Std lane: Grok CLI (grok-4.5 pinned). Orchestrator-authored under ORCH-HANDOFF-AUTHORSHIP-REMEDIAL-0; issued directly per DA amendment. Preserve the 4.1 wgsl_encode boundary and do not absorb the 9.2 sweep."
surfaces: ["crates/simthing-core/src/column_index.rs", "crates/simthing-kernel/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["Family C gated_rates or EML work", "first_slice_mapping_runtime deletion/replacement", "9.2 legacy ColumnIndex::new/raw-oracle sweep or exclusion-list retirement", "shader/WGSL semantic or layout changes", "authored/wire slot or column serialization changes", "new raw-u32 plan fields or public raw ColumnIndex doors"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "adapter-pinned cargo test -p simthing-kernel", "adapter-pinned cargo test -p simthing-sim", "adapter-pinned full simthing-driver battery", "agent-scan and doctrine-scan", "test-inventory and execution-status census", "orientation-check, doc-budget, anchor-check, clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "typed plan requires a WGSL semantic/layout change", "Family B inventory exposes a genuine Family C violation"]
---
## BUILD
- Complete exclusion-audit Family B: inventory every production plan/compile/admission path carrying matrix column identity through GPU round trips (arena allocation, resource-economy, transfer/emission/intensity accumulators, silo/link compiles, region-field admission and equivalents).
- Replace raw `u32` column fields in CPU-side plan structs and intermediate compile records with `ColumnIndex` end-to-end. Optional/sentinel columns remain typed (`Option<ColumnIndex>` or an existing typed state) until encoding.
- Make `simthing-kernel/src/wgsl_encode.rs` the single raw drop/re-materialization boundary: all POD/WGSL rows encode `ColumnIndex` there; production `from_gpu_round_trip`/`raw_u32` use outside that module collapses to zero.
- Add load-bearing referees for typed plan construction and byte-for-byte pre-rung wire/layout parity, including optional/sentinel columns; add a production census proving no raw column plan field or round-trip mint survives outside the boundary.
- Land one signal-only results doc; stamp 4.2 PROBATION and advance posture to `ANCHOR-DISPOSITION-ADMISSION-0`; regenerate orientation.
## FENCES
- Preserve the 4.1 object-residency/row authority and the registry role pathway. `ColumnIndex`, `SlotIndex`, and `RoleOffset` remain non-interchangeable.
- This is Family B only: no Family C EML/gated-rate work, no hardcoded-slice deletion, and no 9.2 legacy mint/exclusion-list retirement. Oracle/rehearsal raw construction remains independently fenced.
- Bit-exact WGSL bytes, buffer layouts, RF-1/replay/determinism, adapter selection, and existing semantic tests remain unchanged; no shader semantic edit.
## EXIT-PROOF
- Zero production plan/intermediate column identities stored as raw integers outside `wgsl_encode`; zero production GPU round-trip mints outside that boundary; the Family B census accounts for the displaced ~40 mints.
- Typed-plan referees and wire-parity falsifier green; workspace/core/kernel/sim/full adapter-pinned driver corpus green; scans/census/inventory/orientation/doc-budget/anchors green; PROBATION + next-pointer stamps land in-diff.
