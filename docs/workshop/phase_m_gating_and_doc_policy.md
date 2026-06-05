# Phase M — Gating & Documentation Policy

**Status:** **Constitutional (V7.7 §5, promoted 2026-05-29).** Binding governance policy; this file
is the operational detail of `docs/design_v7_7.md` §5 "Gating & documentation governance."
**Purpose:** Keep drift discipline **cheap and binding**, and stop the doc-ceremony treadmill that
was burning tokens and looping the implementation agent. Read this **before** picking up any Phase M
slice — it tells you which lane a change is in and exactly how much documentation it needs.

The early Phase M gating cadence (separate design-review → acceptance → parking packet → R-series
hygiene, plus updates to 6–8 narrative docs) was correct when designs were genuinely uncertain
(atlas isolation, temporal-memory contract, bounded feedback). Those reviews resolved real
questions. It has since overshot into ceremony for **settled, within-design** slices. This policy
ends that.

---

## 1. The two lanes

Classify every change before starting. If unsure which lane, default to Tier 2.

### Tier 1 — Fast lane (ship directly, one PR)

A change is fast-lane if **all** hold:
- it is **within an already-accepted design** (the design note / acceptance memo exists);
- it is **generic substrate** — no semantic WGSL, no `simthing-sim` map/Gadget/Personality awareness;
- it is **opt-in / default-off** — changes no production default, wires nothing into the default `SimSession` pass graph;
- where it touches compute, it has **CPU-oracle parity** (bit-exact for `ExactDeterministic`);
- it is **reversible** (no irreversible architecture commitment).

**Fast-lane shipping cost (this is the whole ceremony):**
1. One implementation PR.
2. One test report under `docs/tests/` with the standard posture attestation (§3).
3. One status-table row update (§4).

**Fast lane does NOT require:** a separate design-review memo, an acceptance memo, a parking packet,
a consolidated review packet, or an R-series hygiene pass. Opus review, if wanted, is **post-hoc**
and does not block the merge.

Examples: `M-5A-gradient` (within the accepted gradient design), `M-5B-gradient`, additional
EML gadgets within the accepted gadget library, additional admission rules, additional oracle cases.

### Tier 2 — Gated (full design-review → acceptance → implementation)

A change is gated if **any** hold:
- it touches a **binding invariant** in `docs/invariants.md` (e.g. relaxing a ban);
- it introduces **default-on** behavior or default `SimSession` mapping/pass-graph wiring;
- it is **new architecture / irreversible**, or has a **genuinely open design question**;
- it is on the **standing prohibition list** (§2).

Tier 2 keeps the full cadence: a design review, an Opus/product acceptance memo, then implementation.
This is where the real safety lives; do not shortcut it.

---

## 2. Standing prohibition list (always Tier 2 or banned outright)

These are unchanged and binding. Touching any of them is never fast-lane:
- semantic WGSL; map/faction/AI branching in any shader
- `simthing-sim` map/Gadget/Personality/Memory awareness
- atlas / M-4A implementation; `request_atlas_batching` stays rejected at admission until a §11-gate PR
- production economy→mapping runtime bridge (economy→FIELD_POLICY stays `tests/support` fixture-only)
- default-on mapping execution / default `SimSession` mapping wiring
- Resource Flow E-11 default-on
- new EML opcode (incl. transcendental / `sqrt`); dual-output gradient kernel; L1 cross-field coupling
- source-mask / behavioral source policy (the separate `M-5` source-identity track)
- dense per-cell temporal memory; CPU-side AI planner / CPU urgency computation
- `DailyResolutionBoundary`; day/calendar/pause semantics in `simthing-sim`
- bounded-feedback contract relaxation (recurrent gadgets keep decay<1 and/or clamp)

A fast-lane PR that discovers it needs any of these must **stop and escalate to Tier 2** — do not
implement around the gate.

---

## 3. Documentation discipline (the token-burn fix)

