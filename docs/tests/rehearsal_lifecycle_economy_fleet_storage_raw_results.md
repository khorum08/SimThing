# 2.2 inventory / bounded-stock ingress raw observations

Implementation head `510d4cd3045d3e876e9dce67f4ba9787e4b8d9f6`, tree `9cdde7d866c4372c83c046cada45355a3dd41fde`. Base a14d4a57813952781ec475cfd32d7c4f80fadfda. Continuation of ce632646, no second rebase.

Test logs start at the harness `running N test(s)` line to omit repeated compiler warnings; all subsequent witness output is retained. Ledger correction tests precede the new stock-bound probe. The storage focus and implementation-head full parent retain the newly discovered semantic RED. Final-head reruns/hosted facts are bound in PR/Board. Compiler-inclusive originals remain locally under `.git/economy-fleet-22-ledger-resume`.

Candidate source identity: `fnv1a64:a04e385ae7fa0df5:7527`. Rejected before activation; there is no admitted candidate profile digest.

## Ledger correction: capacity exact focus

Exit 0. Source log `capacity-focused.txt`.

```text
running 1 test
test sequential_capacity_exhaustion_preserves_prior_births ... CAPACITY_FIRST_REFUSAL G17: births=12 funding=(16, 12.0)->(17, 13.0) threshold=12.5 capacity=0 reason=MarketUnresolved { granted: 0 } placements=[(SimThingId(307), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(307), market_grant_key: 10352942583764372668 }, extent: ResidencyExtent { start: 36, length: 3 }, quantity: 3, committed_generation: GenerationStamp(2) }), (SimThingId(310), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(310), market_grant_key: 8573165843398269791 }, extent: ResidencyExtent { start: 39, length: 3 }, quantity: 3, committed_generation: GenerationStamp(3) }), (SimThingId(313), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(313), market_grant_key: 14002190669195122962 }, extent: ResidencyExtent { start: 42, length: 3 }, quantity: 3, committed_generation: GenerationStamp(4) }), (SimThingId(316), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(316), market_grant_key: 8514883878960620981 }, extent: ResidencyExtent { start: 45, length: 3 }, quantity: 3, committed_generation: GenerationStamp(5) }), (SimThingId(319), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(319), market_grant_key: 4438988822954733144 }, extent: ResidencyExtent { start: 48, length: 3 }, quantity: 3, committed_generation: GenerationStamp(6) }), (SimThingId(322), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(322), market_grant_key: 6254572420137714587 }, extent: ResidencyExtent { start: 51, length: 3 }, quantity: 3, committed_generation: GenerationStamp(8) }), (SimThingId(325), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(325), market_grant_key: 18212029171555461294 }, extent: ResidencyExtent { start: 54, length: 3 }, quantity: 3, committed_generation: GenerationStamp(9) }), (SimThingId(328), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(328), market_grant_key: 2548703524055453713 }, extent: ResidencyExtent { start: 57, length: 3 }, quantity: 3, committed_generation: GenerationStamp(10) }), (SimThingId(331), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(331), market_grant_key: 1095901044537297988 }, extent: ResidencyExtent { start: 60, length: 3 }, quantity: 3, committed_generation: GenerationStamp(12) }), (SimThingId(334), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(334), market_grant_key: 14742320039626927559 }, extent: ResidencyExtent { start: 63, length: 3 }, quantity: 3, committed_generation: GenerationStamp(13) }), (SimThingId(337), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(337), market_grant_key: 470047695646874394 }, extent: ResidencyExtent { start: 66, length: 3 }, quantity: 3, committed_generation: GenerationStamp(14) }), (SimThingId(340), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(340), market_grant_key: 2280841519078078333 }, extent: ResidencyExtent { start: 69, length: 3 }, quantity: 3, committed_generation: GenerationStamp(16) })]
CAPACITY_END births=12 first_refusal=Some(17) later_refusals=17 live0=36 capacity0=36
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.57s
```

## Ledger correction: native full

Exit 0. Source log `native-full.txt`.

```text
running 6 tests
test funded_births_join_the_energy_arena_and_settle_their_authored_flow ... G2 birth SimThingId(307): StructuralEnrollmentReport { added_roots: [SimThingId(307)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(307), arena_idx: 0, participant_slot: 36, parent: Some(SimThingId(213)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(308), arena_idx: 0, participant_slot: 37, parent: Some(SimThingId(307)) }], refusals: [], generation_before: 1, generation_after: 2 }
G3 birth SimThingId(310): StructuralEnrollmentReport { added_roots: [SimThingId(310)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(310), arena_idx: 0, participant_slot: 39, parent: Some(SimThingId(213)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(311), arena_idx: 0, participant_slot: 40, parent: Some(SimThingId(310)) }], refusals: [], generation_before: 2, generation_after: 3 }
G2 birth SimThingId(413): StructuralEnrollmentReport { added_roots: [SimThingId(413)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(413), arena_idx: 0, participant_slot: 36, parent: Some(SimThingId(319)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(414), arena_idx: 0, participant_slot: 37, parent: Some(SimThingId(413)) }], refusals: [], generation_before: 1, generation_after: 2 }
G3 birth SimThingId(416): StructuralEnrollmentReport { added_roots: [SimThingId(416)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(416), arena_idx: 0, participant_slot: 39, parent: Some(SimThingId(319)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(417), arena_idx: 0, participant_slot: 40, parent: Some(SimThingId(416)) }], refusals: [], generation_before: 2, generation_after: 3 }
settled energy with born flow 0 / -1: [4.0, 2.0]
ok
test funded_output_births_exactly_the_authored_detached_subtree ... ok
test malformed_product_declarations_refuse_before_activation ... refused (is not registered): StructuralProduct(FundingLocus { product: "terran_corvettes", reason: "property `meridian_material::A1_frigates_quantity` is not registered", span: Some(514) })
refused (is not a scalar sub-field): StructuralProduct(FundingLocus { product: "terran_corvettes", reason: "role Named(\"reserve\") is not a scalar sub-field of `meridian_material::A1_corvettes_quantity`", span: Some(514) })
refused (not in install_targets): StructuralProduct(UnresolvedEntity { product: "terran_corvettes", role: "funding host", entity: "nowhere", reason: "is not in install_targets", span: Some(514) })
refused (not in install_targets): StructuralProduct(UnresolvedEntity { product: "terran_corvettes", role: "parent", entity: "nowhere", reason: "is not in install_targets", span: Some(514) })
refused (is an owner seat): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpf97TXC\\stellaristhing_base.clause: ClauseThing hydration error at token 514: structural_product `terran_corvettes` parent `terran` is an owner seat; declare ownership on the template with `owner_ref`")
refused (unknown owner_ref): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpgd2QZ4\\stellaristhing_base.clause: ClauseThing hydration error at token 534: unknown owner_ref `nobody`")
refused (count must be a positive integer): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpkwcwOl\\stellaristhing_base.clause: ClauseThing hydration error at token 526: structural_product.count must be a positive integer")
refused (does not carry): StructuralProduct(FundingLocus { product: "terran_corvettes", reason: "host `E1` does not carry `meridian_material::A1_corvettes_quantity`", span: Some(514) })
refused (is not registered): StructuralProduct(Template { product: "terran_corvettes", reason: "property `meridian_material::A1_rubble_quantity` is not registered", span: Some(514) })
refused (duplicate structural_product id): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpuF8iqw\\stellaristhing_base.clause: ClauseThing hydration error at token 568: duplicate structural_product id `terran_corvettes`")
ok
test product_meaning_is_independent_of_declaration_order ... declaration-order outcome: [(2, "A1", (2, 1, "terran", 2)), (2, "E1", (2, 1, "pirate", 2)), (3, "A1", (2, 1, "terran", 2)), (3, "E1", (2, 1, "pirate", 2))]
declaration-order outcome: [(2, "A1", (2, 1, "terran", 2)), (2, "E1", (2, 1, "pirate", 2)), (3, "A1", (2, 1, "terran", 2)), (3, "E1", (2, 1, "pirate", 2))]
ok
test sequential_capacity_exhaustion_preserves_prior_births ... CAPACITY_FIRST_REFUSAL G17: births=12 funding=(16, 12.0)->(17, 13.0) threshold=12.5 capacity=0 reason=MarketUnresolved { granted: 0 } placements=[(SimThingId(1695), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1695), market_grant_key: 6664684684200072115 }, extent: ResidencyExtent { start: 36, length: 3 }, quantity: 3, committed_generation: GenerationStamp(2) }), (SimThingId(1698), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1698), market_grant_key: 12883776174534346028 }, extent: ResidencyExtent { start: 39, length: 3 }, quantity: 3, committed_generation: GenerationStamp(3) }), (SimThingId(1701), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1701), market_grant_key: 912873953335216777 }, extent: ResidencyExtent { start: 42, length: 3 }, quantity: 3, committed_generation: GenerationStamp(4) }), (SimThingId(1704), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1704), market_grant_key: 15320242607439861962 }, extent: ResidencyExtent { start: 45, length: 3 }, quantity: 3, committed_generation: GenerationStamp(5) }), (SimThingId(1707), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1707), market_grant_key: 17990460431141411511 }, extent: ResidencyExtent { start: 48, length: 3 }, quantity: 3, committed_generation: GenerationStamp(6) }), (SimThingId(1710), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1710), market_grant_key: 17089988189343564176 }, extent: ResidencyExtent { start: 51, length: 3 }, quantity: 3, committed_generation: GenerationStamp(8) }), (SimThingId(1713), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1713), market_grant_key: 2852310034386724749 }, extent: ResidencyExtent { start: 54, length: 3 }, quantity: 3, committed_generation: GenerationStamp(9) }), (SimThingId(1716), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1716), market_grant_key: 10609369773598259966 }, extent: ResidencyExtent { start: 57, length: 3 }, quantity: 3, committed_generation: GenerationStamp(10) }), (SimThingId(1719), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1719), market_grant_key: 8937020855176226923 }, extent: ResidencyExtent { start: 60, length: 3 }, quantity: 3, committed_generation: GenerationStamp(12) }), (SimThingId(1722), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1722), market_grant_key: 14129580037250649412 }, extent: ResidencyExtent { start: 63, length: 3 }, quantity: 3, committed_generation: GenerationStamp(13) }), (SimThingId(1725), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1725), market_grant_key: 4821042542904771233 }, extent: ResidencyExtent { start: 66, length: 3 }, quantity: 3, committed_generation: GenerationStamp(14) }), (SimThingId(1728), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1728), market_grant_key: 8115104194765900866 }, extent: ResidencyExtent { start: 69, length: 3 }, quantity: 3, committed_generation: GenerationStamp(16) })]
CAPACITY_END births=12 first_refusal=Some(17) later_refusals=17 live0=36 capacity0=36
ok
test unfunded_or_unplaceable_products_birth_nothing ... unplaceable: births [[], [], [], [], [], []] refusals 1
ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 39.80s
```

