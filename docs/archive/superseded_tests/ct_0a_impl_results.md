# CT-0a-IMPL-0 + CT-0a-REMEDIAL-VERIFY-0 — Results

**Verdict: PASS (CT-0a scope, remedial closure).** `simthing-clausething` crate skeleton, vendored jomini
text path, corrected MIT license accounting, and safe synthetic parser smoke test are in place.
ClauseThing remains default-off and isolated from `simthing-sim`, `simthing-gpu`, and driver/runtime
wiring.

**Remedial note (2026-06-10):** license copyright line restored; workspace GPU fingerprint failure
verified pre-existing and isolated from CT-0a by dependency and file-change evidence.

## Scope ledger

| Requirement | Status |
|---|---|
| Create `crates/simthing-clausething/` | **Done** |
| Wire crate into root workspace `Cargo.toml` | **Done** |
| Vendor jomini text path under `src/jomini/` | **Done** (binary save/envelope/melting/serde-de/TokenReader excluded) |
| Include MIT license at `src/jomini/LICENSE` with upstream copyright notice | **Done (remedial)** |
| Per-file vendored provenance headers | **Done** (all `.rs` files; `MODIFIED:` on trimmed modules) |
| Root `THIRD_PARTY_LICENSES.md` | **Done** |
| Safe synthetic ClauseScript smoke test (original text) | **Done** — `tests/ct_0a_synthetic_smoke.rs` |
| CT-0a implementation report (this file) | **Done** |
| Update CT production track status ledger | **Done** |
| No CT-0b+ semantics | **Confirmed** |
| No `simthing-sim` / `simthing-gpu` / runtime default wiring | **Confirmed** |
| No Paradox/lab corpora committed | **Confirmed** |

## Remedial: license notice correction

