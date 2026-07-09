# 0.0.8.4.8.2 — Harness Update: Agent Throughput without Ontology Regression

> **Status: DRAFTED / NOT OPENED (2026-07-08, executive DA).** Owner-directed track to implement the
> DA evaluation of the rustification harness and CI for **agentic coding throughput**. Sits in the
> 0.0.8.4.x Rustification lane after closed 0.0.8.4.6 (CI scaffolding), 0.0.8.4.7 (orchestration harness),
> and parked 0.0.8.4.8 (corpus clearance). **Do not open until 0.0.8.5 Terran-Pirate is parked** —
> no concurrent harness rewiring while the scenario track is mid-closeout.
>
> **Purpose.** Keep the admission ladder, DA-equivalence of green RELIABLE, workshop containment, and
> gate-wiring self-application. Cut **agent tax**: slow default inner loop, ambient HEURISTIC noise,
> novelty-default clearance, dual ledgers, prose-driven orientation completion, PR-body hygiene kabuki,
> and over-weight handoffs for mechanical workshop rungs. Success is measured in **seconds-to-first-fail
> for coding agents**, **DA reviews per precedented envelope PR**, and **INSPECT signal rate** — not in
> new scripts or new prose.
>
> **Governing DA evaluation (source):** session DA review of harness busyness/efficiency (2026-07-08).
> This document converts that review into a production ladder. Amendments only by addition after open.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — admission ladder; residue-as-tripwire; trust flows from admission.
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** (esp. §0.6 anti-ceremony, §0.9 doctrine-as-type / DA-equivalence).
3. **This file** — the 0.0.8.4.8.2 canonical design file.
4. [`ci_screening_surface.md`](ci_screening_surface.md) — screening map + agent onboarding §7 (this track rewrites the *operational* loop, not the RELIABLE seal law).
5. [`design_0_0_8_4_7_orchestration_harness.md`](design_0_0_8_4_7_orchestration_harness.md) — M1–M5 spine this track *extends*, never forks beside.
6. [`handoff_template.md`](handoff_template.md) + [`agent_onboarding.md`](agent_onboarding.md) — authoring surfaces; edits are **gate-wiring**.

**Established decisions — do NOT re-derive:**

- **Do not weaken RELIABLE allowlists, inventory drift, orient receipt freshness on rule sources, or workshop default-delete.** Those are load-bearing.
- **Do not put GPU/Bevy/desktop proof on GHA.** Owner-local remains citable; GHA stays greppable/CPU.
- **Do not add a new Rust orchestrator crate or a parallel `standards/` framework** (0.0.8.4.7 DA ruling stands). Extend the thin engine + data + selftest spine.
- **Do not grow scan count as progress.** Growing RELIABLE/HEURISTIC count without retirement is a regression signal (0.0.8.4.6).
- **Gate-wiring remains DA-reserve.** Edits to clearance/relay/orient/scans/allow/handoff surfaces never self-merge.
- **0.0.8.5 parking is a hard gate on track open.** Opening mid-TP burns the scarce resource this track exists to protect.

---

## 1. Root cause this track closes

The harness successfully **rustified process** (judgment → verdict). Agent throughput now fails for a
different reason: **the operational surface outgrew the minimum productive coding loop.**

Measured / observed (2026-07-08, post-TP live-run):

| Symptom | Effect on agents |
|---|---|
| Whole-tree `doctrine_scan.sh` ~70s+ as "inner loop" | Agents skip local scan; FAIL becomes PR-time surprise |
| ~415 whole-tree HEURISTIC INSPECT (ambient) | Agents learn INSPECT is weather, not a tripwire |
| Workshop composition rungs → `DA-RESERVE(novelty)` by default | DA over-used; clearance ladder under-used |
| Dual inventory + boundary rows per new test | Dual-edit kabuki; drift between tables |
| Ladder completion via prose pattern match | False-complete orientation (e.g. "Complete Scope Ledger… DA…") |
| Full relay/handoff sections on every mechanical rung | Context spent on process, not RF/STEAD |
| Manual PR body `ci_green` / SHA churn | Stale bodies while tree is green |
| Harness selftests listed in coding "inner loop" | Copy-paste ritual; not necessity-scoped |
| ~42 CI scripts, multi-verdict lexicon, 550-line screening surface as cold-start | Cognitive load before first compile |