- **State posture once, not eight times.** The standing "no semantic WGSL / no default wiring /
  defaults unchanged / `simthing-sim` map-free" posture is asserted **once** in the PR's test report
  (a single posture line), not duplicated across the production plan, guidance, state, design note,
  todo, and worklog. The binding rules live in `docs/invariants.md`; do not restate them per slice.
- **Active docs are compact; narrative lives in the worklog.** `mapping_current_guidance.md` and
  `workshop_current_state.md` carry a **compact status table** (§4) and pointers — not an accumulating
  per-slice narrative log. Per-slice history is a one-line worklog entry (`docs/worklog.md`,
  append-only). Verbose per-slice blocks in the active docs are superseded historical log; collapse
  them when you touch the file.
- **No packet proliferation.** Do not create a parking packet for a slice you intend to accept —
  fold it into the single PR. Do not create a consolidated review packet that merely restates test
  reports. One test report per slice is the record.
- **No reflexive R-series.** Do not spawn R1/R2/R3 hygiene passes by default. An R-pass is justified
  only by a **found defect** with a one-line reason. "Tidying wording" is not a defect.
- **Stop rule (anti-loop).** If you are about to write a **third** meta-document for one slice
  (e.g. packet + review + hygiene), stop — you are in the ceremony loop. Ship the code + one test
  report + one status row instead.

---

## 4. The compact status table (single source of truth for "where are we")

`mapping_current_guidance.md` and `workshop_current_state.md` each carry one short table:

| Slice | Lane | Status | Evidence |
|---|---|---|---|

`Status ∈ {approved-for-impl, in-progress, landed, accepted, deferred}`. One row per slice; update
the row in place, don't append a narrative block. "Evidence" links the single test report.

---

## 5. What is explicitly retained (drift protection is NOT relaxed)

- `docs/invariants.md` stays binding; **any change to it is Tier 2.**
- Test reports stay mandatory (oracle parity + one posture attestation line) — that is the real
  safety, and it is cheap.
- The standing prohibition list (§2) is unchanged.
- Tier 2 cadence is unchanged for anything genuinely uncertain or constitutional.

The trade is precise: we remove **redundant narration and redundant gates**, not **enforcement**.

---

## 6. The proven-capability stop rule (anti-recombination loop)

**Constitutional, added by design authority 2026-06-02.** Sibling of `invariants.md`'s "one principle
per class — no per-slice accretion": this is its runtime/test analog. It exists because a landed,
green capability kept attracting ever-more-elaborate *recombination* work (e.g. a proven GPU kernel
re-wrapped as soak → replay-soak → multi-frame-stream-soak → throughput-summary → budget-assertions-
over-the-summary), each shipping a fixture + a report but **no new code path and no new failure
mode**. That is a hygiene loop wearing a test-fixture costume.

**A capability is *reasonably proven* once** (a) its substrate floor is green (the determinism /
parity / invariants it must hold), and (b) **one** representative scale/soak + parity run is green.

**Once proven, further variants that only recombine the proven primitive are NOT authorized** —
additional soaks, replays, multi-frame/stream/batch permutations, Nth scale runs, accounting-over-
accounting, or assertions-over-summaries. The **only** authorized next artifacts are one of:

1. **Close a dangling path** — mark the capability complete, reconcile the track / guidance / worklog
   to reality, and stop.
2. **Escalate a design direction** — a one-paragraph proposal to design authority for the next
   *distinct* path/gate. If that next path is a Tier-2 threshold (default-on, default schedule /
   pass-graph wiring, gameplay surface, semantic WGSL, invariant change, closed-ladder reopen),
   **stop and wait for the gate** — do not fill the wait with more recombination variants.

**The test before adding the N+1th variant:** *does it exercise a code path or failure mode the proven
set does not?* If no, it is a recombination loop — close the path or escalate instead. **Proven once,
not proven N times.** An agent that has shipped two same-shape recombination variants in a row is in
the loop and must switch to (1) or (2).
