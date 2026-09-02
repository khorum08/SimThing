# RESIDENT-CLEARING-APPORTIONMENT-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED / no 14.5**

Authority: Board handoff `5510249901`, re-dispatch/remand `5511124134`, DA ruling
`5504763991`, owner-approved repository handoff
`handoffs/RESIDENT-CLEARING-APPORTIONMENT-0.hd.md`.

- exact live-master base: `35db328de5ddadf9557454b0453a2f42d971b565`
- coding ORIENT receipt: `b9535319b9be`
- orientation rule stamp: `52aa2c585b39f3d5`
- HD receipt: `878d979ee78c`
- posture: candidate proof only; 14.3 remains frozen; coding does not merge, graduate,
  move the pointer, retire CPU doors, or begin 14.5

## Archaeology map

| Obligation | Existing authority reopened | Binding used here |
|---|---|---|
| Exact clear law | `simthing-spec/src/spec/constrained_clearing.rs::clear_constrained_claims_at_generation` | checked totals; equality-band proportional base; exact numerator/quotient/remainder; descending remainder then claimant id; exact-tie rotation by `granter + generation`; claimant-id result order; checked unresolved `u32` sum |
| Resident semantic identity/budget | `simthing-kernel/src/resident_clearing_plan.rs`; `simthing-gpu/src/resident_clearing_plan.rs` | immutable canonical owner/resource/scope/draw rows, semantic digest, per-tree buffers, already-admitted 64-byte scratch row |
| Continuous authority | 14.3 `AllocatedFlow` in sealed `WorldGpuState.resolved.values()` | the exact encoder is crate-private and the public world-state encode door supplies only the sealed live values buffer; no caller-provided score/weight/continuous buffer is accepted |
| Terminal placement | `ArenaBandLayout::integration_band` | driver planner stamps that exact existing band into the immutable exact plan; the kernel only encodes into the caller's existing command encoder and creates no schedule/band/clock/host clear phase |
| Recursive product | Germ Self-Consumption Law §1.5.1 and the 14.2 recursive-port census | one `ResidentConstrainedProduct`; `ResidentSettlementOutput` and `ResidentRecursiveSupplyIntake` are aliases, with no conversion or materialized intermediary |
| Legacy bridge fence | private `MarketGrantRecord::from_cleared_offering` in `simthing-spec/src/spec/flow_market.rs` | zero references in the resident exact Rust/WGSL path; the legacy constructor remains private and local |

Archaeology found no missing exact-apportionment authority and minted no score,
pressure, demand, grant, scheduler, clock, or recursive adapter ontology.

## Exact-law equivalence

| CPU operation | Resident operation | Proof |
|---|---|---|
| canonical scope grouping | canonical resident owner/resource/scope ordinals | alternating-scope and permuted-admission witness |
| score-band sequence already decided upstream | admitted hard `precedence`; no `f32` score-bit synthesis | no score or normalized-weight field exists in the resident executor |
| `requested_total: u64` checked sum | `vec2<u32>` low/high sum with explicit carry | max-domain argument and live boundary vectors |
| `available_for_band = min(remaining, requested_total)` | exact pair compare/subtract/min | zero, full, and short supply witnesses |
| checked `available_for_band * requested` | 32-step shift/add `u32 × u32 → u64` | `u32::MAX × u32::MAX` live GPU parity |
| quotient/remainder by exact integer division | 64-step restoring binary division | all focused cases compare exactly to the frozen CPU authority |
| descending exact remainder, claimant id ascending | pair comparison plus source-id rank | canonical `100:200:300 / 100 = 17:33:50` |
| rotate each exact-remainder tie by `(granter + generation) % tie_len` | exact-pair addition/division and rotated canonical tie index | successive-generation `[1,1] / 1` witness |
| checked `unresolved_total: u32` | exact scope-total minus supply; high limb refuses | two `u32::MAX` requests with zero supply refuse in CPU, mirror, and GPU |
| grants sorted by scope, then claimant id | products sorted by scope key, then `source_simthing_id` | exact vector equality and canonical maps |
| zero requests omitted | zero requests removed at plan construction | CPU/GPU exact parity for `[0,5] / 3` |

