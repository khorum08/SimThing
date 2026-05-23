# Workshop docs index

**Parking (2026-05-23):** `master` @ `afcbd53`. **Tests:** 326 passed, 1 ignored. **Next:** Codex O2 (replay v3).

## Read first — current state

- [`../design_v6.5.md`](../design_v6.5.md) — **parking synthesis** (HEAD, open work, doc map)
- [`../worklog.md`](../worklog.md) — O1b, EffectTarget, S5, O4 landing notes
- [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) — PR 1–11 + O1 implementation ledger

**Architecture decisions:**

- [`../adr/pr11_track_a_session_assembly.md`](../adr/pr11_track_a_session_assembly.md)
- [`../adr/capability_effect_target_scope.md`](../adr/capability_effect_target_scope.md) — Accepted
- [`../adr/scripted_event_scope_model.md`](../adr/scripted_event_scope_model.md) — Accepted (O4)
- [`../adr/game_mode_session_installation.md`](../adr/game_mode_session_installation.md) — Accepted (O1)

**Before O2:**

- [`../adr/spec_session_state_replay.md`](../adr/spec_session_state_replay.md)

## Authoring & design (workshop)

| Document | Role |
|----------|------|
| [`simthing_modder_object_guide.md`](simthing_modder_object_guide.md) | Modder-facing core authoring objects (properties, overlays, capability trees, events) |
| [`simthing_base_economic_system_working_doc.md`](simthing_base_economic_system_working_doc.md) | Base economic model working doc — resources, transfers, thresholds, fission as SimThing patterns |
| [`capability_tree_v1.md`](../capability_tree_v1.md) | Capability-tree RON reference (§13 install; §14 EffectTarget) |
| [`../examples/README.md`](../examples/README.md) | InstallTargetSpec RON examples |

## Source / historical (in repo — rationale only)

| Document | Role |
|----------|------|
| [`capability_tree_studio_workshop.md`](capability_tree_studio_workshop.md) | Original Claude workshop Q&A (2026-05-22); **superseded** for implementation |
| [`tech_tree_decisions.md`](tech_tree_decisions.md) | Prior session decisions; **superseded** — see progress log § approved decisions |

## Superseded handoffs (local archive)

Superseded PR handoffs live in [`archive/`](archive/). Handoff **bodies** are
gitignored (local only); the sunset manifest is tracked:

- [`archive/SUNSET.md`](archive/SUNSET.md) — file → canonical replacement map
- [`archive/README.md`](archive/README.md) — folder policy

**Do not implement from archived handoffs.** Use [`design_v6.5.md`](../design_v6.5.md) and
`simthing_spec_progress_log.md`.
