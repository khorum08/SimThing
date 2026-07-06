# 0.0.8.4.8 — First Post-Rustification Corpus Clearance

> **Status: OPEN / DA-OPENED (2026-07-06).** The first corpus-wide Necessity-Test sweep of the Rustification
> era. The 0.0.8.4.x tracks *built the gates*; this track *applies them to the legacy corpus* that predates
> them. Driven entirely by existing harness tooling — no new mechanism.

## 1. Why now
The Rustification initiative migrated invariants to types/admission/scans and built the test-lifecycle
harness (birth-track tripwire, `test_inventory.tsv`, lifecycle expiry, dsu tiers, drift gate, clearance
router). The harness now exists to enforce the **Necessity Test** — but the bulk of the corpus was born
*before* it and has never been swept. This track is that sweep.

## 2. Baseline (measured 2026-07-06, master `e9bde33091`)
`test_inventory.tsv`: **916 rows.** By class: seal-proof 264, oracle-parity 250, **unknown 137**,
golden-byte 134, stead-required 121, behavior-regression 8, dependency-floor 2. By birth_track:
**pre-lifecycle 644** (never Necessity-Tested), 0.0.8.4.7 150, 0.0.8.5 85, 0.0.8.4.6 37.

**Sweep candidates = the 781 rows the harness has never judged:** the **644 `pre-lifecycle`** rows and the
**137 `unknown`-class** rows. The 264 seal-proof / 250 oracle-parity / 134 golden-byte / 121 stead-required
rows carry durable classes and are presumed-retained (spot-audited, not swept wholesale).

## 3. The clearance criterion (Necessity Test — existing doctrine, not new)
A test survives **only** if it catches a regression that neither (1) the compiler / a type boundary, (2) a
production admission hard-error on a live path, nor (3) an existing integration path already catches. If
deleting it cannot break production and it is not a downstream dependency or required for canonical function,
**delete it.** Per-boundary floor is **zero**, not one. Run **necessity-deletion waves, not
representative-curation waves** — this track removes redundant witnesses; it does not add or "balance" tests.