**Ratio estimate (DA, 2026-07-08):** ~60% load-bearing · ~25% necessary-but-overweight · ~15% agent tax.
This track attacks the 15% and thins the 25% **without** cutting the 60%.

---

## 2. Success metrics (falsifiable at closeout)

| Metric | Baseline (open day) | Target at closeout |
|---|---|---|
| Coding agent default local screen | whole-tree scan ~70s | **delta/changed-file screen &lt;10s** p50 on Windows git-bash |
| Coding role checklist commands | 6–10 including selftests | **≤4** unconditional steps |
| `DA-RESERVE(novelty)` rate on workshop-envelope PRs matching a registered class shape | high (TP 8.0/8.1 pattern) | **&lt;20%** of such PRs (rest ORCHESTRATOR-CLEARABLE or FAIL/remedy) |
| Whole-tree HEURISTIC INSPECT presented as agent-relevant | ~415 ambient | **delta-only** agent report; ambient baseline is operator diagnostic only |
| Dual-edit required for new test rows | inventory + boundary | **one write path** |
| Orientation false-complete from open-rung prose | known (closeout cell) | **zero** under machine status cells + selftest |
| DA reviews spent on process residue (stale body, dual ledger, wrong inner loop) | recurring | **zero required** for mechanical envelope rungs |

Telemetry: extend `clearance_ledger.tsv` sampling + a small `harness_throughput.tsv` snapshot at open and close (wall times + novelty rates). No dashboard.

---

## 3. Standing DA rulings for this track

1. **Throughput is not a licence to weaken ontology defense.** Any rung that softens RELIABLE FAIL, allowlist grammar, or inventory drift FAIL is out of scope and rejected.
2. **Prefer deleting process over adding process.** If a ceremony has no machine gate and no measured benefit, delete or demote — do not document it harder.
3. **Role-thin loops.** Coding / orchestrator / DA checklists must be disjoint; selftests are maintainer/gated surfaces only.
4. **Precedented classes keep pace with production envelopes.** A repeated workshop shape that always escalates novelty is a **harness defect**, not a DA feature.
5. **Machine state over prose state.** Ladder completion, receipt freshness (already rule-stamped), and PR proof identity must not depend on English pattern match where avoidable.
6. **Track open only after 0.0.8.5 park.** Recorded as a binding condition row at open: `HU-TRACK-OPEN-0 blocked-until-0.0.8.5-parked`.

---

## 4. PR ladder

> **Tier key:** T1 = mechanical / precedented-class eligible after class registration · T2 = gate-wiring / DA-reserve · Docs = docs-only when isolated.
> **Sequencing:** Phase 0 must land first. Phase 1 is the critical path for coding agents. Phase 2 unblocks orchestrator/DA cost. Phase 3 is ceremony shrink. Phase 4 closeout.
> **Every gate-wiring PR:** ORIENT-RECEIPT, clearance selftest + relay-lint selftest when those surfaces change, doctrine_scan green, disposable closeout sample if closeout substrate touched.

### Phase 0 — Track open (after 0.0.8.5 park)

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 0.0 | `HU-TRACK-OPEN-0` | This document lands active; `active_track` may point here only after 0.0.8.5 park; binding condition `blocked-until-0.0.8.5-parked` **discharged** in-PR; evidence-index row; harness header cited. Docs only. | Track open; orientation points here or explicit dual-track note; condition row discharged. | Docs / DA |

