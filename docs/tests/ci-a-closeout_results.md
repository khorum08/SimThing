# CI-A-CLOSEOUT-0 — results

**Status:** DA-CLOSED (Executive DA: Opus/Owner, 2026-07-01)
**Track:** 0.0.8.4.6 CI Scaffolding — Track A → **CLOSED**

## What closed
- **DA-equivalence contract** — a clean RELIABLE (allowlist) scan is treated as "the DA ran it"; FAIL = HOLD; INSPECT routes to §1A triage. Recorded as closed doctrine (landed core §1.2/§1.2.1, constitution §0.x, handoff spine via #1043).
- **Retirement contract** — a scan is rung-3 residue; when its invariant promotes to a type/admission hard-error, the same PR deletes/narrows the now-redundant scan. Landed at all three altitudes.
- **Three-altitude doctrine landing** — verified in the tree (below).
- **Reliability legend** — published in the track doc (§ reliability legend) and here.
- **SEALED_TYPES data-file debt** — cleared: migrated out of `scan_allowlists.py` into `scripts/ci/allow/sealed_types.txt`; the engine now loads the sealed-type set from data and fails loudly if the file is missing/empty.

## SEALED_TYPES debt — fix detail
- New data file `scripts/ci/allow/sealed_types.txt` (12 names, faithful copy of the former tuple — set neither widened nor narrowed).
- `scan_allowlists.py`: hard-coded tuple removed; `load_sealed_types()` reads the data file, rejects a missing/empty file with `SystemExit(2)` → surfaced by `doctrine_scan.sh` as a `scanner/data error` → FAIL verdict (never a silent pass).
- Placed under `allow/` so the self-test's existing `cp allow/*.txt` bundle copies it into the sandbox — no change to `doctrine_selftest.sh` needed.

## Verified PRs
- #1041 repair chain — DA-cleared (independent tree run)
- #1042 DA clearance + scratch cleanup / `.gitignore` guard
- #1043 doctrine landing (merge `a1ef8334ff`) + evidence follow-up `451b664490`

## Load-bearing commands (Opus local run, 2026-07-01)
```
bash scripts/ci/doctrine_selftest.sh       -> DOCTRINE-SELFTEST-VERDICT: PASS
bash scripts/ci/doctrine_scan.sh           -> DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
bash scripts/ci/doctrine_pr_scan.sh --prove-delta   -> PR-delta proof: PASS
bash scripts/ci/inspect_spam_check.sh --prove       -> INSPECT-SPAM-PROOF: PASS
python scripts/ci/verify_kernel_surface.py          -> 195/195, missing: [], extra: []
```
Fail-loud proof: with `sealed_types.txt` removed, `doctrine_scan.sh` emits `scanner/data error: … missing sealed-types data file` (not PASS).

## Three-altitude landing verification (grep)
- `docs/simthing_core_design.md` — DA-equivalence + retirement/promotion-target present.
- `docs/design_0_0_8_3.md` — DA-equivalence + retirement present (contract + §1A + merge-hold + verify-the-tree).
- `docs/handoff_template.md` — CI doctrine-scan spine + retirement + seal-residue-risk present.

## Remaining non-blocking debt
- `doctrine_selftest.sh` runtime ~7 min (process-spawn bound, not rg / not PCRE2). A faster self-test is the anti-fabrication guarantee — optimize the per-line `rg -q` exclusion filter + per-case sandbox re-copy when convenient. Explicitly non-blocking.
