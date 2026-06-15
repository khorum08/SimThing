# CT-1a implementation results — ClauseThing literal entity hydration parity

Status: **IMPLEMENTED / PASS** (CT-1a-INSTALL-PARITY-CLOSURE-1 closed driver installed-tree parity, 2026-06-10)

## Scope ledger (cumulative)
- `crates/simthing-spec/src/spec/overlay.rs` — `OverlaySpec.install: InstallTargetSpec` with serde
  default `SessionRoot`.
- `crates/simthing-driver/src/install.rs` — domain-pack standalone overlay install wiring +
  unit test `domain_pack_standalone_overlay_installs_on_session_root`.
- `crates/simthing-clausething/src/hydrate.rs` — hydrate `install: SessionRoot` on modifiers.
- `crates/simthing-clausething/tests/ct_1a_entity.rs` — installed-tree parity via
  `preview_install` + canonical `InstalledTreeFingerprint`.
- `crates/simthing-clausething/Cargo.toml` — test-only `simthing-driver` / `simthing-gpu` deps.
- `crates/simthing-spec/tests/pr2_compile.rs` — `OverlaySpec.install` in compile tests.
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 CT-1a → IMPLEMENTED / PASS.

## Files changed (closure)
See scope ledger.

## Standalone domain-pack overlay install semantics
After all domain-pack and game-mode properties register, `compile_and_install` calls
`install_pack_standalone_overlays` for each `DomainPackSpec`:

1. `compile_overlay` admission per `OverlaySpec`.
2. Resolve `OverlaySpec::install` via existing `resolve_install_target`.
3. For each resolved owner: seed target property on owner host (`seed_effect_props_on`).
4. Attach one re-stamped overlay per owner on that host with `affects = [owner_id]`.
5. If overlays were installed and root has no slot yet, `allocator.populate_from_tree(root)`.

Default install target: **`SessionRoot`** (serde default on `OverlaySpec.install`).

## Spec/driver install-path changes
| Layer | Change |
|---|---|
| `OverlaySpec` | Added `install: InstallTargetSpec` defaulting to `SessionRoot` |
| `compile_and_install` | Step 1b: `install_pack_standalone_overlays` after property registration |
| CT-1a proof | `simthing_driver::preview_install` on `GameModeSpec` wrapping hydrated/RON pack |

## Fixture paths
- ClauseScript: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity.clause`
- RON baseline: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity_baseline.ron`

## Authoring canonical equality
**Pass.** Hydrated `DomainPackSpec` canonical JSON matches hand-authored RON baseline.

## Installed-tree parity
**Pass.** `clause_and_ron_installed_trees_match_via_preview_install` exercises
`preview_install` for ClauseScript-hydrated and RON-baseline packs. Canonical
`InstalledTreeFingerprint` equality (overlay ids / SimThing ids normalized to tree paths):

- Registry: `["simthing::potency"]`
- Root node: property `simthing::potency` seeded with registry default
- Root overlay: `Policy` / `Player` / `Permanent`, `Multiply(1.25)` on `Amount` for
  `simthing::potency`, `affects_paths = ["root"]`

`LiteralInstallSnapshot` remains **CPU overlay/property parity only**, not installed-tree proof.

## CPU overlay/property parity
**Pass.** `compile_property` / `compile_overlay` + `apply_to_data`; seeded `40.0` → `50.0`.

## Commands run
- `cargo test -p simthing-clausething --test ct_1a_entity` — pass (4 tests).
- `cargo test -p simthing-clausething` — pass (33 tests + ignored utilities).
- `cargo test -p simthing-driver domain_pack_standalone` — pass (unit test in `install.rs`).
- `cargo test -p simthing-spec --test pr2_compile` — pass (11 tests).
- `cargo test -p simthing-spec` — 1 pre-existing unrelated failure
  (`sqrt_promote0_f_artifact_hash_guard` in JIT artifact suite; not caused by CT-1a).
- `cargo fmt --all -- --check` — clean.
- `cargo test --workspace` — **not run**.

## Closure questions
1. **Standalone overlay semantics?** compile → resolve install → seed property → attach per owner.
2. **OverlaySpec widened?** Yes — `install: InstallTargetSpec` default `SessionRoot`.
3. **Properties before overlays?** Yes — step 1 then step 1b.
4. **Target properties seeded?** Yes — `seed_effect_props_on` on each owner host.
5. **Overlays on tree via preview_install?** Yes.
6. **Both ClauseScript and RON paths exercised?** Yes.
7. **Installed trees canonically identical?** Yes — `InstalledTreeFingerprint` equality.
8. **Canonical authoring equality?** Yes.
9. **CPU overlay/property parity?** Yes.
10. **Unsupported fields hard-error?** Yes.
11. **simthing-spec admission firewall?** Yes — `compile_property` / `compile_overlay`.
12. **Paradox/lab corpus?** No.
13. **simthing-sim untouched?** Yes.
14. **simthing-gpu/WGSL untouched?** Yes (test-only gpu dep for `SlotAllocator`).
15. **Runtime/default schedule wiring?** No new wiring.
16. **Artifacts cleaned?** Yes.
17. **Artifacts under docs/tests / test fixtures?** Yes.
18. **cargo test --workspace avoided?** Yes.
19. **Ledger updated?** Yes — CT-1a IMPLEMENTED / PASS; SCOPE-MEMO unblocked (not started).
20. **SCOPE-MEMO implemented?** No.

## Confirmations
- No Paradox / lab corpus material committed: **confirmed**.
- `simthing-sim` untouched: **confirmed**.
- `simthing-gpu` / WGSL production untouched: **confirmed**.
- No runtime/default schedule wiring: **confirmed**.
- `cargo test --workspace` not run: **confirmed**.
