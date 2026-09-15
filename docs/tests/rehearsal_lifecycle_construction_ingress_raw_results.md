# Construction Leaf A — raw ingress evidence

Status: PROBATION / ingress-proof-present / BLOCKED. Model 1 undecided.

Captured 2026-09-15 at base ffa5b944928c3aa3fc66832700330dc43b2a0f78.
Command and interpretation: rehearsal_lifecycle_construction_ingress_results.md.

## Actual focused-test output

```text
running 1 test
test construction_leaf_a_requires_qualified_ordinary_session ... adapter: AdapterInfo { name: "NVIDIA GeForce RTX 4080 Laptop GPU", vendor: 4318, device: 10144, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "595.79", backend: Vulkan }
qualification: ResidentClearingQualification {
    backend: "Vulkan",
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    vendor: 4318,
    device: 10144,
    device_class: "DiscreteGpu",
    driver_runtime: "NVIDIA 595.79",
    features: "Features(DEPTH_CLIP_CONTROL | DEPTH32FLOAT_STENCIL8 | TEXTURE_COMPRESSION_BC | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | SHADER_F16 | RG11B10UFLOAT_RENDERABLE | BGRA8UNORM_STORAGE | FLOAT32_FILTERABLE | TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | TEXTURE_BINDING_ARRAY | BUFFER_BINDING_ARRAY | STORAGE_RESOURCE_BINDING_ARRAY | SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING | PARTIALLY_BOUND_BINDING_ARRAY | MULTI_DRAW_INDIRECT | MULTI_DRAW_INDIRECT_COUNT | PUSH_CONSTANTS | ADDRESS_MODE_CLAMP_TO_ZERO | ADDRESS_MODE_CLAMP_TO_BORDER | POLYGON_MODE_LINE | POLYGON_MODE_POINT | CONSERVATIVE_RASTERIZATION | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | SPIRV_SHADER_PASSTHROUGH | MULTIVIEW | TEXTURE_FORMAT_NV12 | RAY_TRACING_ACCELERATION_STRUCTURE | RAY_QUERY | SHADER_F64 | SHADER_I16 | SHADER_PRIMITIVE_INDEX | DUAL_SOURCE_BLENDING | SHADER_INT64 | SUBGROUP | SUBGROUP_VERTEX | SUBGROUP_BARRIER | PIPELINE_CACHE | SHADER_INT64_ATOMIC_MIN_MAX | SHADER_INT64_ATOMIC_ALL_OPS)",
    compiler: "rustc 1.95.0 (59807616e 2026-04-14)\nbinary: rustc\ncommit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860\ncommit-date: 2026-04-14\nhost: x86_64-pc-windows-msvc\nrelease: 1.95.0\nLLVM version: 22.1.2",
    cargo_features: "EML_RESOURCE_PROFILING",
    shader_compiler: "wgpu 22.1.0 / naga 22.1.0",
    cargo_lock_hash: 2005979115394712535,
    semantic_kernel_bundle_hash: 14318815736946470529,
    workgroups: [
        32,
        64,
    ],
    subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority",
    abi_version: 1,
}
required_fingerprint: 6f287da986750d29
observed_fingerprint: 6fe1d809c05ee0f4
bundle: c6b6a70c64f7ae81

thread 'construction_leaf_a_requires_qualified_ordinary_session' (31716) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_ingress.rs:47:23:
Leaf A stopped before economics: ResidentClearing(LiveHead(UnqualifiedAdapter { required: 8009790104337190185, observed: 8061962344363647220 }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    construction_leaf_a_requires_qualified_ordinary_session

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.62s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_ingress`
```

## Read-only byte/tuple forensics

No historical source bytes were installed or admitted. Recomputed the existing
length-prefixed FNV-1a bundle and tuple algorithm over git blobs and the captured
adapter/compiler/features tuple; varied only the semantic bundle hash.

```json
{
  "method": "Read-only Git blob and captured-tuple FNV recomputation; never installed or admitted.",
  "base": "ffa5b944928c3aa3fc66832700330dc43b2a0f78",
  "last_qualification_commit": "dae7ab1b",
  "canonical_worktree_components": 32,
  "old_bundle": "dfcb52ff6a412e2e",
  "current_bundle": "c6b6a70c64f7ae81",
  "current_fingerprint": "6fe1d809c05ee0f4",
  "same_captured_tuple_with_old_bundle": "6f287da986750d29",
  "changed_components": [
    {
      "path": "crates/simthing-driver/src/arena_allocation_plan.rs",
      "before_bytes": 24996,
      "after_bytes": 23211,
      "before_sha256": "72fa9ad2be93ac9f86d208d2604f31f1321185435a223595fba25be6cd4a2019",
      "after_sha256": "edd90cf904e8c7505fc78c3d339be0ee1e97a7405c8e94a58ee48fa7cae25122"
    },
    {
      "path": "crates/simthing-kernel/src/accumulator_op/session.rs",
      "before_bytes": 135280,
      "after_bytes": 125998,
      "before_sha256": "65a27e1105be7f114e13ae55f95b2a4e9afba8abd868e4f3ae29284dc2ed9811",
      "after_sha256": "50a1ae405c8b02da09c710918c63eee3634d2567cbf1635d8be5ce725e4ad7f6"
    },
    {
      "path": "crates/simthing-driver/src/session.rs",
      "before_bytes": 112631,
      "after_bytes": 109690,
      "before_sha256": "9f47b2b0507fb4b9c65422e844ae863215e7ab31620f40b8dc3947be91b2ebcc",
      "after_sha256": "e31d5a2dd6f2ba81789f8648afb5673aae8d767239ed8c0542848020c5df832d"
    }
  ]
}
```

## Pre-edit anchor acknowledgements

```text
ANCHOR-ACK: rehearsal-0088-acceptances 3d95d334fafe6daf9beab094cf3c6ba6e012d2841c694f52ae843286d754d28a
ANCHOR-ACK: rehearsal-0088-binding-laws 25af224d1debc66ac937bc0f0a74c0fb210c62c926908ffaf993208bb86c046e
ANCHOR-ACK: rehearsal-0088-charter 056bfe9205e7d20776f9a1ae4687e3e4a9e21a35f77c5b7a2137b589f91f1f8a
ANCHOR-ACK: rehearsal-0088-combat-contract d338735485a6bd84ca4f2f1b32c2c7b738df7f370167789f0275497c14c94209
ANCHOR-ACK: rehearsal-0088-construction-contract bdd51c7c4ae275ce76e645847539dc86701068cfe6cf23f23b59a55478413745
ANCHOR-ACK: rehearsal-0088-exposure-matrix 08cd697d3851528791945c37c1120ac36a7c6c367f33b2b57a3effbcf2106fe8
ANCHOR-ACK: rehearsal-0088-growth 27dc695086319f2b51b818c2f824cd2d6213539d5b230a0e679636667f1fe88f
ANCHOR-ACK: rehearsal-0088-ladder 6b7ca0ea6e0109ce6cad5a0ad62c67aaa8713d6595a765c0371fd34ec66d6b09
ANCHOR-ACK: rehearsal-0088-legacy-census 9dd38aedb7207acbf0e9747f1fb3ca7c14b101449e256ae3f8285289535d718f
ANCHOR-ACK: rehearsal-0088-measurement 8d639c581ade4a807094dc70282da22772f1624a2db635b6b768b7c2118454cc
ANCHOR-ACK: rehearsal-0088-mission c9a7689166ed203488ce54c2ac95d2f8dfa71523e70c3d9d081d068fab98e351
ANCHOR-ACK: rehearsal-0088-movement-contract 6e48ba6b9d6c4837087118be6f0aae7eea9463f8ddad87b31e0254910af6861e
ANCHOR-ACK: rehearsal-0088-pillars f08b68dddbc6ea26e8311652b2a5ff8a87dc72078609839c5d8c50142781becb
ANCHOR-ACK: rehearsal-0088-routing ac4e7d2c5a61f1079344a9aa8eb0b348664ddf06bc70dc7dfaf5e06812eb6470
```
