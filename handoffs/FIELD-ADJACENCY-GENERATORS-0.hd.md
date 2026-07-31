---
rung: FIELD-ADJACENCY-GENERATORS-0
kind: rung
track: 0.0.8.7
base_sha: bbe45411c611ffacb117966867b41f1f28ccd0b1
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Owner-approved 5.x amendment #1482; DA graduation ruling 5143085476. 5.5 landed one bit-exact generic N4 interpreter. 5.6 owns adjacency as the registration axis; 5.7 owns counters/resource classes/specialization/JIT. Emergence is the exit falsifier; 2026-08-11 remains sovereign."
surfaces: ["crates/simthing-core/src", "crates/simthing-kernel/src/field_sweep.rs", "crates/simthing-kernel/src/shaders/field_sweep.wgsl", "crates/simthing-gpu/src", "crates/simthing-driver", "crates/simthing-clausething/tests/stead_spatial_contract_guards.rs", "scripts/ci", "docs"]
forbidden: ["5.7 counters, occupancy, resource classes, threshold adjudication, specialization, or JIT", "5.8 comparative projections or Phase 6+ work", "field/algebra enum, semantic tag, operator dispatch, or second interpreter", "implicit/free N8 diagonal weights or scheduler-authored conductance", "new bespoke field WGSL or deletion/weakening of the seven migration oracles", "corpus/scenario/referee edits or unrelated lifecycle exemption"]
required_checks: ["cargo build --workspace", "core/kernel/gpu suites", "adapter-pinned full driver battery", "N4/N8/LinkGraph CPU-GPU parity and emergence falsifier", "weighted-degree/conductance admission negatives", "canonical-order and degree-bucket order invariance", "zero-production-legacy-caller census + planted selftest", "FIELD-SWEEP-SINGLE-PATH scan + selftests", "stead_spatial_contract_guards", "inventory/lifecycle/detachability checks", "agent/doctrine scans", "orientation/anchors/doc-budget/clearance"]
stop_conditions: ["stale-orient-receipt", "same generic interpreter cannot execute all admitted adjacency forms", "existing LinkGraph compiler cannot supply sorted+deduped undirected canonical lists", "N4/N8/LinkGraph dynamics are qualitatively indistinguishable", "N8 diagonals require an implicit metric/default", "scheduler must set chi, edge weights, or neighbor order", "5.7 work, new bespoke kernel, corpus edit, oracle deletion, or scope widening is required"]
---
## BUILD
- Generalize the sealed field-sweep adjacency registration from fixed GridN4 to weighted `GridOffsets [(dx,dy,w)]` and LinkGraph lists over the existing `INPUT_LIST`; retain one map/fixed-linear-fold/post interpreter and typed slot/column authority through the existing wire frontier.
- Provide admitted N4, N8, and radius-r GridOffsets constructors. Every edge weight is authored/admitted data; N8 diagonal weights are explicit inputs, never inferred as Chebyshev, Euclidean, or `sqrt(2)` defaults.
- Admit LinkGraph only from the existing link compiler's sorted+deduped undirected neighbor lists. Mint `CanonicalOrderProof` and symmetry/law evidence from that compiler output; do not add a second sort, graph registry, or runtime topology dispatch.
- Add per-node weighted-degree/conductance metadata and a sealed certificate proving `chi_i * sum_j(abs(c_ij)) <= admitted_bound`. Reject non-finite/negative weights, malformed reverses, duplicate/order drift, missing certificates, and over-bound nodes with spanned admission errors.
- Degree-homogeneous scheduling buckets may read admitted degree/certificate metadata only. They must not choose chi, rewrite weights, reorder any node's neighbors, or change CPU/GPU fold order; session binding covers lists, weights, proofs, and layout.
- Migrate every production caller of the retiring seven bespoke field shaders to the generic registration path. Keep the seven implementations unmodified as test-only migration oracles; add a permanent zero-production-caller census with a planted production-call failure.
- Add load-bearing parity/admission/order proofs, one results doc, one evidence-index line, inventory/status rows, and 5.6 PROBATION. Coding does not move the pointer.
## FENCES
- Use the same authored map/fold/post program, coefficients, seed state, and observation rule for GridN4, GridN8, and LinkGraph. Only admitted adjacency data may differ.
- The emergence test is output-derived, not scripted per topology: N4 must form Manhattan/diamond geometry, authored-weight N8 an octagonal/non-Manhattan contour, and LinkGraph topology-following propagation. A planted adjacency alias that makes two outputs indistinguishable must flip the falsifier red.
- No performance verdict is required. Preserve 5.5 bit-exactness, `FIELD-SWEEP-SINGLE-PATH`, RF/replay determinism, homogeneous lanes, Native Ingestion, Corpus Boundary, and the Invariant Set.
- Do not implement border/chokepoint projections, resource specialization, counters, or legacy-oracle deletion; those remain 5.7/5.8/10.1.
## EXIT-PROOF
- N4, N8, radius-r, and LinkGraph registrations execute through the one generic CPU/GPU path; identical admitted inputs are bit-exact, malformed/over-bound admissions reject before upload/dispatch, and bucketed versus unbucketed execution preserves each node's authored order and bits.
- The same-program emergence falsifier distinguishes N4 diamond, N8 authored-octagonal, and LinkGraph topology-following geometry; the planted indistinguishability control fails as designed.
- Grep/census proves zero production callers of all seven bespoke shaders while existing oracle comparisons remain green; the planted production caller and existing algebra/shader selftests all bite.
- STEAD guard, build/suites/adapter battery, lifecycle/detachability/scans/orientation/anchors/doc-budget, results/index/status, and exact-head clearance are green. Return PROBATION for DA deep-tree graduation; no 5.7 dispatch.
