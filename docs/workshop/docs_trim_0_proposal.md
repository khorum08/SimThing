# DOCS-TRIM-0 — Proposal: trim governance scaffolding from the mandatory-read set

**Status:** APPLIED (product approved all edits, 2026-06-09; applied by design authority in the same session — D-1…D-7, I-1…I-7, A-1 all landed; TRIM-NEXT-1 remains flagged, not applied)
**Date:** 2026-06-09
**Author:** Opus (design authority synthesis), on product request
**Scope:** `docs/design_0_0_8_1.md`, `docs/invariants.md`, `docs/agents.md`,
`docs/simthing_core_design.md` (evaluation only — no cuts proposed), plus one flagged
follow-on for the production-track ledger.

---

## 1. The evaluation — what is rigor and what is theater

**The verdict: the testing requirements are NOT the theater. The history is.**

The rules that cost tokens to *comply* with — CPU-oracle bit-exact parity, Scenario Proof,
Deviation Records, Scope Ledgers, opt-in/default-off — each exists because it caught a real,
expensive failure (silent tier flattening, math-in-a-vacuum scenario closures, CPU-fold
regression drift). They are the greenfield rigor. **None are proposed for removal.**

The theater is everything that costs tokens merely to *read*:

1. **Inline provenance.** Dates, reviewer attributions, acceptance-memo links, artifact
   hashes, and incident narratives embedded inside binding rule text. A rule's authority
   does not depend on the reader knowing it was "Released by design authority (Opus,
   2026-05-29) on the SQRT-EXACT-5F exhaustive proof" — the rule binds either way, and the
   provenance is one `git log` away.
2. **Triple statement.** Specification Fidelity is stated in full three times (constitution
   §0.6, `invariants.md` table, guidance SPEC-FIDELITY-0 row). The §2.4 substrate-vocabulary
   gate is stated verbatim twice (constitution + invariants EML section). Each restatement
   is ~40 lines an agent re-reads with zero new information.
3. **Meta-ceremony.** §0.5's sub-rules about citation counts, link-layer ephemerality, and
   header sizing are ~25 lines of rules about how to format documents — ceremony regulating
   ceremony. The six-line checklist inside it is the load-bearing part.
4. **Closure narratives in the wrong home.** §4A's RR-ladder non-claims and open-findings
   paragraphs are closure-record material that already lives, verbatim, in the RR-4 test
   report. The constitution needs the verdict and the pointer, not the record.
5. **Fossils.** `agents.md` (990 lines) describes the Week-2/Week-3 implementation state,
   contains an OPEN decision prompt ("FMA divergence — decision required before writing
   Pass 1") for a decision long since made, and sketches WGSL for legacy passes deleted at
   S-3/S-6. This is the worst case: not dead weight but **actively contradicting reality**
   for any low-context agent that loads it.

**Token arithmetic (mandatory-read set for a typical track):**

| Doc | Now | After | Notes |
|---|---|---|---|
| `agents.md` | ~6,400 words | ~900 | rewrite as a pointer brief (A-1) |
| `design_0_0_8_1.md` | ~4,700 words | ~3,100 | D-1…D-7 |
| `invariants.md` | ~4,300 words | ~2,600 | I-1…I-7 |
| `simthing_core_design.md` | ~3,600 words | ~3,600 | unchanged — see §3 |
| **Total** | **~19,000 words (≈25k tokens)** | **~10,200 (≈13.5k)** | **≈46% cut, zero rules removed** |

---

## 2. Constitutional flag — §0 edits require explicit product mandate

§0 declares itself carry-forward-verbatim, "amending only by *addition* — never silent
removal or weakening." Edits **D-1** and **D-2** remove *text* from §0 (an implementation
note and formatting sub-rules), while preserving every doctrinal obligation. The rule's
purpose is preventing **silent** weakening; an openly recorded, product-approved compression
that preserves substance is the opposite of the failure mode it guards. Approving this
proposal **is** that mandate; the worklog entry records it. If product prefers strict
construction, D-1/D-2 can instead land as a version bump to 0.0.8.2 with the compressed §0 —
same content, more ceremony.

---

## 3. `simthing_core_design.md` — evaluated, no cuts proposed

