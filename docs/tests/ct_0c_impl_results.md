# CT-0c implementation results — ClauseThing expansion passes + worked plague golden

Status: **IMPLEMENTED / PASS** (frontier-tier implementation per the production track §5 gating).

## Scope ledger
- `crates/simthing-clausething/src/expand.rs` — new expansion module (passes: `@vars` →
  `inline_script` + `$PARAM$` + `[[PARAM]]`/`[[!PARAM]]` → `@[ ]` recognition; `value:` symbolic).
- `crates/simthing-clausething/src/error.rs` — `ExpandError` with optional `RawSpan` diagnostics.
- `crates/simthing-clausething/src/lib.rs` — module wiring and public exports
  (`expand_document`, `ExpansionInput`, `ExpansionOptions`, `is_inline_math`,
  `is_value_reference`, `ExpandError`).
- `crates/simthing-clausething/tests/ct_0c_expansion.rs` — 16 tests + ignored golden regenerator.
- 5 fixtures, 3 goldens (below). No other crate touched.

## Fixtures added (all original SimThing-authored synthetic text)
`expand_plague_main.clause`, `expand_plague_lib_blight_wave.clause`,
`expand_scope_untouched.clause`, `expand_include_order_main.clause`,
`expand_include_order_lib.clause`.

## Golden files added (expanded raw model JSON, not hydration)
`expand_plague_quarantine.json` (worked plague, `[[QUARANTINE]]` branch),
`expand_plague_open.json` (worked plague, `[[!QUARANTINE]]` branch),
`expand_scope_untouched.json` (expansion-order pitfall: scope-like text untouched).

## Expansion-order summary
Implemented and tested in the binding order: (1) document-local `@var` collection
(top-level scalar definitions; doc-local overrides synthetic; definitions stripped) and
whole-token `@name` substitution; (2) recursive structural pass handling, in source order,
`[[PARAM]]`/`[[!PARAM]]` conditional splice-or-drop, `inline_script` inclusion (scalar and
block call forms; call params replace the environment inside the include; includes re-enter
the full pass at depth+1 with cycle detection via include stack plus a hard depth cap), and
`$NAME$` substitution on unquoted scalars (keys and values; quoted scalars never
substituted); (3) `@[ ... ]` inline math receives parameter substitution inside its text and
is otherwise preserved verbatim as a symbolic raw scalar (`is_inline_math` recognizer);
(4) `value:` references untouched (`is_value_reference` recognizer). Scope resolution is not
performed anywhere. Documented deterministic rules: truthiness = defined and ≠ `"no"`;
include splice = library document's top-level properties at the call-site position;
unpaired `$` preserved verbatim; unknown `@name` (including dynamic-identifier forms) stays
symbolic; conditional/include bodies with mixed tails are rejected with diagnostics.

## Commands run
- `cargo test -p simthing-clausething` — pass (smoke 1; CT-0b 3 passed / 1 ignored;
  CT-0c 16 passed / 1 ignored golden writer).
- `cargo test -p simthing-clausething --test ct_0c_expansion -- --ignored
  write_expansion_goldens` — golden generation (developer utility, run once).
- `cargo fmt --all -- --check` — clean.
- `cargo test --workspace` — **not run** (only `simthing-clausething` and docs changed;
  agents.md default rule and the CT-0c handoff both forbid it for this change shape).

## Worked plague expansion evidence
`relay_blight_outbreak` expands with `SEVERITY=3`, `QUARANTINE=$QSTATE$`, root params
`WAVE_SCALE=2`, `QSTATE=yes|no`: include splices `wave_strength=3`, quarantine-branch
`lockdown { tier=3 tier=fallback }` + `throughput_mult=0.5` (or `spread_mult=1.5` in the
open variant), `casualty_estimate=@[ 3 * 12 ]` (substituted, unevaluated),
`echo_rate=0.04` (doc-local `@blight_base_rate`), followed by `decay { rate=0.04
rate=0.01 }` (ordered duplicates), `mortality=value:blight_mortality` (symbolic), and
`surge=@[100*2]`. Both variants are golden-pinned.

