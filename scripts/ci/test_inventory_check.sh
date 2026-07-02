#!/usr/bin/env bash
# TEST-PARE-INVENTORY-0: validate the checked-in test corpus inventory.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY"
import csv
import pathlib
import re
import sys

root = pathlib.Path(sys.argv[1])
inventory = pathlib.Path(sys.argv[2])

required = [
    "crate",
    "file",
    "test_name",
    "kind",
    "class",
    "superseding_boundary",
    "verdict",
    "note",
]
allowed_kind = {"unit", "integration", "doc", "compile_fail", "trybuild", "fixture", "unknown"}
allowed_class = {
    "behavior-regression",
    "oracle-parity",
    "seal-proof",
    "golden-byte",
    "invariant-required",
    "stead-required",
    "admission-superseded",
    "admission-adjacent",
    "usecase-superseded",
    "duplicate-battery",
    "hygiene-theater",
    "unknown",
}
allowed_verdict = {"KEEP", "PARE", "AUDIT"}
collapse_re = re.compile(r"^COLLAPSE\([0-9]+(?:->|→)1\)$")

test_attr_re = re.compile(r"#\[\s*(?:(?:tokio|async_std)::)?test(?:\(|\])")
fn_re = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)")
cfg_test_re = re.compile(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]")
mod_re = re.compile(r"\bmod\s+([A-Za-z_][A-Za-z0-9_]*)")

def crate_for(path: pathlib.Path) -> str:
    parts = path.parts
    if len(parts) >= 2 and parts[0] == "crates":
        return parts[1]
    if len(parts) >= 2 and parts[0] == "scripts" and parts[1] == "ci":
        return "scripts-ci"
    return "unknown"

def norm(path: pathlib.Path) -> str:
    return path.as_posix()

def rust_files() -> list[pathlib.Path]:
    files: set[pathlib.Path] = set()
    for pattern in ("crates/*/src/**/*.rs", "crates/*/tests/**/*.rs", "crates/*/benches/**/*.rs"):
        files.update(root.glob(pattern))
    return sorted(path.relative_to(root) for path in files)

def discovered_items() -> set[tuple[str, str, str, str]]:
    items: set[tuple[str, str, str, str]] = set()
    for rel in rust_files():
        text = (root / rel).read_text(encoding="utf-8", errors="replace").splitlines()
        file_kind = "unit" if "/src/" in f"/{rel.as_posix()}/" else "integration"
        for index, line in enumerate(text):
            if test_attr_re.search(line):
                name = None
                for later in text[index : min(index + 8, len(text))]:
                    m = fn_re.search(later)
                    if m:
                        name = m.group(1)
                        break
                if name:
                    items.add((crate_for(rel), norm(rel), name, file_kind))
            if cfg_test_re.search(line):
                name = None
                for later in text[index : min(index + 8, len(text))]:
                    m = mod_re.search(later)
                    if m:
                        name = f"cfg_test_mod::{m.group(1)}"
                        break
                if name:
                    items.add((crate_for(rel), norm(rel), name, "unit"))
            if "```compile_fail" in line:
                items.add((crate_for(rel), norm(rel), f"compile_fail_line_{index + 1}", "compile_fail"))
            if "trybuild::TestCases" in line or ".compile_fail(" in line:
                items.add((crate_for(rel), norm(rel), f"trybuild_line_{index + 1}", "trybuild"))
    fixtures = sorted((root / "scripts/ci/fixtures").glob("**/*"))
    for path in fixtures:
        if path.is_file():
            rel = path.relative_to(root)
            items.add(("scripts-ci", norm(rel), rel.name, "fixture"))
    return items

errors: list[str] = []
inspect: list[str] = []

if not inventory.exists():
    errors.append(f"missing inventory {inventory}")
else:
    with inventory.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != required:
            errors.append(f"bad header: {reader.fieldnames!r}")
            rows = []
        else:
            rows = list(reader)

    seen: set[tuple[str, str, str, str]] = set()
    for line_no, row in enumerate(rows, start=2):
        key = (row["crate"], row["file"], row["test_name"], row["kind"])
        if key in seen:
            errors.append(f"line {line_no}: duplicate inventory key {key}")
        seen.add(key)
        if row["kind"] not in allowed_kind:
            errors.append(f"line {line_no}: invalid kind {row['kind']}")
        if row["class"] not in allowed_class:
            errors.append(f"line {line_no}: invalid class {row['class']}")
        if row["verdict"] not in allowed_verdict and not collapse_re.match(row["verdict"]):
            errors.append(f"line {line_no}: invalid verdict {row['verdict']}")
        if (row["verdict"] == "PARE" or row["verdict"].startswith("COLLAPSE(")) and not row["superseding_boundary"].strip():
            errors.append(f"line {line_no}: {row['verdict']} row lacks superseding_boundary")
        if row["class"] == "admission-adjacent" and row["verdict"] != "AUDIT":
            if not row["superseding_boundary"].strip():
                errors.append(f"line {line_no}: admission-adjacent non-AUDIT row lacks hard boundary")
        never_pare = (
            row["kind"] in {"compile_fail", "trybuild"}
            or row["class"] in {"seal-proof", "oracle-parity", "golden-byte", "invariant-required", "stead-required"}
            or row["test_name"] == "custom_layout_ethics_axis"
        )
        if never_pare and row["verdict"] != "KEEP":
            errors.append(f"line {line_no}: never-pare row is {row['verdict']}: {key}")

    discovered = discovered_items()
    missing = sorted(discovered - seen)
    extra = sorted(seen - discovered)
    if missing:
        inspect.append(f"mechanical enumeration missing {len(missing)} rows; first={missing[:5]}")
    if extra:
        inspect.append(f"inventory has {len(extra)} rows not currently enumerated; first={extra[:5]}")

    print("TEST-INVENTORY-CHECK REPORT")
    print(f"  rows: {len(rows)}")
    print(f"  discovered: {len(discovered)}")
    print(f"  missing: {len(missing)}")
    print(f"  extra: {len(extra)}")
    if inspect:
        print("  inspect:")
        for item in inspect:
            print(f"    {item}")
    else:
        print("  inspect: none")

if errors:
    print("TEST-INVENTORY-CHECK-VERDICT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

if inspect:
    print("TEST-INVENTORY-CHECK-VERDICT: INSPECT")
    sys.exit(0)

print("TEST-INVENTORY-CHECK-VERDICT: PASS")
PY
