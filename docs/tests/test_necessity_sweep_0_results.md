# TEST-NECESSITY-SWEEP-0 Results

## Status

**PROBATION / DA/OWNER REVIEW** — implemented on branch `grok/test-necessity-sweep-0`; merge not authorized for Grok/Cursor.

## Mission

One-rung deletion of every test failing the Necessity Test across the GHA-safe corpus. No representative-curation, no consolidation-for-tidiness, no forbidden desktop/GPU probing.

The Necessity Test supersedes "one representative per boundary." This sweep deletes tests whose invariants are already caught by type boundaries, production admission hard-errors on live paths, or existing canonical/integration paths. The per-boundary floor is zero.

## Constitutional basis

- PR #1121 (`bc8383d0`) — Necessity Test doctrine repair
- `docs/invariants.md` — Necessity Test row
- `docs/ci_screening_surface.md` — per-boundary floor zero
- Retired fossil: `permanent-residue:promoted-admission-representative`

## Scope

In scope: `crates/**` test deletion, ledger reconciliation (`test_inventory.tsv`, `test_pare_boundary_rows.tsv`, `test_pare_audit.tsv`, `test_pare_boundaries.tsv`, `test_residue_classes.tsv`), promotion plan, manifest, this results doc, design doc D2aa row, evidence index.

Out of scope: `.github/**`, production logic edits, forbidden owner-deep proof.

## Keep set applied

| Class | Treatment |
|---|---|
| CI scanner fixtures (`scripts/ci/fixtures/**`) | KEEP |
| CPU/GPU oracle-parity survivors | KEEP |
| Determinism/golden-byte | KEEP |
| STEAD-required / doc-named invariant | KEEP |
| Active TP live-rung suites | KEEP |
| Genuine behavior-regression AUDIT corpus | KEEP (not admission enumeration) |

## Deletion summary

| Metric | Count |
|---|---:|
| Tests deleted (source) | 80 |
| Files deleted | 3 (`admission_boundary.rs`, both hygiene consolidation files) |
| Files edited | ~45 crate test/src files |
| Crates touched | 14 |

Primary deletion classes:
- Tier 2 admission representatives (production `validate` / `parse` / admission hard-error owns coverage)
- `permanent-residue:promoted-admission-representative` (6 mapgenerator rows — retired class)
- All 25 `promotion-target:*admission*` rows
- Hygiene-theater table batteries (2 promotion targets)
- Closed-rung sunset default-path / legacy-shader proofs (6 sim rows; oracle-parity golden survivors kept)
- Owner-deep admission rows in tools/mapeditor/gpu (source deleted; owner-local compile deferred)

## Kept summary

| Bucket | Approx. live inventory rows |
|---|---:|
| KEEP_NECESSARY (oracle, golden, STEAD, seal-proof, behavior AUDIT, TP) | 3990 |
| KEEP_OWNER_DEEP_PENDING_LOCAL (manifest) | 12 |
| ESCALATE_SEALED_BOUNDARY | 0 |

## Owner-deep manifest summary

| Crate | Rows deleted in source | Proof deferred |
|---|---:|---|
| simthing-tools | 5 typeface admission | owner local `cargo check -p simthing-tools --tests` |
| simthing-mapeditor | 5 unit + 1 integration admission | owner local `cargo check -p simthing-mapeditor --tests` |
| simthing-gpu | 3 unit + 1 integration admission | owner local `cargo test -p simthing-gpu` (forbidden in GHA) |

Manifest: `docs/tests/test_necessity_sweep_0_manifest.tsv`

## Inventory delta

| | Count |
|---|---:|
| Before | 4070 |
| After | 3990 |
| Delta | −80 |

## Promotion backlog delta

| | Count |
|---|---:|
| Before | 25 |
| After | 0 |
| Delta | −25 |

All admission and hygiene promotion targets retired by deletion under Necessity Test.

## Proof

| Gate | Result |
|---|---|
| Doctrine Scan | (see PR proof block) |
| Digest `--check` | (see PR proof block) |
| Inventory check | (see PR proof block) |
| Boundary check | (see PR proof block) |
| Drift check | (see PR proof block) |
| Five-crate survivor floor | PASS — core, kernel, sim, workshop, mapgenerator `--tests` compile |
| Targeted survivor proof | Not run (admission deletes are production-owned) |
| `git diff --check` | (see PR proof block) |

## Forbidden proof avoided

- `cargo test --workspace` — not run
- Bare full-crate test batteries — not run
- `simthing-tools` / `simthing-mapeditor` / `simthing-gpu` test execution — not run
- Owner-deep doctrine-exec profiles — not run
- workflow_dispatch / Bevy / winit / wgpu / desktop / GPU proof — not run
- System package installs — not run

## Escalations

| Class | Count |
|---|---:|
| Sealed boundary | 0 |
| Owner-local compile required | 12 (tools/mapeditor/gpu manifest) |
| Dependency/canonical-function uncertainty | 0 |

## Graduation routing

- **Risk class:** owner-mandated one-rung deletion wave / DA-owner-held
- **DA question:** Does this manifest correctly delete every test failing the Necessity Test and keep only necessary survivors?
- **Merge:** not authorized for Grok/Cursor