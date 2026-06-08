# SCENARIO-0080-2 — ATLAS-BATCH-0-STORE-GPU Status Row

| Date | Rung | Status | Evidence | Next |
|---|---|---|---|---|
| 2026-06-03 | `ATLAS-BATCH-0-STORE-GPU` | IMPLEMENTED / PASS (EC-A3-gpu ExactDeterministic bit-exact; discrete RTX) | `dress_rehearsal_atlas_batch_0_store_gpu.rs`; tests; [`scenario_0080_2_atlas_batch_0_store_gpu_report.md`](scenario_0080_2_atlas_batch_0_store_gpu_report.md); raw: [`scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_store_gpu_cargo_test_2026_06_03.txt); parity: [`scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt`](scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt); adapter **NVIDIA GeForce RTX 4080 Laptop GPU** (`SIMTHING_GPU_ADAPTER_CONTAINS=RTX`); 38/38 oracle entries bit-exact; prior Intel-only STORE-GPU log superseded | `ATLAS-BATCH-0-CLOSE` |

§0.5 posture: whitelisted `EvalEML`/`Sum` fixture only vs CPU `StoreOracle`; discrete RTX/NVIDIA adapter required (Intel fails); OWNER runtime + R3 parked; PACK-GPU Intel evidence unchanged.
