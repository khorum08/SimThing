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
- **STANDING RULING — no SHA-equality routing (owner-commissioned, DA-ruled 2026-07-05).** Superseded —
  enforced by `LIVE-POINTER` in `relay_lint.sh` plus tested-code-SHA binding in `clearance_check.sh`; see §6
  sunset ledger.
- **THE IMMUTABILITY LAW (DA first-principles pass, 2026-07-05).** Committed evidence and selftest fixtures
  record immutable facts only (PR number, `tested_code_sha` + `coverage_basis`, merge SHA at graduation,
  content hashes, state transitions). Live pointers are forbidden and enforced as
  `RELAY-LINT-VERDICT: FAIL(live-pointer: <field>)`; receipt selftests use fixture-local orientation
  snapshots. See rung 2cR and §6 sunset ledger.

## 3. The mechanisms

**M1 — Clearance Router (the spine).** `scripts/ci/clearance_check.sh <PR|range>` emits
`CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE | DA-RESERVE(reason) | FAIL(remedy)`. Mechanizes the §5A
precedented-class checks 1–8 exactly as the DA ran them by hand (diff-scope globs, tested-code-SHA
coverage basis, lifecycle/drift verdicts, CI status, citable GPU-proof presence, triage-row presence,
binding-conditions lookup). Data: `precedented_classes.tsv` (class_id | scope_globs | envelope |
requirements | status) and `binding_conditions.tsv` (rung | condition | set_by | status) — DA conditions
become rows, not DA memory. Routing drift becomes impossible, not discouraged. Novelty always has an
exit: DA-RESERVE and the breakthrough valve are verdicts, never blocks.

*M1 harness laws (final-pass, 2026-07-05):* (a) **Fail-closed** — malformed TSV, script error, or
ambiguous class resolution → `DA-RESERVE(harness-error)`, never a silent CLEARABLE; selftest fixtures
must prove it. (b) **Self-application** — the harness's own TSVs and scripts are **gate-wiring**: edits
to them are never orchestrator-clearable (§0.9.7 exclusion made explicit), and the router hard-refuses to
clear a PR that touches its own surface. (c) **Kill-switch as data** — flipping a class row to
`status=suspended` makes the router return `DA-RESERVE(class-suspended)`; revoking delegated authority is
a one-row DA edit, not a code change. (d) **Condition-discharge symmetry** — the PR that discharges a
binding condition flips its row `status` in that same PR (same law as rule expiry; the register never
accretes stale rows). (e) **Verdict ledger** — every GHA-side verdict appends one machine-parseable row
(`verdict | class | pr | sha | date | sketch?`) to `scripts/ci/clearance_ledger.tsv`: the DA spot-audit
sampling surface, and the closeout telemetry that turns this track's claimed savings into measured ones.

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

*LED instrumentation (final-pass — the parked pass@1 study becomes measurable for free):* the lint
**recognizes** the optional, DA/orchestrator-authored `§5.1 design-space sketch` block
(`handoff_template.md §5.1`, probationary since the LED study, arXiv:2602.01698), and the M1 verdict
ledger **tags** rungs that carried one (`sketch?` column). At closeout, 0R-rates with vs. without the
sketch are a queryable comparison — the probationary practice is promoted or retired on **evidence**, not
kept as unfalsifiable prose. Never mandated; instrumented.

