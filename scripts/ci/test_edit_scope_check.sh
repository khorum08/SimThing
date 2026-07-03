#!/usr/bin/env bash
# TEST-EDIT-SCOPE-GATE-0: validate Track D crate test-edit scope data.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCOPE_TABLE="${ROOT}/scripts/ci/test_edit_scope.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - "$ROOT" "$SCOPE_TABLE" "$@" <<'PY'
import csv
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
scope_table = pathlib.Path(sys.argv[2])
args = sys.argv[3:]

required = [
    "scope_id",
    "path_glob",
    "authorized_risk_class",
    "authorized_profile",
    "rationale",
    "retirement_condition",
]
protected_crates = {
    "simthing-kernel",
    "simthing-sim",
    "simthing-gpu",
    "simthing-driver",
    "simthing-mapeditor",
    "simthing-tools",
    "simthing-workshop",
}
errors: list[str] = []


def is_crate_edit(path: str) -> bool:
    parts = pathlib.PurePosixPath(path).parts
    return len(parts) >= 4 and parts[0] == "crates" and parts[2] in {"src", "tests", "benches"}


def read_scope_rows() -> list[dict[str, str]]:
    if not scope_table.exists():
        errors.append(f"missing test edit scope table {scope_table.relative_to(root)}")
        return []
    with scope_table.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != required:
            errors.append(f"bad test edit scope header: {reader.fieldnames!r}")
            return []
        rows = list(reader)
    if not rows:
        errors.append(f"empty test edit scope table {scope_table.relative_to(root)}")
    seen: set[str] = set()
    for line_no, row in enumerate(rows, start=2):
        for field in required:
            if not row[field].strip():
                errors.append(f"test edit scope line {line_no}: empty {field}")
        scope_id = row["scope_id"].strip()
        if scope_id in seen:
            errors.append(f"test edit scope line {line_no}: duplicate scope_id {scope_id}")
        seen.add(scope_id)

        path_glob = row["path_glob"].strip()
        glob_parts = pathlib.PurePosixPath(path_glob).parts
        rationale = row["rationale"].strip()
        src_only_glob = (
            len(glob_parts) >= 4
            and glob_parts[0] == "crates"
            and glob_parts[2] == "src"
            and all(part != "tests" and part != "benches" for part in glob_parts[3:])
        )
        if len(glob_parts) >= 2 and glob_parts[0] == "crates" and glob_parts[1] in protected_crates:
            if src_only_glob:
                if "Owner/DA-approved" not in rationale:
                    errors.append(
                        f"test edit scope line {line_no}: protected crate src/** requires Owner/DA-approved marker"
                    )
            else:
                errors.append(
                    f"test edit scope line {line_no}: protected crate {glob_parts[1]} must not be authorized"
                )
        if "/src/" in f"/{path_glob}/" and "DA-approved" not in rationale:
            errors.append(
                f"test edit scope line {line_no}: src/** authorization requires explicit DA-approved marker"
            )
    return rows


def matching_rows(rows: list[dict[str, str]], path: str) -> list[dict[str, str]]:
    pure = pathlib.PurePosixPath(path)
    matches: list[dict[str, str]] = []
    for row in rows:
        glob = row["path_glob"].strip()
        if pure.match(glob) or pure.full_match(glob):
            matches.append(row)
    return matches


def check_paths(rows: list[dict[str, str]], paths: list[str]) -> int:
    failed = False
    for path in paths:
        if not is_crate_edit(path):
            continue
        matches = matching_rows(rows, path)
        if not matches:
            print(f"TEST-EDIT-SCOPE: unauthorized crate edit {path}")
            failed = True
            continue
        for row in matches:
            print(
                "TEST-EDIT-SCOPE: authorized "
                f"{path} -> {row['scope_id']} "
                f"(risk={row['authorized_risk_class']}, profile={row['authorized_profile']})"
            )
    return 1 if failed else 0


prove_cases = [
    ("crates/simthing-mapgenerator/tests/params.rs", True),
    ("crates/simthing-clausething/tests/ct_scenario_container.rs", True),
    ("crates/simthing-spec/tests/scenario_ingestion_admission.rs", True),
    ("crates/simthing-driver/tests/example.rs", False),
    ("crates/simthing-mapgenerator/src/lib.rs", False),
    ("crates/simthing-kernel/tests/example.rs", False),
    ("crates/simthing-sim/tests/example.rs", False),
    ("crates/simthing-gpu/tests/example.rs", False),
    ("crates/simthing-mapeditor/tests/example.rs", False),
    ("crates/simthing-tools/tests/example.rs", False),
    ("crates/simthing-workshop/tests/example.rs", False),
]

rows = read_scope_rows()
if errors:
    print("TEST-EDIT-SCOPE-CHECK-VERDICT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

if args == ["--prove"]:
    failures: list[str] = []
    for path, expected in prove_cases:
        actual = bool(matching_rows(rows, path)) if is_crate_edit(path) else False
        status = "PASS" if actual == expected else "FAIL"
        expectation = "authorized" if expected else "unauthorized"
        observed = "authorized" if actual else "unauthorized"
        print(f"  {status}: {path} expected={expectation} observed={observed}")
        if actual != expected:
            failures.append(path)
    if failures:
        print("TEST-EDIT-SCOPE-CHECK-VERDICT: FAIL")
        sys.exit(1)
    print("TEST-EDIT-SCOPE-CHECK-VERDICT: PASS")
    sys.exit(0)

if args and args[0] == "--paths":
    exit_code = check_paths(rows, args[1:])
    if exit_code:
        print("TEST-EDIT-SCOPE-CHECK-VERDICT: FAIL")
        sys.exit(exit_code)
    print("TEST-EDIT-SCOPE-CHECK-VERDICT: PASS")
    sys.exit(0)

if args:
    print(f"unknown args: {args!r}")
    print("TEST-EDIT-SCOPE-CHECK-VERDICT: FAIL")
    sys.exit(1)

print(f"TEST-EDIT-SCOPE-CHECK REPORT")
print(f"  rows: {len(rows)}")
for row in rows:
    print(f"  scope: {row['scope_id']} -> {row['path_glob']}")
print("TEST-EDIT-SCOPE-CHECK-VERDICT: PASS")
PY
