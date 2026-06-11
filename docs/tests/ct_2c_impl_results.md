# CT-2c implementation results - economic-category ClauseScript hydration

Status: **IMPLEMENTED / PASS** (2026-06-11, CT-2c-REMEDIAL-2)

RF-BASE-INTRINSIC-0 (PR #592) is consumed. ClauseScript unit-template literal `produces` / `upkeep`
`_add` keys now lower to install-consumed `BaseFlowObligationSpec` entries on `ResourceFlowSpec`.
The normal install/session path seeds admitted participant `IntrinsicFlow` columns; GPU Resource
Flow oracle proof no longer reads `HydratedCategoryEconomyPack.contributions`.

## Remedial closure (post #592)

What remained unfinished after PR #592:
- CT-2c category decoder, Daily Economy parity, bounded admission, and Resource Flow admission shape were landed in PR #590, but literal category `produces` / `upkeep` still lived only in hydration sidecars and tests manually injected those rates into GPU/session state.

How CT-2c now consumes RF-BASE-INTRINSIC-0:
- `hydrate_category_economy.rs` emits deterministic `BaseFlowObligationSpec` rows for each unit-template literal `_add` produce/upkeep key.
- Obligation ids: `{unit_template_id}_{category}_{resource}_{produce|upkeep}`.
- Install target: `InstallTargetSpec::ScenarioListed { target_id: unit_template_id }` (fixture maps `farmer` to the hosted cohort).
- Direction/rate: `BaseFlowDirectionSpec::Produce` / `Upkeep` with non-negative authored rates; signed net flow remains produce minus upkeep.

Where literal rates live after hydration:
- Canonical authoring: `GameModeSpec.resource_flow.base_obligations`.
- Runtime values: participant `IntrinsicFlow` sub-field on admitted arena participants, seeded during install by `seed_base_flow_obligations()` (driver; landed in #592).

## Scope ledger

- `crates/simthing-clausething/src/hydrate_category_economy.rs` — category decoder + `BaseFlowObligationSpec` emission; `contributions` retained as diagnostic-only metadata.
- `crates/simthing-clausething/src/hydrate_resource_flow.rs` — compile fix: empty `base_obligations` default for CT-2a pack (no CT-2a semantic change in this remedial).
- `crates/simthing-clausething/tests/ct_2c_category_economy.rs` — install-consumed obligation proof, GPU oracle without contribution injection, global column mapping for multi-property registry.
- `crates/simthing-clausething/tests/fixtures/ct2c_categories_baseline.ron` — includes `base_obligations` for farmer food produce (+6) and energy upkeep (+1).
- `docs/design_0_0_8_1_clausething_production_track.md` — CT-2c row updated to IMPLEMENTED / PASS.

No `simthing-spec` production widening, no `simthing-sim`, no `simthing-gpu` production or WGSL changes, and no new `AccumulatorRole`.

## Doctrine / closure Q&A

| Question | Answer |
|---|---|
| Canonical RON baseline includes base obligations? | Yes — `farmer_settlement_food_produce` and `farmer_settlement_energy_upkeep` in `ct2c_categories_baseline.ron`. |
| Install/session consumes rates without manual side-channel writes? | Yes — `install_consumes_category_base_obligations_without_manual_side_channel` asserts shadow intrinsic flow = 6.0 after `open_from_spec`. |
| GPU oracle avoids `contributions` injection? | Yes — oracle/GPU test seeds weights only; intrinsic flow comes from install. |
| `contributions` retained? | Yes, diagnostic-only signed-rate mirror for decoder debugging. |
| Category economy compiles away before runtime? | Yes — categories remain hydration/admission metadata only. |
| Resource Flow opt-in/default-off? | Yes — presence alone inactive; `FlatStarOptIn` enables GPU path in fixture only. |
| Bounded participation explicit? | Yes — 3 explicit participants per arena, capped by `max_participants`. |
| Daily Economy ClauseScript parity? | Yes — still matches existing driver RON original. |
| New AccumulatorRole? | No. |
| simthing-spec production widened in this remedial? | No. |
| simthing-sim / simthing-gpu touched? | No production changes. |
| CPU fallback economy logic? | No. |
| Paradox/lab corpus committed? | No. |
| Sqrt/magnitude in this remedial? | No — not applicable; no exact GPU sqrt paths exercised. |
| Full workspace test run? | No — targeted commands only (below). |

## Validation (2026-06-11)

- `cargo fmt --all -- --check` — PASS
- `cargo test -p simthing-clausething --test ct_2c_category_economy` — PASS (10 tests)
- `cargo test -p simthing-clausething` — PASS (56 passed, 5 ignored)
- `cargo test -p simthing-spec --test e10_resource_flow_admission` — PASS (17 tests)
- `cargo test -p simthing-driver --test resource_flow_base_intrinsic` — PASS (3 tests)

Not run: `cargo test --workspace`, lab corpus scans, mobility GPU replay gates, simthing-gpu suite.

Existing warning noise remains in unrelated `simthing-core`, `simthing-driver`, and vendored ClauseThing helper code.
