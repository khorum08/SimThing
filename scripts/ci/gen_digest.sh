#!/usr/bin/env bash
# CI-C-DIGEST-0: generate the sanctioned-surface digest from CI data.
set -euo pipefail

usage() {
  echo "usage: $0 [--check]"
  exit 2
}

mode="${1:-generate}"
if [[ $# -gt 1 || ( "$mode" != "generate" && "$mode" != "--check" ) ]]; then
  usage
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

python_bin=""
if command -v python3 >/dev/null 2>&1; then
  python_bin="python3"
elif command -v python >/dev/null 2>&1; then
  python_bin="python"
else
  echo "gen_digest: python3/python not found on PATH" >&2
  exit 2
fi

exec "$python_bin" - "$REPO_ROOT" "$mode" <<'PY'
import hashlib
import pathlib
import sys
import tempfile

REPO_ROOT = pathlib.Path(sys.argv[1])
MODE = sys.argv[2]
CHECK = MODE == "--check"
DIGEST_PATH = REPO_ROOT / "docs/sanctioned_surface.md"

SOURCES = [
    "scripts/ci/allow/sealed_producers.txt",
    "scripts/ci/allow/inert_buffer_handles.txt",
    "scripts/ci/allow/kernel_surface.txt",
    "scripts/ci/allow/sealed_types.txt",
    "scripts/ci/scans.tsv",
]


def fail(message):
    print(f"gen_digest: {message}", file=sys.stderr)
    sys.exit(1)


def read_text(rel_path):
    path = REPO_ROOT / rel_path
    if not path.is_file():
        fail(f"missing source file: {rel_path}")
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        fail(f"unreadable source file {rel_path}: {exc}")


def data_lines(rel_path):
    rows = []
    for lineno, raw in enumerate(read_text(rel_path).splitlines(), start=1):
        stripped = raw.strip()
        if stripped and not stripped.startswith("#"):
            rows.append((lineno, raw))
    return rows


def split_record(rel_path, lineno, raw, fields, allow_empty=()):
    parts = [part.strip() for part in raw.split(" | ")]
    if len(parts) != fields or any(part == "" and idx not in allow_empty for idx, part in enumerate(parts)):
        fail(f"malformed record in {rel_path}:{lineno}: expected {fields} fields")
    return tuple(parts)


def parse_allow(rel_path):
    rows = [split_record(rel_path, lineno, raw, 4) for lineno, raw in data_lines(rel_path)]
    if not rows:
        fail(f"empty generated section source: {rel_path}")
    return rows


def parse_sealed_types(rel_path):
    rows = []
    for lineno, raw in data_lines(rel_path):
        value = raw.strip()
        if " | " in value or " " in value or "\t" in value:
            fail(f"malformed sealed type in {rel_path}:{lineno}: expected one bare name")
        rows.append((value,))
    if not rows:
        fail(f"empty generated section source: {rel_path}")
    return rows


def parse_scans(rel_path):
    rows = [split_record(rel_path, lineno, raw, 7, allow_empty={4}) for lineno, raw in data_lines(rel_path)]
    if not rows:
        fail(f"empty generated section source: {rel_path}")
    return rows


def ensure_unique(rows, section, key_index=0):
    seen = set()
    for row in rows:
        key = row[key_index]
        if key in seen:
            fail(f"duplicate generated key in {section}: {key}")
        seen.add(key)


def source_fingerprint(rel_path):
    data = read_text(rel_path).encode("utf-8")
    return len(data_lines(rel_path)), hashlib.sha256(data).hexdigest()


def source_name(rel_path):
    return pathlib.PurePosixPath(rel_path).name


def md_escape(value):
    return value.replace("\\", "\\\\").replace("|", "\\|").replace("\r", "").replace("\n", " ")


def md_unescape(value):
    out = []
    escaped = False
    for ch in value:
        if escaped:
            out.append(ch)
            escaped = False
        elif ch == "\\":
            escaped = True
        else:
            out.append(ch)
    if escaped:
        out.append("\\")
    return "".join(out).strip()


def md_row(values):
    return "| " + " | ".join(md_escape(value) for value in values) + " |"


def table(headers, rows):
    lines = [md_row(headers), "| " + " | ".join("---" for _ in headers) + " |"]
    lines.extend(md_row(row) for row in rows)
    return lines


def build_model():
    model = {
        "sealed_producers": parse_allow("scripts/ci/allow/sealed_producers.txt"),
        "inert_handles": parse_allow("scripts/ci/allow/inert_buffer_handles.txt"),
        "kernel_surface": parse_allow("scripts/ci/allow/kernel_surface.txt"),
        "sealed_types": parse_sealed_types("scripts/ci/allow/sealed_types.txt"),
        "scans": parse_scans("scripts/ci/scans.tsv"),
    }
    for section, rows in model.items():
        ensure_unique(rows, section)
    return model


def with_source(rows, rel_path):
    return [tuple(row) + (source_name(rel_path),) for row in rows]


def generate_markdown(model):
    lines = [
        "# Sanctioned Surface Digest",
        "",
        "> GENERATED FILE. Do not hand-edit. Regenerate with `bash scripts/ci/gen_digest.sh`.",
        "> Source of truth: `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`.",
        "",
        "This digest is a derived context artifact for low-context agents. If it disagrees with CI data, the CI data wins and this file or generator is wrong.",
        "",
        "## Source Manifest",
        "",
    ]

    manifest_rows = []
    for rel_path in SOURCES:
        count, digest = source_fingerprint(rel_path)
        manifest_rows.append((rel_path, str(count), digest))
    lines.extend(table(["source", "data rows", "sha256"], manifest_rows))

    lines.extend(["", "## Sanctioned Sealed Producers", ""])
    lines.extend(table(
        ["symbol", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["sealed_producers"], "scripts/ci/allow/sealed_producers.txt"),
    ))

    lines.extend(["", "## Inert Buffer Handles", ""])
    lines.extend(table(
        ["symbol", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["inert_handles"], "scripts/ci/allow/inert_buffer_handles.txt"),
    ))

    lines.extend(["", "## Kernel Surface", ""])
    lines.extend(table(
        ["symbol/signature", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["kernel_surface"], "scripts/ci/allow/kernel_surface.txt"),
    ))

    lines.extend(["", "## Sealed Types", ""])
    lines.extend(table(
        ["sealed type", "source"],
        [(row[0], "sealed_types.txt") for row in model["sealed_types"]],
    ))

    scan_rows = []
    for scan_id, severity, target, pattern, exclude, doctrine_ref, promotion_blocker in model["scans"]:
        scan_rows.append((
            scan_id,
            severity,
            doctrine_ref,
            target,
            pattern,
            exclude if exclude else "(none)",
            promotion_blocker,
            "scans.tsv",
        ))
    lines.extend(["", "## Forbidden / Screened Patterns", ""])
    lines.extend(table(
        ["scan-id", "reliability", "why", "target", "pattern/source", "exclude", "promotion-blocker", "source"],
        scan_rows,
    ))
    lines.append("")
    return "\n".join(lines)


def split_markdown_row(line):
    if not line.startswith("| ") or not line.endswith(" |"):
        fail(f"malformed generated markdown table row: {line}")
    cells = []
    current = []
    escaped = False
    for ch in line[2:-2]:
        if escaped:
            current.append("\\" + ch)
            escaped = False
        elif ch == "\\":
            escaped = True
        elif ch == "|":
            cells.append(md_unescape("".join(current)))
            current = []
        else:
            current.append(ch)
    if escaped:
        current.append("\\")
    cells.append(md_unescape("".join(current)))
    return tuple(cell.strip() for cell in cells)


def parse_generated_table(text, heading):
    lines = text.splitlines()
    marker = f"## {heading}"
    try:
        idx = lines.index(marker) + 1
    except ValueError:
        fail(f"missing generated section: {heading}")
    while idx < len(lines) and lines[idx].strip() == "":
        idx += 1
    if idx + 1 >= len(lines) or not lines[idx].startswith("| ") or not lines[idx + 1].startswith("| "):
        fail(f"missing generated table in section: {heading}")
    rows = []
    idx += 2
    while idx < len(lines) and lines[idx].startswith("| "):
        rows.append(split_markdown_row(lines[idx]))
        idx += 1
    if not rows:
        fail(f"empty generated table rows: {heading}")
    return rows


def verify_generated_exactness(text, model):
    expected = {
        "Sanctioned Sealed Producers": with_source(model["sealed_producers"], "scripts/ci/allow/sealed_producers.txt"),
        "Inert Buffer Handles": with_source(model["inert_handles"], "scripts/ci/allow/inert_buffer_handles.txt"),
        "Kernel Surface": with_source(model["kernel_surface"], "scripts/ci/allow/kernel_surface.txt"),
        "Sealed Types": [(row[0], "sealed_types.txt") for row in model["sealed_types"]],
    }
    for heading, rows in expected.items():
        if parse_generated_table(text, heading) != rows:
            fail(f"generated {heading} rows do not exactly match source data")


model = build_model()
generated = generate_markdown(model)

if CHECK:
    if not DIGEST_PATH.is_file():
        fail("docs/sanctioned_surface.md is missing")
    current = DIGEST_PATH.read_text(encoding="utf-8")
    verify_generated_exactness(current, model)
    if current != generated:
        with tempfile.NamedTemporaryFile("w", encoding="utf-8", delete=False, suffix=".md") as tmp:
            tmp.write(generated)
            tmp_path = tmp.name
        fail(f"docs/sanctioned_surface.md is stale; expected output written to {tmp_path}")
    print("gen_digest --check: PASS")
else:
    DIGEST_PATH.parent.mkdir(parents=True, exist_ok=True)
    DIGEST_PATH.write_text(generated, encoding="utf-8", newline="\n")
    print("generated docs/sanctioned_surface.md")
PY
