# Workshop docs index

**Parking (2026-05-23):** `master` @ `ce904e8`. **Tests:** 323 passed, 3 ignored. **Cursor handoff:** complete (#56–#59). **Next:** Codex O1b → S5 → O4/O2; Opus EffectTarget ADR.

## Canonical — `simthing-spec` implementation progress

**Read this first for PR 1–11 status, architecture, tests, and open work:**

- [`simthing_spec_progress_log.md`](simthing_spec_progress_log.md) — unified progress log

**Architecture decision (PR 11 Track A):**

- [`../adr/pr11_track_a_session_assembly.md`](../adr/pr11_track_a_session_assembly.md)

**Phase 1 ADRs (O2 / O4 — read before implementation; O1 landed PR #53):**

- [`../adr/game_mode_session_installation.md`](../adr/game_mode_session_installation.md)
- [`../adr/scripted_event_scope_model.md`](../adr/scripted_event_scope_model.md)
- [`../adr/spec_session_state_replay.md`](../adr/spec_session_state_replay.md)

## Source / historical (in repo)

| Document | Role |
|----------|------|
| [`capability_tree_studio_workshop.md`](capability_tree_studio_workshop.md) | Original Claude workshop Q&A (2026-05-22) |
| [`tech_tree_decisions.md`](tech_tree_decisions.md) | Prior session decisions; crate naming partially superseded |
| [`capability_tree_v1.md`](../capability_tree_v1.md) | Capability-tree RON reference (§13 install targets, §14 v0 effect scope) |
| [`../examples/README.md`](../examples/README.md) | InstallTargetSpec RON examples (`AllOfKind`, `ScenarioListed`, `SessionRoot`) |
| [`simthing_modder_object_guide.md`](simthing_modder_object_guide.md) | Modder-facing object reference (draft, local) |

## Superseded handoffs (local archive only)

Superseded PR handoffs and worksheets live in **`archive/`** on disk only. That
folder is **gitignored** and is not pushed to the remote.

If you need them locally (archaeology, decision-log detail), copy or restore
from a pre-archive commit or ask a teammate who has the files. Filenames:

- `simthing_spec_master_handoff.md`
- `simthing_spec_workshop.md`
- `pr5_handoff_digest.md`
- `opus_current_state_handoff.md`
- `pr11_session_assembly_handoff.md`
- `pr11_track_a_handoff.md`
- `pr11_post_track_a_handoff.md`

**Do not implement from archived handoffs.** Use `simthing_spec_progress_log.md`.
