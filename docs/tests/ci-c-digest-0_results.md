# CI-C-DIGEST-0 Results

## Status

**PROBATION** - pending DA/Owner review. C3 and CF remain DEFERRED.

## PR / branch / merge

- Branch: `ci-c-digest-0`
- PR: pending
- Merge: pending

## What changed

- Added `scripts/ci/gen_digest.sh`, invokable as `bash scripts/ci/gen_digest.sh` and `bash scripts/ci/gen_digest.sh --check`.
- Added generated `docs/sanctioned_surface.md`, derived only from `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`.
- Updated the `CI-C-DIGEST-0` lifecycle row to PROBATION and added the evidence-index row.

No allowlist, scan definition, scanner engine, workflow, dashboard, metrics, C3 addendum, Track B, or Rust/runtime file was touched.

## Generated digest exactness

`gen_digest.sh` parses:

- `scripts/ci/allow/sealed_producers.txt` - 15 rows
- `scripts/ci/allow/inert_buffer_handles.txt` - 2 rows
- `scripts/ci/allow/kernel_surface.txt` - 195 rows
- `scripts/ci/allow/sealed_types.txt` - 12 rows
- `scripts/ci/scans.tsv` - 14 rows

The generated digest includes a source manifest with row counts and SHA-256 fingerprints. `--check` regenerates the digest in memory, byte-compares it with `docs/sanctioned_surface.md`, and parses the generated Markdown tables for the sanctioned door/type sections to verify they exactly equal the parsed allowlist/type rows.

## Load-bearing proofs

Local Git Bash needed the bundled workspace Python prepended to `PATH` for commands that invoke the Python allowlist scanner/generator. The command under proof remained the requested `bash ...` command.

### Fresh base doctrine scan before C2 edits

This was run from current `master` before creating branch `ci-c-digest-0`, satisfying Orchestration's first-proof requirement.

```bash
bash scripts/ci/doctrine_scan.sh
```

```
DOCTRINE SCAN REPORT  (commit de95064a4c, 2026-07-01T07:33:09Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 1 entries
```

### Generate digest

```bash
bash scripts/ci/gen_digest.sh
```

```
generated docs/sanctioned_surface.md
```

### Generated artifact diff

```bash
git diff -- docs/sanctioned_surface.md
```

Output: a new-file diff for `docs/sanctioned_surface.md` (`new file mode 100644`, 279 generated lines). The diff begins with the generated-file warning and source manifest, then the sections required by the handoff:

```text
diff --git a/docs/sanctioned_surface.md b/docs/sanctioned_surface.md
new file mode 100644
index 0000000000..dab171e281
--- /dev/null
+++ b/docs/sanctioned_surface.md
@@ -0,0 +1,279 @@
+# Sanctioned Surface Digest
+> GENERATED FILE. Do not hand-edit. Regenerate with `bash scripts/ci/gen_digest.sh`.
+> Source of truth: `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`.
```

The full emitted diff is the PR diff for `docs/sanctioned_surface.md`; it is not hand-authored.

### Exactness check

```bash
bash scripts/ci/gen_digest.sh --check
```

```
gen_digest --check: PASS
```

### Shell syntax

```bash
bash -n scripts/ci/gen_digest.sh
```

```
<no output; exit 0>
```

### Final doctrine scan on C2 branch

```bash
bash scripts/ci/doctrine_scan.sh
```

```
DOCTRINE SCAN REPORT  (commit ed0c525c42, 2026-07-01T07:41:27Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 1 entries
```

### Scope diff

```bash
git diff --name-only master...HEAD
```

```
docs/design_0_0_8_4_6_ci_scaffolding.md
docs/sanctioned_surface.md
docs/tests/ci-c-digest-0_results.md
docs/tests/current_evidence_index.md
scripts/ci/gen_digest.sh
```

## INSPECT / triage

No INSPECT fired in the fresh base scan or final C2 branch scan. No `scripts/ci/triage_log.tsv` row was added.

## Scope Ledger

| Path | Touched | Note |
|---|---|---|
| `scripts/ci/gen_digest.sh` | yes | new canonical generator/checker |
| `docs/sanctioned_surface.md` | yes | generated digest artifact |
| `docs/tests/ci-c-digest-0_results.md` | yes | this evidence doc |
| `docs/tests/current_evidence_index.md` | yes | one evidence row |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes | C2 row to PROBATION |
| `scripts/ci/triage_log.tsv` | no | no INSPECT fired |
| `scripts/ci/scans.tsv`, `scripts/ci/allow/**`, scanner engines, workflows, crates, C3/CF/addendum/dashboard/metrics files | no | forbidden / untouched |

## Known gaps / next

- Replace pending PR/merge fields after PR creation and merge.
- Do not merge until live GitHub Actions Doctrine Scan succeeds on this PR's own head.
