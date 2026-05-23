# Game mode install target examples

Modder- and developer-facing **v0** examples for `InstallTargetSpec` on
`CapabilityTreeSpec.install`. Each file is a complete, parseable `GameModeSpec`
RON document.

**Related:** [`docs/adr/game_mode_session_installation.md`](../adr/game_mode_session_installation.md)

## Files

| File | Install target | Use when |
|------|----------------|----------|
| [`game_mode_install_all_factions.ron`](game_mode_install_all_factions.ron) | `AllOfKind { kind: "Faction" }` | Every faction in the scenario tree gets its own cloned tree |
| [`game_mode_install_scenario_listed.ron`](game_mode_install_scenario_listed.ron) | `ScenarioListed { target_id: "player_faction" }` | Only explicit owners listed by the scenario |
| [`game_mode_install_session_root.ron`](game_mode_install_session_root.ron) | `SessionRoot` | One tree attached directly under the session root (`Scenario::root`) |

Parse smoke check (from repo root):

```powershell
cargo test -p simthing-spec loads_install_target_examples
```

## `AllOfKind`

```ron
install: AllOfKind(kind: "Faction"),
```

At install time, `simthing-driver::install::resolve_install_target` walks the
live scenario root and collects every `SimThingId` whose kind **exactly**
matches the authored string via `simthing_core::kind_matches`.

- Built-in kinds use enum identifier spelling: `"World"`, `"Faction"`,
  `"StarSystem"`, `"Location"`, `"Cohort"`, `"Fleet"`, `"Station"`.
- Custom kinds match their authored label, e.g. `"national_ideas"` on
  `SimThingKind::Custom("national_ideas")`.
- Matching is **case-sensitive** and **exact** in v0 (no tags, no fuzzy match).
- If no nodes match, install fails with
  `InstallError::NoMatchingOwners { tree_id, target }`.

Omitting `install` on a capability tree defaults to `AllOfKind { kind: "Faction" }`.

## `ScenarioListed`

```ron
install: ScenarioListed(target_id: "player_faction"),
```

The scenario (not the game mode RON) must define which owners receive the tree:

```rust
scenario
    .install_targets
    .insert("player_faction".into(), vec![player_faction_id]);
```

- The `target_id` string must exist as a key in `Scenario::install_targets`.
  A missing key is `InstallError::UnknownInstallTarget`.
- The value is a `Vec<SimThingId>`; v0 installs one cloned tree per listed id.
- An empty owner list resolves to zero owners → `NoMatchingOwners`.

This pattern is for campaign setups where the scenario author names the player
faction (or a small explicit set) without encoding SimThing ids in the game mode
file.

## `SessionRoot`

```ron
install: SessionRoot,
```

Installs **once** on `Scenario::root.id`. The cloned capability tree becomes a
direct child of the session root `SimThing` (typically the world node).

v0 scripted events also install at session-global scope using the root slot;
capability trees may use `SessionRoot` when the authored content is
world-level rather than per-faction.

## Runtime entry point

```rust
let game_mode = simthing_spec::deserialize_game_mode_ron(&ron_text)?;
let session = SimSession::open_from_spec(scenario, &game_mode)?;
```

See `crates/simthing-driver/tests/session_integration.rs` (O1 install tests) for
GPU integration coverage of `AllOfKind` and `ScenarioListed`.

## Out of scope (v0)

Do not expect tag selectors, owner expressions, dynamic filters, or scenario RON
expansion of `install_targets` — those are future design work.