### Phase 1 — Coding agent loop (critical path)

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 1.0 | `HU-DELTA-SCAN-0` | **Delta-first local screen as the default coding command.** Ship `scripts/ci/agent_scan.sh` (or thin wrapper over `doctrine_pr_scan.sh`) that: (a) RELIABLE whole-tree or changed-crate scoped as needed for hard FAIL, (b) HEURISTIC **PR-delta / changed-files only**, (c) prints a one-line footer `AGENT-SCAN-VERDICT: PASS\|FAIL\|INSPECT … elapsed=Ns`. Document as the **only** routine local screen. Whole-tree `doctrine_scan.sh` remains CI/master positive control + maintainer tool — **not** the coding inner loop. | Agent scan p50 &lt;10s on a sample docs+workshop delta; whole-tree still used in GHA; selftest/prove cases for delta HEURISTIC; orientation + `agents.md` + handoff Canonical Entrypoints point at agent_scan, not whole-tree. | T2 gate-wiring |
| 1.1 | `HU-ROLE-CHECKLIST-0` | **Role-minimal checklists** generated into orientation by role: **coding** = orient-once (session) → `cargo check -p` → `agent_scan` → focused test → results skeleton; **orchestrator** = clearance + relay-lint + triage; **da** = tree + residue only. Remove harness selftests from coding checklist; keep them as "when surface changes" conditionals (already §4B of 0.0.8.4.8 — enforce in generated orient, not only prose). Update `agent_onboarding.md` tables. | Role sections in orientation show ≤4 unconditional coding steps; selftest not listed as unconditional; cold-start fixtures updated. | T2 gate-wiring |
| 1.2 | `HU-AGENT-DIGEST-0` | **One-screen agent cold-start digest** (generated): sanctioned doors summary pointer + forbidden pattern ids + the 3–4 coding commands + active rung pointer. Either a section of `orient.sh --role=coding` or `docs/agent_surface.md` freshness-gated like `sanctioned_surface.md`. **Does not replace** `ci_screening_surface.md` (reference stays full). | Coding orient output fits one screen of high-signal commands; freshness `--check` in CI; no second source of allowlist truth. | T2 gate-wiring |

### Phase 2 — Clearance + signal quality (DA cost decay)

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 2.0 | `HU-ENVELOPE-CLASSES-0` | **Register precedented classes for repeated production envelopes** so novelty is rare: at minimum (a) `workshop-scenario-rung` — workshop src/tests + results + inventory/boundary under an open scenario `birth_track`; (b) `docs-status-stamp` — design ladder + results COMPLETE + orientation regen only; (c) optional `clausething-fixture-rung` if still needed post-0.0.8.5. Requirements: `tested_code_sha`, `coverage_basis`, `ci_green`, `no_engine_crate` / scope globs as appropriate. Suspend-friendly rows. | Clearance selftests: matching fixture → `ORCHESTRATOR-CLEARABLE`; engine-crate touch → novelty/reserve; class `status=suspended` → class-suspended. Document which historical TP shapes would have cleared. | T2 gate-wiring |
| 2.1 | `HU-HEURISTIC-BUDGET-0` | **Kill ambient INSPECT for agents.** (1) Agent/PR report surfaces **delta HEURISTIC only**; whole-tree HEURISTIC count is operator diagnostic (`doctrine_scan` master control), never the agent footer. (2) Cadence pass: for each HEURISTIC id, either promote-path note, tighten excludes, or schedule delete if chronic non-finding (start with `SPEC-LOWERER-KIND-READ` triage strategy — not silent deletion without data). (3) Optional: `INSPECT-SPAM` already exists — wire agent-facing "0 delta inspect" as the happy path. | Agent scan footer inspect count = branch-introduced only on sample PRs; written disposition per HEURISTIC id in results doc; no RELIABLE weakened. | T2 (scan/report) |
| 2.2 | `HU-CLEARANCE-FASTPATH-0` | **Coding/orchestrator clearance UX:** document + optional `clearance_check.sh --pr` quick path that does not require running full `--selftest` for routine PR routing; selftest remains when clearance surface changes + CI. Ensure novelty reasons name **which class almost matched** (FAIL-as-teacher) to drive envelope-class maintenance. | Sample PR routing without selftest; selftest still required on harness edit; almost-match messaging in fixture. | T1/T2 |

