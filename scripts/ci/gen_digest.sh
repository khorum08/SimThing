#!/usr/bin/env bash
# CI-C-DIGEST-0: generate the sanctioned-surface digest from CI data.
set -euo pipefail

usage() {
  echo "usage: $0 [--check] [--track-doc PATH] [--output PATH]"
  exit 2
}

mode="generate"
track_doc=""
output_path=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --check)
      mode="--check"
      shift
      ;;
    --track-doc)
      track_doc="${2:-}"
      [[ -n "$track_doc" ]] || usage
      shift 2
      ;;
    --output)
      output_path="${2:-}"
      [[ -n "$output_path" ]] || usage
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    *)
      usage
      ;;
  esac
done

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

exec "$python_bin" - "$REPO_ROOT" "$mode" "$track_doc" "$output_path" <<'PY'
import hashlib
import pathlib
import sys
import tempfile

REPO_ROOT = pathlib.Path(sys.argv[1])
MODE = sys.argv[2]
TRACK_DOC_ARG = sys.argv[3]
OUTPUT_ARG = sys.argv[4]
CHECK = MODE == "--check"
DIGEST_PATH = REPO_ROOT / "docs/sanctioned_surface.md"

GLOBAL_SOURCES = [
    "scripts/ci/allow/sealed_producers.txt",
    "scripts/ci/allow/inert_buffer_handles.txt",
    "scripts/ci/allow/kernel_surface.txt",
    "scripts/ci/allow/sealed_types.txt",
    "scripts/ci/scans.tsv",
]
TRACK_ALLOW_FILES = [
    "sealed_producers.txt",
    "inert_buffer_handles.txt",
    "kernel_surface.txt",
    "sealed_types.txt",
]


def fail(message):
    print(f"gen_digest: {message}", file=sys.stderr)
    sys.exit(1)


def repo_rel_path(path_arg, label):
    path = pathlib.Path(path_arg)
    abs_path = path if path.is_absolute() else REPO_ROOT / path
    try:
        rel = abs_path.resolve().relative_to(REPO_ROOT.resolve())
    except ValueError:
        fail(f"{label} must be inside the repo: {path_arg}")
    return rel.as_posix()


TRACK_DOC_REL = repo_rel_path(TRACK_DOC_ARG, "--track-doc") if TRACK_DOC_ARG else ""
if TRACK_DOC_REL and not (REPO_ROOT / TRACK_DOC_REL).is_file():
    fail(f"--track-doc is not a repo file: {TRACK_DOC_ARG}")

if OUTPUT_ARG:
    OUTPUT_PATH = REPO_ROOT / repo_rel_path(OUTPUT_ARG, "--output")
elif TRACK_DOC_REL:
    OUTPUT_PATH = None
else:
    OUTPUT_PATH = DIGEST_PATH

if CHECK and TRACK_DOC_REL and OUTPUT_PATH is None:
    fail("--track-doc --check requires --output PATH")


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
        "sealed_producers": [(row, "scripts/ci/allow/sealed_producers.txt") for row in parse_allow("scripts/ci/allow/sealed_producers.txt")],
        "inert_handles": [(row, "scripts/ci/allow/inert_buffer_handles.txt") for row in parse_allow("scripts/ci/allow/inert_buffer_handles.txt")],
        "kernel_surface": [(row, "scripts/ci/allow/kernel_surface.txt") for row in parse_allow("scripts/ci/allow/kernel_surface.txt")],
        "sealed_types": [(row, "scripts/ci/allow/sealed_types.txt") for row in parse_sealed_types("scripts/ci/allow/sealed_types.txt")],
        "scans": [(row, "scripts/ci/scans.tsv") for row in parse_scans("scripts/ci/scans.tsv")],
        "sources": list(GLOBAL_SOURCES),
    }
    for section in ("sealed_producers", "inert_handles", "kernel_surface", "sealed_types", "scans"):
        ensure_unique([row for row, _ in model[section]], section)

    if TRACK_DOC_REL:
        add_track_addendum(model)
    return model


def section_key(section, row):
    return row[0]


def add_rows(model, section, rel_path, rows):
    if not rows:
        fail(f"empty track addendum source: {rel_path}")
    existing = {section_key(section, row) for row, _ in model[section]}
    local = set()
    for row in rows:
        key = section_key(section, row)
        if key in existing:
            if section == "scans":
                fail(f"{rel_path}: track addendum redefines global scan-id '{key}'")
            fail(f"{rel_path}: track addendum duplicates global {section} key '{key}'")
        if key in local:
            fail(f"{rel_path}: duplicate track addendum {section} key '{key}'")
        local.add(key)
        model[section].append((row, rel_path))
    if rel_path not in model["sources"]:
        model["sources"].append(rel_path)


