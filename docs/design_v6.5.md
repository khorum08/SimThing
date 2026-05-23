# SimThing — Design V6.5 Current-State Synthesis

**Purpose:** Single entry point for **implementation state**, parking, open work, and
documentation routing. V6.5 does **not** replace `design_v6.md` — that document remains
the architecture specification for simulation mechanics (overlays, fission, GPU passes,
boundary protocol). Read V6.5 first when picking up work; read V6 when changing sim behavior.

**Last updated:** 2026-05-23  
**Master HEAD:** `393db00` (parking sync post Opus O1b–O4)  
**Verification:** `cargo test --workspace` → **326** passed, **1** ignored, zero warnings.

---

## 1. Parking snapshot

| Item | Value |
|------|-------|
| **Branch** | `master` synced with `origin/master` |
| **Spec layer** | PRs 1–11 + O1 + O1b + EffectTarget + S5 + O4 complete |
| **Cursor handoff** | Complete (PRs #56–#59); O1b/S5 tests now **green** |
| **Next owner** | Codex: **O2** (replay v3) |

### Ignored tests (CI)

| Test | Crate | Notes |
|------|-------|-------|
| `pipeline_timing_1000_slots_64_dims` | `simthing-gpu` | Pre-existing perf diagnostic |

Former RED tests (now passing):

- `open_from_spec_capability_unlock_activates_overlay_for_next_tick` — O1b (`2eff1e0`)
- `fission_with_cloned_capability_subtree_reduction_topology_matches_full_rebuild` — S5 (`dcc74cc`)

---

## 2. Architecture (unchanged from V6)

SimThing is a GPU-native recursive world simulation. One type (`SimThing`), one evaluation
algorithm, one overlay lifecycle model (`Permanent | Transient | Suspended`), boundary-time
structural mutations, and CPU semantic interpretation of GPU output.

**V6 additions (landed):** suspended overlays, `ActivateOverlay` / `SuspendOverlay`,
opt-in capability-subtree cloning on fission via `clone_capability_children` +
`capability_container_kinds`. B2 fission-growth optimizations (Approaches A/B/C) are
landed; Approach C is **disabled** when fission clones capability subtrees (S5 conservative
fix). Tighter incremental topology for internal clone edges is future work.

**Spec layer (landed):** authored RON compiles to runtime artifacts in `simthing-spec`;
session ownership lives in `simthing-driver`; `simthing-sim` stays spec-free and exposes a
generic post-readback boundary hook.

For full mechanics, GPU pass order, and invariants see `design_v6.md`, `invariants.md`, and
`state-authority.md`.

---

## 3. Crate graph (as implemented)

```text
simthing-core
    ↑
simthing-feeder   ← CapabilityUnlockRegistration/Event,
                    ScriptedEventTriggerRegistration/Event
    ↑         ↑
simthing-spec     simthing-sim   ← ThresholdSemantic, extract_* bridges,
(production:      (production)     BoundaryHookContext, external threshold regs,
 core + feeder                    FissionOutcome cloned_capability_roots
 only)
    ↑
simthing-driver   ← SpecSessionState, install::compile_and_install,
                    SimSession::open_from_spec, react_to_fission_clones,
                    per-owner capability + scripted-event instances

simthing-gpu      ← WorldGpuState, TopologyState, B2 append paths
simthing-studio   ← deferred GUI (depends on spec)
```

### Key session path (O1 + O4)

1. `SimSession::open_from_spec(scenario, &game_mode)` — opens scenario, installs spec.
2. `install::compile_and_install` — properties, overlays, capability trees (per-owner clone
   with `EffectTarget` overlay placement + `overlay_hosts`), scripted events (one definition +
   N instances per `EventSpec.install`).
3. After each tick's GPU readback, driver invokes capability + scripted-event handlers via
   `BoundaryProtocol::execute_with_boundary_hook`.
4. After fission with `clone_capability_children`, `react_to_fission_clones` registers
   cloned capability instances + threshold registrations for spawned subtrees.

**Code entry points:** `install.rs`, `spec_session.rs`, `session.rs`, `boundary/capability_handler.rs`,
`boundary/event_handler.rs`, `simthing-sim/fission.rs`.

---

## 4. Landed work (post–PR 11)

| Milestone | Commit area | Notes |
|-----------|-------------|-------|
| V6 sim core | `f39fe6d`, PRs #38–#43 | Suspended overlays, capability fission clone, B2 A/B/C |
| `simthing-spec` PRs 1–11 | 2026-05-22 | Compile pipeline through session assembly |
| **O1** session install | PR #53 | `open_from_spec`, `InstallTargetSpec`, per-owner clone |
| Cursor handoff | PRs #56–#59 | O1b/S5 regression tests, install examples, kind docs |
| **O1b** | `2eff1e0` | Handler resolves per-clone overlay ids via `instance.by_overlay` |
| **EffectTarget** | `8da4be9`, `7febdd1` | ADR Accepted; `Owner` default; `overlay_hosts` + overlay placement on host |
| **S5** | `dcc74cc` | Disable Approach C when fission clones capability subtrees |
| **S5 follow-up** | `1253a97` | Overlay-id re-stamp on fission clone; `react_to_fission_clones` registers instances + thresholds |
| **O4** | `8904522` | Per-owner scripted event instances; `EventSpec.install`; ADR Accepted |

**Detailed archaeology:** `docs/workshop/simthing_spec_progress_log.md` · session notes in `worklog.md`.

---

## 5. Open work (ordered)

| Priority | ID | Owner | Scope | Notes |
|----------|-----|-------|-------|-------|
| **P0** | **O2** | Codex | Replay v3 — `SpecSnapshot` / `SpecDelta` for spec runtime state | ADR: `spec_session_state_replay.md` |
| — | Modder guide | — | `simthing_modder_object_guide.md` in repo; align with EffectTarget §14 as needed | Workshop doc |
| — | Base economy doc | — | `simthing_base_economic_system_working_doc.md` — design working doc, not implementation spec | Workshop doc |
| — | B2 topology | — | Tighter Approach C for fission clone internal edges | Future perf work |
| — | Scripted scope | — | `ScopeRef::Owner`, cross-owner events, cross-instance priority | Deferred in O4 ADR |
| — | Scenario RON expansion | — | Inline tree/registry/shadow seeds | Tabled |
| — | `simthing-studio` GUI | — | Designer surface | Tabled |

---

## 6. Known footguns

- **GPU overlay-prep vs `affects`:** Pass 3 walks the SimThing tree and applies overlays on
  hosts that carry the target property — it does **not** read `overlay.affects`. EffectTarget
  install places overlays on the correct host and stamps `overlay_hosts` for boundary activate/suspend.
- **Partial install mutation:** `compile_and_install` mutates registry/root in place on error.
  Safe for `open_from_spec` discard; Studio preview needs clone-then-commit later.
- **Replay gap:** Structural overlay activations replay via boundary delta log; spec runtime
  state (capability selections, scripted cooldowns, diagnostics) does not — **O2**.
- **Fission clone registration:** `react_to_fission_clones` synthesizes instances from source
  templates; exotic custom install paths may need explicit review.
- **Empty-boundary skip:** Non-empty scripted events may disable skip via `requires_boundary_tick()`;
  event classification revisit deferred.
- **O1c ruled out:** Registry/GPU dimension sync after install — not the blocker (`n_dims ==
  total_columns` after install).

---

## 7. Documentation map

### Read first (current state)

| Document | Role |
|----------|------|
| **This file (`design_v6.5.md`)** | Parking, open work, doc routing |
| `todo.md` | Priority table + session order |
| `worklog.md` | Session-by-session landing notes (O1b–O4) |
| `workshop/simthing_spec_progress_log.md` | PR 1–11 + O1 implementation ledger |
| `agents.md` | Agent briefing + repo layout |

### ADRs (`docs/adr/`)

| ADR | Status | Topic |
|-----|--------|-------|
| `pr11_track_a_session_assembly.md` | Accepted | Driver-owned session state, boundary hook |
| `game_mode_session_installation.md` | Accepted | RON-driven session init (O1) |
| `capability_effect_target_scope.md` | Accepted | EffectTarget — Owner default (Opus P3) |
| `scripted_event_scope_model.md` | Accepted | Per-owner scripted events (O4) |
| `spec_session_state_replay.md` | Proposed | O2 replay v3 |

### Architecture & reference

| Document | Role |
|----------|------|
| `design_v6.md` | Simulation architecture spec |
| `capability_tree_v1.md` | RON reference; §13 install; §14 EffectTarget |
| `examples/README.md` | InstallTargetSpec RON fixtures |
| `workshop/simthing_modder_object_guide.md` | Modder authoring objects |
| `workshop/simthing_base_economic_system_working_doc.md` | Base economic system (working doc) |
| `invariants.md` | Non-negotiable code rules |

---

## 8. Read order for new agents

1. **This document**
2. `docs/worklog.md` — O1b, EffectTarget, S5, O4 landing notes
3. `docs/adr/capability_effect_target_scope.md` + `scripted_event_scope_model.md`
4. `docs/adr/spec_session_state_replay.md` (before O2)
5. `docs/todo.md`
6. `docs/design_v6.md` + `docs/capability_tree_v1.md`
7. Code: `install.rs`, `spec_session.rs`, `session.rs`, boundary handlers

---

## 9. Verification

```powershell
cargo test --workspace
cargo build --workspace --tests
cargo build --workspace --release --tests
cargo test --workspace --release
git status --short --branch
```

Expected: **326** passed, **1** ignored (GPU perf bench), zero warnings, clean tracked tree.

Key E2E proofs:

```powershell
cargo test -p simthing-driver open_from_spec_capability_unlock
cargo test -p simthing-driver open_from_spec_owner_targeted_effect
cargo test -p simthing-driver open_from_spec_installs_one_scripted_event
cargo test -p simthing-sim fission_with_cloned_capability_subtree_reduction
cargo test -p simthing-driver fission_cloned_capability_subtree_registers
```
