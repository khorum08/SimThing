# SimThing — Agent Briefing

> **This file is a router, not a knowledge base.** It tells you what to read and how to run
> the project — nothing else. It deliberately contains **no implementation state** (the
> production-track status ledger owns that), **no design rationale** (`simthing_core_design.md`
> owns that), and **no history** (`docs/archive/` owns that). The previous 990-line v4-era
> briefing is archived verbatim at `archive/superseded_design/agents_v4_briefing.md`; do not
> treat it as current — its crate states, open decisions, and shader sketches are obsolete.

---

## Read order (mandatory, in this order)

1. [`simthing_core_design.md`](simthing_core_design.md) — **the paradigm. Hold it in context
   for your entire task.** The recursive GPU-resident SimThing tree, SimProperty→Value,
   resource flow arenas, overlays, the Movement-Front automaton, the EML opcode discipline,
   and the drift-detector litmus tests.
2. [`invariants.md`](invariants.md) — the binding rule tables. A violation is a compile
   error, a test failure, or a voided closure.
3. [`design_0_0_8_3.md`](design_0_0_8_3.md) — **the ACTIVE constitution** (carry-forward
   transient constitution §0 + the ClauseThing-vertical closeout addendum §A). It supersedes
   `design_0_0_8_1.md`; that predecessor stays in place and §B of 0.0.8.3 incorporates its
   operating mechanics (§2), parked inventory (§3), and closed questions (§4) by reference.
4. The live status ledger row for your track
   ([`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md))
   and **the one test report for the slice you are touching.**
5. **If your task is in the ClauseScript / MapThing / MapGenerator vertical** (now CLOSED):
   start at [`clausething/ClauseThingDoc.md`](clausething/ClauseThingDoc.md) (clearinghouse:
   concepts/practices/APIs) and [`adr/ClauseThingADR.md`](adr/ClauseThingADR.md) (decisions).
   The production-track ladders are archived under `archive/closed_production/`.
6. **MANDATORY for any spatial task** — [`stead_spatial_contract.md`](stead_spatial_contract.md)
   is **required reading** (in addition to core §0/§7) before you touch any of: MapGen lattice,
   MapGeneratorCLI, Location grids / `grid_metadata` placements, the Movement-Front engine, STEAD,
   heatmaps / falloff / fronts, PALMA, Gu-Yang / SaturatingFlux, Resource Flow or Accumulator
   arenas whose participants are gridcell `Location`s, structural vs render coordinates, layout
   vs execution-profile admission, or field visualization. It is short, normative, and enforced by
   `crates/simthing-clausething/tests/stead_spatial_contract_guards.rs`. STEAD/Mapping has drifted
   catastrophically three times; this contract exists to make that impossible.

Nothing else by default. If your handoff cites additional rung-local links (≤3), read those
too. If something seems missing, search the repo before re-deriving from priors —
**source code is ground truth over any document.**

## Repository layout

```
crates/
  simthing-core/      Core types: SimThing {properties, overlays, children}, SimProperty,
                      PropertyLayout/SubFieldSpec, PropertyValue, Overlay, ids. No GPU, no IO.
  simthing-gpu/       wgpu device/queue, persistent buffers, the AccumulatorOp WGSL kernel,
                      StructuredFieldStencilOp, EvalEML interpreter. Semantic-free.
  simthing-sim/       Tick/boundary orchestration, BoundaryProtocol, structural mutation,
                      threshold registry, replay. Semantic-/map-/arena-/gadget-free, permanently.
  simthing-driver/    Session assembly and execution paths; compiles specs/registries into
                      flat AccumulatorOp registrations before upload. Production-path tests.
  simthing-spec/      Designer/RON admission layer: scenario specs, guardrail diagnostics,
                      gadget registry/compiler, CLAUSE-SPEC. Semantics live HERE and compile away.
  simthing-feeder/    Bridge between authoritative gameplay state and the GPU pipeline.
  simthing-workshop/  Sandbox probes and measurement harnesses. Never a production dependency.
docs/                 Active docs (this folder). docs/archive/ = historical record only.
scenarios/            Authored RON scenarios.
```

## How to run tests

```
cd C:\Users\mvorm\SimThing
cargo fmt --all -- --check
```

**Default rule:** do **not** run `cargo test --workspace` unless the rung absolutely requires
full-workspace proof. For isolated parser/doc/license/report work, run **touched-crate tests**
plus inspection only.

**Targeted examples:**

```
cargo test -p simthing-clausething
cargo test -p simthing-driver mobility_gpu_kernel7
cargo test -p simthing-driver mobility_gpu_kernel8
cargo test -p simthing-driver mobility_gpu_kernel9
cargo test -p simthing-driver mobility_gpu_kernel10
cargo test -p simthing-driver mobility_gpu_kernel11
```

**Expensive mobility GPU replay gates (kernel7–kernel11):** conformance/replay/accounting/budget
batteries are `#[ignore]` and run only for mobility GPU replay/accounting/budget/dispatch/shader
changes:

```
cargo test -p simthing-driver mobility_gpu_kernel10 -- --ignored
```

Full workspace testing is not routine proof until a future TEST-PARING-0 classifies the corpus.
When a rung does require workspace scope, GPU tests skip cleanly when no adapter is available
(`try_gpu()` returns `None`). Other ignored diagnostics (e.g. timing capture, exhaustive sweeps)
run with `cargo test -- --ignored` only when explicitly needed.

Run a recorded session (record needs a GPU adapter; replay is CPU-only):

```
cargo run -p simthing-driver -- record --scenario scenarios/rebellion_demo.ron --out demo.replay.ldjson
cargo run -p simthing-driver -- replay --in demo.replay.ldjson
```

Named guard tests you must never weaken:

- `custom_layout_ethics_axis` — proof the property-layout generalization works beyond the
  standard amount/velocity/intensity layout. New layout capability ⇒ add a test in this pattern.
- `pass3_overlay_matches_evaluator` — proof GPU transform application stays bit-exact with
  the CPU `Evaluator` across all `TransformOp` variants. New transform variant ⇒ extend it
  with a parity assertion.

## Working rules (the short version — binding text is in the read order above)

- Classify your change first: Tier-1 fast lane ships as one PR + one test report + one
  status row; Tier-2 keeps the full gated cadence
  (`workshop/phase_m_gating_and_doc_policy.md`).
- Everything is opt-in / default-off; no default `SimSession` wiring without a named gate.
- Exact claims carry bit-exact CPU-oracle parity (`f32::to_bits()`).
- A scenario is proven only through a real reduction — never admission-rejection tests plus
  a CPU math loop.
- Stop-and-escalate on any stop-condition in your handoff or any conflict with
  `simthing_core_design.md` §9. Do not rationalize; do not re-derive architecture.

## Code style notes

- No comments explaining what the code does. Names should do that.
- Comments only for non-obvious WHY: a hidden constraint, a specific invariant reference,
  a workaround for a wgpu behavior, a simulation design decision.
- Reference invariants when a comment explains a rule: `// invariant: velocity pin`.
- Tests live in the module they test (`#[cfg(test)] mod tests` at the bottom of each file).
- New types go in the module that owns them. Don't create new files for small additions.
- No `unwrap()` in non-test code without a comment explaining why the None case is impossible.
