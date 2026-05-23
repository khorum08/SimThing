# simthing-spec — Unified Progress Log (PRs 1–11 + O1)

**Status:** Canonical **implementation ledger** for `simthing-spec`, PR 11 session
assembly, and O1 session installation.  
**Parking synthesis:** [`docs/design_v6.5.md`](../design_v6.5.md) — read first for HEAD, gates, doc map.  
**Replaces:** superseded PR handoff/workshop docs (see [`archive/SUNSET.md`](archive/SUNSET.md)).  
**Last updated:** 2026-05-23  
**Master HEAD:** `8904522` (O4 per-owner scripted events)  
**Verification:** `cargo test --workspace` → **326** passed, **1** ignored, zero warnings.  
`cargo build --workspace --tests` and release profile build/tests clean.

---

## Current source of truth

| Item | Value |
|------|-------|
| **Crate** | `crates/simthing-spec` — RON → runtime compiler; does not execute simulation |
| **Production deps** | `simthing-core`, `simthing-feeder` only |
| **Session owner** | `simthing-driver::SpecSessionState` + `simthing-driver::install` |
| **Session open** | `SimSession::open_from_spec` (spec-driven) or `open` + manual `install_spec_state` |
| **Boundary wiring** | Generic `BoundaryHookContext` in `simthing-sim`; handlers invoked from driver after GPU readback |
| **ADR** | `docs/adr/pr11_track_a_session_assembly.md` + Phase 1 ADRs (#50) |
| **Design docs** | `docs/design_v6.md` (scripted events addendum), `docs/capability_tree_v1.md` (§12 unlock bridge) |

### Dependency graph (as implemented)

```text
simthing-core
    ↑
simthing-feeder   ← CapabilityUnlockRegistration, CapabilityUnlockEvent,
                    ScriptedEventTriggerRegistration, ScriptedEventTriggerEvent
    ↑         ↑
simthing-spec     simthing-sim   ← ThresholdSemantic arms, extract_* bridges,
(production:      (production)     BoundaryHookContext, external threshold regs
 core + feeder
 only)
    ↑
simthing-driver   ← SpecSessionState, install::compile_and_install,
                    open_from_spec, boundary hook wiring

simthing-studio   ← deferred GUI
```

### Module layout (final)

```text
crates/simthing-spec/src/
  lib.rs, error.rs, diagnostics.rs, keys.rs, metadata.rs, version.rs, validate.rs, ron.rs
  spec/           — RON authoring structs (capability, property, overlay, event, script, …)
  compile/        — property, overlay, capability, trigger, effect, event compilers
  runtime/        — definitions, capability state types, scripted event definitions
  boundary/       — CapabilityTreeBoundaryHandler, ScriptedEventBoundaryHandler
  preview/        — capability effect preview

crates/simthing-driver/src/
  install.rs      — compile_and_install, per-owner tree clone, InstallTargetSpec resolution
  spec_session.rs — SpecSessionState, boundary hook implementation
  session.rs      — SimSession::open, open_from_spec, install_spec_state

crates/simthing-sim/src/
  boundary.rs     — BoundaryHookContext, execute_with_boundary_hook, external threshold storage
  gpu_sync.rs     — full rebuild includes external capability/scripted registrations
  threshold_registry.rs — build_with_*, append_*, extract_*
```

---

## Phase 0 — Architectural pivot (pre-PR 1)

**Commits:** `8ff1308` (crate + doc pivot); exploratory slice PR #45 `470016f`; revert PR #46 `7eb48dc`.

**Outcome:** Renamed workshop to `simthing_spec_workshop.md`. Established **`simthing-spec`** as universal RON→runtime compiler; **`simthing-studio`** deferred as GUI. PR #45 vertical slice (builder + boundary + threshold in one landing) was reverted; work re-sequenced into PRs 1–11.

**Note:** Do not re-land PR #45 as a single slice. Use this progress log for ordering.

---

## PR 1 — Authoring scaffold

**Commit:** `7eb48dc` (merge PR #46)  
**Tests at landing:** 212 workspace (+8 in `simthing-spec`)

### Delivered

- Crate + workspace membership; depends on `simthing-core` only.
- `GameModeSpec`, `DomainPackSpec`, capability RON structs, `PropertySpec` / `OverlaySpec` placeholders.
- Generic `SpecDiagnostics`, `SpecVersion`, `DisplayMeta`, logical keys (`CategoryKey`, etc.).
- RON loaders: `deserialize_game_mode_ron`, `deserialize_capability_tree_ron`.
- Lightweight `validate_capability_tree`.

### Surfaces

| Area | Files |
|------|-------|
| Spec structs | `spec/capability.rs`, `spec/property.rs`, `spec/overlay.rs`, `spec/game_mode.rs`, … |
| Load/validate | `ron.rs`, `validate.rs` |
| Tests | `tests/pr1_spec.rs` (6), unit tests in `validate.rs` (2) |

### Caveats

- No compiler, builder, boundary handler, or sim/feeder integration.

---

## PR 2 — Property + overlay compiler

**Commit:** `da0bf1b`  
**Tests:** `tests/pr2_compile.rs` — 11 passing

### Delivered

- `compile/property.rs`, `compile/overlay.rs`, `compile/context.rs`.
- `PropertySpec` expanded: `description`, `sub_fields` (empty → `PropertyLayout::standard(0)`).
- `OverlaySpec` expanded: `targets_property`, `sub_field_deltas`, `lifecycle`, `kind`, `source`.
- Duplicate-property guard before `DimensionRegistry::register`.
- Errors: `DuplicateProperty`, `UnknownProperty`, `InvalidGovernedByRole`, `InvalidSubFieldRole`, `InvalidPropertyReference`.

---

## PR 3 — CapabilityTreeBuilder

**Commit:** `f1dbfa1`  
**Tests:** `tests/pr3_capability_builder.rs` — 16 passing

### Delivered

- `runtime/capability_definition.rs`: `CapabilityTreeDefinitionId`, `CapabilityTreeDefinition`, `CapabilityDefinition`, `CapabilityPrereq`.
- `compile/capability.rs::CapabilityTreeBuilder::build`.
- One `SimProperty` per category; `ReductionRule::Max` via `reduction_override`.
- Template `SimThing` (`Custom(tree_kind)`); suspended overlays per effect.
- `CapabilityUnlockRegistration` placeholder (moved to feeder in PR 4).
- `ActivationMode::OnPrereqMet` added; rejected as authored default in `validate.rs`.

---

## PR 4 — Capability unlock registration bridge

**Commit:** `aac6d1f`  
**Tests:** feeder (1), `threshold_registry` (4), GPU `capability_unlock_fires_in_boundary_integration_test`

### Delivered

- `CapabilityUnlockRegistration` in `simthing-feeder` (serde).
- `ThresholdSemantic::CapabilityUnlock` in `simthing-sim`.
- `ThresholdBuilder::build_with_capability_unlocks`, private `push_capability_unlocks`.
- Full-rebuild path only; B2 append deferred.

### Caveats

- Integration test proves GPU Pass 7 + semantic resolution only — **not** spec handler or overlay activation (closed in PR 11 Track A E2E).

---

## PR 5 — Capability runtime state + boundary handler

**Commit:** `a0d3501`  
**Tests:** `tests/pr5_capability_handler.rs` — 10 passing

### Delivered

- `runtime/capability_state.rs`: `CapabilityTreeInstance`, `CapabilityTreeState`, notifications, diagnostics.
- `boundary/capability_handler.rs`: `CapabilityTreeBoundaryHandler`.
- Threshold unlock → overlay activation; failed-prereq reset; `OnPrereqMet` fixpoint sweeps; player selection; `Limited(1)` sibling suspension.
- `MaxActivePolicy` with `ReplacementPolicy::SuspendOldest`.

### Caveats

- Handler designed for session coordinator — **not wired to live boundary until PR 11 Track A**.
- Handler API uses owner-keyed maps; driver later bridges multi-tree keys (Track A).

---

## PR 6 — Preview + mutual exclusivity

**Commit:** `7fb1311`  
**Tests:** `tests/pr6_capability_preview.rs` — 5 passing

### Delivered

- `preview/capability_preview.rs`: definition-only CPU preview, per-overlay breakdown, combined net deltas.
- `CapabilityDefinition.effect_transforms` for preview without template SimThing.
- National-ideas activate-switch verification through real structural overlay activation.

---

## PR 7 — Script IR

**Commit:** `991e35d`  
**Tests:** `tests/pr7_script_ir.rs` — 10 passing

### Delivered

- `spec/script.rs`: `ScriptExpr`, `ScriptPredicate`, `PropertyKey`, `ScopeRef`.
- CPU evaluator over `DimensionRegistry + shadow + n_dims`.
- Serde round-trips; hard errors for unknown property/role, bad slots, div-by-zero, invalid clamp.

### Deferred

- EML, parser, full script surface.

---

## PR 8 — Trigger / effect / event compiler

**Commit:** `8a8061c`  
**Tests:** `tests/pr8_event_compiler.rs` — 7 passing

### Delivered

- `TriggerSpec`, `EffectSpec`, `EventSpec` → `CompiledTrigger`, `CompiledEffect`, `ScriptedEventDefinition`.
- Threshold triggers resolve property/role/column; predicate triggers use PR 7 IR.
- Effects compile to `BoundaryRequest` templates (remove / activate / suspend overlay).

### Deferred

- Event runner, GPU threshold upload (PR 10), boundary handler (PR 9).

---

## PR 9 — Scripted event boundary handler (predicate path)

**Commit:** `dc61929`  
**Tests:** `tests/pr9_event_handler.rs` — 8 passing

### Delivered

- `boundary/event_handler.rs`: `ScriptedEventBoundaryHandler`, context, diagnostics.
- Predicate triggers only at this PR; cooldowns and priority ordering.
- `UnresolvedEffectTarget`, `TriggerEvalError` diagnostics.

---

## PR 10 — Scripted event GPU threshold path

**Commit:** `3e4f6ea`  
**Tests:** `tests/pr10_scripted_event_thresholds.rs` — 11 passing (+ feeder serde)

### Delivered

- `simthing_feeder::ScriptedEventTriggerRegistration`, `ScriptedEventTriggerEvent`.
- `ThresholdSemantic::ScriptedEventTrigger`, `build_with_scripted_event_triggers`, `extract_scripted_event_triggers`.
- `ScriptedEventBoundaryHandler::handle_tick` unifies predicate + threshold paths; `UnknownEventId` diagnostic.
- `simthing_core::Direction`: `Copy + PartialEq + Eq`.

---

## Dependency cleanup (between PR 9 and 10)

**Commit:** `07fb2da`

- `simthing_feeder::CapabilityUnlockEvent` — spec consumes resolved events, not raw GPU types.
- Handler renamed to `handle_capability_unlock_events`.
- `ThresholdRegistry::extract_capability_unlocks` bridge in sim.
- **Production:** spec depends on core + feeder only; sim/gpu remain dev-deps for integration tests.

---

## PR 11 Track B — Mechanical prep

**Merge:** PR #47 `392992f`  
**Commits:** `84e03fc`, `f2ed680`, `e8d2980`, `795bc69`  
**Tests:** 306 workspace (+8)

| Task | Deliverable |
|------|-------------|
| B5 | Release profile build/tests verified |
| B2 | `EventKey: From<&str>` / `From<String>` |
| B1 | `Display` for `ScriptedEventDiagnostic*`, `CapabilityTreeDiagnostic` |
| B3 | `ThresholdBuilder::append_capability_unlocks`, `append_scripted_event_triggers` |
| B4 | Addenda in `design_v6.md`, `capability_tree_v1.md` §12 |

---

## PR 11 Track A — Session / driver assembly

**Commit:** `01fb572`  
**Doc:** `9e63718`, `docs/adr/pr11_track_a_session_assembly.md`  
**Tests:** 311 workspace (+5)

### Architecture decisions

- **Session state in `simthing-driver`** — not in `BoundaryProtocol` (sim stays spec-free).
- **Generic boundary hook** after GPU value readback, before lifecycle/expiry/fission/structural mutation.
- **Multi-tree-safe storage:** `CapabilityInstanceKey { owner_id, definition_id, tree_thing_id }`; temporary owner maps for PR 5 handler API.
- **V0 scripted events:** session-global `scripted_current_slot` + `EventKey` cooldowns.

### Delivered

| Area | Surfaces |
|------|----------|
| Driver | `SpecSessionState`, `SimSession::install_spec_state`, `spec_session.rs` unit tests (2) |
| Sim | `BoundaryHookContext`, `execute_with_boundary_hook`, external threshold registration setters |
| GPU sync | Full threshold rebuild includes external capability/scripted registrations |
| Handlers wired | `handle_capability_unlock_events`, `sweep_on_prereq_met`, `handle_tick`, queued player selection |
| E2E | `spec_session_capability_unlock_activates_overlay_for_next_tick` (manual install); `open_from_spec_capability_unlock_activates_overlay_for_next_tick` (**ignored/RED**, O1b) |

### E2E proof (Track A definition of done)

```text
capability progress threshold → GPU Pass 7 → session hook →
CapabilityTreeBoundaryHandler → ActivateOverlay → boundary mutation →
next-tick value change
```

---

## Test inventory (simthing-spec + PR 11 driver)

| Suite | File | Count (approx.) |
|-------|------|-----------------|
| PR 1 | `pr1_spec.rs` + validate unit | 8 |
| PR 2 | `pr2_compile.rs` | 11 |
| PR 3 | `pr3_capability_builder.rs` | 16 |
| PR 5 | `pr5_capability_handler.rs` | 10 |
| PR 6 | `pr6_capability_preview.rs` | 5 |
| PR 7 | `pr7_script_ir.rs` | 10 |
| PR 8 | `pr8_event_compiler.rs` | 7 |
| PR 9 | `pr9_event_handler.rs` | 8 |
| PR 10 | `pr10_scripted_event_thresholds.rs` | 11 |
| Track A driver | `spec_session.rs` unit + `session_integration.rs` E2E | 3+ |

PR 4 tests live in `simthing-feeder` and `simthing-sim/threshold_registry`.

---

## Open work (post O1b / EffectTarget / S5 / O4)

### P0 — Codex

| ID | Owner | Scope | ADR |
|----|-------|-------|-----|
| **O2** | Codex | Replay v3 — `SpecSnapshot`/`SpecDelta`, logical keys, cooldown serialization | [`spec_session_state_replay.md`](../adr/spec_session_state_replay.md) |

### Done (2026-05-23, Opus — `2eff1e0`–`8904522`)

| ID | Commit | Scope |
|----|--------|-------|
| **O1b** | `2eff1e0` | Per-clone overlay ids in handler; property seed on clone; E2E green |
| **EffectTarget** | `8da4be9`, `7febdd1` | ADR Accepted; `Owner` default; `overlay_hosts`; host overlay placement |
| **S5** | `dcc74cc` | Approach C disabled when fission clones capability subtrees; S5 test green |
| **S5 follow-up** | `1253a97` | Fission overlay-id re-stamp; `react_to_fission_clones`; instance + threshold registration |
| **O4** | `8904522` | Per-owner scripted event instances; `EventSpec.install`; ADR Accepted |

### Done (2026-05-23, earlier)

| ID | Scope |
|----|-------|
| **InstallTarget docs** | `capability_tree_v1.md` §13, `docs/examples/` |
| **O1** | Session init — `InstallTargetSpec`, `open_from_spec`, per-owner clone (PR #53) |
| **O3** | `queue_player_selection_by_key` (PR #51) |
| **S3/S4** | Topology guard + capability reverse map (PR #52) |
| **Cursor handoff** | O1b/S5 tests, install examples, kind docs (PRs #56–#59) |

### Deferred / out of scope

- `simthing-studio` GUI
- EML backend, full Clausewitz parser
- Scenario RON expansion (inline tree/registry/shadow seeds)
- Map-scale representation doc spike
- Install clone-then-commit (Studio preview / hot-reload safety)
- B2 tighter incremental topology for fission clone internal edges
- Scripted: `ScopeRef::Owner`, cross-owner events, cross-instance priority ordering
- Event boundary-skip classification (`requires_boundary_tick`)

### Known footguns

- **GPU overlay-prep vs `affects`:** transforms apply via overlay placement on the host
  SimThing tree, not `overlay.affects`. EffectTarget install + `overlay_hosts` drive routing.
- **Partial install mutation** — `compile_and_install` mutates registry/root in place; Studio
  preview needs clone-then-commit.
- **Replay** — structural overlay activations replay; spec runtime state does not (O2).
- **Fission clone registration** — `react_to_fission_clones` synthesizes from source instance;
  exotic install paths need review.
- **Empty-boundary skip** — scripted events may disable skip; classification revisit deferred.
- **O1c ruled out** — dimension sync after install not the blocker.

---

## Approved design decisions (retained from workshop)

These remain valid; see original Q&A in `capability_tree_studio_workshop.md` for rationale.

| ID | Decision |
|----|----------|
| D0 | `simthing-spec` is RON→runtime compiler; sim never sees domain progression concepts |
| D1 | `simthing-studio` deferred; depends on spec |
| D11 | `CapabilityUnlockRegistration` lives in `simthing-feeder` |
| — | Suspended overlays become `Permanent` on capability unlock (PR 5 handler) |
| — | `ActivationMode`: `Threshold`, `PlayerSelection`; `OnPrereqMet` runtime-only |
| — | Category identity = `namespace::name` (`CategoryKey`), not separate category id field |
| — | PR 11: driver owns session state; sim uses generic hook |

---

## Read order for new agents

1. **`docs/design_v6.5.md`** — parking synthesis
2. **This document** — PR archaeology
3. `docs/adr/pr11_track_a_session_assembly.md`
4. **Phase 1 ADRs** (before O2/O4 implementation):
   - `docs/adr/game_mode_session_installation.md` (O1 — landed PR #53)
   - `docs/adr/scripted_event_scope_model.md`
   - `docs/adr/spec_session_state_replay.md`
5. `docs/todo.md` (parking state)
6. `docs/design_v6.md` + `docs/capability_tree_v1.md` addenda (§13 install targets, §14 v0 effect scope)
7. `docs/examples/README.md` — InstallTargetSpec RON examples
8. Code: `install.rs`, `spec_session.rs`, `session.rs`, `boundary/capability_handler.rs`, `boundary/event_handler.rs`

**Ignore for implementation:** archived handoffs in `docs/workshop/archive/` — see [`archive/SUNSET.md`](archive/SUNSET.md).

---

## Verification

```powershell
cargo test --workspace
cargo build --workspace --tests
cargo build --workspace --release --tests
cargo test --workspace --release
git status --short --branch
```

Expected: **326** passed, **1** ignored, zero warnings, clean tracked tree.
