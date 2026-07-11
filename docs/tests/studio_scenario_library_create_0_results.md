# STUDIO-SCENARIO-LIBRARY-CREATE-0 Results

## Status

**PROBATION / proof-present / DA-review-pending.** Not graduated and not merged.

## PR / branch / merge

| Field | Value |
|---|---|
| PR | pending publication |
| branch | `codex/studio-scenario-library-create-0` |
| base | `master` |
| merge | NOT MERGED |

## What changed

- Activated the existing Scenario Library Create tab with scenario ID input and a blank-create action.
- Added a neutral one-cell structural authority builder: World root, map Location, one gridcell Location, one neutral Cohort payload, no links, and `blank_minimal` provenance.
- Hydrated the created authority through `StudioSession::from_loaded_scenario`; no parallel session store or generator template.
- Adopted successful creates through the existing Studio adoption/scene rebuild path.
- Requested live-bridge detach on successful authority replacement, retained modal pause, and never restored Play.
- Kept failed creation atomic: status is fail-loud and the prior session remains current.

## Load-bearing proofs

| Proof | Result / catches |
|---|---|
| `cargo check -p simthing-mapeditor` | PASS; create builder, UI action, adoption, and reset request type-check |
| `cargo test -p simthing-mapeditor --test studio_scenario_library_create_0` | PASS, 12/12; loadable session, STEAD/links, authority boundary, save/load identity, failure atomicity, pause/no-autoplay, zero bridge tick, activated affordance, no TP/workshop/gameplay, fail-loud errors |
| `cargo test -p simthing-mapeditor --test studio_scenario_library_ui_0` | PASS, 12/12; 9.5 modal/I/O behavior retained with active Create affordance |
| `cargo test -p simthing-mapeditor --test studio_live_observe_0` | PASS, 10/10 |
| `cargo test -p simthing-mapeditor --test studio_live_session_bridge_0` | PASS, 8/8 |
| `cargo test -p simthing-mapeditor --test studio_sim_clock_ui_0` | PASS, 6/6 |
| `cargo test -p simthing-mapeditor --test studio_sim_clock_0` | PASS, 4/4 |
| `cargo build -p simthing-mapeditor --bin simthing-studio` | PASS |
| `cargo fmt -p simthing-mapeditor -- --check` | BASELINE FAIL in untouched pre-existing files such as `tp_studio_clause_picker_0.rs`; scoped rustfmt for changed implementation/test files PASS |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS; 0 unledgered and 0 stale rows |
| `bash scripts/ci/agent_scan.sh` | `INSPECT(1)`, no reliable failures; 12 distinct load-bearing tests explicitly justified and triaged green |

## Scope Ledger

| Scope | Result |
|---|---|
| Specified | Minimal blank create, loadable session, authority roundtrip, modal pause/no-autoplay, fail-loud atomic replacement |
| Implemented | One-cell structural shell builder + session helper + Create UI + existing adoption/rebuild + bridge reset request |
| Proxied | `StudioSession::from_loaded_scenario` rebuilds document, admission summary, projections, hydration, summary, and view model from authority |
| Deferred | Rich templates/marketplace; live-ops clearance class (9.7); polish/hardening (9.8) |
| Out of scope | Generator templates, driver/session execution APIs, kernel/sim/WGSL, workshop, gameplay/RF, CPU planner, gate/workflow/class changes |

Authority surfaces: driver NO; kernel/sim/WGSL NO; workshop NO; clearance/gate/class NO.

## Conformance

- `SimThingScenarioSpec` remains sole model authority; presentation state is never serialized.
- Created session is loadable and has valid STEAD mapping, links, structural projection, summary, and document.
- Existing scenario I/O saves and reloads the created authority with identity preserved.
- Create failure does not receive or mutate the prior session; UI adopts only the success branch.
- Successful replacement requests bridge detach before the next live bridge update.
- Modal visibility pauses the clock; Create and close never autoplay or execute live ticks.
- No TP fixture/test seed, workshop dependency, generator template, CPU planner, or gameplay semantics.

## Known gaps / next

- Blank creation uses the smallest Studio-admitted legacy World structural shell because canonical Scenario authority requires authored GameSession/Owner/GalaxyMap semantics that are not blank.
- No template selector; the handoff requires one minimal mode and template work remains optional.
- Next rung is `STUDIO-LIVE-OPS-CLASS-0` (9.7, gate-wiring/DA-reserve).

## Graduation routing

PROBATION. Risk class: model-authority-create + modal-pause-control. No forbidden surfaces touched. Medium DA review posture; final clearance sticky controls routing.

## Orientation / anchors

`ORIENT-RECEIPT: a1c8c3f9683d` (role: coding; orientation digest `1de88b7aa142580efd0e18e696db1fddcc40f860883113c5fa2b1b9400c8f35f`).

- `ANCHOR-ACK: movement-front@a0592b2f37ca`
- `ANCHOR-ACK: orientation-harness-core@8a365d1c0864`
- `ANCHOR-ACK: session-lifecycle-adr-family@d73fe5a83f25`
- `ANCHOR-ACK: structural-execution-convergence@17fa0732f44d`
