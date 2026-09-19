# 2.2 RF/capacity execution observations

Implementation head `4d6f43a7a968698ea6a4da74426d039731c0938d`, tree `b28175a71c9aa14a69a95a39ce19f9550c1c7b1a`, exact release `a14d4a57813952781ec475cfd32d7c4f80fadfda`.
This companion retains test-harness output and witness observations. Repeated compiler warnings are omitted by starting each test log at `running N test`; no test output after that point is omitted. The ordered initial gates preceded implementation-head edits; the strengthened RF run and code-head full target include the new enrollment assertions. Raw compiler-inclusive logs remain in the local `.git/economy-fleet-22-rf-resume` packet. Final-head reruns/hosted artifacts are bound in the PR/Board return.

## Conjunction / canonical locus FIRST

Source log: `rehearsal_economy_fleet_refinery_retains_every_authored_cost.txt`. Exit 0.

```text
running 1 test
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp5HZrOK\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:22932d30206f7150:21795 targets={"terran_mine": [SimThingId(211)], "A1": [SimThingId(213)], "pirate_generator_1": [SimThingId(214)], "pirate": [SimThingId(208)], "pirate_mine": [SimThingId(216)], "pirate_refinery": [SimThingId(217)], "pirate_generator_2": [SimThingId(215)], "E1": [SimThingId(218)], "stellaristhing_base": [SimThingId(231)], "terran": [SimThingId(207)], "terran_generator_1": [SimThingId(209)], "terran_generator_2": [SimThingId(210)], "terran_refinery": [SimThingId(212)]}
CELL case=minerals-then-energy generation=0 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [220, 123, 212, 185, 201, 71, 86, 240, 77, 156, 197, 160, 126, 16, 192, 28], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=207 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=208 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=213 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=218 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [220, 123, 212, 185, 201, 71, 86, 240, 77, 156, 197, 160, 126, 16, 192, 28], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpNit4nS\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:fdba2997d49b0bed:21795 targets={"terran_generator_2": [SimThingId(261)], "terran_mine": [SimThingId(262)], "E1": [SimThingId(269)], "pirate_refinery": [SimThingId(268)], "A1": [SimThingId(264)], "pirate": [SimThingId(259)], "pirate_mine": [SimThingId(267)], "terran_generator_1": [SimThingId(260)], "terran_refinery": [SimThingId(263)], "pirate_generator_2": [SimThingId(266)], "stellaristhing_base": [SimThingId(282)], "pirate_generator_1": [SimThingId(265)], "terran": [SimThingId(258)]}
CELL case=energy-then-minerals generation=0 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [195, 107, 17, 69, 178, 158, 199, 114, 206, 88, 144, 86, 19, 237, 203, 115], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=258 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=259 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=264 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=269 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [195, 107, 17, 69, 178, 158, 199, 114, 206, 88, 144, 86, 19, 237, 203, 115], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 5.06s
```

## Capped refinery + generator scarcity/recovery SECOND

Source log: `rehearsal_economy_fleet_generator_stock_preserves_frozen_economy.txt`. Exit 0.

```text
running 1 test
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp38RhGm\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:c7631b0c5019a989:21574 targets={"terran_generator_1": [SimThingId(209)], "pirate_mine": [SimThingId(216)], "terran": [SimThingId(207)], "stellaristhing_base": [SimThingId(231)], "pirate_generator_2": [SimThingId(215)], "terran_generator_2": [SimThingId(210)], "pirate": [SimThingId(208)], "terran_mine": [SimThingId(211)], "pirate_generator_1": [SimThingId(214)], "E1": [SimThingId(218)], "terran_refinery": [SimThingId(212)], "A1": [SimThingId(213)], "pirate_refinery": [SimThingId(217)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp2WKXFK\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:68e85262ce664cd5:21573 targets={"terran_mine": [SimThingId(262)], "pirate": [SimThingId(259)], "A1": [SimThingId(264)], "E1": [SimThingId(269)], "terran": [SimThingId(258)], "pirate_generator_2": [SimThingId(266)], "terran_generator_1": [SimThingId(260)], "terran_generator_2": [SimThingId(261)], "terran_refinery": [SimThingId(263)], "pirate_generator_1": [SimThingId(265)], "pirate_mine": [SimThingId(267)], "pirate_refinery": [SimThingId(268)], "stellaristhing_base": [SimThingId(282)]}
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
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.63s
```

## Native funded structural birth THIRD

Source log: `rehearsal_economy_fleet_native_funded_output_must_birth_fleets.txt`. Exit 0.

