# simthing-spec — Sonnet / Opus Handoff

**Date:** 2026-05-23  
**Master HEAD:** `e6dd9c3` (PR #68 parking sync)  
**Verification:** `cargo test --workspace` → **345** passed, **1** ignored, zero warnings  
**Parking synthesis:** [`docs/design_v6.5.md`](../design_v6.5.md)

---

## 1. Executive summary

**Opus P0 batch complete.** No P0 code work outstanding on the spec ladder.

| ID | Status | PR / commit |
|----|--------|-------------|
| O1, O1b, EffectTarget, S5, O4 | ✅ | `2eff1e0`–`8904522` |
| **O2** Replay v3 | ✅ | #65 `2f2a7b5` |
| **B3** boundary-skip precision | ✅ | #66 `defb42c` |
| **I1** install clone-then-commit | ✅ | #67 `6b8de81` |

**Next work:** Sonnet/Composer **authoring surface** (examples, modder guide, reference docs). Opus optional design-only (S6, I2, H1). **E0 base economy deferred** — do not start.

### Agent split

| Agent | Sweet spot | Do **not** |
|-------|------------|------------|
| **Opus** | ADRs, cross-crate architecture, new semantic overlays/runtimes | Mechanical doc/example PRs |
| **Sonnet / Composer** | Examples, modder guide, parse smoke tests, parking doc sync, B2′ mechanical | Redesign ADR decisions without Opus |

---

## 2. Architecture (spec layer)

```text
simthing-driver
  install.rs       preview_install, install_atomic, apply_install_preview (I1)
  spec_replay.rs   SpecSnapshot, SpecDelta, open_replay_with_spec (O2)
  spec_session.rs  requires_boundary_tick (B3), capability + scripted instances
  session.rs       open_from_spec, record_to_path, react_to_fission_clones

simthing-spec
  boundary/        capability + scripted event handlers
  compile/         builders, event compiler
  spec/script.rs   ScopeRef { Current, Slot } — S6 would extend here
```

---

## 3. Opus P0 — complete

### O2 Replay v3 (#65)

- `spec_replay.rs`: `SpecSnapshot`, `SpecDelta`, logical keys only
- LDJSON: `spec_snapshot` line + per-frame `spec_entries`
- `open_replay_with_spec`, round-trip E2E in `session_integration.rs`
- ADR: [`spec_session_state_replay.md`](../adr/spec_session_state_replay.md) → **Accepted**

### B3 boundary skip (#66)

- `requires_boundary_tick`: 6 precise force-tick conditions
- Threshold-only scripted sessions skip quiet boundaries again
- Integration tests: `b3_threshold_only_*`, `b3_predicate_*`

### I1 install atomicity (#67)

- `preview_install`, `install_atomic`, `apply_install_preview`
- `open_from_spec` uses `install_atomic`
- ADR: [`install_clone_then_commit.md`](../adr/install_clone_then_commit.md) → **Accepted**

---

## 4. Sonnet / Composer — active backlog

| Priority | ID | Scope |
|----------|-----|-------|
| **P1** | **D2** | `docs/examples/` — Owner `effect_target`, per-faction `EventSpec.install`, CapabilityTree-opt-in |
| **P1** | **T1** | `loads_*_examples` parse smoke tests (`simthing-spec/tests/`) |
| **P1** | **D1** | `simthing_modder_object_guide.md` — `effect_target`, `overlay_hosts`, `EventSpec.install`, preview/replay |
| **P2** | **D3** | `capability_tree_v1.md` — preview `owner_slot`/`root_slot` worked example |
| **P2** | **R1** | Replay fixture + README for `open_replay_with_spec` |
| **P2** | **B2′** | Append-only external thresholds on fission-clone path |
| **P2** | **P1** | Preview/diagnostic `Display` polish (Track B style) |

**Parallel-safe now** — no Opus gate for D1/D2/T1.

---

## 5. Opus — optional (on demand)

| ID | Scope | Crate |
|----|-------|-------|
| **S6** | `ScopeRef::Owner` ADR + implementation | `simthing-spec` script IR + event handler |
| **I2** | Mid-session `apply_install_preview` + GPU resync ADR | `simthing-driver` + `simthing-gpu` |
| **H1** | Spec hot-reload with state preservation | `simthing-driver` (replay merge semantics) |

**E0 / base economy:** deferred — ignore A0–A8 until explicitly requested.

---

## 6. Tabled

- `simthing-studio` GUI
- Scenario RON expansion
- EML / Clausewitz
- B2 tighter Approach C for fission clone internal edges
- Cross-owner scripted events, cross-instance priority (O4 ADR deferred)

---

## 7. Footguns

1. **Overlay placement vs `affects`** — GPU overlay-prep walks host tree; EffectTarget + `overlay_hosts` drive routing.
2. **Mid-session preview** — I1 covers session-open atomicity; running-session apply needs I2.
3. **Replay keys** — logical keys only; no raw `OverlayId` in spec replay payloads.
4. **345 / 1 ignored** — GPU perf bench only.

---

## 8. Key files

| Area | Path |
|------|------|
| Spec replay (O2) | `crates/simthing-driver/src/spec_replay.rs` |
| Install / preview (I1) | `crates/simthing-driver/src/install.rs` |
| Boundary skip (B3) | `crates/simthing-driver/src/spec_session.rs` |
| E2E tests | `crates/simthing-driver/tests/session_integration.rs` |
| Capability handler | `crates/simthing-spec/src/boundary/capability_handler.rs` |
| Event handler | `crates/simthing-spec/src/boundary/event_handler.rs` |

---

## 9. One-line pickup

**Opus:** P0 done — park or pick S6/I2/H1 on demand. **Do not start E0.**  
**Sonnet:** D2 + T1 examples, then D1 modder guide; D3/R1 reference docs.
