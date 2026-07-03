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
audit = root / "scripts/ci/test_pare_audit.tsv"
boundary_rows = root / "scripts/ci/test_pare_boundary_rows.tsv"
residue_classes = root / "scripts/ci/test_residue_classes.tsv"
edit_scope_check = root / "scripts/ci/test_edit_scope_check.sh"

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
candidate_classes = {
    "admission-adjacent",
    "hygiene-theater",
    "usecase-superseded",
    "unknown",
    "duplicate-battery",
}
audit_required = [
    "crate",
    "file",
    "test_name",
    "kind",
    "current_class",
    "audit_class",
    "audit_verdict",
    "superseding_boundary",
    "representative_to_keep",
    "deletion_wave",
    "confidence",
    "note",
]
allowed_audit_verdict = {
    "PARE",
    "KEEP",
    "KEPT",
    "PARED",
    "BLOCKED",
    "AUDIT-BLOCKED",
    "COLLAPSED-REPRESENTATIVE",
}
allowed_confidence = {"high", "medium", "low"}

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

    audit_rows: list[dict[str, str]] = []
    if audit.exists():
        with audit.open("r", encoding="utf-8", newline="") as f:
            reader = csv.DictReader(f, delimiter="\t")
            if reader.fieldnames != audit_required:
                errors.append(f"bad audit header: {reader.fieldnames!r}")
            else:
                audit_rows = list(reader)

        audit_seen: set[tuple[str, str, str, str]] = set()
        for line_no, row in enumerate(audit_rows, start=2):
            key = (row["crate"], row["file"], row["test_name"], row["kind"])
            inv = inventory_by_key.get(key)
            inv_missing = inv is None
            verdict = row["audit_verdict"]
            if inv is None:
                if verdict == "PARED":
                    inv = {"class": row["current_class"], "kind": row["kind"], "test_name": row["test_name"]}
                else:
                    errors.append(f"audit line {line_no}: row does not reference inventory key {key}")
                    continue
            if key in audit_seen:
                errors.append(f"audit line {line_no}: duplicate audit key {key}")
            audit_seen.add(key)
            if row["current_class"] != inv["class"]:
                errors.append(
                    f"audit line {line_no}: current_class {row['current_class']} does not match inventory {inv['class']}"
                )
            if verdict == "PARED" and not inv_missing:
                errors.append(f"audit line {line_no}: PARED row is still present in inventory {key}")
            is_collapse = collapse_re.match(verdict) is not None
            if verdict not in allowed_audit_verdict and not is_collapse:
                errors.append(f"audit line {line_no}: invalid audit_verdict {verdict}")
            if row["confidence"] not in allowed_confidence:
                errors.append(f"audit line {line_no}: invalid confidence {row['confidence']}")
            if verdict in {"PARE", "PARED"} and not row["superseding_boundary"].strip():
                errors.append(f"audit line {line_no}: PARE lacks superseding_boundary")
            if is_collapse or verdict == "COLLAPSED-REPRESENTATIVE":
                if not row["superseding_boundary"].strip():
                    errors.append(f"audit line {line_no}: COLLAPSE lacks superseding_boundary")
                if not row["representative_to_keep"].strip():
                    errors.append(f"audit line {line_no}: COLLAPSE lacks representative_to_keep")
            if verdict in {"AUDIT-BLOCKED", "BLOCKED"} and not row["note"].strip():
                errors.append(f"audit line {line_no}: AUDIT-BLOCKED lacks reason note")
            never_pare = (
                inv["kind"] in {"compile_fail", "trybuild"}
                or inv["class"] in {"seal-proof", "oracle-parity", "golden-byte", "invariant-required", "stead-required"}
                or inv["test_name"] == "custom_layout_ethics_axis"
            )
            if never_pare and verdict != "KEEP":
                errors.append(f"audit line {line_no}: never-pare row is {verdict}: {key}")

        candidate_keys = {key for key, row in inventory_by_key.items() if row["class"] in candidate_classes}
        missing_audit = sorted(candidate_keys - audit_seen)
        pared_keys = {
            (row["crate"], row["file"], row["test_name"], row["kind"])
            for row in audit_rows
            if row["audit_verdict"] == "PARED"
        }
        extra_audit = sorted(audit_seen - candidate_keys - pared_keys)
        if missing_audit:
            errors.append(f"audit missing {len(missing_audit)} candidate rows; first={missing_audit[:5]}")
        if extra_audit:
            errors.append(f"audit has {len(extra_audit)} non-candidate rows; first={extra_audit[:5]}")
        print("TEST-PARE-AUDIT LEGACY REPORT")
        print(f"  audit rows: {len(audit_rows)}")
        print(f"  candidate rows: {len(candidate_keys)}")
        print(f"  missing audit rows: {len(missing_audit)}")
        print(f"  extra audit rows: {len(extra_audit)}")
    else:
        print("TEST-PARE-AUDIT LEGACY REPORT")
        print("  audit file: absent")

    print("TEST-PARE-BOUNDARY AUTHORITY")
    if boundary_rows.exists():
        print(f"  boundary rows file: {boundary_rows.relative_to(root)}")
        print("  status: authoritative deletion/collapse ledger; validate with scripts/ci/test_pare_boundary_check.sh")
    else:
        errors.append(f"missing boundary authority file {boundary_rows}")

    try:
        diff = subprocess.run(
            ["git", "diff", "--name-only", "--diff-filter=ACDMRTUXB", "origin/master...HEAD"],
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )
        changed = diff.stdout.splitlines() if diff.returncode == 0 else []
    except OSError:
        changed = []
    crate_edits = [
        path
        for path in changed
        if re.match(r"^crates/[^/]+/(src|tests|benches)/", path)
    ]
    if edit_scope_check.exists():
        cmd = bash_cmd(edit_scope_check)
        if crate_edits:
            cmd.extend(["--paths", *crate_edits])
        scope = subprocess.run(
            cmd,
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )
        if scope.stdout:
            print(scope.stdout.rstrip())
        if scope.stderr:
            print(scope.stderr.rstrip())
        if scope.returncode != 0:
            errors.append("test edit scope check failed")
    else:
        errors.append(f"missing test edit scope checker {edit_scope_check}")

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
