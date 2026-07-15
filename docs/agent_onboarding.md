# Agent Onboarding — the human operator's guide to bringing up an agent cold

**Audience: you, the operator.** This is the human-facing counterpart to the machine orientation
(`orient.sh` / `orchestrator_orientation.md`). It answers one question: *I am starting a fresh agent of
type X — what do I do?* `AGENTS.md` points here.

There are three role slots. Each has one cold-start ritual and one authority boundary. Do not mix them.
Example surfaces: coding = Grok CLI / Cursor cloud; orchestration = Codex/webchat; DA = Fable / Claude / Codex.

---

## The one-line rule for every tier
**A fresh agent session orients once, then carries the receipt.** Orientation is generated from live harness
state; a receipt proves the session oriented against *this* state, not a stale memory. Subsequent handoffs in
that same session carry the existing receipt unless governance moved.

---

## Coding role — in-repo shell
**Handoffs arrive as HD projections — render yours:** `handoff_dispatch.sh --render coding handoffs/<RUNG>.hd.md` on "Implement handoff `<RUNG>`"; obey its BUILD/FENCES/EXIT-PROOF + `owner_notes`, quote its `HD-RECEIPT` (the "approved, implement" protocol).

- **Cold start:** when the user / Owner / DA opens a fresh coding-agent session, the agent runs
  `bash scripts/ci/orient.sh --role=coding` once and carries the emitted `ORIENT-RECEIPT`.
- **Inner loop (unconditional, ≤4 steps — HU-DELTA-SCAN-0):** after orient-once,
  `cargo check -p <touched-crate>` → `bash scripts/ci/agent_scan.sh` → focused `cargo test` when required.
  Doctrine: `anchor_query.sh` (not raw greps); after anchored-doc edits, `anchor_check.sh --resync`.
  Whole-tree `doctrine_scan.sh` is CI/maintainer, not the coding default.
- **You hand it:** the rung's production handoff (from the orchestrator or DA) plus the instruction to carry
  its existing session receipt. Do not make each handoff re-run full orientation.
- **Mid-session governance movement:** if the receipt is stale or missing, stop and report that to the
  operator / DA instead of reprinting full orientation by habit.
- **It returns:** `PROBATION / proof-present / DA-review-pending` — never a self-graduation.
- **Authority:** it **does not merge.** Gate-wiring, exceptions, and anything reserved go back up. It builds,
  proves, and hands back.

## Orchestration role — GitHub connector, no shell
**Handoffs arrive as HD projections — render yours** (`--render orchestrator`) on "Current handoff approved, implement"; route coding and quote the `HD-RECEIPT`.

Decomposes DA handoffs into rungs, verifies coding-agent work against the tree, routes clearance, runs triage.

- **Cold start:** read `docs/orchestrator_orientation.md` at head (generated, freshness-gated — it cannot be
  stale) and carry its embedded receipt. GHA-side, it can also `/orient role=orchestrator` on any open PR.
- **Track selection:** local operators use `bash scripts/ci/gen_orientation.sh --open <track.md>` to open/create
  or realign the active orchestration track; `/orient` and `orient.sh` emit orientation only.
- **You hand it:** the DA's authorization/handoff for a track or rung.
- **It does:** verify the tree when merge-clearing (prefer branch confirmation over the relayed report);
  route each rung via the clearance ladder — **merge-clear conforming precedented-class rungs itself**,
  **escalate true DA residue** (gate-wiring, novelty, seal, binding, genuine unclassified — not router debt
  alone); on `DA-RESERVE`, sticky emits `REQUIRED-ANCHORS:` — handoffs ACK with `ANCHOR-ACK`. Land a
  `/triage` row for every INSPECT delta. Empty-class machine split (`CLEARANCE-ADMITTED-SCOPE-GAP-0`):
  novelty claim → `DA-RESERVE(novelty)`; valid `admitted_envelope` + proofs →
  `DA-RESERVE(admitted-scope-router-gap)` (router debt — class-harden, not fresh DA design); else →
  `DA-RESERVE(unclassified-scope)`. Missing admitted-scope fields →
  `FAIL(missing-admitted-scope-router-gap-fields...)`. See
  `docs/tests/clearance_admitted_scope_gap_0_results.md`. **Exit-proof residual only:** for
  **ORCHESTRATOR-GRADUATED** self-clears where the design row cannot hold the final merge SHA pre-merge,
  land the docs-only status-stamp before the next rung. **DA-passed** rungs are stamped and stamp-merged
  by the DA (see DA section) — do not reassign.
