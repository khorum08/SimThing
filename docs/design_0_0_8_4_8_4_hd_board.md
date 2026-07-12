# 0.0.8.4.8.4 — HD Board: Handoff Dispatch, Owner Interface & Library Stewardship

> **Status: OPEN / Owner-directed (2026-07-12).** Owner parked 0.0.8.6 and directed regeneration;
> `active_track.txt` points here. Entry rung: HD-1 `HD-TRUTH-GUARDS-0`. 0.0.8.6 remains parked
> (its closeout is still gated by `STUDIO-OWNER-CLOSURE-0`) — this track does not touch Studio UI.
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
| HD-1 | `HD-TRUTH-GUARDS-0` | **Mechanical truth guards** (the #1316 class dies): (a) master-ancestry gate — no graduation/exit-stamp claim lints unless the named merge commit `--is-ancestor` of master; (b) clearance sticky gains `body_sha: fresh\|evidence-tail\|STALE` line (router already computes it — surface it); (c) stamp-in-diff relay-lint rule — a rung PR must carry its own ladder-row stamp in the same diff. Selftests: mislanded-merge fixture FAILs; evidence-tail passes; stampless rung PR FAILs. | PROBATION / PR #1328 / DA review pending; code head `7c3548753f9399b4767c3d10a15f17436809692b`; relay-lint 34 fixtures PASS; clearance 99 fixtures PASS; results: `docs/tests/hd_truth_guards_0_results.md` | Std |
| HD-2 | `HD-DISPATCH-SUBSTRATE-0` | **The object + projections + receipt (§1.1–§1.2, §1.4–§1.5):** `handoffs/` dir, `handoff_dispatch.sh` (lint · render · receipt · board-state JSON), sticky + board-issue sync job, `owner_approved` dispatch gate, lease-on-graduation, `owner_directives.tsv` (directive·scope·status·set_by) rendered into every projection — the standing-dicta litany is retyped never again. Fixtures: draft blocks dispatch; receipt drift FAILs relay-lint; projections regenerate byte-stable. | NOT STARTED | Std |
| HD-3 | `HD-OWNER-INTERFACE-0` | **Prompt-native Owner/collaborator door (§1.3):** `/handoff approve\|amend\|hold\|status` comment commands (reuse doctrine-exec command machinery); scribe protocol documented for orchestrators (prose → .hd mutation, echo the diff back for confirmation); "Current handoff approved, implement" ingress protocol for all roles; `owner_notes` guaranteed-render proof. Fixtures: /handoff amend from a non-owner routes to owner-review; approve flips the gate; hold freezes dispatch. | NOT STARTED | Std |
| HD-4 | `HD-LIBRARIAN-0` | **Stewardship verbs (Owner: "check library staleness", "cull dead tsv rows"):** `librarian.sh` with `--staleness` (anchor resync/orphans + dead-listener domains no glob emits + reach-log prune + lease/artifact aging + doc-budget headroom, one ≤60-line report), `--cull` (compose existing `track_closeout --discover/--decommission` + reach-log prune + orphan-anchor retirement; dry-run default, `--confirm` to act; src/authority paths always route to DA — reaper safety rules inherited), and `--catalog [--role coding\|orchestrator\|da]` (which anchors + payload sections + always-on spine each role can reach; the agent-confusion antidote). `/librarian` comment command + prompt-verb mapping. No new gating — observability + reaping only. | NOT STARTED | Std |
| HD-5 | `HD-DOCS-CASCADE-0` | **Loud onboarding visibility + the payoff compression.** Entry-point docs updated so HD is unmissable at onboarding: `agent_onboarding.md` (every tier's first section: "handoffs arrive as HD projections — render yours; 'approved, implement' protocol"), `docs/agents.md`, `ci_screening_surface.md` (HD rows in the screening map, within cap), `handoff_template.md` **compressed to schema + authoring rules and fenced against re-fattening** (required-reading blocks deleted — anchors carry doctrine): the compressed template gets (a) a hard per-file DOC-BUDGET cap recorded in the budget TSV at this rung's graduation, and (b) a self-describing anti-reaccretion header — "schema only; new doctrine goes to anchors; growth here is the regression this track closed" — the old template is where hygiene-kabuki historically accreted, so the thin survivor is the likeliest cruft re-attractor and must carry its own tripwire. Relay short-form keyed to HD-RECEIPT. Generated docs (orchestrator_orientation) get it via generator data, not hand edits. **Exit criteria: net corpus prose DECREASES** (HD-CLOSEOUT-0 binding) **and the template cap is live**. | NOT STARTED | Std |
| HD-C | `HD-CLOSEOUT-0` | Measured close: median metered-agent ingress (prose lines per handoff) before/after; prose-delta table proving net decrease; ancestry-gate + receipt-mismatch fixtures green; discharge bindings; close via `track_closeout.sh`. | NOT STARTED | DA |

**Sequencing:** HD-1 → HD-2 → (HD-3 ∥ HD-4) → HD-5 → HD-C. HD-1 is independently valuable and lands
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

## 4. Non-goals

No pointer change at authoring time · no new crate · no clearance-lexicon change · no replacement of
relay-lint/clearance (HD layers on them) · no dashboard beyond the ≤60-line board digest · no
weakening of DA-reserve routing.

## 5. References

Exit-interview token analysis + Fable adjudication (2026-07-12 session) · `anchor_triggers.tsv` /
`anchor_query.sh` (doctrine delivery HD reuses) · clearance sticky workflow (delivery vehicle) ·
`track_closeout_protocol.md` (lease/reaper HD reuses) · CC-HANDOFF-SPINE (#1182, compression precedent).
