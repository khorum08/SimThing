# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status
**BLOCKED — NEED-PROFILE INSTALL SEAM / OWNER-ORCHESTRATOR ADJUDICATION** — remand-4 telemetry accepted; full 12.9 exit (production/**need** accrete) not claimed. **[OVL] OPEN** — Owner screenshots required. Local Windows debug Studio build recorded below (exe not committed).

## Identity
| Field | Value |
|---|---|
| Rung | `STUDIO-FIELD-SESSION-ELEVATE-0` (12.9) |
| Branch | `coder/studio-field-session-elevate-0` |
| birth_track | `0.0.8.6-studio-live-ops` |
| HD-RECEIPT | `aed2a0dbc147` (PR-body continuity base `5edbc7cbc863`) |
| OVL build source SHA | `02d446cf287b658863a6ca3a04c60736ef7e21c1` |

## What changed
- Field-bearing path via production Studio bridge; structural-shell fallback
- Telemetry sampler binds to **materialized emission `source_slot`/`source_col`** (not hard-coded slot 0); open tick-0 sample + per-tick samples
- Owner-slot policy with-vs-without; live Rising threshold (canonical below-thr clone); fail-closed GPU
- Workshop drives production bridge only
- **Need/weight_profile: STOP / scope gap (see below)** — not silently relabeled as “need”
- **[OVL]** Windows debug `simthing-studio` built + launched at source SHA above; Owner capture runbook below

## Need-profile doctrine gap (honest STOP)
| Query | Finding |
|---|---|
| Authored surface | Canonical ClauseScript has three `weight_profile` EML stacks (`HydratedFieldEconomy.weight_profiles` → `EmlGadgetStackSpec` / WeightedAccumulator) |
| Lowered into GameMode? | **No.** `FieldEconomyLowering` only emits `properties`, `overlays`, `ResourceEconomySpec` — weight_profiles stay on hydrate pack only |
| GameModeSpec field | **None** for EmlGadgetStack / weight profiles |
| open_from_spec install | Installs resource_economy + domain-pack overlays; **no** admitted EML-gadget install for field-economy weight stacks |
| Existing EML seams | Driver EML registries exist for RF/mapping/gated rates, not for field-economy `weight_profile` via mapeditor open |

**Verdict:** No admitted generic seam to execute authored need/weight profiles without new grammar/spec surface, driver install, or bespoke tick logic. Canonical 12.9 EXIT still includes **production/need accrete from authored buildings and policy overlays** — need half is **blocked**, not amended by coder. Proven subset: disruption accretion, production transfer, owner-policy live differentials, threshold events under ordinary ticks, Studio_ops sample deltas. Disposition of need criterion (split 12.9a vs design amend) is **Owner/orchestrator adjudication**.

## Load-bearing proofs
| suite | tests |
|---|---|
| `studio_field_session_elevate_0` | 7/7 |
| `tp_field_session_elevate_0` | 3/3 |
| regression `studio_live_session_bridge_0` | 8/8 |

## Scope Ledger
| | |
|---|---|
| Specified | Field-bearing open; live production/**need** accretion + policy overlays; threshold-only decisions; structural fallback; [OVL] telemetry + debug build |
| Implemented | Field-bearing/session/telemetry + production/policy/threshold proofs; need profiles **not** executed; local OVL debug exe built |
| Deferred / blocked | Need-profile install seam (Owner/orchestrator adjudicate); Owner [OVL] screenshots |
| Out of scope | Kernel/WGSL widen; inventing need APIs; class/router edits; self-narrowing EXIT; committing exe/target/** |

## Graduation routing
| Field | Value |
|---|---|
| CI verdict | local focused battery PASS; hosted re-check at final head |
| Triage entries | TEST-BUDGET justified (7 mapeditor + 3 workshop proofs); evidence-only triage rebind |
| Risk class | studio-live-ops elevation; OVL telemetry honesty; partial 12.9 |
| Falsification check | corrupt sampler slot/col → readout sample delta fails; strip owner_policy → owner-slot delta fails; strip thresholds → live events fail; Unsupported fails closed |
| Recommended posture | **BLOCKED** (need-seam / adjudication); **not** full 12.9 PROBATION; **[OVL] OPEN**; do not claim GRADUATED; clearance may remain class-envelope-violation |

## [OVL] Windows debug build evidence (local only)

| Field | Value |
|---|---|
| Source SHA (`git rev-parse HEAD` at build) | `02d446cf287b658863a6ca3a04c60736ef7e21c1` |
| Build command | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Executable | `C:\Users\mvorm\SimThing\target\debug\simthing-studio.exe` |
| Size (bytes) | `86716416` |
| SHA-256 | `98A9807E2FA705912CBE5E12DA7ED138DDD062A8D384CDE9C1DAFB3FCBDB8CE7` |
| Launch | **OK** — process started (pid observed), `Responding=True`, terminated after ~10s verify (no panic/early exit) |
| Committed? | **No** — do not commit `target/**`, `.exe`, or screenshots |

### Rebuild recipe (from repo root)
```powershell
$head = git rev-parse HEAD
cargo build -p simthing-mapeditor --bin simthing-studio
$exe = Resolve-Path .\target\debug\simthing-studio.exe
Get-Item $exe | Select-Object FullName,Length,LastWriteTime
Get-FileHash $exe -Algorithm SHA256
& $exe
```

## [OVL] Owner capture path (numbered)

Begin at repository root `C:\Users\mvorm\SimThing` (or the Owner’s clone of the same source SHA).

1. **Build** (if not already at the reported SHA):
   ```powershell
   git checkout 02d446cf287b658863a6ca3a04c60736ef7e21c1
   cargo build -p simthing-mapeditor --bin simthing-studio
   ```
2. **Launch** exact executable:
   ```powershell
   & .\target\debug\simthing-studio.exe
   ```
   Expected: Studio window opens without panic.
3. **Load canonical clause scenario**
   - Left panel → **Library...**
   - Modal **Load ClauseScript Scenario**
   - **Select File…** → choose `scenarios\terran_pirate_galaxy.clause` (repo-relative path under clone root)
   - Click **Load**; wait for loader stages to pass (no red Failure rows)
4. **Start live ticks**
   - Left panel → **Sim clock (transport only)**
   - Click **Play** (not Pause); optional rate **1×** / **2×**
   - Confirm left-panel line `Live bridge: … · Executed: N` with N increasing, or Studio_ops `executed ticks` > 0
5. **Open Studio Ops telemetry**
   - Top-right toolbar → **Telemetry**
   - In the Performance Telemetry dialog → **Show Studio_ops Telemetry**
   - Confirm window **Studio_ops Telemetry** shows:
     - `session path` = `field-bearing`
     - `path preference` = `auto` or `field-bearing`
     - `production path` = `simthing_driver::SimSession::open_from_spec + step_once`
     - no unsupported/error label
6. **Screenshots**
   - **A — identity:** after load + Studio_ops open; field-bearing path labels visible; canonical scenario context; zero or low ticks OK; no error.
   - **B — live accretion:** after Play advances ticks (`executed ticks` > 0); **Field accretion samples** table shows `tp_economy::…` rows; prefer `tp_economy::pirate_outpost_disruption_presence`; same property at ≥2 tick indices with **different amounts**; decision counters visible (zero OK — do not manufacture a threshold event).
   - **C — progression:** later capture same session; higher executed-tick count; same property amount changed again from B.
7. **OVL failure conditions** (any one fails the visual gate)
   - field-bearing path not shown / production path remains structural-shell only
   - ticks do not advance under Play
   - Field accretion table empty or amounts static across ticks
   - unsupported/error runtime state
   - canonical `terran_pirate_galaxy.clause` cannot be loaded through the executable

**[OVL] remains OPEN** until the Owner supplies and approves screenshots A/B/C. Coder does not close OVL.
