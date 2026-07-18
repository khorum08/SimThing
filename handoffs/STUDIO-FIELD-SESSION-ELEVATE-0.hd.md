---
rung: STUDIO-FIELD-SESSION-ELEVATE-0
kind: rung
track: 0.0.8.6
base_sha: c0e2202694bc9c2a329e5a6d4620a078c7ba71ea
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "RF-3 is DA-graduated and merged in #1412 at d42b9109; merge stamp c0e22026. RF-4 resumes 12.9 on the ordinary recursive Arena RF substrate. PR #1405 remains open as salvage-only provenance and must close only after RF-4 has mined it. Orchestration owns review/remands/merge; Owner alone closes [OVL] with screenshots."
surfaces: ["crates/simthing-mapeditor/src", "crates/simthing-mapeditor/tests", "crates/simthing-workshop/tests", "crates/simthing-workshop/Cargo.toml", "crates/simthing-clausething/src", "crates/simthing-spec/src", "docs/design_0_0_8_6_studio_live_ops.md", "docs/tests", "docs/orchestrator_orientation.md", "scripts/ci/test_inventory.tsv", "scripts/ci/inspect_justifications.tsv", "scripts/ci/triage_log.tsv"]
forbidden: ["merge, rebase, or wholesale cherry-pick PR #1405", "Studio-side RF arithmetic, synthetic parent growth, feeder patches, or direct accumulator mutation", "RUNTIME-0080 RR-3/RR-4 production transplant", "new kernel/WGSL primitive, accumulator role/combine/gate, grammar, planner, or scenario API", "parallel owner hierarchy or synthetic Owner shells replacing the authority tree", "treat emission/source_slot/source_col telemetry as RF proof", "claim [OVL] closure without Owner screenshots"]
required_checks: ["cargo-check", "focused-tests", "gpu-proof", "studio-build", "agent-scan", "doctrine-scan", "orientation-check", "test-inventory", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "salvage-requires-wholesale-1405-history", "need-install-requires-new-grammar-or-primitive", "field-bearing-profile-cannot-carry-admitted-resource-flow-on-existing-surfaces", "authority-root-cannot-be-used-without-parallel-hierarchy", "ancestor-aggregate-proof-collapses-to-emission-or-one-to-one-copy"]
---
## BUILD
- First reconcile governance: stamp RF-3 **DA-GRADUATED / merged #1412 @ `d42b9109`**, make RF-4 / 12.9 the active dispatched rung, and regenerate orientation. Preserve the master merge stamp in `docs/tests/rf_legacy_retire_reanchor_0_results.md`.
- Treat PR #1405 at exact head `34bb730fb24b550eeac520e83fbf5e2408a0f7c4` as a **salvage source only**. Produce a salvage manifest that names each accepted or rejected file/hunk. Mine only the accepted loader/profile/path/telemetry work; do not merge, rebase, or wholesale cherry-pick #1405. Leave #1405 open until orchestration confirms the mining is complete, then orchestration closes it.
- Rebase the field-bearing Studio path conceptually on the landed contract: `ResourceFlowExecutionProfile::RecursiveArenaResourceFlow` is the ordinary default, `SimSession::step_once` executes the admitted Arena plan, and unchanged RF-1 judges conservation. Remove the old “missing production RF seam” assumption from salvaged framing.
- Compose the field-bearing Studio session from the canonical authority tree (`authority_root` or its existing one-tree equivalent) and an admitted `ResourceFlowSpec` / ArenaRegistry derived from already-authored 12.6/12.8 data. Do not retain synthetic Owner shells or a location-only proxy as RF authority.
- Transport existing authored need / `weight_profile` stacks through an already-admitted GameMode/open seam. If this requires a new grammar, primitive, or authority contract, STOP and propose a bounded RF-5 split; do not invent the seam inside Studio.
- Bind Studio telemetry to actual live ancestor/Owner aggregate columns after ordinary ticks. Keep emission / `source_slot` / `source_col` rows explicitly diagnostic and visually distinct.
- Mine or rewrite tests from #1405 onto the new base. Preserve structural-shell fallback, production clause ingest, field-bearing open, pause semantics, and threshold-only decisions, but replace emission-only claims with recursive-RF proofs.

## FENCES
- Generic RF/STEAD pipeline only. No bespoke economy loop in the Studio tick and no UI/Bevy mutation of ScenarioSpec.
- No new ClauseScript syntax or sealed-crate TP special case. Scenario-flavored proofs remain workshop-homed.
- #1405 evidence is historical diagnostic input, not current proof. Every load-bearing claim must be reproduced on the RF-4 branch against the post-RF-3 base.
- Owner [OVL] is a separate visual gate. Code may reach PROBATION before screenshots, but only the Owner closes [OVL].

## EXIT-PROOF
- Production Studio clause ingest opens the canonical scenario through the field-bearing path and ordinary `SimSession::step_once` executes `RecursiveArenaResourceFlow`; structural-shell fallback remains selectable and frozen when chosen.
- One named participating child source causally increases one named ancestor/Owner **sibling aggregate** through admitted Arena RF. Paired control disables only that child while sibling participation and all other inputs remain fixed; the ancestor differential equals the child’s admitted marginal contribution. The ancestor is not a one-to-one child copy.
- The same run passes unchanged RF-1 over measured GPU Balance and fails meaningfully when the governed Balance connection or the named child enrollment is removed.
- Studio telemetry reports session path, named child, named ancestor aggregate, before/after values, and RF activity from live state. Emission-locus rows cannot satisfy this proof.
- Existing authored need / `weight_profile` stacks affect the live field-bearing session through an admitted seam, or an orchestrator-approved RF-5 split is recorded before RF-4 graduation; no silent omission.
- Decisions fire only on declared FIELD_POLICY threshold crossings; a paired below-threshold control fires nothing.
- Focused mapeditor and workshop tests bite; Studio build is green; no new untriaged test inventory drift; agent/doctrine/orientation/doc-budget checks are green.
- Results and PR metadata bind tested code SHA, evidence head, HD receipt, ORIENT receipt, #1405 salvage manifest, and Owner [OVL] status. #1405 remains open until orchestration confirms all accepted salvage has landed, then orchestration closes it.
