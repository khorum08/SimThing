# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status

**PROBATION — recursive RF implementation and headless GPU proof PASS; bounded RF-5 split approval required for need/`weight_profile`; Owner [OVL] OPEN.** Codex does not close [OVL].

## Identity

| Field | Value |
|---|---|
| Rung | `STUDIO-FIELD-SESSION-ELEVATE-0` (RF-4 / 12.9) |
| Branch | `coder/studio-field-session-elevate-rf4-0` |
| Base/master stamp | `c0e2202694bc9c2a329e5a6d4620a078c7ba71ea` |
| RF-3 graduation | merged PR #1412 @ `d42b9109c032f96a66784a9274b5812107a32e45` |
| HD-RECEIPT | `a8e70c897f36` |
| Handoff head | `7cd17c7e9a9666b2e20ed54b7f4627ce6e163c13` |
| Salvage source | PR #1405 @ `34bb730fb24b550eeac520e83fbf5e2408a0f7c4` (read-only provenance; remains open) |

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
RF4_LIVE owner_aggregate=15 disabled_aggregate=10 named_marginal=5 budget=27.1 sum_disbursed=27.100002 arithmetic_residual=-0.0000019073486 measured_balance_delta=-0.0000019073486 bound=0.000077533725
RF4_RUNTIME_NEGATIVE governed_balance=disconnected actual_gpu_balance_delta=0 violation=ResidualNotIntegrated { arithmetic_residual: -1.9073486e-6, reported_balance_residual: Some(0.0) }
```

The named child contributes exactly `5`; disabling only that child changes the real Owner aggregate from `15` to `10` while its two real siblings remain. The Owner is therefore neither a one-to-one copy nor a synthetic parent. The independently recomputed f32 residual is non-zero, inside the RF-1 bound, and equals the measured governed Balance delta. Removing `governed_by` from the runtime path leaves the actual GPU Balance delta at zero and unchanged RF-1 rejects it with `ResidualNotIntegrated`.

## Need / `weight_profile` stop and bounded RF-5 proposal

Canonical ClauseScript hydrates three `HydratedFieldEconomyWeightProfile` EML gadget stacks, but those stacks remain hydrate-pack data. `GameModeSpec`/`open_from_spec` has no admitted install/consumer contract that binds them to recursive Arena participant weights or a live need cell. Existing overlays target hosted authority nodes, while RF execution reads materialized Arena-participant wrapper cells; copying or patching those cells in Studio would violate the no-feeder/no-direct-mutation fence.

RF-4 therefore stops at the handoff's explicit contract boundary. Proposed bounded RF-5:

1. define one generic spec-owned binding from an existing hydrated EML gadget stack to an existing Arena participant role/cell;
2. install it through the ordinary GameMode/session-open compiler and existing EML/Accumulator machinery;
3. add live need/threshold and below-threshold controls without new ClauseScript syntax, kernel/WGSL, Studio arithmetic, or synthetic hierarchy;
4. expose the admitted readout in the existing Studio telemetry projection.

Until orchestration approves that split, the UI states `not admitted in RF-4; bounded RF-5 split required`; no need screenshot is claimed.

## Salvage disposition

The accepted/rejected file-and-hunk audit is recorded in [`studio_field_session_elevate_0_salvage_manifest.md`](studio_field_session_elevate_0_salvage_manifest.md). No merge, rebase, or wholesale cherry-pick from PR #1405 was performed.

## Owner [OVL] capture runbook

Owner alone posts screenshots to PR #1413 and rules [OVL]. Required filenames:

- `RF4_OVL_A_loaded_baseline.png`
- `RF4_OVL_B_recursive_accretion.png`
- `RF4_OVL_C_need_threshold.png` only after an admitted need seam lands; otherwise record an approved RF-5 split
- `RF4_OVL_P_exe_provenance.png`

Capture steps:

1. Verify the reported executable SHA-256 matches the file being launched.
2. Launch the debug executable and use **Open ClauseScript Scenario…** to select `scenarios\terran_pirate_galaxy.clause`.
3. Keep the session paused after load. Arrange the window so scenario identity, tick, session path, RF activity, named child, and named ancestor aggregate are all visible.
4. Capture **A — loaded baseline** at tick 0 or the earliest stable paused tick.
5. Press Play at 1×. Let ordinary `SimSession::step_once` advance until the named ancestor aggregate visibly changes. Pause without changing scenario, profile, child enrollment, or layout.
6. Capture **B — recursive accretion** showing the later tick and the same child/ancestor rows. The ancestor value must differ from A in the direction predicted by the accepted headless proof.
7. When an admitted need seam is present, capture **C — need/threshold** with the authored need/`weight_profile` live state, threshold crossing, and below-threshold/no-fire control. If orchestration approved RF-5, state that explicitly and do not counterfeit this screenshot.
8. Capture **P — provenance** showing `git rev-parse HEAD`, executable SHA-256, file size, and build time for the binary used.

Owner PASS requires the same canonical scenario and executable, a later tick, field-bearing path, recursive RF active, the same named child and real Owner aggregate, and an aggregate change between A and B. Structural-shell, RF inactive, a flat/zero or one-to-one aggregate, emission-only proof, mismatched executable identity, or missing need evidence without an approved split is FAIL/remand.

## Windows debug executable identity

The executable is built only after the implementation/test SHA is frozen and all remands are clean. The executable is never committed. Final values are filled from that frozen SHA:

| Field | Value |
|---|---|
| `ovl_exe_source_sha` | PENDING final frozen SHA |
| `ovl_exe_sha256` | PENDING final build |
| Full path | `C:\Users\mvorm\SimThing\target\debug\simthing-studio.exe` |
| Byte size / UTC build time | PENDING final build |
| Build command | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Working tree clean | PENDING final build |

## Verification ledger

| Check | Result |
|---|---|
| Focused RF-4 workshop suite | PASS, 4/4; output above |
| `cargo check -p simthing-spec` | PENDING final run |
| `cargo check -p simthing-mapeditor` | PENDING final run |
| Studio debug build | PENDING frozen-SHA build |
| Doctrine PR scan | PENDING final SHA |
| Orientation / inventory | PENDING final regeneration |
| Doc budget | PENDING final run |

## Graduation routing

Recommended posture is **PROBATION / RF-5 split approval required / Owner [OVL] OPEN**. This ledger does not claim graduation, merge authorization, PR-body clearance, need transport, or [OVL] closure.
