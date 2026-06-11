# CT-1b implementation results — recalc stress measurement

Status: **IMPLEMENTED / PASS** (2026-06-11, frontier agent).
**Verdict on the rung's question: "every-tick is a net simplification" — CONFIRMED at measured
scale.** No caching/invalidation machinery is warranted: an armed 256-triggered-modifier corpus
costs zero CPU boundary work in steady state, scales sub-linearly in tick cost, and is *cheaper*
per tick than the equivalent corpus of active permanent modifiers.

## Scope ledger

| Specified element | Status |
|---|---|
| Large `triggered_modifier` corpus hydrated through ClauseThing | **implemented** — `clause_corpus(n)` generator, same-scope lowering in `hydrate.rs` |
| → `Suspended` overlay counts | **implemented** — N suspended overlays per N tms, asserted |
| → `Threshold` registration counts | **implemented** — N GPU threshold registrations per N tms, asserted |
| → column counts | **implemented** — 2N properties → exactly 2N×3+3 columns (99/387/1539 at N=16/64/256) |
| Tick cost vs RON baseline | **implemented** — RON corpus is canonically identical (same artifacts ⇒ same cost); marginal-cost comparison run against a permanent-modifier baseline at equal column count |
| Measured report | this document |
| Same-scope only (SCOPE-MEMO §8) | **honored** — `ScopeRef::Current` everywhere; cross-scope rejected by construction |
| Deactivation on potential exit | **deferred** — v1 dialect activates to `Permanent`; `Suspended{when_activated: Transient{…}}` is already authorable at the spec layer and waits for a consumer |

## What was built

1. **Hydration** (`crates/simthing-clausething/src/hydrate.rs`):
   `triggered_modifier { id, potential { property, at_least }, modifier { … } }` →
   one `Suspended { when_activated: Permanent }` standalone `OverlaySpec` + one `EventSpec`
   with a same-scope `TriggerSpec::Threshold` (`Current`, Amount, Rising) and an
   `ActivateOverlayRef` effect referencing the payload by authored id. Multiple `property`
   blocks now hydrate (`HydratedEntityPack.seeds`); unsupported fields stay spanned hard errors
   (CT-1a guard retargeted to `on_action`).
2. **`simthing-spec` widening** (the §6 EffectSpec-widening backlog item, pulled by this rung):
   `EffectSpec::ActivateOverlayRef { target, overlay_ref: String }` — the authorable form of
   activation (runtime `OverlayId` cannot appear in authored files). `compile_effect` on an
   unresolved ref is a hard `SpecError::UnresolvedOverlayRef`.
3. **Driver install resolution** (`crates/simthing-driver/src/install.rs`):
   standalone-overlay install now returns authored-id → installed `OverlayId`s;
   `compile_and_install_event` resolves refs before compilation. Hard errors:
   `UnknownOverlayRef` (dangling), `AmbiguousOverlayRef` (≠1 installed instance — per-owner
   resolution over shared definitions is SPEC-SCOPE-1 territory, rejected not approximated),
   `DuplicateOverlayRefId` (cross-pack collision).

No `simthing-sim`, `simthing-gpu`, or WGSL changes. No new kernel, opcode, or default wiring.

## The consumer ran (real reduction)

`triggered_modifier_fires_and_activates_overlay` (GPU): a hydrated 4-tm corpus with a permanent
driver modifier pushing `stress_0` over its potential threshold — GPU Pass 7 fires the threshold,
the boundary handler emits `ActivateOverlay`, and **exactly** the matching payload overlay flips
`Suspended → Permanent`; the three unfired tms stay `Suspended`. Value readback confirms the
crossing (`stress_0 ≥ 10`).

## Measurement evidence

Host: the standing CT GPU host. 100 ticks per row (`ticks_per_day=10`, 10 boundaries), single
owner (SessionRoot), one slot. "permanent" = identical 2N-property surface with N *active*
permanent modifiers and zero triggers — isolates the marginal cost of the armed-threshold
machinery at equal column count. Timings are **CPU-observed wall-clock phase totals from
`RunSummary`** (`Instant`-based), not GPU timestamp queries — measurement-grade scaling evidence;
no performance-win claim is made or needed by this rung.

Run 1:

| corpus | N | columns | thresholds | suspended | tick total ms | gpu pipeline ms | boundary total ms | µs/tick |
|---|---|---|---|---|---|---|---|---|
| triggered | 16 | 99 | 16 | 16 | 35.7 | 26.1 | 0.000 | 357 |
| permanent | 16 | 99 | 0 | 0 | 31.4 | 22.6 | 0.000 | 314 |
| triggered | 64 | 387 | 64 | 64 | 37.0 | 27.5 | 0.000 | 370 |
| permanent | 64 | 387 | 0 | 0 | 34.7 | 25.6 | 0.000 | 348 |
| triggered | 256 | 1539 | 256 | 256 | 59.1 | 45.9 | 0.000 | 591 |
| permanent | 256 | 1539 | 0 | 0 | 69.5 | 53.3 | 0.000 | 696 |

