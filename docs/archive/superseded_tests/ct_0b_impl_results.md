# CT-0b-IMPL-0 — Results

**Verdict: PASS.** ClauseThing CT-0b lossless raw model, safe synthetic fixtures, deterministic JSON
goldens, and parse → emit → reparse round-trip tests are implemented in `simthing-clausething` only.

## Scope ledger

| Requirement | Status |
|---|---|
| Lossless raw model (ordered keys, nesting, scalar/block shape) | **Done** — `src/raw.rs`, `src/parse.rs` |
| Deterministic JSON goldens | **Done** — `tests/goldens/*.json` via `src/json.rs` |
| Re-emit raw model to text | **Done** — `src/emit.rs` (+ vendored `begin_mixed_tail` for mixed containers) |
| Parse → JSON golden tests | **Done** — `tests/ct_0b_raw_model.rs` |
| Parse → emit → reparse tests | **Done** — canonical JSON equality |
| Safe synthetic fixtures only | **Done** — `tests/fixtures/*.clause` (SimThing-authored) |
| No CT-0c+ semantics | **Confirmed** |
| No `simthing-sim` / `simthing-gpu` / runtime wiring | **Confirmed** |
| No Paradox/lab corpora committed | **Confirmed** |
| `cargo test --workspace` not run | **Confirmed** |

## Fixtures added

| Fixture | Coverage |
|---|---|
| `duplicate_keys.clause` | Ordered duplicate keys |
| `quoted_scalars.clause` | Quoted vs unquoted scalars |
| `nested_blocks.clause` | Nested object blocks |
| `mixed_siblings.clause` | Scalar/block siblings |
| `repeated_nested.clause` | Repeated nested objects |
| `operators.clause` | `>`, `<=`, `==` operator metadata |
| `header_value.clause` | Header + payload (`rgb { … }`) |
| `mixed_container.clause` | Mixed keyed + array tail |

**Comments:** out of CT-0b scope — jomini text path does not preserve comments; not normalized.

## Commands run

```text
cargo check -p simthing-clausething
cargo test -p simthing-clausething
cargo fmt --all -- --check
```

`cargo test --workspace` was **not** run (parser-isolated rung per `docs/agents.md` discipline).

## Closure answers

1. **Specified vs implemented?** Full CT-0b scope implemented; comments explicitly out of scope.
2. **Ordered duplicate keys?** Yes — `Vec<RawProperty>` insertion order.
3. **Nested block structure?** Yes — `RawValue::Block` / `RawValue::Array`.
4. **Quoted vs unquoted?** Yes — `ScalarForm` from parser token kind.
5. **Operator/header metadata?** Yes — `RawOperator` on properties; `RawValue::Header` for header payloads.
6. **JSON goldens deterministic?** Yes — all eight fixtures pass.
7. **Parse → emit → reparse?** Yes — canonical JSON matches for all fixtures.
8. **Safe synthetic fixtures?** Yes — original SimThing text under `tests/fixtures/`.
9. **Paradox/lab corpus committed?** No.
10. **`simthing-sim` untouched?** Yes.
11. **`simthing-gpu`/WGSL untouched?** Yes.
12. **Runtime/default wiring untouched?** Yes.
13. **Unneeded artifacts deleted?** Yes — no scratch logs or dumps retained.
14. **Retained artifacts under `docs/tests/` only?** Yes — this report only.
15. **`cargo test --workspace` avoided?** Yes.
