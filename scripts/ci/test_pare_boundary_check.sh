#!/usr/bin/env bash
# TEST-PARE-STANDARD-DA-0: validate boundary-keyed Track D paring authority.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BOUNDARIES="${ROOT}/scripts/ci/test_pare_boundaries.tsv"
BOUNDARY_ROWS="${ROOT}/scripts/ci/test_pare_boundary_rows.tsv"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"
AUDIT="${ROOT}/scripts/ci/test_pare_audit.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$BOUNDARIES" "$BOUNDARY_ROWS" "$INVENTORY" "$AUDIT"
import csv
import pathlib
import sys
from collections import Counter

boundaries_path = pathlib.Path(sys.argv[1])
boundary_rows_path = pathlib.Path(sys.argv[2])
inventory_path = pathlib.Path(sys.argv[3])
audit_path = pathlib.Path(sys.argv[4])

boundary_header = [
    "boundary_id",
    "boundary_tier",
    "owner",
    "boundary_kind",
    "owning_artifact",
    "representative_requirement",
    "representative",
    "retirement_policy",
    "notes",
]
row_header = [
    "crate",
    "file",
    "test_name",
    "kind",
    "current_class",
    "boundary_id",
    "boundary_tier",
    "recommended_disposition",
    "representative_to_keep",
    "consolidation_target",
    "promotion_required",
    "confidence",
    "note",
]

allowed_tiers = {
    "TIER1_TYPE_SEAL",
    "TIER2_ADMISSION_HARD_ERROR",
    "TIER3_DOCTRINE_SCAN",
    "TIER4_CLASSIFIER_CONSOLIDATION",
    "TIER5_BEHAVIOR_REGRESSION",
    "TIER6_PROMOTION_REQUIRED",
    "TIER7_NEVER_PARE",
}
allowed_kinds = {
    "type-newtype",
    "kernel-seal",
    "forbid-unsafe",
    "hydration-span-error",
    "parser-span-error",
    "import-admission-error",
    "doctrine-scan",
    "semantic-free-scan",
    "surface-scan",
    "forge-pattern-scan",
    "classifier-input-family",
    "cpu-oracle",
    "golden-byte",
    "stead-required",
    "escaped-bug-regression",
    "unknown-unowned-invariant",
    "usecase-superseded-owner",
}
allowed_policies = {
    "zero-runtime-representatives",
    "one-negative-representative",
    "scan-self-test",
    "table-driven-consolidation",
    "keep-terminal-proof",
    "promote-missing-boundary",
    "keep-behavior-regression",
    "retire-superseded-usecase",
}
allowed_dispositions = {
    "DELETE",
    "COLLAPSE_TO_REPRESENTATIVE",
    "CONSOLIDATE_TO_TABLE",
    "KEEP",
    "PROMOTION_REQUIRED",
    "NEVER_PARE",
}
allowed_confidence = {"high", "medium", "low"}
allowed_keep_targets = {
    "permanent-residue:oracle-parity",
    "permanent-residue:golden-byte",
    "permanent-residue:seal-proof",
    "permanent-residue:determinism",
    "permanent-residue:behavior-regression",
    "permanent-residue:escaped-bug",
}

errors: list[str] = []

def read_tsv(path: pathlib.Path, header: list[str], label: str) -> list[dict[str, str]]:
    if not path.exists():
        errors.append(f"missing {label}: {path}")
        return []
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != header:
            errors.append(f"{label} bad header: {reader.fieldnames!r}")
            return []
        return list(reader)

boundaries = read_tsv(boundaries_path, boundary_header, "boundary table")
rows = read_tsv(boundary_rows_path, row_header, "boundary-row table")

with inventory_path.open("r", encoding="utf-8", newline="") as f:
    inventory = list(csv.DictReader(f, delimiter="\t"))

inventory_keys = {
    (row["crate"], row["file"], row["test_name"], row["kind"]): row
    for row in inventory
}

historical_pared_keys: set[tuple[str, str, str, str]] = set()
if audit_path.exists():
    with audit_path.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f, delimiter="\t"):
            if row.get("audit_verdict") == "PARED":
                historical_pared_keys.add((row["crate"], row["file"], row["test_name"], row["kind"]))

