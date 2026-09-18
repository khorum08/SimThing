# 2.2 resume raw execution

Code proof head: `c88d1b1ba9a9afd17fff355e337e00ee1491cfb8`.
Base: `219ca81361dc73a15b06bf93d47697b4e42d198e`; fifteenth E8 pin unchanged.
Toolchain compilation warnings omitted; test output and all counts retained.

## Conjunction-first gate (executed before any resumed economy probe)

Command: `cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_refinery_retains_every_authored_cost -- --exact --nocapture --test-threads=1`

```text
running 1 test
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpQJeelv\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
CELL case=minerals-then-energy generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 103, 148, 114, 198, 224, 162, 45, 13, 89, 200, 116, 117, 1, 173, 108], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 103, 148, 114, 198, 224, 162, 45, 13, 89, 200, 116, 117, 1, 173, 108], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpaPM2BI\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
CELL case=energy-then-minerals generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [8, 120, 255, 137, 164, 210, 162, 77, 114, 88, 237, 86, 179, 100, 153, 183], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [8, 120, 255, 137, 164, 210, 162, 77, 114, 88, 237, 86, 179, 100, 153, 183], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 8.01s
```

## Full focused suite at the code proof head

Command: `cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1`

```text
running 3 tests
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
PROFILE_FULL case=canonical identity=fnv1a64:8abc7eba9742e501:21555 targets={"A1": [SimThingId(213)], "pirate": [SimThingId(208)], "pirate_generator_1": [SimThingId(214)], "pirate_mine": [SimThingId(216)], "stellaristhing_base": [SimThingId(231)], "terran": [SimThingId(207)], "E1": [SimThingId(218)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "terran_mine": [SimThingId(211)], "terran_refinery": [SimThingId(212)], "pirate_refinery": [SimThingId(217)], "pirate_generator_2": [SimThingId(215)]}
N0 case=canonical root=230 existing_ids={207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 230, 231, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255} targets={"A1": [SimThingId(213)], "pirate": [SimThingId(208)], "pirate_generator_1": [SimThingId(214)], "pirate_mine": [SimThingId(216)], "stellaristhing_base": [SimThingId(231)], "terran": [SimThingId(207)], "E1": [SimThingId(218)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "terran_mine": [SimThingId(211)], "terran_refinery": [SimThingId(212)], "pirate_refinery": [SimThingId(217)], "pirate_generator_2": [SimThingId(215)]}
CELL case=canonical generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [75, 156, 38, 198, 107, 210, 25, 20, 69, 87, 125, 76, 83, 34, 242, 141], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [75, 156, 38, 198, 107, 210, 25, 20, 69, 87, 125, 76, 83, 34, 242, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [75, 156, 38, 198, 107, 210, 25, 20, 69, 87, 125, 76, 83, 34, 242, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [75, 156, 38, 198, 107, 210, 25, 20, 69, 87, 125, 76, 83, 34, 242, 141], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp6c5DMS\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
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
PROFILE_FULL case=energy-withheld identity=fnv1a64:eb0d82d3d7779337:21554 targets={"pirate_refinery": [SimThingId(268)], "stellaristhing_base": [SimThingId(282)], "pirate_generator_2": [SimThingId(266)], "pirate": [SimThingId(259)], "terran_generator_1": [SimThingId(260)], "terran_mine": [SimThingId(262)], "terran_refinery": [SimThingId(263)], "terran_generator_2": [SimThingId(261)], "A1": [SimThingId(264)], "terran": [SimThingId(258)], "pirate_mine": [SimThingId(267)], "pirate_generator_1": [SimThingId(265)], "E1": [SimThingId(269)]}
N0 case=energy-withheld root=281 existing_ids={258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 281, 282, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306} targets={"pirate_refinery": [SimThingId(268)], "stellaristhing_base": [SimThingId(282)], "pirate_generator_2": [SimThingId(266)], "pirate": [SimThingId(259)], "terran_generator_1": [SimThingId(260)], "terran_mine": [SimThingId(262)], "terran_refinery": [SimThingId(263)], "terran_generator_2": [SimThingId(261)], "A1": [SimThingId(264)], "terran": [SimThingId(258)], "pirate_mine": [SimThingId(267)], "pirate_generator_1": [SimThingId(265)], "E1": [SimThingId(269)]}
CELL case=energy-withheld generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [64, 197, 216, 35, 52, 182, 125, 117, 216, 171, 3, 79, 231, 251, 144, 241], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [64, 197, 216, 35, 52, 182, 125, 117, 216, 171, 3, 79, 231, 251, 144, 241], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [64, 197, 216, 35, 52, 182, 125, 117, 216, 171, 3, 79, 231, 251, 144, 241], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [64, 197, 216, 35, 52, 182, 125, 117, 216, 171, 3, 79, 231, 251, 144, 241], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpGEM1xE\stellaristhing_base.clause identity=fnv1a64:9f9d1af572ab41c8:7420 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:7327633ef9f77495:7632
PROFILE_FULL case=generator-stock identity=fnv1a64:80046281ec23826e:21516 targets={"pirate": [SimThingId(310)], "A1": [SimThingId(315)], "pirate_generator_1": [SimThingId(316)], "pirate_refinery": [SimThingId(319)], "stellaristhing_base": [SimThingId(333)], "terran_mine": [SimThingId(313)], "terran_generator_2": [SimThingId(312)], "pirate_generator_2": [SimThingId(317)], "terran": [SimThingId(309)], "pirate_mine": [SimThingId(318)], "terran_refinery": [SimThingId(314)], "E1": [SimThingId(320)], "terran_generator_1": [SimThingId(311)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=1 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=14
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=1 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=10 energy_after=2 minerals_before=20 minerals_after=3 alloys_before=4 alloys_after=14
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=7 energy_after=3 minerals_before=14 minerals_after=3 alloys_before=3 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=2 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=15
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=2 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=3 minerals_after=4 alloys_before=14 alloys_after=15
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=3 minerals_after=4 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=3 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=17
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=3 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=13
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=3 settled=2 consumed=2 energy_after=3 minerals_before=4 minerals_after=3 alloys_before=15 alloys_after=17
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=4 settled=2 consumed=2 energy_after=4 minerals_before=4 minerals_after=3 alloys_before=11 alloys_after=13
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=4 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=18
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=4 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=14
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=3 minerals_after=4 alloys_before=17 alloys_after=18
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=3 minerals_after=4 alloys_before=13 alloys_after=14
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=5 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=20
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=3
STOCK_CELL case=generator-stock generation=5 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=16
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=4 settled=2 consumed=2 energy_after=4 minerals_before=4 minerals_after=3 alloys_before=18 alloys_after=20
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=5 settled=2 consumed=2 energy_after=5 minerals_before=4 minerals_after=3 alloys_before=14 alloys_after=16
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=6 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=21
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=4
STOCK_CELL case=generator-stock generation=6 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=17
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=3 minerals_after=4 alloys_before=20 alloys_after=21
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=5 settled=2 consumed=1 energy_after=6 minerals_before=3 minerals_after=4 alloys_before=16 alloys_after=17
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpmn5TX8\stellaristhing_base.clause identity=fnv1a64:96987e754f036b70:7419 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:7327633ef9f77495:7632
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:426b9df723cc19c6:21515 targets={"E1": [SimThingId(371)], "pirate_mine": [SimThingId(369)], "stellaristhing_base": [SimThingId(384)], "terran": [SimThingId(360)], "terran_refinery": [SimThingId(365)], "pirate": [SimThingId(361)], "pirate_generator_1": [SimThingId(367)], "terran_generator_1": [SimThingId(362)], "pirate_generator_2": [SimThingId(368)], "terran_generator_2": [SimThingId(363)], "terran_mine": [SimThingId(364)], "pirate_refinery": [SimThingId(370)], "A1": [SimThingId(366)]}
AUTHORED_N0 case=generator-withheld-restored site=terran_mine minerals=20
AUTHORED_N0 case=generator-withheld-restored site=pirate_mine minerals=14
STOCK_CELL case=generator-withheld-restored generation=0 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=0 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=0 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-withheld-restored generation=0 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-withheld-restored energy=[0.0, 0.0]
STOCK_CELL case=generator-withheld-restored generation=1 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=1 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=1 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-withheld-restored generation=1 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=1 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=1 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=14 minerals_after=17 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=1 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=2 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=2 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=2 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-withheld-restored generation=2 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=2 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=2 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=17 minerals_after=20 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=2 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=3 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=3 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=3 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-withheld-restored generation=3 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=3 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=26 minerals_after=29 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=3 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=20 minerals_after=23 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=3 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=4 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=4 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=4 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-withheld-restored generation=4 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=4 owner=terran energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=29 minerals_after=32 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=terran authored_effective_flow=[1.0, 1.0]
STOCK_FLOW case=generator-withheld-restored generation=4 owner=pirate energy_before=0 settled=0 consumed=0 energy_after=0 minerals_before=23 minerals_after=26 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=4 owner=pirate authored_effective_flow=[1.0, 1.0]
STOCK_CELL case=generator-withheld-restored generation=5 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=generator-withheld-restored generation=5 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-withheld-restored generation=5 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=generator-withheld-restored generation=5 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_FLOW case=generator-withheld-restored generation=5 owner=terran energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=32 minerals_after=35 alloys_before=4 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=5 owner=pirate energy_before=0 settled=2 consumed=0 energy_after=2 minerals_before=26 minerals_after=29 alloys_before=3 alloys_after=3
GENERATOR_FLOW case=generator-withheld-restored generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=2 energy_after=2 minerals_before=35 minerals_after=34 alloys_before=4 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=2 energy_after=2 minerals_before=29 minerals_after=28 alloys_before=3 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]

thread 'rehearsal_economy_fleet_generator_stock_preserves_frozen_economy' (23212) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:644:5:
2.2 STOP: frozen refinery throughput not preserved: [
    "generator-stock/G1/terran: 10 alloys, frozen maximum 1",
    "generator-stock/G1/pirate: 7 alloys, frozen maximum 1",
    "generator-stock/G3/terran: 2 alloys, frozen maximum 1",
    "generator-stock/G3/pirate: 2 alloys, frozen maximum 1",
    "generator-stock/G5/terran: 2 alloys, frozen maximum 1",
    "generator-stock/G5/pirate: 2 alloys, frozen maximum 1",
    "generator-withheld-restored/G6/terran: 2 alloys, frozen maximum 1",
    "generator-withheld-restored/G6/pirate: 2 alloys, frozen maximum 1",
]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpjo7gzc\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:904658dbb620d9d8:21795 targets={"terran_generator_1": [SimThingId(413)], "terran_mine": [SimThingId(415)], "pirate_generator_1": [SimThingId(418)], "terran_refinery": [SimThingId(416)], "terran_generator_2": [SimThingId(414)], "pirate_mine": [SimThingId(420)], "pirate_generator_2": [SimThingId(419)], "pirate_refinery": [SimThingId(421)], "stellaristhing_base": [SimThingId(435)], "E1": [SimThingId(422)], "pirate": [SimThingId(412)], "terran": [SimThingId(411)], "A1": [SimThingId(417)]}
CELL case=minerals-then-energy generation=0 host=terran id=411 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=412 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=417 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=417 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=422 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=422 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [204, 101, 4, 152, 255, 159, 21, 2, 92, 125, 102, 34, 78, 213, 235, 133], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=411 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=412 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=417 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=417 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=422 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=422 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [204, 101, 4, 152, 255, 159, 21, 2, 92, 125, 102, 34, 78, 213, 235, 133], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpUhReyL\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:1bed02f338f2cace:21795 targets={"terran_generator_2": [SimThingId(465)], "terran_mine": [SimThingId(466)], "pirate": [SimThingId(463)], "terran_refinery": [SimThingId(467)], "pirate_generator_1": [SimThingId(469)], "terran_generator_1": [SimThingId(464)], "terran": [SimThingId(462)], "stellaristhing_base": [SimThingId(486)], "A1": [SimThingId(468)], "pirate_mine": [SimThingId(471)], "pirate_generator_2": [SimThingId(470)], "pirate_refinery": [SimThingId(472)], "E1": [SimThingId(473)]}
CELL case=energy-then-minerals generation=0 host=terran id=462 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=463 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=468 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=468 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=473 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=473 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [44, 12, 157, 55, 173, 57, 12, 63, 38, 28, 133, 52, 184, 242, 85, 102], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=462 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=463 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=468 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=468 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=473 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=473 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [44, 12, 157, 55, 173, 57, 12, 63, 38, 28, 133, 52, 184, 242, 85, 102], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

failures:

failures:
    rehearsal_economy_fleet_generator_stock_preserves_frozen_economy

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.47s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`
```
