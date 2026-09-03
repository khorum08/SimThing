#!/usr/bin/env bash
# Rustified Test Lifecycle: validate checked-in survivor inventory.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY" "$@"
import csv
import hashlib
import os
import pathlib
import re
import subprocess
import sys
import tempfile

root = pathlib.Path(sys.argv[1])
inventory = pathlib.Path(sys.argv[2])
args = sys.argv[3:]
residue_classes = root / "scripts/ci/test_residue_classes.tsv"
lifecycle_expiry_check = root / "scripts/ci/test_lifecycle_expiry_check.sh"

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
    "birth_track",
    "dsu_survivals",
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
    "until-closeout:behavior-regression",
    "until-closeout:escaped-bug",
    "regression test",
}

compile_fail_fence_re = re.compile(
    r"```compile_fail(?P<codes>(?:,E[0-9]{4})+)\s*$"
)


def strip_doc_prefix(line: str) -> str:
    return re.sub(r"^\s*//(?:/|!)\s?", "", line)


def compile_fail_identity(lines: list[str], index: int) -> str:
    match = compile_fail_fence_re.search(lines[index])
    if not match:
        raise ValueError("compile_fail fence lacks pinned E#### error identity")
    codes = match.group("codes").lstrip(",").split(",")
    snippet: list[str] = []
    for raw in lines[index + 1 :]:
        cleaned = strip_doc_prefix(raw)
        if "```" in cleaned:
            break
        if cleaned.startswith("# "):
            cleaned = cleaned[2:]
        snippet.append(cleaned.rstrip())
    normalized = "\n".join(snippet).strip() + "\n"
    digest = hashlib.sha256(normalized.encode("utf-8")).hexdigest()[:12]
    return f"compile_fail_{'-'.join(codes)}_{digest}"

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
        "until-closeout:behavior-regression",
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


def prove_compile_fail_identity() -> None:
    right = ["/// ```compile_fail,E0308", '/// let _: u32 = "wrong";', "/// ```"]
    wrong_code = ["/// ```compile_fail,E0425", '/// let _: u32 = "wrong";', "/// ```"]
    changed = ["/// ```compile_fail,E0308", '/// let _: u64 = "wrong";', "/// ```"]
    if len({
        compile_fail_identity(right, 0),
        compile_fail_identity(wrong_code, 0),
        compile_fail_identity(changed, 0),
    }) != 3:
        print("COMPILE-FAIL-IDENTITY-PROVE-VERDICT: FAIL(identity-collision)")
        sys.exit(1)
    try:
        compile_fail_identity(["/// ```compile_fail", "/// missing();", "/// ```"], 0)
    except ValueError:
        pass
    else:
        print("COMPILE-FAIL-IDENTITY-PROVE-VERDICT: FAIL(unpinned-fence-accepted)")
        sys.exit(1)

    # Rustdoc itself accepts any failure for compile_fail. Compile probes must
    # therefore compare the emitted rustc code with the pinned identity.
    with tempfile.TemporaryDirectory() as td:
        fixture = pathlib.Path(td) / "probe.rs"

        def rustc_codes(source: str) -> set[str]:
            fixture.write_text(source, encoding="utf-8")
            proc = subprocess.run(
                [
                    "rustc", "--edition", "2021", "--crate-type", "lib",
                    "--emit", "metadata", "--error-format", "json", str(fixture),
                ],
                capture_output=True,
                text=True,
                check=False,
            )
            import json
            codes: set[str] = set()
            for line in proc.stderr.splitlines() + proc.stdout.splitlines():
                try:
                    message = json.loads(line)
                except json.JSONDecodeError:
                    continue
                code = (message.get("code") or {}).get("code")
                if code:
                    codes.add(code)
            return codes

        wrong = rustc_codes("pub fn probe() { missing_compile_fail_name(); }\n")
        right_codes = rustc_codes('pub fn probe() { let _: u32 = "wrong"; }\n')
        if wrong != {"E0425"} or wrong == {"E0308"}:
            print("COMPILE-FAIL-IDENTITY-PROVE-VERDICT: FAIL(wrong-error-passed)")
            sys.exit(1)
        if right_codes != {"E0308"}:
            print("COMPILE-FAIL-IDENTITY-PROVE-VERDICT: FAIL(right-error-rejected)")
            sys.exit(1)
    print("COMPILE-FAIL-IDENTITY-PROVE-VERDICT: PASS")
    sys.exit(0)

if args == ["--prove-judgment-note-rule"]:
    prove_judgment_note_rule()
if args == ["--prove-compile-fail-identity"]:
    prove_compile_fail_identity()
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
            # A cfg(test) module is a source container, not an executable test.
            # Its child #[test] functions are inventoried individually above.
            if "```compile_fail" in line:
                try:
                    identity = compile_fail_identity(text, index)
                except ValueError as exc:
                    errors.append(f"{norm(rel)}:{index + 1}: {exc}")
                    continue
                items.add((crate_for(rel), norm(rel), identity, "compile_fail"))
            if "trybuild::TestCases" in line or ".compile_fail(" in line:
                # Content-hash identity, never line-keyed (positional-identity
                # defect class; harness fix session 2026-09-03).
                tb_digest = hashlib.sha256(line.strip().encode("utf-8")).hexdigest()[:12]
                items.add((crate_for(rel), norm(rel), f"trybuild_{tb_digest}", "trybuild"))
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
        if re.search(r"_line_\d+$", row["test_name"]):
            errors.append(
                f"line {line_no}: line-keyed test identity {row['test_name']} refused"
                " (positional-identity class; use symbol or content-hash identity)"
            )
        if row["kind"] not in allowed_kind:
            errors.append(f"line {line_no}: invalid kind {row['kind']}")
        if row["class"] not in allowed_class:
            errors.append(f"line {line_no}: invalid class {row['class']}")
        if row["verdict"] not in allowed_verdict and not collapse_re.match(row["verdict"]):
            errors.append(f"line {line_no}: invalid verdict {row['verdict']}")
        if row["verdict"] == "KEEP":
            target = row["promotion_target"].strip()
            if target not in allowed_keep_targets and not target.startswith("promotion-target:"):
                errors.append(f"line {line_no}: KEEP row lacks until-closeout class or promotion target")
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

    # HU-INVENTORY-ONEWRITE-0: boundary-row audit ledger retired; inventory is the
    # sole survivor table. Policy doctrine remains in test_lifecycle_boundaries.tsv.
    print("TEST-LIFECYCLE-BOUNDARY AUTHORITY")
    print("  status: boundary audit ledger RETIRED (HU-INVENTORY-ONEWRITE-0); one table = test_inventory.tsv")

    print("TEST-LIFECYCLE-EXPIRY AUTHORITY")
    if lifecycle_expiry_check.exists():
        expiry = subprocess.run(
            bash_cmd(lifecycle_expiry_check) + ["--schema"],
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )
        if expiry.stdout:
            print(expiry.stdout.rstrip())
        if expiry.stderr:
            print(expiry.stderr.rstrip())
        if expiry.returncode != 0:
            errors.append("lifecycle expiry schema check failed")
    else:
        errors.append(f"missing lifecycle expiry checker {lifecycle_expiry_check}")

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
