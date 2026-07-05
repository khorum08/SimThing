# 0.0.8.4.7 — Orchestration Harness & Guidance Compression

> **Status: OPEN / DA-OPENED (2026-07-05).** Owner-commissioned (Grok/Gemini root-cause studies absorbed,
> DA-scrutinized, two proposals overruled — see §2). Sits inside the Rustification initiative
> (`ci_screening_surface.md §0`): migrate orchestration guidance from **prose → data → verdict**, so
> compliance is admission behavior and DA/Owner attention is spent only on residue.

## 1. Root cause this track closes

Every drift permanently eliminated in this repo died the same way: **a judgment was converted into a
verdict** (kind-drift → `SPEC-LOWERER-KIND-READ`; test propagation → birth-track tripwire; inventory
drift → drift gate). Orchestrator triage/routing drift persists because the **clearance decision itself is
still prose-guided judgment** held in a low-context tier: "is this precedented? does it need DA? did I log
that INSPECT?" Secondary causes (confirmed by commissioned studies + this session's governance audit):
context dilution across relays, incentive gradient toward "quiet scanner," prose fidelity loss, and the
§5A reading-list orientation model — the dilution machine itself.

## 2. DA rulings on the commissioned proposals

- **ACCEPTED:** standards as visible/auditable/per-track-extensible TSV data; compliance as admission
  behavior with FAIL-as-teacher remedies; auto-logging to cut token spend; prose→data→type promotion.
- **OVERRULED — no new Rust crate now.** `simthing-orchestrator-harness` is recorded as the **promotion
  target**, buildable only when a named consumer proves the script layer's ceiling (same law as the
  widening valve: substrate on demand, never speculatively). Trigger examples: validation logic exceeding
  what the script+selftest pattern can prove, or a second repo adopting the harness.
- **OVERRULED — no `scripts/ci/standards/` directory of parallel per-domain TSVs.** That is sprawl in
  uniform. This track **extends the proven spine** — one thin engine + one rule surface + self-test +
  freshness-gated digest (`doctrine_scan.sh`/`scans.tsv`/`doctrine_selftest.sh`/`gen_digest.sh`) — and
  **consolidates** the existing ~20-script/10-TSV surface rather than adding a framework beside it.

## 3. The four mechanisms

**M1 — Clearance Router (the spine).** `scripts/ci/clearance_check.sh <PR|range>` emits
`CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE | DA-RESERVE(reason) | FAIL(remedy)`. Mechanizes the §5A
precedented-class checks 1–8 exactly as the DA ran them by hand (diff-scope globs, tested-code-SHA
coverage basis, lifecycle/drift verdicts, CI status, citable GPU-proof presence, triage-row presence,
binding-conditions lookup). Data: `precedented_classes.tsv` (class_id | scope_globs | envelope |
requirements | status) and `binding_conditions.tsv` (rung | condition | set_by | status) — DA conditions
become rows, not DA memory. Routing drift becomes impossible, not discouraged. Novelty always has an
exit: DA-RESERVE and the breakthrough valve are verdicts, never blocks.

**M2 — Orientation Digest (kill the reading list).** `gen_orientation.sh` generates
`docs/orchestrator_orientation.md` from the live TSVs (clearance ladder state, precedented classes, DA
reserve, open envelopes, binding conditions, tested-code-SHA proof rule, active track pointers) —
freshness-gated in CI exactly like `sanctioned_surface.md`. A derived digest cannot drift from its
sources. New-session orientation drops from ~2,000 prose lines to one guaranteed-current page; §5A's
reading list survives only for DA/deep-work onboarding.

**M3 — Relay/handoff lint (codified handoff shaping).** `relay_lint.sh` validates the de-facto relay
schema that orchestration already converged on: identity block, `tested_code_sha` + `coverage_basis`,
Homing Boundary classification table, lifecycle posture, load-bearing proofs, graduation routing.
Malformed relay → rejected with the missing block named (FAIL-as-teacher). Advisory first, blocking after
one clean cycle. The anti-kabuki template becomes a gate, not a memory.

