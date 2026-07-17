# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status
**PROBATION (proof-present, DA-review-pending)** — remand 4 / OVL build gate. **[OVL] OPEN**.

## Identity
| Field | Value |
|---|---|
| Rung | `STUDIO-FIELD-SESSION-ELEVATE-0` (12.9) |
| Branch | `coder/studio-field-session-elevate-0` |
| birth_track | `0.0.8.6-studio-live-ops` |
| HD-RECEIPT | `2976856875d0` (PR-body continuity base `5edbc7cbc863`) |

## What changed
- Field-bearing path via production Studio bridge; structural-shell fallback
- Telemetry sampler binds to **materialized emission `source_slot`/`source_col`** (not hard-coded slot 0); open tick-0 sample + per-tick samples
- Owner-slot policy with-vs-without; live Rising threshold (canonical below-thr clone); fail-closed GPU
- Workshop drives production bridge only
- **Need/weight_profile: STOP / scope gap (see below)** — not silently relabeled as “need”
- **[OVL]** Windows debug Studio build + operator runbook (exe not committed)

## Need-profile doctrine gap (honest STOP)
| Query | Finding |
|---|---|
| Authored surface | Canonical ClauseScript has three `weight_profile` EML stacks (`HydratedFieldEconomy.weight_profiles` → `EmlGadgetStackSpec` / WeightedAccumulator) |
| Lowered into GameMode? | **No.** `FieldEconomyLowering` only emits `properties`, `overlays`, `ResourceEconomySpec` — weight_profiles stay on hydrate pack only |
| GameModeSpec field | **None** for EmlGadgetStack / weight profiles |
| open_from_spec install | Installs resource_economy + domain-pack overlays; **no** admitted EML-gadget install for field-economy weight stacks |
| Existing EML seams | Driver EML registries exist for RF/mapping/gated rates, not for field-economy `weight_profile` via mapeditor open |

**Verdict:** No admitted generic seam to execute authored need/weight profiles without new grammar/spec surface, driver install, or bespoke tick logic. Need-output live proof is **not claimed**. Proven: disruption accretion, production transfer, owner-policy live differentials, threshold events under ordinary ticks, Studio_ops sample deltas. Correction of handoff EXIT-PROOF “need” wording is orchestrator/Owner adjudication residue.

## Load-bearing proofs
| suite | tests |
|---|---|
| `studio_field_session_elevate_0` | 7/7 |
| `tp_field_session_elevate_0` | 3/3 |
| regression `studio_live_session_bridge_0` | 8/8 |

## Scope Ledger
| | |
|---|---|
| Specified | Field-bearing open; live accretion; threshold-only decisions; structural fallback; [OVL] telemetry + debug build |
| Implemented | As above; need profiles **not** executed |
| Deferred | Owner [OVL] screenshots; need-profile install seam (if authorized) |
| Out of scope | Kernel/WGSL widen; inventing need APIs; class/router edits |

## Graduation routing
| Field | Value |
|---|---|
| CI verdict | local focused battery PASS; hosted re-check at final head |
| Triage | TEST-BUDGET justified (7 mapeditor proofs); evidence-only triage rebind |
| Risk class | studio-live-ops elevation; OVL telemetry honesty |
| Falsification | corrupt sampler slot/col → readout sample delta fails; strip owner_policy → owner-slot delta fails; strip thresholds → live events fail; Unsupported fails closed |
| Posture | PROBATION; **[OVL] OPEN**; do not claim GRADUATED; clearance may remain class-envelope-violation |

## [OVL] Windows debug build + operator runbook

### Build (at final source SHA)
```powershell
$head = git rev-parse HEAD
cargo build -p simthing-mapeditor --bin simthing-studio
$exe = Resolve-Path .\target\debug\simthing-studio.exe
Get-Item $exe | Select-Object FullName,Length,LastWriteTime
Get-FileHash $exe -Algorithm SHA256
& $exe
```
Do **not** commit `target/**`, the `.exe`, or screenshots.

### Click sequence (Owner)
1. Launch `target\debug\simthing-studio.exe` built at the reported `$head`.
2. Load **`scenarios\terran_pirate_galaxy.clause`** via Studio scenario loader.
3. Ensure session preference is **auto** or **field-bearing** (not structural-shell-only).
4. Start **live ticks** (Play / unpause sim clock).
5. Open top-right **Telemetry**.
6. Select **Show Studio_ops Telemetry**.

### Required screenshots
- **A — identity:** canonical scenario context; `session path = field-bearing`; preference auto/field-bearing; production path `SimSession::open_from_spec + step_once`; no error/unsupported.
- **B — live accretion:** executed ticks > 0; `tp_economy::...` rows; same property at multiple tick indices with **different amounts**; decision counters visible (zero OK if no real crossing in that interval — do not manufacture a visual event).
- **C — progression:** later tick count and later changed value for the same canonical property.
