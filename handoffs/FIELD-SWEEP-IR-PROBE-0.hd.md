---
rung: FIELD-SWEEP-IR-PROBE-0
kind: rung
track: 0.0.8.7
base_sha: 6b2420c4
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(admitted-scope-router-gap)
owner_notes: "Owner-approved 5.x amendment #1482; DA ruling 5137572177. This is a disposable measurement rung, not an engine landing. The probe is born mortal: workshop-homed, birth-track expiry inherited, dsu_survivals=0, no permanent-residue shield or fabricated downstream consumer. A negative performance result is admissible and routes 5.5 to specialization/JIT with the IR retained as specification. 2026-08-11 remains sovereign."
surfaces: ["crates/simthing-workshop/src", "crates/simthing-workshop/tests", "docs/tests/field_sweep_ir_probe_0_results.md", "docs/tests/current_evidence_index.md", "scripts/ci/test_inventory.tsv", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md"]
forbidden: ["production source outside simthing-workshop", "game-corpus or scenarios reads/edits", ".github or scripts/ci gate changes", "production WGSL/opcodes/FieldSweepRegistration/adjacency APIs/resource classes", "semiring or algebra enum / field-kind tag", "5.5-5.8 or Phase 6+ work"]
required_checks: ["cargo check -p simthing-workshop", "adapter-pinned field_sweep_ir_probe_0 parity + measurement run", "test_inventory_drift_check.sh", "test_lifecycle_expiry_check.sh --scheduled", "detachability_check.sh (+ selftest)", "agent_scan.sh", "doctrine-scan", "clearance"]
stop_conditions: ["stale-orient-receipt", "probe cannot remain wholly workshop-homed", "N4 parity is not bit-exact", "matched occupancy or required counter surface cannot be measured truthfully", "N8 requires an engine adjacency change", "any proof needs the game corpus"]
---
## BUILD
- Build one private/test-only workshop field-sweep IR probe: minimal target/neighbor context map program + deterministic linear fold program over an existing gather. It may consume existing public engine/GPU doors but creates no production export, registration, opcode, kernel authority, or reverse dependency.
- Exercise N4 against the existing bespoke PALMA and Gu-Yang paths with identical inline synthetic state, canonical neighbor order, coefficients, iteration count, output columns, and measured occupancy. Include the pre-named fallback forms `MIN × INPUT_LIST` and `PRODUCT × INPUT_LIST + banded flux`; do not encode field identity as an enum/tag/branch.
- Exercise N8 only through a throwaway workshop-owned gather. Do not add N8 to engine topology/admission; 5.6 owns that landing.
- Publish per adapter/case: adapter/backend, adjacency kind, theater size, degree distribution, nodes/edge, actual max stack depth, column reads/edge, resource class, matched occupancy, warmup/sample counts, time/sweep, edges/s, median, worst, and available stall/memory counters. Missing required counters is a STOP, not permission to infer the memory-shadow claim from timing alone.
- Re-derive current EML cap/stack facts from the live implementation and measured programs; report configured limits separately from observed use. Do not amend caps or resource classes in this rung.
- Land one signal-only results doc with raw rows, aggregation method, threshold verdict, N8 cliff, and next-route decision. Threshold failure completes the probe honestly as `ROUTE-SPECIALIZATION/JIT`; it never authorizes abandonment of the IR or preservation of bespoke kernels as the final architecture.
- Row every new test as birth track `0.0.8.7`, `dsu_survivals=0`; add no permanent-residue or renewal claim. Stamp 5.4 PROBATION only; authoritative pointer stays on 5.4 until DA graduation.
## FENCES
- The Invariant Set remains the substrate proof surface: reuse existing parity/oracle authority; add no permanent per-rung referee unless it names and falsifies a genuinely new invariant. Emergence/performance observations are demonstrations and measurements, not new gates.
- Native ingestion is fixture-witnessed; the game corpus is an external asset. The probe and every proof use inline/synthetic data and never read `scenarios/terran_pirate_galaxy.clause` or TP semantics.
- Corpus Boundary + Detachability hold: no engine production source change, no engine→workshop dependency, production coupling 0, proof coupling ceiling 0.
- The probe source, tests, and measurement artifacts are disposable by default and may not acquire a compatibility promise, public API consumer, or lifecycle exemption.
## EXIT-PROOF
- Absolute bit-exact N4 parity for every measured PALMA/Gu-Yang case before timing is admitted.
- At matched measured occupancy, generic N4 median is `≤1.25×` bespoke and every supported-adapter worst is `≤1.5×`, with counter evidence supporting or refuting the memory-shadow claim; otherwise verdict is the explicit specialization/JIT route.
- N8 cliff is located and reported without engine N8 changes; current EML node/stack/cap facts are reproduced from source + measurements.
- Diff proves workshop-only implementation, zero corpus reads, zero production/proof coupling, lifecycle/inventory/homing green, and no new permanent referee.
- Adapter-pinned probe run, workspace-compatible build, agent/doctrine scans, exact-head clearance, one results doc/index row, and 5.4 PROBATION posture are green; then STOP for orchestration/DA.
