# CI doctrine scan data home (CI-A-SCAN-DEFS-0)

Track A grep-only tripwire data lives here. **Heuristics and allowlists are data; the runner is a thin engine.**

## Layout

| Path | Role |
|---|---|
| `scans.tsv` | One scan per non-comment line: `id \| severity \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker` |
| `allow/sealed_producers.txt` | Sanctioned sealed-type producer doors (`read_*` / `dispatch_*` / `apply_*` / `cpu_oracle_*`) |
| `allow/inert_buffer_handles.txt` | Provably-inert public buffer utilities (`inert-util` only) |
| `allow/kernel_surface.txt` | Closed set of kernel `lib.rs` exports (`surface-inert` / `authority-export` / `sealed-export`) |
| `audit_kernel_surface.py` | Re-derive `kernel_surface.txt` from `lib.rs` (grouped + single-line `pub use`) |
| `doctrine_scan.sh` | Thin runner: reads data, runs `rg -U`, emits the §1 report |

Field separator in all data files: ` | ` (space-pipe-space). Lines starting with `#` are comments.

## Run locally (optional)

```bash
bash scripts/ci/doctrine_scan.sh
```

Authoritative execution is on GitHub (`ubuntu-latest`) after `CI-A-WORKFLOW-0`. Exit non-zero only on hard `FAIL` or scanner/data-format error; `INSPECT` exits zero.

**CI-A-WORKFLOW-0 rule:** run **RELIABLE** scans whole-tree; run **HEURISTIC** scans against the **PR diff only**. Whole-tree `doctrine_scan.sh` output is positive-control evidence, not the per-PR triage volume. §1A spam-bounds count branch-introduced (delta) INSPECTs, never baseline.

## Allowlist door-class rules (file-aware)

| File | Allowed `door-class` values | Grammar |
|---|---|---|
| `sealed_producers.txt` | `read`, `dispatch`, `apply`, `cpu_oracle` | Symbol must match producer prefix grammar |
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
6. Do **not** edit `doctrine_scan.sh` for doctrine changes (HEURISTIC-only generic filters live in the runner).

## Earn one allowlist record

Add one conforming line to the relevant `allow/*.txt` with symbol, door-class, rationale, and promotion-blocker. Allowlist edits are deliberate reviewed widenings — never shell edits to defeat a scan.

## Scans shrink as invariants promote to types

When an invariant promotes to a type boundary, **delete the scan** in the same PR — do not accumulate prose guards.

## What this rung does not include

- Closed-set allowlist **enforcement** scans (`CI-A-ALLOWLIST-SCANS-0`)
- Fixtures, self-test, workflow YAML, hook installer, triage log
- Any fourth allowlist/config layer
