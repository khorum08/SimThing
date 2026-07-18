# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status

**OWNER OVL PASS / orchestration-review-complete / DA-relay-ready / RF-5 SPLIT APPROVED.** Owner ruling [comment `5012654906`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) closes the replacement [OVL], accepts the crash-fenced executable and existing screenshots, and lifts emergency hold `5012226484` for runtime evidence. Codex did not self-close [OVL] and does not self-relay to DA.

## Identity

| Field | Value |
|---|---|
| Rung | `STUDIO-FIELD-SESSION-ELEVATE-0` (RF-4 / 12.9) |
| Branch | `coder/studio-field-session-elevate-rf4-0` |
| Base/master stamp | `c0e2202694bc9c2a329e5a6d4620a078c7ba71ea` |
| RF-3 graduation | merged PR #1412 @ `d42b9109c032f96a66784a9274b5812107a32e45` |
| HD-RECEIPT | `a8e70c897f36` |
| Handoff head | `7cd17c7e9a9666b2e20ed54b7f4627ce6e163c13` |
| Salvage source | PR #1405 @ `34bb730fb24b550eeac520e83fbf5e2408a0f7c4` (read-only provenance; salvage complete; closed unmerged) |
| Final Owner [OVL] verdict | **PASS / CLOSED**, [comment `5012654906`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) |
| Superseded first-executable Owner verdict | [comment `5012128000`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) |
| RF-5 split approval | **APPROVED**, [comment `5012077482`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012077482) |
| Evidence-amendment ruling | [comment `5012132787`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012132787) |
| Emergency DA hold | [comment `5012226484`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012226484) |
| GPU remedial handoff | [comment `5012228350`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012228350) |
| Final OVL correction / closeout handoff | [comment `5012654906`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) |
| Superseded DX12-only remedial SHA | `b99fa6326b729cd6fb8a1e9adba364795015dccd` |
| Superseded backend-all correction SHA | `d6688bb704927d9fd909b0082d7265ffcb57f147` |
| Tested / implementation / executable-source SHA | `4334acda94284622c4502bf67ff09c94f766b85a` |
| Orchestrator-reviewed pre-closeout evidence head | `059881f64cb18265d23ebbd664579c11ce01ea06` |
| Doctrine scan / PR base | `c0e2202694bc9c2a329e5a6d4620a078c7ba71ea` |

## Implemented path

- Production Studio clause ingest composes the admitted canonical authority tree: real `GameSession` → real `Owner` → three real authored children.
- The field-bearing bridge retains `ResourceFlowSpec`, opens through ordinary `SimSession::open_from_spec`, selects `RecursiveArenaResourceFlow`, and advances only through ordinary `step_once`.
- The RF property is pre-registered at the existing field-bearing column-zero seam before generic spec installation; there is no kernel, WGSL, grammar, planner, scenario API, execution-default, or driver API change.
- Telemetry resolves actual Arena-participant cells. It exposes path, tick/play state, recursive profile/activity, a named child, its real Owner sibling aggregate, and loaded/live aggregate values. Emission loci remain a separate diagnostic table.
- Structural-shell fallback, ClauseScript ingest, pause, authored emission, and threshold-only decision behavior remain covered by the focused suite.
- Windows Studio renderer creation allows Bevy-supported backends except DX12, with high-performance adapter selection and Bevy's fallback request disabled. Post-init validation remains fail-closed unless the actual `RenderAdapterInfo` is exactly `NVIDIA GeForce RTX 4080 Laptop GPU`, NVIDIA vendor `0x10de`, and `DiscreteGpu`, and independently rejects DX12 after initialization. The actual safe backend remains truthful telemetry.
- Adapter telemetry state now exists before plugin `finish`; the actual adapter populates name, vendor/device, backend, device type, and an explicit policy row. Missing identity or any mismatch aborts startup with required and observed details.

## Load-bearing recursive proof

