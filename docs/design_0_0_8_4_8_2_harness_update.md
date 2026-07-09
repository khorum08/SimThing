# 0.0.8.4.8.2 — Harness Update: Agent Throughput without Ontology Regression

> **Status: OPEN (2026-07-09) as an Owner-authorized CONCURRENT CARVE-OUT.** The original
> "blocked-until-0.0.8.5-parked" gate is **superseded by Owner authorization** (recorded in
> `binding_conditions.tsv`): the gate failed in practice — product urgency deferred the tax cut
> indefinitely while per-PR ceremony grew. This track runs concurrent with the active product lane;
> the orientation Next-Rung pointer **stays on the product track** (dual-track note, this line).
>
> **Roles:** Fable = DA + orchestrator. Grok = implementer (writes code + tests per handoff).
> **Shape mandate (Owner):** few rungs, narrow handoffs, closed out via `track_closeout.sh` —
> and **no follow-on gen_orientation refactor pass**: any orientation-environment change this track
> needs lands inside its own rungs or not at all.
>
> **Purpose.** Keep the admission ladder, DA-equivalence of green RELIABLE, workshop containment, and
> gate-wiring self-application. Cut **agent tax**: the flat per-PR ceremony floor (~3–6k tokens on
> light PRs) and the meta-sprawl surfaces (class detectors, body attestations, fixture accretion,
> orientation prose). Source evaluations: Fable DA review (2026-07-09) + Grok charter response —
> converged; do not re-litigate.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — admission ladder; residue-as-tripwire; trust flows from admission.
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** (esp. §0.6 anti-ceremony, §0.9 doctrine-as-type / DA-equivalence).
3. **This file** — the 0.0.8.4.8.2 canonical design file.
4. [`ci_screening_surface.md`](ci_screening_surface.md) — screening map (this track rewrites the *operational* loop, not the RELIABLE seal law).
5. [`design_0_0_8_4_7_orchestration_harness.md`](design_0_0_8_4_7_orchestration_harness.md) — M1–M5 spine this track *extends*, never forks beside.
6. [`handoff_template.md`](handoff_template.md) + [`agent_onboarding.md`](agent_onboarding.md) — authoring surfaces; edits are **gate-wiring**.
7. [`track_closeout_protocol.md`](track_closeout_protocol.md) — this track closes through its own substrate (dogfood).

**Established decisions — do NOT re-derive:**

- **Do not weaken RELIABLE allowlists, inventory drift FAIL, orient receipt rule-stamp freshness, or workshop default-delete.**
- **Do not put GPU/Bevy/desktop proof on GHA.**
- **Do not add a new Rust orchestrator crate or a parallel `standards/` directory.**
- **Do not grow scan count, class count, or fixture count as progress.** Growth without paired retirement is a regression signal.
- **Gate-wiring remains DA-reserve.** Every rung here is gate-wiring unless marked otherwise; nothing self-merges.
- **Precedented classes stay; their *implementation style* is the defect** (hardcoded detectors, body attestations). Fix the style, keep the concept.
- **Treeverify stays** — data + lifecycle intact; only its delivery vehicle folds into the clearance router (rung HU-2).

**Binding conditions (recorded at open):**

| rung | condition | status |
|---|---|---|
| HU-TRACK-OPEN-0 | 0.0.8.5-park-gate-superseded-by-owner-carveout-authorization | discharged (Owner 2026-07-09) |
| HU-CLOSEOUT-0 | no-new-clearance-class-registration-without-retirement-pairing-during-track | open |

---

## 1. Success metrics (falsifiable at closeout; snapshot at open + close in `docs/tests/hu_throughput_snapshot.tsv`)

| Metric | Baseline (2026-07-09) | Target |
|---|---|---|
| Coding-agent default local screen | whole-tree scan ~70s, ~415 ambient INSPECT | delta screen, p50 <10s, delta-only INSPECT |
| Unconditional coding checklist steps | 6–10 | ≤4 |
| Writes per new test | 2 tables by hand (inventory + boundary) | 1 write path |
| Required body attestations on newest class | 11 fields | diff-verifiable predicates; ~0 attestations |
| New-class implementation cost | ~250 lines bespoke bash | data rows only (DSL) |
| Inventory rows born to open-forever harness tracks | 415 @ 0.0.8.4.6 (open since birth) | 0.0.8.4.6 **closed**; fixtures on a fused lifecycle |
| `clearance_check.sh` / generated-orientation line counts | 1470 / 247 | tracked; net decrease |

---

