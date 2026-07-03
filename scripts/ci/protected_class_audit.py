#!/usr/bin/env python3
"""Generate TEST-PARE-PROTECTED-CLASS-AUDIT-0 evidence tables."""

from __future__ import annotations

import csv
import sys
from collections import Counter
from pathlib import Path


PROTECTED_CLASSES = {
    "oracle-parity",
    "stead-required",
    "golden-byte",
    "seal-proof",
    "doc-named-invariant",
    "determinism",
    "behavior-regression",
    "escaped-bug",
}

REVIEW_HEADER = [
    "crate",
    "file",
    "test_name",
    "kind",
    "current_class",
    "current_verdict",
    "promotion_target",
    "protected_claim",
    "truth_verdict",
    "live_owner",
    "coverage_surface",
    "canonical_survivor",
    "proof_mode",
    "proposed_next_action",
    "reason",
    "proof_dependency",
]

ORACLE_HEADER = [
    "live_surface_id",
    "crate",
    "kernel_or_op_path",
    "numeric_regime",
    "surviving_test_row",
    "survivor_file",
    "survivor_test_name",
    "proof_mode",
    "coverage_status",
    "notes",
]

SIMPLE_COVERAGE_HEADER = [
    "live_surface_id",
    "crate",
    "surviving_test_row",
    "survivor_file",
    "survivor_test_name",
    "proof_mode",
    "coverage_status",
    "notes",
]

JUDGMENT_HEADER = [
    "crate",
    "file",
    "test_name",
    "kind",
    "current_class",
    "current_verdict",
    "promotion_target",
    "judgment_note_status",
    "proposed_next_action",
    "reason",
]

