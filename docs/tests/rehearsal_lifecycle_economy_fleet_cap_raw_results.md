# 2.2 cap resume: raw execution

Code-head binding: `71a0b6e410f2cf5316f6f387cda9451a740540b4`; exact base `0e5ff59668b737f77a4de067a113175e11735949`.
Warnings/build chatter omitted; execution output below is unedited. The final
PR body separately binds the documentation-head rerun and hosted artifacts.
Earlier RED packets remain byte-unchanged in their historical companion files.

## Conjunction FIRST, before cap adaptation

```text
running 1 test
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpDTscvs\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:10dec9a6665111c2:21795 targets={"terran": [SimThingId(207)], "terran_generator_2": [SimThingId(210)], "E1": [SimThingId(218)], "terran_refinery": [SimThingId(212)], "pirate": [SimThingId(208)], "pirate_generator_1": [SimThingId(214)], "pirate_mine": [SimThingId(216)], "stellaristhing_base": [SimThingId(231)], "pirate_generator_2": [SimThingId(215)], "A1": [SimThingId(213)], "terran_mine": [SimThingId(211)], "pirate_refinery": [SimThingId(217)], "terran_generator_1": [SimThingId(209)]}
CELL case=minerals-then-energy generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [32, 244, 141, 201, 76, 142, 251, 4, 21, 227, 19, 97, 88, 142, 120, 246], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [32, 244, 141, 201, 76, 142, 251, 4, 21, 227, 19, 97, 88, 142, 120, 246], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp5IHwlz\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:66a129abc448fe0b:21795 targets={"terran": [SimThingId(258)], "pirate_refinery": [SimThingId(268)], "terran_generator_1": [SimThingId(260)], "A1": [SimThingId(264)], "pirate": [SimThingId(259)], "terran_generator_2": [SimThingId(261)], "terran_refinery": [SimThingId(263)], "pirate_generator_2": [SimThingId(266)], "stellaristhing_base": [SimThingId(282)], "terran_mine": [SimThingId(262)], "E1": [SimThingId(269)], "pirate_mine": [SimThingId(267)], "pirate_generator_1": [SimThingId(265)]}
CELL case=energy-then-minerals generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [173, 2, 144, 31, 250, 87, 57, 223, 80, 104, 66, 119, 153, 155, 185, 232], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [173, 2, 144, 31, 250, 87, 57, 223, 80, 104, 66, 119, 153, 155, 185, 232], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 3.44s


```

## Isolated capped-stock gate

```text
running 1 test
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpiPbG0X\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:08752738a1077075:21574 targets={"terran_generator_1": [SimThingId(209)], "terran_mine": [SimThingId(211)], "pirate": [SimThingId(208)], "terran": [SimThingId(207)], "pirate_mine": [SimThingId(216)], "terran_refinery": [SimThingId(212)], "terran_generator_2": [SimThingId(210)], "stellaristhing_base": [SimThingId(231)], "E1": [SimThingId(218)], "A1": [SimThingId(213)], "pirate_generator_2": [SimThingId(215)], "pirate_generator_1": [SimThingId(214)], "pirate_refinery": [SimThingId(217)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpem3DNi\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:1bab1bad73250c37:21573 targets={"stellaristhing_base": [SimThingId(282)], "E1": [SimThingId(269)], "terran_refinery": [SimThingId(263)], "terran_mine": [SimThingId(262)], "A1": [SimThingId(264)], "pirate_mine": [SimThingId(267)], "pirate": [SimThingId(259)], "pirate_generator_2": [SimThingId(266)], "terran_generator_1": [SimThingId(260)], "pirate_refinery": [SimThingId(268)], "terran_generator_2": [SimThingId(261)], "pirate_generator_1": [SimThingId(265)], "terran": [SimThingId(258)]}
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
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 2.32s


```

## Entire focused target at the committed code head

