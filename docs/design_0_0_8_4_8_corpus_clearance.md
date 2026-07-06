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
| S | `CC-HANDOFF-SPINE-0` (parallel; run early) | Compress the `handoff_template.md` context spine: for each spine line, if it is now enforced **mechanically** (a scan/verdict) or by the **admission substrate** (type boundary / hard-error), replace the verbatim restatement with a one-line pointer to the enforcing surface; **keep** only lines no mechanism enforces (e.g. the gate-wiring merge-authority norm). Add a compact **Canonical Entrypoints** block naming the CI utilities every handoff exercises — `orient.sh --role=<role>` (cold-start + receipt), `cargo check -p <crate>` + `doctrine_scan.sh` (inner loop), `clearance_check.sh` / `relay_lint.sh` (routing/relay), `gen_orientation.sh --check` / `gen_digest.sh --check` (freshness when docs touched), `test_inventory_drift_check.sh` (when tests change), and the GHA comment commands (`/orient /clearance /relay-lint /triage /anchor /seal-proof`) — so handoffs *reference* them instead of re-deriving them | `handoff_template.md` spine **line count decreases**; every removed line cites its enforcing mechanism; retained lines are only the unmechanized ones; Canonical Entrypoints block present; a template-shaped handoff still passes `relay_lint.sh`; net effect is **less restated doctrine per handoff** → reduced input-token churn |
| 0 | `CC-BASELINE-0` | Freeze the §2 baseline as an artifact; resolve every **`unknown`-class** row to a durable class or mark it a deletion candidate; no deletions yet | 0 rows remain `unknown`-class; each reclassification cites its retention basis; drift gate PASS |
| 1..N | `CC-SWEEP-<crate>` | Per-crate necessity-deletion waves over `pre-lifecycle` rows: for each, name the higher-rung owner (type/admission/scan/integration path) that makes the test redundant, delete it + its inventory row, prove production intact | crate compiles; remaining gates green; deleted rows leave no drift; **inventory row count decreases**; each deletion cites the superseding boundary |
| C | `CC-CLOSEOUT-0` (closing) | Corpus-reduction report; every survivor carries a durable class or a justified downstream-utility lease; zero `unknown`; zero un-owned `pre-lifecycle` | Net `test_inventory.tsv` row count **decreased** vs baseline; reduction quantified; lifecycle expiry + drift + doctrine scan green; DA sign-off |

Waves are orchestrator-buildable (they delete tests + rows and prove, a precedented shape); `CC-BASELINE-0`'s
reclassification, `CC-HANDOFF-SPINE-0` (it edits the binding handoff template), and `CC-CLOSEOUT-0` are
DA-reviewed (they set retention/authoring doctrine). `CC-HANDOFF-SPINE-0` has no dependency on the sweep and
should land first so every subsequent handoff carries the leaner spine.

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