The resident CPU function is an equivalence mirror only. The frozen spec door remains
the semantic oracle and is not narrowed or retired by this rung.

## Software-wide boundary proof

The representation is the exact pair `(low: u32, high: u32)` for every CPU `u64`.
Addition returns a third carry bit, subtraction is used only after an exact pair
comparison, multiplication consumes both 32-bit operands one bit at a time, and
division consumes all 64 numerator bits. No floating conversion occurs in integer
settlement.

For a resident plan, `row_count <= u32::MAX` and every request is `u32`, so the
largest admitted requested total is `(2^32-1)^2`, which is representable in `u64`.
The largest numerator is also `(2^32-1)^2`. The implementation nevertheless carries
and checks addition overflow at each sum, quotient high limbs, grant increment,
unresolved narrowing, and host-side byte/index arithmetic. No accepted request,
supply, identity, or generation range was narrowed.

Enumerated live vectors: `17:33:50`, zero supply, full supply, short supply,
`u32::MAX × u32::MAX`, `granter=u32::MAX + generation=u32::MAX`, zero request,
successive exact ties, and unresolved-total overflow/refusal.

## Integration-band and product proof

`plan_resident_exact_apportionment` receives the graduated arena layout and stamps
exactly `layout.band_layout.integration_band`. `WorldGpuState` supplies its sealed
live `AllocatedFlow` cells and encodes the exact pass into the existing caller-owned
command encoder. There is no standalone host-clear runner and no second terminal
band. Non-finite/negative live allocation is a typed refusal; a live NaN witness
proves the resident cell is actually read.

`ResidentConstrainedProduct` is 32 bytes and is written directly into the output
half of each already-admitted 64-byte 14.2 scratch row. It carries semantic row,
claimant identity, exact grant, exact unresolved `U`, generation, and integration
band. Both recursive role names are aliases of that one type. The compile-fail
doctest rejects a `GrantRow -> ChildSupplyRow` conversion at the recursive port.

## Physical-order matrix

| Perturbation | Compared shape | Result |
|---|---|---|
| claim upload order | ascending vs reverse 65-claim upload | exact products identical |
| physical semantic admission | ascending vs reverse alternating-scope admissions | canonical bytes/digest identical |
| scope storage | two interleaved logical scopes, canonicalized independently | exact products identical |
| epoch rebound | identity-preserving slot permutation `(i*17) mod 65` | exact products identical |
| workgroup cardinality | WG64 vs WG32 pipelines | exact products identical |
| dispatch partition | one 65-row pass vs 7-row command partitions | exact products identical |
| atomic/arrival mutant | first-arrival remainder winners under forward/reverse order | deliberately differs (RED), while resident products remain equal |

The shader contains no atomic operation. Partition passes write only the product half
at canonical semantic-row destinations, so later partitions never overwrite input
words read by other claims.

## Changed-file census

- kernel exact product/plan/mirror/session and exact WGSL
- sealed `WorldGpuState` encode binding and kernel exports
- GPU resident-buffer ownership checks and re-exports
- driver terminal-band planner/export
- spec alias re-export only (CPU law unchanged)
- focused workshop referee, test inventory, evidence index, anchor reach log, bounded
  kernel-surface/shader-census allow records, and this report

No manifest, lockfile, workflow, CI gate code, governance pointer, workplan stamp,
orientation guide, closeout artifact, or 14.5/14.6 surface changed.

## Verification

Focused evidence is `crates/simthing-workshop/tests/resident_clearing_apportionment_0.rs`
(4/4 green on the local Vulkan adapter). The certification relay/PR records the exact
head and hosted Doctrine Scan/Exec, `/clearance`, and `/relay-lint` workflow runs.

Local certification was green for touched-crate `cargo check`, kernel compile-fail
doctests (50/50), the 14.4 referee (4/4), frozen 14.2/14.3 witnesses, the
symbol-keyed host-clearing-door census, clearing weight-span unification, test
inventory and drift (1379/1379), anchor check, `git diff --check`, and Agent Scan
(0 hard failures, 0 inspect flags). Existing unrelated compiler warnings remain.