```text
running 1 test
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpczwoUn\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:fd8b69468b733f07:26980 targets={"E1": [SimThingId(220)], "pirate_mine": [SimThingId(218)], "terran_mine": [SimThingId(212)], "terran_generator_2": [SimThingId(211)], "terran": [SimThingId(207)], "pirate": [SimThingId(208)], "terran_shipyard": [SimThingId(209)], "terran_generator_1": [SimThingId(210)], "terran_refinery": [SimThingId(213)], "stellaristhing_base": [SimThingId(235)], "pirate_refinery": [SimThingId(219)], "pirate_shipyard": [SimThingId(215)], "A1": [SimThingId(214)], "pirate_generator_1": [SimThingId(216)], "pirate_generator_2": [SimThingId(217)]}
STOCK_CELL case=native-funded-birth generation=0 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=0 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=0 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=14
STOCK_CELL case=native-funded-birth generation=0 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=1 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=1 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=1 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=15
STOCK_CELL case=native-funded-birth generation=1 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=1 owner=terran shipyard_id=209 energy_before=0 settled=1 energy_after=1 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=1 owner=pirate shipyard_id=215 energy_before=0 settled=1 energy_after=1 alloys_before=3 alloys_after=4 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=2 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=2 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=2 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=16
STOCK_CELL case=native-funded-birth generation=2 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=2 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=2 owner=pirate shipyard_id=215 energy_before=1 settled=1 energy_after=2 alloys_before=4 alloys_after=5 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=3 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=3 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=7
STOCK_CELL case=native-funded-birth generation=3 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=17
STOCK_CELL case=native-funded-birth generation=3 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=3 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=3 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=5 alloys_after=6 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=4 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=4 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=8
STOCK_CELL case=native-funded-birth generation=4 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=18
STOCK_CELL case=native-funded-birth generation=4 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=7
FUNDING_FLOW generation=4 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=7 alloys_after=8 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
FUNDING_FLOW generation=4 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=6 alloys_after=7 scalar_funded_total=0 scalar_funded_delta=0 action_generation=Some(0)
STOCK_CELL case=native-funded-birth generation=5 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=5 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=5 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=19
STOCK_CELL case=native-funded-birth generation=5 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=5 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=1 alloys_before=8 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
FUNDING_FLOW generation=5 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=1 alloys_before=7 alloys_after=2 scalar_funded_total=1 scalar_funded_delta=1 action_generation=Some(1)
NATIVE_BIRTH generation=6 parent=214 id=262 faction=0
NATIVE_BIRTH generation=6 parent=220 id=268 faction=1
STOCK_CELL case=native-funded-birth generation=6 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=6 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=6 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=20
STOCK_CELL case=native-funded-birth generation=6 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=6 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=6 owner=pirate shipyard_id=215 energy_before=1 settled=1 energy_after=2 alloys_before=2 alloys_after=3 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=7 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=7 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=7 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=21
STOCK_CELL case=native-funded-birth generation=7 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=7 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=7 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=3 alloys_after=4 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=8 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=8 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=8 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=22
STOCK_CELL case=native-funded-birth generation=8 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=8 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
FUNDING_FLOW generation=8 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=4 alloys_after=5 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(1)
STOCK_CELL case=native-funded-birth generation=9 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=9 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=9 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=23
STOCK_CELL case=native-funded-birth generation=9 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=9 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=1 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(2)
FUNDING_FLOW generation=9 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=5 alloys_after=6 scalar_funded_total=1 scalar_funded_delta=0 action_generation=Some(2)
NATIVE_BIRTH generation=10 parent=214 id=265 faction=0
STOCK_CELL case=native-funded-birth generation=10 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=10 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=10 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=24
STOCK_CELL case=native-funded-birth generation=10 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=10 owner=terran shipyard_id=209 energy_before=1 settled=1 energy_after=2 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=10 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=2 alloys_before=6 alloys_after=1 scalar_funded_total=2 scalar_funded_delta=1 action_generation=Some(3)
NATIVE_BIRTH generation=11 parent=220 id=271 faction=1
STOCK_CELL case=native-funded-birth generation=11 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=11 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=11 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=25
STOCK_CELL case=native-funded-birth generation=11 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=11 owner=terran shipyard_id=209 energy_before=2 settled=1 energy_after=3 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=11 owner=pirate shipyard_id=215 energy_before=2 settled=1 energy_after=3 alloys_before=1 alloys_after=2 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=12 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=12 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=12 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=26
STOCK_CELL case=native-funded-birth generation=12 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=12 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=12 owner=pirate shipyard_id=215 energy_before=3 settled=1 energy_after=4 alloys_before=2 alloys_after=3 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=13 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=13 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=13 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=27
STOCK_CELL case=native-funded-birth generation=13 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=13 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=5 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=13 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=3 alloys_after=4 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=14 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=14 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=14 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=28
STOCK_CELL case=native-funded-birth generation=14 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=14 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=14 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=6 alloys_before=4 alloys_after=5 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=15 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=15 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=15 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=29
STOCK_CELL case=native-funded-birth generation=15 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=15 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=3 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=15 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=5 alloys_after=6 scalar_funded_total=2 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=16 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=16 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=16 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=30
STOCK_CELL case=native-funded-birth generation=16 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=16 owner=terran shipyard_id=209 energy_before=3 settled=1 energy_after=4 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=16 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=4 alloys_before=6 alloys_after=1 scalar_funded_total=3 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=17 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=17 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=17 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=31
STOCK_CELL case=native-funded-birth generation=17 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=17 owner=terran shipyard_id=209 energy_before=4 settled=1 energy_after=5 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=17 owner=pirate shipyard_id=215 energy_before=4 settled=1 energy_after=5 alloys_before=1 alloys_after=2 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=18 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=18 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=18 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=32
STOCK_CELL case=native-funded-birth generation=18 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=18 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=18 owner=pirate shipyard_id=215 energy_before=5 settled=1 energy_after=6 alloys_before=2 alloys_after=3 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=19 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=19 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=19 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=33
STOCK_CELL case=native-funded-birth generation=19 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=19 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=7 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=19 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=3 alloys_after=4 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=20 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=20 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=20 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=34
STOCK_CELL case=native-funded-birth generation=20 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=20 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=20 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=8 alloys_before=4 alloys_after=5 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=21 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=21 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=21 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=35
STOCK_CELL case=native-funded-birth generation=21 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=21 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=5 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=21 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=5 alloys_after=6 scalar_funded_total=3 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=22 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=22 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=22 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=36
STOCK_CELL case=native-funded-birth generation=22 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=22 owner=terran shipyard_id=209 energy_before=5 settled=1 energy_after=6 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=22 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=6 alloys_before=6 alloys_after=1 scalar_funded_total=4 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=23 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=23 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=23 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=37
STOCK_CELL case=native-funded-birth generation=23 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=23 owner=terran shipyard_id=209 energy_before=6 settled=1 energy_after=7 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=23 owner=pirate shipyard_id=215 energy_before=6 settled=1 energy_after=7 alloys_before=1 alloys_after=2 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=24 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=24 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=24 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=38
STOCK_CELL case=native-funded-birth generation=24 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=24 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=24 owner=pirate shipyard_id=215 energy_before=7 settled=1 energy_after=8 alloys_before=2 alloys_after=3 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=25 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=25 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=25 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=39
STOCK_CELL case=native-funded-birth generation=25 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=25 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=9 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=25 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=3 alloys_after=4 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=26 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=26 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=26 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=40
STOCK_CELL case=native-funded-birth generation=26 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=26 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=26 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=10 alloys_before=4 alloys_after=5 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=27 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=27 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=27 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=41
STOCK_CELL case=native-funded-birth generation=27 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=27 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=7 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=27 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=5 alloys_after=6 scalar_funded_total=4 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=28 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=28 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=28 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=42
STOCK_CELL case=native-funded-birth generation=28 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=28 owner=terran shipyard_id=209 energy_before=7 settled=1 energy_after=8 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=28 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=8 alloys_before=6 alloys_after=1 scalar_funded_total=5 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=29 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=29 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=29 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=43
STOCK_CELL case=native-funded-birth generation=29 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=29 owner=terran shipyard_id=209 energy_before=8 settled=1 energy_after=9 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=29 owner=pirate shipyard_id=215 energy_before=8 settled=1 energy_after=9 alloys_before=1 alloys_after=2 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=30 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=30 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=30 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=44
STOCK_CELL case=native-funded-birth generation=30 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=30 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=30 owner=pirate shipyard_id=215 energy_before=9 settled=1 energy_after=10 alloys_before=2 alloys_after=3 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=31 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=31 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=31 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=45
STOCK_CELL case=native-funded-birth generation=31 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=31 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=11 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=31 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=3 alloys_after=4 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=32 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=32 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=32 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=46
STOCK_CELL case=native-funded-birth generation=32 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=32 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=32 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=12 alloys_before=4 alloys_after=5 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=33 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=33 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=33 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=47
STOCK_CELL case=native-funded-birth generation=33 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=33 owner=terran shipyard_id=209 energy_before=12 settled=1 energy_after=9 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=33 owner=pirate shipyard_id=215 energy_before=12 settled=1 energy_after=13 alloys_before=5 alloys_after=6 scalar_funded_total=5 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=34 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=34 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=34 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=48
STOCK_CELL case=native-funded-birth generation=34 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=34 owner=terran shipyard_id=209 energy_before=9 settled=1 energy_after=10 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=34 owner=pirate shipyard_id=215 energy_before=13 settled=1 energy_after=10 alloys_before=6 alloys_after=1 scalar_funded_total=6 scalar_funded_delta=1 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=35 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=55
STOCK_CELL case=native-funded-birth generation=35 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=3
STOCK_CELL case=native-funded-birth generation=35 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=49
STOCK_CELL case=native-funded-birth generation=35 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=2
FUNDING_FLOW generation=35 owner=terran shipyard_id=209 energy_before=10 settled=1 energy_after=11 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=35 owner=pirate shipyard_id=215 energy_before=10 settled=1 energy_after=11 alloys_before=1 alloys_after=2 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=36 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=56
STOCK_CELL case=native-funded-birth generation=36 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=4
STOCK_CELL case=native-funded-birth generation=36 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=50
STOCK_CELL case=native-funded-birth generation=36 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=3
FUNDING_FLOW generation=36 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=36 owner=pirate shipyard_id=215 energy_before=11 settled=1 energy_after=12 alloys_before=2 alloys_after=3 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=37 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=57
STOCK_CELL case=native-funded-birth generation=37 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=5
STOCK_CELL case=native-funded-birth generation=37 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=51
STOCK_CELL case=native-funded-birth generation=37 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=4
FUNDING_FLOW generation=37 owner=terran shipyard_id=209 energy_before=12 settled=1 energy_after=13 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=37 owner=pirate shipyard_id=215 energy_before=12 settled=1 energy_after=13 alloys_before=3 alloys_after=4 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=38 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=58
STOCK_CELL case=native-funded-birth generation=38 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=6
STOCK_CELL case=native-funded-birth generation=38 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=52
STOCK_CELL case=native-funded-birth generation=38 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=5
FUNDING_FLOW generation=38 owner=terran shipyard_id=209 energy_before=13 settled=1 energy_after=14 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=38 owner=pirate shipyard_id=215 energy_before=13 settled=1 energy_after=14 alloys_before=4 alloys_after=5 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=39 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=59
STOCK_CELL case=native-funded-birth generation=39 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=1
STOCK_CELL case=native-funded-birth generation=39 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=53
STOCK_CELL case=native-funded-birth generation=39 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=6
FUNDING_FLOW generation=39 owner=terran shipyard_id=209 energy_before=14 settled=1 energy_after=11 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
FUNDING_FLOW generation=39 owner=pirate shipyard_id=215 energy_before=14 settled=1 energy_after=15 alloys_before=5 alloys_after=6 scalar_funded_total=6 scalar_funded_delta=0 action_generation=Some(3)
STOCK_CELL case=native-funded-birth generation=40 host=terran_mine id=212 slot=Some(SlotIndex(30)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=60
STOCK_CELL case=native-funded-birth generation=40 host=A1 id=214 slot=Some(SlotIndex(26)) property=PropertyKey { namespace: "meridian_material", name: "A1_alloys_quantity" } role=Amount value=2
STOCK_CELL case=native-funded-birth generation=40 host=pirate_mine id=218 slot=Some(SlotIndex(36)) property=PropertyKey { namespace: "meridian", name: "minerals" } role=Named("balance") value=54
STOCK_CELL case=native-funded-birth generation=40 host=E1 id=220 slot=Some(SlotIndex(32)) property=PropertyKey { namespace: "meridian_material", name: "E1_alloys_quantity" } role=Amount value=1
FUNDING_FLOW generation=40 owner=terran shipyard_id=209 energy_before=11 settled=1 energy_after=12 alloys_before=1 alloys_after=2 scalar_funded_total=7 scalar_funded_delta=0 action_generation=Some(3)
FUNDING_FLOW generation=40 owner=pirate shipyard_id=215 energy_before=15 settled=1 energy_after=12 alloys_before=6 alloys_after=1 scalar_funded_total=7 scalar_funded_delta=1 action_generation=Some(3)
BIRTH_SHAPE owner=terran id=262 children=[SimThingId(263), SimThingId(264)] hull=17179870000 extent=3 parent=214 generation=6
BIRTH_SHAPE owner=terran id=265 children=[SimThingId(266), SimThingId(267)] hull=1073741800 extent=3 parent=214 generation=10
BIRTH_SHAPE owner=pirate id=268 children=[SimThingId(269), SimThingId(270)] hull=17179870000 extent=3 parent=220 generation=6
BIRTH_SHAPE owner=pirate id=271 children=[SimThingId(272), SimThingId(273)] hull=536870900 extent=3 parent=220 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 234, 235, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259} fresh_ids=[262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273] capacity=76
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 1.98s
```