**Defect (PR #571):** committed `LICENSE` contained MIT permission/warranty text only — no copyright line.

**Upstream verification (pinned commit `fff00d8c7f8f06c084d776d1a2c98b34324e64ed`):**

- `LICENSE.txt` at lab path `C:\Users\mvorm\Clauser\jomini\LICENSE.txt` — permission text only (no copyright line).
- Same at `https://raw.githubusercontent.com/rakaly/jomini/fff00d8c7f8f06c084d776d1a2c98b34324e64ed/LICENSE.txt`.
- Upstream `Cargo.toml` at pinned commit: `authors = ["Nick Babcock <nbabcock19@hotmail.com>"]`.
- Repository inception: `2020-07-27` (`git log --reverse` on lab clone).

**Remedial fix:** `crates/simthing-clausething/src/jomini/LICENSE` now begins with `Copyright (c) Nick Babcock`
followed by the verbatim MIT permission/warranty text from upstream `LICENSE.txt`. `THIRD_PARTY_LICENSES.md`
records the copyright holder and notes upstream's omitted copyright line.

## Remedial: workspace GPU fingerprint isolation

### PR #571 file-change boundary

`git diff 6937858..bb44f4b --name-only` touched only:

- `Cargo.toml`, `Cargo.lock`
- `crates/simthing-clausething/**`
- `THIRD_PARTY_LICENSES.md`
- `docs/tests/ct_0a_impl_results.md`
- `docs/design_0_0_8_1_clausething_production_track.md`

**No changes** to `simthing-driver`, `simthing-sim`, `simthing-gpu`, `simthing-spec`, or WGSL.

### Dependency isolation

- `crates/simthing-driver/Cargo.toml` dependencies: `simthing-core`, `simthing-feeder`, `simthing-gpu`,
  `simthing-sim`, `simthing-spec` — **no `simthing-clausething`**.
- `Cargo.lock`: `simthing-clausething` is a workspace member with **zero reverse dependencies**.
- No crate `Cargo.toml` lists `simthing-clausething` as a dependency.

### Failing test (focused)

| Field | Value |
|---|---|
| Test binary | `simthing-driver --test phase_m_frontier_v1_4_field_policy_route_replay` |
| Test name | `frontier_v1_4_route_replay_reproducibility` |
| Golden constant | `FRONTIER_V1_FIELD_POLICY_ROUTE_REPLAY_FINGERPRINT = "4382ec7ef93c9174"` |
| Observed `combined_hex()` (this host) | `a9e397321e4f8e9b` |
| Assertion site | line 451 — observed vs golden (lines 447–450: `run_a` vs `run_b` **match**) |

### Pre-CT-0a control (same host)

At parent commit `6937858` (pre-PR #571), same test fails with **identical** values:

- observed: `a9e397321e4f8e9b`
- golden: `4382ec7ef93c9174`

### Repeat-run stability (post-CT-0a, same host)

Three consecutive invocations of
`cargo test -p simthing-driver frontier_v1_4_route_replay_reproducibility` all produced observed
`a9e397321e4f8e9b`. The mismatch is **stable** (host/GPU golden drift), not nondeterministic
between back-to-back runs within a single test (`run_a` == `run_b`).

### Why CT-0a cannot affect this failure

CT-0a added an isolated parser crate with no dependency edges into driver/sim/gpu/spec and no shared
runtime code changes. The failure is a GPU route-replay golden fingerprint mismatch in an existing
FrontierV1 driver integration test — outside CT-0a scope and unchanged by PR #571. GPU golden
fingerprints were **not** updated in this remedial (not authorized).

## Commands run

```powershell
cargo test -p simthing-clausething
cargo fmt --all -- --check
cargo test --workspace
cargo test -p simthing-driver frontier_v1_4_route_replay_reproducibility
git diff 6937858..bb44f4b --name-only
git checkout 6937858
cargo test -p simthing-driver frontier_v1_4_route_replay_reproducibility
git checkout master
```

**Clippy:** not run — no active repo/CI clippy gate found.

### Test results

| Command | Result |
|---|---|
| `cargo test -p simthing-clausething` | **PASS** (1 integration test) |
| `cargo fmt --all -- --check` | **PASS** |
| `cargo test --workspace` | **FAIL** — 1 test: `frontier_v1_4_route_replay_reproducibility` (see isolation section) |
| Pre-CT-0a control (`6937858`) same GPU test | **FAIL** — same fingerprint values |

## Safety / non-goal confirmations

- **No Paradox / Stellaris / Clausewitz source material committed.**
- **No runtime or default-on wiring.**
- **`simthing-sim` untouched.**
- **`simthing-gpu` untouched.**
- **`simthing-driver` code untouched** (remedial and PR #571).
- **No WGSL / GPU primitive / `AccumulatorRole`.**
- **No ClauseScript hydration to `simthing-spec`.**
- **No CT-0b drift.**

## Scratch / artifact cleanup

No temporary logs, parser dumps, duplicate reports, or superseded `docs/tests` artifacts committed.
This file is the single CT-0a visibility record (updated in place for remedial).

---

## Required closure questions

### 1. specified vs implemented?

**Yes — match.** CT-0a delivered the isolated parser crate skeleton, vendored jomini text path,
license accounting (now corrected), synthetic smoke test, and production-track ledger update. No
CT-0a requirement was proxied or deferred. Remedial repaired license notice only; no scope expansion.

### 2. Did the license file include the required upstream copyright notice?

**Yes (after remedial).** `crates/simthing-clausething/src/jomini/LICENSE` now includes
`Copyright (c) Nick Babcock` plus verbatim MIT permission text from upstream `LICENSE.txt` at
commit `fff00d8`. Upstream's checked-in `LICENSE.txt` omits the copyright line; holder name is
sourced from upstream `Cargo.toml` `authors` at the pinned commit.

### 3. Did CT-0a alter any runtime/GPU/driver behavior?

**No.** PR #571 and this remedial changed only `simthing-clausething`, workspace membership,
license/docs. No driver/sim/gpu/spec/runtime code.

### 4. Did `cargo test -p simthing-clausething` pass?

**Yes.**

### 5. Did `cargo fmt --all -- --check` pass?

**Yes.**

### 6. Did `cargo test --workspace` pass?

**No.** One failure: `simthing-driver::frontier_v1_4_route_replay_reproducibility`.

### 7. If `cargo test --workspace` did not pass, is the failure isolated from CT-0a by dependency/file-change evidence?

**Yes.** No `simthing-clausething` dependency in `simthing-driver`; PR #571 did not touch
driver/sim/gpu/spec; pre-CT-0a commit `6937858` fails the same test with the same fingerprint
values on this host. CT-0a cannot affect GPU route-replay golden matching.

### 8. Are any new/superseded test-result files left behind?

**No.** Only this updated report under `docs/tests/`.
