# SimThing — Design V6.5 Current-State Synthesis

**Purpose:** Single entry point for **implementation state**, parking, open work, and
documentation routing. V6.5 does **not** replace `design_v6.md` — that document remains
the architecture specification for simulation mechanics (overlays, fission, GPU passes,
boundary protocol). Read V6.5 first when picking up work; read V6 when changing sim behavior.

**Last updated:** 2026-05-23  
**Master HEAD:** `030ef3e` (PR #61 V6.5 doc synthesis)  
**Verification:** `cargo test --workspace` → **323** passed, **3** ignored, zero warnings.

---

## 1. Parking snapshot

| Item | Value |
|------|-------|
| **Branch** | `master` synced with `origin/master` |
| **Spec layer** | `simthing-spec` PRs 1–11 complete; O1 session install landed (PR #53) |
| **Cursor handoff** | Complete (PRs #56–#59); two RED tests await Codex fixes |
| **Next owners** | Codex: O1b → S5 → O4/O2 · Opus: EffectTarget ADR |

### Ignored tests (must stay green in CI)

| Test | Crate | Blocker |
|------|-------|---------|
| `open_from_spec_capability_unlock_activates_overlay_for_next_tick` | `simthing-driver` | O1b: handler uses template `overlay_ids`, not per-clone `instance.by_overlay` |
| `fission_with_cloned_capability_subtree_reduction_topology_matches_full_rebuild` | `simthing-sim` | S5: Approach C topology append misses cloned capability-subtree edges |
| GPU day-boundary timing budget | `simthing-gpu` | Pre-existing perf diagnostic |

---

## 2. Architecture (unchanged from V6)

SimThing is a GPU-native recursive world simulation. One type (`SimThing`), one evaluation
algorithm, one overlay lifecycle model (`Permanent | Transient | Suspended`), boundary-time
structural mutations, and CPU semantic interpretation of GPU output.

**V6 additions (landed):** suspended overlays, `ActivateOverlay` / `SuspendOverlay`,
opt-in capability-subtree cloning on fission via `clone_capability_children` +
`capability_container_kinds`.

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
(production:      (production)     BoundaryHookContext, external threshold regs
 core + feeder
 only)
    ↑
simthing-driver   ← SpecSessionState, install::compile_and_install,
                    SimSession::open_from_spec, boundary hook wiring

simthing-gpu      ← WorldGpuState, TopologyState, B2 append paths
simthing-studio   ← deferred GUI (depends on spec)
```

### Key session path (O1)

1. `SimSession::open_from_spec(scenario, &game_mode)` — opens scenario, then installs spec.
2. `install::compile_and_install` — compiles properties/overlays/trees from `GameModeSpec`,
   resolves `InstallTargetSpec` per owner, clones capability trees with fresh `OverlayId`s,
   registers external threshold registrations on full GPU rebuild.
3. After each tick's GPU readback, driver invokes capability + scripted-event handlers via
   `BoundaryProtocol::execute_with_boundary_hook`.

**Code entry points:** `crates/simthing-driver/src/install.rs`, `spec_session.rs`, `session.rs`;
`crates/simthing-spec/src/boundary/`; `crates/simthing-sim/src/boundary.rs`.

---

## 4. Landed work (spec + session)

| Milestone | PR / commit area | Notes |
|-----------|------------------|-------|
| V6 sim core | `f39fe6d`, PRs #38–#43 | Suspended overlays, capability fission clone, B2 A/B/C |
| `simthing-spec` PRs 1–10 | 2026-05-22 | Compile pipeline, builders, handlers, script IR, event compiler |
| PR 11 Track A | `01fb572`, ADR | Driver-owned `SpecSessionState`, boundary hook in sim |
| PR 11 Track B | PR #47 | Append-only threshold helpers, diagnostic `Display`, docs |
| Phase 1 ADRs | PR #50 | O1 install, O4 scope, O2 replay (design only) |
| Composer S1–S4, O3 | PRs #49–#52 | Crate docs, reverse maps, player selection API |
| **O1** session install | PR #53 | `open_from_spec`, `InstallTargetSpec`, per-owner clone |
| Doc/examples | PRs #54–#58 | Parking sync, install RON examples, kind strings §13–§14 |
| Cursor handoff tests/docs | PRs #56–#59 | O1b + S5 RED tests; examples; v0 effect-scope warning |

**Detailed PR archaeology:** `docs/workshop/simthing_spec_progress_log.md` (canonical ledger).

---

## 5. Open work (ordered)

| Priority | ID | Owner | Scope | Gate |
|----------|-----|-------|-------|------|
| **P0** | **O1b** | Codex | Fix `CapabilityTreeBoundaryHandler::emit_activation` to resolve overlay ids via per-clone `instance.by_overlay`, not template `CapabilityDefinition.overlay_ids` | Un-ignore O1b E2E test |
| **P1** | **S5/O5** | Codex | Disable Approach C topology append when `clone_capability_children`; conservative fix first; append-only external thresholds for new clones | Un-ignore S5 test |
| **P1** | **O1c** | — | Registry/GPU dimension sync after install — **ruled out** by O1b run (`n_dims == registry.total_columns`) | Only if reopened |
| **P2** | **O4** | Codex | Per-owner scripted events (Option B in ADR) | After O1b green |
| **P2** | **O2** | Codex | Replay v3 (`SpecSnapshot` / `SpecDelta`); `by_overlay` on instance is precondition | After O1b green |
| **P3** | **EffectTarget** | Opus | ADR: capability effect target scope (`Owner` vs `CapabilityTree` vs `SessionRoot`) | Before Studio/modder exposure |
| — | Scenario RON expansion | — | Inline tree/registry/shadow seeds | Tabled |
| — | `simthing-studio` GUI | — | Designer surface | Tabled; depends on spec |

**Do not start O4/O2 until O1b handler fix is green.**

---

## 6. Known footguns

- **O1b RED:** Install re-stamps overlay ids on each clone (`instance.by_overlay`), but the
  capability handler still emits `ActivateOverlay` with template ids → misses cloned tree overlays.
- **Overlay `affects` (v0):** Capability effect overlays target the **cloned tree**, not the
  owner. Documented in `capability_tree_v1.md` §14; modder semantics pending Opus ADR.
- **Partial install mutation:** `compile_and_install` mutates registry/root in place on error.
  Safe for `open_from_spec` discard; Studio preview needs clone-then-commit later.
- **B2 Approach C:** Incremental topology append is incorrect when fission clones multi-node
  capability subtrees (S5). Conservative fix: disable append for that case.
- **Replay gap:** Structural overlay activations replay via boundary delta log; spec runtime
  state (capability selections, scripted cooldowns, diagnostics) does not — O2.
- **Empty-boundary skip:** Non-empty `scripted_events` disables skip via `requires_boundary_tick()`;
  revisit in O4.

---

## 7. Documentation map

### Read first (current state)

| Document | Role |
|----------|------|
| **This file (`design_v6.5.md`)** | Parking, open work, doc routing |
| `todo.md` | Priority table + session order |
| `workshop/simthing_spec_progress_log.md` | PR 1–11 + O1 implementation ledger |
| `agents.md` | Agent briefing + repo layout |

### Architecture & reference

| Document | Role |
|----------|------|
| `design_v6.md` | Simulation architecture spec (mechanics; §18 counts superseded here) |
| `capability_tree_v1.md` | RON reference; §13 install targets; §14 v0 effect scope |
| `examples/README.md` | InstallTargetSpec RON fixtures |
| `invariants.md` | Non-negotiable code rules |
| `state-authority.md` | Tick vs boundary numeric truth |

### ADRs (`docs/adr/`)

| ADR | Status | Topic |
|-----|--------|-------|
| `pr11_track_a_session_assembly.md` | Accepted | Driver-owned session state, boundary hook |
| `game_mode_session_installation.md` | Accepted (O1 landed) | RON-driven session init |
| `scripted_event_scope_model.md` | Proposed | O4 per-owner scripted events |
| `spec_session_state_replay.md` | Proposed | O2 replay v3 |

### Historical (in repo — rationale only)

| Document | Superseded by |
|----------|---------------|
| `workshop/capability_tree_studio_workshop.md` | Progress log + ADRs + `capability_tree_v1.md` |
| `workshop/tech_tree_decisions.md` | Progress log § approved decisions; crate naming → `simthing-spec` |
| `design_v5.md`, `design_v4.md` | `design_v6.md` |
| `workshop/simthing_modder_object_guide.md` | Draft (local); wait for EffectTarget ADR |

### Archived handoffs (local only)

Superseded PR handoffs live in `docs/workshop/archive/` (gitignored bodies). See
`docs/workshop/archive/SUNSET.md` for the manifest. **Do not implement from archive files.**

---

## 8. Read order for new agents

1. **This document** — parking and gates
2. `docs/adr/pr11_track_a_session_assembly.md`
3. Phase 1 ADRs before O2/O4: `game_mode_session_installation.md`, `scripted_event_scope_model.md`, `spec_session_state_replay.md`
4. `docs/todo.md`
5. `docs/design_v6.md` + `docs/capability_tree_v1.md` (§13–§14)
6. `docs/examples/README.md`
7. Code: `install.rs`, `spec_session.rs`, `boundary/capability_handler.rs`, `boundary/event_handler.rs`

---

## 9. Verification

```powershell
cargo test --workspace
cargo build --workspace --tests
cargo build --workspace --release --tests
cargo test --workspace --release
git status --short --branch
```

Expected: **323** passed, **3** ignored, zero warnings, clean tracked tree.
