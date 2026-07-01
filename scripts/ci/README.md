# CI doctrine scan data home (CI-A-SCAN-DEFS-0)

Track A grep-only tripwire data lives here. **Heuristics and allowlists are data; the runner is a thin engine.**

## Layout

| Path | Role |
|---|---|
| `scans.tsv` | One scan per non-comment line: `id \| severity \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker` |
| `sealed_producers.txt` | Sanctioned sealed-type producer doors (`read_*`/`readback_*` / `dispatch_*` / `apply_*` / `cpu_oracle_*`) |
| `allow/inert_buffer_handles.txt` | Provably-inert public buffer utilities (`inert-util` only) |
| `allow/kernel_surface.txt` | Closed set of kernel `lib.rs` exports (`surface-inert` / `authority-export` / `sealed-export`) |
| `audit_kernel_surface.py` | Re-derive `kernel_surface.txt` from `lib.rs` (grouped + single-line `pub use`) |
| `verify_kernel_surface.py` | Diff `kernel_surface.txt` against `lib.rs` exports |
| `scan_allowlists.py` | Closed-set allowlist scan engine (`sealed-producers`, `buffer-handles`, `kernel-surface`) |
| `doctrine_scan.sh` | Thin runner: reads data, runs `rg -U` and `@ALLOWLIST:` scans, emits the §1 report |
| `doctrine_selftest.sh` | Fixture self-test runner: exercises `fixtures/` corpus against sandbox copies of `doctrine_scan.sh` |
| `fixtures/` | Known-bad and false-positive trap corpus for **`CI-A-SELF-TEST-0`** |

GitHub Actions gate: `.github/workflows/doctrine-scan.yml` (**`CI-A-WORKFLOW-0`**) — self-test then production scan on `pull_request` and push to `master`.

Field separator in all data files: ` | ` (space-pipe-space). Lines starting with `#` are comments.

## Fixture corpus (`fixtures/`)

Committed in **`CI-A-FIXTURES-0`**. See `fixtures/README.md` for the known-bad and trap inventory.

- **known_bad/** — one fixture per RELIABLE scan (plus HEURISTIC production negative controls).
- **traps/** — false-positive shapes (comments, `#[cfg(test)]`, `pub(crate)`, jomini `write_*`, etc.).

Normal `doctrine_scan.sh` does **not** scan `fixtures/` — production globs target `crates/**` and allowlists only. **`doctrine_selftest.sh`** injects fixtures into temporary sandbox trees and runs a copied scanner against those paths.

## Run locally (optional)

```bash
bash scripts/ci/doctrine_selftest.sh   # fixture corpus self-test (must PASS before trusting scans)
bash scripts/ci/doctrine_scan.sh       # production tree scan
```

`doctrine_selftest.sh` proves each RELIABLE known-bad still trips its scan, HEURISTIC production controls yield INSPECT, traps do not hard-FAIL, and neutralizing a scan pattern is detected (rot test). **`CI-A-WORKFLOW-0`** runs the self-test in CI before the production scan on every PR and push to `master`.

Authoritative execution is on GitHub (`ubuntu-latest`) via `.github/workflows/doctrine-scan.yml`. Exit non-zero only on hard `FAIL` or scanner/data-format error; `INSPECT` exits zero.

**CI-A-WORKFLOW-0 rule:** run **RELIABLE** scans whole-tree; run **HEURISTIC** scans against the **PR diff only**. Whole-tree `doctrine_scan.sh` output is positive-control evidence, not the per-PR triage volume. §1A spam-bounds count branch-introduced (delta) INSPECTs, never baseline.

## Allowlist door-class rules (file-aware)

| File | Allowed `door-class` values | Grammar |
|---|---|---|
| `sealed_producers.txt` | `read`, `dispatch`, `apply`, `cpu_oracle` | `read_*`/`readback_*`; `dispatch_*`; `apply_*`; `cpu_oracle_*` or sanctioned `execute_*_cpu` oracle batch doors |
| `inert_buffer_handles.txt` | `inert-util` | Genuinely inert caller-owned buffer utilities only |
| `kernel_surface.txt` | `surface-inert`, `authority-export`, `sealed-export` | **`inert-util` forbidden** — no laundering authority as inert |

`kernel_surface.txt` markers:

- **surface-inert** — inert constants/helpers with no authority-bearing state/effect
- **authority-export** — runtime/GPU/session/readback/oracle surfaces; xref `sealed_producers:<symbol>` when applicable
- **sealed-export** — sealed record/type exports (`ThresholdEvent`, `EmissionRecord`, …)

Re-derive kernel surface after `lib.rs` export changes:

```bash
python scripts/ci/audit_kernel_surface.py
python scripts/ci/verify_kernel_surface.py   # completeness diff vs lib.rs
```

## Add one scan

1. Append one line to `scans.tsv` with all seven fields.
2. `severity` must be `RELIABLE` or `HEURISTIC`.
3. Every `RELIABLE` line needs a non-empty `promotion-blocker`.
4. Use `rg -U` multiline patterns; put false-positive filters in `exclude` (semicolon-separated).
5. Prefix pattern with `@REQUIRE:` when the scan must find the pattern in every target file.
6. Prefix pattern with `@ALLOWLIST:sealed-producers`, `@ALLOWLIST:buffer-handles`, or `@ALLOWLIST:kernel-surface` for closed-set RELIABLE allowlist scans (implemented by `scan_allowlists.py`).
7. Do **not** edit `doctrine_scan.sh` for doctrine changes (HEURISTIC-only generic filters and generic `@ALLOWLIST:` dispatch live in the runner).

## Closed-set allowlist scans (RELIABLE)

| Scan ID | Allowlist | Behavior |
|---|---|---|
| `ALLOW-SEALED-PRODUCERS` | `sealed_producers.txt` | Every public `pub fn` in `simthing-kernel/src` returning a sealed type must be allowlisted (multiline signatures; `pub(crate)` excluded) |
| `ALLOW-BUFFER-HANDLES` | `inert_buffer_handles.txt` | Every public `Buffer`/`&Buffer`/`BindingResource` escape must be allowlisted or `pub(crate)` |
| `ALLOW-KERNEL-SURFACE` | `kernel_surface.txt` | `lib.rs` exports must exactly match the allowlist (grouped + single-line `pub use`) |

On FAIL, add one conforming row to the relevant `allow/*.txt` — do not edit the scanner.

## Earn one allowlist record

Add one conforming line to the relevant `allow/*.txt` with symbol, door-class, rationale, and promotion-blocker. Allowlist edits are deliberate reviewed widenings — never shell edits to defeat a scan.

## Scans shrink as invariants promote to types

When an invariant promotes to a type boundary, **delete the scan** in the same PR — do not accumulate prose guards.

## What this track does not include

- Workflow YAML, hook installer, triage log
- Any fourth allowlist/config layer

Fixtures live under `fixtures/` — see `fixtures/README.md`. Self-test runner: `doctrine_selftest.sh`.