boundary_by_id: dict[str, dict[str, str]] = {}
for line_no, row in enumerate(boundaries, start=2):
    bid = row["boundary_id"]
    if not bid:
        errors.append(f"boundary line {line_no}: missing boundary_id")
    if bid in boundary_by_id:
        errors.append(f"boundary line {line_no}: duplicate boundary_id {bid}")
    boundary_by_id[bid] = row
    if row["boundary_tier"] not in allowed_tiers:
        errors.append(f"boundary line {line_no}: invalid tier {row['boundary_tier']}")
    if row["boundary_kind"] not in allowed_kinds:
        errors.append(f"boundary line {line_no}: invalid kind {row['boundary_kind']}")
    if row["retirement_policy"] not in allowed_policies:
        errors.append(f"boundary line {line_no}: invalid retirement_policy {row['retirement_policy']}")
    for field in ("owner", "owning_artifact", "representative_requirement", "retirement_policy", "notes"):
        if not row[field].strip():
            errors.append(f"boundary line {line_no}: empty {field}")

row_seen: set[tuple[str, str, str, str]] = set()
disposition_counts: Counter[str] = Counter()
tier_counts: Counter[str] = Counter()
module_total = 0
module_mapped = 0
module_promoted = 0
module_never_pare = 0
active_tp_protected = 0

def is_never_pare_inventory(row: dict[str, str]) -> bool:
    return (
        row["kind"] in {"compile_fail", "trybuild"}
        or row["class"] in {"seal-proof", "oracle-parity", "golden-byte", "invariant-required", "stead-required"}
        or row["test_name"] == "custom_layout_ethics_axis"
    )

def is_active_tp(row: dict[str, str]) -> bool:
    hay = f"{row['file']} {row['test_name']}".lower()
    return "/tp_" in hay or hay.startswith("tp_") or " tp_" in hay or "terran_pirate" in hay or "terran-pirate" in hay

for line_no, row in enumerate(rows, start=2):
    key = (row["crate"], row["file"], row["test_name"], row["kind"])
    inv = inventory_keys.get(key)
    historical = key in historical_pared_keys
    if inv is None and not historical:
        errors.append(f"boundary-row line {line_no}: references neither live inventory nor historical PARED row {key}")
        continue
    if inv is not None:
        if key in row_seen:
            errors.append(f"boundary-row line {line_no}: duplicate live inventory mapping {key}")
        row_seen.add(key)
        if row["current_class"] != inv["class"]:
            errors.append(
                f"boundary-row line {line_no}: current_class {row['current_class']} != inventory {inv['class']}"
            )
        if inv.get("superseding_boundary") != row["boundary_id"]:
            errors.append(
                f"boundary-row line {line_no}: inventory superseding_boundary {inv.get('superseding_boundary')} "
                f"!= boundary_id {row['boundary_id']}"
            )

    bid = row["boundary_id"]
    boundary = boundary_by_id.get(bid)
    if not bid:
        errors.append(f"boundary-row line {line_no}: missing boundary_id")
        continue
    if boundary is None:
        errors.append(f"boundary-row line {line_no}: unknown boundary_id {bid}")
        continue
    if row["boundary_tier"] != boundary["boundary_tier"]:
        errors.append(f"boundary-row line {line_no}: tier {row['boundary_tier']} != boundary tier {boundary['boundary_tier']}")

    disposition = row["recommended_disposition"]
    disposition_counts[disposition] += 1
    tier_counts[row["boundary_tier"]] += 1
    if disposition not in allowed_dispositions:
        errors.append(f"boundary-row line {line_no}: invalid disposition {disposition}")
    if row["confidence"] not in allowed_confidence:
        errors.append(f"boundary-row line {line_no}: invalid confidence {row['confidence']}")

    if disposition == "DELETE":
        if row["boundary_tier"] not in {"TIER1_TYPE_SEAL", "TIER3_DOCTRINE_SCAN"} and boundary["boundary_kind"] != "usecase-superseded-owner":
            errors.append(f"boundary-row line {line_no}: DELETE lacks Tier 1, Tier 3, or usecase-superseded owner")
    if disposition == "COLLAPSE_TO_REPRESENTATIVE":
        if row["boundary_tier"] != "TIER2_ADMISSION_HARD_ERROR":
            errors.append(f"boundary-row line {line_no}: COLLAPSE row is not Tier 2")
        if not row["representative_to_keep"].strip():
            errors.append(f"boundary-row line {line_no}: COLLAPSE row lacks representative_to_keep")
    if disposition == "CONSOLIDATE_TO_TABLE":
        if row["boundary_tier"] != "TIER4_CLASSIFIER_CONSOLIDATION":
            errors.append(f"boundary-row line {line_no}: CONSOLIDATE row is not Tier 4")
        if not row["consolidation_target"].strip():
            errors.append(f"boundary-row line {line_no}: CONSOLIDATE row lacks consolidation_target")
    if disposition == "PROMOTION_REQUIRED" and not row["promotion_required"].strip():
        errors.append(f"boundary-row line {line_no}: PROMOTION_REQUIRED row lacks missing boundary")
    if disposition == "NEVER_PARE" and row["boundary_tier"] != "TIER7_NEVER_PARE":
        errors.append(f"boundary-row line {line_no}: NEVER_PARE row is not Tier 7")

    if inv is not None and is_never_pare_inventory(inv) and disposition != "NEVER_PARE":
        errors.append(f"boundary-row line {line_no}: never-pare inventory row is {disposition}: {key}")

    if inv is not None and inv.get("verdict") == "KEEP":
        target = inv.get("promotion_target", "").strip()
        if target not in allowed_keep_targets and not target.startswith("promotion-target:"):
            errors.append(f"boundary-row line {line_no}: inventory KEEP row lacks legal promotion_target: {key}")
        if inv["crate"] in {"simthing-kernel", "simthing-sim"} and target not in allowed_keep_targets:
            errors.append(f"boundary-row line {line_no}: kernel/sim KEEP row is not permanent-residue: {key}")

    if row["test_name"].startswith("cfg_test_mod::"):
        module_total += 1
        marker_note = f"{row['promotion_required']} {row['note']}"
        if disposition == "PROMOTION_REQUIRED":
            if "mapped_to_child_inventory" in marker_note:
                module_mapped += 1
            elif "mechanical_expansion_required" in marker_note:
                module_promoted += 1
            else:
                errors.append(f"boundary-row line {line_no}: module-marker PROMOTION_REQUIRED lacks explicit expansion reason")
        elif disposition == "NEVER_PARE":
            module_never_pare += 1
        else:
            errors.append(f"boundary-row line {line_no}: module-marker row is neither PROMOTION_REQUIRED nor NEVER_PARE")

    active_ref = inv if inv is not None else row
    if is_active_tp(active_ref):
        if disposition not in {"KEEP", "NEVER_PARE"}:
            errors.append(f"boundary-row line {line_no}: active TP row is not protected: {key} -> {disposition}")
        else:
            active_tp_protected += 1