## 4. Rungs
| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| S | `CC-HANDOFF-SPINE-0` (parallel; run early) | Compress the `handoff_template.md` context spine: for each spine line, if it is now enforced **mechanically** (a scan/verdict) or by the **admission substrate** (type boundary / hard-error), replace the verbatim restatement with a one-line pointer to the enforcing surface; **keep** only lines no mechanism enforces (e.g. the gate-wiring merge-authority norm). Add a compact **Canonical Entrypoints** block naming the CI utilities every handoff exercises — `orient.sh --role=<role>` (cold-start + receipt), `cargo check -p <crate>` + `doctrine_scan.sh` (inner loop), `clearance_check.sh` / `relay_lint.sh` (routing/relay), `gen_orientation.sh --check` / `gen_digest.sh --check` (freshness when docs touched), `test_inventory_drift_check.sh` (when tests change), and the GHA comment commands (`/orient /clearance /relay-lint /triage /anchor /seal-proof`) — so handoffs *reference* them instead of re-deriving them. **Also fix the orientation-per-handoff flaw (§4A):** orientation is a **session-start ritual invoked once by the user/owner/DA**, not a per-handoff command. Rewrite template §10b so a handoff **requires the agent to carry its session `ORIENT-RECEIPT`** (relay-lint validates freshness) and, only if stale, refresh via `orient.sh --since=<receipt>` — remove any "cold start / first command: run `orient.sh`" framing that makes each handoff re-invoke a full orientation. Mirror the session-start rule in `docs/agent_onboarding.md`. **Also necessity-scope the proof battery (§4B):** a handoff must not mandate a gate's selftest on a PR that cannot have affected that gate. In particular, `doctrine_selftest.sh` (14 scan invocations + git-sandbox churn) runs **only when the PR touches the scanner surface** (`scans.tsv` / `doctrine_scan.sh` / `doctrine_selftest.sh` / `fixtures/known_bad/` / `allow/`); otherwise the handoff cites *"scanner unchanged — selftest not required."* Encode the rule generally in the Canonical Entrypoints block (each selftest is conditional on its own surface being in the diff), with `doctrine_selftest.sh` as the named exemplar. | `handoff_template.md` spine **line count decreases**; every removed line cites its enforcing mechanism; retained lines are only the unmechanized ones; Canonical Entrypoints block present **with proof commands necessity-scoped to their surface** (`doctrine_selftest.sh` marked scanner-surface-only); **§10b requires a carried session receipt, not a per-handoff `orient.sh` invocation** (`--since` is the only sanctioned mid-session refresh); a template-shaped handoff still passes `relay_lint.sh`; net effect is **less restated doctrine and fewer redundant gate runs per handoff** → reduced input-token churn |
| G | `CC-RELAY-CLEARANCE-GATE-0` (gate-wiring; DA-reviewed; high priority — run before further DA escalations) | Close the critical routing-bypass hole (§4C): make it **mechanically impossible to post a DA-review relay without a fresh clearance-router verdict.** `relay_lint.sh` must **FAIL** any relay carrying a Graduation-routing / DA-review block that lacks a `CLEARANCE-VERDICT` (from `clearance_check.sh` or a visible `/clearance` comment) **bound to the current PR head**; a stale/absent/unbound verdict → `FAIL(missing-clearance-verdict)`. An `ORCHESTRATOR-CLEARABLE` verdict means the orchestrator merges — no DA relay is warranted at all. Also close the label gap: add the handoff-authoring/onboarding surfaces (`docs/handoff_template.md`, `docs/agent_onboarding.md`) to `clearance_check.sh` `GATE_WIRING_PATHS` so they route `DA-RESERVE(gate-wiring)`, not `DA-RESERVE(novelty)`. Selftest fixtures: a DA relay without a clearance verdict → FAIL; with a fresh head-bound reserve verdict → PASS; a handoff-template edit → `gate-wiring`. | `relay_lint --selftest` proves a manually-written DA escalation lacking `CLEARANCE-VERDICT` **fails**; a relay with a fresh head-bound verdict passes; `clearance_check` routes a `handoff_template.md` edit as `gate-wiring`; scanner-surface untouched, so selftest necessity-scoped per §4B; doctrine_scan green |
| 0 | `CC-BASELINE-0` | Freeze the §2 baseline as an artifact; resolve every **`unknown`-class** row to a durable class or mark it a deletion candidate; no deletions yet | 0 rows remain `unknown`-class; each reclassification cites its retention basis; drift gate PASS |
| 1..N | `CC-SWEEP-<crate>` | Per-crate necessity-deletion waves over `pre-lifecycle` rows: for each, name the higher-rung owner (type/admission/scan/integration path) that makes the test redundant, delete it + its inventory row, prove production intact | crate compiles; remaining gates green; deleted rows leave no drift; **inventory row count decreases**; each deletion cites the superseding boundary |
| C | `CC-CLOSEOUT-0` (closing) | Corpus-reduction report; every survivor carries a durable class or a justified downstream-utility lease; zero `unknown`; zero un-owned `pre-lifecycle` | Net `test_inventory.tsv` row count **decreased** vs baseline; reduction quantified; lifecycle expiry + drift + doctrine scan green; DA sign-off |

Waves are orchestrator-buildable (they delete tests + rows and prove, a precedented shape); `CC-BASELINE-0`'s
reclassification, `CC-HANDOFF-SPINE-0` (it edits the binding handoff template), and `CC-CLOSEOUT-0` are
DA-reviewed (they set retention/authoring doctrine). `CC-HANDOFF-SPINE-0` has no dependency on the sweep and
should land first so every subsequent handoff carries the leaner spine.

### 4A. Orientation is session-scoped, not per-handoff (binding)
Orientation is performed **once, at the start of a new agent session, invoked by the user/owner/DA** (per
`docs/agent_onboarding.md`). The agent reports its `ORIENT-RECEIPT` once; that receipt is its proof of
currency for the whole session. A **handoff must not command a fresh `orient.sh` run** — doing so reprints
the full orientation page as input tokens on every rung (the exact churn this track removes) and merely
re-emits the same receipt. Instead, a handoff **requires the agent to carry its session receipt**; relay-lint
validates it against the live digest. The **only** sanctioned mid-session action is `orient.sh --since=<receipt>`
— a cheap governance *delta* — run when (and only when) the receipt has gone stale because governance moved.
Latent seam being closed: template §10b's "emit via `orient.sh`" is ambiguous and was hardened by an early
handoff into a per-handoff "first command." `CC-HANDOFF-SPINE-0` removes that ambiguity.

