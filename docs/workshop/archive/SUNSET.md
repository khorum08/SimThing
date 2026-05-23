# Workshop archive — sunset manifest

**Status:** These files are **superseded** and kept on disk for archaeology only.  
**Do not implement from them.** Use the canonical sources listed below.

The archive folder is **gitignored** except this manifest (and `README.md`). Handoff bodies
are not pushed to the remote; teammates restore from local copies or pre-archive commits if
needed.

**Sunset date:** 2026-05-23  
**Superseding synthesis:** [`docs/design_v6.5.md`](../../design_v6.5.md)

---

## File → canonical replacement

| Archived file | Was | Read instead |
|---------------|-----|--------------|
| `simthing_spec_master_handoff.md` | Opus master implementation handoff (PR ladder) | [`simthing_spec_progress_log.md`](../simthing_spec_progress_log.md) · [`design_v6.5.md`](../../design_v6.5.md) §4–§5 |
| `simthing_spec_workshop.md` | Phase 0 crate pivot worksheet | Progress log § Phase 0 · [`design_v6.md`](../../design_v6.md) spec pivot addendum |
| `pr5_handoff_digest.md` | PR 5 capability handler acceptance digest | Progress log § PR 5 · `tests/pr5_capability_handler.rs` |
| `opus_current_state_handoff.md` | Opus parking snapshot (pre–PR 11) | [`design_v6.5.md`](../../design_v6.5.md) · [`todo.md`](../../todo.md) |
| `pr11_session_assembly_handoff.md` | PR 11 umbrella handoff | [`adr/pr11_track_a_session_assembly.md`](../../adr/pr11_track_a_session_assembly.md) · progress log § PR 11 |
| `pr11_track_a_handoff.md` | Track A implementation brief | Same ADR · `simthing-driver` install/spec_session modules |
| `pr11_post_track_a_handoff.md` | Post–Track A parking | [`design_v6.5.md`](../../design_v6.5.md) §5 · progress log open work |

---

## In-repo historical docs (not in this folder)

These remain in git for Q&A rationale but are **not** implementation sources:

| File | Role |
|------|------|
| [`capability_tree_studio_workshop.md`](../capability_tree_studio_workshop.md) | 2026-05-22 studio workshop Q&A |
| [`tech_tree_decisions.md`](../tech_tree_decisions.md) | 2026-05-21 decisions (crate naming superseded) |

---

## Safe deletion

Local archive `.md` bodies may be deleted once this manifest is committed. No code or tests
reference filenames in this folder.
