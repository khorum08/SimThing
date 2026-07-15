# 0.0.8.4.8.4 — HD Board: Handoff Dispatch, Owner Interface & Library Stewardship

> **Status: CLOSED (2026-07-15, HD-CLOSEOUT-0).** All 9 rungs graduated; the handoff-board dispatch
> substrate, owner interface, librarian, per-role library catalog, pointer-lifecycle gate, and
> park/redirect state are live. Closeout reaped 8 graduated `.hd` objects + 2 expired prior-track
> manifests + 10 lease rows, folded ladder cell history to stamps, and discharged both bindings; net
> corpus prose DECREASED. Historical record only — this workplan is done. Pointer moved on to the
> next active track. Formerly: OPEN / Owner-directed (2026-07-12).
>
> **Why.** The harness rustified verification (types, scans, router, anchors) but doctrine still
> travels between agents as retyped prose billed to metered models: long handoffs, required-reading
> lists, restated dicta, and relays whose consistency is trusted rather than proven. The exit-interview
> analysis (2026-07-12) located the remaining burn precisely: webchat orchestrator *production* is
> ~free; metered-agent *ingestion* is the bill. This track mechanizes the handoff layer the same way
> clearance and anchors were mechanized — one handoff object, generated role projections, receipt-
> proven consistency — while making Owner participation *simpler* than today, prompt-native, and
> usable by future collaborators who will never edit a repo file.
>
> **Roles:** Fable (or step-in) = DA. Codex = orchestrator + Owner's scribe. Grok = implementer.
> **Shape:** narrow rungs, results ≤60 lines, stamps ride the rung PR, close via `track_closeout.sh`.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base:** `simthing_core_design.md` §1.2 · `design_0_0_8_3.md` §0 (§0.6, §0.9) · this file ·
`ci_screening_surface.md` · `track_closeout_protocol.md` · `handoff_template.md` (authoring surface;
edits gate-wiring). **Held decisions:** no RELIABLE weakening; no new crate/framework; growth without
retirement = regression; attestations the router cannot diff-verify stay banned; meta-objects face the
Necessity Test; every new table carries a decay rule at birth; machine state over prose.

**Binding conditions (record at open):**

| rung | condition | status |
|---|---|---|
| HD-TRACK-OPEN-0 | blocked-until-owner-parks-0.0.8.6-and-directs-regeneration | discharged (Owner 2026-07-12) |
| HD-CLOSEOUT-0 | net corpus prose must DECREASE at close (template + reading-list deletions ≥ additions) | active |

---

## 1. Design (binding for all rungs)

**1.1 The handoff object.** `handoffs/<RUNG-ID>.hd.md`: machine frontmatter (rung, kind:
`rung|transport|remedial|stamp`, track, base_sha, audience, model_tier, expected_route,
`owner_approved`, `owner_notes`, surfaces, forbidden, required_checks, stop_conditions) + a
**delta-only body ≤80 lines** (BUILD / FENCES / EXIT PROOF only). Doctrine is never restated:
the dispatcher runs declared `surfaces` through the existing `anchor_triggers.tsv` and emits
`REQUIRED-ANCHORS: <ids>`; agents resolve on demand via `anchor_query.sh` (reach-logged).
Required-reading lists are retired.

**1.2 Projections + receipt.** `handoff_dispatch.sh --render coding|orchestrator|da` generates the
role view from the one file (coding: build/fences/exit-proof + anchors; orchestrator: routing/
sequence/merge authority; da: audit targets + residue). Every projection carries an **HD-RECEIPT**
(content hash, ORIENT/CLOSEOUT-receipt pattern). PR bodies and relays quote it; relay-lint compares.
Same receipt ⇒ provably same handoff. Projections are generated, so they cannot diverge.

