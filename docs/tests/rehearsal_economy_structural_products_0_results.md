# Native funded structural products → the existing 2.1 ActionBand door + SEVENTEENTH E8 roll

DA increment per Orchestration relay `5737649159` (Astra return `5737628239` accepted as a
truthful substrate STOP on 2.2 `0088-ECONOMY-FLEET-0`; #2075 retained RED at `ec04a635`). Model 1
remains UNDECIDED; Model 2 remains CLOSED; no fleet semantics; the first consumer remains #2075.

## Ruling

ADMITTED as the smallest generic authoring + installation mapping. Native source can now declare
a funded, detached structural template. It lowers once, at session build, into the SAME frozen
ActionBand session product that 2.1 graduated on. The session adds no executor, queue, callback or
polling.

```
structural_product = terran_corvettes {
  funding = { entity = A1 property = "meridian_material::A1_corvettes_quantity" role = Amount }
  count = 7
  parent = A1
  template = {
    kind = Fleet
    owner_ref = terran
    property_value = { property = "…" Amount = 1 }
    overlays = { modifier = { … } }
    children = { child = crew { kind = Cohort } }
  }
}
```

## The mapping (relay rules 1–12)

- **Unchanged execution path (1, 2).** Unit `k` of a product lowers to the bounded 2.1 sequence:
  - an `EmitOnThresholdRegistration` at `k + 0.5` on the funding cell;
  - the values-plane crossing registration the boundary's band-crossing source consumes (an
    observation cost band, the step 2.1's installer also performs);
  - one ActionBand template, with `reserved_instance_rows = 1`;
  - one `StructuralAuthorization(AddChild { parent, child })` consequence;
  - one active instance at the funding host.

  All units freeze through `ActionBandSessionBuildDoor::admit_once_at_session_build` and install at
  tick zero through `install_action_band_commitments`. That door refuses any later or second bind.
- **Funding locus (3).** The funding locus is the existing property, role and host (typically a
  recipe's target quantity). No receipt property or event ledger is minted. The host must carry the
  property. 2.1's crossing provenance, source-generation and threshold-definition laws apply
  unchanged.
- **Detached template, ordinary semantics (4, 5).** The template is never in the N0 tree, and
  `fleet_ship_payload` is untouched. Each unit is its own freshly minted instance. The born node
  carries:
  - kind and children;
  - authored property cells (existing properties, scalar roles, finite values);
  - its own overlays, compiled by `compile_overlay` with `affects` set to the born node;
  - an explicit `owner_ref` binding; absent, ownership inherits from the parent.

  The parent must be an existing structural host and never an owner seat.
- **Placement law reused (6).** Births place only through the canonical implicit residency market.
  A game mode that declares products therefore carries its substrate
  (`install_default_resident_rf_property`), and authored arenas no longer retire it. This is the
  composition every graduated 2.1 construction session ran. With no extent, the result is the
  existing typed `GrowthResidencyRefused`: no subtree, no placement. Such sessions admit resident
  clearing and are seal-gated.
- **Exactly-once and bounded (7, 9).** `count` (a `NonZeroU32`) lowers to exactly `count`
  templates, so funding past the last threshold births nothing more. No unbounded spawner is
  inferred.
- **Generic (8).** No fleet vocabulary enters the driver or kernel.
- **Canonical fidelity (10).** The declaration lives on `GameModeSpec.structural_products` (empty
  and skipped when absent, so every existing asset's JSON is unchanged). It rides the derived cache,
  and a cache rebind births the same meaning.
- **Fails closed (11).** Hydration refuses with a span: syntax, repeated fields, a duplicate id,
  zero count, an unknown `owner_ref`, an owner-seat parent. Session build refuses typed with the
  declaration's token: an unknown funding property or role, an unresolved host or parent, a host
  lacking the property, an unknown template property, a failed overlay compile.
- **No persistence (12).** Nothing new is persisted.

## Proof floor executed (reference machine, ordinary native path)

Each case uses the shipped bundle re-authored in a temp copy (a shipyard recipe
`2 alloys → 1 corvette` capped at 1 unit per generation), run through native parse → hydrate →
profile → `SimSession`:

- **Funded birth:**
  - N0 and G1 have no product and no placement;
  - the first crossing (corvettes 1.0 > 0.5) births at G2 and the second at G3, then nothing more,
    while funding rises to 7 (`count = 2`);
  - each birth has a fresh identity under A1, with subtree size 3 (template + 2 crew), 1 own
    overlay, owner `terran`, the cargo property, committed residency placement of quantity 3, and
    `ChildOf(A1)`;
  - a session rebuilt from the derived cache births identically on the same generations.
- **Unfunded:** shipyard cost 100 → no corvettes, no identities, no refusals.
- **Unplaceable:** an 81-node template → one typed `GrowthResidencyRefused`, and the tree is
  unchanged.
- **Declaration order:** terran and pirate declared in both orders give identical outcomes, two
  births per faction.
- **Malformed:** ten shapes refuse before activation with provenance: unknown funding property;
  non-scalar role; unknown host; unknown parent; owner-seat parent; unknown owner; zero count; host
  lacking the property; unknown template property; duplicate product id.
- **Mutants RED:**
  - drop the values-plane crossing registration → no births;
  - drop the template's children → the born shape is wrong.
- Existing law GREEN on the new pin:
  - 2.1 suites (recipe 8/8, WIP 3/3, discriminator 2/2, ingress 1/1) and the settlement witness;
  - driver lib;
  - the full clausething, spec, sim and mapeditor suites;
  - the #2077/#2078 recipe witnesses 3/3.

## SEVENTEENTH E8 roll

Sealed components moved: `crates/simthing-driver/src/session.rs` (the tick-zero install call, the
residency-substrate carry for product sessions, and one typed `SessionError::StructuralProduct`
variant) and `crates/simthing-clausething/src/hydrate_shipsize_decoder.rs` (one `GameModeSpec`
literal gains the empty field). The lowering itself lives in the new, unsealed
`crates/simthing-driver/src/structural_product.rs`.

- **Old-pin refusal (RED, required first) on the final source:**
  - driver resident path: `UnqualifiedAdapter { required: 11066379952262892183, observed: 1702590768412684334 }`;
  - parity referee FAILED with `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 17a0d1487d18f02e`.

  The two derivations agree.
- **Roll:** both literals `0x9993_adef_3422_4a97` → `0x17a0_d148_7d18_f02e`. No comparator,
  component-list, `build.rs`, or Cargo change. A WIP checkpoint on the branch briefly carried an
  intermediate pin (`0xcacd_bab6_cf4c_4e34`), which was superseded before the hook moved into the
  unsealed module. It was never on master.
- Pin chain: `…0xa3e6…` → `0x9993…` → `0x17a0_d148_7d18_f02e`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `d51e877c` (the exact seventeenth-roll commit)
- command: fresh `git clone` with `core.autocrlf=false` (canonical LF checkout) at that exact commit,
  in sibling directory `simthing-e8-roll17-verify` (not under %TEMP%);
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 17a0d1487d18f02e`; referee
  `1 passed; 0 failed`. The fresh canonical-LF clone reproduces the pinned fingerprint.
- The later typed-owner commit touches only unsealed files (`OwnerRef` replaces a stringly
  `owner_ref`, per the doctrine scan's `SPEC-STRING-CHANNEL`). The product witnesses stay GREEN
  on the pin.
- **Overlay-germ census:** the lowering's `instantiate` pushes compiled template overlays onto a
  DETACHED, not-yet-born node. It is classified as residue (`authoring-product`,
  GENUINELY-STRUCTURAL / keep) in `overlay_germ_archaeology_census.tsv`. Those overlays enter the
  tree only through the classified `AddChild` structural apply, so the site is no runtime attach or
  lifecycle route (the same reason ClauseThing authoring is out of census scope). The gate logic is
  unchanged.