## Born-energy RF upkeep FOURTH

Source log: `rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow.txt`. Exit 0.

```text
running 1 test
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpTLbUnq\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:9cde5fb11f511787:27258 targets={"pirate_refinery": [SimThingId(219)], "terran_mine": [SimThingId(212)], "stellaristhing_base": [SimThingId(235)], "pirate_shipyard": [SimThingId(215)], "terran_refinery": [SimThingId(213)], "terran": [SimThingId(207)], "pirate_mine": [SimThingId(218)], "E1": [SimThingId(220)], "pirate": [SimThingId(208)], "A1": [SimThingId(214)], "pirate_generator_2": [SimThingId(217)], "pirate_generator_1": [SimThingId(216)], "terran_generator_2": [SimThingId(211)], "terran_generator_1": [SimThingId(210)], "terran_shipyard": [SimThingId(209)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpbHDypb\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:0b554d3c2aaba6c0:27260 targets={"terran_refinery": [SimThingId(280)], "terran": [SimThingId(274)], "terran_shipyard": [SimThingId(276)], "pirate_generator_2": [SimThingId(284)], "stellaristhing_base": [SimThingId(302)], "terran_mine": [SimThingId(279)], "terran_generator_1": [SimThingId(277)], "pirate_generator_1": [SimThingId(283)], "pirate": [SimThingId(275)], "pirate_mine": [SimThingId(285)], "terran_generator_2": [SimThingId(278)], "A1": [SimThingId(281)], "pirate_shipyard": [SimThingId(282)], "E1": [SimThingId(287)], "pirate_refinery": [SimThingId(286)]}
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
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(38), subtree_root: SimThingId(329), parent: Some(SimThingId(281)) }]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[ArenaMember { arena_idx: 1, slot: SlotIndex(41), subtree_root: SimThingId(335), parent: Some(SimThingId(287)) }]
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.02s
```