**M4 — Rule & prose expiry (the apparatus shrinks by one law).** Extend the lifecycle-expiry pattern from
tests to the apparatus itself: (a) `rule_expiry_check.sh` — cadence sweep listing scan/standard rows whose
`promotion_blocker` is satisfied → retire in the same PR that satisfies it; (b) **DOC-BUDGET tripwire** —
`ci_screening_surface.md` (and this doc's operator sections) may not grow: new guidance lands as data or
displaces prose, enforced as a scan; (c) every prose paragraph superseded by a mechanism in this track is
replaced by a one-line pointer to the enforcing surface. Tests expire, rules expire, prose expires.

**M5 — Cold-start protocol: receipted orientation (owner-commissioned, 2026-07-05).** Kills the
hand-regenerated orientation brief. Two landings, one forcing function:

- **Coding agents (in-repo, have a shell):** `scripts/ci/orient.sh --role=coding|orchestrator|da` prints a
  compact, role-keyed landing page **generated from the same TSVs as M2** (that tier's clearance contract,
  active track/rung, tested-code-SHA proof rule, inner-loop commands) and ends with
  `ORIENT-RECEIPT: <12-char content hash>` of the current orientation state.
- **Webchat orchestrators (GitHub read-only):** the M2 `orchestrator_orientation.md` embeds the same
  generated receipt line.
- **Forcing function:** relay lint (M3) and the clearance router (M1) **require a valid receipt** in the
  relay/PR body. Missing → lint FAIL; stale (governance moved since) → `RE-ORIENT` verdict naming the
  delta. `orient.sh --since=<old-receipt>` prints only what changed, so long-running chats re-orient at
  delta cost.
- **Entry stubs:** auto-read agent files (`agents.md` / `CLAUDE.md`-class) are reduced to the one-line
  pointer to `orient.sh` — restated guidance there is a recorded drift source and is deleted, not
  maintained. A stub scan keeps them ≤5 lines.
- **Handoff-template amendment (downstream compression):** once receipts are enforced, the verbatim
  context-spine restatement in every handoff is replaced by the receipt requirement + a pointer — the
  spine lives in one generated page instead of every handoff (~40–60 lines saved per handoff, forever).
- **Honest limit (recorded):** a receipt proves the agent had the *current* contract available — currency,
  not comprehension. Comprehension failures remain caught downstream by router/lint/scan gates; the
  receipt closes the actual recurring vector, orientation-against-outdated-governance.

**M6 — Doctrine-anchor integrity: anchors are routed, never compressed (owner caveat 1, binding).** The
compression laws of this track (M2/M4/M5) apply to **operational guidance only**. The doctrine anchors —
`simthing_core_design.md`, the constitution (`design_0_0_8_3.md`), `invariants.md`, and the key ADRs
(`mapping_sparse_regioncell.md`, `resource_flow_substrate.md`, …) — are a different class: the paradigm,
not the procedure. A generated digest must never become the de-facto constitution (summary-fidelity loss
at paradigm altitude is a recorded, repeated failure mode — e.g. core §7 map-reasoning drift).

- **Anchor register:** `scripts/ci/doctrine_anchors.tsv` — `anchor_id | doc | section | trigger_domains |
  content_hash`. The anchors named above are the seed rows; tracks may add rows, never remove them without
  owner sign-off.
- **Quote-verbatim law (scannable):** the orientation digest and any generated surface may **quote** anchor
  sentences byte-verbatim with citation, or **point** to sections — never paraphrase doctrine. Enforced
  mechanically: every quoted anchor span in a generated doc must grep exactly in its source, or the
  freshness gate FAILs.
- **Domain-triggered ANCHOR-ACK:** when a rung's diff or declared domain hits an anchor's
  `trigger_domains` (map/movement → core §7; engine-crate edits → §1.2 + seal law; RF work → the RF ADR),
  the relay must carry `ANCHOR-ACK: <anchor_id>@<content_hash>` — an attestation that the **full section**
  was evaluated this rung. Missing ack on a triggered domain → relay lint FAIL. Narrow quotes by default;
  full-section evaluation forced exactly when drift risk is live — never blanket context flooding.
- **Anchors version the receipts:** `ORIENT-RECEIPT` hashes **include** the anchor content hashes. An
  anchor edit stales every receipt fleet-wide → `RE-ORIENT` names which anchor moved. Doctrine can never
  silently version out from under a working agent.

**M7 — GHA-side delivery law: every surface ships webchat-executable (owner caveat 2, binding).** Codex
and webchat orchestrators execute GHA-hosted scripts via the proven comment-command surface
(`doctrine_exec_commands.yml`: collaborator-only, no fork code under a write token). Therefore **every
M1–M6 mechanism ships dual-mode — local script + GHA comment command — in its birth rung**, never
local-first-wire-later:

- `/clearance` — runs the M1 router against the PR GHA-side; posts `CLEARANCE-VERDICT` as the sticky
  comment. The webchat orchestrator self-serves its routing verdict.
- `/relay-lint` — validates the PR body / results doc against the M3 schema (incl. receipt + ANCHOR-ACK).
- `/orient [role] [--since <receipt>]` — posts the role-keyed digest + current receipt (or the governance
  delta) into the thread.
- `/anchor <domain|anchor_id>` — posts the **verbatim** anchor section(s) into the PR thread: exact
  constitution/core-design text, self-served into webchat context, zero courier cost, zero paraphrase
  (M6's quote-verbatim law applies to the command output).

**M7 execution boundary (owner ruling, 2026-07-05 — "fullest exploitation" does NOT mean everything runs
GHA-side).** GHA runners are headless Linux: **no GPU adapter, no X11/ALSA/winit desktop.** Therefore:

- **GHA-executable:** scans, lints, router verdicts, digests, receipts, anchor serving, CPU-only checks —
  and **validation of recorded proofs** (presence, `tested_code_sha` binding, coverage-basis freshness).
- **Owner-local only, never GHA:** bevy-, GPU-resident-, and desktop-facing test execution. These route
  through the existing Track B owner-local harness (`doctrine_tests.sh` owner-local profiles, which
  already refuse under `GITHUB_ACTIONS` — INSPECT, never silent PASS), produce a citable
  `DOCTRINE-TESTS-VERDICT: PASS` bound by `tested_code_sha + coverage_basis`, recorded **in-repo** (the
  committed proof-doc pattern, e.g. `tp_combat_arena_0_gpu_proof.md`), and the GHA-side surfaces
  **consume and verify the recorded verdict — never attempt execution.** The M1 router's GPU-proof check
  is exactly this consumption: it verifies binding mechanically; it does not run the test.
- **Forbidden proof, carried forward (§10 / Track B rulings):** adding a GPU/bevy/desktop execution leg to
  any workflow — including apt-get/x11/wayland/ALSA bootstrap on a runner — is rejected at review, not
  attempted. The flow is always: local GPU/desktop run → committed citable proof → GHA validates the
  binding → merge.

## 4. Rungs

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 0 | `OH-CLEARANCE-ROUTER-0` | `clearance_check.sh` + `precedented_classes.tsv` + `binding_conditions.tsv` (backfill: PALMA→6.2 conditions as the first rows) + `clearance_ledger.tsv` emission + selftest fixtures (clearable / reserve / fail-closed / self-application-refusal / suspended-class cases) | **DA-GRADUATED / merged [#1162](https://github.com/khorum08/SimThing/pull/1162) @ `39802af5`** — M1 router live; `/clearance` on doctrine-exec-commands; evidence [`oh_clearance_router_0_results.md`](tests/oh_clearance_router_0_results.md) |
| 1 | `OH-RELAY-LINT-0` | `relay_lint.sh` + schema doc block; advisory mode wired to the comment surface | **DA-GRADUATED / merged [#1163](https://github.com/khorum08/SimThing/pull/1163) @ `d4969f1c8`** — M3 relay lint + `/relay-lint` live on master; evidence [`oh_relay_lint_0_results.md`](tests/oh_relay_lint_0_results.md) |
| 1R | `OH-CLEARANCE-ROUTER-0R` | empty-diff precision + local PR-number resolution in `clearance_check.sh` | **DA-GRADUATED / merged [#1164](https://github.com/khorum08/SimThing/pull/1164) @ `ad46a0be8`** — empty/unresolved targets route `DA-RESERVE(harness-error)`; novelty reserved for resolved non-empty unmatched diffs; bare PR-number local path resolves or hard-errors with `--range` remedy; evidence [`oh_clearance_router_0r_results.md`](tests/oh_clearance_router_0r_results.md) |
| 2 | `OH-ORIENTATION-DIGEST-0` | `gen_orientation.sh` + generated `docs/orchestrator_orientation.md` + CI freshness gate + `/orient` | **DA-GRADUATED / merged [#1165](https://github.com/khorum08/SimThing/pull/1165) @ `eee9d4714`** — generated orientation digest + freshness gate + `/orient` live on master; freshness gate verified derived; evidence [`oh_orientation_digest_0_results.md`](tests/oh_orientation_digest_0_results.md) |
| 2b | `OH-COLD-START-0` (after 2) | `orient.sh` + `ORIENT-RECEIPT` emission; receipt validation in `relay_lint.sh` | **DA-GRADUATED / merged [#1166](https://github.com/khorum08/SimThing/pull/1166) @ `d5c76215e`** — orientation receipts live; relay-lint validates missing/stale/wrong-role receipts; router hook deferred as named future hook; evidence [`oh_cold_start_0_results.md`](tests/oh_cold_start_0_results.md) |
| 2c | `OH-ANCHOR-INTEGRITY-0` (after 2b) | `doctrine_anchors.tsv` (seed rows: core design, constitution, invariants, key ADRs, incl. core §7 with map/movement trigger domain); quote-verbatim scan on generated docs; `ANCHOR-ACK` requirement in relay lint keyed to trigger domains; anchor hashes folded into `ORIENT-RECEIPT`; `/anchor` comment command | **DA-GRADUATED / merged [#1167](https://github.com/khorum08/SimThing/pull/1167) @ `131cf858a3`** — doctrine anchors live; anchor hash drift, missing/stale/unknown ANCHOR-ACK validation, anchor-bound receipts, and `/anchor` serving active; `anchor_check.sh --resolve` gives anchor-id exact-match priority over trigger-domain collision; DA-cleared under the no-SHA-equality routing ruling (§2) — proof-to-tested-code binding + green CI on the merged tree, docs-row SHA drift is not a gate; evidence [`oh_anchor_integrity_0_results.md`](tests/oh_anchor_integrity_0_results.md). Post-merge `/anchor` GHA smoke reposted per rung caveat |
| 2cR | `OH-IMMUTABLE-EVIDENCE-0` (remedial; absorbs `OH-SELFTEST-DECOUPLE-0`; implements the §2 Immutability Law) | **(a)** Decouple `relay_lint.sh`/`orient.sh` receipt selftests from the **live** orientation digest — fixture-local orientation snapshot per fixture (self-contained), then CI-gate both selftests in `doctrine-scan.yml`. **(b)** relay-lint rule `LIVE-POINTER` — FAIL (named field) on live-pointer fields in relay bodies and `docs/tests/**` results docs (`current_pr_head`, live/docs-refresh head, latest-run ids, self-referential as-of SHAs); one-time cleanup sweep strips the existing live-pointer fields from the ~5 TP/OH results docs (immutable rows stay). **(c)** Dead-pointer process fix: doctrine-exec command checkout uses a resolved `checkout_ref`; open PRs use `head_ref`, merged PRs use `merge_commit_sha` when the head ref is deleted, so commands work on the merged PR itself. **(d)** Hash-stability hardening: receipt/anchor content hashing normalizes line endings and strips BOM before hashing, so Windows-written bytes cannot fake drift | **DA-GRADUATED / merged [#1171](https://github.com/khorum08/SimThing/pull/1171) @ `af31f0caf9c841f4d1f26febf83c730627e8916d`** — live-pointer SHA churn mechanically impossible; SHA-hygiene prose retired to §6; fixture-local receipt selftests, LIVE-POINTER lint, merge-commit checkout fallback, and BOM/CRLF hash normalization live; evidence [`oh_immutable_evidence_0_results.md`](tests/oh_immutable_evidence_0_results.md) |
| 3 | `OH-TRIAGE-INDUCTION-0` | Router requires landed `/triage` rows for INSPECT deltas (check 7 live); `doctrine_exec_triage.sh` strictness (justification mandatory); backfill TP-COMBAT-ARENA-0 GameSession rows | **PROBATION / proof-present / DA-review-pending** — clearance router reserves INSPECT deltas without landed triage rows; `/triage` rejects malformed or empty-reason rows; TP-COMBAT-ARENA-0 GameSession residue backfilled; DA clearance required (gate-wiring); evidence [`oh_triage_induction_0_results.md`](tests/oh_triage_induction_0_results.md) |
| 4 | `OH-DOCS-SUNSET-0` (closing rung) | Prose compression: every §5A/§1A/§12 paragraph now enforced by M1–M3 replaced with a pointer line; DOC-BUDGET scan row; `rule_expiry_check.sh`; sunset ledger in this doc listing each retired paragraph → enforcing surface | `ci_screening_surface.md` net line count **decreases**; DOC-BUDGET green; rule-expiry sweep runs clean; zero orphaned pointers; closeout telemetry readout from `clearance_ledger.tsv` (clears vs relays vs RE-ORIENTs; §5.1-sketch 0R comparison for the LED promotion/retirement call) |
| 5 | `OH-HARNESS-CRATE-0` (**DEFERRED**) | The Rust harness crate — only on a named trigger (§2) | Trigger recorded + DA/Owner authorization; not before |

**Delivery law (M7, applies to every rung):** each rung's surface ships dual-mode — local script **and**
GHA comment command — in the same PR (`/clearance` with rung 0, `/relay-lint` with rung 1, `/orient` with
rung 2b, `/anchor` with rung 2c), on the existing `doctrine_exec_commands.yml` carrier with its trust
constraints intact. A local-only surface does not exit its rung.

Rungs 0–3 (incl. 2b/2c) are orchestrator-buildable under the standing handoff regime; rung 4 is
DA-reviewed (it edits doctrine text), 2b's handoff-template amendment and 2c's anchor seed rows are
DA-reviewed within their rungs. All new tests: `birth_track = 0.0.8.4.7-orchestration-harness` (register the track at
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
| §2 no-SHA-equality routing prose | LIVE-POINTER relay_lint + tested-code-SHA binding in clearance_check | 2026-07-05 |
| §2 Immutability Law mechanization argument | LIVE-POINTER relay_lint + immutable evidence fixtures | 2026-07-05 |
| ci_screening_surface.md §5A SHA-hygiene paragraph | LIVE-POINTER relay_lint + tested-code-SHA binding in clearance_check | 2026-07-05 |
| post-merge command-smoke-on-next-open-PR rule | doctrine-exec checkout_ref merge-commit fallback | 2026-07-05 |
| _(populated at rung 4)_ | | |
