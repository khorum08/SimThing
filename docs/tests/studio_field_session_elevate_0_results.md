# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status

**PROBATION / OWNER [OVL] PASS / RF-5 SPLIT APPROVED / DA-review-pending.** Owner supplied and ruled the screenshots; Codex did not self-close [OVL].

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
| Owner [OVL] verdict | **PASS**, [comment `5012128000`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) |
| RF-5 split approval | **APPROVED**, [comment `5012077482`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012077482) |
| Evidence-amendment ruling | [comment `5012132787`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012132787) |

## Implemented path

- Production Studio clause ingest composes the admitted canonical authority tree: real `GameSession` → real `Owner` → three real authored children.
- The field-bearing bridge retains `ResourceFlowSpec`, opens through ordinary `SimSession::open_from_spec`, selects `RecursiveArenaResourceFlow`, and advances only through ordinary `step_once`.
- The RF property is pre-registered at the existing field-bearing column-zero seam before generic spec installation; there is no kernel, WGSL, grammar, planner, scenario API, execution-default, or driver API change.
- Telemetry resolves actual Arena-participant cells. It exposes path, tick/play state, recursive profile/activity, a named child, its real Owner sibling aggregate, and loaded/live aggregate values. Emission loci remain a separate diagnostic table.
- Structural-shell fallback, ClauseScript ingest, pause, authored emission, and threshold-only decision behavior remain covered by the focused suite.

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

## Owner [OVL] evidence

Owner supplied the two artifacts below and ruled [OVL] PASS in [comment `5012128000`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000). The GitHub connector could not upload the binary images, so that comment records their filenames and binds the directly reviewed Owner submission; orchestration independently accepted them in [comment `5012132787`](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012132787).

| Owner artifact | Durable GitHub binding | Observed proof |
|---|---|---|
| `RF4_OVL_P_exe_provenance.png` | [Owner verdict and exact identity](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) | Source SHA, executable SHA-256, full path, `86773760`-byte size, and `2026-07-18T16:34:03.1221258Z` build time match the frozen executable. |
| `RF4_OVL_AB_recursive_transition.png` | [Owner verdict and visual observations](https://github.com/khorum08/SimThing/pull/1413#issuecomment-5012128000) | Canonical `terran_pirate_galaxy`, tick `9` paused, `field-bearing`, ordinary `open_from_spec + step_once`, `RecursiveArenaResourceFlow / active`, arena `studio_recursive_owner_flow`, child `Infrastructure SIM-003042`, real `Owner terran / 3 siblings`, aggregate loaded/live `0.000000 / 15.000000`. |

The combined AB image is sufficient because the UI retains the frozen loaded baseline and live post-step value in one readout. Emission rows are diagnostic only and do not carry the RF proof. The headless bite remains enabled Owner aggregate `15`, child-disabled aggregate `10`, exact marginal `5`, and governed-Balance disconnect → `ResidualNotIntegrated`.

Owner—not Codex—supplied and ruled the screenshots. Screenshot C is deferred to the approved bounded RF-5 split and is not an RF-4 requirement.

## Windows debug executable identity

The executable was built from a clean detached worktree at the frozen implementation/test SHA. The executable is never committed:

| Field | Value |
|---|---|
| `ovl_exe_source_sha` | `7df4319d6f24b0ae68c85817ca93238f4cb1c4da` |
| `ovl_exe_sha256` | `da39bd3c4da799698e8e72d01ff95b37e2cf112b96e31ffcbb967778388112a5` |
| Full path | `C:\Users\mvorm\SimThing\target\rf4-ovl-clean\target\debug\simthing-studio.exe` |
| Byte size / UTC build time | `86773760` bytes / `2026-07-18T16:34:03.1221258Z` |
| Build command | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Working tree clean | YES; detached worktree was clean immediately before and after the build |

## Verification ledger

| Check | Result |
|---|---|
| Focused RF-4 workshop suite | PASS, 4/4; output above |
| Focused Studio field-bearing suite | PASS, 8/8 |
| Studio bridge regression suite | PASS, 8/8 |
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-mapeditor` | PASS |
| Studio debug build | PASS from clean detached worktree @ `7df4319d6f24b0ae68c85817ca93238f4cb1c4da`; executable identity above |
| Doctrine PR scan | PASS; `WORKSHOP-HOMING-DETECTION PASS 0`, `TEST-BUDGET PASS 0`, inspect `0` |
| Agent scan | `AGENT-SCAN-VERDICT: PASS delta_inspect=0` |
| Orientation | `gen_orientation --check: PASS` |
| Inventory | `TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS` |
| Doc budget | `DOC-BUDGET-VERDICT: PASS` |

## Graduation routing

Recommended posture is **PROBATION / OWNER [OVL] PASS / RF-5 SPLIT APPROVED / DA-review-pending**. This ledger does not claim graduation, merge authorization, RF-5 need transport, or DA approval.
