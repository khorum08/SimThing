# CT-1a implementation results — ClauseThing literal entity hydration parity

Status: **PARTIAL / INSTALL PARITY PENDING** (remedial CT-1a-INSTALL-PARITY-REMEDIAL-0, 2026-06-10)

Hydration, canonical authoring equality, CPU overlay/property parity, and unsupported-field
rejection are **implemented and passing**. Installed-tree bit-exact parity through the existing
driver install path is **not yet proven** and remains the blocking gap before CT-1a can close to
PASS and SCOPE-MEMO can proceed.

## Remedial finding: installed-tree path investigation

**Was an actual existing installed-tree path found for CT-1a's fixture shape?** Partially.

| API | Location | What it installs for `DomainPackSpec` |
|---|---|---|
| `simthing-driver::preview_install` | `crates/simthing-driver/src/install.rs` | Full session install preview (`InstallPreview`: registry, root, allocator, `SpecSessionState`) |
| `simthing-driver::install_atomic` | same | Commits `preview_install` result to caller state |
| `simthing-driver::compile_and_install` | same | In-place worker used by the above |
| `compile_pack_properties` | `install.rs` (private helper) | **Properties only** — iterates `pack.properties` via `compile_property`; **does not process `pack.overlays`** |
| `simthing-spec::compile_overlay` | `crates/simthing-spec/src/compile/overlay.rs` | Admission compile to `Overlay`; doc states caller attaches at runtime — no domain-pack install orchestration |

Global/game-mode overlays are explicitly deferred in `compile_and_install` (ADR
`game_mode_session_installation.md` §4). Capability-tree overlays install only through
`CapabilityTreeBuilder::build` + per-owner clone — not through standalone domain-pack
`modifier` blocks.

**Conclusion:** For the CT-1a demo entity (one `PropertySpec` + one standalone `OverlaySpec`),
the public driver install path registers the property in `DimensionRegistry` but **never attaches
the overlay to any `SimThing` tree node**. There is no test-available helper that produces a
bit-identical installed tree including standalone domain-pack modifiers without driver production
widening.

**Decision needed from design authority:** Wire domain-pack standalone `OverlaySpec` installation
through the existing `compile_and_install` / `install_atomic` path (or accept an alternate
documented install artifact for CT-1a), then re-run installed-tree parity.

## Scope ledger (cumulative + remedial)
- `crates/simthing-clausething/src/hydrate.rs` — literal entity hydration → `DomainPackSpec`.
- `crates/simthing-clausething/src/literal_install.rs` — CPU overlay/property parity only
  (remedial: doc clarified; not installed-tree substitute).
- `crates/simthing-clausething/tests/ct_1a_entity.rs` — remedial: test renamed to
  `clause_and_ron_cpu_overlay_parity_match`.
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 CT-1a row updated to PARTIAL.

## Files changed (this remedial)
- `crates/simthing-clausething/src/literal_install.rs`
- `crates/simthing-clausething/tests/ct_1a_entity.rs`
- `docs/tests/ct_1a_impl_results.md`
- `docs/design_0_0_8_1_clausething_production_track.md`

## Fixture paths
- ClauseScript: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity.clause`
- RON baseline: `crates/simthing-clausething/tests/fixtures/ct1a_demo_entity_baseline.ron`

## Hydration mapping summary (unchanged)
| ClauseScript field | `simthing-spec` target |
|---|---|
| Top-level entity key | `DomainPackSpec.id` |
| `display_name` | `DomainPackSpec.display_name` |
| `property { id, namespace, name, display_name }` | `PropertySpec` |
| `property.seed_amount` | install-time seed (CPU parity proof only) |
| `modifier { id, display_name, targets_property, amount_mult }` | `OverlaySpec` with `TransformOp::Multiply` on `SubFieldRole::Amount` |

## Authoring canonical equality result
**Pass.** Hydrated `DomainPackSpec` canonical JSON matches hand-authored RON baseline
(`hydrated_domain_pack_matches_ron_baseline`).

## Installed-tree parity result
**Pending — not proven.**

- Driver `install_atomic` / `preview_install` were identified but **not exercised** in this
  remedial: they cannot produce overlay-inclusive installed-tree parity for standalone domain-pack
  modifiers without production widening.
- `LiteralInstallSnapshot` is **not** claimed as installed-tree proof.

## CPU overlay/property parity result
**Pass.** `compile_property` / `compile_overlay` admission + `PropertyTransformDelta::apply_to_data`
on Amount column. ClauseScript and RON paths match:
- `property_keys`: `["simthing::potency"]`
- `seeded_amount`: `40.0`, `final_amount`: `50.0` (40 × 1.25)

## Unsupported forms / rejections
Unchanged — `triggered_modifier` and other unknown fields hard-error with spanned `HydrateError`.

## Commands run
- `cargo test -p simthing-clausething --test ct_1a_entity` — pass (3 tests).
- `cargo test -p simthing-clausething` — pass.
- `cargo fmt --all -- --check` — clean.
- `cargo test --workspace` — **not run**.

## Closure questions (remedial)
1. **Actual installed-tree path found?** Public driver install APIs exist, but none install
   standalone domain-pack `OverlaySpec` for the CT-1a fixture without production widening.
2. **Path exercised for ClauseScript and RON?** No — installed-tree path not reachable in scope.
3. **Installed artifacts bit-identical?** **Pending** — not demonstrated.
4. **`LiteralInstallSnapshot` only CPU parity?** **Yes** — explicitly documented; not installed-tree substitute.
5. **Canonical authoring equality?** **Yes** — still passes.
6. **CPU overlay/property parity?** **Yes** — still passes.
7. **Unsupported fields hard-error?** **Yes** — unchanged.
8. **simthing-spec admission firewall?** **Yes** — `compile_property` / `compile_overlay`.
9. **Paradox/lab corpus committed?** **No.**
10. **simthing-sim untouched?** **Yes.**
11. **simthing-gpu/WGSL untouched?** **Yes.**
12. **Runtime/default wiring untouched?** **Yes.**
13. **Unneeded artifacts deleted?** **Yes.**
14. **Artifacts under docs/tests only?** **Yes.**
15. **cargo test --workspace avoided?** **Yes.**
16. **Production ledger updated honestly?** **Yes** — PARTIAL / INSTALL PARITY PENDING.

## Confirmations
- No Paradox / lab corpus material committed: **confirmed**.
- `simthing-sim` untouched: **confirmed**.
- `simthing-gpu` / WGSL untouched: **confirmed**.
- No runtime/default wiring: **confirmed**.
- `cargo test --workspace` not run: **confirmed**.
- SCOPE-MEMO not advanced: **confirmed** (blocked until CT-1a honestly closes).
