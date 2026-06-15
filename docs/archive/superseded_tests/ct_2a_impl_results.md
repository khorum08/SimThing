# CT-2a implementation results — ClauseScript literal produces/upkeep to IntrinsicFlow

Status: **IMPLEMENTED / PASS** (2026-06-10)

## Scope ledger
- `crates/simthing-clausething/src/hydrate_resource_flow.rs` — CT-2a hydration:
  literal `produces`/`upkeep` → `GameModeSpec` with flow-property sub-fields +
  `ResourceFlowSpec` arena admission.
- `crates/simthing-clausething/src/lib.rs` — export `hydrate_resource_flow_pack`,
  `HydratedResourceFlowPack`, `net_intrinsic_flow`.
- `crates/simthing-clausething/tests/fixtures/ct2a_micro_economy.clause` — synthetic
  micro-economy ClauseScript fixture.
- `crates/simthing-clausething/tests/fixtures/ct2a_micro_economy_baseline.ron` —
  hand-authored `GameModeSpec` baseline.
- `crates/simthing-clausething/tests/ct_2a_intrinsic_flow.rs` — authoring parity,
  opt-in posture, bounded arena install, GPU micro-economy vs E-11 oracle.
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 CT-2a → IMPLEMENTED / PASS.

## Files changed
See scope ledger. No `simthing-sim`, `simthing-gpu` production, WGSL, or `simthing-spec`
production widening.

## Named consumer unblocked
**ClauseThing resource-flow proof consumer:** ClauseScript-authored literal continuous
economy rates lower into existing Resource Flow / `IntrinsicFlow` substrate and run as
an opt-in GPU micro-economy fixture without a separate economy engine.

## Fixture paths
- ClauseScript: `crates/simthing-clausething/tests/fixtures/ct2a_micro_economy.clause`
- RON baseline: `crates/simthing-clausething/tests/fixtures/ct2a_micro_economy_baseline.ron`

## Supported ClauseScript dialect (CT-2a)
Top-level entity with exactly:
- `display_name` (optional scalar)
- `flow_property { id namespace name [display_name] [description] }`
- `arena { name flow_property max_participants max_coupling_fanout max_orderband_depth [opt_in] }`
  — `opt_in` = `FlatStarOptIn` or `Disabled` (default `Disabled` when omitted)
- `produces { property = namespace::name rate = <literal f32> }`
- `upkeep { property = namespace::name rate = <literal f32> }`

Rates are exposed on `HydratedResourceFlowPack` as `produces_rate`, `upkeep_rate`;
net root intrinsic for fixtures = `produces_rate - upkeep_rate`.

## Explicitly unsupported (hard-error with spans)
Entity-level: `property`, `modifier`, `triggered_modifier`, `tradition_tree`, `resources`,
`value`, formulas, category maps, dynamic identifiers, scope chains, iterators.
Arena-level: `balance_property`, `enrollment`, `wildcard_admission`, `fission_policy`,
`explicit_participants`, reserved-gap fields, couplings.
Produces/upkeep: any field other than `property` and `rate` (e.g. `value`, `trigger`,
`economic_category`).

## ResourceFlowSpec / IntrinsicFlow structures emitted
- `PropertySpec` with sub-fields:
  - `Named("flow")` → `AccumulatorRole::IntrinsicFlow`
  - `Named("allocated")` → `AccumulatorRole::AllocatedFlow { arena }`
  - `Named("weight")` → `AccumulatorRole::AllocatorWeight { arena }`
- `ResourceFlowSpec` with one `ArenaSpec`:
  - explicit `flow_property` key, caps, `fission_policy: Reject`, empty
    `explicit_participants` (filled at install), `opt_in_mode` from arena block.

## Resource Flow opt-in mechanism
- Authored `arena.opt_in = FlatStarOptIn` sets `ResourceFlowOptInMode::FlatStarOptIn` on
  the hydrated `GameModeSpec`.
- `SimSession::open_from_spec` enables GPU Resource Flow only when opt-in resolves true
  (`resolve_resource_flow_execution`).
- Test `resource_flow_presence_without_opt_in_stays_disabled` mutates opt-in to `Disabled`
  and confirms flags stay off.
