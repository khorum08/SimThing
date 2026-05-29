# Workshop archive — sunset manifest

**Status:** These files are **superseded** and kept for archaeology only.  
**Do not implement from them.** Use the canonical sources listed below.

**Sunset policy updated:** 2026-05-29  
**Superseding synthesis:** [`../workshop_current_state.md`](../workshop_current_state.md)

---

## Recently archived (2026-05-19)

| Archived file | Was | Read instead |
|---------------|-----|--------------|
| `simthing_spec_sonnet_opus_handoff.md` | Opus P0 / Sonnet backlog routing (HEAD `2ff84bf`) | `workshop_current_state.md` §3 · `todo.md` · `simthing_spec_progress_log.md` |
| `capability_tree_studio_workshop.md` | 2026-05-22 studio workshop Q&A | `design_v6.5.md` · progress log |
| `tech_tree_decisions.md` | 2026-05-21 workshop decisions | progress log · `capability_tree_v1.md` |
| `soft_aggregate_tolerance_audit.md` | A-4 Opus audit (pre-#90) | `adr_accumulator_op_v2.md` · landed `SoftAggregateGuard` |
| `chatgpt_implementation_review.md` | 2026-05 arch/perf review | Historical; see `worklog.md` for subsequent fixes |

---

## Earlier archived handoffs (2026-05-23)

| Archived file | Was | Read instead |
|---------------|-----|--------------|
| `simthing_spec_master_handoff.md` | Opus master implementation handoff (PR ladder) | `simthing_spec_progress_log.md` · `design_v6.5.md` §4–§5 |
| `simthing_spec_workshop.md` | Phase 0 crate pivot worksheet | Progress log § Phase 0 · `design_v6.md` spec pivot addendum |
| `pr5_handoff_digest.md` | PR 5 capability handler acceptance digest | Progress log § PR 5 · `tests/pr5_capability_handler.rs` |
| `opus_current_state_handoff.md` | Opus parking snapshot (pre–PR 11) | `design_v6.5.md` · `todo.md` |
| `pr11_session_assembly_handoff.md` | PR 11 umbrella handoff | `adr/pr11_track_a_session_assembly.md` · progress log § PR 11 |
| `pr11_track_a_handoff.md` | Track A implementation brief | Same ADR · `simthing-driver` install/spec_session modules |
| `pr11_post_track_a_handoff.md` | Post–Track A parking | `design_v6.5.md` §5 · progress log open work |
| `composer_handoff_c2_refinement_c3.md` | C-2/C-3 refinement handoff | `worklog.md` · C-2/C-3 parity tests |
| `c1_perf_reframe_memo.md` | Duplicate copy | Active memo: [`../c1_perf_reframe_memo.md`](../c1_perf_reframe_memo.md) |

---

## Safe deletion

Archive bodies may be deleted locally once this manifest is committed, if disk space
matters. Prefer keeping them in git for teammate archaeology.

No code or tests reference filenames in this folder directly.

---

## Phase M documentation cleanup archive pass (2026-05-29)

This pass moved superseded Phase M parking packets, old review packets, the original 2A report (superseded by its R1 hygiene successor), and other clearly historical artifacts into `docs/workshop/archive/tests/` and `docs/workshop/archive/reviews/`.

| Archived file/class | Was | Read instead |
|---|---|---|
| Old `phase_m_*_parking_test_results.md` (first-slice vertical, product fixture chain, summary validity R1, etc.) | Parking/test reports superseded by Opus acceptance memos or later implementation evidence | Active guidance + latest relevant acceptance memo or implementation report (e.g. 2A R1, 2B, 2C reports) |
| Superseded `phase_m_*_review_packet.md` files (where an `acceptance_opus_review` counterpart exists) | Review packets superseded by the corresponding acceptance memo | The `*_acceptance_opus_review.md` file |
| Original `phase_m_eml_gadget_2a_snapshot_copy_test_results.md` | Original 2A report containing sequence-evidence caveats later corrected in R1 | `phase_m_eml_gadget_2a_snapshot_copy_r1_hygiene_test_results.md` |
| Various unreferenced or superseded full logs from early Phase M sandboxes | Historical logs | Latest active test report for the relevant milestone or the archive README |

The 2026-05-29 pass also improved active guidance files and archive metadata for clarity (see `phase_m_docs_cleanup_archive_test_results.md` and the R1 follow-up report).