## 2. PR ladder (5 rungs; each = one narrow Grok handoff; all gate-wiring / DA-reviewed)

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 1 | `HU-DELTA-SCAN-0` | **Delta-first coding screen.** `scripts/ci/agent_scan.sh`: thin wrapper — RELIABLE scoped for hard FAIL, HEURISTIC changed-files only, one-line footer `AGENT-SCAN-VERDICT: PASS\|FAIL\|INSPECT delta_inspect=N elapsed=Ns` (ambient whole-tree INSPECT never appears). Whole-tree `doctrine_scan.sh` remains CI/master control + maintainer tool. Update generated orientation coding checklist to ≤4 unconditional steps (orient-once → `cargo check -p` → `agent_scan` → focused test) — a generator data/text edit, **not** a generator refactor. Selftest fixtures: delta FAIL caught, ambient INSPECT excluded, footer stable. | **PROOF-PRESENT** — evidence [`hu_delta_scan_0_results.md`](tests/hu_delta_scan_0_results.md) | T2 |
| 2 | `HU-CLEARANCE-DSL-0` | **Data-driven class predicates + treeverify fold.** (a) Requirement DSL: class predicates (scope globs, forbidden globs, required diff-verifiable checks) move to TSV rows interpreted by one engine; **ban new `check_*_field` bash per class**; migrate the two TP admitted classes; body attestations that the router cannot verify from the diff are dropped from `requirements` (DA substantive review owns them). (b) Fold: every `DA-RESERVE(...)` verdict also emits `DA-TREEVERIFY-PROFILE:` from the same changed-file list (shared lib with `da_treeverify_lib.py`); `ORCHESTRATOR-CLEARABLE` never emits it; standalone CLI stays as maintainer tool; `--check-lifecycle` stays in doctrine-scan. Net `clearance_check.sh` line count must **decrease**. | **PROOF-PRESENT** — evidence [`hu_clearance_dsl_0_results.md`](tests/hu_clearance_dsl_0_results.md) | T2 |
| 3 | `HU-INVENTORY-ONEWRITE-0` | **One write path for tests.** Adding/removing a test edits `test_inventory.tsv` only; `test_lifecycle_boundary_rows.tsv` is derived (generator or sync step) with drift gate unchanged as truth. Agents never hand-edit two tables for one test. Prove: new-test fixture = one edit; drift PASS; boundary check PASS. | NOT STARTED | T2 |
| 4 | `HU-FIXTURE-LIFECYCLE-0` | **Harness eats its own dog food.** Mint `harness-fixture` birth_track (open, fused semantics documented); rebirth the 0.0.8.4.6 seal-proof fixture cohort onto it with a documented necessity note per fixture *family* (not per file); then **close `0.0.8.4.6-ci-scaffolding` via `track_closeout.sh`** (build-manifest → check-eval → apply). Fixture-count snapshot recorded; unused families flagged to the pen. No fixture deleted without its scan surface named. | NOT STARTED | T2 |
| 5 | `HU-CLOSEOUT-0` | **Caps + metrics + close.** DOC-BUDGET rows for `orchestrator_orientation.md`, `handoff_template.md`, `agent_onboarding.md`, `agents.md`; `hu_throughput_snapshot.tsv` close-side numbers filled; §1 metrics table stamped with measured values; discharge open binding condition; **close this track via `track_closeout.sh`**. | NOT STARTED | T2 / DA |

**Sequencing:** 1 → 2 → 3 land in order (each unblocks measured tax); 4 may run parallel after 1; 5 last.
**Every rung:** carried ORIENT-RECEIPT, clearance verdict on the PR, doctrine_scan green, selftests only for surfaces the diff touches (§4B necessity-scoping), results doc ≤60 lines.

---

## 3. Standing DA rulings for this track

1. Throughput is not a licence to weaken ontology defense — any rung softening RELIABLE FAIL, allowlist grammar, or drift FAIL is rejected.
2. Prefer deleting process over adding process; a ceremony with no machine gate is cruft.
3. Meta-objects (classes, required fields, fixtures, orientation bullets, scripts) face the same Necessity Test as tests.
4. An attestation the router cannot verify from the diff is banned from `requirements` — not capped, banned.
5. Machine state over prose state; no new English-pattern completion matching.
6. Results docs for this track are ≤60 lines; the manifest/report substrate carries the detail.

## 4. What this track does NOT do

Does **not** re-open corpus necessity sweeps · does **not** touch GPU proof · does **not** auto-merge gate-wiring ·
does **not** grow HEURISTIC to hard-FAIL · does **not** refactor `gen_orientation.sh` beyond the rung-1 checklist text ·
does **not** register any new clearance class without a paired retirement (binding condition).

## 5. References

Fable DA harness evaluation (2026-07-09, this session) · Grok charter response (2026-07-09) ·
[`design_0_0_8_4_8_corpus_clearance.md`](design_0_0_8_4_8_corpus_clearance.md) §4B necessity-scoped selftests ·
[`track_closeout_protocol.md`](track_closeout_protocol.md).