The core design doc is the *replacement* for what is being cut elsewhere: it absorbs the
pedagogy ("What These Invariants Buy"), the paradigm rationale, and the theoretical anchors,
so the other docs can shrink to pure rule tables and process. Its token cost is the one
deliberate spend: it is the single file a low-context agent holds for an entire task. Every
section is either a binding shape, a litmus test, or the *why* that prevents
rationalization. Trimming it would re-create the vacuum that caused the drift.

---

## 4. The edits (approve individually; each is independently applicable)

### File: `docs/design_0_0_8_1.md`

**D-1 — §0.2 implementation note: 14 lines → 4.** Replace the full blockquote with:

> *Status note:* the recursive ladder is proven end-to-end on GPU against a recursive CPU
> oracle (RUNTIME-0080-RR-0…RR-4, §4A); §0.2 is demonstrated doctrine, not yet default
> `SimSession` wiring. Bounded by §0.6: parking specified depth is honest only as a
> recorded, approved Deviation — never a silent flat proxy.

**D-2 — §0.5 two-layer harness sub-bullets: ~16 lines → 3.** Replace the "Two layers —
fixed base + rung-local" sub-bullet block with:

> Fixed base = 4–6 durable links (always: §0 + the track's one canonical design file). A
> handoff may add ≤3 rung-local links it directly consumes; rung-local links are ephemeral
> and never accrete into the base — promote durable ones into the canonical design file.

Rules 1–3 headlines and the six-line checklist stay verbatim.

**D-3 — §0.6 preamble: trim mandate narrative to one sentence.** Keep: "Added 2026-06-07 by
design authority on direct product mandate after a specified recursive structure was
silently flattened to a flat proxy and closed as PASS." Cut the rest of the blockquote.
All five numbered bindings stay verbatim.

**D-4 — §1 design lineage: 45 lines → 9-line table.** The section is self-declared
non-binding archaeology. Replace prose with:

| Version | Archived file | One-line contribution |
|---|---|---|
| v4–v6.5 | `archive/superseded_design/design_v4..v6.5` | tree, GPU pipeline, economy formation |
| v7/v7.6 | `design_v7.md` | the architectural floor: recursive SimThing + AccumulatorOp |
| v7.7 | `design_v7_7.md` | closed baseline amendment |
| v7.8 | `design_v7_8.md` | §2.6 non-negotiables origin |
| 0.0.7.9 | `design_v7_9_…` | mobility/transfer substrate, proven + parked; anti-loop origin |
| 0.0.8.0 | `design_0_0_8_0.md` | consumer-pulled redirection; §0.6 author |

Keep §1.1 (the consumer-pulled lesson) at 4 lines.

**D-5 — §2.4 second paragraph: ~16 lines → 4 + pointer.** The substrate-vocabulary gate is
stated verbatim in `invariants.md` (EML Gadget section). Replace with:

> The "no new opcode" rule binds the gadget layer only. Extending the generic interpreter
> (new semantic-free `EvalEML` opcode / combine fn / kernel) is a **Tier-2 gate, not a
> prohibition**, under the §2.3 conditions; binding text lives in `invariants.md` → EML
> Gadget Library. Rung "no new op/WGSL" stop-lines are scheduling hygiene, narrowable by
> design authority to that gate.

**D-6 — §2.5 fourth bullet (retired boilerplate): ~10 lines → 2.**

> **No per-scenario `Gate`/`Surface`/`ForbiddenRequests` boilerplate (retired 2026-06-02).**
> Standing prohibitions live once (gating §2 + `invariants.md`); a scenario's evidence is
> its reduction, never a forbidden-flag matrix.

**D-7 — §4/§4A closure record: ~55 lines → ~18.** Keep: the two CLOSED verdicts (E-11B,
M-4) at 2 lines each; the RR ladder diagram verbatim; one closure line: "RR-4 ACCEPTED/
CLOSED 2026-06-07 — integrated recursive 100-tick GPU rehearsal, bit-exact vs RR-0 oracle;
full Scope Ledger and non-claims in [`tests/runtime_0080_rr_4_results.md`]." Cut: the
non-claims paragraph and open-findings paragraph (they live verbatim in the report).

### File: `docs/invariants.md`

**I-1 — Specification Fidelity preamble: 7-line blockquote → 2 lines.** Keep the five rows;
tighten each row's "Enforced by" text to ≤30 words (no substance change — strip the
restated examples and cross-references already present in §0.6).

**I-2 — Scenario Proof: keep as-is** (one row, already tight).