## Expansion-order pitfall test evidence
All nine handoff pitfalls have dedicated assertions: scope-like text structurally identical
pre/post expansion (golden-pinned); parameters substitute inside included content;
provided/omitted parameters select `[[PARAM]]`/`[[!PARAM]]` branches respectively
(golden-pinned both ways); include order preserved at the call site
(`before_marker, probe_alpha, probe_beta, after_marker`); mutual recursion rejected with the
include stack named in the diagnostic; depth cap enforced; missing inline target yields the
exact deterministic message; `value:` symbolic; `@[ ]` preserved/unevaluated. Plus: missing
`$PARAM$` is a spanned diagnostic; ordered duplication survives both in-document and through
includes; doc-local variable definitions strip, substitute, and override synthetic ones.

## Unsupported forms / rejections (deterministic diagnostics)
Non-scalar scripted-variable definition; conditional body that is not a block; conditional
or include body with a mixed tail; non-scalar `inline_script` call entry; `inline_script`
block without `script`; missing library target; recursive inclusion; depth-cap excess;
unresolved `$NAME$`. Unknown `@name` forms remain symbolic by design (dynamic identifiers
are later-rung scope).

## Confirmations
- No Paradox / lab corpus material committed: **confirmed** (all fixtures original).
- `simthing-sim` untouched: **confirmed**.
- `simthing-gpu` / WGSL untouched: **confirmed**.
- No runtime/default wiring: **confirmed** (pure library module + tests).
- `value:` left symbolic: **confirmed** (test + golden).
- `@[ ]` not evaluated: **confirmed** (test + golden).
- Vendored jomini untouched: **confirmed**.
- `simthing-spec` untouched: **confirmed**.
- Mobility GPU replay gates not run: **confirmed**.

## Specified-vs-implemented closure
Everything the CT-0c handoff specifies is implemented and tested, with the documented
deterministic rules above standing in where the handoff allowed a choice (truthiness,
splice position, quoted-scalar non-substitution, unpaired `$`). Nothing beyond CT-0c scope
was implemented: no scope extraction, no `scopes.log` validation, no SCOPE-MEMO, no
modifier classification, no `value:` evaluation, no hydration. `cargo test --workspace`
was not run; justification above.

## Closure questions (handoff order)
1. Specified vs implemented: fully implemented as specified; choices documented above.
2. Expansion order `@vars → inline_script + params/conditionals → math recognition →
   value: symbolic`: **yes**, by construction and by pitfall tests.
3. inline_script inclusions structural and compile-time only: **yes**.
4. `$PARAM$` substitution deterministic and tested: **yes**.
5. `[[PARAM]]`/`[[!PARAM]]` tested for present and absent parameters: **yes**, golden-pinned.
6. Included content preserves ordered duplication: **yes** (`tier=3, tier=fallback`).
7. Worked plague expands to deterministic golden output: **yes**, two variants.
8. Expansion-order pitfall tests present: **yes**, all nine.
9. Recursive/missing inline scripts rejected/capped with deterministic diagnostics: **yes**.
10. `value:` left symbolic: **yes**.
11. `@[ ]` preserved/symbolically marked, not evaluated: **yes**.
12. All fixtures safe synthetic SimThing-authored text: **yes**.
13. Paradox/lab corpus committed: **no**.
14. `simthing-sim` untouched: **yes**.
15. `simthing-gpu`/WGSL untouched: **yes**.
16. Runtime/default wiring untouched: **yes**.
17. Unneeded/superseded artifacts deleted: **yes** (probe test deleted before commit; no
    scratch JSON, logs, or console captures retained).
18. Retained artifacts only under `docs/tests/`: **yes** — exactly this report.
19. `cargo test --workspace` avoided: **yes**.
