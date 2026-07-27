---
rung: OVERLAY-EFFECT-HOST-ADMISSION-0
kind: rung
track: 0.0.8.7
base_sha: f47b9e06c6e4f67e061a6d754dfa21ff6b96db74
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 2.1, Phase 2 opener (P2 four-family overlay law). Frontier lane (Codex 5.6). Closes the DOCUMENTED v0 trap in adr/capability_effect_target_scope.md: overlay-prep applies transforms by TREE POSITION, not overlay.affects — a capability effect authored against the wrong host silently lands on a column nobody reads. Silence becomes a spanned admission error."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-spec/src", "crates/simthing-driver/tests", "crates/simthing-clausething/src", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/adr/capability_effect_target_scope.md", "docs/capability_tree_v1.md", "docs/tests"]
forbidden: ["changing overlay-prep's tree-position application semantics (the kernel walk stays; ADMISSION is where truth is enforced)", "behavior changes to correctly-hosted existing overlays (referees unedited)", "a second modification mechanism or listener path (core design §6: overlays are the only modifier)", "new ColumnIndex mints outside the 0.1 doors"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "an-existing-production-overlay-is-discovered-mis-hosted (means live silent misdelivery exists — STOP, DA-route with the evidence)", "admission-check-requires-kernel/WGSL-changes"]
---
## BUILD
- Close the effect-target v0 trap AT ADMISSION (ANCHOR-ACK core-0087; ADR
  `capability_effect_target_scope.md` is the contract): for every overlay, admission verifies
  the resolved host (per `EffectTarget` Owner / CapabilityTree / SessionRoot via
  `overlay_hosts`) actually CARRIES the target property (registered + layout-resolvable
  through the role pathway). A transform against a host lacking the property is a **spanned
  hard error** naming overlay id, resolved host, property key, and source span — never a
  silent no-op landing on an unread column.
- `overlay_hosts` becomes the admission-checked canonical placement path (install layer
  already computes it; admission now PROVES it): host placement, property seeding, and
  `affects` documentation-consistency validated together; divergence between placement and
  resolved target = spanned error.
- Misdelivery fixture (exit-proof-bound): a capability effect authored against a host lacking
  the property hard-errors at admission with the correct span; a correctly-hosted twin admits
  and transforms.
- Doctrine-CI co-evolution rides the PR; stamp 2.1 exit-proof cell + advance posture row to
  `CAPABILITY-PREREQ-DAG-ADMISSION-0` in-diff; regen orientation.
## FENCES
- Admission-side only: the GPU overlay-prep walk is untouched; zero behavior deltas for
  correctly-hosted overlays (full battery green, referees unedited). Greenfield charter
  applies to the admission-check design.
## EXIT-PROOF
- Misdelivery fixture spans at admission; correctly-hosted twin admits; existing capability
  install/activation tests green unmodified; workspace build + full driver battery on live
  GPU green; ADR §14 warning superseded-note updated (doc, not code). Stamp + posture
  advance in-diff per the graduation ritual.
