# 2.2 economy-to-fleet ingress STOP: raw execution

Implementation/test head: `9cc68e55a4ee168beca1df2a4629a14c9b6bfe39`. Base: `34cf26df39cfd39b0f2a183f489604271f9253c0`.

Command: `cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1`.
Workshop's existing dev dependency enables `simthing-gpu/eml-resource-profiling`; no Cargo edit.
Exit **101**; **1 passed /1 failed /0 ignored /0 measured /0 filtered**, 5.09s.
The failing assertion is retained, not inverted or marked should_panic/ignored.

Full build+execution log SHA-256: `2ffae98ea5d030f45d511bff6a1e3d6708972f6077daf58c7107ef97637b03bd`.
Build completed in 3m25s; existing compiler warnings omitted below. The entire test execution follows, including all recorded IDs, slots, stocks, allocator census and failures. Realm bytes / SimThing IDs are per-run identities, not stable content digests.

```text
running 2 tests
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
N0 case=canonical root=230 existing_ids={207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 230, 231, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255} targets={"pirate_generator_2": [SimThingId(215)], "stellaristhing_base": [SimThingId(231)], "terran_generator_2": [SimThingId(210)], "pirate_refinery": [SimThingId(217)], "pirate_mine": [SimThingId(216)], "terran": [SimThingId(207)], "terran_generator_1": [SimThingId(209)], "terran_refinery": [SimThingId(212)], "pirate": [SimThingId(208)], "terran_mine": [SimThingId(211)], "A1": [SimThingId(213)], "E1": [SimThingId(218)], "pirate_generator_1": [SimThingId(214)]}
CELL case=canonical generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [151, 168, 217, 56, 125, 166, 101, 127, 186, 153, 161, 64, 198, 143, 92, 189], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [151, 168, 217, 56, 125, 166, 101, 127, 186, 153, 161, 64, 198, 143, 92, 189], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [151, 168, 217, 56, 125, 166, 101, 127, 186, 153, 161, 64, 198, 143, 92, 189], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [151, 168, 217, 56, 125, 166, 101, 127, 186, 153, 161, 64, 198, 143, 92, 189], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpTvLNeb\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
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
N0 case=energy-withheld root=281 existing_ids={258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 281, 282, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306} targets={"pirate": [SimThingId(259)], "pirate_generator_1": [SimThingId(265)], "stellaristhing_base": [SimThingId(282)], "E1": [SimThingId(269)], "terran_generator_2": [SimThingId(261)], "A1": [SimThingId(264)], "pirate_generator_2": [SimThingId(266)], "pirate_mine": [SimThingId(267)], "terran_refinery": [SimThingId(263)], "terran_generator_1": [SimThingId(260)], "pirate_refinery": [SimThingId(268)], "terran": [SimThingId(258)], "terran_mine": [SimThingId(262)]}
CELL case=energy-withheld generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [108, 148, 56, 52, 74, 19, 111, 92, 11, 158, 4, 128, 86, 40, 54, 123], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [108, 148, 56, 52, 74, 19, 111, 92, 11, 158, 4, 128, 86, 40, 54, 123], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [108, 148, 56, 52, 74, 19, 111, 92, 11, 158, 4, 128, 86, 40, 54, 123], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [108, 148, 56, 52, 74, 19, 111, 92, 11, 158, 4, 128, 86, 40, 54, 123], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpcGglac\stellaristhing_base.clause identity=fnv1a64:d7198602c73a6ee7:5892 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:d44ccbe88ade842a:10760
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian_material", "A1_energy_quantity", 1.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian_material", "E1_energy_quantity", 1.0, Some("E1"), Amount)]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpedXxWi\stellaristhing_base.clause identity=fnv1a64:ae8a448726a37b6b:5892 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:beaa4408e5b3f4aa:8892
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]

thread 'rehearsal_economy_fleet_refinery_retains_every_authored_cost' (19292) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:239:5:
2.2 STOP: refinery input conjunction lost by native ingestion: [
    "minerals-then-energy/material_conversion_recipe_terran_refining: authored minerals=2 AND energy=1; retained [(\"meridian_material\", \"A1_energy_quantity\", 1.0, Some(\"A1\"), Amount)]",
    "minerals-then-energy/material_conversion_recipe_pirate_refining: authored minerals=2 AND energy=1; retained [(\"meridian_material\", \"E1_energy_quantity\", 1.0, Some(\"E1\"), Amount)]",
    "energy-then-minerals/material_conversion_recipe_terran_refining: authored minerals=2 AND energy=1; retained [(\"meridian_material\", \"A1_minerals_quantity\", 2.0, Some(\"A1\"), Amount)]",
    "energy-then-minerals/material_conversion_recipe_pirate_refining: authored minerals=2 AND energy=1; retained [(\"meridian_material\", \"E1_minerals_quantity\", 2.0, Some(\"E1\"), Amount)]",
]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    rehearsal_economy_fleet_refinery_retains_every_authored_cost

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.09s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`
```