## Ledger correction: parent full

Exit 0. Source log `parent-full.txt`.

```text
running 5 tests
test rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow ... NATIVE_SOURCE_BEGIN case=zero-upkeep-control
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp0NYE12\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:de1e1bb5c38de973:27258 targets={"pirate_generator_2": [SimThingId(217)], "stellaristhing_base": [SimThingId(235)], "terran_generator_1": [SimThingId(210)], "terran_shipyard": [SimThingId(209)], "terran": [SimThingId(207)], "pirate_generator_1": [SimThingId(216)], "terran_generator_2": [SimThingId(211)], "terran_refinery": [SimThingId(213)], "A1": [SimThingId(214)], "pirate_mine": [SimThingId(218)], "terran_mine": [SimThingId(212)], "pirate": [SimThingId(208)], "pirate_shipyard": [SimThingId(215)], "E1": [SimThingId(220)], "pirate_refinery": [SimThingId(219)]}
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=214 id=262
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=220 id=268
UPKEEP_BIRTH_ENROLLMENT case=zero-upkeep-control generation=6 non_carriers_excluded=true report=StructuralEnrollmentReport { added_roots: [SimThingId(262), SimThingId(268)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(262), arena_idx: 1, participant_slot: 38, parent: Some(SimThingId(214)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(268), arena_idx: 1, participant_slot: 41, parent: Some(SimThingId(220)) }], refusals: [], generation_before: 1, generation_after: 2 }
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=262 parent=214 slot=Some(SlotIndex(38)) authored_observed_flow=0 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(38), subtree_root: SimThingId(262), parent: Some(SimThingId(214)) }]
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=268 parent=220 slot=Some(SlotIndex(41)) authored_observed_flow=0 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(41), subtree_root: SimThingId(268), parent: Some(SimThingId(220)) }]
NATIVE_SOURCE_BEGIN case=one-energy-upkeep
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpvq4NaI\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:a7795d79157244fe:27260 targets={"pirate_mine": [SimThingId(285)], "terran_refinery": [SimThingId(280)], "pirate_shipyard": [SimThingId(282)], "A1": [SimThingId(281)], "E1": [SimThingId(287)], "terran_mine": [SimThingId(279)], "terran": [SimThingId(274)], "pirate": [SimThingId(275)], "pirate_generator_2": [SimThingId(284)], "terran_generator_1": [SimThingId(277)], "pirate_generator_1": [SimThingId(283)], "pirate_refinery": [SimThingId(286)], "terran_generator_2": [SimThingId(278)], "stellaristhing_base": [SimThingId(302)], "terran_shipyard": [SimThingId(276)]}
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=281 id=329
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=287 id=335
UPKEEP_BIRTH_ENROLLMENT case=one-energy-upkeep generation=6 non_carriers_excluded=true report=StructuralEnrollmentReport { added_roots: [SimThingId(329), SimThingId(335)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(329), arena_idx: 1, participant_slot: 38, parent: Some(SimThingId(281)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(335), arena_idx: 1, participant_slot: 41, parent: Some(SimThingId(287)) }], refusals: [], generation_before: 1, generation_after: 2 }
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(38), subtree_root: SimThingId(329), parent: Some(SimThingId(281)) }]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(41), subtree_root: SimThingId(335), parent: Some(SimThingId(287)) }]
ok
test rehearsal_economy_fleet_existing_asset_executes_without_profile_replacement ... SOURCE path=C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop\../../scenarios/stellaristhing_base.clause identity=fnv1a64:ee4e4df9e8c9fbd9:5798 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=canonical: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=canonical identity=fnv1a64:70abad1cc363f419:21555 targets={"terran_generator_2": [SimThingId(344)], "terran_mine": [SimThingId(345)], "terran_refinery": [SimThingId(346)], "pirate_refinery": [SimThingId(351)], "E1": [SimThingId(352)], "A1": [SimThingId(347)], "pirate_mine": [SimThingId(350)], "stellaristhing_base": [SimThingId(365)], "pirate": [SimThingId(342)], "terran_generator_1": [SimThingId(343)], "pirate_generator_1": [SimThingId(348)], "pirate_generator_2": [SimThingId(349)], "terran": [SimThingId(341)]}
N0 case=canonical root=364 existing_ids={341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 364, 365, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389} targets={"terran_generator_2": [SimThingId(344)], "terran_mine": [SimThingId(345)], "terran_refinery": [SimThingId(346)], "pirate_refinery": [SimThingId(351)], "E1": [SimThingId(352)], "A1": [SimThingId(347)], "pirate_mine": [SimThingId(350)], "stellaristhing_base": [SimThingId(365)], "pirate": [SimThingId(342)], "terran_generator_1": [SimThingId(343)], "pirate_generator_1": [SimThingId(348)], "pirate_generator_2": [SimThingId(349)], "terran": [SimThingId(341)]}
CELL case=canonical generation=0 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [230, 25, 183, 137, 85, 106, 150, 225, 167, 19, 106, 207, 224, 199, 18, 185], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [230, 25, 183, 137, 85, 106, 150, 225, 167, 19, 106, 207, 224, 199, 18, 185], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [230, 25, 183, 137, 85, 106, 150, 225, 167, 19, 106, 207, 224, 199, 18, 185], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [230, 25, 183, 137, 85, 106, 150, 225, 167, 19, 106, 207, 224, 199, 18, 185], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpH9YsOV\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=energy-withheld: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=energy-withheld identity=fnv1a64:99a92696421983bc:21554 targets={"A1": [SimThingId(398)], "stellaristhing_base": [SimThingId(416)], "terran_generator_1": [SimThingId(394)], "terran_refinery": [SimThingId(397)], "pirate_mine": [SimThingId(401)], "pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "terran_mine": [SimThingId(396)], "E1": [SimThingId(403)], "pirate_generator_2": [SimThingId(400)], "pirate": [SimThingId(393)], "terran": [SimThingId(392)], "pirate_refinery": [SimThingId(402)]}
N0 case=energy-withheld root=415 existing_ids={392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 415, 416, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440} targets={"A1": [SimThingId(398)], "stellaristhing_base": [SimThingId(416)], "terran_generator_1": [SimThingId(394)], "terran_refinery": [SimThingId(397)], "pirate_mine": [SimThingId(401)], "pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "terran_mine": [SimThingId(396)], "E1": [SimThingId(403)], "pirate_generator_2": [SimThingId(400)], "pirate": [SimThingId(393)], "terran": [SimThingId(392)], "pirate_refinery": [SimThingId(402)]}
CELL case=energy-withheld generation=0 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [49, 187, 93, 12, 175, 146, 43, 60, 214, 246, 180, 113, 168, 136, 59, 4], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [49, 187, 93, 12, 175, 146, 43, 60, 214, 246, 180, 113, 168, 136, 59, 4], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [49, 187, 93, 12, 175, 146, 43, 60, 214, 246, 180, 113, 168, 136, 59, 4], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [49, 187, 93, 12, 175, 146, 43, 60, 214, 246, 180, 113, 168, 136, 59, 4], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpO66Va6\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:32e5fea2aa51c47a:21574 targets={"pirate_generator_1": [SimThingId(450)], "terran_refinery": [SimThingId(448)], "pirate": [SimThingId(444)], "stellaristhing_base": [SimThingId(467)], "pirate_refinery": [SimThingId(453)], "terran_generator_2": [SimThingId(446)], "terran": [SimThingId(443)], "A1": [SimThingId(449)], "terran_mine": [SimThingId(447)], "terran_generator_1": [SimThingId(445)], "E1": [SimThingId(454)], "pirate_generator_2": [SimThingId(451)], "pirate_mine": [SimThingId(452)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpXQ3lxS\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:bd041ab3522a8008:21573 targets={"terran_generator_1": [SimThingId(496)], "pirate_generator_2": [SimThingId(502)], "terran_generator_2": [SimThingId(497)], "stellaristhing_base": [SimThingId(518)], "pirate_mine": [SimThingId(503)], "terran_mine": [SimThingId(498)], "terran_refinery": [SimThingId(499)], "pirate_refinery": [SimThingId(504)], "A1": [SimThingId(500)], "E1": [SimThingId(505)], "terran": [SimThingId(494)], "pirate_generator_1": [SimThingId(501)], "pirate": [SimThingId(495)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
ok
test rehearsal_economy_fleet_native_funded_output_must_birth_fleets ... NATIVE_SOURCE_BEGIN case=native-funded-birth
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpDB7DUi\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:5f3a536f3eb15846:26980 targets={"pirate": [SimThingId(546)], "pirate_generator_1": [SimThingId(554)], "A1": [SimThingId(552)], "terran_shipyard": [SimThingId(547)], "terran_generator_2": [SimThingId(549)], "pirate_shipyard": [SimThingId(553)], "terran_refinery": [SimThingId(551)], "terran": [SimThingId(545)], "pirate_generator_2": [SimThingId(555)], "pirate_mine": [SimThingId(556)], "E1": [SimThingId(558)], "terran_generator_1": [SimThingId(548)], "terran_mine": [SimThingId(550)], "pirate_refinery": [SimThingId(557)], "stellaristhing_base": [SimThingId(573)]}
STOCK_CELL case=native-funded-birth generation=0 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=0 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=0 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=native-funded-birth generation=0 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=1 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=1 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=1 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=native-funded-birth generation=1 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=547 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=1 owner=pirate shipyard_id=553 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=2 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=2 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=2 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=native-funded-birth generation=2 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=2 owner=pirate shipyard_id=553 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=3 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=3 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=native-funded-birth generation=3 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=native-funded-birth generation=3 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=3 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=4 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=4 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=native-funded-birth generation=4 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=native-funded-birth generation=4 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=4 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=5 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=5 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=5 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=native-funded-birth generation=5 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
FUNDING_FLOW generation=5 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
NATIVE_BIRTH generation=6 parent=552 id=600 faction=0
NATIVE_BIRTH generation=6 parent=558 id=606 faction=1
STOCK_CELL case=native-funded-birth generation=6 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=6 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=6 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=6 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=6 owner=pirate shipyard_id=553 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=7 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=7 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=7 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=7 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=7 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=8 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=8 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=8 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=8 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=8 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=9 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=9 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=9 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=9 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(2)
FUNDING_FLOW generation=9 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(2)
NATIVE_BIRTH generation=10 parent=552 id=603 faction=0
STOCK_CELL case=native-funded-birth generation=10 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=10 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=10 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=10 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=547 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=10 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(3)
NATIVE_BIRTH generation=11 parent=558 id=609 faction=1
STOCK_CELL case=native-funded-birth generation=11 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=11 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=11 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=11 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=547 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=11 owner=pirate shipyard_id=553 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=12 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=12 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=12 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=12 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=12 owner=pirate shipyard_id=553 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=13 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=13 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=13 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=13 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=13 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=14 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=14 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=14 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=14 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=14 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=15 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=15 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=15 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=15 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=15 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=16 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=16 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=16 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=16 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=547 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=16 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=17 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=17 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=17 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=17 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=547 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=17 owner=pirate shipyard_id=553 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=18 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=18 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=18 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=18 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=18 owner=pirate shipyard_id=553 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=19 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=19 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=19 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=19 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=19 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=20 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=20 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=20 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=20 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=20 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=21 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=21 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=21 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=21 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=21 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=22 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=22 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=22 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=22 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=547 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=22 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=23 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=23 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=23 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=23 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=547 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=23 owner=pirate shipyard_id=553 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=24 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=24 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=24 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=24 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=24 owner=pirate shipyard_id=553 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=25 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=25 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=25 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=25 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=25 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=26 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=26 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=26 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=26 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=26 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=27 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=27 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=27 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=27 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=27 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=28 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=28 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=28 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=28 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=547 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=28 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=29 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=29 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=29 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=29 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=547 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=29 owner=pirate shipyard_id=553 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=30 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=30 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=30 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=30 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=30 owner=pirate shipyard_id=553 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=31 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=31 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=31 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=31 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=31 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=32 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=32 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=32 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=32 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=32 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=33 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=33 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=33 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=33 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=547 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=33 owner=pirate shipyard_id=553 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=34 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=34 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=34 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=34 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=547 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=34 owner=pirate shipyard_id=553 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=35 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=native-funded-birth generation=35 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=35 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=35 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=547 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=35 owner=pirate shipyard_id=553 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=36 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=native-funded-birth generation=36 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=36 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=36 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=36 owner=pirate shipyard_id=553 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=37 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=native-funded-birth generation=37 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=37 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=37 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=547 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=37 owner=pirate shipyard_id=553 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=38 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=native-funded-birth generation=38 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=38 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=38 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=547 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=38 owner=pirate shipyard_id=553 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=39 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=native-funded-birth generation=39 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=39 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=39 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=547 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=39 owner=pirate shipyard_id=553 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=40 host=terran_mine id=550 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=native-funded-birth generation=40 host=A1 id=552 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=40 host=pirate_mine id=556 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=40 host=E1 id=558 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=547 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=40 owner=pirate shipyard_id=553 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
BIRTH_SHAPE owner=terran id=600 children=[SimThingId(601), SimThingId(602)] hull=17179870000 extent=3 parent=552 generation=6
BIRTH_SHAPE owner=terran id=603 children=[SimThingId(604), SimThingId(605)] hull=1073741800 extent=3 parent=552 generation=10
BIRTH_SHAPE owner=pirate id=606 children=[SimThingId(607), SimThingId(608)] hull=17179870000 extent=3 parent=558 generation=6
BIRTH_SHAPE owner=pirate id=609 children=[SimThingId(610), SimThingId(611)] hull=536870900 extent=3 parent=558 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 572, 573, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597} fresh_ids=[600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611] capacity=76
ok
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpvSpbJz\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:b798441f83d9466c:21795 targets={"terran_generator_1": [SimThingId(614)], "terran": [SimThingId(612)], "terran_generator_2": [SimThingId(615)], "A1": [SimThingId(618)], "terran_refinery": [SimThingId(617)], "pirate": [SimThingId(613)], "terran_mine": [SimThingId(616)], "pirate_refinery": [SimThingId(622)], "pirate_generator_1": [SimThingId(619)], "pirate_mine": [SimThingId(621)], "stellaristhing_base": [SimThingId(636)], "E1": [SimThingId(623)], "pirate_generator_2": [SimThingId(620)]}
CELL case=minerals-then-energy generation=0 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [175, 58, 141, 10, 94, 189, 233, 75, 80, 198, 234, 11, 230, 94, 40, 89], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [175, 58, 141, 10, 94, 189, 233, 75, 80, 198, 234, 11, 230, 94, 40, 89], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpDA9bxf\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:43248e3fd4076521:21795 targets={"pirate_generator_1": [SimThingId(670)], "terran_generator_1": [SimThingId(665)], "pirate": [SimThingId(664)], "terran_generator_2": [SimThingId(666)], "terran": [SimThingId(663)], "pirate_refinery": [SimThingId(673)], "terran_refinery": [SimThingId(668)], "pirate_mine": [SimThingId(672)], "terran_mine": [SimThingId(667)], "E1": [SimThingId(674)], "A1": [SimThingId(669)], "pirate_generator_2": [SimThingId(671)], "stellaristhing_base": [SimThingId(687)]}
CELL case=energy-then-minerals generation=0 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [60, 95, 151, 100, 81, 158, 211, 58, 148, 79, 236, 140, 116, 209, 92, 207], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [60, 95, 151, 100, 81, 158, 211, 58, 148, 79, 236, 140, 116, 209, 92, 207], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.00s
```

