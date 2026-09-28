# Pin-19 continuation raw evidence


Tested code `beaa99b97f1f68149368951a3a04c0ff12295cb8`; baseline `ee805483b1a0b1bc3591e9f04bef573e8f9389e7`. Required semantic status: FAIL / STOP. No ignored-test bypass.


Full logs are retained at `C:/Users/mvorm/SimThing/.git/economy-fleet-22-pin19-resume`. The following are verbatim selected records, not complete logs. Hashes below bind the complete files.


## sync-conjunction

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_refinery_retains_every_authored_cost -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 82.92 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 1m 18s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.69s

```

## sync-stock

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_generator_stock_preserves_frozen_economy -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 3.09 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 0.59s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.25s

```

## sync-birth

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_native_funded_output_must_birth_fleets -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 4.63 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 0.57s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.80s

```

## sync-rf

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 3.45 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 0.57s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.64s

```

## sync-capacity

```text

COMMAND: cargo test --locked -p simthing-workshop --test native_structural_products_session_0 sequential_capacity_exhaustion_preserves_prior_births -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 21.83 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 17.67s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 2.86s

```

## sync-native

```text

COMMAND: cargo test --locked -p simthing-workshop --test native_structural_products_session_0 -- --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 23.61 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 0.57s

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.38s

```

## sync-parent

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 13.95 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `test` profile [optimized + debuginfo] target(s) in 0.55s

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.09s

```

## sync-inventory

```text

COMMAND: C:/Program Files/Git/bin/bash.exe scripts/ci/test_inventory_drift_check.sh

EXIT: 0 WALL_SECONDS: 4.24 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

TEST-INVENTORY-DRIFT-CHECK REPORT

  rows: 1139

  discovered: 1139

  unledgered: 0

  stale: 0

TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS

```

## sync-check

```text

COMMAND: cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet --test native_structural_products_session_0

EXIT: 0 WALL_SECONDS: 12.13 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

    Finished `dev` profile [optimized + debuginfo] target(s) in 12.03s

```

## sync-scan

```text

COMMAND: C:/Program Files/Git/bin/bash.exe scripts/ci/agent_scan.sh --base 681600b0cb670b38e63ece5380927c1b0bede3f9 --head ee805483b1a0b1bc3591e9f04bef573e8f9389e7

EXIT: 0 WALL_SECONDS: 53.5 HEAD: ee805483b1a0b1bc3591e9f04bef573e8f9389e7 TREE: ed079eeaa24bfd4559b901b2bb650b481c6d80a4

DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED

AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=53s

```

## final-stock

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_generator_stock_preserves_frozen_economy -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 22.55 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

    Finished `test` profile [optimized + debuginfo] target(s) in 12.71s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 8.41s

```

## final-birth

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_native_funded_output_must_birth_fleets -- --exact --nocapture --test-threads=1

EXIT: 0 WALL_SECONDS: 10.47 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

    Finished `test` profile [optimized + debuginfo] target(s) in 0.53s

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 9.66s

```

## final-rf-alloy

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow -- --exact --nocapture --test-threads=1

EXIT: 101 WALL_SECONDS: 3.26 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

    Finished `test` profile [optimized + debuginfo] target(s) in 0.54s

2.2 STOP: canonical refinery output cannot target the existing sole alloy Balance through native authoring; full 0.2-alloy upkeep remains unexecuted: SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpvibenc\\stellaristhing_base.clause: ClauseThing hydration error at token 720: unsupported output field `entity`")

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.48s

```

## final-parent

```text

COMMAND: cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1

EXIT: 101 WALL_SECONDS: 27.96 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

    Finished `test` profile [optimized + debuginfo] target(s) in 0.56s

2.2 STOP: canonical refinery output cannot target the existing sole alloy Balance through native authoring; full 0.2-alloy upkeep remains unexecuted: SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmppyuYgf\\stellaristhing_base.clause: ClauseThing hydration error at token 720: unsupported output field `entity`")

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 27.12s

```

## final-inventory

```text

COMMAND: C:/Program Files/Git/bin/bash.exe scripts/ci/test_inventory_drift_check.sh

EXIT: 0 WALL_SECONDS: 0.37 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

TEST-INVENTORY-DRIFT-CHECK REPORT

  rows: 1139

  discovered: 1139

  unledgered: 0

  stale: 0

TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS

```

## final-check

