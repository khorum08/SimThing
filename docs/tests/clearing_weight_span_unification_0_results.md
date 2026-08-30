# CLEARING-WEIGHT-SPAN-UNIFICATION-0 Results

- Status: **PROBATION / proof-present / DA-review-pending**
- Rung: `CLEARING-WEIGHT-SPAN-UNIFICATION-0`
- Handoff-authored provenance base: `630e57d0d9747af3372d79310d4a49ccf5c976e5`
- Exact dispatch base: `9d43d793c5052a42f1fed4bdb14eb7751d690f52`
- tested_code_sha / final_head_sha: PR- and Board-relay-bound after this evidence commit; this file does not self-hash
- ORIENT-RECEIPT: `7f8318ec9261`
- role: `coding`
- orientation_rule_stamp: `164def77d171c853`
- orientation_digest_sha: `7bfbb737d1df043f559b4c5d80ff6b9395dbf045d3dff83d37e7136b94668043`
- HD-RECEIPT: `545149dd0604`
- coverage_basis: **PASS** — canonical-span, compatibility-removal, germ, 11.2f, 11.3, cross-domain, policy, and zero-red workspace evidence

## Archaeology and dispatch-base oracle

The required pre-edit inventory and oracle were published before production
edits in Board comment `5468732723`.

| Surface | Dispatch-base census |
|---|---|
| `resolve_effective_clearing_weights` | One body in `flow_market.rs`; one executable caller in `stemthing_b_flow_market_germ_0.rs`; the result was a full `BTreeMap<SimThingId, f32>`. |
| Graduated 7.8a substrate | Private generic `DerivedSpanProjection<D>` plus public `OverlaySpanProjection::compile`; its frozen `LogicalSubtreeDirectory`, interned profiles, and maximal spans are the canonical access path. |
| Canonical constrained clear | `clear_constrained_claims_at_generation`; production callers in `growth_entitlement.rs` and the internal stamped route, plus existing embedder/sim/driver witnesses. |
| Synthetic compatibility clear | Exactly one body, one consumer file (`contention_arena_executed_0.rs`), and four calls. The DA census matched before editing. |

Dispatch-base exact truth:

- Germ effective weights were `compute_a=2.0` and `compute_b=1.0`; compute grants were `4/1` of 5, residency grants totaled 2, and exact-tie winners differed at generations 12/13.
- The 11.2f grant lifecycle was `6 -> 8 -> 7 -> (3,4) -> 7 -> 0`, with accepted lane `[14,0,6,20]` and six replayed lifecycle facts.
- The 11.3 convergence control/emergence vectors were `[0,0,0,0,0]` and `[0,0,0,1,1]`; crossing/delivery/consequence generations were `2/2/3`; the resident capacity/dimensions/admission generation were `6/7/0`.
- Base blob IDs: germ `c662078aa74e64158c69a04663d0e91e333488ac`; 11.2f `ca7d1f451d120d320df4c017600ecf94444c4a5f`; 11.3 `95775d437d0f4a5e4bd9e44125c8b94518267d25`.
- Base execution was germ 4 passed, 11.2f 3 passed, 11.3 1 passed, 0 failed.

## What changed

`resolve_effective_clearing_weights` now accepts the already-compiled
`OverlaySpanProjection`. A kernel-owned specialization resolves sparse
inherited overrides against that projection's frozen logical subtree directory,
partitions only at override boundaries, and admits bit-exact effective values
into the existing generic `DerivedSpanProjection<f32>` substrate. Point lookup
uses the canonical logical directory and maximal interval spans. It does not
walk `SimThing.children`, materialize participant rows, build a participant
weight `BTreeMap`, or retain a fallback resolver/cache.

The old recursive body was deleted from `flow_market.rs`. The old
synthetic-generation `clear_constrained_claims` body was deleted from
`constrained_clearing.rs` and from all public re-export surfaces. Its sole
consumer now makes these truthful stamped calls:

| Old call | New call | Explicit authority |
|---|---|---|
| fitting clear | `clear_constrained_claims_at_generation` | granter `supply_id`, generation 10 |
| lawful order A | `clear_constrained_claims_at_generation` | granter `supply_id`, generation 10 |
| lawful order B | `clear_constrained_claims_at_generation` | granter `supply_id`, generation 10 |
| segregated scopes | `clear_constrained_claims_at_generation` | granter `supply_id`, generation 10 |