**I-3 — JIT Kernel Registry: 13 rows → 6; biggest single cut (~55 lines).** The artifact
narratives (FNV hash, entry point, bit-IO domain, candidate genealogy E3/D/C/f64, release
attribution) move behind one pointer. Proposed replacement section:

| Rule | Meaning |
|---|---|
| No semantic WGSL; JIT kernels are semantic-free straight-line shaders | admission rejects semantic names |
| `ProductionCandidatePreview` is default-off | no default SimSession wiring; test/fixture invocation only until a named gate |
| Exact authority requires a pinned fixed-point chain + artifact-backed proven sqrt | The whole arithmetic chain must be pinned fixed-point (Q16.16 class) and any sqrt/mag stage the exhaustively-proven, hash-pinned artifact admitted through descriptor admission. f32 magnitude/weighting/scoring is `ApproximateDiagnostic` and may not feed exact state. Artifact identity, hashes, domains, and proofs: `workshop/sqrt_candidates.md` + descriptor admission. |
| Approximate outputs never feed exact inputs | rejected at admission and execution gates |
| GPU atomic event compaction = exact count + unordered membership, never ordering | under declared capacity/overflow contracts; ordering needs separate proof |
| FIELD_POLICY closure posture | no CPU planner/urgency/commitment; no scheduler/cache/default wiring; no economy→mapping bridge; `simthing-sim` stays semantic-free; proposals route only through accepted substrates (RF allocator, Threshold+EmitEvent→BoundaryRequest, own columns) |

**I-4 — Mapping section: keep every rule, strip inline evidence (~25 lines).** Each row
loses its test-report citations and historical asides (e.g. the embedded "Admission
rejection is a guardrail" narrative duplicating §2.1; the M-5 design-note pointers repeated
in three rows become one pointer line under the table). No rule is removed or weakened.

**I-5 — EML Gadget section: strip dates/review links and ladder history (~15 lines).**
"Temporal gadgets (velocity/EMA/acceleration/hysteresis/decay) are admitted per the landed
EML-GADGET-2 slices" replaces the 2A→2E dated ladder narrative. The bounded-feedback
contract row stays verbatim — it is Movement-Front P3 for formulas.

**I-6 — Boundary resolution: 6 rows → 4 (~12 lines).** Merge the two naming rows (the
product-preference narrative appears in both); keep no-DailyResolutionBoundary,
pause-is-host-layer, CPU-consumes-never-recomputes, discrete-banking≠continuous-flow.

**I-7 — Tail pedagogy (~35 lines).** Delete "What These Invariants Buy" (now lives as
`simthing_core_design.md` §3's rationale). Compress "The Proof Test" to 5 lines — it names
a real regression guard (`custom_layout_ethics_axis`) and earns that much.

### File: `docs/agents.md`

**A-1 — wholesale replacement (~990 lines → ~120).** The current file is a v4-era briefing:
Week-2/Week-3 crate states, an open "decision required" for the long-decided FMA question,
and shader sketches for deleted legacy passes. Archive it verbatim to
`docs/archive/superseded_design/agents_v4_briefing.md` and replace with a pointer brief:

1. **Read order (mandatory):** `simthing_core_design.md` → `invariants.md` →
   `design_0_0_8_1.md` (§0 + §2) → the track ledger row + the one test report for the slice
   being touched. Nothing else by default.
2. **Repository layout** (current crate list, one line each).
3. **How to run tests** (kept from the current file, verified commands).
4. **Code style notes** (kept).
5. **What this file is not:** implementation state (the status table owns it), design
   rationale (the core design owns it), history (the archive owns it).

This is the single largest token recovery in the proposal (~5,500 words) and removes the
only file that actively misinforms.

---

## 5. Flagged follow-on (not in this proposal)

**TRIM-NEXT-1 — ledger archival discipline.** `design_0_0_8_0_consumer_pulled_production_track.md`
is 11,500 words because closed rungs retain full narrative. Propose (separately): on CLOSE,
a rung collapses to one status row + report link; narrative moves to
`archive/production_paths/`. Same treatment for the `mapping_current_guidance.md`
forward-horizon blockquote, which has accreted into a wall.

---

## 6. What is deliberately NOT cut

CPU-oracle parity; Scenario Proof; Deviation Records + Scope Ledgers; the §0.6 five
bindings; the §0.5 six-line checklist; opt-in/default-off; the two-layer guardrail
placement; the bounded-feedback contract; every Mapping rule; the prohibition lists.
Rigor stays at full strength — it just stops being narrated.