## New stock-bound ingress focus

Exit 101. Source log `storage-focused.txt`.

```text
running 1 test
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpyD3OSu\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:829ca1bf28c6bc77:21574 targets={"terran_mine": [SimThingId(211)], "A1": [SimThingId(213)], "terran_generator_2": [SimThingId(210)], "terran_refinery": [SimThingId(212)], "pirate_generator_2": [SimThingId(215)], "pirate_mine": [SimThingId(216)], "E1": [SimThingId(218)], "pirate_generator_1": [SimThingId(214)], "stellaristhing_base": [SimThingId(231)], "terran": [SimThingId(207)], "pirate_refinery": [SimThingId(217)], "pirate": [SimThingId(208)], "terran_generator_1": [SimThingId(209)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=211 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=213 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=216 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=218 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpKf8ckm\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:8ec723a0163f0461:21573 targets={"terran": [SimThingId(258)], "pirate_refinery": [SimThingId(268)], "terran_generator_2": [SimThingId(261)], "terran_mine": [SimThingId(262)], "A1": [SimThingId(264)], "pirate_generator_1": [SimThingId(265)], "E1": [SimThingId(269)], "pirate_generator_2": [SimThingId(266)], "pirate_mine": [SimThingId(267)], "stellaristhing_base": [SimThingId(282)], "pirate": [SimThingId(259)], "terran_generator_1": [SimThingId(260)], "terran_refinery": [SimThingId(263)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=262 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=264 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=267 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=269 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
BOUNDED_STORAGE_INGRESS source=# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    properties = { property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance clamp = Bounded { min = 0 max = 24 } }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }
}


thread 'rehearsal_economy_fleet_generator_stock_preserves_frozen_economy' (43752) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:701:33:
2.2 STOP: native source cannot express the finite mineral Balance bound (0..24) needed by bounded storage: SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpnIi0kd\\stellaristhing_base.clause: ClauseThing hydration error at token 272: unsupported sub_field field `clamp`")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    rehearsal_economy_fleet_generator_stock_preserves_frozen_economy

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 4.37s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`
```

## Implementation-head parent full

Exit 101. Source log `storage-parent-full.txt`.

```text
running 5 tests
test rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow ... NATIVE_SOURCE_BEGIN case=zero-upkeep-control
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = 0 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp8saxwS\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:17106fd4b3959049:27258 targets={"pirate": [SimThingId(208)], "terran_generator_1": [SimThingId(210)], "terran_generator_2": [SimThingId(211)], "pirate_generator_2": [SimThingId(217)], "terran": [SimThingId(207)], "terran_mine": [SimThingId(212)], "terran_shipyard": [SimThingId(209)], "pirate_refinery": [SimThingId(219)], "pirate_shipyard": [SimThingId(215)], "A1": [SimThingId(214)], "E1": [SimThingId(220)], "pirate_generator_1": [SimThingId(216)], "stellaristhing_base": [SimThingId(235)], "pirate_mine": [SimThingId(218)], "terran_refinery": [SimThingId(213)]}
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=214 id=262
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=220 id=268
UPKEEP_BIRTH_ENROLLMENT case=zero-upkeep-control generation=6 non_carriers_excluded=true report=StructuralEnrollmentReport { added_roots: [SimThingId(262), SimThingId(268)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(262), arena_idx: 1, participant_slot: 38, parent: Some(SimThingId(214)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(268), arena_idx: 1, participant_slot: 41, parent: Some(SimThingId(220)) }], refusals: [], generation_before: 1, generation_after: 2 }
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=262 parent=214 slot=Some(SlotIndex(38)) authored_observed_flow=0 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(38), subtree_root: SimThingId(262), parent: Some(SimThingId(214)) }]
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=268 parent=220 slot=Some(SlotIndex(41)) authored_observed_flow=0 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(41), subtree_root: SimThingId(268), parent: Some(SimThingId(220)) }]
NATIVE_SOURCE_BEGIN case=one-energy-upkeep
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      property_value = { property = "meridian::energy" flow = -1 weight = 0 balance = 0 }
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp6pU2aP\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:6bfdbaf4447a36b6:27260 targets={"pirate_generator_2": [SimThingId(284)], "terran_generator_2": [SimThingId(278)], "pirate_shipyard": [SimThingId(282)], "terran": [SimThingId(274)], "terran_mine": [SimThingId(279)], "pirate": [SimThingId(275)], "A1": [SimThingId(281)], "stellaristhing_base": [SimThingId(302)], "pirate_mine": [SimThingId(285)], "E1": [SimThingId(287)], "pirate_generator_1": [SimThingId(283)], "terran_refinery": [SimThingId(280)], "terran_generator_1": [SimThingId(277)], "terran_shipyard": [SimThingId(276)], "pirate_refinery": [SimThingId(286)]}
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=281 id=329
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=287 id=335
UPKEEP_BIRTH_ENROLLMENT case=one-energy-upkeep generation=6 non_carriers_excluded=true report=StructuralEnrollmentReport { added_roots: [SimThingId(329), SimThingId(335)], admissions: [StructuralEnrollmentAdmission { simthing_id: SimThingId(329), arena_idx: 1, participant_slot: 38, parent: Some(SimThingId(281)) }, StructuralEnrollmentAdmission { simthing_id: SimThingId(335), arena_idx: 1, participant_slot: 41, parent: Some(SimThingId(287)) }], refusals: [], generation_before: 1, generation_after: 2 }
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(38), subtree_root: SimThingId(329), parent: Some(SimThingId(281)) }]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(41), subtree_root: SimThingId(335), parent: Some(SimThingId(287)) }]
ok
test rehearsal_economy_fleet_existing_asset_executes_without_profile_replacement ... SOURCE path=C:\Users\mvorm\SimThing-0088-economy-fleet\crates\simthing-workshop\../../scenarios/stellaristhing_base.clause identity=fnv1a64:ee4e4df9e8c9fbd9:5798 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=canonical: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=canonical identity=fnv1a64:6b7be7d3597feaf5:21555 targets={"terran_generator_1": [SimThingId(343)], "E1": [SimThingId(352)], "terran_generator_2": [SimThingId(344)], "pirate_refinery": [SimThingId(351)], "pirate_mine": [SimThingId(350)], "pirate_generator_1": [SimThingId(348)], "pirate": [SimThingId(342)], "pirate_generator_2": [SimThingId(349)], "terran_refinery": [SimThingId(346)], "terran": [SimThingId(341)], "terran_mine": [SimThingId(345)], "A1": [SimThingId(347)], "stellaristhing_base": [SimThingId(365)]}
N0 case=canonical root=364 existing_ids={341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 364, 365, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389} targets={"terran_generator_1": [SimThingId(343)], "E1": [SimThingId(352)], "terran_generator_2": [SimThingId(344)], "pirate_refinery": [SimThingId(351)], "pirate_mine": [SimThingId(350)], "pirate_generator_1": [SimThingId(348)], "pirate": [SimThingId(342)], "pirate_generator_2": [SimThingId(349)], "terran_refinery": [SimThingId(346)], "terran": [SimThingId(341)], "terran_mine": [SimThingId(345)], "A1": [SimThingId(347)], "stellaristhing_base": [SimThingId(365)]}
CELL case=canonical generation=0 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [127, 162, 166, 217, 236, 34, 72, 244, 7, 206, 241, 244, 129, 56, 174, 175], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [127, 162, 166, 217, 236, 34, 72, 244, 7, 206, 241, 244, 129, 56, 174, 175], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [127, 162, 166, 217, 236, 34, 72, 244, 7, 206, 241, 244, 129, 56, 174, 175], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [127, 162, 166, 217, 236, 34, 72, 244, 7, 206, 241, 244, 129, 56, 174, 175], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmppFUoht\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
RECIPES case=energy-withheld: Some(
    ResourceEconomySpec {
        transfers: [],
        recipes: [
            ResourceRecipeSpec {
                id: "material_conversion_recipe_terran_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "A1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "A1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "A1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
            ResourceRecipeSpec {
                id: "material_conversion_recipe_pirate_refining",
                inputs: [
                    RecipeInputSpec {
                        property: PropertyKey {
                            namespace: "meridian_material",
                            name: "E1_minerals_quantity",
                        },
                        role: Amount,
                        unit_cost: 2.0,
                        host_entity: Some(
                            "E1",
                        ),
                        host_span_token: None,
                    },
                ],
                target: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_alloys_quantity",
                },
                target_role: Amount,
                target_host_entity: Some(
                    "E1",
                ),
                target_host_span_token: None,
                output_coefficient: 1.0,
                order_band: 0,
                throttle_hint_max_per_tick: 1,
                max_units_per_generation: None,
            },
        ],
        emissions: [
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_terran_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "A1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "A1",
                ),
                host_span_token: Some(
                    404,
                ),
            },
            ResourceEmissionSpec {
                id: "material_conversion_quantity_emission_pirate_mining",
                source: PropertyKey {
                    namespace: "meridian_material",
                    name: "E1_minerals_quantity",
                },
                source_role: Amount,
                formula: Constant(
                    3.0,
                ),
                host_entity: Some(
                    "E1",
                ),
                host_span_token: Some(
                    436,
                ),
            },
        ],
        emit_on_threshold: [],
    },
)
PROFILE_FULL case=energy-withheld identity=fnv1a64:700947f4357e609c:21554 targets={"pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "terran_refinery": [SimThingId(397)], "A1": [SimThingId(398)], "pirate": [SimThingId(393)], "pirate_generator_2": [SimThingId(400)], "E1": [SimThingId(403)], "stellaristhing_base": [SimThingId(416)], "terran": [SimThingId(392)], "terran_generator_1": [SimThingId(394)], "pirate_mine": [SimThingId(401)], "terran_mine": [SimThingId(396)], "pirate_refinery": [SimThingId(402)]}
N0 case=energy-withheld root=415 existing_ids={392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 415, 416, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440} targets={"pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "terran_refinery": [SimThingId(397)], "A1": [SimThingId(398)], "pirate": [SimThingId(393)], "pirate_generator_2": [SimThingId(400)], "E1": [SimThingId(403)], "stellaristhing_base": [SimThingId(416)], "terran": [SimThingId(392)], "terran_generator_1": [SimThingId(394)], "pirate_mine": [SimThingId(401)], "terran_mine": [SimThingId(396)], "pirate_refinery": [SimThingId(402)]}
CELL case=energy-withheld generation=0 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [47, 40, 48, 57, 224, 65, 31, 35, 17, 102, 45, 207, 202, 127, 143, 13], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [47, 40, 48, 57, 224, 65, 31, 35, 17, 102, 45, 207, 202, 127, 143, 13], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [47, 40, 48, 57, 224, 65, 31, 35, 17, 102, 45, 207, 202, 127, 143, 13], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [47, 40, 48, 57, 224, 65, 31, 35, 17, 102, 45, 207, 202, 127, 143, 13], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp2AXTtN\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:f08262e0c86b6c9e:21574 targets={"stellaristhing_base": [SimThingId(467)], "pirate_generator_1": [SimThingId(450)], "terran_generator_2": [SimThingId(446)], "E1": [SimThingId(454)], "terran_generator_1": [SimThingId(445)], "terran_mine": [SimThingId(447)], "pirate_generator_2": [SimThingId(451)], "pirate_refinery": [SimThingId(453)], "pirate_mine": [SimThingId(452)], "terran_refinery": [SimThingId(448)], "pirate": [SimThingId(444)], "terran": [SimThingId(443)], "A1": [SimThingId(449)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=447 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=449 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=452 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=454 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpJuCPp4\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:ab8b1c636daf2e7e:21573 targets={"E1": [SimThingId(505)], "A1": [SimThingId(500)], "pirate": [SimThingId(495)], "pirate_generator_1": [SimThingId(501)], "pirate_mine": [SimThingId(503)], "terran": [SimThingId(494)], "terran_mine": [SimThingId(498)], "pirate_generator_2": [SimThingId(502)], "pirate_refinery": [SimThingId(504)], "terran_refinery": [SimThingId(499)], "stellaristhing_base": [SimThingId(518)], "terran_generator_2": [SimThingId(497)], "terran_generator_1": [SimThingId(496)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=498 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=500 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=503 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=505 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
BOUNDED_STORAGE_INGRESS source=# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    properties = { property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance clamp = Bounded { min = 0 max = 24 } }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }
}


thread 'rehearsal_economy_fleet_generator_stock_preserves_frozen_economy' (31532) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:701:33:
2.2 STOP: native source cannot express the finite mineral Balance bound (0..24) needed by bounded storage: SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpRcPFBl\\stellaristhing_base.clause: ClauseThing hydration error at token 272: unsupported sub_field field `clamp`")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test rehearsal_economy_fleet_native_funded_output_must_birth_fleets ... NATIVE_SOURCE_BEGIN case=native-funded-birth
# Meridian Arm, first Studio slice (0088-STUDIO-SLICE-0, Leaf A).
# Spatial residency, per-resource RF parentage, and ownership are separate below.
# One generation is one economic tick; Studio speed changes wall-clock pacing only.
@mine_rate = 3
@refinery_input = 2
@refinery_output = 1
@generator_rate = 2
@facility_upkeep = -1

scenario = stellaristhing_base {
  metadata = {
    display_name = "StellarisThing - Meridian Arm"
    description = "Two faction economies on the Meridian Arm. Energy supply and facility upkeep use native recursive RF; local mineral conversion uses admitted recipes."
  }
  static_galaxy_scenario = arm {
    namespace = meridian
    source_json = "stellaristhing_base.base.json"
    map_quality_status = PASS
  }

  # This common RF root has no external energy injection.
  property_value = { property = "meridian::energy" flow = 0 weight = 1 }
  property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
  owner = terran {
    display_name = "Terran Directorate"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = terran_industrial_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1
    } }
  }
  owner = pirate {
    display_name = "Pirate Compact"
    property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
    resource_parent = { property = "meridian::energy" parent = stellaristhing_base }
    resource_parent = { property = "meridian::minerals" parent = stellaristhing_base }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    overlays = { modifier = {
      id = pirate_supply_policy
      targets_property = "meridian::energy"
      sub_field = weight
      amount_mult = 1.5
    } }
  }

  planet_surface_payload = ambient {
    applies_to = neutral_systems
    planets_per_system_min = 1
    surface_grid = "1x1"
    factory_min = 0
    cohort_min = 0
  }

  # A1 is mapped to system A through STEAD, owned by Terran, RF-parented to Terran.
  location = A1 {
    name = "A1 - Directorate Works"
    system_target = row0_col0
    owner_ref = terran
    resource_parent = { property = "meridian::energy" parent = terran }
    resource_parent = { property = "meridian::minerals" parent = terran }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::A1_alloys_quantity" Amount = 4 }
    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = meridian_energy
      namespace = meridian
      name = energy
      display_name = "Energy"
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } }
    children = {
      child = terran_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = terran_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = terran_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = terran
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 20 }
      }
      child = terran_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = terran
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 10 }
      }
    }
  }

  # E1 is mapped to system E; its Owner is not its spatial parent.
  location = E1 {
    name = "E1 - Compact Works"
    system_target = row0_col4
    owner_ref = pirate
    resource_parent = { property = "meridian::energy" parent = pirate }
    resource_parent = { property = "meridian::minerals" parent = pirate }
    property_value = { property = "meridian::minerals" flow = 0 weight = 1 }
    property_value = { property = "meridian::energy" flow = 0 weight = 1 }
    # Quantity emission seeds the native mineral locus to @mine_rate at admission.
    property_value = { property = "meridian_material::E1_alloys_quantity" Amount = 3 }
    children = {
      child = pirate_shipyard { kind = Cohort name = "Shipyard energy WIP"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = 0 weight = 1 balance = 0 }
      }
      child = pirate_generator_1 { kind = Cohort name = "Generator 1"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_generator_2 { kind = Cohort name = "Generator 2"
        property_value = { property = "meridian::energy" flow = @generator_rate weight = 0 }
      }
      child = pirate_mine { kind = Cohort name = "Mine upkeep"
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 0 }
        owner_ref = pirate
        property_value = { property = "meridian::minerals" flow = @mine_rate weight = 1 balance = 14 }
      }
      child = pirate_refinery { kind = Cohort name = "Refinery upkeep"
        owner_ref = pirate
        property_value = { property = "meridian::energy" flow = @facility_upkeep weight = 1 balance = 8 }
      }
    }
  }

  # Material flows are local at their declared sites, with no cross-owner transfer.
  # Energy upkeep is a signed RF flow; material conversion is a separate local recipe.
  # The throttle is a hint, not a claim of a hard one-batch-per-generation cap.
  field_economy = material_conversion {
    namespace = meridian_material
    production_building = terran_corvette_funding {
      location = A1
      input = { resource = alloys amount = 6 }
      input = { entity = terran_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = terran_refining {
      location = A1
      input = { entity = terran_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = terran_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_corvette_funding {
      location = E1
      input = { resource = alloys amount = 6 }
      input = { entity = pirate_shipyard property = "meridian::energy" role = balance amount = 4 }
      output = { resource = corvette coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_refining {
      location = E1
      input = { entity = pirate_mine property = "meridian::minerals" role = balance amount = @refinery_input }
      input = { entity = pirate_refinery property = "meridian::energy" role = balance amount = 1 }
      output = { resource = alloys coefficient = @refinery_output }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
  }

  structural_product = terran_corvettes {
    funding = { entity = A1 property = "meridian_material::A1_corvette_quantity" role = Amount }
    count = 2
    parent = A1
    template = {
      kind = Fleet
      owner_ref = terran
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = terran_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }

  structural_product = pirate_corvettes {
    funding = { entity = E1 property = "meridian_material::E1_corvette_quantity" role = Amount }
    count = 2
    parent = E1
    template = {
      kind = Fleet
      owner_ref = pirate
      property_value = { property = "corvette::hull" Amount = 1 }
      overlays = { modifier = { id = pirate_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 } }
      children = {
        child = crew { kind = Cohort }
        child = engine { kind = Cohort }
      }
    }
  }
}

NATIVE_SOURCE_END
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpvKa2RQ\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:496a181f9588e9f0:26980 targets={"pirate_refinery": [SimThingId(559)], "pirate_generator_1": [SimThingId(556)], "terran": [SimThingId(547)], "terran_generator_2": [SimThingId(551)], "terran_refinery": [SimThingId(553)], "stellaristhing_base": [SimThingId(575)], "A1": [SimThingId(554)], "E1": [SimThingId(560)], "pirate_shipyard": [SimThingId(555)], "pirate_generator_2": [SimThingId(557)], "pirate_mine": [SimThingId(558)], "terran_mine": [SimThingId(552)], "terran_generator_1": [SimThingId(550)], "pirate": [SimThingId(548)], "terran_shipyard": [SimThingId(549)]}
STOCK_CELL case=native-funded-birth generation=0 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=0 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=0 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=native-funded-birth generation=0 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=1 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=1 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=1 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=native-funded-birth generation=1 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=549 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=1 owner=pirate shipyard_id=555 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=2 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=2 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=2 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=native-funded-birth generation=2 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=549 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=2 owner=pirate shipyard_id=555 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=3 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=3 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=native-funded-birth generation=3 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=native-funded-birth generation=3 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=549 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=3 owner=pirate shipyard_id=555 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=4 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=4 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=native-funded-birth generation=4 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=native-funded-birth generation=4 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=549 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=4 owner=pirate shipyard_id=555 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=5 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=5 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=5 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=native-funded-birth generation=5 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=549 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
FUNDING_FLOW generation=5 owner=pirate shipyard_id=555 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
NATIVE_BIRTH generation=6 parent=554 id=602 faction=0
NATIVE_BIRTH generation=6 parent=560 id=608 faction=1
STOCK_CELL case=native-funded-birth generation=6 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=6 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=6 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=6 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=549 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=6 owner=pirate shipyard_id=555 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=7 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=7 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=7 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=7 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=549 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=7 owner=pirate shipyard_id=555 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=8 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=8 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=8 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=8 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=549 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=8 owner=pirate shipyard_id=555 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=9 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=9 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=9 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=9 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=549 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(2)
FUNDING_FLOW generation=9 owner=pirate shipyard_id=555 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(2)
NATIVE_BIRTH generation=10 parent=554 id=605 faction=0
STOCK_CELL case=native-funded-birth generation=10 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=10 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=10 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=10 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=549 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=10 owner=pirate shipyard_id=555 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(3)
NATIVE_BIRTH generation=11 parent=560 id=611 faction=1
STOCK_CELL case=native-funded-birth generation=11 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=11 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=11 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=11 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=549 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=11 owner=pirate shipyard_id=555 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=12 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=12 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=12 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=12 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=549 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=12 owner=pirate shipyard_id=555 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=13 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=13 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=13 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=13 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=549 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=13 owner=pirate shipyard_id=555 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=14 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=14 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=14 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=14 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=549 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=14 owner=pirate shipyard_id=555 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=15 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=15 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=15 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=15 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=549 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=15 owner=pirate shipyard_id=555 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=16 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=16 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=16 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=16 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=549 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=16 owner=pirate shipyard_id=555 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=17 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=17 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=17 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=17 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=549 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=17 owner=pirate shipyard_id=555 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=18 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=18 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=18 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=18 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=549 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=18 owner=pirate shipyard_id=555 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=19 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=19 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=19 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=19 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=549 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=19 owner=pirate shipyard_id=555 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=20 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=20 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=20 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=20 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=549 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=20 owner=pirate shipyard_id=555 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=21 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=21 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=21 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=21 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=549 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=21 owner=pirate shipyard_id=555 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=22 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=22 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=22 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=22 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=549 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=22 owner=pirate shipyard_id=555 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=23 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=23 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=23 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=23 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=549 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=23 owner=pirate shipyard_id=555 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=24 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=24 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=24 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=24 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=549 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=24 owner=pirate shipyard_id=555 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=25 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=25 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=25 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=25 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=549 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=25 owner=pirate shipyard_id=555 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=26 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=26 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=26 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=26 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=549 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=26 owner=pirate shipyard_id=555 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=27 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=27 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=27 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=27 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=549 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=27 owner=pirate shipyard_id=555 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=28 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=28 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=28 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=28 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=549 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=28 owner=pirate shipyard_id=555 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=29 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=29 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=29 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=29 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=549 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=29 owner=pirate shipyard_id=555 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=30 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=30 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=30 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=30 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=549 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=30 owner=pirate shipyard_id=555 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=31 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=31 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=31 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=31 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=549 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=31 owner=pirate shipyard_id=555 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=32 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=32 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=32 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=32 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=549 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=32 owner=pirate shipyard_id=555 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=33 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=33 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=33 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=33 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=549 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=33 owner=pirate shipyard_id=555 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=34 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=34 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=34 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=34 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=549 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=34 owner=pirate shipyard_id=555 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=35 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=native-funded-birth generation=35 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=35 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=35 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=549 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=35 owner=pirate shipyard_id=555 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=36 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=native-funded-birth generation=36 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=36 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=36 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=549 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=36 owner=pirate shipyard_id=555 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=37 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=native-funded-birth generation=37 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=37 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=37 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=549 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=37 owner=pirate shipyard_id=555 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=38 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=native-funded-birth generation=38 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=38 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=38 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=549 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=38 owner=pirate shipyard_id=555 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=39 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=native-funded-birth generation=39 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=39 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=39 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=549 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=39 owner=pirate shipyard_id=555 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=40 host=terran_mine id=552 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=native-funded-birth generation=40 host=A1 id=554 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=40 host=pirate_mine id=558 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=40 host=E1 id=560 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=549 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=40 owner=pirate shipyard_id=555 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
BIRTH_SHAPE owner=terran id=602 children=[SimThingId(603), SimThingId(604)] hull=17179870000 extent=3 parent=554 generation=6
BIRTH_SHAPE owner=terran id=605 children=[SimThingId(606), SimThingId(607)] hull=1073741800 extent=3 parent=554 generation=10
BIRTH_SHAPE owner=pirate id=608 children=[SimThingId(609), SimThingId(610)] hull=17179870000 extent=3 parent=560 generation=6
BIRTH_SHAPE owner=pirate id=611 children=[SimThingId(612), SimThingId(613)] hull=536870900 extent=3 parent=560 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 560, 574, 575, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597, 598, 599} fresh_ids=[602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613] capacity=76
ok
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpJjzSJd\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:d829e14e24858c7b:21795 targets={"pirate_refinery": [SimThingId(624)], "E1": [SimThingId(625)], "stellaristhing_base": [SimThingId(638)], "terran_generator_1": [SimThingId(616)], "pirate_generator_2": [SimThingId(622)], "terran_generator_2": [SimThingId(617)], "terran_mine": [SimThingId(618)], "pirate_generator_1": [SimThingId(621)], "pirate": [SimThingId(615)], "A1": [SimThingId(620)], "terran": [SimThingId(614)], "terran_refinery": [SimThingId(619)], "pirate_mine": [SimThingId(623)]}
CELL case=minerals-then-energy generation=0 host=terran id=614 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=615 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=620 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=620 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=625 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=625 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [188, 2, 64, 112, 99, 56, 82, 56, 248, 212, 245, 22, 129, 173, 187, 89], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=614 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=615 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=620 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=620 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=625 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=625 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [188, 2, 64, 112, 99, 56, 82, 56, 248, 212, 245, 22, 129, 173, 187, 89], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpQgNAt9\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:caa1a7d5d564562f:21795 targets={"terran_refinery": [SimThingId(670)], "pirate_generator_1": [SimThingId(672)], "pirate_refinery": [SimThingId(675)], "E1": [SimThingId(676)], "terran_generator_1": [SimThingId(667)], "pirate_mine": [SimThingId(674)], "stellaristhing_base": [SimThingId(689)], "terran_generator_2": [SimThingId(668)], "pirate_generator_2": [SimThingId(673)], "terran_mine": [SimThingId(669)], "pirate": [SimThingId(666)], "A1": [SimThingId(671)], "terran": [SimThingId(665)]}
CELL case=energy-then-minerals generation=0 host=terran id=665 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=666 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=671 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=671 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=676 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=676 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [238, 148, 209, 69, 48, 182, 167, 44, 146, 243, 246, 76, 68, 204, 80, 216], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=665 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=666 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=671 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=671 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=676 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=676 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [238, 148, 209, 69, 48, 182, 167, 44, 146, 243, 246, 76, 68, 204, 80, 216], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

failures:

failures:
    rehearsal_economy_fleet_generator_stock_preserves_frozen_economy

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.26s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`
```

## First inventory correction

```text
TEST-INVENTORY-DRIFT-CHECK REPORT
  rows: 1120
  discovered: 1120
  unledgered: 0
  parked (pen, accounted): 0
  stale: 0
  promotion-target rows: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

## Post-probe inventory

```text
TEST-INVENTORY-DRIFT-CHECK REPORT
  rows: 1120
  discovered: 1120
  unledgered: 0
  parked (pen, accounted): 0
  stale: 0
  promotion-target rows: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS
```

## Ledger-head AGENT-SCAN

```text
DOCTRINE SCAN REPORT  (commit f975a647, 2026-09-19T20:29:30Z)
  scanner self-test: SKIPPED
  scan mode: PR delta (a14d4a57..f975a647)
  reliable scope: whole-tree
  heuristic scope: changed files / changed lines
  --- results ---
  FIELD-SWEEP-SINGLE-PATH-ALGEBRA  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; algebra is authored EML data and never an enum/tag/operator match in the sweep path
  FIELD-SWEEP-SINGLE-PATH-SHADERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; no eighth bespoke field shader in either production shader home beside the exact canonical generic interpreter
  FIELD-SWEEP-LEGACY-CALLERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; the seven retiring operators are migration referees only and have zero compiled production callers
  FIELD-SWEEP-DENSE-CAP-CROSSING  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; generic and sparse LinkGraph adjacency cannot inherit dense REGION_FIELD theater caps
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  TEST-BUDGET  PASS  0  design §0.9.5 test admission budget
  SPEC-LOWERER-KIND-READ  PASS  0  ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but closed-lowerer hits are higher suspicion because lowerers are constitutionally closed unless a DA-authorized amendment names them
  GUARD-KABUKI-TRIPWIRE  PASS  0  handoff_template section H + ci_screening_surface section 4; HEURISTIC tripwire for bespoke source-scanning guards and test-side include_str source greps; HC-6: symbol with well-formed FRESH HORIZON-ENTRY(iso-date): consumer/ref is EXEMPT (dated+assessable; unmarked/stale stay FLAGGED; never bare-token forever-pass); HC-8 accepted evasion residue: PRIVATE fn source-scanner or var-bound include_str! evades the pub-fn-anchored arms (DA review is the backstop; regex intentionally NOT widened — would false-fire on legit parsers); legitimate cases route to INSPECT triage, never FAIL
  EXECUTION-STATUS-UNCLASSIFIED  PASS  0  design 0.0.8.7 §3 Phase 0 EXECUTION-STATUS-TAXONOMY-0; HEURISTIC: execution-flavored driver/kernel surface missing from scripts/ci/execution_status_taxonomy.tsv (delta-scoped on PR)
  CELL-STORAGE-POLYMORPHISM  PASS  0  design 0.0.8.7 §2 P0(e) fence (i) CELL-STORAGE-POLYMORPHISM; HEURISTIC reach detector for tagged/templated/heterogeneous matrix-cell storage across production crates (workshop excluded)
  BESPOKE-PATHFINDER  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 PALMA BESPOKE-PATHFINDER; HEURISTIC: A* (BinaryHeap+came_from/g_score/open_set) OR ordinary Dijkstra (dist/distance+prev/predecessor with dijkstra/shortest_path/relax_edge) in production crates
  BORDER-SERVICE  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 Gu-Yang BORDER-SERVICE; HEURISTIC: border/frontline semantic service machinery (not mere presentation polyline projection/cache); mapeditor included for service-layer reaches
  OWNER-POLICY-WEIGHT-AUTHORITY-MINT  PASS  0  design 0.0.8.7 §3 Phase 3 FIRST-CITIZEN-SPECIALISTS-0; HEURISTIC: authored clause/scenario-JSON sources minting OWNER_POLICY_WEIGHT_AUTHORITY property id 8_300_318 outside hydration field-economy derivation (hydration-derived dumps excluded)
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 4 entries
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=61s
```

## Implementation-head AGENT-SCAN

```text
DOCTRINE SCAN REPORT  (commit 510d4cd3, 2026-09-19T20:31:43Z)
  scanner self-test: SKIPPED
  scan mode: PR delta (a14d4a57..510d4cd3)
  reliable scope: whole-tree
  heuristic scope: changed files / changed lines
  --- results ---
  FIELD-SWEEP-SINGLE-PATH-ALGEBRA  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; algebra is authored EML data and never an enum/tag/operator match in the sweep path
  FIELD-SWEEP-SINGLE-PATH-SHADERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; no eighth bespoke field shader in either production shader home beside the exact canonical generic interpreter
  FIELD-SWEEP-LEGACY-CALLERS  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; the seven retiring operators are migration referees only and have zero compiled production callers
  FIELD-SWEEP-DENSE-CAP-CROSSING  PASS  0  design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; generic and sparse LinkGraph adjacency cannot inherit dense REGION_FIELD theater caps
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  TEST-BUDGET  PASS  0  design §0.9.5 test admission budget
  SPEC-LOWERER-KIND-READ  PASS  0  ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but closed-lowerer hits are higher suspicion because lowerers are constitutionally closed unless a DA-authorized amendment names them
  GUARD-KABUKI-TRIPWIRE  PASS  0  handoff_template section H + ci_screening_surface section 4; HEURISTIC tripwire for bespoke source-scanning guards and test-side include_str source greps; HC-6: symbol with well-formed FRESH HORIZON-ENTRY(iso-date): consumer/ref is EXEMPT (dated+assessable; unmarked/stale stay FLAGGED; never bare-token forever-pass); HC-8 accepted evasion residue: PRIVATE fn source-scanner or var-bound include_str! evades the pub-fn-anchored arms (DA review is the backstop; regex intentionally NOT widened — would false-fire on legit parsers); legitimate cases route to INSPECT triage, never FAIL
  EXECUTION-STATUS-UNCLASSIFIED  PASS  0  design 0.0.8.7 §3 Phase 0 EXECUTION-STATUS-TAXONOMY-0; HEURISTIC: execution-flavored driver/kernel surface missing from scripts/ci/execution_status_taxonomy.tsv (delta-scoped on PR)
  CELL-STORAGE-POLYMORPHISM  PASS  0  design 0.0.8.7 §2 P0(e) fence (i) CELL-STORAGE-POLYMORPHISM; HEURISTIC reach detector for tagged/templated/heterogeneous matrix-cell storage across production crates (workshop excluded)
  BESPOKE-PATHFINDER  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 PALMA BESPOKE-PATHFINDER; HEURISTIC: A* (BinaryHeap+came_from/g_score/open_set) OR ordinary Dijkstra (dist/distance+prev/predecessor with dijkstra/shortest_path/relax_edge) in production crates
  BORDER-SERVICE  PASS  0  design 0.0.8.7 §4 TRIAD DOORS / P5 Gu-Yang BORDER-SERVICE; HEURISTIC: border/frontline semantic service machinery (not mere presentation polyline projection/cache); mapeditor included for service-layer reaches
  OWNER-POLICY-WEIGHT-AUTHORITY-MINT  PASS  0  design 0.0.8.7 §3 Phase 3 FIRST-CITIZEN-SPECIALISTS-0; HEURISTIC: authored clause/scenario-JSON sources minting OWNER_POLICY_WEIGHT_AUTHORITY property id 8_300_318 outside hydration field-economy derivation (hydration-derived dumps excluded)
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 4 entries
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=57s
```

## Scope / single-row diff / reflog

```text
INVENTORY_DELTA exactly one insertion, no deletions/other edits:
simthing-workshop	crates/simthing-workshop/tests/native_structural_products_session_0.rs	sequential_capacity_exhaustion_preserves_prior_births	integration	invariant-required	0088-ECONOMY-FLEET-0	AUDIT	catches: failure of the exact N+0.5 funded-candidate crossing, loss of full committed placement across exact zero-grant capacity refusal, or fabricated birth/identity/live-count/free-capacity mutation during sequential ordinary exhaustion	ledger-only	0.0.8.8-integrated-rehearsal	0
SEALED_COMPONENTS_UNCHANGED 32
HISTORICAL_PACKETS_UNCHANGED 10
NATIVE_CAPACITY_SOURCE_UNCHANGED true; CANONICAL_SCENARIO_UNCHANGED true
E8 0xee37_12f2_ef18_6934
HEAD 510d4cd3045d3e876e9dce67f4ba9787e4b8d9f6
TREE 9cdde7d866c4372c83c046cada45355a3dd41fde
BASE a14d4a57813952781ec475cfd32d7c4f80fadfda
510d4cd3045d3e876e9dce67f4ba9787e4b8d9f6 commit: test: expose native bounded stock authoring gap
f975a647ca243f94199919d1410fd7f08d40a60b commit: test: ledger the authorized native capacity exhaustion proof
ce632646400067d4de494ee6c488cc2c0e7a4c68 commit: docs: return RF capacity proof and explicit inventory scope STOP
4d6f43a7a968698ea6a4da74426d039731c0938d commit: test: harden born RF admission and salvage exact capacity proof
895803e4faae5c20ae998f7b0ab321ffbd7da588 rebase (finish): returning to refs/heads/codex/0088-economy-fleet
895803e4faae5c20ae998f7b0ab321ffbd7da588 rebase (pick): docs: record native birth proof and post-birth RF STOP
4339591b2ca72f650e29dbdf20bcfee6bb65e77c rebase (pick): test: prove native fleet births and expose missing RF enrollment
389bc3342094523ca37abebf5246811a0e6d6958 rebase (pick): docs: bind final table-driven 2.2 STOP execution
7e929a847ab5bec2ef9143c7952996e2e8f00dac rebase (pick): test: express capped recovery expectations as explicit case data
c0dd504f88ac58c3b37f6ef24d1706858d3d047d rebase (pick): docs: record capped economy GREEN and native funded-birth STOP
c2665bc8c16df736890bd64f0459ad08abc12617 rebase (pick): test: prove capped refinery and expose native funded-birth ingress gap
8953beb63e3a69562811e761ca0336090c3a852f rebase (pick): docs: record 2.2 resumed accounting and refinery throughput STOP
e8ee4282fb3012c6c9afe9d17b51e45f463ba949 rebase (pick): test: resume 2.2 conjunction and expose missing refinery rate cap
a86d0921565d4c8852f172e1abec3fce0baa64fe rebase (pick): docs: return 2.2 STOP with native recipe input-loss evidence
7c9b7cf4f747a15bd970504e2d616ba9062a23fc rebase (pick): test: expose 2.2 refinery authoring input loss on the Meridian asset
a14d4a57813952781ec475cfd32d7c4f80fadfda rebase (start): checkout a14d4a57813952781ec475cfd32d7c4f80fadfda
ddf62c55caac534f7a82e9f3eada9d5cd698652c commit: docs: record native birth proof and post-birth RF STOP
```
