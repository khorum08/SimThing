# 0.0.8.4.8.3 — Orientation Curation: Constitutional Coverage without Context Bloat

> **Status: OPEN (2026-07-10, Owner-directed).** Corrects the curation deficit found in the
> temp-DA orientation review (2026-07-09): the designated doctrine carrier (`doctrine_anchors.tsv`)
> held **4 rows** against ~15 constitutional surfaces, and its trigger fired on **relay prose
> keywords** — circular (only agents who already speak the doctrine trigger it) and covering only
> 2 of 4 existing anchors' domains. The orientation Next-Rung pointer moves here; the 0.0.8.6
> production lane is **parked** at 9.1h/9.2 boundary until this track closes (Owner ruling).
>
> **Roles:** Fable = DA + orchestrator. Codex/Grok = implementers per handoff.
> **Shape mandate:** few rungs, narrow handoffs, results docs ≤60 lines, close via
> `track_closeout.sh`. Kernel lane is **owner-gated per rung** (amendment valve).
>
> **Doctrine of this track:** anchors are *thin pointers served on mechanical triggers*, never
> digest essays. `gen_orientation` stays operational; doctrine reaches agents through the anchor
> library + a queryable, observable curation surface. Coverage is guaranteed by **diff-path
> adjacency**, not by an agent knowing the right words.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base:** `simthing_core_design.md` §1.2/§1.2.1 · `design_0_0_8_3.md` §0 (esp. §0.6, §0.9) ·
this file · `ci_screening_surface.md` · `stead_spatial_contract.md` · `docs/invariants.md` ·
`track_closeout_protocol.md`. **Held decisions:** no RELIABLE weakening; no new crate/framework;
no class/scan/fixture growth without retirement; gate-wiring stays DA-reserve; attestations the
router cannot diff-verify stay banned; meta-objects face the Necessity Test.

**Binding conditions (recorded at open):**

| rung | condition | status |
|---|---|---|
| OC-KERNEL-LANE | each `OC-K*` rung blocked-until-owner-amendment-valve-authorization (per rung, not per lane) | open |
| OC-CLOSEOUT-0 | reach-log and anchor tables must carry decay rules before close (no unbounded growth surfaces) | open |

---

## 1. Coverage catalogue (the target anchor library)

Grok's 11 risks, refined by DA survey of core design, constitution, STEAD contract, invariants,
ADRs, and the kernel crate. `always-on` = one-line cold-start spine in orientation; everything
else is trigger-served. **Trigger domains bind to diff paths (rung OC-2), not prose.**

