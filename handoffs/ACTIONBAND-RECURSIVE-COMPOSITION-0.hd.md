---
rung: ACTIONBAND-RECURSIVE-COMPOSITION-0
kind: rung
track: 0.0.8.7
base_sha: afe6c7db23a8bc44f45635b8d914d29b42b257a6
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "DA 5233518871 graduated 7.2 (#1699 @ 6b2792b7; stamp #1702 @ afe6c7db) and cleared 7.3 to author. Frozen 7.1/7.2 authority stands. The 7.2 carry baseline is 4096 ns / 32 bytes at cardinality 1; 7.3 owns the cardinality curve and §15.6 remedy only if scaling demonstrates need."
surfaces: ["crates/simthing-core/src", "crates/simthing-spec/src", "crates/simthing-kernel/src", "crates/simthing-gpu/src", "crates/simthing-driver/src", "crates/simthing-sim/src", "crates/simthing-spec/tests", "crates/simthing-kernel/tests", "crates/simthing-driver/tests", "crates/simthing-sim/tests", "docs/tests", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md"]
forbidden: ["widen/duplicate graduated 7.1 admission/dependency/crossing authority or graduated 7.2 current-next/structural/world-value boundaries", "CPU child scheduling/evaluation, pointer recursion, same-generation convergence, or semantic ordering from append/atomic/physical-row order", "ActionBand-local RF/CostBand/conservation/clearing/holding/transaction authority; pre-8.x persistent scarce holding or atomic common-depth commitment", "7.4 movement, 7.5 semantic shadow/readback, 8.x, StemThing-B, Vector CostBand, or successor work", "raw shared-value writes/binding-7 widening, gate-code edits, weakened guards/ledgers, or duplicate proof for type/admission-guaranteed conditions"]
required_checks: ["fresh coding ORIENT + rendered HD receipt + all required anchor ACKs", "consume frozen dependency spans/caps; child-next at t executes at t+1; terminal child collapses; no runtime semantic child construction", "prove concurrent siblings deterministic under row/append perturbation; inline trivial requirements and materialize only admitted independent-lifecycle children", "compose state predicates with existing native RF claims and scalar CostBand only; no ActionBand-local resource/sink/clearing authority", "measure carry across increasing active cardinalities with rows/bytes/method; apply §15.6 compact-list remedy only if scaling demonstrates need and preserves 7.2 current-next law, otherwise STOP", "run focused 7.3 plus inherited 7.1/7.2 and full affected batteries + hosted Doctrine; only 7.3 becomes PROBATION"]
stop_conditions: ["requires widening 7.1/7.2, runtime child construction, unbounded allocation, or a second dependency registry", "requires CPU scheduling, same-generation convergence, StateCurrent writes, or per-row current-next authority selection", "native RF/CostBand needs raw writes/new local conservation/clearing/holding authority or scarce multi-arena semantics", "performance repair weakens 7.2 law, or completion requires successor/gate-code work"]
---
## BUILD
- Implement only 7.3: pre-admitted child activation, stable dependency spans, concurrent siblings, collapse, generation pacing, and trivial-child inlining on the frozen 7.1/7.2 substrate.
- Compose ordinary state predicates with existing RF claims and scalar CostBand; add no ActionBand-local resource/sink/clearing authority.
- Measure the 7.2 carry over meaningful active cardinalities before any §15.6 layout remedy.
## FENCES
- Parent writes child-next at t; child executes at t+1 or later after the barrier/swap. No same-generation recursion or CPU child scheduler.
- Recursion stays physically flat/deterministic via admitted indices/spans and next-state bits, never pointer/append/atomic/physical-row semantic order.
- Multi-arena scarce holding/common-depth stays admission-fail/defer-closed; preserve all graduated 7.1/7.2 boundaries and fence 7.4+.
## EXIT-PROOF
- Parent→child-next→later child execution→ordinary consequence→child collapse→later parent resolution is proven across generations.
- Siblings progress concurrently and remain bit-identical under row/append perturbation; semantic-order mutant is RED/unconstructible.
- Trivial requirement inlines without a child row; independent lifecycle uses only pre-admitted child span; native state+RF+scalar-CostBand multisource gate works.
- Report carry cardinality curve and any justified §15.6 remedy; return PROBATION with focused+inherited batteries and hosted artifacts. Coding does not `/clearance`, merge, move pointer, or start successors.