Command:

```text
cargo test -p simthing-workshop --test tp_field_session_elevate_0 -- --nocapture
```

Observed actual GPU output:

```text
RF4_LIVE loaded_owner_aggregate=0 live_owner_aggregate=15 disabled_aggregate=10 named_marginal=5 budget=27.1 sum_disbursed=27.100002 arithmetic_residual=-0.0000019073486 measured_balance_delta=-0.0000019073486 bound=0.000077533725
RF4_RUNTIME_NEGATIVE governed_balance=disconnected actual_gpu_balance_delta=0 violation=ResidualNotIntegrated { arithmetic_residual: -1.9073486e-6, reported_balance_residual: Some(0.0) }
```

The named child contributes exactly `5`; disabling only that child changes the real Owner aggregate from `15` to `10` while its two real siblings remain. The Owner is therefore neither a one-to-one copy nor a synthetic parent. The independently recomputed f32 residual is non-zero, inside the RF-1 bound, and equals the measured governed Balance delta. Removing `governed_by` from the runtime path leaves the actual GPU Balance delta at zero and unchanged RF-1 rejects it with `ResidualNotIntegrated`.

## Approved RF-5 split for need / `weight_profile`

Canonical ClauseScript hydrates three `HydratedFieldEconomyWeightProfile` EML gadget stacks, but those stacks remain hydrate-pack data. `GameModeSpec`/`open_from_spec` has no admitted install/consumer contract that binds them to recursive Arena participant weights or a live need cell. Existing overlays target hosted authority nodes, while RF execution reads materialized Arena-participant wrapper cells; copying or patching those cells in Studio would violate the no-feeder/no-direct-mutation fence.

RF-4 therefore stopped at the handoff's explicit contract boundary. Orchestration approved this bounded RF-5 split in comment `5012077482`:

1. define one generic spec-owned binding from an existing hydrated EML gadget stack to an existing Arena participant role/cell;
2. install it through the ordinary GameMode/session-open compiler and existing EML/Accumulator machinery;
3. add live need/threshold and below-threshold controls without new ClauseScript syntax, kernel/WGSL, Studio arithmetic, or synthetic hierarchy;
4. expose the admitted readout in the existing Studio telemetry projection.

The UI honestly states `not admitted in RF-4; bounded RF-5 split required`. Screenshot C is **not required for RF-4** because orchestration approved the split; no need execution is claimed or counterfeited.

## Salvage disposition

The accepted/rejected file-and-hunk audit is recorded in [`studio_field_session_elevate_0_salvage_manifest.md`](studio_field_session_elevate_0_salvage_manifest.md). No merge, rebase, or wholesale cherry-pick from PR #1405 was performed.

## Superseded Owner [OVL] evidence

Owner supplied the two artifacts below and ruled [OVL] PASS in [comment `5012128000`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000). The emergency hold in comment `5012226484` supersedes that ruling and both artifacts because their Performance Telemetry identity rows were `unavailable`. They remain historical provenance only and do not satisfy the replacement OVL.

| Owner artifact | Durable GitHub binding | Observed proof |
|---|---|---|
| `RF4_OVL_P_exe_provenance.png` | [Owner verdict and exact identity](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) | Source SHA, executable SHA-256, full path, `86773760`-byte size, and `2026-07-18T16:34:03.1221258Z` build time match the frozen executable. |
| `RF4_OVL_AB_recursive_transition.png` | [Owner verdict and visual observations](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) | Canonical `terran_pirate_galaxy`, tick `9` paused, `field-bearing`, ordinary `open_from_spec + step_once`, `RecursiveArenaResourceFlow / active`, arena `studio_recursive_owner_flow`, child `Infrastructure SIM-003042`, real `Owner terran / 3 siblings`, aggregate loaded/live `0.000000 / 15.000000`. |