The stamped RF report used by the test is generation 10, so there is no
replacement default/current/synthetic-generation helper.

## Single-path and deletion proof

- `flow_market.rs`: 68 old resolver lines deleted and 5 canonical re-export lines added (net `-63`).
- `constrained_clearing.rs`: 24 compatibility-door lines deleted (net `-24`).
- Searches find zero `clear_constrained_claims(` definitions or calls and exactly four `_at_generation` migrations in the authorized consumer.
- The canonical specialization contains no `BTreeMap`, no `children` access, and no recursive `visit`/tree walker. `OverlaySpanProjection::logical_directory` is its only participant-topology input.
- The generic span substrate remains the singular profile/span implementation; no allowlist expansion or alternate root-level kernel surface was added.

## Bit identity and immutable regression seals

The germ's authored assertions and expected values are diff-identical to the
dispatch base. Its only delta is harness plumbing: import
`OverlaySpanProjection`, compile the participant projection once, and pass that
projection instead of the root to the resolver. The green post-change germ
therefore re-proves the exact base vector and all base clearing outcomes.

The 11.2f and 11.3 witness files have zero diff and retain their exact base blob
IDs (`ca7d1f...` and `95775d...`). Their post-change results are respectively
3 passed and 1 passed. Clearing arithmetic, integer apportionment, eligibility,
epsilon behavior, tie ordering, exact integer DecimalField behavior, and replay
semantics were not edited.

## Cross-domain matrix

`clearing_weight_span_unification_0.rs` routes ship, commodity, and freighter
claims through the canonical projection and proves:

- root `0.5`, ship/freighter `2.0`, and commodity one ULP above `1.0` remain bit exact;
- the no-op freighter descendant coalesces with ship, yielding exactly three profiles and three maximal spans;
- duplicate, unknown, negative, and invalid-default inputs fail closed;
- physical claim reversal produces an identical clear;
- the equal ship/freighter weighted tie rotates under generations 7/8;
- the same stamped clear replays identically;
- integer hundredths `100:200:300` apportioned from `100` are exactly `17:33:50`, total 100, remainder 0, with no float conversion or tolerance in clearing arithmetic.

## Verification

| Command / proof | Result |
|---|---|
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-kernel derived_span_projection::tests` | PASS — 4 passed, 0 failed |
| Five driver seals: new matrix, compatibility consumer, germ, 11.2f, 11.3 | PASS — `2 + 1 + 4 + 3 + 1`, 0 failed |
| `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet` | `STRUCTURAL-CERTIFICATE-SUMMARY suites=126 passed=478 failed=0 ignored=14 exit=0` |
| `test_inventory_check.sh` / `test_inventory_drift_check.sh` | PASS — 1,357 discovered/ledgered; unledgered 0; stale 0; lifecycle PASS |
| `agent_scan.sh` | PASS — hard 0, delta inspect 0 |
| `doctrine_selftest.sh` | PASS |
| `doctrine_scan.sh` | INSPECT — hard 0; 417 standing whole-tree heuristic findings covered by the existing justification ledger |
| `handoff_dispatch.sh --lint/--receipt` | PASS / `545149dd0604` |
| `gen_orientation.sh --check` / `orient.sh --selftest` | PASS / 4 fixtures |
| `anchor_check.sh` | PASS — pending and curation PASS; standing coverage advisory retained |
| `cargo fmt --all -- --check` / `git diff --check` | PASS / PASS |

## Scope and routing

Changed implementation scope is limited to the kernel span specialization and
crate-private accessors, spec binding/re-exports and dependency, the two
authorized driver consumers, the new driver matrix, Cargo lock data, the test
inventory, append-only anchor reach rows, and this evidence/index pair. There
are no workflow or gate-code edits, clearing-arithmetic edits, Vector CostBand
or other banked-performance work, pointer movement, closeout preparation, or
engineering-review ceremony.

Return posture is **PROBATION / proof-present / DA-review-pending** under
`DA-RESERVE(binding)`. Coding does not merge, graduate, move the pointer, begin
closeout, or invoke engineering review.
