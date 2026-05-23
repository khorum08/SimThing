# simthing-spec — Sonnet / Opus Handoff

**Date:** 2026-05-23  
**Master HEAD:** `b93c1b3` (workshop authoring docs @ `afcbd53`)  
**Verification:** `cargo test --workspace` → **326** passed, **1** ignored, zero warnings  
**Parking synthesis:** [`docs/design_v6.5.md`](../design_v6.5.md)

---

## 1. Executive summary

The **simthing-spec ladder (PRs 1–11) is complete**, along with post-PR11 work:

| ID | Status | Commit(s) |
|----|--------|-----------|
| O1 session install | ✅ | PR #53 |
| O1b per-clone overlay activation | ✅ | `2eff1e0` |
| EffectTarget ADR + implementation | ✅ | `8da4be9`, `7febdd1` |
| S5 Approach C disable + fission clone hooks | ✅ | `dcc74cc`, `1253a97` |
| O4 per-owner scripted events | ✅ | `8904522` |

**One P0 code item remains:** **O2 — Replay v3** for spec runtime state.

Everything else is deferred design, mechanical follow-up, workshop/docs, or tabled product work (`simthing-studio`, scenario RON expansion).

### Agent split (this handoff)

| Agent | Sweet spot | Do **not** |
|-------|------------|------------|
| **Opus** | ADRs, cross-crate architecture, vertical features touching spec + driver + replay format | Large mechanical test-only PRs better suited to Sonnet |
| **Sonnet / Composer** | Mechanical prep, tests, examples, doc alignment, small scoped PRs after ADR lands | Redesign ADR decisions or sim-crate semantics without Opus review |

Historical precedent: Opus landed PR 11 Track A, EffectTarget, O1b–O4. Composer/Cursor landed Track B (append helpers, Display), S3/S4 guards, and regression tests.

---

## 2. Current architecture (spec layer)

```text
simthing-core
    ↑
simthing-feeder   ← registrations/events (capability + scripted)
    ↑         ↑
simthing-spec     simthing-sim   ← spec-free; BoundaryHookContext
(production:      (production)
 core + feeder)
    ↑
simthing-driver   ← SpecSessionState, install, open_from_spec,
                    react_to_fission_clones, boundary hook wiring
```

**Session open:** `SimSession::open_from_spec` → `install::compile_and_install`  
**Capability:** per-owner clone, `instance.by_overlay`, `overlay_hosts`, `EffectTarget::Owner` default  
**Scripted events:** one `ScriptedEventDefinition` + N `ScriptedEventInstance` per `EventSpec.install`  
**Fission clones:** `react_to_fission_clones` registers capability instances + thresholds for spawned subtrees

**Read first:** `design_v6.5.md` · `worklog.md` (O1b–O4 entries) · `simthing_spec_progress_log.md`

---

## 3. P0 — O2 Replay v3

**ADR:** [`docs/adr/spec_session_state_replay.md`](../adr/spec_session_state_replay.md) — **Proposed** (needs Accept + implementation)  
**Blocks:** durable replay of capability runtime, scripted cooldowns, player selections, notifications

### Problem

Replay v2 reconstructs tree + registry + structural boundary deltas. Spec runtime is **empty** on replay open:

- Capability activation modes, active sets, mutual-exclusivity state
- Per-instance scripted cooldowns and slots
- Queued player selections
- Capability notifications stream

Raw `OverlayId` is **not stable across processes** — replay must use logical keys (`CapabilityEntryKey`, `EventKey`, `tree_id`, `owner_id`).

### Opus owns

