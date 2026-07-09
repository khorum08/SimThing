# 0.0.8.4.8.2 — Harness Update: Agent Throughput without Ontology Regression

> **Status: CLOSED (2026-07-09)** via `track_closeout.sh` dogfood (`HU-CLOSEOUT-0`).
> Owner-authorized CONCURRENT CARVE-OUT completed; product lane Next-Rung pointer was never
> hijacked (dual-track held).
>
> **Roles:** Fable = DA + orchestrator. Grok = implementer (writes code + tests per handoff).
> **Shape mandate (Owner):** few rungs, narrow handoffs, closed out via `track_closeout.sh` —
> and **no follow-on gen_orientation refactor pass**: any orientation-environment change this track
> needed landed inside its own rungs.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; held every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2 / §1.2.1** — admission ladder; residue-as-tripwire; trust flows from admission.
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** (esp. §0.6 anti-ceremony, §0.9 doctrine-as-type / DA-equivalence).
3. **This file** — the 0.0.8.4.8.2 canonical design file.
4. [`ci_screening_surface.md`](ci_screening_surface.md) — screening map (this track rewrote the *operational* loop, not the RELIABLE seal law).
5. [`design_0_0_8_4_7_orchestration_harness.md`](design_0_0_8_4_7_orchestration_harness.md) — M1–M5 spine this track *extended*, never forked beside.
6. [`handoff_template.md`](handoff_template.md) + [`agent_onboarding.md`](agent_onboarding.md) — authoring surfaces; edits are **gate-wiring**.
7. [`track_closeout_protocol.md`](track_closeout_protocol.md) — this track closed through its own substrate (dogfood).

**Established decisions — held:**

- Did **not** weaken RELIABLE allowlists, inventory drift FAIL, orient receipt rule-stamp freshness, or workshop default-delete.
- Did **not** put GPU/Bevy/desktop proof on GHA.
- Did **not** add a new Rust orchestrator crate or a parallel `standards/` directory.
- Did **not** grow scan count as progress; fixture lifecycle fused then closed open-forever track.
- **Gate-wiring remained DA-reserve** — nothing self-merged.
- **Precedented classes kept; implementation style fixed** (data-driven predicates).
- **Treeverify stayed** — folded into clearance DA-RESERVE emission (rung HU-2).

**Binding conditions:**

| rung | condition | status |
|---|---|---|
| HU-TRACK-OPEN-0 | 0.0.8.5-park-gate-superseded-by-owner-carveout-authorization | discharged (Owner 2026-07-09) |
| HU-CLOSEOUT-0 | no-new-clearance-class-registration-without-retirement-pairing-during-track | **discharged** (zero new classes during track; verified at close) |

---

## 1. Success metrics (measured at close; full rows in [`hu_throughput_snapshot.tsv`](tests/hu_throughput_snapshot.tsv))

| Metric | Baseline (2026-07-09) | Close (measured) |
|---|---|---|
| Coding-agent default local screen | whole-tree ~70s, ~415 ambient INSPECT | **agent_scan** live light **10–11s**, `delta_inspect=0`; selftest p50~2s |
| Unconditional coding checklist steps | 6–10 | **4** (orient → cargo check -p → agent_scan → focused test) |
| Writes per new test | 2 tables (inventory + boundary) | **1** (`test_inventory.tsv`; boundary ledger retired) |
| Required body attestations on newest class | 11 fields | **3** proof-identity (`tested_code_sha\|coverage_basis\|ci_green`) |
| New-class implementation cost | ~250 lines bespoke bash | **data rows** (`class_predicates.tsv` + generic engine) |
| Inventory rows on open-forever harness track | 415 @ 0.0.8.4.6 (423 at closeout) | **0** on 0.0.8.4.6 (**closed**); 423 on `harness-fixture` |
| `clearance_check.sh` / orientation lines | 1470 / 247 | **1415** / **226** (orientation re-measured at close) |

---

## 2. PR ladder (5 rungs — all GRADUATED)

| # | Rung | Deliverable | Exit proof | Merge |
|---|---|---|---|---|
| 1 | `HU-DELTA-SCAN-0` | Delta-first coding screen (`agent_scan.sh`); checklist ≤4 | [`hu_delta_scan_0_results.md`](tests/hu_delta_scan_0_results.md) | **GRADUATED** #1249 `e3e42c66` |
| 2 | `HU-CLEARANCE-DSL-0` | Data-driven class predicates + treeverify fold; attestation ban | [`hu_clearance_dsl_0_results.md`](tests/hu_clearance_dsl_0_results.md) | **GRADUATED** #1250 `a801ffe0` |
| 3 | `HU-INVENTORY-ONEWRITE-0` | Boundary audit ledger retired; one table | [`hu_inventory_onewrite_0_results.md`](tests/hu_inventory_onewrite_0_results.md) | **GRADUATED** #1251 `e3e84a25` |
| 4 | `HU-FIXTURE-LIFECYCLE-0` | `harness-fixture` + close 0.0.8.4.6 via closeout substrate | [`hu_fixture_lifecycle_0_results.md`](tests/hu_fixture_lifecycle_0_results.md) | **GRADUATED** #1252 `f069b392` |
| 5 | `HU-CLOSEOUT-0` | Caps + metrics + close this track through dogfood closeout | [`hu_closeout_0_results.md`](tests/hu_closeout_0_results.md) | **GRADUATED** #1253 (this PR) |

---

## 3. Standing DA rulings (held through close)

1. Throughput is not a licence to weaken ontology defense.
2. Prefer deleting process over adding process.
3. Meta-objects face the same Necessity Test as tests.
4. Attestations the router cannot verify from the diff are banned from `requirements`.
5. Machine state over prose state.
6. Results docs for this track ≤60 lines.

## 4. What this track did NOT do

Did **not** re-open corpus necessity sweeps · did **not** touch GPU proof · did **not** auto-merge gate-wiring ·
did **not** grow HEURISTIC to hard-FAIL · did **not** refactor `gen_orientation.sh` beyond the rung-1 checklist text ·
did **not** register any new clearance class without a paired retirement (binding discharged clean).

## 5. References

Fable DA harness evaluation (2026-07-09) · Grok charter response (2026-07-09) ·
[`design_0_0_8_4_8_corpus_clearance.md`](design_0_0_8_4_8_corpus_clearance.md) §4B ·
[`track_closeout_protocol.md`](track_closeout_protocol.md) ·
[`hu_throughput_snapshot.tsv`](tests/hu_throughput_snapshot.tsv).