```text
running 4 tests
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
PROFILE_FULL case=canonical identity=fnv1a64:143ee9ebab611ca7:21555 targets={"E1": [SimThingId(218)], "pirate_generator_2": [SimThingId(215)], "terran_refinery": [SimThingId(212)], "pirate_generator_1": [SimThingId(214)], "pirate_refinery": [SimThingId(217)], "pirate_mine": [SimThingId(216)], "stellaristhing_base": [SimThingId(231)], "A1": [SimThingId(213)], "terran": [SimThingId(207)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "terran_mine": [SimThingId(211)], "pirate": [SimThingId(208)]}
N0 case=canonical root=230 existing_ids={207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 230, 231, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255} targets={"E1": [SimThingId(218)], "pirate_generator_2": [SimThingId(215)], "terran_refinery": [SimThingId(212)], "pirate_generator_1": [SimThingId(214)], "pirate_refinery": [SimThingId(217)], "pirate_mine": [SimThingId(216)], "stellaristhing_base": [SimThingId(231)], "A1": [SimThingId(213)], "terran": [SimThingId(207)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "terran_mine": [SimThingId(211)], "pirate": [SimThingId(208)]}
CELL case=canonical generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [93, 182, 252, 71, 103, 54, 186, 53, 103, 170, 29, 61, 161, 151, 130, 220], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [93, 182, 252, 71, 103, 54, 186, 53, 103, 170, 29, 61, 161, 151, 130, 220], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [93, 182, 252, 71, 103, 54, 186, 53, 103, 170, 29, 61, 161, 151, 130, 220], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [93, 182, 252, 71, 103, 54, 186, 53, 103, 170, 29, 61, 161, 151, 130, 220], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpzr2kC1\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
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
PROFILE_FULL case=energy-withheld identity=fnv1a64:14d71d1a4d404ff5:21554 targets={"pirate_refinery": [SimThingId(268)], "terran_refinery": [SimThingId(263)], "stellaristhing_base": [SimThingId(282)], "terran": [SimThingId(258)], "A1": [SimThingId(264)], "E1": [SimThingId(269)], "pirate": [SimThingId(259)], "pirate_generator_1": [SimThingId(265)], "pirate_generator_2": [SimThingId(266)], "pirate_mine": [SimThingId(267)], "terran_generator_1": [SimThingId(260)], "terran_generator_2": [SimThingId(261)], "terran_mine": [SimThingId(262)]}
N0 case=energy-withheld root=281 existing_ids={258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 281, 282, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306} targets={"pirate_refinery": [SimThingId(268)], "terran_refinery": [SimThingId(263)], "stellaristhing_base": [SimThingId(282)], "terran": [SimThingId(258)], "A1": [SimThingId(264)], "E1": [SimThingId(269)], "pirate": [SimThingId(259)], "pirate_generator_1": [SimThingId(265)], "pirate_generator_2": [SimThingId(266)], "pirate_mine": [SimThingId(267)], "terran_generator_1": [SimThingId(260)], "terran_generator_2": [SimThingId(261)], "terran_mine": [SimThingId(262)]}
CELL case=energy-withheld generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 239, 225, 50, 30, 249, 114, 20, 73, 13, 69, 28, 80, 147, 73, 199], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 239, 225, 50, 30, 249, 114, 20, 73, 13, 69, 28, 80, 147, 73, 199], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 239, 225, 50, 30, 249, 114, 20, 73, 13, 69, 28, 80, 147, 73, 199], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [74, 239, 225, 50, 30, 249, 114, 20, 73, 13, 69, 28, 80, 147, 73, 199], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpxyxZMg\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:c41483b17d8d7520:21574 targets={"pirate_generator_2": [SimThingId(317)], "A1": [SimThingId(315)], "pirate_mine": [SimThingId(318)], "E1": [SimThingId(320)], "terran": [SimThingId(309)], "pirate_generator_1": [SimThingId(316)], "stellaristhing_base": [SimThingId(333)], "terran_generator_2": [SimThingId(312)], "terran_refinery": [SimThingId(314)], "terran_mine": [SimThingId(313)], "pirate": [SimThingId(310)], "pirate_refinery": [SimThingId(319)], "terran_generator_1": [SimThingId(311)]}
AUTHORED_N0 case=generator-stock site=terran_mine minerals=20
AUTHORED_N0 case=generator-stock site=pirate_mine minerals=14
STOCK_CELL case=generator-stock generation=0 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=0 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=generator-stock generation=0 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=generator-stock generation=0 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_N0 case=generator-stock energy=[10.0, 8.0]
STOCK_CELL case=generator-stock generation=1 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=1 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-stock generation=1 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=generator-stock generation=1 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-stock generation=1 owner=terran energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=20 minerals_after=21 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=1 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=1 owner=pirate energy_before=8 settled=2 consumed=1 energy_after=9 minerals_before=14 minerals_after=15 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-stock generation=1 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=2 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=2 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-stock generation=2 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=generator-stock generation=2 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-stock generation=2 owner=terran energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=21 minerals_after=22 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=2 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=2 owner=pirate energy_before=9 settled=2 consumed=1 energy_after=10 minerals_before=15 minerals_after=16 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-stock generation=2 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=3 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=generator-stock generation=3 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-stock generation=3 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=generator-stock generation=3 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-stock generation=3 owner=terran energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=22 minerals_after=23 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=3 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=3 owner=pirate energy_before=10 settled=2 consumed=1 energy_after=11 minerals_before=16 minerals_after=17 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-stock generation=3 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=4 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=generator-stock generation=4 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=generator-stock generation=4 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=generator-stock generation=4 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
STOCK_FLOW case=generator-stock generation=4 owner=terran energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=23 minerals_after=24 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=4 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=4 owner=pirate energy_before=11 settled=2 consumed=1 energy_after=12 minerals_before=17 minerals_after=18 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-stock generation=4 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=5 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=generator-stock generation=5 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=9
STOCK_CELL case=generator-stock generation=5 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=generator-stock generation=5 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=8
STOCK_FLOW case=generator-stock generation=5 owner=terran energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=24 minerals_after=25 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=5 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=5 owner=pirate energy_before=12 settled=2 consumed=1 energy_after=13 minerals_before=18 minerals_after=19 alloys_before=7 alloys_after=8
GENERATOR_FLOW case=generator-stock generation=5 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=6 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=generator-stock generation=6 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=10
STOCK_CELL case=generator-stock generation=6 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=generator-stock generation=6 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=9
STOCK_FLOW case=generator-stock generation=6 owner=terran energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=25 minerals_after=26 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=6 owner=pirate energy_before=13 settled=2 consumed=1 energy_after=14 minerals_before=19 minerals_after=20 alloys_before=8 alloys_after=9
GENERATOR_FLOW case=generator-stock generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=7 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=generator-stock generation=7 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=11
STOCK_CELL case=generator-stock generation=7 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=generator-stock generation=7 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=10
STOCK_FLOW case=generator-stock generation=7 owner=terran energy_before=16 settled=2 consumed=1 energy_after=17 minerals_before=26 minerals_after=27 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=7 owner=pirate energy_before=14 settled=2 consumed=1 energy_after=15 minerals_before=20 minerals_after=21 alloys_before=9 alloys_after=10
GENERATOR_FLOW case=generator-stock generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-stock generation=8 host=terran_mine id=313 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=generator-stock generation=8 host=A1 id=315 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=12
STOCK_CELL case=generator-stock generation=8 host=pirate_mine id=318 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=generator-stock generation=8 host=E1 id=320 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=11
STOCK_FLOW case=generator-stock generation=8 owner=terran energy_before=17 settled=2 consumed=1 energy_after=18 minerals_before=27 minerals_after=28 alloys_before=11 alloys_after=12
GENERATOR_FLOW case=generator-stock generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-stock generation=8 owner=pirate energy_before=15 settled=2 consumed=1 energy_after=16 minerals_before=21 minerals_after=22 alloys_before=10 alloys_after=11
GENERATOR_FLOW case=generator-stock generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpELMmJk\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:0c82b6be168297fe:21573 targets={"A1": [SimThingId(366)], "stellaristhing_base": [SimThingId(384)], "terran_generator_1": [SimThingId(362)], "pirate_mine": [SimThingId(369)], "pirate": [SimThingId(361)], "E1": [SimThingId(371)], "pirate_refinery": [SimThingId(370)], "terran_generator_2": [SimThingId(363)], "terran_mine": [SimThingId(364)], "pirate_generator_1": [SimThingId(367)], "terran": [SimThingId(360)], "terran_refinery": [SimThingId(365)], "pirate_generator_2": [SimThingId(368)]}
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
STOCK_CELL case=generator-withheld-restored generation=6 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=generator-withheld-restored generation=6 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=generator-withheld-restored generation=6 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=generator-withheld-restored generation=6 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
STOCK_FLOW case=generator-withheld-restored generation=6 owner=terran energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=35 minerals_after=36 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=6 owner=pirate energy_before=2 settled=2 consumed=1 energy_after=3 minerals_before=29 minerals_after=30 alloys_before=3 alloys_after=4
GENERATOR_FLOW case=generator-withheld-restored generation=6 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=7 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=generator-withheld-restored generation=7 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=generator-withheld-restored generation=7 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=generator-withheld-restored generation=7 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
STOCK_FLOW case=generator-withheld-restored generation=7 owner=terran energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=36 minerals_after=37 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=7 owner=pirate energy_before=3 settled=2 consumed=1 energy_after=4 minerals_before=30 minerals_after=31 alloys_before=4 alloys_after=5
GENERATOR_FLOW case=generator-withheld-restored generation=7 owner=pirate authored_effective_flow=[2.0, 2.0]
STOCK_CELL case=generator-withheld-restored generation=8 host=terran_mine id=364 slot=Some(SlotIndex(29)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=generator-withheld-restored generation=8 host=A1 id=366 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=generator-withheld-restored generation=8 host=pirate_mine id=369 slot=Some(SlotIndex(34)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=generator-withheld-restored generation=8 host=E1 id=371 slot=Some(SlotIndex(31)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
STOCK_FLOW case=generator-withheld-restored generation=8 owner=terran energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=37 minerals_after=38 alloys_before=6 alloys_after=7
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=terran authored_effective_flow=[2.0, 2.0]
STOCK_FLOW case=generator-withheld-restored generation=8 owner=pirate energy_before=4 settled=2 consumed=1 energy_after=5 minerals_before=31 minerals_after=32 alloys_before=5 alloys_after=6
GENERATOR_FLOW case=generator-withheld-restored generation=8 owner=pirate authored_effective_flow=[2.0, 2.0]
ok
test rehearsal_economy_fleet_native_funded_output_must_birth_fleets ... BIRTH_AUTHORING_REFUSAL case=action-band source=fnv1a64:83ca57616a8b8522:8628 error=SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmp93WmTm\\stellaristhing_base.clause: ClauseThing hydration error at token 753: unsupported scenario field `action_band`")
BIRTH_AUTHORING_REFUSAL case=structural-effect source=fnv1a64:c087b12816730a57:8736 error=SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmp1YDeMm\\stellaristhing_base.clause: ClauseThing hydration error at token 769: unsupported effect field `add_child`")
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpm5BNgM\stellaristhing_base.clause identity=fnv1a64:7512f765f444cb46:8594 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:e30be03d782da9fb:10670
PROFILE_FULL case=funded-output-birth-gap identity=fnv1a64:8597b5ec7a63b549:25113 targets={"E1": [SimThingId(452)], "terran_refinery": [SimThingId(445)], "terran_shipyard": [SimThingId(441)], "pirate_mine": [SimThingId(450)], "terran_generator_2": [SimThingId(443)], "terran_generator_1": [SimThingId(442)], "pirate_shipyard": [SimThingId(447)], "stellaristhing_base": [SimThingId(467)], "pirate_generator_2": [SimThingId(449)], "pirate": [SimThingId(440)], "pirate_refinery": [SimThingId(451)], "pirate_generator_1": [SimThingId(448)], "terran_mine": [SimThingId(444)], "A1": [SimThingId(446)], "terran": [SimThingId(439)]}
STOCK_CELL case=funded-output-birth-gap generation=0 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=funded-output-birth-gap generation=0 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=0 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=funded-output-birth-gap generation=0 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=1 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=funded-output-birth-gap generation=1 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=1 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=funded-output-birth-gap generation=1 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=441 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=1 owner=pirate shipyard_id=447 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=2 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=funded-output-birth-gap generation=2 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=2 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=funded-output-birth-gap generation=2 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=441 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=2 owner=pirate shipyard_id=447 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=3 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=funded-output-birth-gap generation=3 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=funded-output-birth-gap generation=3 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=funded-output-birth-gap generation=3 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=441 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=3 owner=pirate shipyard_id=447 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=4 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=funded-output-birth-gap generation=4 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=funded-output-birth-gap generation=4 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=funded-output-birth-gap generation=4 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=441 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=4 owner=pirate shipyard_id=447 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=5 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=funded-output-birth-gap generation=5 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=5 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=funded-output-birth-gap generation=5 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=441 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=5 owner=pirate shipyard_id=447 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=6 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=funded-output-birth-gap generation=6 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=6 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=funded-output-birth-gap generation=6 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=441 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=6 owner=pirate shipyard_id=447 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=7 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=funded-output-birth-gap generation=7 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=7 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=funded-output-birth-gap generation=7 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=441 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=7 owner=pirate shipyard_id=447 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=8 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=funded-output-birth-gap generation=8 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=8 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=funded-output-birth-gap generation=8 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=441 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=8 owner=pirate shipyard_id=447 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=9 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=funded-output-birth-gap generation=9 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=9 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=funded-output-birth-gap generation=9 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=441 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=9 owner=pirate shipyard_id=447 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=10 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=funded-output-birth-gap generation=10 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=10 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=funded-output-birth-gap generation=10 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=441 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=10 owner=pirate shipyard_id=447 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=11 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=funded-output-birth-gap generation=11 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=11 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=funded-output-birth-gap generation=11 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=441 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=11 owner=pirate shipyard_id=447 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=12 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=funded-output-birth-gap generation=12 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=12 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=funded-output-birth-gap generation=12 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=441 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=12 owner=pirate shipyard_id=447 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=13 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=funded-output-birth-gap generation=13 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=13 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=funded-output-birth-gap generation=13 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=441 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=13 owner=pirate shipyard_id=447 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=14 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=funded-output-birth-gap generation=14 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=14 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=funded-output-birth-gap generation=14 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=441 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=14 owner=pirate shipyard_id=447 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=15 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=funded-output-birth-gap generation=15 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=15 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=funded-output-birth-gap generation=15 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=441 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=15 owner=pirate shipyard_id=447 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=16 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=funded-output-birth-gap generation=16 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=16 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=funded-output-birth-gap generation=16 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=441 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=16 owner=pirate shipyard_id=447 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=17 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=funded-output-birth-gap generation=17 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=17 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=funded-output-birth-gap generation=17 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=441 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=17 owner=pirate shipyard_id=447 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=18 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=funded-output-birth-gap generation=18 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=18 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=funded-output-birth-gap generation=18 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=441 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=18 owner=pirate shipyard_id=447 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=19 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=funded-output-birth-gap generation=19 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=19 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=funded-output-birth-gap generation=19 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=441 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=19 owner=pirate shipyard_id=447 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=20 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=funded-output-birth-gap generation=20 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=20 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=funded-output-birth-gap generation=20 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=441 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=20 owner=pirate shipyard_id=447 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=21 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=funded-output-birth-gap generation=21 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=21 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=funded-output-birth-gap generation=21 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=441 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=21 owner=pirate shipyard_id=447 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=22 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=funded-output-birth-gap generation=22 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=22 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=funded-output-birth-gap generation=22 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=441 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=22 owner=pirate shipyard_id=447 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=23 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=funded-output-birth-gap generation=23 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=23 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=funded-output-birth-gap generation=23 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=441 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=23 owner=pirate shipyard_id=447 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=24 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=funded-output-birth-gap generation=24 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=24 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=funded-output-birth-gap generation=24 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=441 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=24 owner=pirate shipyard_id=447 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=25 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=funded-output-birth-gap generation=25 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=25 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=funded-output-birth-gap generation=25 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=441 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=25 owner=pirate shipyard_id=447 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=26 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=funded-output-birth-gap generation=26 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=26 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=funded-output-birth-gap generation=26 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=441 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=26 owner=pirate shipyard_id=447 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=27 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=funded-output-birth-gap generation=27 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=27 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=funded-output-birth-gap generation=27 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=441 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=27 owner=pirate shipyard_id=447 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=28 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=funded-output-birth-gap generation=28 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=28 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=funded-output-birth-gap generation=28 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=441 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=28 owner=pirate shipyard_id=447 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=29 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=funded-output-birth-gap generation=29 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=29 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=funded-output-birth-gap generation=29 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=441 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=29 owner=pirate shipyard_id=447 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=30 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=funded-output-birth-gap generation=30 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=30 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=funded-output-birth-gap generation=30 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=441 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=30 owner=pirate shipyard_id=447 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=31 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=funded-output-birth-gap generation=31 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=31 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=funded-output-birth-gap generation=31 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=441 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=31 owner=pirate shipyard_id=447 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=32 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=funded-output-birth-gap generation=32 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=32 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=funded-output-birth-gap generation=32 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=441 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=32 owner=pirate shipyard_id=447 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=33 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=funded-output-birth-gap generation=33 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=33 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=funded-output-birth-gap generation=33 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=441 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=33 owner=pirate shipyard_id=447 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=34 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=funded-output-birth-gap generation=34 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=34 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=funded-output-birth-gap generation=34 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=441 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=34 owner=pirate shipyard_id=447 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=35 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=funded-output-birth-gap generation=35 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=funded-output-birth-gap generation=35 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=funded-output-birth-gap generation=35 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=441 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=35 owner=pirate shipyard_id=447 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=36 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=funded-output-birth-gap generation=36 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=funded-output-birth-gap generation=36 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=funded-output-birth-gap generation=36 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=441 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=36 owner=pirate shipyard_id=447 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=37 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=funded-output-birth-gap generation=37 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=funded-output-birth-gap generation=37 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=funded-output-birth-gap generation=37 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=441 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=37 owner=pirate shipyard_id=447 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=38 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=funded-output-birth-gap generation=38 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=funded-output-birth-gap generation=38 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=funded-output-birth-gap generation=38 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=441 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=38 owner=pirate shipyard_id=447 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=39 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=funded-output-birth-gap generation=39 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=funded-output-birth-gap generation=39 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=funded-output-birth-gap generation=39 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=441 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=None
FUNDING_FLOW generation=39 owner=pirate shipyard_id=447 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=None
STOCK_CELL case=funded-output-birth-gap generation=40 host=terran_mine id=444 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=funded-output-birth-gap generation=40 host=A1 id=446 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=funded-output-birth-gap generation=40 host=pirate_mine id=450 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=funded-output-birth-gap generation=40 host=E1 id=452 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=441 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=None
FUNDING_FLOW generation=40 owner=pirate shipyard_id=447 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=None
BIRTH_GAP first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 466, 467, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491} g40_ids={439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 466, 467, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491} fresh_ids=[] n0_capacity=76 g40_capacity=76 action_generation=None

thread 'rehearsal_economy_fleet_native_funded_output_must_birth_fleets' (32072) panicked at crates\simthing-workshop\tests\rehearsal_lifecycle_economy_fleet.rs:801:5:
2.2 STOP: both corvette recipes were funded, but native scenario execution produced no fresh structural fleet identities; no authored funded-birth consequence reaches the existing 2.1 ActionBand door
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpZemAOG\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:a363f43c74606500:21795 targets={"E1": [SimThingId(505)], "pirate_generator_2": [SimThingId(502)], "pirate_mine": [SimThingId(503)], "pirate_refinery": [SimThingId(504)], "stellaristhing_base": [SimThingId(518)], "terran": [SimThingId(494)], "terran_generator_1": [SimThingId(496)], "A1": [SimThingId(500)], "pirate_generator_1": [SimThingId(501)], "terran_generator_2": [SimThingId(497)], "terran_mine": [SimThingId(498)], "terran_refinery": [SimThingId(499)], "pirate": [SimThingId(495)]}
CELL case=minerals-then-energy generation=0 host=terran id=494 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=495 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=500 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=500 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=505 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=505 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [137, 96, 132, 236, 142, 6, 119, 107, 208, 196, 164, 131, 197, 125, 206, 130], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=494 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=495 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=500 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=500 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=505 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=505 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [137, 96, 132, 236, 142, 6, 119, 107, 208, 196, 164, 131, 197, 125, 206, 130], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpsvcMBt\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:1d7ed91b2d58a028:21795 targets={"terran_refinery": [SimThingId(550)], "pirate_refinery": [SimThingId(555)], "A1": [SimThingId(551)], "pirate_mine": [SimThingId(554)], "pirate": [SimThingId(546)], "E1": [SimThingId(556)], "pirate_generator_1": [SimThingId(552)], "stellaristhing_base": [SimThingId(569)], "terran": [SimThingId(545)], "terran_generator_1": [SimThingId(547)], "terran_generator_2": [SimThingId(548)], "terran_mine": [SimThingId(549)], "pirate_generator_2": [SimThingId(553)]}
CELL case=energy-then-minerals generation=0 host=terran id=545 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=546 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=551 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=551 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=556 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=556 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [168, 119, 122, 236, 207, 122, 167, 111, 118, 239, 203, 81, 15, 152, 176, 20], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=545 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=546 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=551 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=551 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=556 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=556 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [168, 119, 122, 236, 207, 122, 167, 111, 118, 239, 203, 81, 15, 152, 176, 20], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

failures:

failures:
    rehearsal_economy_fleet_native_funded_output_must_birth_fleets

test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.31s

error: test failed, to rerun pass `-p simthing-workshop --test rehearsal_lifecycle_economy_fleet`

```
