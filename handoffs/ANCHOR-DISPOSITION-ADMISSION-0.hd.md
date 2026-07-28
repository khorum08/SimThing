---
rung: ANCHOR-DISPOSITION-ADMISSION-0
kind: rung
track: 0.0.8.7
base_sha: 15149ff5b771fffb04cb6394cdb081bd6fef9f2e
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Owner-committed rung 5.1. Frontier lane: Codex 5.6/Fable; DA review and issuance required before dispatch. Greenfield discretion charter applies. Build only P0(e) fulcrum 1: disposition at admission; 5.2 write-door deltas, 5.3 anchor table, and 5.4+ field sweep are out of scope."
surfaces: ["crates/simthing-core/src", "crates/simthing-spec/src", "crates/simthing-sim/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["WRITE-DOOR-BAND-DELTA-0 or structural anchor-remap implementation", "ANCHOR-TABLE-SURFACE-0 GPU table or consumer repointing", "FIELD-SWEEP rungs 5.4-5.8", "kind-gated anchoring or runtime consumer/listener enrollment", "shader/WGSL/repr(C)/matrix-layout changes", "new raw ColumnIndex/SlotIndex/RoleOffset doors", "CPU decision branching on observed values"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "cargo test -p simthing-spec", "cargo test -p simthing-sim", "adapter-pinned full simthing-driver battery", "agent-scan and doctrine-scan", "test-inventory and execution-status census", "orientation-check, doc-budget, anchor-check, clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "total disposition requires a second property registry or listener framework", "default anchoring changes RF/replay bytes", "fulcrum 1 cannot land without 5.2 or 5.3 semantics"]
---
## BUILD
- Add typed admission disposition for every resource-bearing property: omitted authoring => `Anchored`; the only opt-out is authored `Unobserved { reason }`, with a non-empty reason and source span. No deferred, unresolved, or waiting-for-consumer state.
- Carry disposition through ordinary spec compile/hydrate/install reporting on canonical property identity and the registry/role pathway; no optional enrollment, listener, or sidecar authority. Existing authored corpora remain unchanged and default all resource cells to anchored.
- Publish a deterministic dark-cell inventory from live admitted state to Board/orientation: counts plus stable property identities and reasons, generated rather than hand-maintained. This is governance visibility, not a second observation API.
- Add minimal load-bearing referees: omission defaults anchored; explicit opt-out preserves reason/span and appears in the dark inventory; blank reason hard-errors; canonical TP installation proves every resource property has exactly one disposition and no 12.3-style walled/deferred residue.
- Land one signal-only results doc; stamp 5.1 PROBATION and advance the pointer to `WRITE-DOOR-BAND-DELTA-0`; regenerate orientation and any co-evolved anchor/scan digest in the same PR.
## FENCES
- Fulcrum 1 only: do not implement band-crossing mutation/write-door work (5.2), the GPU-derived anchor table or consumer repointing (5.3), or field-sweep work (5.4-5.8).
- Default anchoring is property-driven, never kind-driven. `Unobserved` is explicit authored data with a reason, not a runtime toggle, consumer absence, or `DefaultDisabled` alias.
- Preserve RF-1, wire/replay determinism, row/column/slot authority, and homogeneous lanes. No shader/WGSL/layout change, second observation/read seam, or CPU decision authority.
## EXIT-PROOF
- Zero-wiring canonical installation yields a total disposition over every resource property; the default anchored count matches the live corpus, and the explicit `Unobserved` fixture is the only dark-cell residue.
- Admission negatives bite with spans; the generated Board dark-cell surface is fresh; workspace/core/spec/sim plus adapter-pinned full driver corpus and scans/inventory/orientation/doc-budget/anchors/clearance are green.
- PR remains PROBATION for DA deep-tree graduation; no 5.2 dispatch.
