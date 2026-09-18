# Settlement ownership law + RF-1 governed-leaf fact + FIFTEENTH E8 roll

DA increment found while landing relay `5730468245` (2.2 native recipe-input repair). Running the
clausething suite showed `ct_2a_intrinsic_flow` and `ct_2c_category_economy` RED on master.
**Both regressions are the DA's own #2065** (bisected: `72d3c338` PASS, `ec078b30` FAIL). Model 1
remains UNDECIDED; Model 2 remains CLOSED; 2.2 stays parked until this and the recipe-input repair
land.

## A. RF-1 could not express a governed leaf (oracle vocabulary gap)

RF-1's structural identity summed every leaf's allocation as a terminal handoff AND summed every
`balance_delta`. Since #2065 a balance-governed leaf settles its allocation into its own Balance,
so the same mass was counted twice (ct_2a: sources 8 / sinks 16; ct_2c: 10.5 / 21).

**Repair:** `ArenaMemberObservation` gains `balance_governed`, an explicit caller-supplied fact
(the declared sub-field governance — never inferred from executed deltas). Only an UNGOVERNED leaf's
allocation is a terminal sink. For every ungoverned observation the identity is unchanged. ct_2a and
ct_2c supply the fact from the resolved layout both tests already use: both GREEN.

A wrong fact cannot hide a violation. If a caller declares a settling leaf ungoverned, the mass is
counted twice. If it declares a non-settling leaf governed, the leaf's allocation is counted
nowhere. RF-1 refuses both. With the fact in place, RF-1 caught both engine defects in B below:
mass creation and mass destruction.

The dead adapter `flat_star_observations` is deleted. It had no caller after its consumer was
reaped, and it built member observations on the old "every leaf is terminal" assumption.

## B. Settlement ownership law (engine; supersedes the #2065 leaf rule)

The disbursement law pools a participant's intrinsic into its parent's `intrinsic_flow_sum_col`
exactly when that parent sits below the root; the root disburses only its own intrinsic. #2065's
settlement did not mirror that:

- **Mass created:** every leaf seeded its own intrinsic at every depth, but a leaf below depth 1
  had already received that intrinsic (with its siblings) as AllocatedFlow.
- **Mass destroyed (pre-existing interior path):** a depth-1 interior settled only its pool, and
  nobody disburses its own intrinsic, so that intrinsic vanished.

**Empirical RED first** (ordinary nested D=4 session, RF-1 with the fact above):

| case | sources | sinks | defect |
|---|---|---|---|
| deep-leaf intrinsic | 5.5 | 10.0 | +4.5 created (Σ deep-leaf intrinsic) |
| depth-1 interior intrinsic | 3.0 | 0.0 | −3.0 destroyed |
| every level | 9.25 | 10.75 | net +1.5 |

**Law:** each governed participant settles exactly what it owns. It owns its own intrinsic only
when its parent does not pool it (depth ≤ 1). An interior below the root owns the pool it
disburses. Everyone settles AllocatedFlow received minus AllocatedFlow disbursed. Across any tree,
received and disbursed flow cancel, so every intrinsic unit settles exactly once. The root, deeper
interiors, and depth-1 leaves keep their historical op shapes. A pooled leaf now seeds zero. A
depth-1 interior adds its own intrinsic in the residual band that was reserved but unused
(`seed + 2`), so the band budget is unchanged.

**Shipped-scenario reach (measured):** `stellaristhing_base`'s energy tree is root → owner →
location → cohorts. The cohorts are depth-3 governed leaves with authored flows `2, 2, −1, −1`.
A scratch probe loaded the shipped bundle through the ordinary native loader and settled one
generation:

| engine | generator settles | upkeep settles | Σ settled | Σ intrinsic |
|---|---|---|---|---|
| master (#2065 law, 14th pin) | 2.5 | −0.5 | **8** | 4 |
| repaired (15th pin) | 0.5 | 0.5 | **4** | 4 |

On master, the 2.1 energy economy created 2 energy per location per generation. Owner balances
(terran 10, pirate 8) are static under both engines: owners have no authored flow of their own,
and everything they receive they disburse.

## Proof floor executed (reference machine)

- `nested_governed_arena_settles_every_intrinsic_unit_exactly_once`: RED before (table above) →
  GREEN after for all three cases. Each pooled leaf's settlement bit-equals its allocation, and each
  unpooled leaf's bit-equals its own intrinsic plus its allocation.
- `settlement_ownership_mirrors_the_disbursement_law` (planner, GPU-free): per-participant op
  sources are root `{own, allocated, children}`, depth-1 interior
  `{pool, own, allocated, children}`, pooled leaf `{0, allocated}`, and unpooled leaf
  `{own, allocated}`.
- ct_2a + ct_2c GREEN (the explicit fact). #2065's witnesses stay GREEN
  (`balance_governed_leaves_receive_residual_closure`, `arena_plans_carry_no_governed_integration`).
- 2.1 construction suites on the new pin: recipe 8/8, WIP 3/3, discriminator 2/2, ingress 1/1.
  Driver lib 20/20. The full clausething and mapeditor suites are GREEN (no failures).

## FIFTEENTH E8 roll (`arena_allocation_plan.rs` is a bundle component)

- **Old-pin refusal (RED, required first)** on the repaired source: ordinary ingress
  `UnqualifiedAdapter { required: 17153818183069797343, observed: 11810271022427289334 }`; parity
  referee FAILED with `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: a3e682ebe0de36f6`. The two
  derivations agree.
- **Roll:** both literals atomically `0xee0e_9ac0_af83_0bdf` → `0xa3e6_82eb_e0de_36f6`. No third
  site. No comparator, component-list, `build.rs`, or Cargo change.
- Pin chain: `…0xec5a…` → `0xee0e…` → `0xa3e6_82eb_e0de_36f6`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `478969a9` (the exact fifteenth-roll commit)
- command: fresh `git clone` with `core.autocrlf=false` (canonical LF checkout) at that exact commit,
  in sibling directory `simthing-e8-roll15-verify` (not under %TEMP%);
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: a3e682ebe0de36f6`; referee
  `1 passed; 0 failed`. The fresh canonical-LF clone reproduces the pinned fingerprint.
