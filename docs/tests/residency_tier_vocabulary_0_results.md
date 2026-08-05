# RESIDENCY-TIER-VOCABULARY-0 — witness results

Rung: 6.5 `RESIDENCY-TIER-VOCABULARY-0` · dispatch `5197384131` · authority DA
`5197291879` (production-path amendment) · HD-RECEIPT `0191ec3a74f5` · base
`3e4f6edd`.

## What landed

**The StemThing-A residency vocabulary — authored data over a small closed
generic engine vocabulary** (`crates/simthing-core/src/residency_tier.rs`).
A tier is a price vector, never a category:

- Closed engine vocabulary (generic, small, sealed): `LaneSet`
  {participate, act, originate, receive}, `ResidencyShapeClass`
  {SpatialBlock, CompactRow}, `AdjacencyParticipation` {Absent, GridN4},
  `ResidencyChurnClass` {Static, Recyclable, Elastic} — churn classes name
  dynamism only and carry no movement/placement meaning (6.4 logical
  `SlotIndex` law preserved).
- Authored `ResidencyTierRow` = `{name, lanes, shape, adjacency, churn,
  unit_cost_rows}` — exactly the sealed price-vector dimensions; `name` is
  authored label used ONLY for duplicate detection at admission and the
  authoring-side `tier_id_by_name` binding lookup.
- **Session freeze:** `SessionTierSet::admit` validates (zero unit cost,
  empty lane set, duplicate names, width bound — all spanned failures with
  row indices) and freezes; the type has NO mutation surface. The ONE
  production session door is
  `SpecSessionState::admit_session_residency_tiers`; a second call is a
  mid-session tier mint and refuses with
  `MidSessionTierMintRefused { admitted, attempted }` citing the chartered
  but absent Owner-gated epoch-boundary door.
- **Consumption is identity-blind by construction:**
  `resolve_residency_draw(tier, n)` reads only the priced components (rows =
  `N·C` CostBand, lanes, shape, adjacency, churn); `ResidencyDrawShape`
  deliberately carries no tier id or name — downstream machinery receives
  shape, never identity.

**Exact hard-currency partition** (`ResidencyCapacityPartition`):
`free + in_flight + occupied = capacity` with the in-flight seam holding
account inside the judged universe; discrete-exact `issue`/`deliver`/
`cancel_in_flight`/`release` transitions each re-verify the invariant
(`verify_exact` — the 8.1-class conservation judge's operand). No approximate
conservation; no placement/extent geometry — capacity is quantity.

**Sparse granting-node census** (`materialize_granting_census`): given the
node universe and the granting-active subset, fixed-width lanes (counts,
churn, growth velocity per admitted tier — width session-fixed by the frozen
set) are allocated for EXACTLY the granting-active nodes. Non-granting nodes
are ABSENT — `lanes()` returns `None`, never a zero-filled row; bytes scale
with granting activity, never `O(nodes × tiers)`.

Out of scope, untouched: granting arena, grant execution, allocator
retirement, placement/extents, compaction policy, movement, contention,
7.x/8.x, dynamic mid-session tier door, StemThing-B.

## Witnesses (crates/simthing-driver/tests/residency_tier_vocabulary_0.rs — 4/4 green)

1. **Admission + freeze + mid-session mint RED** through the production
   session door: invalid rows fail with spanned admission errors and freeze
   nothing; a lawful set freezes (width 4); ANY second admission REDs
   `MidSessionTierMintRefused { admitted: 4, attempted: 1 }` leaving the
   frozen set untouched.
2. **Exact partition over inline scenario-neutral synthetic grants:** a
   14-step interleaved issue/deliver/cancel/release script with live
   in-flight throughout holds `free + in_flight + occupied = capacity`
   exactly after every transition and returns to `free == capacity`;
   over-draws refuse exactly on all four doors.
3. **Identity blindness + health ratio:** two admitted rows with identical
   price vectors and different names (one named `granting-root`, the shape an
   identity branch would single out) resolve to byte-equal draw shapes at
   every quantity; forty authored entity names bind to the four generic tiers
   as pure data (`BTreeMap<String, TierId>`) and resolve through at most the
   admitted price vectors — zero engine change.
4. **Sparse census memory profile:** 200-node universe, 3 granting-active →
   exactly 3 lane blocks of `width × 3 lanes × 4 bytes = 48` bytes each;
   non-granting nodes return `None`; total bytes `3 × 48`; a 10× larger
   universe with the same granting set costs identical bytes.

## Production-path mutation runs (DA `5197291879` amendment — all four RED, reverted, green)

Each defect was planted INTO the production path it guards, the battery run,
the RED captured, and the mutant reverted (battery green after every revert):

1. **Mid-session mint** — the freeze refusal deleted from
   `SpecSessionState::admit_session_residency_tiers` → witness 1 REDs
   (`assertion failed: matches!(... MidSessionTierMintRefused ...)`).
2. **Omitted `in_flight`** — `issue` mutated to drop the holding-account
   credit (`self.in_flight += rows` deleted) in
   `ResidencyCapacityPartition::issue` → witness 2 REDs at the first
   post-issue `verify_exact` (`partition holds after every transition:
   PartitionNotExact`).
3. **Match-on-tier** — an identity branch
   (`if tier.name == "granting-root" { +1 }`) planted into
   `resolve_residency_draw` → witness 3 REDs
   (`authored tier identity leaked into engine behavior at n=0`).
4. **Dense census** — the granting filter deleted from
   `materialize_granting_census` (lanes on every node) → witness 4 REDs
   (`granting_node_count` 200 ≠ 3; bytes scale with the universe).

## Zero-engine-branching source proof

`grep -rn "\.name"` over `residency_tier.rs` closes onto: admission
validation (duplicate detection + error spans, lines 175–191) and the
authoring-side `tier_id_by_name` lookup (line 223) — the ONLY name-aware
surface, used to bind authored entity names to frozen ids as data. The
consumption path (`resolve_residency_draw`) and every downstream shape
(`ResidencyDrawShape`) are name-free; no `match` on tier identity exists in
any engine crate (`ResidencyTierRow`/`SessionTierSet`/`resolve_residency_draw`
consumers close onto `simthing-core` + the one driver door). Domain nouns
appear in no tier row: the four canonical rows are generic shape labels
(spatial-container, compact-participant, compact-policy-holder,
granting-root).

## Bookkeeping

- Inventory: +4 driver witness rows, +5 core unit rows.
- Ladder: ONLY the 6.5 status row moved (TODO → PROBATION, single anchored
  line edit); all future rung statuses remain TODO; orientation regenerated.
- Evidence index: one line added for this rung.
- 6.4 preserved: no slot/remap/order surface touched; `SlotIndex` logical
  identity law intact (churn classes are vocabulary, not movement).