### Phase 3 — Bookkeeping + ceremony shrink

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 3.0 | `HU-INVENTORY-ONEWRITE-0` | **One write path for new tests.** Adding/removing a test updates `test_inventory.tsv` (or a single generator input); boundary rows are **derived** (`test_lifecycle_boundary_rows.tsv` regen from inventory + policy) or dual-write is automated by `scripts/ci/inventory_sync.sh`. Drift gate remains the truth. Agents never hand-edit two tables for one test. | New test fixture: one command or one file edit; drift PASS; boundary check PASS; docs updated. | T2 gate-wiring |
| 3.1 | `HU-LADDER-STATUS-CELLS-0` | **Machine ladder status** for orientation next-rung pointer. Production track tables gain a dedicated status token column or fenced machine cell: `status=open\|done\|parked` (exact grammar), **not** inferred from English "Complete/DA/…". Update `gen_orientation.sh` `is_completed_exit` to prefer machine tokens; keep prose as human narrative only. Selftest: open closeout-shaped prose without `status=done` does **not** complete; `status=done` does. | Orientation pointer selftests green; 0.0.8.5-style false-complete fixture fails closed; active tracks migrated or dual-read during transition. | T2 gate-wiring |
| 3.2 | `HU-HANDOFF-TIERS-0` | **Tiered handoffs.** `handoff_template.md` gains **short form** (mechanical workshop / docs-stamp / inventory-only) vs **full form** (gate-wiring, novelty, closeout, seal-residue). Relay-lint: short form requires reduced block set; full form unchanged. Coding agents default short when class is envelope-clearable. | Relay-lint fixtures for short PASS / short missing-proof FAIL / full still required on gate-wiring; template + onboarding updated. | T2 gate-wiring |
| 3.3 | `HU-PROOF-FOOTER-0` | **Generated proof identity over PR-body kabuki.** Sticky comment or `scripts/ci/emit_proof_footer.sh` writes `tested_code_sha` / `coverage_basis` / `ci_green` from live head + checks; relay-lint accepts footer **or** body fields; body churn no longer required for green CI updates. Optional GHA job posts sticky on doctrine-scan success. | Stale body with fresh sticky/footer clears lint when policy says so; missing both still FAIL; no live-pointer regression. | T2 gate-wiring |
| 3.4 | `HU-STATUS-STAMP-AUTOMATION-0` | **Optional automation for design-ladder status stamps** after DA/orchestrator graduation: docs-only PR or bot PR filling merge SHA + results COMPLETE + orientation regen from a template. Human/DA still approves gate-wiring. Reduces manual status-stamp PRs that currently follow every graduation. | One dry-run stamp on a fixture track doc; does not auto-merge gate-wiring; orientation check PASS. | T1 (docs tooling) |

### Phase 4 — Maintainer cost + closeout

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 4.0 | `HU-SELFTEST-SPEED-0` | **Maintainer/CI selftest cost down without coverage loss:** parallelize independent fixture cases where safe; cache sandbox trees; document Windows git-bash cost. Does **not** remove known-bad coverage. Target: material wall-time reduction on `doctrine_selftest.sh` (measure before/after). | Before/after seconds recorded; selftest still FAILS each known-bad and stays quiet on traps. | T1/T2 |
| 4.1 | `HU-SCREENING-DOC-SPLIT-0` | **Split operator surface from reference:** `ci_screening_surface.md` remains full reference; agent path is orient + agent digest only. Compress any duplicated onboarding that restates §7 into pointers. Doc budget: no net prose growth without sunset of restated blocks. | Doc budget PASS; agents.md / onboarding point to orient, not 550-line cold-start; screening surface still authoritative for scan edits. | Docs |
| 4.2 | `HU-CLOSEOUT-0` | Scope Ledger over every §1 metric; Deviation Records for deferred items; sunset any temporary dual-read (ladder status prose fallback); clearance class list frozen for next scenario track; DA sign-off. | Metrics table filled with measured numbers; orientation + agent_scan + envelope classes live on master; track closed via `track_closeout.sh` discipline. | DA |

---

## 5. Additional high-impact interventions (included above or parked)

