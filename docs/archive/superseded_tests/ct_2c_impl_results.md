# CT-2c implementation results — economic-category ClauseScript hydration

Status: **IMPLEMENTED / PASS (CT-2c-REMEDIAL-3, 2026-06-11 — executive design-authority review
and remediation of the Codex implementation; prior REMEDIAL-2 PASS ruled VOID under §0.6)**

## Executive review ruling (design authority, by product delegation)

The REMEDIAL-2 IMPLEMENTED / PASS was **constitutionally void**: silent scope collapse against
the binding §5 rung text with no Deviation Record. Specifically:

1. **The modifier half was dead code.** Hydrated `_mult`/`_add` overlays were emitted into
   `GameModeSpec.overlays`, which the driver install path explicitly defers and never installs —
   no modifier ever affected any flow in any test. The mechanism was also wrong if wired:
   per-tick `Multiply` on a rate column **compounds** every tick, and overlay stacking would be
   multiplicative where the documented semantic is additive-in-effect.
2. **Inheritance asymmetry was absent.** Categories were a flat map; the *required* `depth`
   field was parsed and discarded; no parent relation existed for `_mult` to sweep.
3. **`value:` lowering was absent** (the rung text's first clause).
4. Implementation began before its own design memo's acceptance gate.

The RF floor (decoder with longest-match + ambiguity rejection, Daily Economy authoring parity,
RF-BASE-INTRINSIC-0 install-seeded obligations) was sound and is retained.

## Remediation (REMEDIAL-3)

**Static category modifiers fold into effective obligation rates at hydration** — the
§6 inheritance asymmetry implemented as compile-time semantics:

- `category_map` entries carry `kind`, `depth` (now used), and optional `parent`; the table is
  validated (unknown parents, cycles, and depth inversions are hard errors — **broadcast is
  down-only**). Builtins: `country(1) ← planet(2) ← pop(3)`.
- `_mult` keys **sweep the category ancestor chain**: a `polity_food_produces_mult` applies to
  every descendant category's food production (compile-time fan-out — never a runtime category
  walk; categories still compile away entirely).
- `_add` standalone modifier keys apply **leaf-only** (exact category match, per producer).
- **Stacking is additive-in-effect, exactly:**
  `effective = (base + Σadd) × (1 + Σmult)`, summed in deterministic BTreeMap key order,
  asserted with `to_bits()`. Two mults of +25% and +50% yield ×1.75, never ×1.875.
- The folded effective rate is what `BaseFlowObligationSpec` carries and what
  `seed_base_flow_obligations` seeds — so the proven install/session/GPU path consumes
  modifiers with **zero new runtime machinery**, and a `Multiply`-per-tick compounding bug is
  impossible by construction.
- **Dead modifiers are hard errors** (a fold matching no authored production is rejected), as
  are negative/non-finite effective rates and modifier amounts.
- The dead overlay emission was **ripped out**; `decoded_modifier_keys` and the signed base-rate
  `contributions` mirror remain as authoring-side provenance.