## RF boundary strengthening

Source log: `rf-boundary-strengthened.txt`. Exit 0.

```text
running 1 test
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpZvdJ5j\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:22edf2985694b983:27258 targets={"pirate_mine": [SimThingId(218)], "pirate_refinery": [SimThingId(219)], "stellaristhing_base": [SimThingId(235)], "A1": [SimThingId(214)], "pirate_generator_1": [SimThingId(216)], "pirate_generator_2": [SimThingId(217)], "E1": [SimThingId(220)], "terran_generator_1": [SimThingId(210)], "terran_generator_2": [SimThingId(211)], "terran_refinery": [SimThingId(213)], "pirate_shipyard": [SimThingId(215)], "terran_shipyard": [SimThingId(209)], "terran_mine": [SimThingId(212)], "pirate": [SimThingId(208)], "terran": [SimThingId(207)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpzEGvH5\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:a0ec3065fdbbd1c8:27260 targets={"terran_refinery": [SimThingId(280)], "pirate_mine": [SimThingId(285)], "pirate_generator_2": [SimThingId(284)], "A1": [SimThingId(281)], "stellaristhing_base": [SimThingId(302)], "terran_generator_2": [SimThingId(278)], "E1": [SimThingId(287)], "pirate": [SimThingId(275)], "pirate_shipyard": [SimThingId(282)], "terran": [SimThingId(274)], "terran_generator_1": [SimThingId(277)], "pirate_generator_1": [SimThingId(283)], "terran_mine": [SimThingId(279)], "pirate_refinery": [SimThingId(286)], "terran_shipyard": [SimThingId(276)]}
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.39s
```

## Capacity focused

Source log: `capacity-focused.txt`. Exit 0.

```text
running 1 test
test sequential_capacity_exhaustion_preserves_prior_births ... CAPACITY_FIRST_REFUSAL G17: births=12 funding=(16, 12.0)->(17, 13.0) threshold=12.5 capacity=0 reason=MarketUnresolved { granted: 0 } placements=[(SimThingId(307), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(307), market_grant_key: 10352942583764372668 }, extent: ResidencyExtent { start: 36, length: 3 }, quantity: 3, committed_generation: GenerationStamp(2) }), (SimThingId(310), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(310), market_grant_key: 8573165843398269791 }, extent: ResidencyExtent { start: 39, length: 3 }, quantity: 3, committed_generation: GenerationStamp(3) }), (SimThingId(313), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(313), market_grant_key: 14002190669195122962 }, extent: ResidencyExtent { start: 42, length: 3 }, quantity: 3, committed_generation: GenerationStamp(4) }), (SimThingId(316), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(316), market_grant_key: 8514883878960620981 }, extent: ResidencyExtent { start: 45, length: 3 }, quantity: 3, committed_generation: GenerationStamp(5) }), (SimThingId(319), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(319), market_grant_key: 4438988822954733144 }, extent: ResidencyExtent { start: 48, length: 3 }, quantity: 3, committed_generation: GenerationStamp(6) }), (SimThingId(322), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(322), market_grant_key: 6254572420137714587 }, extent: ResidencyExtent { start: 51, length: 3 }, quantity: 3, committed_generation: GenerationStamp(8) }), (SimThingId(325), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(325), market_grant_key: 18212029171555461294 }, extent: ResidencyExtent { start: 54, length: 3 }, quantity: 3, committed_generation: GenerationStamp(9) }), (SimThingId(328), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(328), market_grant_key: 2548703524055453713 }, extent: ResidencyExtent { start: 57, length: 3 }, quantity: 3, committed_generation: GenerationStamp(10) }), (SimThingId(331), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(331), market_grant_key: 1095901044537297988 }, extent: ResidencyExtent { start: 60, length: 3 }, quantity: 3, committed_generation: GenerationStamp(12) }), (SimThingId(334), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(334), market_grant_key: 14742320039626927559 }, extent: ResidencyExtent { start: 63, length: 3 }, quantity: 3, committed_generation: GenerationStamp(13) }), (SimThingId(337), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(337), market_grant_key: 470047695646874394 }, extent: ResidencyExtent { start: 66, length: 3 }, quantity: 3, committed_generation: GenerationStamp(14) }), (SimThingId(340), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(230), grantee: SimThingId(340), market_grant_key: 2280841519078078333 }, extent: ResidencyExtent { start: 69, length: 3 }, quantity: 3, committed_generation: GenerationStamp(16) })]
CAPACITY_END births=12 first_refusal=Some(17) later_refusals=17 live0=36 capacity0=36
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.57s
```

## Native structural-product full target

