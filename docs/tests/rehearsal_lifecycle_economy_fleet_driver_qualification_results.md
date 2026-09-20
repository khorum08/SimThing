# 0088-ECONOMY-FLEET-0: synchronized base, driver-qualification STOP

Status: **STOP / PROBATION / proof-present / orchestration-review-pending**.
Same draft PR #2075, same branch `codex/0088-economy-fleet`, one coding agent.
No merge, graduation, Model 2 reopening, 2.3 dispatch, or final coding-agent clearance.

## Authority and exact synchronization

Sole operative Board handoff [5744331095](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744331095),
revision **2026-09-20T04:46:07Z**, releases exactly one additional rebase after DA
[5745970438](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5745970438).

- Pre-sync head: `a2fa980141409ea152085cf60f6f9cd3c2876363`; tree `6d39ab3b18b0f0e0346d74d1e1b740e5f02bcaaf`.
- Exact authorized base: `d3b58b7513ee9dd56ad361c02aac460f45df4b77` (#2083/#2084/#2085 landed).
- Post-sync / **tested_code_sha: 067aae6e6b16a2c1be04e07131fddf738819f0de**.
- Tested tree: `2e61089b6c38487441e2ff7f0cab7f54a69ffaf4`.
- One `rebase (start)` at that base, 15 picks, one `rebase (finish)` at this head;
  no conflict and no subsequent rebase. Complete reflog retained with the local evidence.
- Remote backup `codex/0088-economy-fleet-stop-a2fa9801` points to the pre-sync head.
  Previous STOP backup heads and Board return 5745459860 remain preserved.
- Both workshop tests and all 12 prior result documents are blob-identical to pre-sync.
  The accepted capacity inventory row is present exactly once and unchanged.
- This new continuation commits only this evidence document after the rebase. The final
  return binds its descendant head/tree and verifies code equivalence to the tested SHA.

coverage_basis: **FAIL** — ordered gate 3 refuses before activation; later gates and the
remaining 2.2 floor are not proved on the new base/runtime tuple.

## Required ordered execution

Every command below ran at the tested SHA in the stated order, with the ordinary production
qualification guard intact. No test was ignored or weakened. Counts are
passed / failed / ignored / measured / filtered. Seconds are test harness durations.

| Order | Owning test / target | Counts | Exit | Seconds |
| --- | --- | --- | --- | --- |
| 1 | `rehearsal_economy_fleet_refinery_retains_every_authored_cost` | 1 / 0 / 0 / 0 / 4 | 0 | 17.02 |
| 2 | `rehearsal_economy_fleet_generator_stock_preserves_frozen_economy` | 1 / 0 / 0 / 0 / 4 | 0 | 10.35 |
| 3 | `rehearsal_economy_fleet_native_funded_output_must_birth_fleets` | 0 / 1 / 0 / 0 / 4 | 101 | 1.57 |
| 4 | born-energy RF upkeep focus | NOT RUN after STOP | — | — |
| 5 | exact capacity focus | NOT RUN after STOP | — | — |
| 6 | full `native_structural_products_session_0` | NOT RUN after STOP | — | — |
| 7 | full parent `rehearsal_lifecycle_economy_fleet` | NOT RUN after STOP | — | — |

Commands (substitute the exact owning test in the table):

```text
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet TEST_NAME -- --exact --nocapture --test-threads=1
```

Inventory drift, `cargo check` on both owning targets, AGENT-SCAN / TEST-BUDGET / INSPECT,
and hosted Doctrine Scan + Exec are reported separately in the final Board/PR return.
They cannot discharge this required execution refusal. Prior accepted birth/RF/capacity
results remain historical evidence, not passes on the current runtime.

## Native clamp ingress is now GREEN; storage remains unproved

The second gate passes the existing two stock-accounting cases and the previously RED
native clamp probe. The ordinary ingress projects the mineral balance's authored
`Bounded { min: 0.0, max: 24.0 }` into the hydrated game-mode specification exactly.
This discharges the *authoring* STOP in return 5745459860.

The probe does not activate the bounded candidate. It therefore proves neither live
registry/cache rebind, bounded execution, saturation, spend-created headroom, refill,
lower-bound behavior nor conservation. No bounded conserved RF leaf was executed here.
The DA conservation fence is unchanged: every conserved allocation must be accounted
for, or the bounded cell must sit outside that conserved path in the same economy.

## Exact failure and qualified-runtime dependency

The funded-birth source resolves and projects, but ordinary session opening refuses at
`rehearsal_lifecycle_economy_fleet.rs:820`, before any generation or funded birth:

```text
ResidentClearing(LiveHead(UnqualifiedAdapter {
    required: 17165209339348674868,
    observed: 17263054877348392797
}))
required hex: ee3712f2ef186934
observed hex: ef92b0f2866cef5d
```

Read-only capture through the same compiled GPU library reports **Vulkan / NVIDIA
GeForce RTX 4080 Laptop GPU / NVIDIA 616.92**, profiling enabled. The exact compiler,
features, lock, source bundle, workgroups and ABI are preserved below.

An independent offline implementation of the production FNV fingerprint reproduces
`ef92b0f2866cef5d` from this captured record. Changing **only** the diagnostic input
`driver_runtime` from `NVIDIA 616.92` to the historical `NVIDIA 595.79` reproduces
the required pin **`ee3712f2ef186934`** exactly. This is a counterfactual hash comparison,
not execution under the former driver and not admission of the new driver.
Historical driver evidence: `docs/tests/construction_retention_payer_checkpoint/e8_pin_proof.txt`.
The eighteenth pin's clean-checkout proof remains in
`rehearsal_economy_structural_rf_enrollment_0_results.md`.

Both E8 literals remain **`0xee37_12f2_ef18_6934`**. All **32 sealed working-file bytes**
equal the exact release git blobs. Their bundle is **`0xca5fca9690ccccd8`** and the
lock hash is **`0x1bd6ab6097f443d7`**; the active profiling build provenance agrees
with every file. This isolates the driver string as sufficient to explain the refusal.
It does not demonstrate a birth, RF or capacity semantic regression on the new base.

**Exact return dependency:** provide the already qualified reference runtime, or route
lawful qualification of NVIDIA 616.92 through the E8 authority and qualification battery.
This consumer handoff authorizes neither a pin change nor a guard bypass. No driver
installation, rollback, backend substitution, protected edit, or altered production
qualification record was attempted. Resume the ordered accepted gates only after the
runtime dependency is lawfully resolved; no additional rebase is authorized here.

## Remaining floor and fences

Not completed: bounded stock live/cache execution and conservation accounting; separate
insufficient-alloy fleet recovery; separate insufficient-energy-work recovery; refusal
funding/reservation disposition; full recurring 1 energy + 0.2 alloy per corvette;
complete post-birth stocks; full authored/declaration/order equivalence;
identity/replay/fail-stop negatives; post-birth save/restore.
The earlier refinery recovery does not substitute for fleet-input recovery.
No cancellation/removal/reparenting of an enrolled born subtree, fusion/tombstones,
recycled-slot law, second stock authority, or protected runtime/parser/gate change.
Orchestration owns final current-head clearance and DA routing.

## Captured qualification record

The scratch probe calls only `GpuContext::new_blocking()`,
`ResidentClearingQualification::capture`, `fingerprint`, and `semantic_kernel_components`.
It does not call admission or mutate a qualification record. The initial standalone
link lacked `windows.0.52.0.lib` (linker 1181); supplying the installed Cargo dependency's
native library directory linked and ran successfully. That launcher retry is not a
product test failure.

```text
QUALIFICATION_RECORD ResidentClearingQualification {
    backend: "Vulkan",
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    vendor: 4318,
    device: 10144,
    device_class: "DiscreteGpu",
    driver_runtime: "NVIDIA 616.92",
    features: "Features(DEPTH_CLIP_CONTROL | DEPTH32FLOAT_STENCIL8 | TEXTURE_COMPRESSION_BC | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | SHADER_F16 | RG11B10UFLOAT_RENDERABLE | BGRA8UNORM_STORAGE | FLOAT32_FILTERABLE | TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | TEXTURE_BINDING_ARRAY | BUFFER_BINDING_ARRAY | STORAGE_RESOURCE_BINDING_ARRAY | SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING | PARTIALLY_BOUND_BINDING_ARRAY | MULTI_DRAW_INDIRECT | MULTI_DRAW_INDIRECT_COUNT | PUSH_CONSTANTS | ADDRESS_MODE_CLAMP_TO_ZERO | ADDRESS_MODE_CLAMP_TO_BORDER | POLYGON_MODE_LINE | POLYGON_MODE_POINT | CONSERVATIVE_RASTERIZATION | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | SPIRV_SHADER_PASSTHROUGH | MULTIVIEW | TEXTURE_FORMAT_NV12 | RAY_TRACING_ACCELERATION_STRUCTURE | RAY_QUERY | SHADER_F64 | SHADER_I16 | SHADER_PRIMITIVE_INDEX | DUAL_SOURCE_BLENDING | SHADER_INT64 | SUBGROUP | SUBGROUP_VERTEX | SUBGROUP_BARRIER | PIPELINE_CACHE | SHADER_INT64_ATOMIC_MIN_MAX | SHADER_INT64_ATOMIC_ALL_OPS)",
    compiler: "rustc 1.95.0 (59807616e 2026-04-14)\nbinary: rustc\ncommit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860\ncommit-date: 2026-04-14\nhost: x86_64-pc-windows-msvc\nrelease: 1.95.0\nLLVM version: 22.1.2",
    cargo_features: "EML_RESOURCE_PROFILING",
    shader_compiler: "wgpu 22.1.0 / naga 22.1.0",
    cargo_lock_hash: 2005979115394712535,
    semantic_kernel_bundle_hash: 14582596866472201432,
    workgroups: [
        32,
        64,
    ],
    subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority",
    abi_version: 1,
}
QUALIFICATION_FINGERPRINT ef92b0f2866cef5d
```

## Sealed byte comparison

Every component below: working bytes == release `d3b58b7513ee9dd56ad361c02aac460f45df4b77`; active build hash matches.

| Component | FNV-1a64 |
| --- | --- |
| `Cargo.lock` | `1bd6ab6097f443d7` |
| `crates/simthing-driver/src/child_share_eml.rs` | `b3868ac91cb8bb48` |
| `crates/simthing-core/src/eml_nodes.rs` | `2d2a8647755786a9` |
| `crates/simthing-core/src/accumulator_op.rs` | `4b61497a6e3558cf` |
| `crates/simthing-core/src/accumulator_op_builder.rs` | `5f1312f38b07c57c` |
| `crates/simthing-driver/src/arena_allocation_plan.rs` | `700ca68469a41406` |
| `crates/simthing-driver/src/arena_allocation_sync.rs` | `d0bd39c0e23a042e` |
| `crates/simthing-driver/src/arena_hierarchy.rs` | `7b4ec1cce214dd71` |
| `crates/simthing-kernel/src/accumulator_op/mod.rs` | `2c91cde5f81f6c5a` |
| `crates/simthing-kernel/src/accumulator_op/types.rs` | `e7d249349a1fe054` |
| `crates/simthing-kernel/src/accumulator_op/encode.rs` | `904c5b926d763eb0` |
| `crates/simthing-kernel/src/accumulator_op/cpu_oracle.rs` | `9cf7b0902d319352` |
| `crates/simthing-kernel/src/accumulator_op/session.rs` | `dc3a47ba85d60711` |
| `crates/simthing-kernel/src/shaders/accumulator_op.wgsl` | `ca10144847085a88` |
| `crates/simthing-kernel/src/resident_clearing_plan.rs` | `57b8fc88c7bc291d` |
| `crates/simthing-core/src/persistence_deformation.rs` | `4219e35bfbbec05b` |
| `crates/simthing-kernel/src/resident_clearing_apportionment.rs` | `96fb66e869a6d512` |
| `crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl` | `2b17010816f192b4` |
| `crates/simthing-kernel/src/resident_recursive_intake_transform.rs` | `d03b0163a624ed91` |
| `crates/simthing-kernel/src/shaders/resident_recursive_intake_transform.wgsl` | `7f077d7537cfe7a0` |
| `crates/simthing-gpu/src/resident_clearing_plan.rs` | `c1fa0e1ca1a04903` |
| `crates/simthing-driver/src/resident_clearing_runtime.rs` | `df7b6d8b5fea19b2` |
| `crates/simthing-driver/src/session.rs` | `1b1e44c92af20d35` |
| `crates/simthing-driver/src/growth_entitlement.rs` | `8e25b4b92fabb6a4` |
| `crates/simthing-driver/src/spec_session.rs` | `e9fc93598c33fdf0` |
| `crates/simthing-sim/src/boundary.rs` | `f12ccbaa61ee6a7d` |
| `crates/simthing-sim/src/sim_runtime_tree.rs` | `0e1a31d6b0a8db3d` |
| `crates/simthing-spec/src/spec/flow_market.rs` | `77ee8ce43b7bd068` |
| `crates/simthing-spec/src/spec/constrained_clearing.rs` | `ecccf0ba2c40bc2b` |
| `crates/simthing-spec/src/spec/scenario.rs` | `02eb6a42254b1685` |
| `crates/simthing-core/src/owner_channel.rs` | `4bf27ccf5effd5fe` |
| `crates/simthing-clausething/src/hydrate_shipsize_decoder.rs` | `272b087e14a6fec2` |

## Local evidence manifest

Scratch evidence is retained under `C:/Users/mvorm/SimThing/.git/economy-fleet-22-bounded-resume`.
The following raw files bind the exact execution/diagnosis before this evidence-only commit.

| File | SHA-256 |
| --- | --- |
| `handoff.json` | `b21d44ef309a1bb1d1c5bf2b199b08bc6b1966b94362f917b6d7e0be834826a8` |
| `rebase-reflog.txt` | `0cbb0923e18231b20a1ba3712df5756ed6b1928220a6892164a23cb471d0085c` |
| `sync-conjunction.txt` | `0dcff29e0b68c8fd85a75d7f68320f302a5327e06f0c0cc1306f0263a8dfca05` |
| `sync-stock.txt` | `725449acc793bfc1e646f297771c3d4bbc0e62d3e995d1aac07a411731523787` |
| `sync-birth.txt` | `07c4651b804a745598c5b9ef9f4f78a64f3800455d5a83c8434cd58b415afd16` |
| `sync-validation.json` | `619cac8968acea6f1bbe9fd1710f8efe3f484a49cf739c53db5ec2d9dae0ba23` |
| `qualification-record.txt` | `4a40ff6a26ebeac3dfb4a66d29d224ebf5345292c1dcd330eec9058119674069` |
| `qualification-bytes.json` | `82e0e541ad8f9713157374d302b63430f795456ec5e147f9ae62ad7f63b10462` |
| `driver-diagnosis.json` | `a7dbca3b7d11c2c849da54ef4637d92ff144db5bdfa6f906e75c148b5d253b76` |
| `diagnose_driver.py` | `9f18de94b853a6fcaa44c5182b7ecb838d797a7eb5cfeef5ec435d8e9ae8cac9` |
| `qualification_probe.rs` | `c069eaef7ffb770ea97dff1f4582a31387b7c0d9ea55ef9e9855a40984b421d7` |

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: 389efd70b5e1ef12af9cef88232f9871f78ee3b3f22f3a8ecdfacb7fdef04b77
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b