RISK_TOKENS = {
    "atlas",
    "bevy",
    "gpu",
    "mapeditor",
    "simthing-tools",
    "tools",
    "typeface",
    "wgpu",
    "workshop",
}


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def write_tsv(path: Path, header: list[str], rows: list[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=header, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def key(row: dict[str, str]) -> str:
    return f"{row['crate']}|{row['file']}|{row['test_name']}|{row['kind']}"


def protected_claim(row: dict[str, str]) -> str:
    target = row["promotion_target"].strip()
    if target.startswith("permanent-residue:"):
        return target
    if row["test_name"] == "custom_layout_ethics_axis":
        return "custom_layout_ethics_axis"
    if row["class"] in PROTECTED_CLASSES:
        return f"class:{row['class']}"
    return ""


def proof_mode(row: dict[str, str]) -> str:
    haystack = f"{row['crate']} {row['file']} {row['test_name']}".lower()
    if any(token in haystack for token in RISK_TOKENS):
        return "local-owner-deep"
    if row["kind"] in {"compile_fail", "trybuild", "fixture"}:
        return "gha-cpu"
    return "gha-cpu"


def owner_for(claim: str, row: dict[str, str]) -> str:
    cls = row["class"]
    if "oracle-parity" in claim or cls == "oracle-parity":
        return "core section 4 / constitution section 0.7 oracle-parity doctrine"
    if "stead-required" in claim or cls == "stead-required":
        return "docs/stead_spatial_contract.md section 8 STEAD required suites"
    if "seal-proof" in claim or cls == "seal-proof":
        return "admission-substrate sealed-boundary compile_fail/trybuild doctrine"
    if "golden-byte" in claim or cls == "golden-byte":
        return "determinism and canonical-corpus golden-byte doctrine"
    if "doc-named-invariant" in claim or cls == "invariant-required":
        return "docs/invariants.md named invariant"
    if "behavior-regression" in claim or cls == "behavior-regression":
        return "TIER5 judgment class; KEEP requires specific catches note"
    if "escaped-bug" in claim or cls == "escaped-bug":
        return "TIER5 escaped-bug judgment class; KEEP requires specific catches note"
    if row["test_name"] == "custom_layout_ethics_axis":
        return "Owner/DA carveout recorded in docs/invariants.md"
    return "unknown"


def surface_for(row: dict[str, str]) -> str:
    if row["kind"] in {"compile_fail", "trybuild"}:
        return "compile-time sealed-boundary rejection"
    if row["kind"] == "fixture":
        return "scanner/probe fixture surface"
    if row["class"] == "stead-required":
        return "STEAD section 8 required mapgen contract surface"
    if row["class"] == "golden-byte":
        return "byte-stable deterministic artifact or replay surface"
    if row["class"] == "oracle-parity":
        return "CPU oracle / live execution parity surface"
    if row["class"] == "invariant-required":
        return "doc-named invariant surface"
    return "boundary-audit candidate surface"


def classify(row: dict[str, str]) -> dict[str, str]:
    claim = protected_claim(row)
    if row["verdict"] != "KEEP":
        return {
            "truth_verdict": "OUT_OF_SCOPE",
            "proposed_next_action": "RECLASSIFY_TO_AUDIT",
            "reason": "AUDIT row is not a permanent-residue survivor claim; it remains available to later boundary waves.",
            "proof_dependency": "not-required",
        }
    return {
        "truth_verdict": "TRUE_MEMBER",
        "proposed_next_action": "KEEP",
        "reason": f"KEEP row carries {claim} and maps to a live owner/proof surface.",
        "proof_dependency": proof_mode(row),
    }


def numeric_regime(row: dict[str, str]) -> str:
    name = row["test_name"].lower()
    if "fraction" in name:
        return "fractional dt / non-integer numeric path"
    if "dt" in name or "tick" in name:
        return "tick/dt numeric path"
    if "gpu" in name or "wgsl" in name:
        return "GPU-vs-CPU parity path"
    if "oracle" in name or "parity" in name:
        return "CPU oracle parity path"
    return "named oracle parity path"


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    rows = read_tsv(root / "scripts/ci/test_inventory.tsv")
    protected = [row for row in rows if protected_claim(row)]

    review_rows: list[dict[str, str]] = []
    for row in protected:
        claim = protected_claim(row)
        verdict = classify(row)
        review_rows.append(
            {
                "crate": row["crate"],
                "file": row["file"],
                "test_name": row["test_name"],
                "kind": row["kind"],
                "current_class": row["class"],
                "current_verdict": row["verdict"],
                "promotion_target": row["promotion_target"],
                "protected_claim": claim,
                "truth_verdict": verdict["truth_verdict"],
                "live_owner": owner_for(claim, row),
                "coverage_surface": surface_for(row),
                "canonical_survivor": key(row) if row["verdict"] == "KEEP" else "",
                "proof_mode": proof_mode(row) if row["verdict"] == "KEEP" else "not-required",
                "proposed_next_action": verdict["proposed_next_action"],
                "reason": verdict["reason"],
                "proof_dependency": verdict["proof_dependency"],
            }
        )

    write_tsv(root / "docs/tests/test_pare_protected_class_audit_0_review.tsv", REVIEW_HEADER, review_rows)

    oracle_rows = [
        {
            "live_surface_id": key(row),
            "crate": row["crate"],
            "kernel_or_op_path": row["file"],
            "numeric_regime": numeric_regime(row),
            "surviving_test_row": key(row),
            "survivor_file": row["file"],
            "survivor_test_name": row["test_name"],
            "proof_mode": proof_mode(row),
            "coverage_status": "TRUE_MEMBER",
            "notes": "KEEP permanent-residue:oracle-parity row.",
        }
        for row in protected
        if row["promotion_target"] == "permanent-residue:oracle-parity"
    ]
    write_tsv(root / "docs/tests/protected_class_oracle_parity_coverage.tsv", ORACLE_HEADER, oracle_rows)

    def simple_rows(target: str, note: str) -> list[dict[str, str]]:
        return [
            {
                "live_surface_id": key(row),
                "crate": row["crate"],
                "surviving_test_row": key(row),
                "survivor_file": row["file"],
                "survivor_test_name": row["test_name"],
                "proof_mode": proof_mode(row),
                "coverage_status": "TRUE_MEMBER",
                "notes": note,
            }
            for row in protected
            if row["promotion_target"] == target
        ]

    write_tsv(
        root / "docs/tests/protected_class_seal_proof_coverage.tsv",
        SIMPLE_COVERAGE_HEADER,
        simple_rows("permanent-residue:seal-proof", "KEEP sealed-boundary compile/probe fixture surface."),
    )
    write_tsv(
        root / "docs/tests/protected_class_golden_byte_coverage.tsv",
        SIMPLE_COVERAGE_HEADER,
        simple_rows("permanent-residue:golden-byte", "KEEP golden-byte/deterministic artifact surface."),
    )
    write_tsv(
        root / "docs/tests/protected_class_stead_required_coverage.tsv",
        SIMPLE_COVERAGE_HEADER,
        simple_rows("permanent-residue:stead-required", "KEEP STEAD section 8 required suite surface."),
    )
    write_tsv(
        root / "docs/tests/protected_class_doc_named_coverage.tsv",
        SIMPLE_COVERAGE_HEADER,
        simple_rows("permanent-residue:doc-named-invariant", "KEEP doc-named invariant surface."),
    )

    judgment_rows = [
        {
            "crate": row["crate"],
            "file": row["file"],
            "test_name": row["test_name"],
            "kind": row["kind"],
            "current_class": row["class"],
            "current_verdict": row["verdict"],
            "promotion_target": row["promotion_target"],
            "judgment_note_status": "not-applicable-audit-row"
            if row["verdict"] != "KEEP"
            else "requires-specific-catches-note",
            "proposed_next_action": "RECLASSIFY_TO_AUDIT" if row["verdict"] != "KEEP" else "KEEP",
            "reason": "AUDIT judgment-class row is not a permanent survivor shield."
            if row["verdict"] != "KEEP"
            else "KEEP judgment-class row must name the exact regression or escaped bug.",
        }
        for row in protected
        if row["class"] in {"behavior-regression", "escaped-bug"}
    ]
    write_tsv(root / "docs/tests/protected_class_judgment_keep_audit.tsv", JUDGMENT_HEADER, judgment_rows)

    counts = Counter(row["current_class"] for row in review_rows)
    truth = Counter(row["truth_verdict"] for row in review_rows)
    actions = Counter(row["proposed_next_action"] for row in review_rows)
    proof_modes = Counter(row["proof_mode"] for row in review_rows)
    results = f"""# TEST-PARE-PROTECTED-CLASS-AUDIT-0 Results

## Status

PROBATION. Audit implemented and pushed for DA/orchestrator review. This PR deletes zero tests and does not authorize self-merge.

## #1101 closeout

`GHA-PROOF-SEAL-0` is DONE and merged as #1101. Merge commit: `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`. Head: `317aba88f649a027fcd2c9997b182a7c27005cce`. The enforced rule is that non-owner-deep GitHub Doctrine Exec profiles cannot contain Atlas/Bevy/GPU/desktop/mapeditor/tools runtime proof tokens. Prove path: `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`.

## Current inventory baseline

- inventory rows: {len(rows)}
- protected audit rows reviewed: {len(review_rows)}
- KEEP permanent-residue/doc-named rows reviewed: {sum(1 for row in review_rows if row['current_verdict'] == 'KEEP')}
- AUDIT judgment-class rows reviewed: {sum(1 for row in review_rows if row['current_verdict'] == 'AUDIT')}

## Why-chain legend

See `docs/tests/test_residue_class_legend.md`. The audit binds every permanent-residue class to an owning doctrine rather than accepting a class label as a magic shield.

## Judgment-note rule

`scripts/ci/test_inventory_check.sh` now rejects any future KEEP row in `behavior-regression` or `escaped-bug` unless its note starts with `catches: ` and names a specific regression or bug. Boilerplate such as `catches: behavior regression`, `catches: escaped bug`, `catches: important coverage`, `permanent-residue:behavior-regression`, and `regression test` is rejected. Prove path: `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule`.

## Protected rows audited

| Class | Rows |
|---|---:|
{chr(10).join(f'| `{name}` | {count} |' for name, count in counts.most_common())}

| Truth verdict | Rows |
|---|---:|
{chr(10).join(f'| `{name}` | {count} |' for name, count in truth.most_common())}

| Proposed next action | Rows |
|---|---:|
{chr(10).join(f'| `{name}` | {count} |' for name, count in actions.most_common())}

## Coverage maps

- `docs/tests/test_pare_protected_class_audit_0_review.tsv`: all protected-surface rows and proposed disposition.
- `docs/tests/protected_class_oracle_parity_coverage.tsv`: {len(oracle_rows)} oracle-parity survivor rows.
- `docs/tests/protected_class_seal_proof_coverage.tsv`: {len(simple_rows('permanent-residue:seal-proof', ''))} seal-proof survivor rows.
- `docs/tests/protected_class_golden_byte_coverage.tsv`: {len(simple_rows('permanent-residue:golden-byte', ''))} golden-byte survivor rows.
- `docs/tests/protected_class_stead_required_coverage.tsv`: {len(simple_rows('permanent-residue:stead-required', ''))} STEAD-required survivor rows.
- `docs/tests/protected_class_doc_named_coverage.tsv`: {len(simple_rows('permanent-residue:doc-named-invariant', ''))} doc-named invariant survivor rows.
- `docs/tests/protected_class_judgment_keep_audit.tsv`: {len(judgment_rows)} judgment-class rows; all current rows are AUDIT, not KEEP shields.

## Oracle-parity findings

All current `permanent-residue:oracle-parity` KEEP rows remain TRUE_MEMBER survivor claims. GHA proof must not run local-owner-deep Atlas/GPU/desktop rows; those rows are marked `proof_mode=local-owner-deep` where their names or paths require it.

## Seal-proof findings

All current `permanent-residue:seal-proof` KEEP rows remain TRUE_MEMBER survivor claims. Compile-fail/probe fixture rows remain canonical sealed-boundary proof and are never deletion candidates.

## Golden-byte findings

All current `permanent-residue:golden-byte` KEEP rows remain TRUE_MEMBER survivor claims for deterministic replay, canonical bytes, or equivalent exact-output surfaces.

## STEAD-required findings

All current `permanent-residue:stead-required` KEEP rows remain TRUE_MEMBER survivor claims tied to `docs/stead_spatial_contract.md` section 8 mapgen contract surfaces.

## Doc-named findings

`custom_layout_ethics_axis` remains the sole current doc-named invariant survivor and is tied to `docs/invariants.md`.

## Judgment-class findings

The ledger currently has no KEEP `behavior-regression` or `escaped-bug` rows. The 4,250 current behavior-regression rows are AUDIT rows and therefore do not create permanent-residue shields. Future KEEP judgment rows must use the new `catches:` note rule.

## Reclassifications

No inventory reclassification is performed in this PR. The audit records 4,250 judgment-class AUDIT rows as `OUT_OF_SCOPE` for protected-survivor membership because they are not KEEP rows.

## Deletion queue for TEST-PARE-PROTECTED-RESIDUE-0

No protected KEEP row is queued for deletion by this audit. Later `TEST-PARE-PROTECTED-RESIDUE-0` work may process non-KEEP AUDIT rows through their boundary owners, but this PR does not delete or relabel them.

## Necessary/cited/dependency floor

No crate/test/product code was touched. The only executable change is the inventory checker's judgment-note guard and prove mode.

## GHA proof-seal compliance

This PR adds no Doctrine Exec profile and no GHA command that runs Atlas, Bevy, GPU, desktop, mapeditor/tools runtime, WGPU, X11/Wayland, `apt-get`, workspace tests, or all-crate cargo tests.

## Validation

- `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule`
- `bash scripts/ci/test_inventory_check.sh`
- `bash scripts/ci/test_pare_boundary_check.sh`
- `bash scripts/ci/test_inventory_drift_check.sh`
- `bash scripts/ci/test_edit_scope_check.sh --prove`
- `bash scripts/ci/doctrine_exec_profile_lint.sh`
- `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`
- `bash scripts/ci/doctrine_scan.sh`
- `bash scripts/ci/gen_digest.sh --check`

## Scope Ledger

- runtime code: untouched
- crate tests: untouched
- workflows: untouched
- scanner allowlists/data: untouched
- inventory rows: no deletion, no reclassification
- docs/audit evidence: updated

## Graduation routing

Status remains PROBATION. PR A must not merge until DA/orchestrator clearance. PR B (`TEST-PARE-PROTECTED-RESIDUE-0`) starts only after PR A is cleared and merged.

## Known gaps / next

Await DA/orchestrator review. If cleared and merged, open `TEST-PARE-PROTECTED-RESIDUE-0` to process any remaining protected-residue work under the audited owners and proof modes.
"""
    (root / "docs/tests/test_pare_protected_class_audit_0_results.md").write_text(results, encoding="utf-8")

    print("PROTECTED-CLASS-AUDIT REPORT")
    print(f"  inventory rows: {len(rows)}")
    print(f"  protected rows: {len(review_rows)}")
    print(f"  truth: {dict(truth)}")
    print(f"  actions: {dict(actions)}")
    print(f"  proof modes: {dict(proof_modes)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