Run 2 (stability check):

| corpus | N | µs/tick |
|---|---|---|
| triggered | 16 | 490 (first-row warm-up outlier) |
| permanent | 16 | 338 |
| triggered | 64 | 364 |
| permanent | 64 | 354 |
| triggered | 256 | 536 |
| permanent | 256 | 679 |

## Findings

1. **Counts are exactly linear, no amplification.** N tms → N threshold registrations + N
   suspended overlays + 2N properties (stride 3). Nothing hidden multiplies.
2. **Steady-state boundary cost is zero.** Armed, unfired thresholds with no cooldowns take the
   B3 empty-boundary fast path — `boundary_total_ms = 0.000` on every row. The recalc model's
   CPU cost is pay-on-fire only.
3. **Tick cost grows sub-linearly.** 15.5× column growth (99 → 1539) costs ~1.6× tick time
   (≈357 → ≈560 µs/tick); fixed per-tick overhead dominates at these scales.
4. **Armed triggered machinery beats active permanent overlays at scale** (reproducible across
   both runs at N=256: ~536–591 vs ~679–696 µs/tick): suspended overlays are parked (not
   applied), and threshold evaluation is cheaper than overlay transform application. The
   "expensive" corpus is the cheap one.
5. **Verdict:** the assumption holds. Paradox-style cached/invalidated modifier machinery would
   add complexity to avoid a cost that measures as zero (boundary) plus sub-linear (tick).
   The assumption is **confirmed**, not retired.

## Observed substrate behavior (recorded, no change made)

Threshold events are per-tick readback; the session loop consumes `tick.events` only on boundary
ticks, so a crossing fired on a non-boundary tick does not reach the scripted-event handler.
The smoke test uses `ticks_per_day = 1`. Corpora relying on mid-day crossings need tpd=1 or a
future event-latching decision — flagged for the rung that first needs multi-tick days with
scripted events; not this rung's scope.

## Files changed

- `crates/simthing-spec/src/spec/effect.rs` — `ActivateOverlayRef` variant
- `crates/simthing-spec/src/error.rs` — `UnresolvedOverlayRef`
- `crates/simthing-spec/src/compile/effect.rs` — unresolved-ref hard error
- `crates/simthing-driver/src/install.rs` — overlay-ref map + event resolution + 3 error variants
- `crates/simthing-clausething/src/hydrate.rs` — `triggered_modifier` lowering, multi-property
- `crates/simthing-clausething/tests/ct_1b_recalc.rs` — corpus generator, parity, counts, GPU smoke, ignored measurement ladder
- `crates/simthing-clausething/tests/ct_1a_entity.rs` + `fixtures/ct1a_unsupported_field.clause` — guard retargeted
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 ledger row
- `docs/worklog.md`, this report

## Commands run

```text
cargo test -p simthing-clausething                          # all green (incl. 4 CT-1b + GPU smoke)
cargo test -p simthing-clausething --test ct_1b_recalc -- --ignored ct_1b_recalc_stress_measurement --nocapture   # ×2
cargo test -p simthing-spec                                 # green except pre-existing sqrt_promote0_f_artifact_hash_guard (documented since CT-1a, unrelated)
cargo test -p simthing-driver domain_pack_standalone        # pass
cargo test -p simthing-driver --test session_integration    # 19 passed (GPU)
cargo fmt --all -- --check                                  # clean
```

`cargo test --workspace` — **not run**.

## Closure answers

1. **Corpus hydrated through ClauseThing?** Yes — generated ClauseScript text through the real
   parse→expand→hydrate path; canonical-JSON identical to the independently constructed RON
   baseline at N=16 and N=256 (and the baseline RON round-trips).
2. **Suspended/Threshold/column counts measured?** Yes — exactly linear, asserted and tabulated.
3. **Tick cost vs RON baseline?** RON path produces canonically identical artifacts (CT-1a
   instrument, re-proven here at corpus scale) ⇒ identical cost; marginal cost isolated against
   a permanent-modifier baseline instead, which is the informative comparison.
4. **Assumption confirmed or retired?** **Confirmed** — findings 2–4.
5. **Same-scope only?** Yes; cross-scope/per-owner forms are hard errors pointing at SPEC-SCOPE-1.
6. **simthing-sim / GPU / WGSL untouched?** Yes.
7. **New spec surface consumer-named?** Yes — `ActivateOverlayRef` is the §6 EffectSpec-widening
   item pulled by this rung.
8. **Paradox/lab corpus?** None — corpus is generated original ClauseScript.
9. **Artifacts?** This report only; measurement test is `#[ignore]`d, not a permanent battery.
10. **`cargo test --workspace` avoided?** Yes.
