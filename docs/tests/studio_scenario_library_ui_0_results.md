# STUDIO-SCENARIO-LIBRARY-UI-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Not graduated and not merged.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | [#1289](https://github.com/khorum08/SimThing/pull/1289) |
| branch | `codex/studio-scenario-library-ui-0` |
| base | `master` |
| merge | NOT MERGED |

## What changed

- Added `StudioScenarioLibraryModel` with JSON, ClauseScript, and deferred-Create modes.
- Replaced the inactive top-row Load/Save controls with a primary `Library...` action.
- Added a blocking egui modal showing current session identity and STEAD status.
- Reused existing scenario action flags and `app::scenario_io` handlers for JSON load/save and ClauseScript picker/open.
- Enforced modal pause in both the UI pass and the live bridge update; close does not restore Play.
- Kept runtime candidate save/reopen controls separate from model-authority library I/O.

## Load-bearing proofs

| Proof | Result / catches |
|---|---|
| `cargo check -p simthing-mapeditor` | PASS; production modal and bridge guard type-check |
| `cargo test -p simthing-mapeditor --test studio_scenario_library_ui_0` | PASS, 12/12; modal pause/no-autoplay, production JSON and Clause I/O, explicit resolver, authority-only save, identity/STEAD, deferred Create, non-mutation, no workshop/gameplay dependency, error surface |
| `cargo test -p simthing-mapeditor --test studio_live_observe_0` | PASS, 10/10; observation behavior retained |
| `cargo test -p simthing-mapeditor --test studio_live_session_bridge_0` | PASS, 8/8; live bridge behavior retained |
| `cargo test -p simthing-mapeditor --test studio_sim_clock_ui_0` | PASS, 6/6; transport behavior retained |
| `cargo test -p simthing-mapeditor --test studio_sim_clock_0` | PASS, 4/4; clock substrate retained |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS; Studio binary builds |
| `cargo fmt -p simthing-mapeditor -- --check` | BASELINE FAIL in untouched pre-existing files (for example `tp_studio_clause_picker_0.rs`); scoped rustfmt check for the changed implementation/test files PASS |
| `bash scripts/ci/agent_scan.sh` | `INSPECT(1)`, no reliable failures; `TEST-BUDGET` is explicitly justified and triaged green. There are 12 runtime tests, with the status proof expressed as two mutually exclusive platform-gated source functions |

## Scope Ledger

| Scope | Result |
|---|---|
| Specified | Scenario library modal, current identity/status, JSON load/save, Clause open, modal pause, deferred Create |
| Implemented | Presentation model + egui modal + existing I/O action wiring + bridge-side pause guard + focused proofs |
| Proxied | Native picker UI remains a thin caller over existing injectable production action boundaries |
| Deferred | Blank/template creation (9.6), live-ops clearance class (9.7), hardening/polish battery (9.8) |
| Out of scope | Driver/session API changes, kernel/sim/WGSL, workshop, gameplay/RF attach, CPU planner, gate/workflow/class wiring |

Authority surfaces: driver NO; kernel/sim/WGSL NO; workshop NO; clearance/gate/class NO.

## Conformance

- Studio modal state is presentation-only; `StudioSession.scenario_authority` remains authority.
- JSON save calls existing authority-only save; JSON load calls existing session rebuild path.
- ClauseScript open calls the existing production picker/ingest path with explicit resolver entries.
- Visible modal pauses `StudioSimClockTransport` and produces zero bridge ticks.
- Closing, loading, saving, Clause open, and reopening do not autoplay.
- Modal tab/path interaction does not mutate ScenarioSpec.
- Create is visibly deferred to `STUDIO-SCENARIO-LIBRARY-CREATE-0`.
- No TP defaults, alternate Clause parser, workshop dependency, CPU planner, or gameplay system.

## Known gaps / next

- No blank/template creation; next rung is `STUDIO-SCENARIO-LIBRARY-CREATE-0` (9.6).
- No dedicated live-ops clearance class until 9.7.
- No desktop interaction smoke claimed; headless modal model/action boundaries and Windows Studio build are proven.

## Graduation routing

PROBATION. Clearance sticky on PR #1289: `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` with `DA-TREEVERIFY-PROFILE: DEEP-TREE`. Risk class: presentation + modal-pause-control + authority-adjacent I/O reuse; no driver/kernel/sim/WGSL/workshop surface touched. Clearance and Doctrine Scan are green. Route to DA; coding agent must not merge.

## Orientation / anchors

`ORIENT-RECEIPT: a1c8c3f9683d` (role: coding; regenerated orientation digest `ce03dc8537b6a710040e25ad3fe39e9e5432ca66de2b860b55d0b84738a21af6`).

- `ANCHOR-ACK: movement-front@a0592b2f37ca`
- `ANCHOR-ACK: orientation-harness-core@8a365d1c0864`
- `ANCHOR-ACK: session-lifecycle-adr-family@d73fe5a83f25`
- `ANCHOR-ACK: structural-execution-convergence@17fa0732f44d`