| # | Surface | Source | trigger_domains (path-derived) | always-on line? | kernel lane |
|---|---|---|---|---|---|
| 1 | FIELD_POLICY + time/decisions (CPU's only job) | core §8 + FIELD_POLICY | sim, driver, wgsl, sim-clock | YES (six-line) | OC-K1 |
| 2 | Admission ladder / Necessity Test | constitution §0.9, CI | (none — CI-owned) | pointer only | — |
| 3 | Seal / residue-as-tripwire / cross-crate seal law | constitution §0.9 | kernel, gpu, unsafe-adjacent | — | ongoing |
| 4 | STEAD contract §1–§4, §8, §9 (layout vs execution admission; ambient/sparse; required tests; withdrawn phrases) | stead_spatial_contract | map, spatial, mapgenerator, clausething-spatial | — | narrow (done) |
| 5 | Tree / owners-never-spatial / one-tree | core §2 | spec, hydration | — | — |
| 6 | Property→Value model; RF arenas/channels; Balance; **overlays/orderband**; sparse RegionCell + RF substrate ADRs | core §3, §5, §6 + 2 ADRs | kernel-columns, rf, overlay, driver | — | OC-K2 |
| 7 | EML extension ladder (gadget tree before opcode) | core §5/EML docs | kernel-eml, wgsl | — | OC-K4 |
| 8 | Spec fidelity / anti-ceremony / Deviation Record | constitution §0.6 | (closure surfaces) | YES (one line) | — |
| 9 | Exact numeric authority / Candidate F | constitution §0.7 | kernel-magnitude, threshold paths | — | OC-K3 |
| 10 | Drift detectors / six-line harness | core §9 | (retain set) | YES (pointer) | — |
| 11 | ClauseThing closed vertical | constitution §A, ClauseThingADR | clausething, mapgenerator | — | — |
| 12 | **Structural Execution Convergence Contract** (Studio→GPU: existing ops via driver/sim, never bespoke kernels) | stead §10 | gpu, driver, mapeditor-gpu, studio | — | — |
| 13 | **Session lifecycle ADR family** (game-mode install, clone-then-commit, session assembly, state replay) | 4 ADRs | driver-session, feeder-session, mapeditor-session | — | — |
| 14 | **Peripheral scope ADRs** (scripted-event scope; capability effect target scope) | 2 ADRs | feeder, capability paths | — | — |
| 15 | **Founding ontology** (§0.2 allocation always recursive; §0.3 all conflict is resource flow) + `invariants.md` registry pointer | constitution §0.2/§0.3 | — | YES (two lines) | — |

---

## 2. PR ladder

### Lane A — curation library + mechanical triggers (gate-wiring; DA-reviewed)

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| 1 | `OC-ANCHOR-CATALOG-0` | Populate `doctrine_anchors.tsv` to the §1 catalogue: one row per surface (§-precise headings, content hashes), trigger_domains per table. No digest essays — rows point at existing sections; zero new prose docs. `anchor_check.sh` green; `/anchor` serves each new id verbatim. | NOT STARTED | T2 |
| 2 | `OC-PATH-TRIGGER-0` | **Mechanical adjacency.** New data table `anchor_triggers.tsv` (`glob \| trigger_domains`) mapping repo paths → domains (kernel/**→seal; sim/**+*.wgsl→field-policy; map/spatial paths→stead; gpu+driver→convergence; etc. per §1). `relay_lint.required_trigger_domains()` computes domains from the **changed-file list** (union with the legacy prose regex, which becomes secondary); clearance router emits `REQUIRED-ANCHORS: <ids>` beside `DA-RESERVE` verdicts as a third consumer of its existing file list. Selftests: kernel-path diff whose relay never says a doctrine word → missing-anchor-ack; docs-only diff → none required; all four legacy anchors' domains now reachable. | NOT STARTED | T2 |
| 3 | `OC-QUERY-0` | **Queryable + observable curation.** `scripts/ci/anchor_query.sh --domain <d> \| --paths <files...> \| --grep <term>`: resolves against the anchor library only (serves anchored sections verbatim — agents stop raw-grepping doctrine); appends one row per query to `scripts/ci/anchor_reach_log.tsv` (`date \| role \| query \| anchors_served \| hit`) so the DA observes what agents reach for; `--grep` misses (hit=none) are the curator's update signal. Relay-lint anchor FAILs name the query command as the remedy (FAIL-as-teacher exposes the tool). Reach-log decay: `--prune <days>` mode + closeout reaps entries >30d (binding condition). | NOT STARTED | T2 |
| 4 | `OC-ORIENT-SLICE-0` | **Cold-start spine in orientation** (generated, budgeted): the §1 always-on lines (FIELD_POLICY six-line pointer; spec-fidelity §0.6 one-liner; §0.2/§0.3 ontology lines; invariants-registry pointer; drift-detector pointer) + the `anchor_query.sh` entrypoint, rendered role-scoped by `orient.sh`. Pointers only — DOC-BUDGET cap re-pointed with paid growth; no §3–§7 prose enters the digest. | NOT STARTED | T2 |
| 5 | `OC-DOCS-CASCADE-0` | **Docs cascade once A1–A4 are live** (the HU-DOCS-CONSOLIDATION pattern, in-track): `agent_onboarding.md` — `anchor_query.sh` in the coding loop's doctrine-adjacent line + reach-log added to the sprawl-instrument section; `ci_screening_surface.md` — anchor library / `anchor_triggers.tsv` / query+reach-log rows in the screening map **within its 525 cap** (restructure, never grow past); `docs/agents.md` one-liners; `handoff_template.md` Canonical Entrypoints — anchor_query named as THE doctrine-lookup entrypoint (agents stop raw-grepping doctrine docs); `track_closeout_protocol.md` only if reach-log decay touches it. Caps re-pointed to exact post-edit counts with justification; grep table proves presence; stale-reference count 0. | NOT STARTED | T2 |

### Lane B — kernel admission substrate (owner-gated PER RUNG; Grok implements; DA deep audit; amendment valve)

Each rung opens **only** on its own forgeability evidence: a compiling demonstration that the illegal
state is representable today, and the named HEURISTIC/scan it retires (retirement obligation in the
same PR). `admission-amendment-request: allowed`; `seal-residue-risk` declared per B1–B8.

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| K1 | `OC-K-DECISION-INGRESS-0` | Structural/commitment effects mintable **only** via sealed threshold→emission→boundary doors; CPU diagnostic/approximate types cannot construct commitment ingress (FIELD_POLICY remainder + core §8). | NOT STARTED | T2/Owner |
| K2 | `OC-K-COLUMN-ROLE-0` | `ColumnIndex`/role-keyed column access; raw `.data[N]` unrepresentable on sealed paths; retires the .data[N] HEURISTIC where baseline-zero. | NOT STARTED | T2/Owner |
| K3 | `OC-K-EXACT-GATE-0` | Magnitude-sensitive threshold/commitment registration requires exact-magnitude proof type (Candidate F); `ApproximateDiagnostic` cannot feed those registrations (constitution §0.7). | NOT STARTED | T2/Owner |
| K4 | `OC-K-EML-OPCODE-GATE-0` | Opcode/combine admission hard-gate at registration (no semantic ops); "gadget tree first" stays anchor prose — the gate types only the door. | NOT STARTED | T2/Owner |

### Lane C — close

| # | Rung | Deliverable | Exit proof | Tier |
|---|---|---|---|---|
| C | `OC-CLOSEOUT-0` | First reach-log report (what agents queried; misses → catalogue updates or explicit declines); §1 table stamped with anchor ids; meta-gauges snapshot; un-opened K rungs re-parked to backlog with Deviation Records (never left dangling); binding conditions discharged; close via `track_closeout.sh`. | NOT STARTED | DA |

**Sequencing:** A1 → A2 → (A3 ∥ A4) → A5 (cascade) → C. Lane B rungs slot anywhere after A1, each
on Owner authorization; C does not wait for unauthorized K rungs, but never precedes A5 — the
entry-point docs must describe the finished curation surface before the track closes.

---

## 3. Standing DA rulings

1. Anchors are pointers to existing sections — a rung that writes new doctrine prose to "cover" a
   surface is rejected; coverage means findability, not restatement.
2. Trigger truth is the diff, not the vocabulary. The prose regex survives only as a secondary net.
3. The reach-log is an observability surface, not a gate — no verdict ever depends on it.
4. Kernel rungs are evidence-first: no forgeability demo + no retiring scan ⇒ not opened.
5. Every new TSV in this track carries a decay rule at birth (prune/lease/closeout reap).
6. Results docs ≤60 lines; the catalogue table above is the single source for anchor targets.

## 4. What this track does NOT do

No gen_orientation refactor beyond the A4 slice · no digest essays · no new scans as coverage
theater · no kernel rung without owner valve + forgeability evidence · no touching the parked
0.0.8.6 product lane · no re-litigating the HU track's throughput decisions.

## 5. References

Temp-DA orientation review + Fable assessment (2026-07-09/10, session) · `doctrine_anchors.tsv` ·
`da_review_profile.tsv` (the path-glob pattern A2 reuses) · `track_closeout_protocol.md` ·
`design_0_0_8_4_8_2_harness_update.md` (throughput baselines this track must not regress).