```text

COMMAND: cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet --test native_structural_products_session_0

EXIT: 0 WALL_SECONDS: 1.14 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

    Finished `dev` profile [optimized + debuginfo] target(s) in 1.06s

```

## final-scan

```text

COMMAND: C:/Program Files/Git/bin/bash.exe scripts/ci/agent_scan.sh --base 681600b0cb670b38e63ece5380927c1b0bede3f9 --head beaa99b97f1f68149368951a3a04c0ff12295cb8

EXIT: 0 WALL_SECONDS: 51.42 HEAD: beaa99b97f1f68149368951a3a04c0ff12295cb8 TREE: c55ec542c38d393962ce46a9ee8eb8367c37680b

DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED

AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=50s

```

## Selected final-stock.txt

```text

LOCAL_STORAGE_SOURCE case=local-omitted identity=fnv1a64:e419dffbafa5b584:7118

LOCAL_STORAGE case=local-omitted owner=terran generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=21 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-omitted owner=pirate generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=15 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-omitted owner=terran generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-omitted owner=pirate generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=18 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-omitted owner=terran generation=11 spend=2 offered=3 produced=3 curtailed=0 stock=31 cumulative_produced=33 cumulative_curtailed=0 batches=11

LOCAL_STORAGE case=local-omitted owner=pirate generation=11 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=33 cumulative_curtailed=0 batches=11

LOCAL_STORAGE case=local-omitted owner=terran generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=29 cumulative_produced=33 cumulative_curtailed=0 batches=12

LOCAL_STORAGE case=local-omitted owner=pirate generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=23 cumulative_produced=33 cumulative_curtailed=0 batches=12

LOCAL_STORAGE case=local-omitted owner=terran generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=7 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-omitted owner=pirate generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=1 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-omitted owner=terran generation=25 spend=2 offered=0 produced=0 curtailed=0 stock=3 cumulative_produced=33 cumulative_curtailed=0 batches=25

LOCAL_STORAGE case=local-omitted owner=pirate generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=1 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-omitted owner=terran generation=26 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=36 cumulative_curtailed=0 batches=26

LOCAL_STORAGE case=local-omitted owner=pirate generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=36 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-omitted owner=terran generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=5 cumulative_produced=39 cumulative_curtailed=0 batches=27

LOCAL_STORAGE case=local-omitted owner=pirate generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=5 cumulative_produced=39 cumulative_curtailed=0 batches=24

LOCAL_STORAGE case=local-omitted owner=terran generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=99 cumulative_curtailed=0 batches=47

LOCAL_STORAGE case=local-omitted owner=pirate generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=99 cumulative_curtailed=0 batches=44

LOCAL_STORAGE case=local-omitted owner=terran generation=50 spend=2 offered=3 produced=3 curtailed=0 stock=28 cumulative_produced=108 cumulative_curtailed=0 batches=50

LOCAL_STORAGE case=local-omitted owner=pirate generation=50 spend=2 offered=3 produced=3 curtailed=0 stock=28 cumulative_produced=108 cumulative_curtailed=0 batches=47

LOCAL_STORAGE_SOURCE case=local-unbounded identity=fnv1a64:12500f08648c3e6c:7136

LOCAL_STORAGE case=local-unbounded owner=terran generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=21 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-unbounded owner=pirate generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=15 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-unbounded owner=terran generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-unbounded owner=pirate generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=18 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-unbounded owner=terran generation=11 spend=2 offered=3 produced=3 curtailed=0 stock=31 cumulative_produced=33 cumulative_curtailed=0 batches=11

LOCAL_STORAGE case=local-unbounded owner=pirate generation=11 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=33 cumulative_curtailed=0 batches=11

LOCAL_STORAGE case=local-unbounded owner=terran generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=29 cumulative_produced=33 cumulative_curtailed=0 batches=12

LOCAL_STORAGE case=local-unbounded owner=pirate generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=23 cumulative_produced=33 cumulative_curtailed=0 batches=12

LOCAL_STORAGE case=local-unbounded owner=terran generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=7 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-unbounded owner=pirate generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=1 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-unbounded owner=terran generation=25 spend=2 offered=0 produced=0 curtailed=0 stock=3 cumulative_produced=33 cumulative_curtailed=0 batches=25

LOCAL_STORAGE case=local-unbounded owner=pirate generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=1 cumulative_produced=33 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-unbounded owner=terran generation=26 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=36 cumulative_curtailed=0 batches=26

LOCAL_STORAGE case=local-unbounded owner=pirate generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=36 cumulative_curtailed=0 batches=23

LOCAL_STORAGE case=local-unbounded owner=terran generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=5 cumulative_produced=39 cumulative_curtailed=0 batches=27

LOCAL_STORAGE case=local-unbounded owner=pirate generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=5 cumulative_produced=39 cumulative_curtailed=0 batches=24

LOCAL_STORAGE case=local-unbounded owner=terran generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=99 cumulative_curtailed=0 batches=47

LOCAL_STORAGE case=local-unbounded owner=pirate generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=25 cumulative_produced=99 cumulative_curtailed=0 batches=44

LOCAL_STORAGE case=local-unbounded owner=terran generation=50 spend=2 offered=3 produced=3 curtailed=0 stock=28 cumulative_produced=108 cumulative_curtailed=0 batches=50

LOCAL_STORAGE case=local-unbounded owner=pirate generation=50 spend=2 offered=3 produced=3 curtailed=0 stock=28 cumulative_produced=108 cumulative_curtailed=0 batches=47

LOCAL_STORAGE_SOURCE case=local-bounded identity=fnv1a64:e6c88d7e4c453f31:7155

LOCAL_STORAGE case=local-bounded owner=terran generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=21 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-bounded owner=pirate generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=15 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-bounded owner=terran generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-bounded owner=pirate generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=18 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-bounded owner=terran generation=11 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=26 cumulative_curtailed=7 batches=11

LOCAL_STORAGE case=local-bounded owner=pirate generation=11 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=32 cumulative_curtailed=1 batches=11

LOCAL_STORAGE case=local-bounded owner=terran generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=22 cumulative_produced=26 cumulative_curtailed=7 batches=12

LOCAL_STORAGE case=local-bounded owner=pirate generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=22 cumulative_produced=32 cumulative_curtailed=1 batches=12

LOCAL_STORAGE case=local-bounded owner=terran generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=26 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded owner=pirate generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=32 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded owner=terran generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=26 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded owner=pirate generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=32 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded owner=terran generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=3 cumulative_produced=29 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded owner=pirate generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=3 cumulative_produced=35 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded owner=terran generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=32 cumulative_curtailed=7 batches=24

LOCAL_STORAGE case=local-bounded owner=pirate generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=38 cumulative_curtailed=1 batches=24

LOCAL_STORAGE case=local-bounded owner=terran generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=92 cumulative_curtailed=7 batches=44

LOCAL_STORAGE case=local-bounded owner=pirate generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=98 cumulative_curtailed=1 batches=44

LOCAL_STORAGE case=local-bounded owner=terran generation=50 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=98 cumulative_curtailed=10 batches=47

LOCAL_STORAGE case=local-bounded owner=pirate generation=50 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=104 cumulative_curtailed=4 batches=47

LOCAL_STORAGE_SOURCE case=local-bounded-cache identity=fnv1a64:e6c88d7e4c453f31:7155

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=21 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=1 spend=2 offered=3 produced=3 curtailed=0 stock=15 cumulative_produced=3 cumulative_curtailed=0 batches=1

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=4 spend=2 offered=3 produced=3 curtailed=0 stock=18 cumulative_produced=12 cumulative_curtailed=0 batches=4

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=11 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=26 cumulative_curtailed=7 batches=11

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=11 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=32 cumulative_curtailed=1 batches=11

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=22 cumulative_produced=26 cumulative_curtailed=7 batches=12

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=12 spend=2 offered=0 produced=0 curtailed=0 stock=22 cumulative_produced=32 cumulative_curtailed=1 batches=12

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=26 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=23 spend=2 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=32 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=26 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=25 spend=0 offered=0 produced=0 curtailed=0 stock=0 cumulative_produced=32 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=3 cumulative_produced=29 cumulative_curtailed=7 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=26 spend=0 offered=3 produced=3 curtailed=0 stock=3 cumulative_produced=35 cumulative_curtailed=1 batches=23

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=32 cumulative_curtailed=7 batches=24

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=27 spend=2 offered=3 produced=3 curtailed=0 stock=4 cumulative_produced=38 cumulative_curtailed=1 batches=24

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=92 cumulative_curtailed=7 batches=44

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=47 spend=2 offered=3 produced=3 curtailed=0 stock=24 cumulative_produced=98 cumulative_curtailed=1 batches=44

LOCAL_STORAGE case=local-bounded-cache owner=terran generation=50 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=98 cumulative_curtailed=10 batches=47

LOCAL_STORAGE case=local-bounded-cache owner=pirate generation=50 spend=2 offered=3 produced=2 curtailed=1 stock=24 cumulative_produced=104 cumulative_curtailed=4 batches=47

```