Source log: `capacity-full.txt`. Exit 0.

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
refused (is an owner seat): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpyNQpXJ\\stellaristhing_base.clause: ClauseThing hydration error at token 514: structural_product `terran_corvettes` parent `terran` is an owner seat; declare ownership on the template with `owner_ref`")
refused (unknown owner_ref): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpEaIhc9\\stellaristhing_base.clause: ClauseThing hydration error at token 534: unknown owner_ref `nobody`")
refused (count must be a positive integer): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpTaof6E\\stellaristhing_base.clause: ClauseThing hydration error at token 526: structural_product.count must be a positive integer")
refused (does not carry): StructuralProduct(FundingLocus { product: "terran_corvettes", reason: "host `E1` does not carry `meridian_material::A1_corvettes_quantity`", span: Some(514) })
refused (is not registered): StructuralProduct(Template { product: "terran_corvettes", reason: "property `meridian_material::A1_rubble_quantity` is not registered", span: Some(514) })
refused (duplicate structural_product id): SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpnbsuxp\\stellaristhing_base.clause: ClauseThing hydration error at token 568: duplicate structural_product id `terran_corvettes`")
ok
test product_meaning_is_independent_of_declaration_order ... declaration-order outcome: [(2, "A1", (2, 1, "terran", 2)), (2, "E1", (2, 1, "pirate", 2)), (3, "A1", (2, 1, "terran", 2)), (3, "E1", (2, 1, "pirate", 2))]
declaration-order outcome: [(2, "A1", (2, 1, "terran", 2)), (2, "E1", (2, 1, "pirate", 2)), (3, "A1", (2, 1, "terran", 2)), (3, "E1", (2, 1, "pirate", 2))]
ok
test sequential_capacity_exhaustion_preserves_prior_births ... CAPACITY_FIRST_REFUSAL G17: births=12 funding=(16, 12.0)->(17, 13.0) threshold=12.5 capacity=0 reason=MarketUnresolved { granted: 0 } placements=[(SimThingId(1695), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1695), market_grant_key: 6664684684200072115 }, extent: ResidencyExtent { start: 36, length: 3 }, quantity: 3, committed_generation: GenerationStamp(2) }), (SimThingId(1698), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1698), market_grant_key: 12883776174534346028 }, extent: ResidencyExtent { start: 39, length: 3 }, quantity: 3, committed_generation: GenerationStamp(3) }), (SimThingId(1701), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1701), market_grant_key: 912873953335216777 }, extent: ResidencyExtent { start: 42, length: 3 }, quantity: 3, committed_generation: GenerationStamp(4) }), (SimThingId(1704), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1704), market_grant_key: 15320242607439861962 }, extent: ResidencyExtent { start: 45, length: 3 }, quantity: 3, committed_generation: GenerationStamp(5) }), (SimThingId(1707), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1707), market_grant_key: 17990460431141411511 }, extent: ResidencyExtent { start: 48, length: 3 }, quantity: 3, committed_generation: GenerationStamp(6) }), (SimThingId(1710), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1710), market_grant_key: 17089988189343564176 }, extent: ResidencyExtent { start: 51, length: 3 }, quantity: 3, committed_generation: GenerationStamp(8) }), (SimThingId(1713), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1713), market_grant_key: 2852310034386724749 }, extent: ResidencyExtent { start: 54, length: 3 }, quantity: 3, committed_generation: GenerationStamp(9) }), (SimThingId(1716), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1716), market_grant_key: 10609369773598259966 }, extent: ResidencyExtent { start: 57, length: 3 }, quantity: 3, committed_generation: GenerationStamp(10) }), (SimThingId(1719), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1719), market_grant_key: 8937020855176226923 }, extent: ResidencyExtent { start: 60, length: 3 }, quantity: 3, committed_generation: GenerationStamp(12) }), (SimThingId(1722), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1722), market_grant_key: 14129580037250649412 }, extent: ResidencyExtent { start: 63, length: 3 }, quantity: 3, committed_generation: GenerationStamp(13) }), (SimThingId(1725), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1725), market_grant_key: 4821042542904771233 }, extent: ResidencyExtent { start: 66, length: 3 }, quantity: 3, committed_generation: GenerationStamp(14) }), (SimThingId(1728), CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1618), grantee: SimThingId(1728), market_grant_key: 8115104194765900866 }, extent: ResidencyExtent { start: 69, length: 3 }, quantity: 3, committed_generation: GenerationStamp(16) })]
CAPACITY_END births=12 first_refusal=Some(17) later_refusals=17 live0=36 capacity0=36
ok
test unfunded_or_unplaceable_products_birth_nothing ... unplaceable: births [[], [], [], [], [], []] refusals 1
ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 32.69s
```

## Committed code-head economy/fleet full target

Source log: `code-parent-full.txt`. Exit 0.

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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpPQh4dO\stellaristhing_base.clause identity=fnv1a64:31e8cc4e064bc243:10037 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:da3192ad1a8ccb62:12796
PROFILE_FULL case=zero-upkeep-control identity=fnv1a64:3fb7126930d4739f:27258 targets={"pirate": [SimThingId(208)], "terran": [SimThingId(207)], "E1": [SimThingId(220)], "A1": [SimThingId(214)], "terran_shipyard": [SimThingId(209)], "pirate_mine": [SimThingId(218)], "terran_mine": [SimThingId(212)], "pirate_refinery": [SimThingId(219)], "pirate_shipyard": [SimThingId(215)], "pirate_generator_2": [SimThingId(217)], "terran_generator_1": [SimThingId(210)], "pirate_generator_1": [SimThingId(216)], "stellaristhing_base": [SimThingId(235)], "terran_generator_2": [SimThingId(211)], "terran_refinery": [SimThingId(213)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp151gDb\stellaristhing_base.clause identity=fnv1a64:413b95f4002e88f1:10039 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:ed387709967704dc:12798
PROFILE_FULL case=one-energy-upkeep identity=fnv1a64:fe166c90ddf48126:27260 targets={"terran_generator_2": [SimThingId(278)], "pirate": [SimThingId(275)], "pirate_generator_2": [SimThingId(284)], "pirate_mine": [SimThingId(285)], "E1": [SimThingId(287)], "terran_refinery": [SimThingId(280)], "pirate_shipyard": [SimThingId(282)], "A1": [SimThingId(281)], "pirate_generator_1": [SimThingId(283)], "terran": [SimThingId(274)], "terran_mine": [SimThingId(279)], "terran_generator_1": [SimThingId(277)], "stellaristhing_base": [SimThingId(302)], "terran_shipyard": [SimThingId(276)], "pirate_refinery": [SimThingId(286)]}
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
PROFILE_FULL case=canonical identity=fnv1a64:4dec61a3ff5fc8e7:21555 targets={"terran_generator_2": [SimThingId(344)], "A1": [SimThingId(347)], "pirate_generator_1": [SimThingId(348)], "pirate_refinery": [SimThingId(351)], "stellaristhing_base": [SimThingId(365)], "terran_generator_1": [SimThingId(343)], "terran_mine": [SimThingId(345)], "terran": [SimThingId(341)], "pirate": [SimThingId(342)], "pirate_generator_2": [SimThingId(349)], "pirate_mine": [SimThingId(350)], "terran_refinery": [SimThingId(346)], "E1": [SimThingId(352)]}
N0 case=canonical root=364 existing_ids={341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 364, 365, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389} targets={"terran_generator_2": [SimThingId(344)], "A1": [SimThingId(347)], "pirate_generator_1": [SimThingId(348)], "pirate_refinery": [SimThingId(351)], "stellaristhing_base": [SimThingId(365)], "terran_generator_1": [SimThingId(343)], "terran_mine": [SimThingId(345)], "terran": [SimThingId(341)], "pirate": [SimThingId(342)], "pirate_generator_2": [SimThingId(349)], "pirate_mine": [SimThingId(350)], "terran_refinery": [SimThingId(346)], "E1": [SimThingId(352)]}
CELL case=canonical generation=0 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=0 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=0 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=canonical generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [120, 195, 134, 35, 2, 178, 101, 185, 13, 5, 180, 247, 131, 38, 39, 152], incarnation: 1 }
CELL case=canonical generation=1 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=1 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=1 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=canonical generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [120, 195, 134, 35, 2, 178, 101, 185, 13, 5, 180, 247, 131, 38, 39, 152], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=1 terran=1 pirate=1
CELL case=canonical generation=2 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=2 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=canonical generation=2 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=canonical generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [120, 195, 134, 35, 2, 178, 101, 185, 13, 5, 180, 247, 131, 38, 39, 152], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=2 terran=3 pirate=3
CELL case=canonical generation=3 host=terran id=341 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=canonical generation=3 host=pirate id=342 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=A1 id=347 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=canonical generation=3 host=E1 id=352 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=canonical generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [120, 195, 134, 35, 2, 178, 101, 185, 13, 5, 180, 247, 131, 38, 39, 152], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=canonical generation=3 terran=4 pirate=4
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp2qgggF\stellaristhing_base.clause identity=fnv1a64:874543addf5480a4:5797 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
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
PROFILE_FULL case=energy-withheld identity=fnv1a64:76baa4d997935232:21554 targets={"terran": [SimThingId(392)], "pirate_refinery": [SimThingId(402)], "A1": [SimThingId(398)], "pirate_generator_2": [SimThingId(400)], "pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "stellaristhing_base": [SimThingId(416)], "pirate_mine": [SimThingId(401)], "terran_mine": [SimThingId(396)], "terran_refinery": [SimThingId(397)], "E1": [SimThingId(403)], "pirate": [SimThingId(393)], "terran_generator_1": [SimThingId(394)]}
N0 case=energy-withheld root=415 existing_ids={392, 393, 394, 395, 396, 397, 398, 399, 400, 401, 402, 403, 415, 416, 419, 420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440} targets={"terran": [SimThingId(392)], "pirate_refinery": [SimThingId(402)], "A1": [SimThingId(398)], "pirate_generator_2": [SimThingId(400)], "pirate_generator_1": [SimThingId(399)], "terran_generator_2": [SimThingId(395)], "stellaristhing_base": [SimThingId(416)], "pirate_mine": [SimThingId(401)], "terran_mine": [SimThingId(396)], "terran_refinery": [SimThingId(397)], "E1": [SimThingId(403)], "pirate": [SimThingId(393)], "terran_generator_1": [SimThingId(394)]}
CELL case=energy-withheld generation=0 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=0 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-withheld generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [167, 0, 70, 207, 233, 36, 35, 124, 27, 94, 114, 188, 54, 166, 53, 93], incarnation: 1 }
CELL case=energy-withheld generation=1 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=1 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-withheld generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [167, 0, 70, 207, 233, 36, 35, 124, 27, 94, 114, 188, 54, 166, 53, 93], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=1 terran=1 pirate=1
CELL case=energy-withheld generation=2 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=7
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-withheld generation=2 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=6
CAPACITY case=energy-withheld generation=2 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [167, 0, 70, 207, 233, 36, 35, 124, 27, 94, 114, 188, 54, 166, 53, 93], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=2 terran=3 pirate=3
CELL case=energy-withheld generation=3 host=terran id=392 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=pirate id=393 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=0
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=A1 id=398 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=8
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-withheld generation=3 host=E1 id=403 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=7
CAPACITY case=energy-withheld generation=3 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [167, 0, 70, 207, 233, 36, 35, 124, 27, 94, 114, 188, 54, 166, 53, 93], incarnation: 1 }
OBSERVED_ALLOY_DELTA case=energy-withheld generation=3 terran=4 pirate=4
ok
test rehearsal_economy_fleet_generator_stock_preserves_frozen_economy ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpckXKdI\stellaristhing_base.clause identity=fnv1a64:1c9b4ba3b71f9886:7490 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-stock identity=fnv1a64:1f6238cd092a5240:21574 targets={"terran_generator_2": [SimThingId(446)], "terran_mine": [SimThingId(447)], "terran_refinery": [SimThingId(448)], "E1": [SimThingId(454)], "pirate_generator_2": [SimThingId(451)], "stellaristhing_base": [SimThingId(467)], "A1": [SimThingId(449)], "pirate_refinery": [SimThingId(453)], "terran": [SimThingId(443)], "pirate": [SimThingId(444)], "pirate_generator_1": [SimThingId(450)], "terran_generator_1": [SimThingId(445)], "pirate_mine": [SimThingId(452)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpv5lWU7\stellaristhing_base.clause identity=fnv1a64:f06b9153291843ee:7489 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:48da411d262ba5cd:7690
PROFILE_FULL case=generator-withheld-restored identity=fnv1a64:752e29acb06739a8:21573 targets={"pirate_refinery": [SimThingId(504)], "terran_mine": [SimThingId(498)], "A1": [SimThingId(500)], "E1": [SimThingId(505)], "terran_generator_1": [SimThingId(496)], "stellaristhing_base": [SimThingId(518)], "pirate_generator_1": [SimThingId(501)], "pirate_generator_2": [SimThingId(502)], "terran": [SimThingId(494)], "terran_refinery": [SimThingId(499)], "terran_generator_2": [SimThingId(497)], "pirate": [SimThingId(495)], "pirate_mine": [SimThingId(503)]}
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
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpdQ1MyX\stellaristhing_base.clause identity=fnv1a64:f636fd8ec702a341:9859 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:b345178eed8f1aae:12518
PROFILE_FULL case=native-funded-birth identity=fnv1a64:247bd751264ba70a:26980 targets={"terran_shipyard": [SimThingId(547)], "terran_generator_1": [SimThingId(548)], "terran_mine": [SimThingId(550)], "pirate_generator_2": [SimThingId(555)], "pirate_generator_1": [SimThingId(554)], "A1": [SimThingId(552)], "terran": [SimThingId(545)], "E1": [SimThingId(558)], "terran_refinery": [SimThingId(551)], "pirate_refinery": [SimThingId(557)], "pirate_mine": [SimThingId(556)], "stellaristhing_base": [SimThingId(573)], "terran_generator_2": [SimThingId(549)], "pirate": [SimThingId(546)], "pirate_shipyard": [SimThingId(553)]}
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
test rehearsal_economy_fleet_refinery_retains_every_authored_cost ... SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmp8HZ2KI\stellaristhing_base.clause identity=fnv1a64:42ee87791f36157c:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=minerals-then-energy recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=minerals-then-energy identity=fnv1a64:625f67b47a0416b4:21795 targets={"E1": [SimThingId(623)], "pirate_generator_1": [SimThingId(619)], "stellaristhing_base": [SimThingId(636)], "pirate": [SimThingId(613)], "pirate_refinery": [SimThingId(622)], "terran": [SimThingId(612)], "pirate_mine": [SimThingId(621)], "terran_refinery": [SimThingId(617)], "terran_mine": [SimThingId(616)], "pirate_generator_2": [SimThingId(620)], "terran_generator_1": [SimThingId(614)], "terran_generator_2": [SimThingId(615)], "A1": [SimThingId(618)]}
CELL case=minerals-then-energy generation=0 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=minerals-then-energy generation=0 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=minerals-then-energy generation=0 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=minerals-then-energy generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [72, 164, 120, 86, 215, 75, 226, 11, 185, 2, 50, 250, 206, 243, 65, 104], incarnation: 1 }
CELL case=minerals-then-energy generation=1 host=terran id=612 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=minerals-then-energy generation=1 host=pirate id=613 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=A1 id=618 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=minerals-then-energy generation=1 host=E1 id=623 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=minerals-then-energy generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [72, 164, 120, 86, 215, 75, 226, 11, 185, 2, 50, 250, 206, 243, 65, 104], incarnation: 1 }
SOURCE path=C:\Users\mvorm\AppData\Local\Temp\.tmpm9S1aE\stellaristhing_base.clause identity=fnv1a64:33eaa5da797b5a26:5978 dependencies={"stellaristhing_base.base.json": "fnv1a64:c49f9ca3c8c75e77:20370", "stellaristhing_base.dependencies.json": "fnv1a64:2f064bfb3e043aa0:72"}
GAME_MODE_PROJECTION identity=fnv1a64:8d561ec899c02e11:9132
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_terran_refining hydrated=[("meridian", "energy", 1.0, Some("terran"), Named("balance")), ("meridian_material", "A1_minerals_quantity", 2.0, Some("A1"), Amount)]
AUTHORED_TWO_COSTS case=energy-then-minerals recipe=material_conversion_recipe_pirate_refining hydrated=[("meridian", "energy", 1.0, Some("pirate"), Named("balance")), ("meridian_material", "E1_minerals_quantity", 2.0, Some("E1"), Amount)]
PROFILE_FULL case=energy-then-minerals identity=fnv1a64:028e91b6ff82cfbb:21795 targets={"pirate_generator_2": [SimThingId(671)], "stellaristhing_base": [SimThingId(687)], "A1": [SimThingId(669)], "pirate_generator_1": [SimThingId(670)], "terran": [SimThingId(663)], "pirate": [SimThingId(664)], "pirate_refinery": [SimThingId(673)], "terran_mine": [SimThingId(667)], "terran_generator_2": [SimThingId(666)], "pirate_mine": [SimThingId(672)], "E1": [SimThingId(674)], "terran_refinery": [SimThingId(668)], "terran_generator_1": [SimThingId(665)]}
CELL case=energy-then-minerals generation=0 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=10
CELL case=energy-then-minerals generation=0 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=8
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=4
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=3
CELL case=energy-then-minerals generation=0 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=3
CAPACITY case=energy-then-minerals generation=0 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [60, 234, 42, 252, 169, 87, 41, 151, 243, 173, 98, 244, 251, 225, 6, 196], incarnation: 1 }
CELL case=energy-then-minerals generation=1 host=terran id=663 slot=Some(SlotIndex(2)) property=meridian::energy role=Named("balance") value=9
CELL case=energy-then-minerals generation=1 host=pirate id=664 slot=Some(SlotIndex(3)) property=meridian::energy role=Named("balance") value=7
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=A1 id=669 slot=Some(SlotIndex(26)) property=meridian_material::A1_alloys_quantity role=Amount value=5
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_minerals_quantity role=Amount value=4
CELL case=energy-then-minerals generation=1 host=E1 id=674 slot=Some(SlotIndex(31)) property=meridian_material::E1_alloys_quantity role=Amount value=4
CAPACITY case=energy-then-minerals generation=1 live_rows=36 allocator_capacity=36 execution=PersistedTreeExecutionIdentity { realm_bytes: [60, 234, 42, 252, 169, 87, 41, 151, 243, 173, 98, 244, 251, 225, 6, 196], incarnation: 1 }
CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution
ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 26.20s
```

