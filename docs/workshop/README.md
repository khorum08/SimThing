# Workshop docs index

**Parking (2026-05-23):** `master` @ `14db14e`. **Tests:** 323 passed, 3 ignored. **Cursor handoff:** complete (#56–#59). **Next:** Codex O1b → S5 → O4/O2; Opus EffectTarget ADR.

## Read first — current state

- [`../design_v6.5.md`](../design_v6.5.md) — **parking synthesis** (HEAD, open work, doc map)
- [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) — PR 1–11 + O1 implementation ledger

**Architecture decision (PR 11 Track A):**

- [`../adr/pr11_track_a_session_assembly.md`](../adr/pr11_track_a_session_assembly.md)

**Phase 1 ADRs (O2 / O4 — read before implementation; O1 landed PR #53):**

- [`../adr/game_mode_session_installation.md`](../adr/game_mode_session_installation.md)
- [`../adr/scripted_event_scope_model.md`](../adr/scripted_event_scope_model.md)
- [`../adr/spec_session_state_replay.md`](../adr/spec_session_state_replay.md)

## Source / historical (in repo — rationale only)

| Document | Role |
|----------|------|
| [`capability_tree_studio_workshop.md`](capability_tree_studio_workshop.md) | Original Claude workshop Q&A (2026-05-22); **superseded** for implementation |
| [`tech_tree_decisions.md`](tech_tree_decisions.md) | Prior session decisions; **superseded** — see progress log § approved decisions |
| [`capability_tree_v1.md`](../capability_tree_v1.md) | Capability-tree RON reference (§13 install targets, §14 v0 effect scope) |
| [`../examples/README.md`](../examples/README.md) | InstallTargetSpec RON examples (`AllOfKind`, `ScenarioListed`, `SessionRoot`) |
| [`simthing_modder_object_guide.md`](simthing_modder_object_guide.md) | Modder-facing object reference (draft, local) |

## Superseded handoffs (local archive)

Superseded PR handoffs live in [`archive/`](archive/). Handoff **bodies** are
gitignored (local only); the sunset manifest is tracked:

- [`archive/SUNSET.md`](archive/SUNSET.md) — file → canonical replacement map
- [`archive/README.md`](archive/README.md) — folder policy

**Do not implement from archived handoffs.** Use [`design_v6.5.md`](../design_v6.5.md) and
`simthing_spec_progress_log.md`.
