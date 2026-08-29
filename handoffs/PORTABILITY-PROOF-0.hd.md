---
rung: PORTABILITY-PROOF-0
kind: rung
track: 0.0.8.7
base_sha: 253a4383c0579b04358f6852ae938e443ee0d343
audience: coding
model_tier: std
expected_route: ORCHESTRATOR-CLEARABLE
owner_approved: true
owner_notes: "Phase 12.1 is the portability proof: promote an existing Phase-11 non-game exemplar through the five-verb Vendor Door end-to-end, with zero engine edits. The network-saturation exemplar is the intended full-Triad seed. If standard Run serialization cannot be completed through the existing public embedder surface, STOP rather than widen production APIs in this proof rung."
surfaces: ["crates/simthing-embedder/tests", "docs/tests", "scripts/ci/test_inventory.tsv", "scripts/ci/anchor_reach_log.tsv", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md"]
forbidden: ["any crates/*/src production or engine edit", "any direct test dependency on simthing-driver, simthing-gpu, simthing-kernel, simthing-core, or other lower engine crate instead of simthing_embedder", "new Vendor Door verbs or facade semantics", "new serialization, replay, observation, field, RF, overlay, ActionBand, CostBand, EML, or Triad authority", "shipped scenario/corpus assets or ClauseThing as proof input", "new workflow, gate, allowlist, doctrine checker, or harness authority", "12.2 CORE-CANONIZATION-0 prose or anchor repoint work", "Vector CostBand work or any known ClauseThing baseline red"]
required_checks: ["before edits render/read this handoff; perform fresh Std coding-role orientation on the dispatch master and ACK every rendered REQUIRED-ANCHOR", "FIRST STEP archaeology the existing five-verb public surface plus crates/simthing-embedder/tests/network_saturation_triad_0.rs, finance_toy_0.rs, vendor_door_0.rs, vendor_door_triad_surface_0.rs and the Embedder Guide; identify the already-public Run serialization/replay path before authoring the witness", "promote the existing network-saturation seed into ONE standing 12.1 portability witness; the witness imports only std plus simthing_embedder and does not reach a lower engine crate directly", "make all five verbs load-bearing in the proof: Derive authors domain/specialization/law data; Populate builds/admits the tree; Overlay supplies an attributable finite-horizon domain modifier; Bind observes STEAD/Triad outputs read-only; Run initializes, executes, and serializes through the existing lifecycle", "the domain remains unrelated to every shipped SimThing scenario: no scenarios/** input, no terran-pirate corpus, no ClauseThing lowering, no game-domain vocabulary required by the engine", "prove the network domain actually simulates rather than merely compiles: at least one ordinary generation must change or produce an authoritative observable through the existing GPU/session path, and the observation must be read through the Vendor Door", "prove STEAD observation is real and read-only: consume the existing shadow/band/Triad observation seam and compare it to the authoritative session result without introducing a second decision path", "complete the rung's serialize-untouched obligation through the existing public Run lifecycle; round-trip or replay the serialized product using the already-landed public surface and prove the domain's authoritative observable/state is preserved bit-exact where the API promises bit-exact replay", "if the existing public Vendor Door cannot perform the required serialization/round-trip without a production src edit, new public export, or direct lower-crate escape hatch, STOP with the exact missing surface; do not repair the door in 12.1", "keep the existing finance and network Phase-11 exemplar tests green unmodified; the new witness may reuse their data shape but must stand as its own 12.1 proof", "run the new focused portability witness, the existing simthing-embedder exemplar/vendor-door battery, test inventory/drift if a new test is added, doctrine scan/selftest, orientation freshness, handoff lint, doc budget, agent delta scan, and diff check; return exact base, tested_code_sha, final head, and hosted workflow IDs", "zero production delta means no workspace structural certificate is owed; any production/source change is a STOP and voids this rung shape"]
stop_conditions: ["the standard five-verb Vendor Door lacks a public serialization/replay path sufficient to discharge serialize-untouched", "the witness requires importing or calling a lower engine crate directly", "the network-saturation seed cannot become an end-to-end run/observe/serialize proof without changing production source", "the proof requires shipped scenario assets, ClauseThing, or game-domain vocabulary", "the proof requires a new mechanism, authority, workflow, gate, or public API instead of consuming graduated surfaces", "completion requires 12.2 canonization, Vector CostBand, or ClauseThing-red work"]
---
## BUILD
- Implement **12.1 `PORTABILITY-PROOF-0` only** as a standing portability witness, not an engine feature.
- Use the existing Phase-11 `network_saturation_triad_0` exemplar as the seed domain because it is explicitly non-game and exercises the full Triad through the Vendor Door.
- Add one dedicated proof under `crates/simthing-embedder/tests/` that imports only `std` and `simthing_embedder`, drives Derive -> Populate -> Overlay -> Bind -> Run, executes the ordinary session path, STEAD/Triad-observes it, and completes the existing standard serialization/replay lifecycle.
- Preserve the seed exemplars unmodified where practical; the new proof must demonstrate portability end-to-end rather than broaden the domain or facade.
- The proof must show the same domain artifact/spec survives the standard lifecycle without engine edits, scenario-side plumbing, or lower-crate escape hatches.
- Keep pointer at 12.1 in coding. Return proof-present; no self-graduation.

## FENCES
- Zero production/source edits. This rung consumes the Vendor Door; it does not repair or widen it.
- No shipped scenario/corpus input and no ClauseThing path: portability is proved by an unrelated direct embedder domain.
- No direct engine-crate imports from the witness. If the facade cannot express the required lifecycle, STOP.
- Observation is read-only; decisions remain on the graduated simulation path.
- No 12.2 prose, anchor repointing, Vector CostBand, or ClauseThing-red work.

## EXIT-PROOF
- **Boundary proof:** the standing witness imports only `std` + `simthing_embedder`; no lower engine crate appears in its source or manifest dependencies.
- **Five-verb proof:** Derive, Populate, Overlay, Bind, and Run are each exercised on the same network-saturation domain artifact.
- **Simulation proof:** the domain executes through the ordinary GPU/session path and produces a nontrivial authoritative observable/state.
- **STEAD-observation proof:** the Vendor Door read seam observes the live result without becoming a decision authority.
- **Serialization proof:** the same domain serializes through the existing standard Run lifecycle and the serialized/replayed result preserves the promised authoritative state/observable exactly.
- **Portability proof:** no shipped scenario asset, ClauseThing input, game-domain adapter, engine edit, or scenario-side wiring participates.
- **Regression proof:** existing finance/network/vendor-door exemplars remain green; inventory/drift and doctrine checks remain green.
- **Confinement proof:** final delta is tests/docs/inventory evidence only; no structural certificate owed.