def add_track_addendum(model):
    scans_rel = f"{TRACK_DOC_REL}.ci.tsv"
    if (REPO_ROOT / scans_rel).is_file():
        add_rows(model, "scans", scans_rel, parse_scans(scans_rel))

    allow_dir_rel = f"{TRACK_DOC_REL}.ci.allow"
    allow_dir = REPO_ROOT / allow_dir_rel
    if not allow_dir.is_dir():
        return
    known = set(TRACK_ALLOW_FILES)
    for child in sorted(allow_dir.iterdir()):
        if not child.is_file():
            continue
        if child.name not in known:
            fail(f"{allow_dir_rel}/{child.name}: unknown track addendum allow file")
    addendum_sources = {
        "sealed_producers": (f"{allow_dir_rel}/sealed_producers.txt", parse_allow),
        "inert_handles": (f"{allow_dir_rel}/inert_buffer_handles.txt", parse_allow),
        "kernel_surface": (f"{allow_dir_rel}/kernel_surface.txt", parse_allow),
        "sealed_types": (f"{allow_dir_rel}/sealed_types.txt", parse_sealed_types),
    }
    for section, (rel_path, parser) in addendum_sources.items():
        if (REPO_ROOT / rel_path).is_file():
            add_rows(model, section, rel_path, parser(rel_path))


def with_source(records):
    return [tuple(row) + (source_name(rel_path),) for row, rel_path in records]


def generate_markdown(model):
    lines = [
        "# Sanctioned Surface Digest",
        "",
        "> GENERATED FILE. Do not hand-edit. Regenerate with `bash scripts/ci/gen_digest.sh`.",
        "> Source of truth: `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`; optional track mode reads only the explicit track doc sibling addendum.",
        "",
        "This digest is a derived context artifact for low-context agents. If it disagrees with CI data, the CI data wins and this file or generator is wrong.",
        "",
        "## Source Manifest",
        "",
    ]
    if TRACK_DOC_REL:
        lines.extend([
            f"Track mode: `{TRACK_DOC_REL}`.",
            "",
        ])

    manifest_rows = []
    for rel_path in model["sources"]:
        count, digest = source_fingerprint(rel_path)
        manifest_rows.append((rel_path, str(count), digest))
    lines.extend(table(["source", "data rows", "sha256"], manifest_rows))

    lines.extend(["", "## Sanctioned Sealed Producers", ""])
    lines.extend(table(
        ["symbol", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["sealed_producers"]),
    ))

    lines.extend(["", "## Inert Buffer Handles", ""])
    lines.extend(table(
        ["symbol", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["inert_handles"]),
    ))

    lines.extend(["", "## Kernel Surface", ""])
    lines.extend(table(
        ["symbol/signature", "door-class", "rationale", "promotion-blocker", "source"],
        with_source(model["kernel_surface"]),
    ))

    lines.extend(["", "## Sealed Types", ""])
    lines.extend(table(
        ["sealed type", "source"],
        [(row[0], source_name(rel_path)) for row, rel_path in model["sealed_types"]],
    ))

    scan_rows = []
    for row, rel_path in model["scans"]:
        scan_id, severity, target, pattern, exclude, doctrine_ref, promotion_blocker = row
        scan_rows.append((
            scan_id,
            severity,
            doctrine_ref,
            target,
            pattern,
            exclude if exclude else "(none)",
            promotion_blocker,
            source_name(rel_path),
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
        "Sanctioned Sealed Producers": with_source(model["sealed_producers"]),
        "Inert Buffer Handles": with_source(model["inert_handles"]),
        "Kernel Surface": with_source(model["kernel_surface"]),
        "Sealed Types": [(row[0], source_name(rel_path)) for row, rel_path in model["sealed_types"]],
    }
    for heading, rows in expected.items():
        if parse_generated_table(text, heading) != rows:
            fail(f"generated {heading} rows do not exactly match source data")


model = build_model()
generated = generate_markdown(model)

if CHECK:
    check_path = OUTPUT_PATH or DIGEST_PATH
    if not check_path.is_file():
        fail(f"{check_path.relative_to(REPO_ROOT).as_posix()} is missing")
    current = check_path.read_text(encoding="utf-8")
    verify_generated_exactness(current, model)
    if current != generated:
        with tempfile.NamedTemporaryFile("w", encoding="utf-8", delete=False, suffix=".md") as tmp:
            tmp.write(generated)
            tmp_path = tmp.name
        fail(f"{check_path.relative_to(REPO_ROOT).as_posix()} is stale; expected output written to {tmp_path}")
    print("gen_digest --check: PASS")
else:
    if OUTPUT_PATH is None:
        sys.stdout.write(generated)
    else:
        OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT_PATH.write_text(generated, encoding="utf-8", newline="\n")
        print(f"generated {OUTPUT_PATH.relative_to(REPO_ROOT).as_posix()}")
PY
