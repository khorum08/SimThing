# RUNTIME-0080-RR-OPEN-0 — recursive galaxy→system→planet-surface rehearsal

> **Status: OPEN (consumer track), 2026-06-07, design authority, on direct product mandate.**
> **Supersedes** the mis-scoped `RUNTIME-0080-M4A-OPEN-0` (parallel-theaters). This is the track that
> actually builds the specified recursive structure that the closed flat rehearsal proxied.
> **Governed by `invariants.md` → Specification Fidelity & Anti-Ceremony and constitution §0.6:**
> every rung that claims PASS carries a *Specified vs Implemented* Scope Ledger; no tier may be silently
> collapsed; the specified consumer must actually run; no hygiene theater.

## Fixed harness header (constitution §0.5)

Cite on every rung handoff, self-check the diff against the six base principles:
1. `design_0_0_8_0.md` §0 (carry-forward spine; **§0.6 Specification Fidelity is load-bearing here**).
2. This opening spec (canonical design file for the track).
3. `docs/invariants.md` → Specification Fidelity & Anti-Ceremony + Scenario Proof + Mapping.
4. `docs/scenarios/scenario_0080_2_dress_rehearsal_spec.md` (the structure of record — the spec that was flattened).
5. `crates/simthing-driver/src/atlas_0080_0.rs` (nested descend/ascend sparse residency to generalize) and `crates/simthing-driver/src/runtime_0080_0_r2.rs` (the proven per-tier 100-tick loop to reuse).

Established decisions / do-not-re-derive: everything is a SimThing (§0.1); allocation is recursive reduce-up/disburse-down (§0.2); all conflict is resource flow (§0.3); decisions are GPU threshold crossings, no CPU planner (§2.6); `simthing-sim` stays semantic-free; proven only through a real reduction; opt-in/default-off; CPU-oracle bit-exact parity.

## The horizon (what "done" means — the full specified structure)

A recursive dress rehearsal that runs, with **no tier collapse**:

```text
Galaxy starmap (20×20 gridcells)
└── Star system (Location, 10×10 subgrid)  ×13   ← one per occupied galactic cell
    ├── Starport (building, child of the system gridcell)
    └── Planet (Location, 10×10 surface)
        └── Planet surface (10×10 gridcells)
            ├── Factory district (building = a surface-cell SimThing)
            └── Pop cohort (building = a surface-cell SimThing; +labor/tick)
```

- Terran: 10 systems, each 1 planet → 1 factory + 1 pop cohort. Pirate: 3 systems, parallel economy.
- **Labor economy on the surface:** pop cohort emits labor → factory consumes labor → production →
  reduces **up** planet → system → galaxy → faction stockpile; disburses **down** (§0.2, one recursive
  mechanism, not a flat-star special case).
- Galactic-tier combat/movement/disruption/blockade reuse the **proven** R2 loop unchanged.
- All tiers GPU-resident under **nested sparse residency** (generalize `atlas_0080_0`): galaxy always
  resident; a system's 10×10 and its planet's 10×10 surface materialize only when active.
- Ticked the canonical 100 ticks, compared against a **recursive CPU oracle** (bit-exact parity), with
  an honest Scope Ledger. **No flattening; no proxy economy.**

M-4A (multi-theater sparse residency) is **consumed here** as the residency mechanism for the nested
hierarchy — this is the named consumer that the closure said M-4A required. It is not opened as
standalone parallel theaters.

## Rung ladder (smallest honest, spec-faithful units — consumer-pulled)

Each rung: opt-in/default-off; real reduction (Scenario Proof, not an oracle-only close); CPU-oracle
bit-exact parity where it touches compute; **Scope Ledger required at close**; fixed harness header;
no hygiene theater; no semantic WGSL; no `simthing-sim` map awareness; no default `SimSession` wiring.