## Once-rebase reflog

```text
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
```

## Scope blob checks

```text
SEALED_COMPONENTS_UNCHANGED 32
HISTORICAL_PACKETS_UNCHANGED 8
SCENARIO_ENGINE_CARGO_UNCHANGED true; INVENTORY_UNCHANGED_FROM_REBASED_HEAD true
BASE a14d4a57813952781ec475cfd32d7c4f80fadfda
HEAD 4d6f43a7a968698ea6a4da74426d039731c0938d
TREE b28175a71c9aa14a69a95a39ce19f9550c1c7b1a
```

## Code-head AGENT-SCAN

```text
DOCTRINE SCAN REPORT  (commit 4d6f43a7, 2026-09-19T19:39:18Z)
  scanner self-test: SKIPPED
  scan mode: PR delta (a14d4a57..4d6f43a7)
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
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=58s
```

## Inventory scope failure (exit 1)

```text
TEST-INVENTORY-DRIFT-CHECK REPORT
  rows: 1119
  discovered: 1120
  unledgered: 1
  parked (pen, accounted): 0
  stale: 0
  promotion-target rows: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL
  - unledgered test rows: 1; remedy: add a classified ledger row or remove the test; first=[('simthing-workshop', 'crates/simthing-workshop/tests/native_structural_products_session_0.rs', 'sequential_capacity_exhaustion_preserves_prior_births', 'integration')]
```

