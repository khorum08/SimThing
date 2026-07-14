# SimThing Handoff — schema + authoring rules

> **Anti-reaccretion header (HD-DOCS-CASCADE-0).** *Schema only; new doctrine goes to anchors
> (`anchor_query.sh`), never restated here; growth in this file is the regression this track closed.*
> A handoff is the repo object `handoffs/<RUNG-ID>.hd.md`, never chat paste. Role views are
> **generated** (`bash scripts/ci/handoff_dispatch.sh --render <coding|orchestrator|da> <file>`) and
> carry an `HD-RECEIPT`; hand-edited projections FAIL lint. Operator protocol, board, and lifecycle
> live in one home: [`agent_onboarding.md`](agent_onboarding.md) (HD Board). This file is only the
> fill-in contract: the frontmatter schema, the delta-only body, and the relay format.

---

## 1. Frontmatter schema (validated by `handoff_dispatch.sh`; a missing/unknown key or bad value FAILs lint)

- `rung` — `<RUNG-ID>`; the file MUST be named `<rung>.hd.md`.
- `kind` — `rung | transport | remedial | stamp`.
- `track` — active track id. · `base_sha` — commit the delta is authored against.
- `audience` — `coding | orchestrator | da`. · `model_tier` — e.g. `std`.
- `expected_route` — e.g. `ORCHESTRATOR-CLEARABLE` or `DA-RESERVE(<class>)`.
- `owner_approved` — `true | false`; **dispatch is blocked while `false`**.
- `owner_notes` — free-text intervention channel; renders **verbatim** in every projection.
- `surfaces` — list of touched paths; drives `REQUIRED-ANCHORS` via `anchor_triggers.tsv`.
- `forbidden` — list of out-of-scope path globs. · `required_checks` — batteries the rung lands green.
- `stop_conditions` — hard-stop signals; hitting one means stop and report, not work around it.

Projections additionally emit `HD-RECEIPT` (content hash), `REQUIRED-ANCHORS` (resolved from
`surfaces`), and any `owner_directives` — all quoted in the PR body/relay; `relay_lint.sh` FAILs a
receipt mismatch. Same receipt ⇒ provably the same handoff.

## 2. Body — delta-only (≤80 lines), three sections only

- **BUILD** — what to change, as a contract (capability + where it attaches), not inline implementation.
- **FENCES** — the invariants and limits this rung must hold (cite the enforcing surface / anchor id).
- **EXIT-PROOF** — the observable green state + the stamps that must land in-diff.

No reading lists, no restated doctrine, no hand-authored implementation. Doctrine travels as anchor
IDs; resolve on demand with `anchor_query.sh`, and after editing an anchored doc run
`anchor_check.sh --resync`.

## 3. Authoring rules

- **Doctrine as pointer, not prose.** Cite the enforcing surface or anchor id; never restate a
  mechanized rule. Type boundary > admission hard-error > guard scan > prose.
- **Recipient by Type.** Mechanical impl / refactor / test → coding (Cursor/Grok); bounded writing →
  docs tier; acceptance / sign-off / ontology conformance → DA (Opus/Owner). Mis-route ⇒ fix the Type
  or split the rung — do not burn a DA turn on mechanical edits or route judgment to a coding agent.
- **Proof minimal + load-bearing.** Each test names the regression it catches; a condition guaranteed
  by a type or hard-errored at admission gets **zero** tests (Necessity Test). Reuse existing
  oracles/guards; never re-derive one the repo already runs.
- **Evidence: one doc, one line, one row.** `docs/tests/<rung>_results.md` (signal-only sections) +
  one `current_evidence_index.md` line + one status-row edit. No ceremony triple-update.
- **Escalate, don't sidecar.** A seal that genuinely blocks the rung escalates to the DA (or requests
  the owner-gated amendment valve only when the handoff allows it); never self-grant or build around it.

## 10b. Orientation receipt (relay-lint enforced when carried)

A session orients **once** at cold start and carries — never re-runs — its `ORIENT-RECEIPT`. Stale or
missing ⇒ stop and report to the operator/DA. Required fields when carried:

```
ORIENT-RECEIPT:
role:
orientation_digest_sha:
```

## 10c. Anchor acknowledgement (trigger-domain rungs — relay-lint enforced)

```
ANCHOR-ACK: <anchor_id>@<12-char hash>
```

> One line per required anchor (from `anchor_check.sh --resolve` or `anchor_query.sh`); match the
> sticky `REQUIRED-ANCHORS`.

## 11. Response format (the relay — keyed to HD-RECEIPT + the Graduation-routing block)

```
HD-RECEIPT:        <12-hex, copied from the projection>
Status:
PR / Merge:
What changed:
Load-bearing proofs (+ what each catches):
Scope Ledger:
Conformance (spine / anchors held):
Known gaps / next:
Graduation routing (for DA — why this is PROBATION, not self-marked COMPLETE):
  CI verdict:          <PASS-RELIABLE | INSPECT(n) | FAIL>
  Triage entries:      <none | scan-id:outcome …>
  Risk class:          <none | semantic | data-deliverable | gate-wiring | seal-residue | allowlist-edit>
  Falsification check: <the exact check(s) that confirm/deny "done">
  Recommended posture: <light | deep> — <one line why>
```

> The implementer/orchestrator relays **PROBATION** with the Graduation-routing block and stamps the
> ladder row PROBATION **in the same diff**; the **DA** authors the graduation stamp at merge (HD
> ruling 6). The block says what the scanner cannot see (the structural risk class + the exact
> falsification check); the DA routes review depth from it (`ci_screening_surface.md` §5).

## §H. Anti-kabuki floor (binding; the authority — do not dilute or cite to skip proof)

Litmus for every line: **does it prove or build the feature, or produce a governance artifact about
it?** If the latter, cut it. Rejected at review, both directions:

- **Over-production (kabuki):** batteries for type/admission-guaranteed conditions; bespoke greps that
  restate the scope diff (one forbidden-path `git diff` + the existing guard is the ceiling); triple
  doc ceremony; inline hand-authored implementation; reading lists longer than the rung touches;
  a docs/closeout rung in front of a buildable feature; a noun (registry/table/preflight) standing in
  for an admission hard-error; inert scaffolding that enforces nothing.
- **Under-proof (worse):** run the check and paste its **real** output — never assert, summarize, or
  fabricate; verify the tree, not the relayed report; no merge before DA clearance on any
  authority/gate/PROBATION rung. Citing an anti-kabuki rule to skip proof is itself the non-conformant
  route this floor exists to catch.
