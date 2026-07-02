# CI doctrine scan data home (CI-A-SCAN-DEFS-0)

Track A grep-only tripwire data lives here. **Heuristics and allowlists are data; the runner is a thin engine.**

> **Authoritative reference:** [`docs/ci_screening_surface.md`](../../docs/ci_screening_surface.md) — the full
> map of the screening logic, every file, the **strict rigor to add a `scans.tsv` / allowlist entry**, and the
> triage agent's role. This README is the operational layout; the surface doc is the auditable reference and the
> foundation for Track C introspection/onboarding. Change the screening surface → update both in the same PR.

## Layout

| Path | Role |
|---|---|
| `scans.tsv` | One scan per non-comment line: `id \| severity \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker` |
| `sealed_producers.txt` | Sanctioned sealed-type producer doors (`read_*`/`readback_*` / `dispatch_*` / `apply_*` / `cpu_oracle_*`) |
| `allow/sealed_types.txt` | Closed set of sealed authority **type names** (bare list, not a door-class record). Loaded by `scan_allowlists.py` — migrated from a hard-coded tuple at closeout; missing/empty fails loudly |
| `allow/inert_buffer_handles.txt` | Provably-inert public buffer utilities (`inert-util` only) |
| `allow/kernel_surface.txt` | Closed set of kernel `lib.rs` exports (`surface-inert` / `authority-export` / `sealed-export`) |
| `audit_kernel_surface.py` | Re-derive `kernel_surface.txt` from `lib.rs` (grouped + single-line `pub use`) |
| `verify_kernel_surface.py` | Diff `kernel_surface.txt` against `lib.rs` exports |
| `scan_allowlists.py` | Closed-set allowlist scan engine (`sealed-producers`, `buffer-handles`, `kernel-surface`) |
| `doctrine_scan.sh` | Thin runner: reads data, runs `rg -U` and `@ALLOWLIST:` scans, emits the §1 report (default whole-tree; `--pr-delta BASE HEAD` for PR mode; `--track-doc PATH` for an opt-in sibling track addendum) |
| `doctrine_pr_scan.sh` | PR workflow wrapper — HEURISTIC delta-scope, RELIABLE whole-tree (`--prove-delta` for local proof cases) |
| `doctrine_selftest.sh` | Fixture self-test runner: exercises `fixtures/` corpus against sandbox copies of `doctrine_scan.sh` (CI-A-SELFTEST-0R repaired for determinism) |
| `inspect_spam_check.sh` | Spam-bound checker for INSPECT flags (CI-A-SELFTEST-INSPECT-REPAIR-0 — real branch-history checks + leak-safe `--prove`) |
| `triage_log.tsv` | Append-only triage log (CI-A-INSPECT-TRIAGE-0) |
| `inspect_justifications.tsv` | Optional author-provided per-INSPECT justifications |
| `fixtures/` | Known-bad and false-positive trap corpus for **`CI-A-SELF-TEST-0`** |

GitHub Actions gate: `.github/workflows/doctrine-scan.yml` (**`CI-A-WORKFLOW-0`**) — digest freshness check, self-test, then production scan on `pull_request` and push to `master`.

Track B executable proof (non-blocking): `.github/workflows/doctrine-exec.yml` (**`CI-B-GH-CPU-0`**) + `.github/workflows/doctrine-exec-commands.yml` (**`CI-B-GH-COMMENT-0` / `CI-B-GH-TRIAGE-0`**). Operator quick-reference: `docs/ci_screening_surface.md` §9.

Field separator in all data files: ` | ` (space-pipe-space). Lines starting with `#` are comments.

## Fixture corpus (`fixtures/`)

Committed in **`CI-A-FIXTURES-0`**. See `fixtures/README.md` for the known-bad and trap inventory.

- **known_bad/** — one fixture per RELIABLE scan (plus HEURISTIC production negative controls).
- **traps/** — false-positive shapes (comments, `#[cfg(test)]`, `pub(crate)`, jomini `write_*`, etc.).

Normal `doctrine_scan.sh` does **not** scan `fixtures/` — production globs target `crates/**` and allowlists only. **`doctrine_selftest.sh`** injects fixtures into temporary sandbox trees and runs a copied scanner against those paths.

## Run locally (optional)

```bash
bash scripts/ci/doctrine_selftest.sh   # fixture corpus self-test (must PASS before trusting scans)
bash scripts/ci/gen_digest.sh --check  # global sanctioned-surface freshness check (CI-enforced)
bash scripts/ci/doctrine_scan.sh       # whole-tree production scan (master positive control)
bash scripts/ci/doctrine_scan.sh --track-doc docs/<track>.md   # global floor + that track doc's sibling addendum, if present
bash scripts/ci/doctrine_scan.sh --prove-addendum              # synthetic proof: opt-in, auto-detach, additive-only, track digest scope
bash scripts/ci/doctrine_pr_scan.sh BASE_SHA HEAD_SHA   # PR-delta scan (HEURISTIC delta, RELIABLE whole-tree)
bash scripts/ci/doctrine_pr_scan.sh --prove-delta       # local PR-delta proof cases
bash scripts/ci/gen_digest.sh --track-doc docs/<track>.md --output docs/tests/<track>_digest.md
bash scripts/ci/gen_digest.sh --track-doc docs/<track>.md --output docs/tests/<track>_digest.md --check
bash scripts/ci/inspect_spam_check.sh <branch>         # triage spam check (no name-based fixture aliases)
bash scripts/ci/inspect_spam_check.sh --prove          # synthetic temp-repo proof cases
```

GitHub Actions (`.github/workflows/doctrine-scan.yml`): **pull_request** runs global digest freshness, self-test, then `doctrine_pr_scan.sh` with base/head SHAs; **push to master** runs global digest freshness, self-test, then whole-tree `doctrine_scan.sh`.

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

## Opt-in track addenda

A production track may carry a CI addendum next to its canonical track document:

| Path | Role |
|---|---|
| `<track-doc>.ci.tsv` | Track-local scan rows with the same seven-field schema as `scripts/ci/scans.tsv` |
| `<track-doc>.ci.allow/` | Optional track-local digest context using `sealed_producers.txt`, `inert_buffer_handles.txt`, `kernel_surface.txt`, and/or `sealed_types.txt` with the same row formats as global allowlists |

The addendum is strictly opt-in. `doctrine_scan.sh` loads no track addenda unless invoked with `--track-doc PATH`, and then loads only that document's sibling files. An absent sibling addendum means global-floor-only.

Addenda are additive-only. A track addendum scan-id must not redefine a global `scans.tsv` id, and a track digest allow row must not duplicate a global allow/type key. These are scanner/data errors, not triage findings. Track digest mode reads global sources plus the active track's sibling addendum only; it never overwrites `docs/sanctioned_surface.md` unless run in ordinary global mode.

## Earn one allowlist record

Add one conforming line to the relevant `allow/*.txt` with symbol, door-class, rationale, and promotion-blocker. Allowlist edits are deliberate reviewed widenings — never shell edits to defeat a scan.

## Scans shrink as invariants promote to types

When an invariant promotes to a type boundary, **delete the scan** in the same PR — do not accumulate prose guards.

## What this track does not include

- Workflow YAML, hook installer, triage log
- Any fourth allowlist/config layer

Fixtures live under `fixtures/` — see `fixtures/README.md`. Self-test runner: `doctrine_selftest.sh`.
