#!/usr/bin/env bash
# TEST-PARE-INVENTORY-0: validate the checked-in test corpus inventory.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY" "$@"
import csv
import os
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
inventory = pathlib.Path(sys.argv[2])
args = sys.argv[3:]
boundary_rows = root / "scripts/ci/test_lifecycle_boundary_rows.tsv"
residue_classes = root / "scripts/ci/test_residue_classes.tsv"
lifecycle_boundary_check = root / "scripts/ci/test_lifecycle_boundary_check.sh"

required = [
    "crate",
    "file",
    "test_name",
    "kind",
    "class",
    "superseding_boundary",
    "verdict",
    "note",
    "promotion_target",
]
allowed_kind = {"unit", "integration", "doc", "compile_fail", "trybuild", "fixture", "unknown"}
allowed_class = {
    "behavior-regression",
    "determinism",
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
    "dependency-floor",
    "unknown",
}
allowed_verdict = {"KEEP", "PARE", "AUDIT"}
errors: list[str] = []
inspect: list[str] = []

judgment_note_classes = {"behavior-regression", "escaped-bug"}
bad_judgment_notes = {
    "catches: behavior regression",
    "catches: escaped bug",
    "catches: important coverage",
    "permanent-residue:behavior-regression",
    "permanent-residue:escaped-bug",
    "regression test",
}

def judgment_note_ok(note: str) -> bool:
    normalized = " ".join(note.strip().lower().split())
    if normalized in bad_judgment_notes:
        return False
    if not normalized.startswith("catches: "):
        return False
    detail = normalized.removeprefix("catches: ").strip()
    if len(detail) < 24:
        return False
    if detail in {"behavior regression", "escaped bug", "important coverage", "regression test"}:
        return False
    return True

def prove_judgment_note_rule() -> None:
    bad = [
        "catches: behavior regression",
        "catches: escaped bug",
        "catches: important coverage",
        "permanent-residue:behavior-regression",
        "regression test",
        "kept because it matters",
    ]
    good = [
        "catches: TP-17 route detachment panic when detached child overlays settle twice",
        "catches: bug-2026-06-14 map edge saturation emitted non-monotonic frontier",
    ]
    failed = False
    for note in bad:
        if judgment_note_ok(note):
            print(f"  BAD accepted unexpectedly: {note}")
            failed = True
    for note in good:
        if not judgment_note_ok(note):
            print(f"  GOOD rejected unexpectedly: {note}")
            failed = True
    if failed:
        print("JUDGMENT-NOTE-RULE-VERDICT: FAIL")
        sys.exit(1)
    print("JUDGMENT-NOTE-RULE-VERDICT: PASS")
    sys.exit(0)

if args == ["--prove-judgment-note-rule"]:
    prove_judgment_note_rule()
if args:
    print(f"unknown arg(s): {' '.join(args)}", file=sys.stderr)
    sys.exit(2)

def read_residue_classes(path: pathlib.Path) -> set[str]:
    if not path.exists():
        errors.append(f"missing residue class table {path}")
        return set()
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != ["promotion_target"]:
            errors.append(f"bad residue class header: {reader.fieldnames!r}")
            return set()
        values = {row["promotion_target"].strip() for row in reader if row["promotion_target"].strip()}
    if not values:
        errors.append(f"empty residue class table {path}")
    return values

allowed_keep_targets = read_residue_classes(residue_classes)
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

def bash_cmd(script: pathlib.Path) -> list[str]:
    if os.name == "nt":
        git_bash_exepath = os.environ.get("EXEPATH")
        if git_bash_exepath:
            git_bash = pathlib.Path(git_bash_exepath) / "bash.exe"
            if git_bash.exists():
                return [str(git_bash), str(script)]
    return ["bash", str(script)]

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
    inventory_by_key: dict[tuple[str, str, str, str], dict[str, str]] = {}
    for line_no, row in enumerate(rows, start=2):
        key = (row["crate"], row["file"], row["test_name"], row["kind"])
        if key in seen:
            errors.append(f"line {line_no}: duplicate inventory key {key}")
        seen.add(key)
        inventory_by_key[key] = row
        if row["kind"] not in allowed_kind:
            errors.append(f"line {line_no}: invalid kind {row['kind']}")
        if row["class"] not in allowed_class:
            errors.append(f"line {line_no}: invalid class {row['class']}")
        if row["verdict"] not in allowed_verdict and not collapse_re.match(row["verdict"]):
            errors.append(f"line {line_no}: invalid verdict {row['verdict']}")
        if row["verdict"] == "KEEP":
            target = row["promotion_target"].strip()
            if target not in allowed_keep_targets and not target.startswith("promotion-target:"):
                errors.append(f"line {line_no}: KEEP row lacks permanent-residue class or promotion target")
            if row["class"] in judgment_note_classes and not judgment_note_ok(row["note"]):
                errors.append(
                    f"line {line_no}: KEEP {row['class']} row lacks specific 'catches:' judgment note"
                )
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

    print("TEST-LIFECYCLE-BOUNDARY AUTHORITY")
    if boundary_rows.exists():
        print(f"  boundary rows file: {boundary_rows.relative_to(root)}")
        print("  status: survivor boundary ownership ledger; validate with scripts/ci/test_lifecycle_boundary_check.sh")
    else:
        errors.append(f"missing lifecycle boundary rows file {boundary_rows}")

    if lifecycle_boundary_check.exists():
        boundary = subprocess.run(
            bash_cmd(lifecycle_boundary_check),
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )
        if boundary.stdout:
            print(boundary.stdout.rstrip())
        if boundary.stderr:
            print(boundary.stderr.rstrip())
        if boundary.returncode != 0:
            errors.append("lifecycle boundary check failed")
    else:
        errors.append(f"missing lifecycle boundary checker {lifecycle_boundary_check}")

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
