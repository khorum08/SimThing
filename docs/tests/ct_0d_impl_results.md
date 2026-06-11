# CT-0d-IMPL-0 — Results

**Verdict: PARTIAL / LAB SCAN PENDING (synthetic closure PASS).** Symbolic scope-chain extraction,
validation, safe synthetic goldens, and ignored lab scanner are implemented in
`simthing-clausething` only. Always-on tests pass. `CLAUSER_LAB_DIR` was not set on the
implementation host, so aggregate `scopes.log` frequency evidence was not produced locally.

## Scope ledger

| Requirement | Status |
|---|---|
| Symbolic scope-chain extraction over expanded raw model | **Done** — `src/scope.rs` |
| Runs after CT-0c expansion | **Done** — post-expand fixture + pitfall test |
| `ScopeChain` / `ScopeAtom` / `ScopeReference` / `ScopeExtractionReport` / `ScopeDiagnostic` | **Done** |
| Safe synthetic fixtures + JSON goldens | **Done** — 8 fixtures, 3 goldens |
| Malformed chain diagnostics with spans | **Done** |
| Unknown domain scope not silently accepted | **Done** |
| Ignored lab scanner via `CLAUSER_LAB_DIR` | **Done** — `src/scope_lab.rs`, ignored test |
| Aggregate frequency report | **Pending** — lab dir unavailable locally |
| No `simthing-spec` / `simthing-sim` / GPU / runtime wiring | **Confirmed** |
| `cargo test --workspace` not run | **Confirmed** |

## Files changed

- `crates/simthing-clausething/src/scope.rs` — extraction, parsing, validation
- `crates/simthing-clausething/src/scope_lab.rs` — aggregate lab scanner
- `crates/simthing-clausething/src/scope_json.rs` — deterministic JSON
- `crates/simthing-clausething/src/lib.rs` — exports
- `crates/simthing-clausething/tests/ct_0d_scope.rs` — always-on + ignored tests
- `crates/simthing-clausething/tests/fixtures/scope_*.clause` — 8 synthetic fixtures
- `crates/simthing-clausething/tests/goldens/scope_*.json` — 3 goldens
- `docs/tests/ct_0d_impl_results.md` — this report
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 ledger row

Vendored jomini: **untouched**.

## Scope-chain representation summary

- **`ScopeAtomKind`**: `this`, `root`, `from`/`fromfrom`/… (`repeat`), `prev`/`prevprev`/…,
  `domain`, `event_target`, `unknown` (reserved for later explicit unknown recording).
- **`ScopeChain`**: ordered atoms + preserved `raw_text` + optional `RawSpan`.
- **`ScopeReference`**: `role` (`block_scope_key`, `scalar_path`, `event_target_value`) +
  `context_path` (property key stack, source order).
- **`ScopeExtractionReport`**: ordered `references` + `diagnostics`.
- No runtime slots, no `ScopeRef::Slot`, no hydration structs.

## Extraction rules

1. Traverse expanded [`RawDocument`] deterministically in source order.
2. **Block scope keys**: unquoted identifier keys with block values inside nested blocks
   (not top-level entity/template wrappers).
3. **Scalar paths**: unquoted scalars matching `this`/`root`/`from*`/`prev*`, dot paths, or
   `event_target:` forms; skip `value:` and `@[ ]` scalars.
4. Preserve `raw_text` and token-index spans where available from CT-0b raw scalars.
5. Top-level entity keys (empty `context_path`) are not domain scope transitions.

## Validation rules

- Optional [`ScopeTable`] (synthetic table in always-on tests) validates block scope keys.
- Unknown domain keys → `UnknownDomainScope` diagnostic (not silent acceptance).
- Malformed dot paths (empty segments) → `MalformedChain` diagnostic.

## Diagnostic / rejection rules

| Kind | Trigger |
|---|---|
| `malformed_chain` | Empty dot segment, invalid `event_target:` segment |
| `unknown_domain_scope` | Block scope key absent from validation table |
| `unsupported_form` | Reserved for future explicit unsupported forms |

## Fixtures added

`scope_basic`, `scope_chains`, `scope_event_target`, `scope_malformed`, `scope_unknown_domain`,
`scope_order`, `scope_post_expand_main`, `scope_post_expand_lib` (all SimThing-authored).

## Goldens added

`scope_basic.json`, `scope_chains.json`, `scope_post_expand.json`.

## Commands run

```text
cargo check -p simthing-clausething
cargo test -p simthing-clausething
cargo test -p simthing-clausething --test ct_0d_scope
cargo fmt --all -- --check
```

`cargo test --workspace` — **not run** (parser-isolated rung).

Lab scan — **not run** (`CLAUSER_LAB_DIR` unset on implementation host).

## Lab scanner status

- **Implemented**: `scan_lab_scopes`, ignored test `lab_scopes_log_frequency_scan`.
- **Env var**: `CLAUSER_LAB_DIR` (required for lab run).
- **Local run**: unavailable — env not set.
- **Output policy**: aggregate counts only; no raw lab text committed or printed beyond counts.

To run locally:

```text
CLAUSER_LAB_DIR=<path> cargo test -p simthing-clausething --test ct_0d_scope -- --ignored lab_scopes_log_frequency_scan
```

## Closure answers

1. **Specified vs implemented?** Synthetic scope extraction/validation fully implemented; lab
   frequency aggregate report pending local `CLAUSER_LAB_DIR` run.
2. **Runs after CT-0c expansion?** Yes — `scope_post_expand` golden + `extraction_runs_after_ct_0c_expansion`.
3. **Symbolic representation?** `ScopeChain`, `ScopeAtom`, `ScopeReference`, `ScopeExtractionReport`, `ScopeDiagnostic`.
4. **Recognized atoms/chains?** `this`, `root`, `from*`/`prev*`, domain segments, dot paths, `event_target:`.
5. **Dot paths without runtime slots?** Yes — symbolic atoms only.
6. **`event_target` symbolic?** Yes — dedicated role and atom kind.
7. **Malformed chains rejected?** Yes — deterministic messages + spans.
8. **Spans preserved?** Yes — token indices on scalars/keys where exposed.
9. **Deterministic source order?** Yes — `scope_order` test.
10. **Fixtures + goldens?** Yes — 8 fixtures, 3 goldens.
11. **`scopes.log` lab validation?** Ignored scanner implemented; not run in CI/default tests.
12. **`CLAUSER_LAB_DIR` used?** No — unavailable locally.
13. **Raw Paradox/lab corpus committed?** No.
14. **`simthing-spec` untouched?** Yes.
15. **`simthing-sim` untouched?** Yes.
16. **`simthing-gpu`/WGSL untouched?** Yes.
17. **Runtime/default wiring untouched?** Yes.
18. **Unneeded artifacts deleted?** Yes.
19. **Artifacts under `docs/tests/` only?** Yes — this report only.
20. **`cargo test --workspace` avoided?** Yes.
