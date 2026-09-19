# Structural-addition dynamic RF admission + EIGHTEENTH E8 roll

DA increment per Orchestration relay `5743461789` (Astra return `5738682372` accepted as a
truthful substrate STOP on 2.2 `0088-ECONOMY-FLEET-0`; #2075 retained RED at `ddf62c55`). There is
no 2.2 semantic verdict. Model 1 remains UNDECIDED and Model 2 CLOSED. The first consumer remains
#2075.

## Ruling

ADMITTED as the smallest generic connection. A successful ordinary structural addition is admitted
to EXISTING arenas by the same definition session-build derivation uses, scoped to the added
subtree. The one RF authority is unchanged: `ArenaRegistry::admit_participant_runtime`, one
generation bump, then the existing `sync_resource_flow` and resident-clearing rebind. There is no
second registry, ledger, upkeep manager or readback policy.

**The policy distinction (relay rule 6) is admission ORIGIN, which the derivation already records.**

- **`Derived` arenas take structural additions.** Their membership is a definition: every carrier
  of the flow property. A carrier born later belongs by that same definition.
- **`AuthoredOverride` arenas stay closed.** Explicit, enrollment and wildcard rows are an
  enumeration. A born carrier there is reported as a typed `ClosedArena` note, never admitted, and
  the note does not fail the batch.

No new policy field is needed.

## The law (relay rules 1–14)

- **Trigger (2).** The trigger is the boundary's own record of successful additions,
  `outcome.maintainer.allocated`. It records every node of each `AddChild` subtree, parents first.
  The walk starts only at the true roots: the allocated nodes whose committed parent was not itself
  allocated. A refused or unapplied candidate allocates nothing, so it is never seen. Fission never
  allocates there and keeps its own path (10).
- **Subtree-scoped (3).** Only the added subtrees, each root's committed `ChildOf` parent, and the
  existing membership are read. Nothing is re-derived globally.
- **The derivation rule (4, 5).** For each existing arena and each added node:
  - a node participates only if it carries the arena's flow property, so a parent's arenas are
    never inherited wholesale;
  - one matching resource-parent edge is authoritative;
  - two matching edges refuse `AmbiguousParentEdge`, with every span token;
  - one edge to a non-member refuses `ParentNotParticipant`, with its span;
  - absent an edge, the physical parent is the resource parent only when it is a member (existing,
    or co-admitted in the same batch); otherwise the node joins flat;
  - a missing committed slot refuses `MissingSlot`;
  - a committed slot still held by another member of the arena refuses `SlotHeld`, so one slot
    is never two members. Until departure is law, a removed member's row keeps its retired slot,
    and a later birth placed on that slot must not alias it.
- **Bounded (7).** The whole batch is preflighted before any mutation:
  - `max_participants` refuses `Capacity { declared, computed }`;
  - the arena's OrderBand budget refuses `DepthBudget { needed, max }`, because a deeper tree would
    otherwise fail only at the post-admission sync, after the registry had mutated.

  Nothing widens at runtime.
- **Declared growth sizes derived caps at session build.** A derived cap now counts the carriers
  that authored structural products will birth: `count` × the template nodes carrying the property.
  So `next_power_of_two(N0 + declared)` bounds everything the source declares. Authored caps stay
  authoritative. A session with no products is byte-identical in cap, fanout and band budget.
- **Atomic (8, 9).** Any batch-failing refusal admits nothing. A successful batch bumps the
  generation exactly once. Fission and structural admissions share ONE sync per boundary.
- **Identity (11).** An already-admitted identity is skipped, so a replayed addition never
  duplicates membership. The slot is the one the boundary committed. Fail-stop and retry law is
  untouched.
- **Removal and reparent (12)** are NOT swept in (see the follow-on below).
- **Generic (13).** No domain vocabulary appears in driver or RF code.
- **E8 (14).** Sealed bytes moved, so this increment makes the eighteenth roll (below).

**Surfaces:**
- new unsealed `crates/simthing-driver/src/resource_flow_structural_enrollment.rs`;
- `SimSession::react_to_resource_flow_enrollment`, the one ordinary boundary reaction, which
  replaces the fission-only session method (the burn-in's synthetic outcome allocates nothing, so
  it is unchanged);
- a read-only `SimRuntimeTree::resource_parent_edges_of`;
- `last_resource_flow_structural_enrollment_report`.

## Proof floor executed (reference machine)

**Driver, ordinary session** (`rehearsal_structural_rf_enrollment_0`, 5/5):

- **Admission.**
  - A born fleet (carrier), with a crew carrier and a cargo non-carrier, lands under a member hub.
  - Fleet → hub and crew → fleet are admitted; cargo is not.
  - Participants +2, generation +1.
  - The walk roots at the fleet only.
  - The fleet is admitted on its committed slot.
  - Settlement is exactly `4.0` versus `3.0` for born flow `0` versus `-1`.
- **Edges.**
  - An edge to the root overrides the physical parent.
  - Two edges → `AmbiguousParentEdge [Some(7), Some(7)]`.
  - An edge to a non-member → `ParentNotParticipant`, span `Some(7)`.
  - In both refusal cases, a lawful sibling carrier is NOT half-admitted, and membership and
    generation are unchanged.
- **Closed, capacity, refused and replayed births.**
  - An authored-row arena gives exactly one `ClosedArena` note.
  - At 14 of 16 members, a 3-carrier birth → `Capacity { declared: 16, computed: 17 }`, with the
    registry unchanged.
  - An 81-node birth refused by placement → no report, no membership.
  - A replayed admission → nothing admitted, nothing refused.
- **Depth budget.** Budget 32 (governed bands cost `3D − 1 + 4`):
  - a 6-link chain (D = 9, needs 30) admits all 6;
  - a 7-link chain (D = 10, needs 33) → `DepthBudget { needed: 33, max: 32 }`, with nothing
    mutated;
  - the session keeps running in both cases.
- **Held slot.**
  - A born fleet is enrolled, then canonically removed; its arena row persists.
  - The next birth is placed on the retired slot 12.
  - It refuses `SlotHeld { slot: 12, holder: <removed id> }`, with nothing mutated, and the session
    keeps running.

**Native path** (`native_structural_products_session_0::funded_births_join_the_energy_arena_and_settle_their_authored_flow`):
the shipped bundle is re-authored in a temp copy and run through parse → hydrate → profile →
`SimSession`. The product is two funded corvettes per unit: a fleet carrying `meridian::energy`
flow F, a powered reactor, and an energy-less crew.

- Births land at G2 and G3. Each admits exactly fleet → A1 and reactor → fleet, and never the crew.
- The energy arena grows by 4.
- The second birth fits only because declared growth sized the derived cap: 13 + 4 → cap 32.
- Settled energy: control `4.0` versus funded `2.0`, a difference of exactly `−2`, the two born
  flows. Terran's A1 pool nets to exactly 0; pirate's side is unchanged.
- An interior fleet participates with its children's upsweep weight, so the witness uses reactor
  weight 2 to keep A1's split dyadic and the comparison exact. The shipped 1/6 split rounds per
  node in f32 (control `3.9999997`); arithmetic conservation holds either way.

**Mutants (8/8 RED, each restored).** M1–M6 ran against the driver target's first four tests:
- M1: enrollment never admits → driver 3 of 4 RED + native RED;
- M2: non-carriers inherit → 3 of 4 RED;
- M3: batch not atomic → 3 of 4 RED;
- M4: edges ignored → the edge test RED;
- M5: no depth preflight → the depth test RED;
- M6: no capacity preflight → the capacity test RED;
- M7: declared growth ignored → native RED at the second birth,
  `Capacity { declared: 16, computed: 17 }`;
- M8: no held-slot check → the reused-slot birth is admitted onto slot 12, and the witness goes RED.

**Existing law GREEN on the new pin.** The workspace sweep has 0 failures:
- core 32;
- spec 16;
- kernel 55;
- sim 26;
- gpu 7 (`eml-resource-profiling`);
- driver 54:
  - the 2.1 floor (recipe 8/8, WIP 3/3, discriminator 2/2, ingress 1/1);
  - settlement 1/1;
  - this law 5/5;
- clausething 45;
- mapeditor 25;
- embedder 10;
- workshop 15 (#2079's four product proofs plus this law's native witness).

feeder, mapgenerator and tools declare no tests. One mapeditor attempt failed at `link.exe`
(exit 1102) while a concurrent probe build was linking; it was rerun alone and passed.
Local doctrine gates:
- every step of `doctrine-scan.yml` PASS;
- `TEST-BUDGET` INSPECT justified in `inspect_justifications.tsv`;
- DEAD-EXPORT (52) and exit-proof INSPECTs are identical on base.

## Adjacent defects fixed in passing

- The `MaintainerOutcome.allocated` doc said "one per `AddChild`". It records every node of each
  added subtree, parents first. The first walk re-walked every descendant as a root; it now roots
  only at true roots. The stale doc was corrected in the unsealed `simthing-feeder` file.
- The session's fission-children filter over `allocated` was dead, because fission never allocates
  there. It was removed.
- `install.rs` kept an unused import of the no-growth derivation wrapper. It now imports the
  growth-aware form it calls.
- `ArenaRegistry::admit_participant_runtime` checks only capacity, never slot uniqueness. The
  structural preflight now refuses a held slot (`SlotHeld`). Fission's own path is untouched and
  shares the exposure, so the follow-on below covers both.
- The E-2B-5 fission-enrollment law has no standing test left in the tree: its battery was reaped
  with the pen, and the burn-in/soak harnesses are src-only. The fission path here is a pure
  call-site refactor. A temporary probe ran both dynamic fission burn-in fixtures through the new
  combined reaction: `run_resource_flow_burn_in` passed its contract with 1 and 2 admissions. The
  probe was not committed.

## Named follow-on (relay rule 12): structural REMOVAL / reparent RF departure

`ArenaRegistry` has no departure operation. RF re-syncs only at session open and after
admissions. A canonical `Remove` of an arena member therefore leaves its participant row, and the
synced plan, on the retired slot. This was already true for N0 members. It is now reachable for
born members through 2.1's funded-cancellation `Remove`.

A probe, not committed, measured it directly. After `Remove` of an enrolled born fleet:
- the identity is gone from the tree and the allocator;
- the arena row and the plan still carry slot 12;
- the zeroed row settles 0, so no mass leaked on that shape;
- the stale row still consumes one unit of `max_participants`.

A later birth placed on the retired slot now refuses `SlotHeld` rather than aliasing, so the hazard
fails closed. But a cancel → rebirth cycle cannot re-enroll, and cancellations leak capacity.

The lawful mirror is a separate law, not guessed here:
- a tombstoned identity leaves every arena;
- batch preflight;
- one generation bump;
- one sync;
- fission-side fusion reconciled.

Reparent needs the same treatment. #2075's remaining 2.2 floor (continuation, recovery, identity)
must not cancel an enrolled born subtree until that follow-on lands.

## EIGHTEENTH E8 roll

Sealed components moved: `crates/simthing-driver/src/session.rs` (the report field and the one
combined boundary reaction) and `crates/simthing-sim/src/sim_runtime_tree.rs` (the read-only
edge accessor). The law itself lives in the new unsealed module.

- **Old-pin refusal (RED, required first) on the final source:**
  - driver resident path: `UnqualifiedAdapter { required: 1702590768412684334, observed: 17165209339348674868 }`;
  - parity referee FAILED with `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ee3712f2ef186934`.

  The two derivations agree.
- **Roll:** both literals `0x17a0_d148_7d18_f02e` → `0xee37_12f2_ef18_6934`. No comparator,
  component-list, `build.rs` or Cargo change. Two intermediate rolls existed only in the working
  tree (`0x295e_d730_727b_b9f3`, `0x1310_cac5_2a07_bcef`) and were superseded by later sealed edits.
  No commit carries them: the pre-roll checkpoint `fd8b16a1` holds the seventeenth pin. The refusal
  was recaptured on the final sealed bytes.
- Pin chain: `…0x9993…` → `0x17a0…` → `0xee37_12f2_ef18_6934`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `2e08aeec` (the exact eighteenth-roll commit);
- command: fresh `git clone` with `core.autocrlf=false` (canonical LF checkout) at that exact commit,
  in sibling directory `simthing-e8-roll18-verify` (not under %TEMP%);
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ee3712f2ef186934`; referee
  `1 passed; 0 failed`. The fresh canonical-LF clone reproduces the pinned fingerprint.
- Later commits touch no `build.rs` component. They add the held-slot preflight in the unsealed
  module, witnesses, ledger/taxonomy/justification data, and this packet.
