#!/usr/bin/env bash
# Emit orientation digest (optionally role-filtered) for GHA /orient command.
set -euo pipefail

ROLE="${1:-orchestrator}"
REPORT="${2:-orient-report.txt}"
HEAD_SHA="${3:-}"
BASE_SHA="${4:-}"
PR="${5:-}"

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly ORIENT_DOC="${REPO_ROOT}/docs/orchestrator_orientation.md"

[[ -f "$ORIENT_DOC" ]] || { echo "missing orientation digest: $ORIENT_DOC" >&2; exit 1; }

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - "$ORIENT_DOC" "$ROLE" "$PR" "$HEAD_SHA" "$BASE_SHA" >"$REPORT" <<'PY'
import pathlib
import sys

doc_path, role, pr, head_sha, base_sha = sys.argv[1:6]
text = pathlib.Path(doc_path).read_text(encoding="utf-8")
role = (role or "orchestrator").lower()

SECTIONS_ORCHESTRATOR = None
SECTIONS_CODING = {
    "Source Stamps",
    "Next Rung Pointer",
    "Clearance Router Verdict Meanings",
    "Precedented Classes (active)",
    "tested_code_sha + coverage_basis Rule",
    "Inner Loop (coding agent)",
    "GHA Comment Commands",
}
SECTIONS_DA = {
    "Source Stamps",
    "OH Track / Rung Summary (0.0.8.4.7)",
    "Binding Conditions",
    "Clearance Ledger (recent)",
    "Escalation / DA-RESERVE Posture",
    "Relay Lint Required Blocks",
}

def split_sections(text):
    lines = text.splitlines()
    sections = {"_header": []}
    current = "_header"
    for line in lines:
        if line.startswith("## "):
            current = line[3:].strip()
            sections[current] = [line]
        else:
            sections.setdefault(current, []).append(line)
    return sections

sections = split_sections(text)
header = "\n".join(sections.get("_header", [])).strip()

if role == "orchestrator":
    body = text
else:
    wanted = SECTIONS_CODING if role == "coding" else SECTIONS_DA
    parts = [header, ""]
    for name, content in sections.items():
        if name == "_header":
            continue
        if name in wanted:
            parts.append("\n".join(content).rstrip())
            parts.append("")
    body = "\n".join(parts).rstrip() + "\n"

meta = []
if pr:
    meta.append(f"pr: {pr}")
if head_sha:
    meta.append(f"head_sha: {head_sha}")
if base_sha:
    meta.append(f"base_sha: {base_sha}")
meta.append(f"role: {role}")

print("ORIENT-REPORT: OK")
for line in meta:
    print(line)
print("--- orientation ---")
print(body.rstrip())
PY

echo "ORIENT-REPORT: written to ${REPORT}"