- Global pipeline default remains disabled; no runtime wiring added.

## Canonical authoring equality
**Pass.** Hydrated `GameModeSpec` canonical JSON matches hand-authored RON baseline
(`clause_hydrated_game_mode_matches_ron_baseline`).

## Installed arena / participant expansion
**Pass.** `installed_arena_participation_is_explicit_and_bounded`:
- 3 explicit participants (flat-star D=2: 1 root + 2 leaves), bounded by
  `max_participants = 16`.
- One arena scaffold root; no wildcard expansion.

Session open mirrors the flat-star harness: scenario registry is pre-seeded from hydrated
flow property columns (GPU sizing); game-mode `properties` are cleared before install to
avoid duplicate compile — same pattern as `e11_flat_star` + empty `game_mode.properties`.

## GPU micro-economy result
**Pass** (GPU host). `gpu_micro_economy_matches_arena_allocation_oracle`:
- Flat-star D=2, weights `[1.0, 3.0]`, net intrinsic `8.0` (produces 10 − upkeep 2).
- Seeds root intrinsic + leaf weights, runs `run_arena_allocation_oracle`, then GPU
  `run_resource_flow_bands`; leaf allocated-flow columns match oracle at bit level.

## CPU oracle / parity guard
**Reused existing harness:** `simthing_driver::run_arena_allocation_oracle` (E-11 flat-star
CPU oracle) — same guard as `phase_m_frontier_v1_3_gpu_resource_flow.rs` and
`e11_flat_star` support. No parallel economy oracle invented.

## Commands run
- `cargo test -p simthing-clausething --test ct_2a_intrinsic_flow` — pass (5 tests).
- `cargo test -p simthing-clausething` — pass (38 tests + ignored utilities).
- `cargo test -p simthing-spec --test e10_resource_flow_admission` — pass (13 tests).
- `cargo test -p simthing-driver resource_flow` — pass (targeted RF driver tests).
- `cargo fmt --all -- --check` — clean.
- `cargo test --workspace` — **not run**.

## Closure questions
1. **Named consumer?** ClauseThing first resource-flow proof (literal produces/upkeep → IntrinsicFlow).
2. **Dialect supported?** Literal `produces`/`upkeep` + `flow_property` + bounded `arena`; see above.
3. **Unsupported hard-errors?** See list above; exemplar test: `value` in `produces`.
4. **Structures emitted?** `PropertySpec` flow triple + `ResourceFlowSpec` / `ArenaSpec`.
5. **RON baseline match?** Yes — canonical JSON equality.
6. **Opt-in explicit, default-off?** Yes — `Disabled` default; GPU only with `FlatStarOptIn`.
7. **ResourceFlowSpec alone inactive?** Yes — `opt_in_mode: Disabled` leaves flags off.
8. **Bounded explicit participation?** Yes — 3 explicit participants, cap 16.
9. **GPU micro-economy expected values?** Yes — oracle/GPU bit parity on leaf allocations.
10. **Existing oracle reused?** Yes — `run_arena_allocation_oracle`.
11. **New AccumulatorRole variants?** No.
12. **GPU/WGSL touched?** No production changes.
13. **simthing-sim untouched?** Yes — arena-ignorant.
14. **Runtime global enablement untouched?** Yes.
15. **Paradox/lab corpus committed?** No.
16. **Sqrt/magnitude required?** No — CT-2a rung does not use sqrt/magnitude paths.
17. **Superseded artifacts deleted?** Yes — no scratch dumps retained.
18. **Visibility under docs/tests?** Yes — this report only.
19. **cargo test --workspace avoided?** Yes.
20. **Ledger updated honestly?** Yes — CT-2a IMPLEMENTED / PASS; CT-2c not started.

## Confirmations
- No Paradox / lab corpus material committed: **confirmed**.
- `simthing-sim` untouched: **confirmed**.
- `simthing-gpu` / WGSL production untouched: **confirmed**.
- No global Resource Flow default enablement: **confirmed**.
- Exact GPU sqrt rule: **not applicable** to this rung (no sqrt/magnitude).
- `cargo test --workspace` not run: **confirmed**.