## Selected final-birth.txt

```text

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=1 refined=0 funded_delta=0 funded_total=0 births=0 minerals=23 alloys=4 refinery_energy=0 yard_energy=12

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=1 refined=0 funded_delta=0 funded_total=0 births=0 minerals=17 alloys=3 refinery_energy=0 yard_energy=10

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=4 refined=0 funded_delta=0 funded_total=0 births=0 minerals=32 alloys=4 refinery_energy=0 yard_energy=18

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=4 refined=0 funded_delta=0 funded_total=0 births=0 minerals=26 alloys=3 refinery_energy=0 yard_energy=16

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=5 refined=0 funded_delta=0 funded_total=0 births=0 minerals=35 alloys=4 refinery_energy=0 yard_energy=20

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=5 refined=0 funded_delta=0 funded_total=0 births=0 minerals=29 alloys=3 refinery_energy=0 yard_energy=18

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=6 refined=0 funded_delta=0 funded_total=0 births=0 minerals=38 alloys=4 refinery_energy=1 yard_energy=21

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=6 refined=0 funded_delta=0 funded_total=0 births=0 minerals=32 alloys=3 refinery_energy=1 yard_energy=19

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=7 refined=1 funded_delta=0 funded_total=0 births=0 minerals=39 alloys=5 refinery_energy=1 yard_energy=22

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=7 refined=1 funded_delta=0 funded_total=0 births=0 minerals=33 alloys=4 refinery_energy=1 yard_energy=20

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=9 refined=1 funded_delta=1 funded_total=1 births=0 minerals=41 alloys=1 refinery_energy=1 yard_energy=20

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=10 refined=1 funded_delta=1 funded_total=1 births=0 minerals=36 alloys=1 refinery_energy=1 yard_energy=19

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=15 refined=1 funded_delta=1 funded_total=2 births=1 minerals=47 alloys=1 refinery_energy=1 yard_energy=22

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=16 refined=1 funded_delta=1 funded_total=2 births=1 minerals=42 alloys=1 refinery_energy=1 yard_energy=21

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=21 refined=1 funded_delta=1 funded_total=3 births=2 minerals=53 alloys=1 refinery_energy=1 yard_energy=24

FLEET_RECOVERY case=fleet-alloy-recovery owner=terran generation=22 refined=1 funded_delta=0 funded_total=3 births=2 minerals=54 alloys=2 refinery_energy=1 yard_energy=25

FLEET_RECOVERY case=fleet-alloy-recovery owner=pirate generation=22 refined=1 funded_delta=1 funded_total=3 births=2 minerals=48 alloys=1 refinery_energy=1 yard_energy=23

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=1 refined=1 funded_delta=0 funded_total=0 births=0 minerals=21 alloys=5 refinery_energy=11 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=1 refined=1 funded_delta=0 funded_total=0 births=0 minerals=15 alloys=4 refinery_energy=9 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=4 refined=1 funded_delta=0 funded_total=0 births=0 minerals=24 alloys=8 refinery_energy=14 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=4 refined=1 funded_delta=0 funded_total=0 births=0 minerals=18 alloys=7 refinery_energy=12 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=5 refined=1 funded_delta=0 funded_total=0 births=0 minerals=25 alloys=9 refinery_energy=15 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=5 refined=1 funded_delta=0 funded_total=0 births=0 minerals=19 alloys=8 refinery_energy=13 yard_energy=0

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=6 refined=1 funded_delta=0 funded_total=0 births=0 minerals=26 alloys=10 refinery_energy=15 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=6 refined=1 funded_delta=0 funded_total=0 births=0 minerals=20 alloys=9 refinery_energy=13 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=7 refined=1 funded_delta=0 funded_total=0 births=0 minerals=27 alloys=11 refinery_energy=15 yard_energy=2

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=7 refined=1 funded_delta=0 funded_total=0 births=0 minerals=21 alloys=10 refinery_energy=13 yard_energy=2

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=10 refined=1 funded_delta=1 funded_total=1 births=0 minerals=30 alloys=8 refinery_energy=15 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=10 refined=1 funded_delta=1 funded_total=1 births=0 minerals=24 alloys=7 refinery_energy=13 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=14 refined=1 funded_delta=1 funded_total=2 births=1 minerals=34 alloys=6 refinery_energy=15 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=14 refined=1 funded_delta=1 funded_total=2 births=1 minerals=28 alloys=5 refinery_energy=13 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=18 refined=1 funded_delta=1 funded_total=3 births=2 minerals=38 alloys=4 refinery_energy=15 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=18 refined=1 funded_delta=1 funded_total=3 births=2 minerals=32 alloys=3 refinery_energy=13 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=terran generation=22 refined=1 funded_delta=1 funded_total=4 births=2 minerals=42 alloys=2 refinery_energy=15 yard_energy=1

FLEET_RECOVERY case=fleet-energy-work-recovery owner=pirate generation=22 refined=1 funded_delta=1 funded_total=4 births=2 minerals=36 alloys=1 refinery_energy=13 yard_energy=1

FLEET_DISPOSITION generation=39 owner=terran funded_receipt=7 born=6 refused=0 pending=1 alloys=1 energy_work=11

FLEET_DISPOSITION generation=39 owner=pirate funded_receipt=6 born=6 refused=0 pending=0 alloys=6 energy_work=15

FLEET_REFUSAL generation=40 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(481), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(40), revalue_generation: GenerationStamp(41), market_grant_key: Some(7025569874442022758), reason: MarketUnresolved { granted: 2 } }

FLEET_DISPOSITION generation=40 owner=terran funded_receipt=7 born=6 refused=1 pending=0 alloys=2 energy_work=12

FLEET_DISPOSITION generation=40 owner=pirate funded_receipt=7 born=6 refused=0 pending=1 alloys=1 energy_work=12

FLEET_REFUSAL generation=41 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(631), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(41), revalue_generation: GenerationStamp(42), market_grant_key: Some(16740960155111572611), reason: MarketUnresolved { granted: 2 } }

FLEET_DISPOSITION generation=41 owner=terran funded_receipt=7 born=6 refused=1 pending=0 alloys=3 energy_work=13

FLEET_DISPOSITION generation=41 owner=pirate funded_receipt=7 born=6 refused=1 pending=0 alloys=2 energy_work=13

FLEET_REFUSAL generation=46 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(484), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(46), revalue_generation: GenerationStamp(47), market_grant_key: Some(11636295187174612529), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=47 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(634), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(47), revalue_generation: GenerationStamp(48), market_grant_key: Some(13839548136210990000), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=52 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(487), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(52), revalue_generation: GenerationStamp(53), market_grant_key: Some(9192708388472105304), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=53 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(637), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(53), revalue_generation: GenerationStamp(54), market_grant_key: Some(12090081982300368025), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=58 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(490), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(58), revalue_generation: GenerationStamp(59), market_grant_key: Some(17998322741940010683), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=59 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(640), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(59), revalue_generation: GenerationStamp(60), market_grant_key: Some(17033691333994748950), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=64 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(493), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(64), revalue_generation: GenerationStamp(65), market_grant_key: Some(3103297209329928666), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=65 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(643), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(65), revalue_generation: GenerationStamp(66), market_grant_key: Some(8379326086540538071), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=70 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(496), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(70), revalue_generation: GenerationStamp(71), market_grant_key: Some(15024054092157148693), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=71 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(646), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(71), revalue_generation: GenerationStamp(72), market_grant_key: Some(5929014306479304436), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=76 faction=0 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(415), grantee: SimThingId(499), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(76), revalue_generation: GenerationStamp(77), market_grant_key: Some(3922946558217029052), reason: MarketUnresolved { granted: 2 } }

FLEET_REFUSAL generation=77 faction=1 fact=OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(421), grantee: SimThingId(649), quantity: 3, origin: AddChild }, attempted_generation: GenerationStamp(77), revalue_generation: GenerationStamp(78), market_grant_key: Some(17068472166951846717), reason: MarketUnresolved { granted: 2 } }

FLEET_DISPOSITION generation=80 owner=terran funded_receipt=13 born=6 refused=7 pending=0 alloys=6 energy_work=28

FLEET_DISPOSITION generation=80 owner=pirate funded_receipt=13 born=6 refused=7 pending=0 alloys=5 energy_work=28

```

