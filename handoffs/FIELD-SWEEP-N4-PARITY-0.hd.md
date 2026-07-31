---
rung: FIELD-SWEEP-N4-PARITY-0
kind: rung
track: 0.0.8.7
base_sha: c0126aaf1966821bfcd1a4878c87ba97a9d54734
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-approved 5.x amendment #1482; DA ruling 5138782208. Rung 5.4 retired N4 correctness risk; occupancy/counters/threshold/resource classes/JIT belong to 5.7. Frontier lane: Codex 5.6/Fable. IR remains the specification; 2026-08-11 remains sovereign."
surfaces: ["crates/simthing-core/src", "crates/simthing-gpu/src", "crates/simthing-driver", "scripts/ci", "docs"]
forbidden: ["5.6 adjacency/N8/LinkGraph work", "5.7 resource classes, counters, threshold, specialization, or JIT", "5.8 or Phase 6+ work", "semiring/algebra enum, field-kind tag, or operator dispatch", "corpus/scenario edits or existing oracle deletion", "new bespoke field WGSL or unrelated lifecycle exemption"]
required_checks: ["cargo build --workspace", "core/kernel/gpu suites", "adapter-pinned full driver battery with PALMA and Gu-Yang N4 parity", "stead_spatial_contract_guards", "FIELD-SWEEP-SINGLE-PATH scan + planted selftests", "inventory/lifecycle/detachability checks", "agent/doctrine scans", "orientation/anchors/doc-budget/clearance"]
stop_conditions: ["stale-orient-receipt", "N4 parity is not bit-exact", "generic execution requires algebra/tag/operator dispatch", "law/order proof cannot seal admission", "5.6 or 5.7 work is required", "new bespoke kernel, corpus edit, referee edit, or scope widening is required"]
---
## BUILD
- Land production N4 field sweep as EML over an existing gather. Extend edge context with `{target_slot, neighbor_slot, accumulator, edge_scalar, dt}` and add `TARGET_VALUE` / `NEIGHBOR_VALUE`; field identity never enters evaluator dispatch.
- Add `FieldSweepRegistration { adjacency, map_program, fold_program, identity_bits, post_program, FieldLawProof, CanonicalOrderProof, resource_class }`. Admit only the legacy fixed-32-stack default until 5.7.
- Conservative folds require an undirected-symmetry certificate; every registration carries canonical-order proof; execution uses fixed linear fold and authored N4 offset order.
- Express PALMA and Gu-Yang as authored registrations through the generic path, retaining their bespoke stencils as unmodified migration oracles; reproduce CPU and GPU outputs bit-exact.
- Co-evolve P5, STEAD §10, and `stead_spatial_contract_guards`. Land permanent `FIELD-SWEEP-SINGLE-PATH`: no algebra/tag/operator dispatch, and no eighth bespoke field shader beyond the retiring seven-file allowlist; self-test both arms.
- Add only load-bearing admission/parity proofs, one results doc, one evidence-index line, inventory rows, and 5.5 PROBATION. Coding does not move the pointer.
## FENCES
- IR remains the permanent specification. Do not infer a 5.5 performance verdict or implement 5.7 specialization/resource work.
- Existing N4 adjacency only; 5.6 owns N8, weighted presets, radius, LinkGraph, bucketing, and conductance certificates.
- Preserve corpus bytes and existing PALMA/Gu-Yang referees. Bespoke stencils remain comparison oracles and are neither expanded nor retired here.
- Preserve RF/replay determinism, typed row/column/slot authority, homogeneous lanes, Corpus Boundary, Native Ingestion, and the Invariant Set; no second registry, listener, CPU decision path, or reverse dependency.
## EXIT-PROOF
- Authored PALMA and Gu-Yang registrations execute through generic N4 and match existing CPU/GPU bespoke paths bit-for-bit on identical inputs/order/coefficients/iterations/columns; unedited referees and full corpus battery are green.
- Typed pre-dispatch negatives bite for invalid/missing law proof, order proof, symmetry certificate, non-default resource class, and malformed edge context.
- Census proves one field-sweep path with zero algebra enums/tags/operator matches; both planted `FIELD-SWEEP-SINGLE-PATH` violations fire.
- P5/STEAD/guard, build/suites/adapter battery, lifecycle/detachability/scans/orientation/anchors/doc-budget, results/index/status, and exact-head clearance are green. Return PROBATION for DA deep-tree graduation; no 5.6 dispatch.