1. **Accept the ADR** (or amend with implementation discoveries).
2. **Core implementation** — suggested slice order:
   - `crates/simthing-driver/src/spec_replay.rs` — `SpecSnapshot`, `SpecDelta`, `apply_spec_snapshot`, `apply_spec_delta`
   - Extend LDJSON replay stream: `spec_snapshot` line + per-frame `spec_entries`
   - Hook `SimSession::record_to_path` to emit spec snapshot + per-boundary diffs
   - Hook replay open: `open_replay_with_spec` (re-run `open_from_spec`, then apply snapshot + frame deltas)
   - Logical-id resolution: `definition_logical_id` → live `CapabilityTreeDefinitionId`; never serialize raw `OverlayId` for spec state
3. **Overlay activation replay note:** structural `OverlayActivated` deltas may still name template overlay ids — verify interaction with per-clone `overlay_hosts` during replay apply; fix if replay-open unlock path breaks.
4. **Acceptance:** replay round-trip preserves capability active state + scripted cooldowns across process restart (design one E2E, Sonnet can land the test file once Opus defines the API).

### Sonnet / Composer owns (after Opus lands core)

1. **Integration tests** in `simthing-driver/tests/`:
   - Record session with capability unlock + scripted event fire → reopen replay → assert spec state matches
   - Back-compat: old replay files without `spec_snapshot` parse cleanly
2. **Mechanical serde:** `#[serde(default, skip_serializing_if = "Vec::is_empty")]` on new replay fields
3. **Doc sync:** `design_v6.5.md`, progress log, ADR status → Accepted, `todo.md` parking
4. **Optional:** `docs/examples/` replay fixture + README snippet

### Verification (Opus + Sonnet)

```powershell
cargo test --workspace
cargo test -p simthing-driver replay
cargo test -p simthing-driver spec_replay
```

Target: **326+** passed, **1** ignored (GPU perf bench unchanged).

---

## 4. Deferred spec-layer work (post-O2)

### 4.1 Opus — design / ADR first

| ID | Topic | Why Opus | Inputs |
|----|-------|----------|--------|
| **E0** | Base economic system ADR | New overlay semantics (transfer), threshold completion, queue/fission patterns | [`simthing_base_economic_system_working_doc.md`](simthing_base_economic_system_working_doc.md) § Recommended Implementation Ladder A0 |
| **S6** | `ScopeRef::Owner` + scripted scope extensions | Changes Script IR + event handler contract | O4 ADR § Out of scope |
| **I1** | Install clone-then-commit | Studio preview / hot-reload safety | Footgun in `design_v6.5.md` §6 |
| **B3** | Empty-boundary skip / `requires_boundary_tick` classification | Event vs capability boundary-skip policy | O4 landed; skip behavior still coarse |

**Do not implement E1–E7 code until E0 ADR is Accepted.**

### 4.2 Sonnet / Composer — mechanical / tests / docs

| ID | Topic | Depends on | Scope |
|----|-------|------------|-------|
| **B2′** | Append-only external thresholds for fission clones | S5 follow-up landed instance reg; append path still deferred | Wire `append_capability_unlocks` / `append_scripted_event_triggers` on fission clone path where eligible; regression test |
| **D1** | Modder guide ↔ engine alignment | EffectTarget landed | Update `simthing_modder_object_guide.md`: `effect_target`, `EventSpec.install`, `overlay_hosts` mental model; cross-link §14 |
| **D2** | `docs/examples/` expansion | O4 + EffectTarget | RON fixtures: Owner-targeted effect, per-faction scripted event, `CapabilityTree`-opt-in effect |
| **D3** | `capability_tree_v1.md` / preview docs | EffectTarget | Ensure preview `owner_slot`/`root_slot` documented with worked example |
| **P1** | Preview / diagnostic polish | — | `Display` gaps, preview breakdown labels (Track B style mechanical PRs) |
| **T1** | Parse/load smoke tests | D2 | `loads_*_examples` in `simthing-spec/tests/` |

### 4.3 Base economy ladder (after E0 ADR)

