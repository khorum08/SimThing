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
| HD-1 | `HD-TRUTH-GUARDS-0` | **Mechanical truth guards** (the #1316 class dies): (a) master-ancestry gate — no graduation/exit-stamp claim lints unless the named merge commit `--is-ancestor` of master; (b) clearance sticky gains `body_sha: fresh/evidence-tail/STALE` line (router already computes it — surface it); (c) stamp-in-diff relay-lint rule — a rung PR must carry its own ladder-row stamp in the same diff. Selftests: mislanded-merge fixture FAILs; evidence-tail passes; stampless rung PR FAILs. | **DA-GRADUATED / merged [#1328](https://github.com/khorum08/SimThing/pull/1328)** — DA deep pass 2026-07-12: on-branch relay-lint 34 + clearance 99 selftests PASS; ancestry/stampless/mislanded fixtures bite; live sticky renders `body_sha: evidence-tail`; evidence-tail widening adjudicated necessary for guard composition; evidence `docs/tests/hd_truth_guards_0_results.md`. **Next:** `HD-DISPATCH-SUBSTRATE-0` | Std |
| HD-2 | `HD-DISPATCH-SUBSTRATE-0` | **The object + projections + receipt (§1.1–§1.2, §1.4–§1.5):** `handoffs/` dir, `handoff_dispatch.sh` (lint · render · receipt · board-state JSON), sticky + board-issue sync job, `owner_approved` dispatch gate, lease-on-graduation, `owner_directives.tsv` (directive·scope·status·set_by) rendered into every projection — the standing-dicta litany is retyped never again. Fixtures: draft blocks dispatch; receipt drift FAILs relay-lint; projections regenerate byte-stable. | **DA-GRADUATED / merged [#1331](https://github.com/khorum08/SimThing/pull/1331)** — DA deep pass 2026-07-12: dispatch + relay-lint (36) + clearance (99) selftests PASS on-branch; render byte-stable, `HD-RECEIPT: 990cb20dee0b` reproduced; draft-gate/receipt-drift/body-cap/owner-notes fixtures bite; Amendments 1–2 honored (board digest renders all rung exit-proofs, issue #1332 ≤60 lines); graduated .hd leased per §1.5; evidence `docs/tests/hd_dispatch_substrate_0_results.md`. + REMEDIAL (DA-inline 2026-07-12): resolve-handoff crashed the sync job on any PR without a rung claim (first dispatch PR #1333 went red and left the board stale) — no-claim now resolves to quiet no-ingress, claim-mismatch FAILs unchanged, 2 fixtures added. **Next:** HD-3 ∥ HD-4 | Std |
| HD-3 | `HD-OWNER-INTERFACE-0` | **Prompt-native Owner/collaborator door (§1.3):** `/handoff approve/amend/hold/status` comment commands (reuse doctrine-exec command machinery); scribe protocol documented for orchestrators (prose → .hd mutation, echo the diff back for confirmation); "Current handoff approved, implement" ingress protocol for all roles; `owner_notes` guaranteed-render proof. Fixtures: /handoff amend from a non-owner routes to owner-review; approve flips the gate; hold freezes dispatch. | **DA-GRADUATED / merged [#1336](https://github.com/khorum08/SimThing/pull/1336)** — DA deep pass 2026-07-13: dispatch + relay-lint(36) + clearance(99) selftests on-branch; OWNER-gated (author_association) fork-blocked mutation job; amend JSON-encoded into frontmatter (injection-neutral), parse-validated pre+post write, owner_notes render-checked after every mutation; approve/hold/amend/non-owner-review + resolver-fallback fixtures bite; ingress at cap. Post-merge action: live `/handoff status` verification on the board issue (bootstrap-blocked pre-default-branch). Evidence `docs/tests/hd_owner_interface_0_results.md`. | Std |
| HD-4 | `HD-LIBRARIAN-0` | **Stewardship verbs (Owner: "check library staleness", "cull dead tsv rows"):** `librarian.sh` with `--staleness` (anchor resync/orphans + dead-listener domains no glob emits + reach-log prune + lease/artifact aging + doc-budget headroom, one ≤60-line report), `--cull` (compose existing `track_closeout --discover/--decommission` + reach-log prune + orphan-anchor retirement; dry-run default, `--confirm` to act; src/authority paths always route to DA — reaper safety rules inherited), and `--catalog [--role coding/orchestrator/da]` (which anchors + payload sections + always-on spine each role can reach; the agent-confusion antidote). `/librarian` comment command + prompt-verb mapping. No new gating — observability + reaping only. | **DA-GRADUATED / merged [#1337](https://github.com/khorum08/SimThing/pull/1337)** — DA deep pass 2026-07-13: librarian + dispatch + relay-lint(36) + clearance(99) + anchor-check(7) + anchor-query(12) + closeout-prove selftests on-branch; live cull DRY with hash-proven zero-write tree; two-phase confirm (report admission before mutation); confirmed cull OWNER-gated, non-owner routed; catalog read-only 6 lines/role; caps 45/10/6 vs 60; .hd receipt 561129af1c70 unmutated. Owner verbs live: staleness / cull / catalog. Post-merge: live /librarian verification. Evidence `docs/tests/hd_librarian_0_results.md`. | Std |
| HD-5 | `HD-DOCS-CASCADE-0` | **Loud onboarding visibility + the payoff compression.** Entry-point docs updated so HD is unmissable at onboarding: `agent_onboarding.md` (every tier's first section: "handoffs arrive as HD projections — render yours; 'approved, implement' protocol"), `docs/agents.md`, `ci_screening_surface.md` (HD rows in the screening map, within cap), `handoff_template.md` **compressed to schema + authoring rules and fenced against re-fattening** (required-reading blocks deleted — anchors carry doctrine): the compressed template gets (a) a hard per-file DOC-BUDGET cap recorded in the budget TSV at this rung's graduation, and (b) a self-describing anti-reaccretion header — "schema only; new doctrine goes to anchors; growth here is the regression this track closed" — the old template is where hygiene-kabuki historically accreted, so the thin survivor is the likeliest cruft re-attractor and must carry its own tripwire. Relay short-form keyed to HD-RECEIPT. Generated docs (orchestrator_orientation) get it via generator data, not hand edits. **Exit criteria: net corpus prose DECREASES** (HD-CLOSEOUT-0 binding) **and the template cap is live**. | **DA-GRADUATED / merged [#1340](https://github.com/khorum08/SimThing/pull/1340)** — first fully browser-automated DA-orchestrator-coding rung (Codex orchestrated, Cursor cloud coded; 3 remand loops incl. a DA dispatch-object compaction #1341, receipt eaf1e09dc42e superseding). DA deep pass 2026-07-14: net −198 (136/334) independently recomputed; template 360→112 + anti-reaccretion header + hard cap 112; tier sections lead with HD ingress, operator section folded; results doc 42 lines; full battery + clearance(99) on-branch; .hd leased per §1.5. Evidence `docs/tests/hd_docs_cascade_0_results.md`. **Next:** `HD-CLOSEOUT-0` (Dispatch history: DA 2026-07-13 override-dispatch; DA remedial #1341 receipt supersede; implementer PROBATION at `2bcc5ffb`.) | Std |
| HD-6 | `HD-POINTER-LIFECYCLE-GATE-0` | **Pointer lifecycle gate + board freshness.** `gen_orientation.sh --open` REFUSES the flip while the outgoing track doc status is not CLOSED/PARKED; `--force-owner "<directive>"` escape records an `owner_directives.tsv` row. Board sync also fires on push to master (kills the post-merge stale window); workflow-level fixture for board render with `current_handoff: none`. Fixtures: flip-from-OPEN FAILs; flip-from-PARKED/CLOSED pass; force path records the directive. | **PROBATION** (implementer 2026-07-14, branch `cursor/hd-pointer-lifecycle-gate-0`, `ORIENT-RECEIPT: ada87881548c`, `HD-RECEIPT: caab38a976d2`): `gen_orientation.sh --open` refuses the pointer flip unless the OUTGOING track status header declares CLOSED/PARKED (state token only, not trailing prose); `--open` also enforces one coherent mutation root (`FAIL(incoherent-root)`) so the `ORIENTATION_*` seams cannot mix a fake PARKED root with a victim pointer (remand 1); `--force-owner "<directive>"` is two-phase — admit with zero writes, then record the `owner_directives.tsv` row only on a successful transition (remand 2), and is rejected without `--open`; the clearance workflow board-sync also fires on push to master (PR-ingress skipped). 6 gen_orientation gate fixtures bite (OPEN refuse; PARKED/CLOSED allow; force records; cross-root refuse; forced-invalid-target zero-write; force-requires-open). Battery green: gen_orientation --selftest(19)/--check, agent_scan, doc_budget, relay_lint --selftest(36). DA authors graduation at merge (ruling 6). Dispatched `handoffs/HD-POINTER-LIFECYCLE-GATE-0.hd.md` (2026-07-14T17:40Z; roles orch=Codex coder=Cursor-cloud da=Fable). Evidence `docs/tests/hd_pointer_lifecycle_gate_0_results.md`. | Std |
| HD-7 | `HD-CATALOG-LIBRARY-0` | **Per-role library catalog (the original vision).** `librarian.sh --catalog` v2 enumerates the LIBRARY per role — anchors x trigger domains x role reach, always-on spine, payload sections — from `doctrine_anchors.tsv` + `anchor_triggers.tsv` + orientation, not current-handoff-bound; <=60 lines/role complete-or-fail. Staleness report gains a harness-fixture-count gauge line. | NOT STARTED | Std |
| HD-8 | `HD-OWNER-AUTHORING-GUIDE-0` | **Owner authoring guide + role-slot sweep (cap-budgeted).** Human-facing intro: authoring a new workplan (header, ladder cells lead with stamps, no escaped pipes), amendments (ruling 6), open/park/close lifecycle incl. the HD-6 gate, regenerating library + TSVs, browser-stack protocol (forward-by-pointer, orchestrator-authored coder blocks, verify-send) and the three coder surfaces (Cursor cloud; Grok CLI pinned `-m grok-4.5` + `--worktree`; Claude sonnet-class). Role-first heading sweep across onboarding + template (ruling 7). Net-prose accounting in results; any cap raise paired with compression. | NOT STARTED | Std |
| HD-C | `HD-CLOSEOUT-0` | Measured close: median metered-agent ingress (prose lines per handoff) before/after; prose-delta table proving net decrease; ancestry-gate + receipt-mismatch fixtures green; **graduated ladder cells compressed to stamp+evidence (history folded out)**; **lease sweep verified via librarian cull**; discharge bindings; close via `track_closeout.sh`. | NOT STARTED | DA |

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