### 4B. The proof battery is necessity-scoped (binding)
A handoff must not mandate running a gate's selftest on a PR that **cannot** have affected that gate — a
green selftest proving an unchanged surface is a redundant witness (the Necessity Test, applied to
gate-execution rather than the test corpus). The named exemplar is **`doctrine_selftest.sh`**: it fires
~14 `doctrine_scan.sh` invocations plus git-sandbox churn (slow, worse on Windows git-bash), yet a
docs-only / scenario / inventory PR cannot rot the scanner. So it runs **only when the PR diff touches the
scanner surface** — `scans.tsv`, `doctrine_scan.sh`, `doctrine_selftest.sh`, `fixtures/known_bad/`, or
`allow/`; otherwise the handoff records *"scanner unchanged — selftest not required."* The Canonical
Entrypoints block states the general rule (each selftest is conditional on its own surface appearing in the
diff). This is handoff-guidance only; the CI-side unconditional selftest run is a separate mechanical
optimization, not opened by this rung.

### 4C. No DA relay without a clearance verdict (binding — critical)
The clearance router (`clearance_check.sh`, M1 of 0.0.8.4.7) mechanizes the routing decision, but its verdict
was never **required** at the relay gate — so an orchestrator can hand-write a DA-review relay that never ran
clearance, and `relay_lint` passes it. That is a routing-bypass hole: it lets the orchestrator mint
DA/Owner reviews at will and burn the scarcest resource, defeating the clearance ladder. **A DA-review relay
must carry a fresh clearance-router `CLEARANCE-VERDICT` bound to the current PR head, or it does not lint.**
An `ORCHESTRATOR-CLEARABLE` verdict means the orchestrator merges and *no* DA relay is warranted; a
`DA-RESERVE(...)` verdict is what a legitimate DA relay quotes as its justification. This converts "route via
clearance" from prose into a `relay_lint` verdict (`FAIL(missing-clearance-verdict)`) — the same
prose→verdict law as the rest of the initiative. `CC-RELAY-CLEARANCE-GATE-0` (rung G) lands it and is high
priority: until it merges, orchestrators can and will over-escalate. (PR #1182 was correctly reserved to DA,
but by *manual* routing, not the router — the exact bypass this rung forecloses.)

**Two enforcement boundaries — the harness cannot gate chat (binding).** `CC-RELAY-CLEARANCE-GATE-0`
mechanizes the clearance requirement for **repo artifacts** (PR bodies / results docs that pass through
`relay_lint`). It **cannot** gate a DA-review handoff an orchestrator simply *types into chat* — that prose
never touches the harness. The chat path therefore has two human/agent-discipline backstops, and they are
binding norms, not conveniences:
- **Orchestrator:** before producing *any* DA-review handoff, run `/clearance` (or `clearance_check.sh
  --pr <n>` / `--range`) and **observe the emitted verdict**. If no verdict exists, do not produce the
  handoff — trigger clearance first. An `ORCHESTRATOR-CLEARABLE` result means *merge*, not escalate.
- **DA:** do **not** engage a review until you have **independently run/observed the clearance verdict**
  yourself. A `CLEARANCE-VERDICT` string quoted in a relay is a *claim*, not proof — verify the tree
  (founding principle). And **do not SHA-match your way into or out of a review**: `tested_code_sha`
  identifies the proof point, `coverage_basis` explains an evidence-only tail, and a docs-tail commit is not
  a hold reason (Immutability Law / #1169). The routing authority is the clearance tool, run first-hand —
  not a field-comparison ritual, which is the kabuki that recurs whenever the real mechanism is skipped.

## 5. Harness-driven, no new mechanism
Every gate this track needs already exists: `test_inventory_drift_check.sh` (deletions must leave no drift),
`test_lifecycle_expiry_check.sh` (survivor classes/leases), `test_lifecycle_dsu_tiers.tsv` (rising-cost lease
on kept-but-unjustified), `doctrine_scan.sh` (the whole battery). This track **consumes** them; it adds no
script, no TSV schema, no crate. Its only artifacts are deletions, reclassifications, and the reduction report.

## 6. Fences
No engine-behavior changes (deleting a test must not change production — if it does, the test was load-bearing
and stays). No new test authoring. No representative-curation. No new mechanism. A wave that cannot name the
higher-rung owner making a test redundant **does not delete it** — it escalates it as a genuine survivor.

## 7. Success measure
The single number: `test_inventory.tsv` row count, baseline 916 → closeout `< 916`, with every surviving row
justified. A corpus that did not shrink means the sweep found nothing redundant — which, given 644 never-judged
rows, would itself be the surprising finding worth escalating.