| Idea | Disposition |
|---|---|
| Delta-first agent scan | **In ladder** 1.0 |
| Role-minimal checklists | **In ladder** 1.1 |
| One-screen agent digest | **In ladder** 1.2 |
| Workshop/docs envelope precedented classes | **In ladder** 2.0 |
| HEURISTIC ambient noise kill | **In ladder** 2.1 |
| Clearance almost-match + no routine selftest | **In ladder** 2.2 |
| Single inventory write path | **In ladder** 3.0 |
| Machine ladder status cells | **In ladder** 3.1 |
| Tiered handoffs / short relay | **In ladder** 3.2 |
| Generated proof footer / sticky CI | **In ladder** 3.3 |
| Status-stamp automation | **In ladder** 3.4 |
| Selftest speed | **In ladder** 4.0 |
| Doc split reference vs cold-start | **In ladder** 4.1 |
| **Results-doc skeleton generator** (`scripts/ci/new_rung_results.sh`) | **Parked optional** — open only if short-form handoff still leaves results ceremony high; not on critical path |
| **Parallel doctrine_scan RELIABLE workers** | Fold into 1.0/4.0 if delta path insufficient |
| **Anchor domain auto-ack from diff paths** | **Parked** — nice-to-have after 3.2; don't open unless missing-anchor-ack is measured agent pain |
| **Merge-queue required status = doctrine-scan only** (drop non-blocking exec confusion) | **Docs/process note in 1.1** — do not change branch protection without Owner |
| **Delete unused CI scripts / consolidate README** | **Closeout hygiene in 4.2** — inventory scripts; delete dead; no big-bang rewrite |

---

## 6. What this track does NOT do

- Does **not** open while 0.0.8.5 is active.
- Does **not** re-open corpus necessity sweeps (0.0.8.4.8 parked scope).
- Does **not** invent `simthing-orchestrator` crate or new policy framework directories.
- Does **not** move GPU proof onto GHA.
- Does **not** make HEURISTIC hard-FAIL without promotion design.
- Does **not** auto-merge gate-wiring or allowlist widenings.
- Does **not** canonize 0.0.8.5 workshop residue (that is TP closeout / post-admission).

---

## 7. Dependencies and open gate

```text
[0.0.8.5 TP-DA-CLOSEOUT-0 park]
        │
        ▼
 HU-TRACK-OPEN-0
        │
        ├─► Phase 1 (1.0 → 1.1 → 1.2)     # coding loop — ship first
        │
        ├─► Phase 2 (2.0 → 2.1 → 2.2)     # after or parallel once 1.0 exists
        │         (2.0 may start after 1.0; 2.1 needs 1.0 agent footer)
        │
        ├─► Phase 3 (3.0 … 3.4)           # ceremony shrink; 3.1 unblocks orientation trust
        │
        └─► Phase 4 (4.0 → 4.1 → 4.2)
```

**Binding condition (record at open):**

| rung | condition | status |
|---|---|---|
| HU-TRACK-OPEN-0 | blocked-until-0.0.8.5-parked | open until TP park |

---

## 8. Risk classes (graduation routing defaults)

| Rung class | Default risk | DA posture |
|---|---|---|
| agent_scan / orient checklist / agent digest | `gate-wiring` | deep — prove FAIL-as-teacher + no silent weaken |
| envelope precedented classes | `gate-wiring` + data-deliverable | deep — prove non-match still reserves; suspended works |
| HEURISTIC budget | `semantic` / scan edit | deep if pattern/exclude changes; light if report-only |
| inventory one-write / ladder status cells | `gate-wiring` | deep — drift must still FAIL unledgered tests |
| handoff tiers / proof footer | `gate-wiring` | deep — short form must not launder missing proofs |
| status-stamp automation / selftest speed | `none`–`data-deliverable` | light if docs/tooling only |
| closeout | Owner-channeled | Scope Ledger + metrics |

---

## 9. References

- DA harness efficiency evaluation (session 2026-07-08) — source of §1 symptoms and §2 metrics.
- [`design_0_0_8_4_6_ci_scaffolding.md`](design_0_0_8_4_6_ci_scaffolding.md) — Track A/B/C/D closed baseline.
- [`design_0_0_8_4_7_orchestration_harness.md`](design_0_0_8_4_7_orchestration_harness.md) — M1–M5; extend not fork.
- [`design_0_0_8_4_8_corpus_clearance.md`](design_0_0_8_4_8_corpus_clearance.md) — parked; §4B necessity-scoped selftest precedent.
- [`ci_screening_surface.md`](ci_screening_surface.md) — operator/reference surface this track splits from agent cold-start.
- [`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md) — must park before open.

---

## 10. Open / park note for operators

**Do not run `gen_orientation.sh --open` on this file until 0.0.8.5 is parked and Owner/DA authorizes track open.**  
Until then this document is a **draft production plan only** — not the active orchestration track.