The combined AB image is sufficient because the UI retains the frozen loaded baseline and live post-step value in one readout. Emission rows are diagnostic only and do not carry the RF proof. The headless bite remains enabled Owner aggregate `15`, child-disabled aggregate `10`, exact marginal `5`, and governed-Balance disconnect → `ResidualNotIntegrated`.

Owner—not Codex—supplied and ruled the superseded screenshots. Screenshot C remains deferred to the approved bounded RF-5 split and is not an RF-4 requirement.

## GPU adapter remedial proof

The initialization-order defect was load-bearing: `StudioGpuIdentityInitPlugin::finish` attempted to copy `RenderAdapterInfo`, but `StudioPerformanceTelemetryState` was not created until a later Startup system, so the hook returned without populating telemetry. The remedial creates the resource in plugin `build`, obtains the actual main-world or render-subapp adapter during `finish`, populates telemetry, and then validates exact identity. It never hardcodes observed telemetry values.

The emergency-hand-off tests at `b99fa6326b729cd6fb8a1e9adba364795015dccd` initially proved:

1. exact RTX 4080 Laptop / NVIDIA / discrete / DX12 acceptance;
2. Intel integrated rejection;
3. Microsoft Basic/software rejection;
4. exact RTX model on a non-DX12 backend rejection (superseded by the Owner correction below);
5. wrong NVIDIA discrete model rejection;
6. supplied adapter snapshot populates all identity and policy rows;
7. plugin build creates telemetry before renderer finish can observe `RenderAdapterInfo`.

Owner-local launch of the exact fresh executable below observed these live rows:

```text
GPU adapter: NVIDIA GeForce RTX 4080 Laptop GPU
GPU vendor/device: 0x10de/0x27a0
GPU backend: Dx12
GPU device type: DiscreteGpu
GPU adapter policy: satisfied: exact NVIDIA GeForce RTX 4080 Laptop GPU / NVIDIA / DiscreteGpu / Dx12
```

Before the remedial, the Owner observed Vulkan validation/layer/version warnings, Vulkan-hidden adapters, and optional D3D12 debug-interface warning `0x887A002D`. The DX12-only remedial removed those startup warnings, but the Owner then reproduced a load-time DX12 descriptor allocator failure while creating a `StandardMaterial` bind group. That crash made the single-backend constraint unusable and is superseded by the correction below.

## Superseded backend-all correction (pre-orchestration)

Before a new orchestration handoff, the Owner removed the requirement that Studio launch only with DX12 and directed an immediate correction. Implementation `d6688bb704927d9fd909b0082d7265ffcb57f147` now requests `Backends::all()`, retains `HighPerformance`, validation, and nonfallback selection, and validates only the exact RTX/NVIDIA/discrete adapter identity. The actual backend remains populated from `RenderAdapterInfo` and is included in the policy row.

The biting policy tests now prove that the exact required adapter is accepted on both DX12 and Vulkan; Intel integrated, Microsoft software, and the wrong NVIDIA model still fail closed. The renderer-settings test proves a future change cannot silently restore a single-backend lock.

Owner-local reproduction with the fresh executable below selected Vulkan on the same adapter:

```text
GPU adapter: NVIDIA GeForce RTX 4080 Laptop GPU
GPU vendor/device: 0x10de/0x27a0
GPU backend: Vulkan
GPU device type: DiscreteGpu
GPU adapter policy: satisfied: exact NVIDIA GeForce RTX 4080 Laptop GPU / NVIDIA / DiscreteGpu / backend: Vulkan
```

Studio initially loaded and rendered `scenarios/terran_pirate_galaxy.clause` on Vulkan. During Owner replacement-OVL capture, however, the same executable selected DX12 because `Backends::all()` permits rather than prefers Vulkan. The canonical load then reproduced `wgpu_hal::dx12::descriptor: Unable to allocate descriptors: RangeAllocationError` followed by the fatal `StandardMaterial` bind-group validation panic. The implementation, executable, and provenance capture are superseded.