## Selected final-rf-alloy.txt

```text

UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44

UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44

UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44

UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=0.5 yard_rate=0.5 surplus=1 live=44

ALLOY_STOCK_INGRESS_SOURCE identity=fnv1a64:f0c9b42501b10814:10230

2.2 STOP: canonical refinery output cannot target the existing sole alloy Balance through native authoring; full 0.2-alloy upkeep remains unexecuted: SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; file \\\\?\\C:\\Users\\mvorm\\AppData\\Local\\Temp\\.tmpvibenc\\stellaristhing_base.clause: ClauseThing hydration error at token 720: unsupported output field `entity`")

```

## Selected sync-capacity.txt

```text

CAPACITY_END births=12 first_refusal=Some(17) later_refusals=17 live0=36 capacity0=36

```

## Orientation and synchronization record

```text

ORIENT-SINCE-VERDICT: CURRENT
supplied_receipt: f86020d2b2ae
current_receipt: f86020d2b2ae
role: coding
orientation_rule_stamp: 14f47e2289d3be96
orientation_digest_sha: abf87af75e48f20a87b7aa243f0ab971928962aa35aaf8b35f19216327e14e4f  # informational


ee805483b1a0b1bc3591e9f04bef573e8f9389e7 rebase (finish): returning to refs/heads/codex/0088-economy-fleet
ee805483b1a0b1bc3591e9f04bef573e8f9389e7 rebase (pick): docs: record post-sync GPU driver qualification STOP
3a779f553bf2317952d9979e671d4cda7b128531 rebase (pick): docs: record inventory correction and native stock bound STOP
7291228726292d36cfea7fdf37a27f5159dd8885 rebase (pick): test: expose native bounded stock authoring gap
38632b9863028da9a94c63422305c20be8327eb1 rebase (pick): test: ledger the authorized native capacity exhaustion proof
4baef9395e17099e6bb8cc4c3c5259b84d9cfd51 rebase (pick): docs: return RF capacity proof and explicit inventory scope STOP
cc749343a5edc6b5c235ba0754c6f81157423ff4 rebase (pick): test: harden born RF admission and salvage exact capacity proof
31457a112ebf452c982cb01bebbc8e84a7079f9b rebase (pick): docs: record native birth proof and post-birth RF STOP
dbb907cc0c44e879d640e1a0d420d913a92d842c rebase (pick): test: prove native fleet births and expose missing RF enrollment
897e261963b1c85a1fcaad42150d2911ea4544d4 rebase (pick): docs: bind final table-driven 2.2 STOP execution
f1729665dbdb33ff786c2ec639a6b19b36d9dbee rebase (pick): test: express capped recovery expectations as explicit case data
528b85f1f3fee0c66db62fdb97553435a501c0a2 rebase (pick): docs: record capped economy GREEN and native funded-birth STOP
9d9647cb4d8b1ff7b2adc3445c17ee11fbab1c6c rebase (pick): test: prove capped refinery and expose native funded-birth ingress gap
b04f59b2ccf427761f28e48e45988e4979f0fbfa rebase (pick): docs: record 2.2 resumed accounting and refinery throughput STOP
7d053e8cbcb0b2e0b12933b9173bc1a811344a40 rebase (pick): test: resume 2.2 conjunction and expose missing refinery rate cap
f9a461f6f4689f5c76eda376bdcb0b46b7dd0695 rebase (pick): docs: return 2.2 STOP with native recipe input-loss evidence
bddd112677e464308ba4cf0530afcdc276a1b6fa rebase (pick): test: expose 2.2 refinery authoring input loss on the Meridian asset
681600b0cb670b38e63ece5380927c1b0bede3f9 rebase (start): checkout 681600b0cb670b38e63ece5380927c1b0bede3f9
119c8e75debb801d7a4ee7c7f0851f76622efff3 commit: docs: record post-sync GPU driver qualification STOP
067aae6e6b16a2c1be04e07131fddf738819f0de rebase (finish): returning to refs/heads/codex/0088-economy-fleet


```

