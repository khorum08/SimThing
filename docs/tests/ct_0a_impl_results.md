# CT-0a-IMPL-0 — Results

**Verdict: PASS (CT-0a scope).** `simthing-clausething` crate skeleton, vendored jomini text path,
MIT license accounting, and safe synthetic parser smoke test are in place. ClauseThing remains
default-off and isolated from `simthing-sim`, `simthing-gpu`, and driver/runtime wiring.

## Scope ledger

| Requirement | Status |
|---|---|
| Create `crates/simthing-clausething/` | **Done** |
| Wire crate into root workspace `Cargo.toml` | **Done** |
| Vendor jomini text path under `src/jomini/` | **Done** (binary save/envelope/melting/serde-de/TokenReader excluded) |
| Include MIT license at `src/jomini/LICENSE` | **Done** |
| Per-file vendored provenance headers | **Done** (all `.rs` files; `MODIFIED:` on trimmed modules) |
| Root `THIRD_PARTY_LICENSES.md` | **Done** |
| Safe synthetic ClauseScript smoke test (original text) | **Done** — `tests/ct_0a_synthetic_smoke.rs` |
| CT-0a implementation report (this file) | **Done** |
| Update CT production track status ledger | **Done** |
| No CT-0b+ semantics | **Confirmed** — no raw model, macros, scopes, hydration |
| No `simthing-sim` / `simthing-gpu` / runtime default wiring | **Confirmed** |
| No Paradox/lab corpora committed | **Confirmed** |

## Files changed

**New**

- `crates/simthing-clausething/Cargo.toml`
- `crates/simthing-clausething/src/lib.rs`
- `crates/simthing-clausething/src/jomini/**` (vendored text-path subset + `LICENSE`)
- `crates/simthing-clausething/tests/ct_0a_synthetic_smoke.rs`
- `THIRD_PARTY_LICENSES.md`
- `docs/tests/ct_0a_impl_results.md`

**Modified**

- `Cargo.toml` (workspace member)
- `Cargo.lock` (workspace lock refresh)
- `docs/design_0_0_8_1_clausething_production_track.md` (CT-0a ledger row)

**Explicitly untouched:** `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec` (no admission/hydration changes).

## Vendored jomini source

| Field | Value |
|---|---|
| Upstream | https://github.com/rakaly/jomini |
| Version tag | v0.34.1 |
| Commit | `fff00d8c7f8f06c084d776d1a2c98b34324e64ed` |
| Lab source path (not committed) | `C:\Users\mvorm\Clauser\jomini` |
| License | MIT (`crates/simthing-clausething/src/jomini/LICENSE`) |

**Vendored modules (text path):** `text/{tape,dom,operator,fnv,writer}`, `scalar`, `encoding`, `data`, `errors` (trimmed), `util`, `common/date`, `binary/rgb` (writer stub only).

**Excluded:** binary lexer/parser, envelope, melting, `text/de`, `text/reader`, serde derive integration, upstream integration tests/doctests (`doctest = false` on crate lib).

## Commands run

```powershell
cargo check -p simthing-clausething
cargo test -p simthing-clausething
cargo fmt --all
cargo fmt --all -- --check
cargo test --workspace
```

**Clippy:** not run — no active repo/CI clippy gate found for agent instructions.

### Test results

| Command | Result |
|---|---|
| `cargo test -p simthing-clausething` | **PASS** (1 integration test) |
| `cargo fmt --all -- --check` | **PASS** (after `cargo fmt --all`) |
| `cargo test --workspace` | **1 failure** — `simthing-driver` test `frontier_v1_4_route_replay_reproducibility` (GPU route-replay fingerprint `a9e397321e4f8e9b` vs golden `4382ec7ef93c9174`). Unrelated to CT-0a: `simthing-clausething` is not a workspace dependency of `simthing-driver`; failure is environmental GPU fingerprint drift on this host. All other workspace crates/tests completed in the same run. |

## License accounting confirmation

- Verbatim MIT text at `crates/simthing-clausething/src/jomini/LICENSE`.
- Per-file headers on all vendored `.rs` files cite upstream repo, version/commit, license path.
- `THIRD_PARTY_LICENSES.md` records jomini with scope note (text path only).

## Safety / non-goal confirmations

- **No Paradox / Stellaris / Clausewitz source material committed.** Smoke fixture is original SimThing-authored text.
- **No runtime or default-on wiring.** Crate is workspace-only; no driver/spec/sim dependency edges added.
- **`simthing-sim` untouched.**
- **`simthing-gpu` untouched.**
- **No WGSL / GPU primitive / `AccumulatorRole`.**
- **No ClauseScript hydration to `simthing-spec`.**
- **No CT-0b drift** (no raw model, JSON goldens, macro/`@vars`/`inline_script`/scope-chain work).

## Scratch cleanup

No temporary vendor extraction folders, parser dumps, lab corpora, or `target/` artifacts committed.

## specified vs implemented?

**Yes — match.** Implemented work is exactly CT-0a: isolated `simthing-clausething` crate skeleton, vendored/accounted jomini **text parsing path**, synthetic parser smoke test, license accounting, and production-track ledger update. No CT-0a requirement was proxied, deferred, or silently parked. Non-goals were not violated.