**1.3 Owner & collaborator interface — prompt-native, file-free.** Nobody is asked to edit an
`.hd.md` by hand. Three equivalent doors, all writing the same object:
- **Prompt verbs** (to any chat agent): `approve` · `amend: <text>` · `hold` · `reject: <reason>` ·
  `status` / `board` · `dispatch`. The orchestrator is the **scribe**: it translates prose into the
  .hd mutation (flip `owner_approved`, append `owner_notes`, etc.) and pushes it. "Current handoff
  approved, implement" ⇒ the receiving agent reads its role projection and proceeds — the lint
  hard-blocks any dispatch while `owner_approved: false`.
- **GitHub comment commands** on the board issue / linked PR: `/handoff approve|amend|hold|status`
  (reuses the existing doctrine-exec comment-command machinery). Collaborators participate from the
  GitHub UI with zero repo knowledge.
- **Direct file edit** remains legal for power users; the sticky mirror re-renders on push.
`owner_notes` is the guaranteed-delivery intervention channel: its text MUST render in every
projection verbatim.

**1.4 Visibility.** The clearance workflow gains a handoff job: lint the .hd, post/update the
rendered ingress as a **sticky** on the linked PR, and sync a standing **"SimThing Board" issue**
(board-state JSON rendered ≤60 lines: track/phase/pointer, current handoff + approval state, open
PRs + routes, master head, binding conditions, leases aging). One glance = whole board, for Owner
and collaborators alike.

**1.5 Lifecycle.** .hd files are operational artifacts, never doctrine: on graduation they are
**leased** into `closeout_artifacts.tsv` (existing wall-clock reaper). Caps: body ≤80, sticky ≤60,
board digest ≤60 — enforced.

---