## Owner-directed DX12 load-crash fence (pre-orchestration)

Implementation `4334acda94284622c4502bf67ff09c94f766b85a` removes `Backends::DX12` from the otherwise portable Bevy backend set. It retains `HighPerformance`, validation, nonfallback selection, and exact RTX/NVIDIA/discrete identity enforcement. Post-init validation also rejects an observed `Dx12` backend, so an environment or renderer-selection regression fails at startup rather than reaching scene material allocation.

Focused tests prove that Vulkan and GL remain available, DX12 is absent from renderer settings, exact RTX identity on Vulkan/GL is accepted, and exact RTX identity on DX12 is rejected alongside Intel, software, and wrong-model adapters.

Owner-local reproduction with the fresh executable below selected Vulkan and loaded the canonical 1,500-system / 2,714-link scene. All systems, links, and 1,500 occupied cells rendered; STEAD, RF, and GPU-index readiness remained valid. Studio remained live for an additional 60-second post-load stability interval with the GPU policy satisfied. The DX12 descriptor allocator and `StandardMaterial` panic cannot be reached through this renderer set.

## Superseded DX12-only debug executable identity

This executable successfully proved truthful adapter telemetry but crashed on the Owner's 1,500-system scenario load and must not be used for replacement OVL:

| Field | Value |
|---|---|
| `ovl_exe_source_sha` | `b99fa6326b729cd6fb8a1e9adba364795015dccd` |
| `ovl_exe_sha256` | `0b56f5cd33a7c070e0f31427b9f28d4725d9167e4ff8174f0bec7df88ae07e7c` |
| Full path | `C:\Users\mvorm\SimThing\target\rf4-gpu-remedial-b99fa632\debug\simthing-studio.exe` |
| Byte size / UTC build time | `86788608` bytes / `2026-07-18T17:49:24Z` |
| Build command | `CARGO_TARGET_DIR=target/rf4-gpu-remedial-b99fa632 cargo build -p simthing-mapeditor --bin simthing-studio` |
| Working tree clean | YES for tracked files; pre-existing untracked `scenarios/terran_pirate_galaxy.from-clause.simthing-scenario.json` preserved unchanged |

## Superseded backend-all debug executable identity

| Field | Value |
|---|---|
| `ovl_exe_source_sha` | `d6688bb704927d9fd909b0082d7265ffcb57f147` |
| `ovl_exe_sha256` | `978c7c641754c998c89c89eaf8687206ec2464e608a93ebd1298f3d01c91de59` |
| Full path | `C:\Users\mvorm\SimThing\target\rf4-gpu-backend-d6688bb7\debug\simthing-studio.exe` |
| Byte size / local build time | `86788608` bytes / `2026-07-18T13:23:47.671038500-05:00` |
| Build command | `CARGO_TARGET_DIR=target/rf4-gpu-backend-d6688bb7 cargo build -p simthing-mapeditor --bin simthing-studio` |
| Live load proof | PASS on Vulkan; canonical 1,500-system scenario rendered and process remained live |

This executable later selected DX12 during Owner capture and reproduced the descriptor crash. Its Owner provenance capture is historical only and must not be submitted as replacement OVL.

## Current crash-fenced OVL debug executable identity

| Field | Value |
|---|---|
| `ovl_exe_source_sha` | `4334acda94284622c4502bf67ff09c94f766b85a` |
| `ovl_exe_sha256` | `4f2fa46b3152974e7957117c99ef50190614a319ed3ba8fecda4a89b688ff4c9` |
| Full path | `C:\Users\mvorm\SimThing\target\rf4-gpu-safe-4334acda\debug\simthing-studio.exe` |
| Byte size / local build time | `86788608` bytes / `2026-07-18T13:54:46.527978100-05:00` |
| Build command | `CARGO_TARGET_DIR=target/rf4-gpu-safe-4334acda cargo build -p simthing-mapeditor --bin simthing-studio` |
| Live load proof | PASS on Vulkan; canonical 1,500-system scene rendered and remained live for an additional 60-second stability interval |

