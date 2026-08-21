#!/usr/bin/env bash
# COMPILE-FAIL-ERROR-IDENTITY-0 — prove rustdoc fences fail for the declared reason.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

exec "$PYTHON_BIN" - "$ROOT" "${1:---check}" <<'PY'
from __future__ import annotations

import hashlib
import json
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(sys.argv[1])
MODE = sys.argv[2]
FENCE_RE = re.compile(r"```compile_fail(?P<codes>(?:,E[0-9]{4})+)\s*$")
ERROR_RE = re.compile(r"error\[(E[0-9]{4})\]:")
LOCATION_RE = re.compile(r"^\s*-->\s+(.+\.rs):(\d+):\d+\s*$")


def strip_doc_prefix(line: str) -> str:
    return re.sub(r"^\s*//(?:/|!)\s?", "", line)


def source_identity(lines: list[str], start: int, codes: tuple[str, ...]) -> str:
    snippet: list[str] = []
    for raw in lines[start + 1 :]:
        cleaned = strip_doc_prefix(raw)
        if "```" in cleaned:
            break
        if cleaned.startswith("# "):
            cleaned = cleaned[2:]
        snippet.append(cleaned.rstrip())
    normalized = "\n".join(snippet).strip() + "\n"
    digest = hashlib.sha256(normalized.encode("utf-8")).hexdigest()[:12]
    return f"compile_fail_{'-'.join(codes)}_{digest}"


def discover() -> tuple[dict[str, list[dict]], list[str]]:
    by_package: dict[str, list[dict]] = {}
    errors: list[str] = []
    for path in sorted((ROOT / "crates").glob("*/src/**/*.rs")):
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        rel = path.relative_to(ROOT).as_posix()
        package = path.relative_to(ROOT / "crates").parts[0]
        for index, line in enumerate(lines):
            if "```compile_fail" not in line:
                continue
            match = FENCE_RE.search(line)
            if not match:
                errors.append(f"{rel}:{index + 1}: unpinned compile_fail fence")
                continue
            codes = tuple(match.group("codes").lstrip(",").split(","))
            close = None
            for cursor in range(index + 1, len(lines)):
                if "```" in strip_doc_prefix(lines[cursor]):
                    close = cursor + 1
                    break
            if close is None:
                errors.append(f"{rel}:{index + 1}: unclosed compile_fail fence")
                continue
            by_package.setdefault(package, []).append(
                {
                    "file": rel,
                    "start": index + 1,
                    "end": close,
                    "codes": set(codes),
                    "identity": source_identity(lines, index, codes),
                }
            )
    return by_package, errors


def normalize_report_path(raw: str) -> str:
    value = raw.replace("\\", "/")
    root = ROOT.as_posix().replace("\\", "/")
    if value.lower().startswith(root.lower() + "/"):
        value = value[len(root) + 1 :]
    return value


def observed_codes(output: str, fences: list[dict]) -> dict[str, set[str]]:
    observed = {fence["identity"]: set() for fence in fences}
    pending: str | None = None
    for line in output.splitlines():
        error = ERROR_RE.search(line)
        if error:
            pending = error.group(1)
            continue
        location = LOCATION_RE.match(line)
        if pending is None or not location:
            continue
        rel = normalize_report_path(location.group(1))
        line_no = int(location.group(2))
        matches = [
            fence
            for fence in fences
            if fence["file"].lower() == rel.lower()
            # Rustdoc's generated main wrapper reports snippet diagnostics one
            # source line later than the Markdown line for some crate-level docs.
            and fence["start"] < line_no <= fence["end"] + 1
        ]
        if len(matches) == 1:
            observed[matches[0]["identity"]].add(pending)
        pending = None
    return observed


def rustc_codes(source: str) -> set[str]:
    with tempfile.TemporaryDirectory() as td:
        path = pathlib.Path(td) / "probe.rs"
        path.write_text(source, encoding="utf-8")
        proc = subprocess.run(
            [
                "rustc",
                "--edition",
                "2021",
                "--crate-type",
                "lib",
                "--emit",
                "metadata",
                "--error-format",
                "json",
                str(path),
            ],
            capture_output=True,
            text=True,
            check=False,
        )
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


def selftest() -> int:
    wrong = rustc_codes("pub fn probe() { missing_compile_fail_name(); }\n")
    right = rustc_codes('pub fn probe() { let _: u32 = "wrong"; }\n')
    if wrong != {"E0425"} or right != {"E0308"}:
        print(f"COMPILE-FAIL-ERROR-IDENTITY-SELFTEST: FAIL codes wrong={sorted(wrong)} right={sorted(right)}")
        return 1
    if wrong == {"E0308"} or right != {"E0308"}:
        print("COMPILE-FAIL-ERROR-IDENTITY-SELFTEST: FAIL(wrong-error-accepted)")
        return 1
    print("COMPILE-FAIL-ERROR-IDENTITY-SELFTEST: PASS (wrong failure rejects; right failure admits)")
    return 0


def check() -> int:
    by_package, errors = discover()
    total = sum(len(rows) for rows in by_package.values())
    for package, fences in sorted(by_package.items()):
        proc = subprocess.run(
            [
                "cargo",
                "test",
                "-p",
                package,
                "--doc",
                "--",
                "--nocapture",
                "--test-threads=1",
            ],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        output = proc.stdout + "\n" + proc.stderr
        if proc.returncode != 0:
            errors.append(f"{package}: cargo doc-test command failed ({proc.returncode})")
        seen = observed_codes(output, fences)
        for fence in fences:
            actual = seen[fence["identity"]]
            if actual != fence["codes"]:
                errors.append(
                    f"{fence['file']}:{fence['start']} {fence['identity']}: "
                    f"declared={sorted(fence['codes'])} observed={sorted(actual)}"
                )
        print(f"COMPILE-FAIL-PACKAGE: {package} fences={len(fences)}")
    if errors:
        print(f"COMPILE-FAIL-ERROR-IDENTITY-VERDICT: FAIL fences={total} errors={len(errors)}")
        for error in errors:
            print(f"  - {error}")
        return 1
    print(f"COMPILE-FAIL-ERROR-IDENTITY-VERDICT: PASS fences={total}")
    return 0


if MODE == "--selftest":
    raise SystemExit(selftest())
if MODE == "--check":
    raise SystemExit(check())
print("usage: compile_fail_error_identity_check.sh [--check|--selftest]", file=sys.stderr)
raise SystemExit(2)
PY