missing_live = sorted(set(inventory_keys) - row_seen)
if missing_live:
    errors.append(f"boundary-row table missing {len(missing_live)} live inventory rows; first={missing_live[:5]}")

rows_missing_owner = sum(1 for row in rows if not row["boundary_id"].strip())

print("TEST-PARE-BOUNDARY-CHECK REPORT")
print(f"  boundary rows: {len(boundaries)}")
print(f"  boundary-row mappings: {len(rows)}")
print(f"  live inventory rows: {len(inventory)}")
print(f"  live rows with owning boundary: {len(row_seen)}")
print(f"  live rows missing owning boundary: {len(missing_live) + rows_missing_owner}")
print(f"  historical PARED rows mapped: {sum(1 for row in rows if (row['crate'], row['file'], row['test_name'], row['kind']) in historical_pared_keys)}")
print(f"  Tier 1 DELETE candidates: {sum(1 for row in rows if row['boundary_tier'] == 'TIER1_TYPE_SEAL' and row['recommended_disposition'] == 'DELETE')}")
print(f"  Tier 2 COLLAPSE candidates: {sum(1 for row in rows if row['recommended_disposition'] == 'COLLAPSE_TO_REPRESENTATIVE')}")
print(f"  Tier 3 DELETE candidates: {sum(1 for row in rows if row['boundary_tier'] == 'TIER3_DOCTRINE_SCAN' and row['recommended_disposition'] == 'DELETE')}")
print(f"  Tier 4 CONSOLIDATE candidates: {sum(1 for row in rows if row['recommended_disposition'] == 'CONSOLIDATE_TO_TABLE')}")
print(f"  NEVER_PARE rows: {disposition_counts['NEVER_PARE']}")
print(f"  PROMOTION_REQUIRED rows: {disposition_counts['PROMOTION_REQUIRED']}")
print(f"  module-marker rows expanded/mapped: {module_mapped}")
print(f"  module-marker rows never-pare protected: {module_never_pare}")
print(f"  module-marker rows promotion-required: {module_promoted}")
print(f"  module-marker generic blockers remaining: {module_total - module_mapped - module_never_pare - module_promoted}")
print(f"  active TP rows protected: {active_tp_protected}")

if errors:
    print("TEST-PARE-BOUNDARY-CHECK-VERDICT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

print("TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS")
PY