| Step | Owner | Deliverable |
|------|-------|---------------|
| A0 | **Opus** | ADR: resource balances, transfer overlays, threshold completion, column discipline |
| A1 | Sonnet | Resource tagging in spec (`PropertySpec` convention or kind metadata) |
| A2 | **Opus** | Transfer overlay spec shape + compiler (new semantic — not a one-liner) |
| A3 | **Opus** | Direct transfer runtime in driver/session |
| A4–A7 | Opus + Sonnet | Fixtures (Sonnet tests, Opus runtime if complex) |
| A8 | Sonnet | Modder guide examples + `docs/examples/` |

---

## 5. Tabled (do not start without explicit ask)

- `simthing-studio` GUI
- Full scenario RON expansion (inline tree/registry/shadow seeds)
- Map-scale representation doc spike
- EML backend / Clausewitz parser
- B2 tighter incremental Approach C for fission clone **internal** edges (perf; S5 conservative fix is sufficient for correctness)
- Cross-owner scripted events, cross-instance priority ordering (O4 ADR deferred)

---

## 6. Footguns (both agents — read before coding)

1. **GPU overlay-prep ignores `overlay.affects`** — EffectTarget install **places** overlays on the host SimThing; `overlay_hosts` drives boundary activate/suspend targets.
2. **Partial install mutation** — `compile_and_install` mutates registry/root in place; Studio needs clone-then-commit (I1).
3. **`react_to_fission_clones`** synthesizes from source instance — exotic custom install paths need explicit review when extending.
4. **Replay + OverlayId** — never serialize raw overlay ids for spec state (ADR M1); use logical keys.
5. **O1c ruled out** — dimension sync after install is not the blocker.
6. **326 / 1 ignored** — only `pipeline_timing_1000_slots_64_dims` in `simthing-gpu`; do not regress.

---

## 7. Key files

| Area | Path |
|------|------|
| Install | `crates/simthing-driver/src/install.rs` |
| Spec session state | `crates/simthing-driver/src/spec_session.rs` |
| Session + fission hook | `crates/simthing-driver/src/session.rs` |
| Capability handler | `crates/simthing-spec/src/boundary/capability_handler.rs` |
| Event handler | `crates/simthing-spec/src/boundary/event_handler.rs` |
| Capability builder | `crates/simthing-spec/src/compile/capability.rs` |
| Replay v2 (sim) | `crates/simthing-sim/src/replay.rs` |
| O2 target (new) | `crates/simthing-driver/src/spec_replay.rs` |
| Driver E2E tests | `crates/simthing-driver/tests/session_integration.rs` |

---

## 8. Recommended pickup order

### Opus

1. Read `spec_session_state_replay.md` + `spec_session.rs` field inventory
2. Accept ADR → implement O2 core (`spec_replay.rs`, record/open hooks)
3. Land one replay round-trip E2E (or define API for Sonnet test)
4. **Then** E0 base economy ADR (if product priority) or I1 install clone-then-commit

### Sonnet / Composer

1. Read `design_v6.5.md` + this handoff
2. **Wait for O2 API** → replay integration tests + doc sync
3. Parallel safe work: **D2** examples, **D1** modder guide EffectTarget/O4 sections, **T1** parse smoke tests
4. After E0 ADR: A1, A8, mechanical tests for A4–A7 fixtures

---

## 9. PR discipline

- **One concern per PR** — Opus: vertical slices; Sonnet: tests/docs/examples
- **Do not modify `simthing-sim` production semantics** for spec-layer work unless ADR explicitly requires it (O2 replay format stays driver/spec-owned)
- **Keep `simthing-spec` production deps** at `simthing-core` + `simthing-feeder` only
- Run full workspace tests before merge; release profile smoke if touching driver replay path

---

## 10. One-line pickup

**Opus:** Accept and implement **O2 replay v3** (`SpecSnapshot`/`SpecDelta`, logical keys, driver record/open).  
**Sonnet:** After O2 core lands — replay round-trip tests, examples, modder-guide alignment; parallel doc/example PRs safe now.
