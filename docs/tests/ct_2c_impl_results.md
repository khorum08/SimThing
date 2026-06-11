# CT-2c implementation results - economic-category ClauseScript hydration

Status: **IMPLEMENTED / PASS** (2026-06-11)

## Scope ledger
- `crates/simthing-clausething/src/hydrate_category_economy.rs` - CT-2c category economy hydration into existing `GameModeSpec`, `PropertySpec`, `OverlaySpec`, `ResourceFlowSpec`, `ArenaSpec`, and `ResourceEconomySpec`.
- `crates/simthing-clausething/src/lib.rs` - exports the CT-2c hydrator, economic-key decoder, and result metadata.
- `crates/simthing-clausething/tests/ct_2c_category_economy.rs` - parity, diagnostics, opt-in posture, bounded admission, and GPU allocation oracle tests.
- `crates/simthing-clausething/tests/fixtures/ct2c_categories.clause` - synthetic category/resource economic-key fixture.
- `crates/simthing-clausething/tests/fixtures/ct2c_categories_baseline.ron` - hand-authored expected `GameModeSpec` baseline.
- `crates/simthing-clausething/tests/fixtures/ct2c_daily_economy.clause` - ClauseScript equivalent of the existing Daily Economy RON fixture.
- `docs/design_0_0_8_1_clausething_production_track.md` - CT-2c ledger row advanced to IMPLEMENTED / PASS.

No `simthing-spec` production widening, no `simthing-sim`, no `simthing-gpu` production or WGSL changes, and no new `AccumulatorRole`.

## Implemented behavior
- Categories are ClauseThing hydration/admission metadata only and compile away before runtime.
- Accepted economic key shape is `(category)_(resource)_(produces|upkeep|cost)_(add|mult)`.
- Key decoding uses longest-match over closed registered category/resource sets and hard-errors unknown, ambiguous, missing-suffix, `shipsize`, and `triggered` forms with spans.
- Continuous `produces` and `upkeep` keys lower to existing Resource Flow authoring. `cost` is rejected in the continuous category path and reserved for discrete `ResourceEconomySpec` authoring.
- Modifier keys lower to existing overlays with `InstallTargetSpec::AllOfKind`, `Named("flow")` targets, and existing transform ops.
- Flow properties preserve the CT-2a triple:
  - `Named("flow")` -> `IntrinsicFlow`
  - `Named("allocated")` -> `AllocatedFlow { arena }`
  - `Named("weight")` -> `AllocatorWeight { arena }`
- Resource Flow stays explicit opt-in/default-off. `ResourceFlowSpec` presence alone remains inactive.

## Fixture parity
- Category fixture canonical JSON matches `ct2c_categories_baseline.ron`.
- Daily Economy ClauseScript hydration canonical-matches the existing original RON fixture at `crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron`.

## Validation
- `cargo fmt --all` - PASS.
- `cargo fmt --all -- --check` - PASS.
- `cargo test -p simthing-clausething --test ct_2c_category_economy` - PASS: 9 passed, 0 failed.
- `cargo test -p simthing-clausething` - PASS: 55 passed, 0 failed, 5 ignored.
- `cargo test -p simthing-spec --test e10_resource_flow_admission` - PASS: 13 passed, 0 failed.
- `cargo test -p simthing-driver daily_economy` - PASS: filtered driver Daily Economy tests passed, including RON admission/compile and surplus/deficit authoring previews.
- `cargo test -p simthing-driver resource_flow` - PASS: filtered driver Resource Flow tests passed, including opt-in/default-off, enrollment, nested-resource-flow rejection, and GPU Resource Flow replay paths.
- CT-2c forbidden-path scan over new hydrator/tests/fixtures found no `sqrt`, `magnitude`, WGSL, `simthing-sim`, or new GPU production path usage. The only `AccumulatorRole` hits are the existing CT-2a flow roles listed above.

Existing warning noise remains in unrelated `simthing-core`, `simthing-driver`, and vendored ClauseThing helper code.

## Not run
- Full workspace tests were not run, per handoff.
- Optional lab-only economic-key corpus scan was not run and no lab/corpus material was committed.