## 2. PR ladder (all gate-wiring / DA-reserve; model tier Std unless noted)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| HD-1 | `HD-TRUTH-GUARDS-0` | Mechanical truth guards: master-ancestry gate, `body_sha` freshness line, stamp-in-diff relay-lint. | **DA-GRADUATED / merged [#1328](https://github.com/khorum08/SimThing/pull/1328)** — evidence `docs/tests/hd_truth_guards_0_results.md`. | Std |
| HD-2 | `HD-DISPATCH-SUBSTRATE-0` | `.hd` object + role projections + HD-RECEIPT + board sync + owner_directives.tsv + lease-on-graduation (+remedial #1334 no-rung quiet ingress). | **DA-GRADUATED / merged [#1331](https://github.com/khorum08/SimThing/pull/1331)** — evidence `docs/tests/hd_dispatch_substrate_0_results.md`. | Std |
| HD-3 | `HD-OWNER-INTERFACE-0` | Prompt-native `/handoff approve|amend|hold|status`; OWNER-gated scribe mutation; guaranteed owner_notes render. | **DA-GRADUATED / merged [#1336](https://github.com/khorum08/SimThing/pull/1336)** — evidence `docs/tests/hd_owner_interface_0_results.md`. | Std |
| HD-4 | `HD-LIBRARIAN-0` | `librarian.sh --staleness|--cull|--catalog`; dry-run-default reaping; OWNER-gated confirm. | **DA-GRADUATED / merged [#1337](https://github.com/khorum08/SimThing/pull/1337)** — evidence `docs/tests/hd_librarian_0_results.md`. | Std |
| HD-5 | `HD-DOCS-CASCADE-0` | Loud onboarding cascade; `handoff_template.md` 360→112 + anti-reaccretion cap; required-reading deleted (net −198). | **DA-GRADUATED / merged [#1340](https://github.com/khorum08/SimThing/pull/1340)** — evidence `docs/tests/hd_docs_cascade_0_results.md`. | Std |
| HD-6 | `HD-POINTER-LIFECYCLE-GATE-0` | Pointer-flip refused unless outgoing CLOSED/PARKED (`--force-owner` records directive); board sync on master push. | **DA-GRADUATED / merged [#1344](https://github.com/khorum08/SimThing/pull/1344)** — evidence `docs/tests/hd_pointer_lifecycle_gate_0_results.md`. | Std |
| HD-7 | `HD-CATALOG-LIBRARY-0` | Per-role library catalog (anchors × domains × role reach, library-wide); staleness fixture-count gauge. | **DA-GRADUATED / merged [#1347](https://github.com/khorum08/SimThing/pull/1347)** — evidence `docs/tests/hd_catalog_library_0_results.md`. | Std |
| HD-8 | `HD-OWNER-AUTHORING-GUIDE-0` | Owner authoring guide + role-slot sweep (roles are slots, models are data); browser-stack protocol; cloud caveat. | **DA-GRADUATED / merged [#1349](https://github.com/khorum08/SimThing/pull/1349)** — evidence `docs/tests/hd_owner_authoring_guide_0_results.md`. | Std |
| HD-9 | `HD-PARK-REDIRECT-0` | Parked-for-redirection state: `--park`/`--unpark` move+restore track-scoped rows/.hd into one EOF block; closeout unpark-first; 0.0.8.6 migrated. | **DA-GRADUATED / merged [#1351](https://github.com/khorum08/SimThing/pull/1351)** — evidence `docs/tests/hd_park_redirect_0_results.md`. | Std |
| HD-C | `HD-CLOSEOUT-0` | Measured close: cruft reaped (8 graduated `.hd` + 2 expired prior-track manifests + 10 lease rows), ladder cells compressed, bindings discharged, prose net-negative proven. | **DA-CLOSED / this commit** — closeout report `docs/tests/hd_closeout_0_results.md`; net corpus prose DECREASED (HD-CLOSEOUT-0 discharged). | DA |

**Sequencing:** HD-1 → HD-2 → (HD-3 ∥ HD-4) → HD-5 → HD-6 → HD-7 → HD-8 → HD-C (HD-6..8 Owner-approved 2026-07-14; ONE fully automated rung at a time, Owner review between). HD-1 is independently valuable and lands
even if the rest re-parks.

---

## 3. Standing DA rulings

1. The .hd object is the record; **prompts are the interface**; GitHub is the mirror. No human is
   ever required to hand-edit a handoff file.
2. Projections are generated or they are invalid — hand-edited projections FAIL lint.
3. Doctrine travels as anchor IDs, never as restated prose in handoffs.
4. Librarian reaping inherits reaper safety: dry-run first, src/authority always DA-routed,
   nothing silent.
5. This track deletes more prose than it adds, or it does not close.
6. **Exit-proof column ownership (Owner-directed, 2026-07-12).** Ladder graduation stamps
   (`DA-GRADUATED` / `ORCHESTRATOR-GRADUATED` / `COMPLETE`) are **DA-authored, at merge**, as a DA
   commit on the rung PR after the deep pass. Implementer/orchestrator stamp **PROBATION only**;
   dispatch state (handoff issued, amendments) is stamped by the DA at dispatch. HD-2's schema
   encodes this as a DA-audience mutation; until then the live HD-1 guards catch unstamped claims. Stamps carry UTC timestamps (dispatched_at at dispatch; merge time at graduation) and a `roles:` line recording which agents filled orchestrator/coder/DA for that run.
7. **Roles are slots; models are data (Owner-directed, 2026-07-14).** Orchestrator/coder/DA are
   capability slots (webchat: Codex or Grok; DA: Claude, Codex or Grok; coder: Cursor, Grok CLI or
   Claude sonnet-class). No workflow, script, doc heading, or .hd field may condition on a vendor
   or model name; per-run assignments are recorded in dispatch stamps only.

## 4. Non-goals

No pointer change at authoring time · no new crate · no clearance-lexicon change · no replacement of
relay-lint/clearance (HD layers on them) · no dashboard beyond the ≤60-line board digest · no
weakening of DA-reserve routing.

## 5. References

Exit-interview token analysis + Fable adjudication (2026-07-12 session) · `anchor_triggers.tsv` /
`anchor_query.sh` (doctrine delivery HD reuses) · clearance sticky workflow (delivery vehicle) ·
`track_closeout_protocol.md` (lease/reaper HD reuses) · CC-HANDOFF-SPINE (#1182, compression precedent).