- **RR-0 — recursive world model + CPU oracle.** Define the recursive SimThing containment above as real
  `SimThing` / property / overlay state (galaxy cells; 13 system 10×10 subgrids; planet 10×10 surfaces;
  pop-cohort + factory surface-cell SimThings; labor→production economy). Build the recursive CPU oracle
  (reduce-up / disburse-down across all tiers). **Oracle, explicitly labelled — not a proof of the GPU
  path.** Deliverable proves the *structure* exists and ticks coherently on CPU.
- **RR-1 — nested residency wiring.** Generalize `atlas_0080_0` descend/ascend sparse residency to host
  the RR-0 world: galaxy resident; descend-into-system materializes its 10×10; descend-into-planet
  materializes its surface 10×10. Prove only active theaters materialize (sparse), no cross-tier leakage
  (negative control), residency parity vs oracle.
- **RR-2 — planet-surface labor economy on GPU.** Run pop→factory labor/production on surface cells as
  resource-flow `AccumulatorOp` registrations, GPU-resident, bit-exact vs the RR-0 oracle. Replaces the
  flat galactic stockpile proxy with the real surface economy at the leaf tier.
- **RR-3 — recursive reduce-up / disburse-down across all tiers.** Surface→planet→system→galaxy→faction
  reduction and disbursement in the one recursive pass (§0.2). Bit-exact vs oracle.
- **RR-4 — integrated recursive 100-tick rehearsal (the horizon).** Galactic combat/movement/disruption
  (reuse R2 loop) + the planet-surface economy feeding production, all tiers resident under nested sparse
  residency, ticked 100 ticks, compared to the recursive CPU oracle. Honest Scope Ledger; **zero
  flattening**. This is the recursive analog of R2 and the actual fulfillment of the dress-rehearsal spec.

## What stays parked (until a rung concretely needs it)

- multi-faction economy generality beyond Terran/Pirate (`ECON-0080-MULTIFACTION`);
- default `SimSession` wiring; realtime loop; UI;
- new **semantic** GPU ops (generic semantic-free ops follow the §2.3 Tier-2 gate + CPU-oracle parity);
- pinned-number changes; SCENARIO-0080-2 reopen.

## Stop conditions (return to design authority)

Stop if a rung requires: a CPU planner masquerading as GPU authority; semantic WGSL tied to one fixture;
default session wiring; a pinned-number change; relaxing a binding invariant; or evidence that the
closed flat R2 loop must change to host the recursive tiers (that reopens RUNTIME-0080-0 — return first).
**A rung may not close by flattening a specified tier** (constitution §0.6 / Specification Fidelity).

## Per-rung deliverables

Create per rung: `crates/simthing-driver/src/runtime_0080_rr_<n>.rs`,
`crates/simthing-driver/tests/runtime_0080_rr_<n>.rs`,
`docs/tests/runtime_0080_rr_<n>_results.md` (with the required **Scope Ledger**).
Update: `crates/simthing-driver/src/lib.rs`, `docs/design_0_0_8_0_consumer_pulled_production_track.md`,
`docs/worklog.md`, `docs/workshop/mapping_current_guidance.md`. Do not edit `docs/invariants.md` unless
design authority opens a constitutional change. Delete scratch/log outputs.

## Required foreground commands (no shell redirection, no Tee-Object, no pipes)

```powershell
cargo test -p simthing-driver --test runtime_0080_rr_<n>
cargo test -p simthing-driver --test runtime_0080_0_r2
cargo test -p simthing-driver --test atlas_0080_0
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run
cargo test -p simthing-gpu
cargo build --workspace
cargo fmt --all -- --check
cargo check --workspace
```

## Report (every rung)

`docs/tests/runtime_0080_rr_<n>_results.md` MUST include: verdict; adapter; **Specified vs Implemented
Scope Ledger** (constitution §0.6 / invariant); real-reduction evidence (not oracle-only); CPU-oracle
parity; sparse-residency accounting (RR-1+); nested-tier coverage (no collapse); foreground commands and
results; scratch/log cleanup confirmation. A rung that cannot present the Scope Ledger does not close.
