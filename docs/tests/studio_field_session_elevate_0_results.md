# STUDIO-FIELD-SESSION-ELEVATE-0 Results

## Status
**BLOCKED — NEED-PROFILE INSTALL SEAM / OWNER-ORCHESTRATOR ADJUDICATION** plus **[OVL] FAIL/OPEN (Remand 5)** until replacement screenshots. Remand-5 staged-loader profile attach landed; prior OVL exe superseded.

## Identity
| Field | Value |
|---|---|
| Rung | `STUDIO-FIELD-SESSION-ELEVATE-0` (12.9) |
| Branch | `coder/studio-field-session-elevate-0` |
| birth_track | `0.0.8.6-studio-live-ops` |
| HD-RECEIPT | `aed2a0dbc147` (PR-body continuity base `5edbc7cbc863`) |

## What changed
- Field-bearing path via production Studio bridge; structural-shell fallback
- Telemetry sampler binds to **materialized emission `source_slot`/`source_col`**; open tick-0 + per-tick samples
- Owner-slot policy with-vs-without; live Rising threshold; fail-closed GPU
- Workshop drives production bridge only
- **Remand 5:** staged UI loader (`run_clause_picker_action_staged`) attaches `authored_live_profile_from_pack` after reloaded Spec authority
- **Remand 6:** staged-picker falsifier uses **neutral foundry** TempDir `.clause` (no sealed-crate `terran`/`pirate` path tokens); WORKSHOP-HOMING-DETECTION must report **PASS 0** on exact-head scan
- **Need/weight_profile:** still STOP / scope gap (not silent relabel)
- **[OVL]** FAIL recorded below; replacement debug build after Remand 5 (exe source `f3bdd632`; test-only Remand 6 does not supersede)

## Owner OVL FAIL (Remand 5) — do not treat as success
| Field | Value |
|---|---|
| Reviewed PR head | `674f9448be7b8a5a22de3c0358a90c6132e97f3f` |
| Owner-tested executable source | `02d446cf287b658863a6ca3a04c60736ef7e21c1` |
| Old exe SHA-256 | `98A9807E2FA705912CBE5E12DA7ED138DDD062A8D384CDE9C1DAFB3FCBDB8CE7` — **superseded; not for closure** |
| A (tick 0) | `session path = structural-shell`; production `SimSession::open + step_once`; no field samples |
| B (tick 354) | still structural-shell; samples empty |
| C (tick 1844) | still structural-shell; samples empty |
| Notes | Loader stages passed; RF ready; clock advanced — **not** Unsupported/timing |

**Root cause:** `run_clause_picker_action_staged` rebuilt the session from reloaded JSON via plain `from_loaded_scenario` and never attached `authored_live_profile_from_pack(&ingest.pack)`, so Auto fell through to structural-shell. Headless tests that inject profile directly did not bite.

**Fix:** attach profile on the staged path after SessionBuild reload; biting test `staged_clause_picker_preserves_profile_into_auto_field_bearing_live_bridge` drives the real staged controller.

Launch-only smoke on the old binary is **not** OVL success.

## Need-profile doctrine gap (honest STOP)
| Query | Finding |
|---|---|
| Authored surface | Canonical ClauseScript has three `weight_profile` EML stacks |
| Lowered into GameMode? | **No** — weight_profiles stay on hydrate pack only |
| open_from_spec install | No admitted EML-gadget install for field-economy weight stacks |

Need half of 12.9 remains **blocked** pending Owner/orchestrator adjudication (separate from OVL loader fix).

## Load-bearing proofs
| suite | tests |
|---|---|
| `studio_field_session_elevate_0` | 8/8 |
| `tp_field_session_elevate_0` | 3/3 |
| regression `studio_live_session_bridge_0` | 8/8 |

## Scope Ledger
| | |
|---|---|
| Specified | Field-bearing open via real UI load; live production/**need** + policy; threshold-only decisions; structural fallback; OVL telemetry |
| Implemented | Staged UI profile preserve; field-bearing/session/telemetry + production/policy/threshold proofs; need profiles **not** executed |
| Deferred / blocked | Need-profile install seam; replacement Owner [OVL] screenshots after new exe |
| Out of scope | Kernel/WGSL; invent need APIs; class/router; commit exe |

## Graduation routing
| Field | Value |
|---|---|
| CI verdict | local focused battery PASS; hosted re-check at final head |
| Triage entries | TEST-BUDGET justified (8 mapeditor + 3 workshop proofs) |
| Risk class | studio-live-ops elevation; OVL production-route honesty; partial 12.9 |
| Falsification check | strip staged profile attach → Auto structural-shell / empty samples; corrupt sampler → readout delta fails; Unsupported fails closed |
| Recommended posture | **BLOCKED** need-seam; **[OVL] FAIL/OPEN** until replacement A/B/C; not GRADUATED |

## [OVL] Windows debug build evidence (replacement after Remand 5)

| Field | Value |
|---|---|
| Source SHA | `f3bdd632a471bbba074c05a823b0391157399e97` |
| Build command | `cargo build -p simthing-mapeditor --bin simthing-studio` |
| Executable | `C:\Users\mvorm\SimThing\target\debug\simthing-studio.exe` |
| Size (bytes) | `86725632` |
| SHA-256 | `A1F050B41390678E3ECC36ECB66B0985F3510DC261660732044E8125DD709AF6` |
| Launch | **OK** — process Responding=True; terminated after verify |
| Committed? | **No** — do not use superseded hash `98A9807E…` |

### Rebuild recipe (from repo root)
```powershell
$head = git rev-parse HEAD
cargo build -p simthing-mapeditor --bin simthing-studio
$exe = Resolve-Path .\target\debug\simthing-studio.exe
Get-Item $exe | Select-Object FullName,Length,LastWriteTime
Get-FileHash $exe -Algorithm SHA256
& $exe
```

## [OVL] Owner capture path (replacement screenshots)

Begin at repo root with the **new** exe only (old hash superseded).

1. Build at reported final SHA; launch `.\target\debug\simthing-studio.exe`
2. Left panel **Library...** → **Select File…** → `scenarios\terran_pirate_galaxy.clause` → **Load**
3. Sim clock → **Play**
4. Top-right **Telemetry** → **Show Studio_ops Telemetry**
5. **Required visual result**
   - `session path` = **`field-bearing`**
   - `production path` = **`simthing_driver::SimSession::open_from_spec + step_once`**
   - executed ticks increase
   - Field accretion samples show `tp_economy::...` rows
   - same property at ≥2 tick indices with changed amounts
6. Screenshots **A** identity (field-bearing), **B** live accretion, **C** later progression
7. Fail OVL if still structural-shell, empty table, stuck ticks, error/unsupported, or clause cannot load

**[OVL] remains OPEN** until Owner supplies and approves replacement A/B/C.