**M4 — Rule & prose expiry (the apparatus shrinks by one law).** Extend the lifecycle-expiry pattern from
tests to the apparatus itself: (a) `rule_expiry_check.sh` — cadence sweep listing scan/standard rows whose
`promotion_blocker` is satisfied → retire in the same PR that satisfies it; (b) **DOC-BUDGET tripwire** —
`ci_screening_surface.md` (and this doc's operator sections) may not grow: new guidance lands as data or
displaces prose, enforced as a scan; (c) every prose paragraph superseded by a mechanism in this track is
replaced by a one-line pointer to the enforcing surface. Tests expire, rules expire, prose expires.

## 4. Rungs

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 0 | `OH-CLEARANCE-ROUTER-0` | `clearance_check.sh` + `precedented_classes.tsv` + `binding_conditions.tsv` (backfill: PALMA→6.2 conditions as the first rows) + selftest fixtures (clearable / reserve / fail cases) | Router reproduces the four hand-run DA verdicts of 2026-07-05 (#1150–#1152 CLEARABLE-shaped, #1154 DA-RESERVE:binding-conditions); selftest PASS; doctrine_scan PASS |
| 1 | `OH-RELAY-LINT-0` | `relay_lint.sh` + schema doc block; advisory mode wired to the comment surface | Lints the #1154 relay PASS; three mutated relays (missing coverage_basis / classification / routing) FAIL with named block; selftest PASS |
| 2 | `OH-ORIENTATION-DIGEST-0` | `gen_orientation.sh` + generated `docs/orchestrator_orientation.md` + CI freshness gate | Digest regenerates byte-identical from TSVs; stale digest hard-FAILs like `sanctioned_surface.md`; orientation content covers the §5A operational contract |
| 3 | `OH-TRIAGE-INDUCTION-0` | Router requires landed `/triage` rows for INSPECT deltas (check 7 live); `doctrine_exec_triage.sh` strictness (justification mandatory); backfill TP-COMBAT-ARENA-0 GameSession rows | Un-triaged INSPECT delta → DA-RESERVE(triage-missing); malformed `/triage` rejected with format printed; backfill rows landed |
| 4 | `OH-DOCS-SUNSET-0` (closing rung) | Prose compression: every §5A/§1A/§12 paragraph now enforced by M1–M3 replaced with a pointer line; DOC-BUDGET scan row; `rule_expiry_check.sh`; sunset ledger in this doc listing each retired paragraph → enforcing surface | `ci_screening_surface.md` net line count **decreases**; DOC-BUDGET green; rule-expiry sweep runs clean; zero orphaned pointers |
| 5 | `OH-HARNESS-CRATE-0` (**DEFERRED**) | The Rust harness crate — only on a named trigger (§2) | Trigger recorded + DA/Owner authorization; not before |

Rungs 0–3 are orchestrator-buildable under the standing handoff regime; rung 4 is DA-reviewed (it edits
doctrine text). All new tests: `birth_track = 0.0.8.4.7-orchestration-harness` (register the track at
first test birth). The track's own tools are subject to its own law: every new TSV row carries a
`promotion_blocker`; the router/lint/digest each ship with selftest fixtures or they do not merge.

## 5. What this track does NOT do

No engine-crate edits. No new workflows beyond wiring existing ones to new scripts. No scheduled
workflows without an explicit cadence rung. No orchestrator autonomy expansion — M1 *narrows* discretion
on routing while leaving escalation always open. No speculative crate (§2). No new prose: this document
is the track's entire prose budget.

## 6. Sunset ledger (maintained by OH-DOCS-SUNSET-0)

| Retired prose | Superseded by | Date |
|---|---|---|
| _(populated at rung 4)_ | | |
