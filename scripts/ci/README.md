# CI doctrine scan data home (CI-A-SCAN-DEFS-0)

Track A grep-only tripwire data lives here. **Heuristics and allowlists are data; the runner is a thin engine.**

## Layout

| Path | Role |
|---|---|
| `scans.tsv` | One scan per non-comment line: `id \| severity \| target-glob \| pattern \| exclude \| doctrine-ref \| promotion-blocker` |
| `allow/sealed_producers.txt` | Sanctioned sealed-type producer doors (`read_*` / `dispatch_*` / `apply_*` / `cpu_oracle_*`) |
| `allow/inert_buffer_handles.txt` | Provably-inert public buffer utilities (`inert-util`) |
| `allow/kernel_surface.txt` | Closed set of kernel `lib.rs` exports (populated from the real surface) |
| `doctrine_scan.sh` | Thin runner: reads data, runs `rg -U`, emits the §1 report |

Field separator in all data files: ` | ` (space-pipe-space). Lines starting with `#` are comments.

## Run locally (optional)

```bash
bash scripts/ci/doctrine_scan.sh
```

Authoritative execution is on GitHub (`ubuntu-latest`) after `CI-A-WORKFLOW-0`. Exit non-zero only on hard `FAIL` or scanner/data-format error; `INSPECT` exits zero.

## Add one scan

1. Append one line to `scans.tsv` with all seven fields.
2. `severity` must be `RELIABLE` or `HEURISTIC`.
3. Every `RELIABLE` line needs a non-empty `promotion-blocker` (the type boundary that will retire this scan).
4. Use `rg -U` multiline patterns; put false-positive filters in `exclude` (semicolon-separated).
5. Prefix pattern with `@REQUIRE:` when the scan must find the pattern in every target file (inverted semantics).
6. Do **not** edit `doctrine_scan.sh` for doctrine changes.

## Earn one allowlist record (sanctioned door)

When a new producer/handle/export is genuinely sanctioned:

1. Add **one** conforming line to the relevant `allow/*.txt` file:
   `symbol | door-class | rationale | promotion-blocker`
2. `door-class` ∈ `{read, dispatch, apply, cpu_oracle, inert-util}`.
3. Symbol name must match door-class grammar (`read_*`, `dispatch_*`, `apply_*`, `cpu_oracle_*`, or any symbol for `inert-util`).
4. Rationale and promotion-blocker are mandatory — casual widenings are rejected at parse time.
5. Allowlist edits are deliberate, reviewed widenings in data — **never** shell edits to defeat a scan.

## Scans shrink as invariants promote to types

A grep scan is admission-ladder rung 3. When an invariant promotes to a type boundary, **delete the scan** in the same PR that lands the type — do not accumulate prose guards.

## What this rung does not include

- Closed-set allowlist **enforcement** scans (`CI-A-ALLOWLIST-SCANS-0`)
- Fixtures, self-test, workflow YAML, or hook installer
- Any fourth allowlist/config layer