- **Closeout-substrate PRs:** before merge/DA handback, require a disposable end-to-end
  `track_closeout.sh` rehearsal (build manifest -> resolve -> check-eval -> apply) using a tiny fixture
  with previously expunged rows plus source/auto/explicit doc artifacts, and report the sample verdict.
- **Authority:** merges *conforming* precedented-class work; **routes gate-wiring/reserve to the DA.** An
  unauthorized gate-wiring merge is a process incident (see `docs/tests/incident_oh_docs_sunset_unauthorized_merge.md`).

## DA role — frontier reviewer
**Handoffs arrive as HD projections — render yours** (`--render da`) on "Relay posted on PR #n"; rule and graduate-merge or remand, carrying the `HD-RECEIPT`.

The executive design authority. Reviews escalations, graduates or remands, authors doctrine and handoffs.

- **Cold start:** `bash scripts/ci/orient.sh --role=da`, and read the anchors on demand — the DA reads full
  doctrine sections (core design, constitution, invariants, key ADRs) when a rung's domain triggers them
  (`/anchor <domain>` serves them verbatim), never a summary.
- **You hand it:** the escalation relay (or a strategic question).
- **It does:** graduate-merge or produce a remedial handoff; author doctrine as data/verdict, never
  sprawling prose; spend owner attention only on residue. Weight **verify-the-tree** by load-bearing impact
  (below) — not as a fixed tax on every stage.
- **Verify the tree (weighted — load-bearing first):** a relayed claim is still a claim, not proof. Prefer
  branch confirmation; **require** it when the escalation is **code-facing**, **long-lifecycle**, or
  **horizontally impactful** — e.g. production/elevatable crate surfaces, gate-wiring/harness, clearance
  classes, kernel/admission, or risk classes `data-deliverable` / `gate-wiring` / `seal-residue` /
  `allowlist-edit` / elevation-shaped semantic change. Then: read the escalated paths, run the named
  **Falsification check**(s) (targeted — not full-crate batteries), and only then pass or remand.
  **Relaxed / optional** for pure policy admissions, exit-proof stamps, and light residual where CI already
  vouches and no long-lifecycle or horizontal surface moves — light posture may confirm deliverables from
  the relay + CI without a full tree dig. Light vs deep still sets *depth* when tree review is engaged.
- **DA treeverify advisor (token routing):** before a load-bearing graduate/admit, run
  `bash scripts/ci/da_treeverify.sh --pr <n>` (or `--range`). It emits advisory
  `DA-TREEVERIFY-PROFILE: RELAX|LIGHT-TREE|DEEP-TREE` + focus paths from `scripts/ci/da_review_profile.tsv`.
  **Not** a clearance verdict and **not** a graduation stamp. Core TSV rows are permanent; non-core rows
  require `expires_on` and **must be deleted/retired after expiry** (lifecycle gate in doctrine-scan).
  Expeditionary escape needs `expeditionary: YES` + charter + `expeditionary_until` and cannot RELAX
  production/engine/long-lifecycle surfaces.
- **Exit-proof stamp (binding after a passed verdict):** a DA pass is incomplete until the DA updates the
  active workplan/design-ladder **Exit proof** cell (`DA-GRADUATED / merged #<PR> @ <merge-sha>`, or
  equivalent DONE wording for formal admission/denial), marks the results doc COMPLETE where applicable,
  regenerates orientation (`bash scripts/ci/gen_orientation.sh`), and **lands and merges** that docs-only
  stamp PR. Do not leave the stamp as orchestrator residual after a DA pass — the stamp is part of the
  graduation conclusion, not a follow-up chore.
