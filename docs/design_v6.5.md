# SimThing — Design V6.5 Current-State Synthesis

**Purpose:** Single entry point for **implementation state**, parking, open work, and
documentation routing. V6.5 does **not** replace `design_v6.md` — that document remains
the architecture specification for simulation mechanics (overlays, fission, GPU passes,
boundary protocol). Read V6.5 first when picking up work; read V6 when changing sim behavior.

**Last updated:** 2026-05-23  
**Master HEAD:** `2ff84bf` (PR #69 parking sync)  
**Verification:** `cargo test --workspace` → **345** passed, **1** ignored, zero warnings.

---

## 1. Parking snapshot

| Item | Value |
|------|-------|
| **Branch** | `master` synced with `origin/master` |
| **Spec layer** | PRs 1–11 + O1–O4 + O1b + EffectTarget + S5 + **O2 + B3 + I1** complete |
| **Opus P0 batch** | Complete (PRs #65–#67); no P0 code work outstanding |
| **Next owner** | **Sonnet/Composer** — authoring docs & examples (D1/D2/D3); see handoff |

### Ignored tests (CI)

| Test | Crate | Notes |
|------|-------|-------|
| `pipeline_timing_1000_slots_64_dims` | `simthing-gpu` | Pre-existing perf diagnostic |

---

## 2. Architecture (unchanged from V6)

SimThing is a GPU-native recursive world simulation. One type (`SimThing`), one evaluation
algorithm, one overlay lifecycle model (`Permanent | Transient | Suspended`), boundary-time
structural mutations, and CPU semantic interpretation of GPU output.

**V6 additions (landed):** suspended overlays, fission capability clone, B2 A/B/C (Approach C
disabled when fission clones capability subtrees — S5).

**Spec layer (landed):** RON → runtime in `simthing-spec`; session state in `simthing-driver`;
`simthing-sim` spec-free with generic post-readback boundary hook.

For full mechanics see `design_v6.md`, `invariants.md`, `state-authority.md`.

---

## 3. Crate graph (as implemented)

```text
simthing-core
    ↑
simthing-feeder   ← CapabilityUnlockRegistration/Event,
                    ScriptedEventTriggerRegistration/Event
    ↑         ↑
simthing-spec     simthing-sim   ← BoundaryHookContext, threshold bridges,
(production:      (production)     replay v2 frames + spec_entries slot
 core + feeder)
    ↑
simthing-driver   ← SpecSessionState, install_atomic / preview_install,
                    spec_replay (O2), open_from_spec, react_to_fission_clones

simthing-gpu      ← WorldGpuState, TopologyState, B2 append paths
simthing-studio   ← deferred GUI
```

### Key session paths

**Open:** `SimSession::open_from_spec` → `install_atomic` (I1 clone-then-commit)  
**Preview:** `preview_install` → `apply_install_preview` (Studio-safe; no in-place mutation)  
**Replay:** `record_to_path` emits `spec_snapshot` + per-frame `spec_entries`; `open_replay_with_spec` restores spec runtime  
**Boundary:** capability + scripted handlers after GPU readback; `requires_boundary_tick` uses 6 precise conditions (B3)

**Code entry points:** `install.rs`, `spec_session.rs`, `spec_replay.rs`, `session.rs`, boundary handlers.

---

## 4. Landed work (post–PR 11)

| Milestone | PR / commit | Notes |
|-----------|-------------|-------|
| O1–O4, O1b, EffectTarget, S5 | `2eff1e0`–`8904522` | Session install, per-owner events, fission hooks |
| **O2 Replay v3** | #65 `2f2a7b5` | `SpecSnapshot`/`SpecDelta`, logical keys, round-trip E2E |
| **B3 boundary skip** | #66 `defb42c` | Precise `requires_boundary_tick`; threshold-only scripted sessions can skip |
| **I1 install atomicity** | #67 `6b8de81` | `preview_install`, `install_atomic`, `apply_install_preview` |
| Workshop docs | #63–#64 | Modder guide, base economy working doc, Sonnet/Opus handoff |

**Archaeology:** `workshop/simthing_spec_progress_log.md` · `worklog.md`

---

## 5. Open work (ordered)

**No P0 code items.** Next work is Sonnet/Composer authoring surface (see handoff).

| Priority | ID | Owner | Scope |
|----------|-----|-------|-------|
| P1 | **D2** / **T1** | Sonnet | `docs/examples/` — EffectTarget, per-faction events; parse smoke tests |
| P1 | **D1** | Sonnet | Modder guide — `effect_target`, `overlay_hosts`, `EventSpec.install`, replay/preview |
| P2 | **D3** / **R1** | Sonnet | `capability_tree_v1.md` preview docs; replay example in examples/ |
| P2 | **B2′** | Sonnet | Append-only external thresholds on fission-clone path |
| — | **S6** | Opus (optional) | `ScopeRef::Owner` ADR — `simthing-spec` script IR |
| — | **I2** / **H1** | Opus (optional) | Mid-session GPU resync; spec hot-reload |
| — | Studio / scenario RON / E0 economy | — | **Tabled** (E0 explicitly deferred) |

---

## 6. Known footguns

- **GPU overlay-prep vs `affects`:** Pass 3 uses overlay **placement** on the host tree; `overlay_hosts` drives boundary activate/suspend.
- **Mid-session preview apply:** `apply_install_preview` on a *running* session needs GPU resync (I1 ADR §Out of scope — I2).
- **Spec hot-reload:** preserving cooldowns/selections across re-install needs replay-style merge (H1).
- **Fission clone registration:** `react_to_fission_clones` synthesizes from source instance — exotic install paths need review.
- **Replay logical keys:** never serialize raw `OverlayId` for spec state (O2 ADR M1).
- **O1c ruled out** — dimension sync after install not the blocker.

---

## 7. Documentation map

### Read first

| Document | Role |
|----------|------|
| **This file** | Parking synthesis |
| `workshop/simthing_spec_sonnet_opus_handoff.md` | Sonnet vs Opus task split |
| `todo.md` | Priority table |
| `worklog.md` | O2, B3, I1 landing notes |

### ADRs (`docs/adr/`)

| ADR | Status | Topic |
|-----|--------|-------|
| `pr11_track_a_session_assembly.md` | Accepted | Driver session state, boundary hook |
| `game_mode_session_installation.md` | Accepted | O1 session install |
| `capability_effect_target_scope.md` | Accepted | EffectTarget — Owner default |
| `scripted_event_scope_model.md` | Accepted | O4 per-owner scripted events |
| `spec_session_state_replay.md` | Accepted | O2 replay v3 |
| `install_clone_then_commit.md` | Accepted | I1 preview / atomic install |

---

## 8. Read order for new agents

1. **This document**
2. `workshop/simthing_spec_sonnet_opus_handoff.md`
3. `worklog.md` — O2, B3, I1 entries
4. ADRs: replay, install atomicity, EffectTarget, scripted scope
5. `todo.md` · `design_v6.md` · `capability_tree_v1.md`
6. Code: `spec_replay.rs`, `install.rs`, `spec_session.rs`, `session.rs`

---

## 9. Verification

```powershell
cargo test --workspace
```

Expected: **345** passed, **1** ignored, zero warnings.

Key E2E proofs:

```powershell
cargo test -p simthing-driver record_and_replay_with_spec
cargo test -p simthing-driver b3_threshold_only_scripted
cargo test -p simthing-driver i1_apply_install_preview
cargo test -p simthing-driver open_from_spec_capability_unlock
cargo test -p simthing-driver open_from_spec_owner_targeted_effect
cargo test -p simthing-sim fission_with_cloned_capability_subtree_reduction
```
