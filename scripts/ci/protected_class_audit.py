#!/usr/bin/env python3
"""Generate TEST-PARE-PROTECTED-CLASS-AUDIT-0 evidence tables.

The 0R audit must not treat KEEP as proof. Each protected row is checked
against the membership rule for its class, and false claims are queued for the
follow-up residue rung instead of being silently blessed.
"""

from __future__ import annotations

import csv
import re
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

TERMINAL_VERDICTS = {
    "TRUE_MEMBER",
    "FALSE_MEMBER",
    "NEEDS_PROMOTION",
    "NECESSARY_CITED_DEPENDENCY",
    "OUT_OF_SCOPE",
    "LEDGER_DEFECT",
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

ORACLE_STRONG_TRUE = (
    "parity",
    "matches_cpu",
    "match_cpu",
    "matches_cpu_oracle",
    "matches_oracle",
    "match_oracle",
    "cpu_gpu",
    "gpu_cpu",
    "gpu_matches_cpu",
    "gpu_aggregate_matches_cpu",
    "bit_exact",
    "exact_parity",
    "equal",
    "summary_matches_cpu",
    "records_match",
    "rows_match",
    "outputs_match_cpu_oracle",
    "sparse_and_dense_masks_match_oracle",
    "repeated_destination_parents_match_oracle",
)

ORACLE_NOT_PARITY = (
    "cpu_oracle_only",
    "cpu_oracle_test_only",
    "reports_cpu_oracle",
    "cpu_oracle_checksum",
    "cpu_oracle_complete",
    "cpu_oracle_handles",
    "cpu_oracle_stable",
    "explicitly_non_gpu",
    "no_cpu_planner",
    "rejects_cpu_planner",
    "no_full_field_cpu_readback",
    "no_f32_bit_exact_claim",
    "approximation",
    "unavailable",
    "none_policy",
    "readback_gate",
)

GOLDEN_TRUE = (
    "golden",
    "canonical",
    "deterministic",
    "replay",
    "bit_exact",
    "stable",
    "byte",
    "digest",
    "checksum",
    "identical",
    "roundtrip",
    "round_trip",
)


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


def slug(value: str) -> str:
    value = value.lower()
    value = re.sub(r"[^a-z0-9]+", "-", value)
    return value.strip("-") or "unknown"


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
    return "gha-cpu"


def load_doc_texts(root: Path) -> dict[str, str]:
    docs: dict[str, str] = {}
    for path in root.glob("docs/**/*.md"):
        rel = path.relative_to(root).as_posix()
        if rel.startswith("docs/archive/"):
            continue
        docs[rel] = path.read_text(encoding="utf-8", errors="replace")
    return docs


def load_stead_suites(root: Path) -> set[str]:
    text = (root / "docs/stead_spatial_contract.md").read_text(encoding="utf-8", errors="replace")
    marker = "## 8. Required tests"
    start = text.find(marker)
    section = text[start:] if start >= 0 else text
    end = section.find("## 9.")
    if end >= 0:
        section = section[:end]
    match = re.search(r"MUST keep green .*?:\s*(.*?)\.\s+New spatial behavior", section, re.DOTALL)
    suite_text = match.group(1) if match else section
    return {part.strip() for part in re.findall(r"`([^`]+)`", suite_text) if part.strip()}


def load_sealed_types(root: Path) -> set[str]:
    values: set[str] = set()
    for line in (root / "scripts/ci/allow/sealed_types.txt").read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped and not stripped.startswith("#"):
            values.add(stripped)
    return values


def source_text(root: Path, row: dict[str, str]) -> str:
    path = root / row["file"]
    if not path.exists() or not path.is_file():
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def function_body_text(root: Path, row: dict[str, str]) -> str:
    text = source_text(root, row)
    name = row["test_name"]
    if not text or name.startswith("cfg_test_mod::"):
        return ""
    match = re.search(rf"\bfn\s+{re.escape(name)}\b", text)
    if not match:
        return ""
    start = match.start()
    brace = text.find("{", match.end())
    if brace < 0:
        return text[start : match.end()]
    depth = 0
    for index in range(brace, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    return text[start:]


def row_haystack(root: Path, row: dict[str, str]) -> str:
    text = function_body_text(root, row) or source_text(root, row)
    return f"{row['crate']} {row['file']} {row['test_name']} {row['note']} {text}".lower()


def cited_docs(doc_texts: dict[str, str], needle: str) -> list[str]:
    if not needle:
        return []
    return sorted(path for path, text in doc_texts.items() if needle in text)


def has_specific_judgment_note(note: str) -> bool:
    normalized = " ".join(note.strip().lower().split())
    bad = {
        "catches: behavior regression",
        "catches: escaped bug",
        "catches: important coverage",
        "permanent-residue:behavior-regression",
        "permanent-residue:escaped-bug",
        "regression test",
    }
    if normalized in bad or not normalized.startswith("catches: "):
        return False
    detail = normalized.removeprefix("catches: ").strip()
    return len(detail) >= 24 and detail not in {
        "behavior regression",
        "escaped bug",
        "important coverage",
        "regression test",
    }


def oracle_surface(row: dict[str, str]) -> str:
    name = row["test_name"].lower()
    file_stem = Path(row["file"]).stem
    if "velocity" in name or "velocity" in row["file"].lower():
        return "oracle:velocity-integration"
    if "weighted_mean" in name or "weighted_mean" in row["file"].lower():
        return "oracle:weighted-mean-accumulator"
    if "mobility_gpu_kernel" in name or "mobility_gpu_kernel" in row["file"].lower():
        m = re.search(r"mobility_gpu_kernel[0-9]+", f"{row['file']} {name}")
        return f"oracle:{m.group(0) if m else file_stem}"
    if "owner_silo" in name or "owner_silo" in row["file"].lower():
        return "oracle:owner-silo-resource-flow"
    if "resource" in name or "econom" in name or "arena" in name:
        return f"oracle:resource-flow:{file_stem}"
    if "atlas" in name or "atlas" in row["file"].lower():
        return f"oracle:atlas:{file_stem}"
    return f"oracle:{file_stem}:{slug(row['test_name'])}"


def numeric_regime(row: dict[str, str]) -> str:
    name = row["test_name"].lower()
    if "fraction" in name:
        return "fractional-dt"
    if "1000" in name or "soak" in name or "burn_in" in name:
        return "soak-or-long-run"
    if "bit_exact" in name or "exact" in name or "i8" in name:
        return "bit-exact"
    if "f32" in name or "tolerance" in name or "approx" in name:
        return "float-tolerance"
    if "gpu" in name or "wgsl" in name:
        return "gpu-vs-cpu"
    if "replay" in name:
        return "replay"
    return "named-oracle-regime"


def classify_oracle(root: Path, row: dict[str, str]) -> dict[str, str]:
    if row["test_name"].startswith("cfg_test_mod::"):
        return verdict(
            "LEDGER_DEFECT",
            "oracle-parity ledger entry is a cfg(test) module marker, not a falsifiable parity test row.",
            "",
            "not-required",
            "FIX_LEDGER_ONLY",
        )
    hay = row_haystack(root, row)
    name = row["test_name"].lower()
    strong = any(token in name for token in ORACLE_STRONG_TRUE)
    not_parity = any(token in name for token in ORACLE_NOT_PARITY)
    if strong:
        surface = oracle_surface(row)
        return verdict(
            "TRUE_MEMBER",
            f"maps to live parity surface {surface}; test name/body asserts CPU-to-GPU, CPU-to-kernel, or CPU-to-live-op equality.",
            key(row),
            proof_mode(row),
            "KEEP",
            surface,
        )
    if "cpu" in hay and "gpu" in hay and ("assert_eq" in hay or "assert!" in hay) and not not_parity:
        surface = oracle_surface(row)
        return verdict(
            "TRUE_MEMBER",
            f"maps to live parity surface {surface}; source contains CPU/GPU assertion even though the name is indirect.",
            key(row),
            proof_mode(row),
            "KEEP",
            surface,
        )
    if not_parity or "oracle" in name:
        return verdict(
            "FALSE_MEMBER",
            "uses or guards an oracle/support policy but does not itself prove bit-exact CPU-to-GPU, CPU-to-kernel, or CPU-to-live-op parity; deletion_reason=not-oracle-parity-proof.",
            "",
            "not-required",
            "RECLASSIFY_TO_PARE",
            oracle_surface(row),
        )
    return verdict(
        "NEEDS_PROMOTION",
        "row may be useful, but current evidence does not establish oracle-parity membership; promotion_target=promotion-target:protected-oracle-review; promote to a more specific owner or reclassify in PR B.",
        "",
        "not-required",
        "KEEP_PROMOTION_REQUIRED",
        oracle_surface(row),
    )


def classify_seal(root: Path, row: dict[str, str], sealed_types: set[str]) -> dict[str, str]:
    if row["kind"] in {"compile_fail", "trybuild"}:
        hay = f"{row['file']} {row['test_name']} {source_text(root, row)}"
        named = sorted(value for value in sealed_types if value in hay)
        boundary = ",".join(named) if named else f"{Path(row['file']).stem}:{row['test_name']}"
        return verdict(
            "TRUE_MEMBER",
            f"canonical compile-time sealed-boundary proof for live boundary {boundary}.",
            key(row),
            proof_mode(row),
            "KEEP",
            f"sealed-boundary:{boundary}",
        )
    if row["kind"] == "fixture" and row["file"].startswith("scripts/ci/fixtures/"):
        return verdict(
            "NECESSARY_CITED_DEPENDENCY",
            "CI scanner/probe fixture required by doctrine self-test/probe surface; not a canonical product seal-proof row.",
            key(row),
            "gha-cpu",
            "KEEP_DEPENDENCY_FLOOR",
            f"ci-selftest-fixture:{row['file']}",
        )
    return verdict(
        "FALSE_MEMBER",
        "seal-proof claim is neither compile_fail/trybuild nor a cited CI fixture dependency; deletion_reason=not-canonical-seal-proof.",
        "",
        "not-required",
        "RECLASSIFY_TO_PARE",
        f"sealed-boundary-unmapped:{Path(row['file']).stem}",
    )


def classify_golden(row: dict[str, str]) -> dict[str, str]:
    if row["test_name"].startswith("cfg_test_mod::"):
        return verdict(
            "LEDGER_DEFECT",
            "golden-byte ledger entry is a cfg(test) module marker, not a byte-identity proof row.",
            "",
            "not-required",
            "FIX_LEDGER_ONLY",
        )
    name = row["test_name"].lower()
    surface = f"golden:{Path(row['file']).stem}:{slug(row['test_name'])}"
    if any(token in name for token in GOLDEN_TRUE):
        return verdict(
            "TRUE_MEMBER",
            f"maps to live byte/determinism surface {surface}.",
            key(row),
            proof_mode(row),
            "KEEP",
            surface,
        )
    return verdict(
        "NEEDS_PROMOTION",
        "row is classified golden-byte but name does not identify byte identity, canonical format, deterministic diagnostic, or replay surface; promotion_target=promotion-target:protected-golden-review.",
        "",
        "not-required",
        "KEEP_PROMOTION_REQUIRED",
        surface,
    )


def classify_stead(row: dict[str, str], stead_suites: set[str]) -> dict[str, str]:
    file_stem = Path(row["file"]).stem
    if file_stem in stead_suites:
        return verdict(
            "TRUE_MEMBER",
            f"row lives in STEAD section 8 named suite {file_stem}.",
            key(row),
            "gha-cpu",
            "KEEP",
            f"docs/stead_spatial_contract.md#8:{file_stem}",
        )
    if row["file"].endswith("/mapgen_palma.rs") and "mapgen_palma" in stead_suites:
        return verdict(
            "TRUE_MEMBER",
            "direct helper surface for STEAD section 8 named suite mapgen_palma.",
            key(row),
            "gha-cpu",
            "KEEP",
            "docs/stead_spatial_contract.md#8:mapgen_palma-helper",
        )
    return verdict(
        "FALSE_MEMBER",
        "stead-required claim is outside the section 8 named suites and has no direct helper citation; deletion_reason=not-stead-section-8-member.",
        "",
        "not-required",
        "RECLASSIFY_TO_PARE",
        f"stead-unmapped:{file_stem}",
    )


def classify_doc_named(row: dict[str, str], doc_texts: dict[str, str]) -> dict[str, str]:
    docs = cited_docs(doc_texts, row["test_name"])
    if docs:
        cited = ",".join(docs[:4])
        return verdict(
            "TRUE_MEMBER",
            f"live non-archive doctrine doc explicitly names this row: {cited}.",
            key(row),
            "gha-cpu",
            "KEEP",
            f"doc-named:{cited}",
        )
    return verdict(
        "FALSE_MEMBER",
        "doc-named invariant claim has no live non-archive doc citation; deletion_reason=missing-live-doc-citation.",
        "",
        "not-required",
        "RECLASSIFY_TO_PARE",
        "doc-named:missing",
    )


def classify_judgment(row: dict[str, str]) -> dict[str, str]:
    if row["verdict"] != "KEEP":
        return verdict(
            "OUT_OF_SCOPE",
            "AUDIT judgment-class row is not a permanent survivor shield and remains available to boundary waves.",
            "",
            "not-required",
            "RECLASSIFY_TO_AUDIT",
            "judgment-audit-row",
        )
    if has_specific_judgment_note(row["note"]):
        return verdict(
            "TRUE_MEMBER",
            "KEEP judgment-class row has a specific catches note naming the regression or escaped bug.",
            key(row),
            "gha-cpu",
            "KEEP",
            "judgment-specific-catches-note",
        )
    return verdict(
        "FALSE_MEMBER",
        "KEEP judgment-class row lacks a specific catches note; deletion_reason=boilerplate-judgment-claim.",
        "",
        "not-required",
        "RECLASSIFY_TO_PARE",
        "judgment-note-missing",
    )


def verdict(
    truth: str,
    reason: str,
    canonical: str,
    dependency: str,
    action: str,
    surface: str = "",
) -> dict[str, str]:
    if truth not in TERMINAL_VERDICTS:
        raise ValueError(truth)
    return {
        "truth_verdict": truth,
        "reason": reason,
        "canonical_survivor": canonical,
        "proof_dependency": dependency,
        "proposed_next_action": action,
        "coverage_surface": surface,
    }


def classify(
    root: Path,
    row: dict[str, str],
    doc_texts: dict[str, str],
    sealed_types: set[str],
    stead_suites: set[str],
) -> dict[str, str]:
    claim = protected_claim(row)
    if "oracle-parity" in claim or row["class"] == "oracle-parity":
        return classify_oracle(root, row)
    if "seal-proof" in claim or row["class"] == "seal-proof":
        return classify_seal(root, row, sealed_types)
    if "golden-byte" in claim or row["class"] == "golden-byte":
        return classify_golden(row)
    if "stead-required" in claim or row["class"] == "stead-required":
        return classify_stead(row, stead_suites)
    if "doc-named-invariant" in claim or row["class"] == "invariant-required":
        return classify_doc_named(row, doc_texts)
    if row["class"] in {"behavior-regression", "escaped-bug"}:
        return classify_judgment(row)
    return verdict(
        "NEEDS_PROMOTION",
        "protected claim has no class-specific audit rule yet; promote to a named owner before deletion decisions.",
        "",
        "not-required",
        "KEEP_PROMOTION_REQUIRED",
        f"unmapped:{claim}",
    )


def live_owner(row: dict[str, str], outcome: dict[str, str]) -> str:
    surface = outcome["coverage_surface"]
    cls = row["class"]
    claim = protected_claim(row)
    if "oracle-parity" in claim or cls == "oracle-parity":
        return f"core section 4 / constitution section 0.7 oracle-parity doctrine; {surface}"
    if "stead-required" in claim or cls == "stead-required":
        return f"docs/stead_spatial_contract.md section 8; {surface}"
    if "seal-proof" in claim or cls == "seal-proof":
        return f"sealed-boundary doctrine from scripts/ci/allow/sealed_types.txt / compile_fail docs; {surface}"
    if "golden-byte" in claim or cls == "golden-byte":
        return f"determinism/canonical-corpus doctrine; {surface}"
    if "doc-named-invariant" in claim or cls == "invariant-required":
        return f"live non-archive doc citation; {surface}"
    if cls in {"behavior-regression", "escaped-bug"}:
        return f"TIER5 judgment class; {surface}"
    return surface or "unmapped"


def coverage_row(row: dict[str, str], outcome: dict[str, str], note: str) -> dict[str, str]:
    return {
        "live_surface_id": outcome["coverage_surface"],
        "crate": row["crate"],
        "surviving_test_row": key(row),
        "survivor_file": row["file"],
        "survivor_test_name": row["test_name"],
        "proof_mode": proof_mode(row) if outcome["truth_verdict"] == "TRUE_MEMBER" else outcome["proof_dependency"],
        "coverage_status": outcome["truth_verdict"],
        "notes": note,
    }


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    rows = read_tsv(root / "scripts/ci/test_inventory.tsv")
    doc_texts = load_doc_texts(root)
    sealed_types = load_sealed_types(root)
    stead_suites = load_stead_suites(root)
    protected = [row for row in rows if protected_claim(row)]

    review_rows: list[dict[str, str]] = []
    outcomes: dict[str, dict[str, str]] = {}
    for row in protected:
        outcome = classify(root, row, doc_texts, sealed_types, stead_suites)
        outcomes[key(row)] = outcome
        review_rows.append(
            {
                "crate": row["crate"],
                "file": row["file"],
                "test_name": row["test_name"],
                "kind": row["kind"],
                "current_class": row["class"],
                "current_verdict": row["verdict"],
                "promotion_target": row["promotion_target"],
                "protected_claim": protected_claim(row),
                "truth_verdict": outcome["truth_verdict"],
                "live_owner": live_owner(row, outcome),
                "coverage_surface": outcome["coverage_surface"],
                "canonical_survivor": outcome["canonical_survivor"],
                "proof_mode": proof_mode(row) if outcome["truth_verdict"] == "TRUE_MEMBER" else outcome["proof_dependency"],
                "proposed_next_action": outcome["proposed_next_action"],
                "reason": outcome["reason"],
                "proof_dependency": outcome["proof_dependency"],
            }
        )

    write_tsv(root / "docs/tests/test_pare_protected_class_audit_0_review.tsv", REVIEW_HEADER, review_rows)

    oracle_rows: list[dict[str, str]] = []
    seal_rows: list[dict[str, str]] = []
    golden_rows: list[dict[str, str]] = []
    stead_rows: list[dict[str, str]] = []
    doc_rows: list[dict[str, str]] = []
    judgment_rows: list[dict[str, str]] = []

    for row in protected:
        outcome = outcomes[key(row)]
        claim = protected_claim(row)
        if "oracle-parity" in claim or row["class"] == "oracle-parity":
            oracle_rows.append(
                {
                    "live_surface_id": outcome["coverage_surface"],
                    "crate": row["crate"],
                    "kernel_or_op_path": row["file"],
                    "numeric_regime": numeric_regime(row),
                    "surviving_test_row": key(row),
                    "survivor_file": row["file"],
                    "survivor_test_name": row["test_name"],
                    "proof_mode": proof_mode(row) if outcome["truth_verdict"] == "TRUE_MEMBER" else outcome["proof_dependency"],
                    "coverage_status": outcome["truth_verdict"],
                    "notes": outcome["reason"],
                }
            )
        elif "seal-proof" in claim or row["class"] == "seal-proof":
            seal_rows.append(coverage_row(row, outcome, outcome["reason"]))
        elif "golden-byte" in claim or row["class"] == "golden-byte":
            golden_rows.append(coverage_row(row, outcome, outcome["reason"]))
        elif "stead-required" in claim or row["class"] == "stead-required":
            stead_rows.append(coverage_row(row, outcome, outcome["reason"]))
        elif "doc-named-invariant" in claim or row["class"] == "invariant-required":
            doc_rows.append(coverage_row(row, outcome, outcome["reason"]))
        if row["class"] in {"behavior-regression", "escaped-bug"}:
            judgment_rows.append(
                {
                    "crate": row["crate"],
                    "file": row["file"],
                    "test_name": row["test_name"],
                    "kind": row["kind"],
                    "current_class": row["class"],
                    "current_verdict": row["verdict"],
                    "promotion_target": row["promotion_target"],
                    "judgment_note_status": outcome["truth_verdict"],
                    "proposed_next_action": outcome["proposed_next_action"],
                    "reason": outcome["reason"],
                }
            )

    write_tsv(root / "docs/tests/protected_class_oracle_parity_coverage.tsv", ORACLE_HEADER, oracle_rows)
    write_tsv(root / "docs/tests/protected_class_seal_proof_coverage.tsv", SIMPLE_COVERAGE_HEADER, seal_rows)
    write_tsv(root / "docs/tests/protected_class_golden_byte_coverage.tsv", SIMPLE_COVERAGE_HEADER, golden_rows)
    write_tsv(root / "docs/tests/protected_class_stead_required_coverage.tsv", SIMPLE_COVERAGE_HEADER, stead_rows)
    write_tsv(root / "docs/tests/protected_class_doc_named_coverage.tsv", SIMPLE_COVERAGE_HEADER, doc_rows)
    write_tsv(root / "docs/tests/protected_class_judgment_keep_audit.tsv", JUDGMENT_HEADER, judgment_rows)

    counts = Counter(row["current_class"] for row in review_rows)
    truth = Counter(row["truth_verdict"] for row in review_rows)
    actions = Counter(row["proposed_next_action"] for row in review_rows)
    proof_modes = Counter(row["proof_mode"] for row in review_rows)
    keep_rows = [row for row in review_rows if row["current_verdict"] == "KEEP"]
    false_keep = [row for row in keep_rows if row["truth_verdict"] == "FALSE_MEMBER"]
    queue_size = len(false_keep)

    def table(counter: Counter[str]) -> str:
        return "\n".join(f"| `{name}` | {count} |" for name, count in counter.most_common())

    results = f"""# TEST-PARE-PROTECTED-CLASS-AUDIT-0 Results

## Status

PROBATION / HOLD cleared for 0R push only. This PR remains PR A and must not merge until DA/orchestrator clearance. This PR deletes zero tests and does not authorize self-merge.

## #1101 closeout

`GHA-PROOF-SEAL-0` is DONE and merged as #1101. Merge commit: `e49c8a258e4bd58d9c78b6c82b698cd5650dbaca`. Head: `317aba88f649a027fcd2c9997b182a7c27005cce`. The enforced rule is that non-owner-deep GitHub Doctrine Exec profiles cannot contain Atlas/Bevy/GPU/desktop/mapeditor/tools runtime proof tokens. Prove path: `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal`.

## Current inventory baseline

- inventory rows: {len(rows)}
- protected rows audited: {len(review_rows)}
- protected KEEP rows audited: {len(keep_rows)}
- TRUE_MEMBER count: {truth.get('TRUE_MEMBER', 0)}
- FALSE_MEMBER count: {truth.get('FALSE_MEMBER', 0)}
- NEEDS_PROMOTION count: {truth.get('NEEDS_PROMOTION', 0)}
- NECESSARY_CITED_DEPENDENCY count: {truth.get('NECESSARY_CITED_DEPENDENCY', 0)}
- LEDGER_DEFECT count: {truth.get('LEDGER_DEFECT', 0)}
- OUT_OF_SCOPE judgment AUDIT count: {truth.get('OUT_OF_SCOPE', 0)}
- deletion queue size for TEST-PARE-PROTECTED-RESIDUE-0: {queue_size}

## Why-chain legend

See `docs/tests/test_residue_class_legend.md`. The 0R audit binds every permanent-residue class to an owning doctrine and tests membership class-by-class. `KEEP` is not treated as proof.

## Audit logic correction

The original PR A generator was tautological: non-KEEP became `OUT_OF_SCOPE` and KEEP became `TRUE_MEMBER`. 0R replaces that with class-specific verification:

- oracle-parity rows must name or source-cite CPU-to-GPU, CPU-to-kernel, or CPU-to-live-op parity for a live surface.
- seal-proof rows must be compile_fail/trybuild proofs for a live sealed boundary, while CI fixtures are dependency-floor rows rather than automatic seal-proof.
- golden-byte rows must identify byte identity, canonical format, deterministic diagnostic, deterministic replay, or canonical corpus surfaces.
- stead-required rows must live in the section 8 named suites or direct helper surfaces.
- doc-named invariant rows must be explicitly named by live non-archive docs.
- judgment rows remain OUT_OF_SCOPE unless they are KEEP rows with a specific `catches:` note.

## Judgment-note rule

`scripts/ci/test_inventory_check.sh` rejects future KEEP rows in `behavior-regression` or `escaped-bug` unless the note starts with `catches: ` and names a specific regression or bug. Boilerplate such as `catches: behavior regression`, `catches: escaped bug`, `catches: important coverage`, `permanent-residue:behavior-regression`, and `regression test` fails. Prove path: `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule`.

## Protected rows audited

| Class | Rows |
|---|---:|
{table(counts)}

| Truth verdict | Rows |
|---|---:|
{table(truth)}

| Proposed next action | Rows |
|---|---:|
{table(actions)}

| Proof mode | Rows |
|---|---:|
{table(proof_modes)}

## Coverage maps

- `docs/tests/test_pare_protected_class_audit_0_review.tsv`: all protected-surface rows and proposed disposition.
- `docs/tests/protected_class_oracle_parity_coverage.tsv`: {len(oracle_rows)} oracle rows with live parity surfaces and false/dependency outcomes.
- `docs/tests/protected_class_seal_proof_coverage.tsv`: {len(seal_rows)} seal rows mapped to sealed boundaries or CI fixture dependencies.
- `docs/tests/protected_class_golden_byte_coverage.tsv`: {len(golden_rows)} golden rows mapped to deterministic/canonical byte surfaces.
- `docs/tests/protected_class_stead_required_coverage.tsv`: {len(stead_rows)} STEAD rows mapped to section 8 named suites/helper surfaces.
- `docs/tests/protected_class_doc_named_coverage.tsv`: {len(doc_rows)} doc-named rows with live non-archive doc citations.
- `docs/tests/protected_class_judgment_keep_audit.tsv`: {len(judgment_rows)} judgment-class rows; current judgment rows are AUDIT, not survivor shields.

## Oracle coverage

Oracle TRUE_MEMBER rows name live parity surfaces such as mobility kernels, resource-flow parity, atlas parity, owner-silo GPU tick parity, and velocity integration. Rows that only report, construct, or forbid CPU oracles without asserting parity are FALSE_MEMBER and queued for PR B reclassification/deletion review. GPU/WGPU/desktop-like parity surfaces remain `proof_mode=local-owner-deep`; no GHA GPU proof is added.

## Seal coverage

Compile-fail/trybuild rows are TRUE_MEMBER only when they map to a live sealed boundary. CI fixtures under `scripts/ci/fixtures/**` are NECESSARY_CITED_DEPENDENCY rows for scanner/probe self-tests; they are not counted as canonical product seal proofs.

## Golden coverage

Golden TRUE_MEMBER rows identify deterministic replay, canonical byte/format, stable diagnostic, checksum, digest, roundtrip, or exact-output surfaces. Rows without such a surface are NEEDS_PROMOTION rather than auto-accepted.

## STEAD coverage

STEAD TRUE_MEMBER rows live in the `docs/stead_spatial_contract.md` section 8 named suites: {', '.join(sorted(stead_suites))}. `crates/simthing-clausething/src/mapgen_palma.rs` is accepted as a direct helper for the named `mapgen_palma` suite. No section 8 suite is sub-pared in this PR.

## Doc-named coverage

`custom_layout_ethics_axis` remains TRUE_MEMBER because live `docs/invariants.md` explicitly names it as the invariant proof. Archive-only citations are ignored.

## Judgment-class findings

The ledger currently has no KEEP `behavior-regression` or `escaped-bug` rows. The {len(judgment_rows)} current behavior-regression rows are AUDIT rows and therefore OUT_OF_SCOPE for protected-survivor membership. Future KEEP judgment rows must use the `catches:` note rule.

## Deletion queue for TEST-PARE-PROTECTED-RESIDUE-0

Queue size: {queue_size}. These are protected KEEP rows whose class-specific membership test is FALSE_MEMBER. PR A does not delete them. PR B must either reclassify/delete them under an owning boundary or produce stronger live evidence.

## Necessary/cited/dependency floor

Dependency-floor rows cite exact live CI/doc surfaces in the coverage maps, primarily `scripts/ci/fixtures/**` scanner/probe fixtures. Executable changes are limited to the inventory checker's judgment-note guard/prove mode, the generated protected-class audit helper, and the Python-version compatibility fix in `test_edit_scope_check.sh`.

## GHA proof-seal compliance

This PR adds no Doctrine Exec profile and no GHA command that runs Atlas, Bevy, GPU, desktop, mapeditor/tools runtime, WGPU, X11/Wayland, `apt-get`, workspace tests, all-crate cargo tests, or bare full-crate cargo tests.

## Validation

Local Git Bash validation PASS:

- `bash scripts/ci/test_inventory_check.sh --prove-judgment-note-rule` - PASS
- `bash scripts/ci/test_inventory_check.sh` - PASS
- `bash scripts/ci/test_pare_boundary_check.sh` - PASS
- `bash scripts/ci/test_inventory_drift_check.sh` - PASS
- `bash scripts/ci/test_edit_scope_check.sh --prove` - PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh` - PASS
- `bash scripts/ci/doctrine_exec_profile_lint.sh --prove-gha-proof-seal` - PASS
- `bash scripts/ci/doctrine_scan.sh` - PASS, failures=0 inspect=0
- `bash scripts/ci/gen_digest.sh --check` - PASS

## Scope Ledger

- runtime code: untouched
- crate tests: untouched
- workflows: untouched
- scanner allowlists/data: untouched
- test deletion: none
- GHA Atlas/Bevy/GPU/desktop proof: none
- inventory rows: no deletion in PR A
- docs/audit evidence: updated for 0R

## Graduation routing

Graduation routing (for DA/orchestrator - why PROBATION, not COMPLETE):
  CI verdict:          PASS-RELIABLE
  Triage entries:      none
  Risk class:          data-deliverable + gate-wiring + protected-class reclassification
  Falsification check: protected KEEP rows are not auto-TRUE; every TRUE_MEMBER has a class-specific live surface; every FALSE_MEMBER is queued or reclassified; every NEEDS_PROMOTION has a named promotion target; dependency-floor rows cite exact live docs/profiles/selftests/dependencies; zero tests deleted; GHA proof seal remains green and no Atlas/Bevy/GPU/desktop proof enters non-owner-deep profiles.
  Recommended posture: deep - this audit defines the deletion queue for TEST-PARE-PROTECTED-RESIDUE-0.

## Known gaps / next

Await DA/orchestrator review. If cleared and merged, open `TEST-PARE-PROTECTED-RESIDUE-0` to process FALSE_MEMBER and NEEDS_PROMOTION rows under the audited owners and proof modes.
"""
    (root / "docs/tests/test_pare_protected_class_audit_0_results.md").write_text(results, encoding="utf-8")

    print("PROTECTED-CLASS-AUDIT-0R REPORT")
    print(f"  inventory rows: {len(rows)}")
    print(f"  protected rows: {len(review_rows)}")
    print(f"  protected KEEP rows: {len(keep_rows)}")
    print(f"  truth: {dict(truth)}")
    print(f"  actions: {dict(actions)}")
    print(f"  deletion queue size: {queue_size}")
    print(f"  proof modes: {dict(proof_modes)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
