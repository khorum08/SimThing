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
3. [`design_0_0_8_1.md`](design_0_0_8_1.md) §0 + §2 — carry-forward doctrine and operating
   mechanics (tiers, gating, anti-loop, non-negotiables).
4. The live status ledger row for your track
   ([`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md))
   and **the one test report for the slice you are touching.**

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
cargo test --workspace
cargo fmt --all -- --check
```

The full workspace suite must pass with **zero warnings** before any commit. GPU tests skip
themselves cleanly when no adapter is available (`try_gpu()` returns `None`) — CI without a
GPU still completes. One ignored timing diagnostic runs with `cargo test -- --ignored`.

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