**Driver invariant fix:** `intrinsic_flow_offset` resolved sub-field offsets by enumeration
position; now routes through `PropertyLayout::offset_of` (binding "one home for index
arithmetic" rule — position is only coincidentally correct at width 1).

## Specified vs implemented (Scope Ledger)

| §5 rung clause | Status |
|---|---|
| `value:` amounts | **deferred by Deviation** (below) — literal numerics + CT-0c-folded `@vars` only |
| `economic_category` inheritance → reduction OrderBands | **implemented** — parent-chain category table; `_mult` subtree sweep folded at hydration into the rates the reduction OrderBands consume; aggregates remain ordinary reduced columns |
| `category_map` defaults + hard-error diagnostics | **implemented** — builtins + override + suggested-mapping/unregistered-resource/ambiguity/down-only spanned hard errors |
| generated-key grammar (economic family) | **implemented** — longest-match against registered sets, ambiguity rejection, shipsize-family rejection; lab `modifiers.log` round-trip remains **open** (below) |
| inheritance asymmetry per §6 | **implemented** — `_mult` sweeps, `_add` leaf-only, additive-in-effect stacking, proven bit-exact through install + GPU oracle |
| Daily Economy ClauseScript ≡ RON original | **implemented** — canonical authoring identity (unchanged from REMEDIAL-2) |

## Deviation Record (approved at this review)

1. **`value:` formula trees deferred.** CT-2c accepts literal amounts and CT-0c-folded `@vars`
   only. Full `value: { base add mult }` → `EvalEML` lowering is deferred to
   **CT-RF-EML-RATE-0** (below) — the Daily Economy parity target needs no formulas, and
   speculative EML widening without a consuming fixture is the named anti-pattern.
   Consumer impact: none for the accepted fixtures; formula-bearing content hard-errors with a
   spanned diagnostic naming the ticket.
2. **Static modifiers fold at hydration rather than lowering to runtime overlays.** The §4
   correspondence row ("static modifier → overlay TransformOp") is amended for **rate-typed**
   targets: rates are per-tick flow magnitudes, so install-time folding is exact and a per-tick
   transform on a rate column is wrong (compounding). Runtime overlays remain the lowering for
   **value-typed** targets (CT-1a/CT-1c unchanged). Triggered/gated rate modifiers require
   per-tick recomputation from base columns and are the EML ticket's content.
3. **Trigger-gated produces (conditionals) rejected with a designed path.** Hard error names
   **CT-RF-EML-RATE-0**: a per-tick `EvalEML` effective-rate band —
   `intrinsic_eff = (base + Σadd_gated×gate) × (1 + Σmult_gated×gate)` over explicit base/gate
   columns, ordered before the arena reduce bands — GPU-resident bounded arena arithmetic under
   the product's WGSL/JIT-EML lift for these rungs, handling rising *and* falling edges
   exactly. Named consumer: the CT-3b+4a implementation rung (arena pressure wants gated rates).
   A per-tick `Add` overlay gate was considered and **rejected** (same compounding defect).
4. **Lab `modifiers.log` economic round-trip remains open** (memo §3 evidence demand). The
   decoder's grammar claims stay provisional per the track §6.1 provenance caveat until the
   lab-only ignored scan runs; tracked as the remaining CT-2c follow-up, blocking the grammar
   from being cited as corpus-verified — not blocking this remediation's substrate proof.

## Validation (2026-06-11)

- `cargo test -p simthing-clausething --test ct_2c_category_economy` — **13 passed** (was 10):
  fold-through-install GPU oracle bit parity (folded 10.5 produce disbursed 2.625/7.875 by 1:3
  weights), dead-modifier rejection, negative-effective rejection, down-only depth validation
- `cargo test -p simthing-clausething` — all suites green
- `cargo test -p simthing-driver --test resource_flow_base_intrinsic` — 3 passed (offset_of fix)
- `cargo test -p simthing-spec --test e10_resource_flow_admission` — 17 passed
- `cargo fmt --all -- --check` — clean; `cargo test --workspace` — not run

## Closure answers (delta from REMEDIAL-2)

- Modifiers now provably change installed flows (GPU oracle reads folded rates) — previously
  they changed nothing anywhere.
- Inheritance asymmetry exists and is tested in both directions (sweep applies, leaf-add does
  not sweep, cascade-up is structurally impossible and depth inversions are rejected).
- Mult stacking is additive-in-effect bit-exactly; divergences from unverifiable closed-engine
  corners are pinned and documented per §6.1 (fold order, add-before-mult).
- Conditionals have a designed, ticketed, GPU-resident path instead of a dead-end deferral to a
  rung that does not exist ("CT-2b" appears nowhere in the §5 ladder).
- No `simthing-sim`/`simthing-gpu`/WGSL changes; categories never reach the runtime; opt-in /
  default-off unchanged; no Paradox/lab content committed.