## Final Owner [OVL] evidence — PASS / CLOSED

The Owner exercised the final crash-fenced executable and supplied the existing evidence artifacts below. Orchestration bound those artifacts and issued the authoritative **OWNER [OVL]: PASS / CLOSED** ruling in [comment `5012654906`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906). The ruling explicitly supersedes the duplicate recapture request in comment `5012637592`; nothing is to be rebuilt or recaptured.

| Owner artifact | Durable ruling binding | Accepted observation |
|---|---|---|
| `RF4_OVL_P_rebuilt_exe_provenance.png` | [Final Owner OVL ruling](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) | Source `4334acda94284622c4502bf67ff09c94f766b85a`; executable SHA-256 `4f2fa46b3152974e7957117c99ef50190614a319ed3ba8fecda4a89b688ff4c9`; path, size, and build identity match the frozen executable above. |
| `RF4_OVL_GPU_required_adapter.png` | [Final Owner OVL ruling](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) | Actual `NVIDIA GeForce RTX 4080 Laptop GPU`, vendor `0x10de`, `DiscreteGpu`, Vulkan/non-DX12, policy satisfied, with truthful initialized telemetry. |
| `RF4_OVL_AB_recursive_transition_rebuilt.png` | [Final Owner OVL ruling](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012654906) | Canonical 1,500-system / 2,714-link scene; field-bearing `open_from_spec + step_once`; recursive RF active; real `Owner terran / 3 siblings`; loaded/live aggregate `0.000000 / 15.000000`; scene remained live. |

The headless bite remains enabled Owner aggregate `15`, named-child-disabled aggregate `10`, exact marginal `5`, and governed-Balance disconnect → `ResidualNotIntegrated`. Screenshot C remains outside RF-4 under the approved bounded RF-5 split. Owner—not Codex—supplied and ruled the visual evidence; no additional Owner action is required.

## Verification ledger

| Check | Result |
|---|---|
| Focused RF-4 workshop suite | PASS, 4/4; output above |
| Focused Studio field-bearing suite | PASS, 8/8 |
| Studio bridge regression suite | PASS, 8/8 |
| Focused GPU policy / telemetry | PASS, 8 selected tests; 0 failed (4 new biting tests plus 4 existing GPU-filter matches) |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| Studio debug build | PASS @ crash-fence SHA `4334acda94284622c4502bf67ff09c94f766b85a`; current executable identity above |
| Owner-local exact-adapter launch / [OVL] | **PASS / CLOSED**; actual telemetry was RTX 4080 Laptop / `0x10de:0x27a0` / `DiscreteGpu` / `Vulkan` / policy satisfied; ruling `5012654906` |
| Owner-local canonical scenario load / [OVL] | **PASS / CLOSED**; 1,500 systems and 2,714 links rendered on Vulkan; process remained live; DX12 excluded; ruling `5012654906` |
| Doctrine PR scan | PASS at reviewed head `059881f6`; `WORKSHOP-HOMING-DETECTION PASS 0`, `TEST-BUDGET PASS 0`, inspect `0` |
| Agent scan | PASS at reviewed head `059881f6`; `AGENT-SCAN-VERDICT: PASS delta_inspect=0` |
| Orientation | `gen_orientation --check: PASS` |
| Inventory | `TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS` |
| Doc budget | `DOC-BUDGET-VERDICT: PASS` |

## Graduation routing

Posture is **OWNER OVL PASS / orchestration-review-complete / DA-relay-ready / RF-5 SPLIT APPROVED**. This ledger records the Owner/orchestration closure but does not claim graduation, merge authorization, RF-5 need transport, or DA approval. PR #1413 remains draft/open; orchestration alone performs the final exact-head review and DA relay.
