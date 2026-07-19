# 0.0.8.4.8.4.2 — RF Harness (RF drift interventions)

> **Status: STUB / AUTHORED — NOT OPENED.** Owner-directed 2026-07-18. This workplan is being
> developed while the 0.0.8.6 TP chain finishes (RF-5 → 12.10); it opens (via
> `gen_orientation.sh --open`) only **after** TP is done and the Owner commits the final plan.
> More RF drift interventions will be added before commitment — the ladder below is the first
> intervention only, not the full plan. Do not flip the pointer; do not dispatch from this doc.

## 1. Root cause this track closes

The RF sub-track (RF-1…RF-4, 2026-07-17/18) proved the recursive Arena RF substrate, but the
DA audit that followed found the **guard layer around the GPU-resident row/column ontology is
eroding by exclusion accumulation** while the typed fix sits in a backlog:

- The ontology (design §5 / OC-K-COLUMN-ROLE-0): persistent state is a row-major
  `slots × columns` matrix; an authoritative value is the complete cell `(SlotIndex, ColumnIndex)`;
  columns are obtained ONLY through the semantic role pathway
  (`SubFieldRole → PropertyLayout::offset_of → col_for_role → ColumnIndex`); every modifier is an
  overlay applied by the unified kernel — no CPU-side modification channel, no second ledger.
- The detector for role-pathway bypass, `COLUMN-INDEX-MINT` (HEURISTIC on `ColumnIndex::new`),
  has accumulated **18 file exclusions** on master, growing by *peer-citation* (RF-5 attempted
  #19, `need_weight_profile.rs`, citing "gated_rates peer" — the very file whose install-time
  CPU row-mirror the orchestrator's ontology correction condemned, #1414 comment 5013140700).
- `ColumnIndex` has exactly one door: a bare `pub fn new(raw: usize)` in `simthing-core`
  (**170 call sites** across 6 crates: 121 driver, 35 kernel, 8 core, 2 spec/sim/clausething).
  The scan's own promotion-blocker names the fix: admission-gate the constructor (OC-K2.1).

Erosion mechanics mirror the HC-track "silent exclusion token" defect: a heuristic detector
whose exclusion list grows one defensible-looking row at a time until the law it guards is prose.

## 2. Interim guard (in force while this plan is a stub)

**DA standing order (2026-07-18):** `COLUMN-INDEX-MINT` exclusion additions are **frozen** —
any new exclusion row requires explicit DA sign-off; *peer-citation is not a valid
justification*; RF-5's `need_weight_profile` row must be removed unless the ontology-conformant
rework still genuinely needs it. Orchestrator enforces at review; `scripts/ci/**` already routes
DA-RESERVE(gate-wiring), so no exclusion can merge orchestrator-clearable.

## 3. PR ladder (first intervention; Owner will extend before commitment)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| RFH-1 | `RF-COLUMN-ADMISSION-GATE-0` | **Type boundary (= OC-K2.1a; kernel lane, Owner-gated K2 lineage).** Design + land the `ColumnIndex` constructor taxonomy in `simthing-core`: `new` goes non-public; legal doors = layout-derived paths (`PropertyLayout::offset_of` / `col_for_role` / arena-layout ranges) + two doc-fenced choke points the honest exclusion families need — a GPU round-trip constructor (adapter family re-materializing `gpu.*_col`) and a fenced raw door for oracle/rehearsal code. Retarget `COLUMN-INDEX-MINT` from `ColumnIndex::new` to the choke-point tokens (one watched door, not 18 excused files). Rides along: DA-gate exclusion edits on the scan (mechanize §2). | TODO — small diff, high judgment; DA-reserve review of the taxonomy. Falsifier: post-rung, the scan proves every remaining `::new` is inside the fence. | DA-reserve · Frontier |
| RFH-2 | `RF-COLUMN-MINT-MIGRATE-0` | **Migration sweep (= OC-K2.1b).** Mechanically move all ~170 call sites onto RFH-1's doors; **delete the entire exclusion list**; retire/demote `COLUMN-INDEX-MINT` per its own promotion-blocker. RF-3-style sweep — wide, boring, safe only after RFH-1's doors exist. | TODO — falsifier: exclusion list empty; tripwire retired; full build + RF batteries green. | Std |
| RFH-… | *(reserved)* | Further RF drift interventions — Owner to add before the plan is committed (candidates from the 2026-07-18 audit: RF-2A GPU→CPU→GPU adapter residency wobble; fail-open GPU-skip sweep beyond ct_2a/ct_2c; kernel-columns anchor domain not triggering on driver paths). | — | — |

## 4. Open conditions

1. 0.0.8.6 TP chain closed: RF-5 merged conformant + 12.10 `TP-EMERGENT-TENSION-PROOF-0` graduated.
2. Owner commits the full intervention list (replaces the reserved rows above).
3. Track opens via `gen_orientation.sh --open docs/design_0_0_8_4_8_4_2_RF_harness.md`
   (pointer-lifecycle gate applies; 0.0.8.6 must be CLOSED or PARKED first).

Sequencing note: RFH-1/RFH-2 touch the same driver files RF-5 edits — this ordering is
load-bearing, not ceremonial.
