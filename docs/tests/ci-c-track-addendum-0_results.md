# CI-C-TRACK-ADDENDUM-0 Results

Date: 2026-07-01

## Scope

Implemented the sanctioned sibling addendum shape:

- `<track-doc>.ci.tsv`
- `<track-doc>.ci.allow/`

No global scan or allowlist data was edited. The default scanner and digest paths remain global-only unless a caller explicitly passes `--track-doc`.

## Local proofs

Environment note: on this Windows host, PowerShell `bash` resolves to WSL, which has no installed distro. Local proof commands were run through `C:\Program Files\Git\bin\bash.exe` with the bundled Codex `rg` and Python directories prepended to `PATH`. Inside Git Bash, `bash` resolved to `/usr/bin/bash`, `rg --version` reported `ripgrep 15.1.0`.

### `bash scripts/ci/doctrine_scan.sh`

Result: PASS

Key summary:

```text
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```

### `bash scripts/ci/gen_digest.sh --check`

Result: PASS

```text
gen_digest --check: PASS
```

### `bash scripts/ci/doctrine_scan.sh --prove-addendum`

Result: PASS

```text
doctrine_scan --prove-addendum: PASS
```

This proof creates temporary sibling addenda and verifies:

- opt-in only: global scan ignores an unselected track addendum;
- auto-detach: selecting a different track doc does not load the inactive sibling addendum;
- additive-only: redefining a global scan-id hard-fails as scanner/data error;
- track digest scope: generated track digest contains global data plus the active track addendum only, excluding inactive track addenda.

### `bash -n scripts/ci/doctrine_scan.sh`

Result: PASS

### `bash -n scripts/ci/gen_digest.sh`

Result: PASS

### `git diff -- scripts/ci/scans.tsv scripts/ci/allow`

Result: no output.

Global scan/allowlist data stayed byte-unchanged.

### `git diff --name-only master...HEAD`

Result:

```text
docs/sanctioned_surface.md
docs/tests/ci-c-track-addendum-0_results.md
docs/tests/current_evidence_index.md
scripts/ci/README.md
scripts/ci/doctrine_scan.sh
scripts/ci/gen_digest.sh
```

No `.github/workflows/**`, `crates/**`, global `scripts/ci/scans.tsv`, or global `scripts/ci/allow/**` paths are present.