## Complete raw-log SHA-256 manifest

| File | SHA-256 |
| --- | --- |
| alloy-ingress-focus.txt | `99d515d17039f1ab9d6d20f9761db2d51987faa444d14e88f9984b532bd3fa8a` |
| final-birth.txt | `73c74eb91510942586a7f93fe8dbe62c845a9f3fba090582d2b58c9465942f1d` |
| final-check.txt | `41be848b2c71c3e35ebb35b78dc2e5547ec1f6a0ab728e0f12135861bdcccc22` |
| final-inventory.txt | `4ae5bd58d4a9b3ff4a726190d4b18798946df9afa2eb3e85ee6b0b98b789dc4b` |
| final-parent.txt | `184bcd9c2c31c03d5f3113de80f228c9cbb51a4a11d6e35b5ca7dd54cec66909` |
| final-rf-alloy.txt | `82340013eb046aca513a13c02a88461b0af2475c74eae115ab233c3748d70ea0` |
| final-scan.txt | `62eff370431fb5de2495c947fc6ae7dd4995563b7c3038525613f3e876fc2894` |
| final-stock.txt | `10e06f829ea177e44cdd976ff6824171fe9e5ff9459468a460a72c2219f76ede` |
| orientation-current.txt | `76dd854ca6ad6f5a8bf660145776e1c80bfee7e705bcc75976dd49e42585eb7b` |
| orientation.txt | `2d8d2b5f6e770febfe109eef6ddba5fa6c41fb39e84d2132c1079428b3907a1a` |
| rebase-reflog.txt | `efb9d7e141501ece6601d88d410f0204f18e8080c4b78f21c4a2545336b870cb` |
| recovery-focused-attempt1.txt | `aa1dd6eef05864265bddff153d4ac59bc4b2768b0154dab462a5e5df7ec4af49` |
| recovery-focused-attempt2.txt | `09ac74f7ba8d0d57462adf69766a95bf369498d49ecbe96c7a0832e7bfaf7323` |
| refusal-focused-attempt1.txt | `bc5c6b5f89ec1dce60e00114c2313690b8810b1e896221b715c027eca2b4dbba` |
| refusal-focused-attempt2.txt | `0e6fca96a7d05dd0170b3b1cedacae984cd9ca596c9aa24198eb10de098309bc` |
| storage-check.txt | `7a50c712526c7ccdb174ff77bc1abd7e4d0f916bffe29132c51babe5deecf4a4` |
| storage-focused-attempt1.txt | `fa899e293fadc514087e98ec21217e6c176f59d6af18ba74416b36861546513a` |
| storage-focused-attempt2.txt | `69da839a99580984be3cf0ad9cfedb90a3c78a9598fddfa4571af389b035065a` |
| storage-focused-attempt3.txt | `ee407199dbb05d146128328300c3bdb044677def28d20afe6ecc3f4617bb1445` |
| storage-focused-attempt4.txt | `63b21f43c2f6f5eed29e89f083cf5c1e1b70a7c3f5936eb0a7c88b9506c4d9b2` |
| sync-birth.txt | `133e50f74a486b76987d8aff37592e6094564f31a33fae5bea3c30511e442076` |
| sync-capacity.txt | `bb52dcf202daf4b2b76eadd651406145e58fb9febfcdde27605d4298d33374cd` |
| sync-check.txt | `4f65684de326306f068956233db0c073e0fa4be2c2b2a592fe71b09239ff69db` |
| sync-conjunction.txt | `13e32447c149ee4e4edac2c9acc6ece894684edcd1f8ec19e03e61674bf16d24` |
| sync-inventory.txt | `4ae5bd58d4a9b3ff4a726190d4b18798946df9afa2eb3e85ee6b0b98b789dc4b` |
| sync-native.txt | `b2193fe006b36ad00d96f5c044b7053ad41fe1405b51a2cd2d30e216d911f737` |
| sync-parent.txt | `f065d8a0897d94f99b5afce319948dff3f95539658d0636ef126b456e6cfb2ea` |
| sync-rf.txt | `8c81a5f4df813dacf86b24231e2f555872f58c971e53319b2476be14d84cb948` |
| sync-scan.txt | `379005e6bcddd29827210e33a1a749651bae7efe8c3e63a752bf79357b5f1107` |
| sync-stock.txt | `05433b58846381a02297375253986ce969920e605c9d6e1d421ecb4c1da2de03` |