## Rendered incorporated HD

```text
# HD Projection: coding
rung: 0088-BONSAI-CAPACITY-INTEGRATION-0
track: 0.0.8.8
kind: remedial
audience: coding
model_tier: frontier
base_sha: da1d7c1535f8953cde4ac70703a4a5f8313a0734
expected_route: ORCHESTRATOR-CLEARABLE(rehearsal-lifecycle-integration)
owner_approved: true
HD-RECEIPT: c0c0bf05feb2
REQUIRED-ANCHORS: none
owner_notes:
Owner directs Astra / Frontier to take over the Bonsai capacity-refusal track, preserve useful work, repair the proof gaps, clean trial residue, and integrate the accepted result into the live 2.2 #2075 lineage. SOLO ASTRA only. HOLD coding until DA relay 5743461789 is discharged and Orchestration releases the exact current #2075 base.
owner_directives:
- none
stop_conditions:
- DA relay 5743461789 is still pending or Orchestration has not issued an exact release base
- the cleaned proof needs any file outside the single allowed test file
- the released runtime changes the threshold, placement, or refusal semantics materially enough that this handoff would need redesign
- the actual first refusal is not attributable to exhausted ordinary residency capacity under the existing law
- a production fix or new authority would be required to make the capacity proof pass
required_checks:
- HOLD: before coding, wait for Orchestration release after DA relay 5743461789; then bind to the exact released #2075 head/base and carry the current Astra coding orientation receipt
- review Bonsai source commit 64af8ebcfce2d9ac196f0e371e8eac39fd439438 against base 60830eef4907b2bc9a8bf94325f9b50f8891d528; do not assume its proof claims are accepted
- task anchors: rehearsal-0088-acceptances@3d95d334fafe, rehearsal-0088-construction-contract@bdd51c7c4ae2, rehearsal-0088-binding-laws@25af224d1deb, structural-execution-convergence@6b4cedec482b
- preserve the useful sequential exhaustion structure: successful native funded 3-row births, exact +3 live/-3 capacity per birth, first refusal adds no identity, refusal step leaves live/capacity unchanged, and prior births survive
- repair funding causality: with N successful product births before first refusal, next product threshold is N + 0.5 per structural_product lowering; prove the funding shadow crosses that exact threshold at the causal boundary, not merely value > 0
- repair placement preservation: snapshot the full CommittedResidencyPlacement for every prior born root before and after first refusal and require exact equality of identity, extent, quantity, and committed_generation
- repair refusal specificity: on the released live base require the exact ordinary capacity-refusal reason actually produced; current reference behavior is MarketUnresolved { granted: 0 } with remaining capacity below one 3-row subtree; do not accept arbitrary Placement(_) as equivalent
- clean the test: remove redundant diagnostic vectors/println/commentary where not needed for the three repaired claims; keep enough failure output to diagnose a RED
- run cargo check --locked -p simthing-workshop --test native_structural_products_session_0; run the focused sequential_capacity_exhaustion_preserves_prior_births test; run the full native_structural_products_session_0 target; report exact counts and exits
- integrate the cleaned accepted test into the same #2075 branch after review; rerun it again on the final integrated #2075 head and record any substantive changes from Bonsai's original commit
forbidden_surfaces:
- all files except crates/simthing-workshop/tests/native_structural_products_session_0.rs
- no production/spec/driver/kernel/gpu/clausething changes; this handoff salvages a settled proof only
- no scenario, Cargo, CI/gate, anchor, workplan, inventory, generated-orientation, or sealed edits
- do not preserve Bonsai tooling/posting/environment scaffolding, local-path assumptions, or trial commentary that is not load-bearing Rust proof
- do not broaden refusal matching to make the test pass; STOP on changed semantics
- no merge or rung graduation from this remedial alone; integrate into #2075 only after Orchestration release
## BUILD
Take over Bonsai's one-file capacity-refusal proof and turn it into mainline-quality 2.2
evidence. Use commit 64af8ebc as source material, keep its useful sequential exhaustion
observations, and repair the three review gaps identified by Astra. The finished proof must
bind exact funded-candidate causality, exact committed-placement preservation, and the actual
capacity-refusal discriminator on the live released #2075 base.

Then clean away trial-only verbosity and integrate the repaired test into the SAME #2075
lineage. This is proof hardening and integration only; it does not authorize RF-enrollment,
allocation, structural, or persistence changes.

## FENCES
- Solo Astra / Frontier; the Bonsai parallel exception is expired.
- One test file only.
- The Bonsai commit is evidence/source material, not an accepted completion claim.
- Preserve strict separation of funding, entitlement, placement, and refusal facts.
- No broad alternate-match assertions, synthetic capacity law, or second market/allocator.
- No merge/graduation until the full parent 2.2 contract is independently green.

## EXIT-PROOF
On the released #2075 base, the cleaned focused test and full native structural-product target
compile and pass; the next candidate's exact N+0.5 funding threshold is causally crossed; the
first exhausted generation returns the exact lawful capacity refusal with no birth and no
allocator/identity mutation; every prior CommittedResidencyPlacement is byte/field-equivalent
pre/post; and the resulting one-file patch is integrated into #2075 with command evidence and
a concise record of what Astra changed from Bonsai's original commit.
```
