#!/usr/bin/env bash
# SCANNER-SELFTEST-DELTA-GATE-0 -- prevent scanner self-proof steps from drifting ungated.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly DEFAULT_WORKFLOW="${REPO_ROOT}/.github/workflows/doctrine-scan.yml"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/selftest_gate_guard.sh [workflow.yml]
  bash scripts/ci/selftest_gate_guard.sh --selftest
EOF
  exit 2
}

run_guard() {
  local workflow="$1"
  "$PYTHON_BIN" - "$workflow" <<'PY'
import pathlib
import re
import sys

workflow = pathlib.Path(sys.argv[1])
mandate = "R1-TEST-PURGE / whole-tree-is-maintainer"
guard_re = re.compile(r"steps\.gate\.outputs\.run_selftests\s*==\s*['\"]true['\"]")

if not workflow.is_file():
    print(f"SELFTEST-GATE-GUARD: FAIL(missing-workflow) path={workflow}", file=sys.stderr)
    sys.exit(1)

lines = workflow.read_text(encoding="utf-8").splitlines()
steps = []
current = None
step_re = re.compile(r"^(\s*)-\s+(name|uses):\s*(.*)$")

for lineno, line in enumerate(lines, 1):
    match = step_re.match(line)
    if match:
        if current is not None:
            steps.append(current)
        current = {
            "start": lineno,
            "indent": len(match.group(1)),
            "header": match.group(3).strip().strip("\"'"),
            "lines": [line],
        }
    elif current is not None:
        current["lines"].append(line)

if current is not None:
    steps.append(current)


def step_key_lines(step, key):
    base = step["indent"] + 2
    pattern = re.compile(rf"^\s{{{base}}}{re.escape(key)}:\s*(.*)$")
    out = []
    for line in step["lines"]:
        match = pattern.match(line)
        if match:
            out.append(match.group(1).strip())
    return out


def run_text(step):
    base = step["indent"] + 2
    pattern = re.compile(rf"^\s{{{base}}}run:\s*(.*)$")
    block = []
    capturing = False
    for line in step["lines"]:
        match = pattern.match(line)
        if match:
            capturing = True
            block.append(match.group(1))
            continue
        if capturing:
            block.append(line)
    return "\n".join(block)


def is_scanner_selftest(run):
    return (
        "--selftest" in run
        or "doctrine_selftest.sh" in run
        or re.search(r"track_closeout\.sh[\s\S]*--prove", run) is not None
    )


violations = []
selftest_steps = 0
for step in steps:
    run = run_text(step)
    if not is_scanner_selftest(run):
        continue
    selftest_steps += 1
    if_text = " ".join(step_key_lines(step, "if"))
    if not guard_re.search(if_text):
        violations.append(step)

if violations:
    for step in violations:
        name = step["header"] or "<unnamed>"
        print(
            "SELFTEST-GATE-GUARD: FAIL(ungated-selftest) "
            f"mandate={mandate} step={name!r} line={step['start']}",
            file=sys.stderr,
        )
    sys.exit(1)

print(f"SELFTEST-GATE-GUARD: PASS selftest_steps={selftest_steps}")
PY
}

run_selftest() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/selftest-gate-guard-XXXXXX")"

  cat >"${sandbox}/gated.yml" <<'EOF'
name: Fixture
jobs:
  doctrine-scan:
    steps:
      - name: Orientation receipt selftest
        if: steps.gate.outputs.run_selftests == 'true'
        run: |
          bash scripts/ci/orient.sh --selftest
EOF

  cat >"${sandbox}/ungated.yml" <<'EOF'
name: Fixture
jobs:
  doctrine-scan:
    steps:
      - name: Doctrine self-test
        run: |
          bash scripts/ci/doctrine_selftest.sh
EOF

  if run_guard "${sandbox}/gated.yml" >/dev/null; then
    echo "SELFTEST-GATE-GUARD-SELFTEST: gated->PASS"
  else
    echo "SELFTEST-GATE-GUARD-SELFTEST: gated->FAIL" >&2
    rm -rf "$sandbox"
    return 1
  fi

  if run_guard "${sandbox}/ungated.yml" >/dev/null 2>&1; then
    echo "SELFTEST-GATE-GUARD-SELFTEST: ungated->PASS" >&2
    rm -rf "$sandbox"
    return 1
  fi

  echo "SELFTEST-GATE-GUARD-SELFTEST: ungated->FAIL"
  echo "SELFTEST-GATE-GUARD-SELFTEST: PASS"
  rm -rf "$sandbox"
}

main() {
  case "${1:-}" in
    --selftest)
      run_selftest
      ;;
    -h|--help)
      usage
      ;;
    "")
      run_guard "$DEFAULT_WORKFLOW"
      ;;
    *)
      [[ $# -eq 1 ]] || usage
      run_guard "$1"
      ;;
  esac
}

main "$@"
