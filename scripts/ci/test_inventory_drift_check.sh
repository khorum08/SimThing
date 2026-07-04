#!/usr/bin/env bash
# TEST-ADMISSION-REGIME-0: stock gate for test inventory drift and KEEP admission.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"
PROVE=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prove)
      PROVE=1
      shift
      ;;
    --root)
      ROOT="$(cd "$2" && pwd)"
      INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"
      shift 2
      ;;
    --inventory)
      INVENTORY="$2"
      shift 2
      ;;
    -h|--help)
      echo "usage: $0 [--prove] [--root PATH] [--inventory PATH]"
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY" "$PROVE"
import csv
import pathlib
import re
import shutil
import sys
import tempfile

root = pathlib.Path(sys.argv[1])
inventory_path = pathlib.Path(sys.argv[2])
prove = sys.argv[3] == "1"
residue_classes_path = root / "scripts/ci/test_residue_classes.tsv"

header = [
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

def read_residue_classes(path: pathlib.Path) -> set[str]:
    if not path.exists():
        print(f"TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL")
        print(f"  - missing residue class table {path}")
        sys.exit(1)
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != ["promotion_target"]:
            print(f"TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL")
            print(f"  - bad residue class header: {reader.fieldnames!r}")
            sys.exit(1)
        values = {row["promotion_target"].strip() for row in reader if row["promotion_target"].strip()}
    if not values:
        print(f"TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL")
        print(f"  - empty residue class table {path}")
        sys.exit(1)
    return values

permanent_targets = read_residue_classes(residue_classes_path)
kernel_sim_permanent = permanent_targets | {
    "permanent-residue:seal-proof",
    "permanent-residue:oracle-parity",
    "permanent-residue:golden-byte",
    "permanent-residue:determinism",
    "permanent-residue:behavior-regression",
}

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

def rust_files(base: pathlib.Path) -> list[pathlib.Path]:
    files: set[pathlib.Path] = set()
    for pattern in ("crates/*/src/**/*.rs", "crates/*/tests/**/*.rs", "crates/*/benches/**/*.rs"):
        files.update(base.glob(pattern))
    return sorted(path.relative_to(base) for path in files)

def discovered_items(base: pathlib.Path) -> set[tuple[str, str, str, str]]:
    items: set[tuple[str, str, str, str]] = set()
    for rel in rust_files(base):
        text = (base / rel).read_text(encoding="utf-8", errors="replace").splitlines()
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
    fixtures = sorted((base / "scripts/ci/fixtures").glob("**/*"))
    for path in fixtures:
        if path.is_file():
            rel = path.relative_to(base)
            items.add(("scripts-ci", norm(rel), rel.name, "fixture"))
    return items

def read_inventory(path: pathlib.Path) -> tuple[list[dict[str, str]], list[str]]:
    errors: list[str] = []
    if not path.exists():
        return [], [f"missing inventory {path}"]
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != header:
            return [], [f"bad inventory header: {reader.fieldnames!r}"]
        return list(reader), errors

def check(base: pathlib.Path, inv_path: pathlib.Path) -> tuple[bool, list[str], dict[str, int]]:
    rows, errors = read_inventory(inv_path)
    discovered = discovered_items(base)
    inventory_keys = {
        (row["crate"], row["file"], row["test_name"], row["kind"]): row
        for row in rows
    }

    missing = sorted(discovered - set(inventory_keys))
    stale = sorted(
        key
        for key in (set(inventory_keys) - discovered)
        if inventory_keys[key].get("promotion_target", "").strip()
        != "permanent-residue:dependency-floor"
    )
    if missing:
        errors.append(
            f"unledgered test rows: {len(missing)}; remedy: add a classified ledger row or remove the test; first={missing[:5]}"
        )
    if stale:
        errors.append(f"ledgered-but-deleted test rows: {len(stale)}; stale ledger first={stale[:5]}")

    for line_no, row in enumerate(rows, start=2):
        target = row.get("promotion_target", "").strip()
        verdict = row.get("verdict", "").strip()
        if verdict == "KEEP":
            if target not in permanent_targets and not target.startswith("promotion-target:"):
                errors.append(
                    f"line {line_no}: unowned KEEP row lacks permanent-residue class or promotion target: "
                    f"{row['crate']} {row['file']}::{row['test_name']}"
                )
            if row["crate"] in {"simthing-kernel", "simthing-sim"}:
                if target not in kernel_sim_permanent:
                    errors.append(
                        f"line {line_no}: kernel/sim KEEP row is not permanent-residue: "
                        f"{row['crate']} {row['file']}::{row['test_name']}"
                    )
    promotion_count = sum(1 for row in rows if row.get("promotion_target", "").startswith("promotion-target:"))
    plan_path = base / "docs/tests/test_promotion_wave_plan.md"
    if plan_path.exists():
        text = plan_path.read_text(encoding="utf-8", errors="replace")
        m = re.search(r"total promotion-target rows:\s+([0-9]+)", text)
        if not m:
            errors.append("promotion-wave plan lacks total promotion-target row count")
        elif int(m.group(1)) != promotion_count:
            errors.append(
                f"promotion-wave plan count {m.group(1)} does not match ledger promotion-target rows {promotion_count}"
            )
    elif base == root:
        errors.append("missing docs/tests/test_promotion_wave_plan.md")
    stats = {
        "rows": len(rows),
        "discovered": len(discovered),
        "missing": len(missing),
        "stale": len(stale),
        "promotion_target_rows": promotion_count,
    }
    return not errors, errors, stats

def write_inventory(path: pathlib.Path, rows: list[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=header, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)

def prove_case(name: str, setup) -> bool:
    with tempfile.TemporaryDirectory(prefix=f"test-drift-{name}-") as tmp:
        base = pathlib.Path(tmp)
        setup(base)
        ok, errors, _stats = check(base, base / "scripts/ci/test_inventory.tsv")
        if ok:
            print(f"  {name}: FAIL expected drift failure, got PASS")
            return False
        print(f"  {name}: PASS ({errors[0]})")
        return True

def setup_unledgered(base: pathlib.Path) -> None:
    dst = base / "crates/simthing-spec/tests/unledgered_test.rs"
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(root / "scripts/ci/fixtures/test_drift/unledgered_test.rs", dst)
    write_inventory(base / "scripts/ci/test_inventory.tsv", [])

def setup_stale(base: pathlib.Path) -> None:
    write_inventory(
        base / "scripts/ci/test_inventory.tsv",
        [{
            "crate": "simthing-spec",
            "file": "crates/simthing-spec/tests/deleted.rs",
            "test_name": "deleted_test",
            "kind": "integration",
            "class": "behavior-regression",
            "superseding_boundary": "B-T5-BEHAVIOR-REGRESSION",
            "verdict": "KEEP",
            "note": "stale fixture",
            "promotion_target": "permanent-residue:behavior-regression",
        }],
    )

def setup_unowned_keep(base: pathlib.Path) -> None:
    src = base / "crates/simthing-spec/tests/unowned.rs"
    src.parent.mkdir(parents=True, exist_ok=True)
    src.write_text("#[test]\nfn unowned_keep() {}\n", encoding="utf-8")
    inv = base / "scripts/ci/test_inventory.tsv"
    inv.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(root / "scripts/ci/fixtures/test_drift/unowned_keep.tsv", inv)

def setup_kernel_non_never(base: pathlib.Path) -> None:
    src = base / "crates/simthing-kernel/src/non_never.rs"
    src.parent.mkdir(parents=True, exist_ok=True)
    src.write_text("#[test]\nfn kernel_admission_enumeration() {}\n", encoding="utf-8")
    inv = base / "scripts/ci/test_inventory.tsv"
    inv.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(root / "scripts/ci/fixtures/test_drift/kernel_non_never_pare.tsv", inv)

if prove:
    print("TEST-INVENTORY-DRIFT-PROVE REPORT")
    results = [
        prove_case("unledgered-test", setup_unledgered),
        prove_case("stale-ledger", setup_stale),
        prove_case("unowned-KEEP", setup_unowned_keep),
        prove_case("kernel-sim-strict-tier", setup_kernel_non_never),
    ]
    if all(results):
        print("TEST-INVENTORY-DRIFT-PROVE-VERDICT: PASS")
        sys.exit(0)
    print("TEST-INVENTORY-DRIFT-PROVE-VERDICT: FAIL")
    sys.exit(1)

ok, errors, stats = check(root, inventory_path)
print("TEST-INVENTORY-DRIFT-CHECK REPORT")
print(f"  rows: {stats.get('rows', 0)}")
print(f"  discovered: {stats.get('discovered', 0)}")
print(f"  unledgered: {stats.get('missing', 0)}")
print(f"  stale: {stats.get('stale', 0)}")
print(f"  promotion-target rows: {stats.get('promotion_target_rows', 0)}")
if errors:
    print("TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)
print("TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS")
PY