- **Authority:** merges gate-wiring and DA-reserve work after review; also merges the post-verdict
  exit-proof stamp PR. Above the DA sits the **Owner**, whose authorization is required for gate-wiring
  closeouts and whose supremacy is visible and recorded.

---

## Quick reference

| Tier | Cold-start / default loop | Returns | Merges |
|---|---|---|---|
| Coding role | `orient.sh --role=coding` → check → `agent_scan` → focused test | PROBATION / proof-present | nothing |
| Orchestration role | read `docs/orchestrator_orientation.md` (+ `/orient`) | routed rungs + triage rows | conforming precedented-class only |
| DA role | `orient.sh --role=da` (+ weighted treeverify) | graduation + exit-proof stamp merge | gate-wiring / DA-reserve / stamps |

## HD Board — dispatch prompting & handoff lifecycle (operator protocol)

Owner workplan authoring/revision, the open/park/close lifecycle (`--park`/`--unpark`, HD-6 pointer gate), and the manual-vs-automated progression modes are in [`owner_authoring_guide.md`](owner_authoring_guide.md). Handoffs are repo objects (`handoffs/<RUNG-ID>.hd.md`), never chat paste. The live view is the
**SimThing Board issue** (auto-synced every clearance run: pointer, current handoff + receipt, open
PRs with routes, every rung's exit-proof state). Check the board, not a local file.

- **Prompt protocol (per handoff) — pointers, not payloads:** each tier's ingress line lives in its
  section above; the orchestrator (scribe) authors/merges the `.hd` with `owner_approved` on your word, then each agent renders its own projection.
- **Orientation sequence:** orientation is per-**session** (once; receipt carried); dispatch is
  per-**handoff**. Never re-orient for a new handoff — a stale receipt FAILs mechanically
  (`body_sha:` sticky line + relay-lint) and the agent must stop and report, not re-print orientation.
- **Lifecycle:** author → dispatch (`owner_approved` + exit-proof cell stamped by DA with `HD-RECEIPT`)
  → implement → PROBATION relay (implementer stamps PROBATION in-diff; ruling 6) → DA deep pass →
  DA graduation stamp at merge → graduated `.hd` leased into `closeout_artifacts.tsv` → wall-clock reap.
- **Owner verbs, in prose to any agent or as `/handoff …` GitHub comments:** `approve` · `amend: <text>`
  · `hold` · `status` — the scribe mutates the `.hd` and echoes the diff back before pushing. Stewardship
  verbs: *"check library staleness"* → `librarian.sh --staleness`; *"cull dead tsv rows"* → `--cull`
  (dry-run default, `--confirm` to act); *"what can role X reach"* → `--catalog --role <r>`.
- **Trust anchor:** every projection, relay, and PR quotes `HD-RECEIPT: <12-hex>`; a mismatch is a
  relay-lint FAIL. Same receipt ⇒ provably the same handoff — transcription drift is dead.

## Harness maintenance & sprawl observation (operator / DA)

Pointer-only — mechanics in `docs/track_closeout_protocol.md` and the named scripts:

| Instrument | One-line role |
|---|---|
| `track_closeout.sh --discover` | list end-of-lifecycle assets for a track before building a manifest |
| `track_closeout.sh --artifact-expiry` | wall-clock check on leased closeout artifacts + parking pen |
| `track_closeout.sh --decommission --dry-run` | preview reaping expired parked rows / leased docs |
| `doc_budget_check.sh --check` / `da_treeverify.sh --check-lifecycle` | DOC-BUDGET caps; non-core profile rows must not be past `expires_on` |
| `anchor_reach_log.tsv` / `anchor_check.sh` | query observability (`--prune 30` at closeout); `--resync` after anchored-doc edits |
| `docs/tests/hu_throughput_snapshot.tsv` | harness meta-gauges (scan tax, checklist steps, table counts) |

**When to update this file:** when a tier's session-admission ritual or coding default loop changes.
Per-rung governance lives in the generated orientation digest — this file is a stable operator manual.
