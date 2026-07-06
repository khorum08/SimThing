#!/usr/bin/env bash
# OH-COLD-START-0 — role-keyed orientation landing + ORIENT-RECEIPT emission.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly ORIENT_DOC="${REPO_ROOT}/docs/orchestrator_orientation.md"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

ROLE=""
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0
readonly COLD_START_FIXTURES="${SCRIPT_DIR}/fixtures/cold_start"

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/orient.sh --role=coding
  bash scripts/ci/orient.sh --role=orchestrator
  bash scripts/ci/orient.sh --role=da
  bash scripts/ci/orient.sh --selftest
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --role=*)
        ROLE="${1#--role=}"
        shift
        ;;
      --role)
        ROLE="${2:-}"
        [[ -n "$ROLE" ]] || usage
        shift 2
        ;;
      --selftest)
        FIXTURE_MODE="selftest"
        shift
        ;;
      -h|--help)
        usage
        ;;
      *)
        usage
        ;;
    esac
  done
}

emit_orientation() {
  local role="$1"
  ORIENT_ROLE="$role" ORIENT_DOC_PATH="$ORIENT_DOC" ORIENT_REPO_ROOT="$REPO_ROOT" \
    exec "$PYTHON_BIN" - <<'PY'
import hashlib
import os
import pathlib
import re
import sys

role = os.environ["ORIENT_ROLE"].lower()
orient_doc = pathlib.Path(os.environ["ORIENT_DOC_PATH"])
repo_root = pathlib.Path(os.environ["ORIENT_REPO_ROOT"])
script_dir = repo_root / "scripts" / "ci"

if role not in ("coding", "orchestrator", "da"):
    print(f"orient.sh: invalid role: {role}", file=sys.stderr)
    sys.exit(2)

if not orient_doc.is_file():
    print(f"orient.sh: missing {orient_doc}", file=sys.stderr)
    sys.exit(1)

text = orient_doc.read_text(encoding="utf-8")
digest_sha = hashlib.sha256(text.encode("utf-8")).hexdigest()

sources = [
    script_dir / "precedented_classes.tsv",
    script_dir / "binding_conditions.tsv",
    script_dir / "clearance_ledger.tsv",
    repo_root / "docs" / "design_0_0_8_4_7_orchestration_harness.md",
    script_dir / "relay_lint.sh",
]
source_stamp = hashlib.sha256(
    "|".join(hashlib.sha256(p.read_bytes()).hexdigest() for p in sources if p.is_file()).encode()
).hexdigest()[:16]

receipt = hashlib.sha256(
    f"ORIENT-RECEIPT|{role}|{digest_sha}|{source_stamp}".encode("utf-8")
).hexdigest()[:12]

SECTIONS_CODING = {
    "Source Stamps", "Next Rung Pointer", "Clearance Router Verdict Meanings",
    "Precedented Classes (active)", "tested_code_sha + coverage_basis Rule",
    "Inner Loop (coding agent)", "GHA Comment Commands", "Orientation Receipt (ORIENT-RECEIPT)",
}
SECTIONS_DA = {
    "Source Stamps", "OH Track / Rung Summary (0.0.8.4.7)", "Binding Conditions",
    "Clearance Ledger (recent)", "Escalation / DA-RESERVE Posture",
    "Relay Lint Required Blocks", "Orientation Receipt (ORIENT-RECEIPT)",
}

def split_sections(body):
    lines = body.splitlines()
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
    body_out = text
else:
    wanted = SECTIONS_CODING if role == "coding" else SECTIONS_DA
    parts = [header, ""]
    for name, content in sections.items():
        if name == "_header":
            continue
        if name in wanted:
            parts.append("\n".join(content).rstrip())
            parts.append("")
    body_out = "\n".join(parts).rstrip() + "\n"

print(f"ORIENT-RECEIPT: {receipt}")
print(f"role: {role}")
print(f"orientation_digest_sha: {digest_sha}")
print(f"source_stamp: {source_stamp}")
print("generated_at: source-bound")
print("--- orientation ---")
print(body_out.rstrip())
PY
}

run_selftest() {
  local fixtures=(
    cold_start_selftest_orient_roles
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_orient_selftest_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ORIENT-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  echo "ORIENT-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

run_orient_selftest_fixture() {
  local name="$1"
  local fix="${COLD_START_FIXTURES}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${fix}/expected_result.txt" | head -n 1)"
  local role
  role="$(tr -d '\r' <"${fix}/role.txt" | head -n 1)"
  local got
  got="$(emit_orientation "$role" | head -n 1)"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  [[ -n "$ROLE" ]] || usage
  emit_orientation "$ROLE"
}

main "$@"