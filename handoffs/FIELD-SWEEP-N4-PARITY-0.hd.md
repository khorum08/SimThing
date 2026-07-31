---
rung: FIELD-SWEEP-N4-PARITY-0
kind: rung
track: 0.0.8.7
base_sha: c0126aaf1966821bfcd1a4878c87ba97a9d54734
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-approved 5.x amendment #1482; DA graduation/routing ruling 5138782208. Rung 5.4 retired the N4 correctness risk: PALMA and Gu-Yang are bit-exact on CPU+GPU. Occupancy, stall counters, threshold adjudication, resource classes, and specialization/JIT are assigned to 5.7, not this rung. Frontier lane: Codex 5.6/Fable. The IR remains the specification; the 2026-08-11 reap deadline remains sovereign."
surfaces: ["crates/simthing-core/src", "crates/simthing-gpu/src", "crates/simthing-kernel/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "scripts/ci", "docs", "scripts/ci/test_inventory.tsv"]
forbidden: ["5.6 adjacency generators, N8/radius presets, or LinkGraph landing", "5.7 resource classes, occupancy/counter doors, performance threshold, specialization, or JIT", "5.8 projections or Phase 6+ work", "semiring/algebra enum, field-kind tag, or operator-identity dispatch", "game-corpus or scenario edits", "new bespoke field-specific WGSL/kernel authority", "retirement or deletion of the existing seven bespoke stencil oracles", "expiry extension or permanent-residue exemptions unrelated to load-bearing 5.5 law"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "cargo test -p simthing-kernel", "cargo test -p simthing-gpu", "adapter-pinned full simthing-driver battery including PALMA and Gu-Yang N4 parity", "stead_spatial_contract_guards", "FIELD-SWEEP-SINGLE-PATH scan and planted selftests", "test_inventory_drift_check.sh", "test_lifecycle_expiry_check.sh --scheduled", "detachability_check.sh (+ selftest)", "agent_scan.sh and doctrine-scan", "orientation/anchor/doc-budget checks", "clearance"]
stop_conditions: ["stale-orient-receipt", "N4 authored-instance parity is not bit-exact", "the generic path requires a semiring/tag/operator switch", "conservative admission cannot bind an undirected-symmetry certificate", "canonical fold order cannot be proven without implementing 5.6", "the rung requires non-default resource classes or occupancy/counter work", "a new bespoke field kernel or corpus/referee edit is required", "scope-widening"]
---
## BUILD
- Land the production N4 field-sweep path as EML execution over an existing gather. Extend the edge context with `{target_slot, neighbor_slot, accumulator, edge_scalar, dt}` and add `TARGET_VALUE` / `NEIGHBOR_VALUE` under the EML growth law; do not encode field identity in the evaluator.
- Add `FieldSweepRegistration { adjacency, map_program, fold_program, identity_bits, post_program, FieldLawProof, CanonicalOrderProof, resource_class }`. The only admitted resource class in 5.5 is the existing legacy fixed-32-stack default; reject every other value until 5.7.
- Seal admission: conservative folds require an undirected-symmetry certificate for the authored adjacency; every registration carries canonical-order proof; execution uses a fixed linear neighbor fold and the authored N4 offset-table order exactly.
- Express PALMA and Gu-Yang as authored registrations through this single generic path while retaining their existing bespoke stencils as unmodified migration oracles. Reproduce both CPU and GPU outputs bit-exact on N4 before any status claim.
- Withdraw `SEMIRING-FIELD-TAGS-0` as an implementation direction: no semiring/algebra enum, field-kind tag, or operator-identity `match` anywhere in the sweep path. Co-evolve the P5 pillar, STEAD spatial contract §10, and `stead_spatial_contract_guards` in this PR.
- Land permanent `FIELD-SWEEP-SINGLE-PATH` doctrine with two self-tested arms: no algebra/tag/operator dispatch in the sweep path; no new bespoke field-kernel WGSL outside the retiring seven-shader allowlist. The allowlist is migration debt and empties at 10.1; it is not authority to add an eighth shader.
- Add only load-bearing admission/parity proofs, one signal-only results doc, one evidence-index line, inventory rows, and a 5.5 PROBATION status-row update. Do not advance the pointer in coding.
## FENCES
- The IR is the permanent specification even if 5.7 later specializes or JITs execution. Do not implement or prefigure 5.7, and do not use the 5.4 diagnostic ratios as a 5.5 threshold verdict.
- 5.5 consumes existing N4 adjacency only. No N8, weighted presets, radius generators, LinkGraph, degree bucketing, or conductance certificate work; 5.6 owns those axes.
- Preserve the existing PALMA/Gu-Yang referees and corpus bytes unedited. Bespoke stencils remain comparison oracles through migration and are not deleted, expanded, or blessed as the final architecture.
- Preserve RF/replay determinism, typed row/column/slot authority, homogeneous lanes, Corpus Boundary, Native Ingestion, and the Invariant Set. No second registry, listener, CPU decision path, or reverse dependency.
## EXIT-PROOF
- PALMA and Gu-Yang authored registrations execute through the generic N4 path and match the existing CPU and GPU bespoke paths bit-for-bit on identical inputs, canonical order, coefficients, iterations, and output columns; existing referees remain unedited and the full corpus battery is green.
- Admission negatives prove missing/invalid law proof, missing canonical-order proof, non-symmetric conservative adjacency, non-default resource class, and malformed edge-context usage fail before encode/dispatch with typed errors and spans where authored data exists.
- Tree-wide census proves one field-sweep execution path with zero semiring/algebra enums, field-kind tags, or operator-identity matches; `FIELD-SWEEP-SINGLE-PATH` fires on both planted violations and remains green on the exact tree.
- P5/STEAD/guard coevolution, workspace/core/kernel/gpu/adapter-pinned driver proofs, inventory/lifecycle/detachability/scans/orientation/anchors/doc-budget, exact-head clearance, results/index/status row are green. Return PROBATION for DA deep-tree graduation; no 5.6 dispatch.
