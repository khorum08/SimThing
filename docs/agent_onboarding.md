# Agent Onboarding — the human operator's guide to bringing up an agent cold

**Audience: you, the operator.** This is the human-facing counterpart to the machine orientation
(`orient.sh` / `orchestrator_orientation.md`). It answers one question: *I am starting a fresh agent of
type X — what do I do?* `AGENTS.md` points here.

There are three agent tiers. Each has one cold-start ritual and one authority boundary. Do not mix them.

---

## The one-line rule for every tier
**A fresh agent session orients once, then carries the receipt.** Orientation is generated from live harness
state; a receipt proves the session oriented against *this* state, not a stale memory. Subsequent handoffs in
that same session carry the existing receipt unless governance moved.

---

## Coding agent (Grok / Cursor — in-repo, has a shell)
Builds one rung from a handoff. Cannot see the whole governance picture and does not need to.

- **Cold start:** when the user / Owner / DA opens a fresh coding-agent session, the agent runs
  `bash scripts/ci/orient.sh --role=coding` once and carries the emitted `ORIENT-RECEIPT`.
- **You hand it:** the rung's production handoff (from the orchestrator or DA) plus the instruction to carry
  its existing session receipt. Do not make each handoff re-run full orientation.
- **Mid-session governance movement:** if the receipt is stale or missing, stop and report that to the
  operator / DA instead of reprinting full orientation by habit.
- **It returns:** `PROBATION / proof-present / DA-review-pending` — never a self-graduation.
- **Authority:** it **does not merge.** Gate-wiring, exceptions, and anything reserved go back up. It builds,
  proves, and hands back.

## Orchestration agent (Codex / webchat — GitHub connector, no shell)
Decomposes DA handoffs into rungs, verifies coding-agent work against the tree, routes clearance, runs triage.

- **Cold start:** read `docs/orchestrator_orientation.md` at head (generated, freshness-gated — it cannot be
  stale) and carry its embedded receipt. GHA-side, it can also `/orient role=orchestrator` on any open PR.
- **Track selection:** local operators use `bash scripts/ci/gen_orientation.sh --open <track.md>` to open/create
  or realign the active orchestration track; `/orient` and `orient.sh` emit orientation only.
- **You hand it:** the DA's authorization/handoff for a track or rung.
- **It does:** verify the tree (never the relayed report); route each rung via the clearance ladder —
  **merge-clear conforming precedented-class rungs itself**, **escalate the DA reserve** (gate-wiring,
  exceptions, widening, Tier-2, new proof classes, closeout, binding conditions); land a `/triage` row for
  every INSPECT delta.
- **Closeout-substrate PRs:** before merge/DA handback, require a disposable end-to-end
  `track_closeout.sh` rehearsal (build manifest -> resolve -> check-eval -> apply) using a tiny fixture
  with previously expunged rows plus source/auto/explicit doc artifacts, and report the sample verdict.
- **Authority:** merges *conforming* precedented-class work; **routes gate-wiring/reserve to the DA.** An
  unauthorized gate-wiring merge is a process incident (see `docs/tests/incident_oh_docs_sunset_unauthorized_merge.md`).

## DA agent (Opus / Fable — the frontier reviewer)
The executive design authority. Reviews escalations, graduates or remands, authors doctrine and handoffs.

- **Cold start:** `bash scripts/ci/orient.sh --role=da`, and read the anchors on demand — the DA reads full
  doctrine sections (core design, constitution, invariants, key ADRs) when a rung's domain triggers them
  (`/anchor <domain>` serves them verbatim), never a summary.
- **You hand it:** the escalation relay (or a strategic question).
- **It does:** verify the tree; **graduate-merge or produce a remedial handoff**; author doctrine as
  data/verdict, never sprawling prose; spend owner attention only on residue.
- **Authority:** merges gate-wiring and DA-reserve work after review. Above the DA sits the **Owner**, whose
  authorization is required for gate-wiring closeouts and whose supremacy is visible and recorded.

---

## Quick reference

| Tier | Cold-start command | Returns | Merges |
|---|---|---|---|
| Coding (Grok/Cursor) | `bash scripts/ci/orient.sh --role=coding` once per fresh session | PROBATION / proof-present | nothing |
| Orchestration (Codex/webchat) | read `docs/orchestrator_orientation.md` (+ `/orient`) | routed rungs + triage rows | conforming precedented-class only |
| DA (Opus/Fable) | `bash scripts/ci/orient.sh --role=da` | graduation or remedial handoff | gate-wiring / DA-reserve, after review |

**When to update this file:** only when a *tier's session-admission ritual* changes (a new entrypoint command
or a new agent tier). Per-rung governance is not here — it lives in the generated orientation digest, which
cannot drift. This file is a stable operator manual, not a changelog.
