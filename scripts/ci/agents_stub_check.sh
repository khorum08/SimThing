#!/usr/bin/env bash
# OH-DOCS-SUNSET-0 — AGENTS.md pointer-stub scan (<=5 lines, no extra guidance).
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly AGENTS_MD="${REPO_ROOT}/AGENTS.md"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/agents_stub"

MODE="check"
AGENTS_PATH="$AGENTS_MD"
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/agents_stub_check.sh --check
  bash scripts/ci/agents_stub_check.sh --selftest
  bash scripts/ci/agents_stub_check.sh --fixture <name>
EOF
  exit 2
}

emit_verdict() {
  local ok="$1"
  if [[ "$ok" -eq 1 ]]; then
    printf 'AGENTS-STUB-VERDICT: PASS\n'
    return 0
  fi
  printf 'AGENTS-STUB-VERDICT: FAIL(pointer-stub)\n' >&2
  return 1
}

validate_stub() {
  local path="$1"
  local errors=0
  if [[ ! -f "$path" ]]; then
    echo "missing AGENTS.md" >&2
    return 1
  fi
  local lines
  lines="$(grep -cve '^\s*$' "$path" || true)"
  if [[ "$lines" -gt 5 ]]; then
    echo "AGENTS.md line count ${lines} > 5" >&2
    errors=$((errors + 1))
  fi
  if ! grep -q 'orient\.sh' "$path"; then
    echo "AGENTS.md missing orient.sh pointer" >&2
    errors=$((errors + 1))
  fi
  if ! grep -q 'orchestrator_orientation\.md' "$path"; then
    echo "AGENTS.md missing orchestrator_orientation.md pointer" >&2
    errors=$((errors + 1))
  fi
  if grep -qiE '^(##|###|[0-9]+\.)' "$path"; then
    echo "AGENTS.md contains guidance paragraph headings" >&2
    errors=$((errors + 1))
  fi
  if grep -qi 'do not add guidance' "$path"; then
    :
  else
    echo "AGENTS.md missing pointer-only disclaimer" >&2
    errors=$((errors + 1))
  fi
  [[ "$errors" -eq 0 ]]
}

run_fixture() {
  local name="$1"
  local dir="${FIXTURES_ROOT}/${name}"
  [[ -d "$dir" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${dir}/expected_verdict.txt" | head -n 1)"
  local exit_code=0
  set +e
  if validate_stub "${dir}/AGENTS.md"; then
    emit_verdict 1 >/dev/null
    [[ "$expected" == "AGENTS-STUB-VERDICT: PASS" ]] || exit_code=1
  else
    emit_verdict 0 >/dev/null
    [[ "$expected" == "AGENTS-STUB-VERDICT: FAIL(pointer-stub)" ]] || exit_code=1
  fi
  set -e
  if [[ "$exit_code" -eq 0 ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name} (want ${expected})"
  return 1
}

run_selftest() {
  local fixtures=(
    agents_stub_selftest_pass_valid
    agents_stub_selftest_fail_too_many_lines
    agents_stub_selftest_fail_extra_guidance
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    emit_verdict 1
    echo "AGENTS-STUB-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  emit_verdict 0
  echo "AGENTS-STUB-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  case "${1:-}" in
    --check)
      if validate_stub "$AGENTS_PATH"; then
        emit_verdict 1
      else
        emit_verdict 0
        exit 1
      fi
      ;;
    --selftest)
      run_selftest
      exit $?
      ;;
    --fixture)
      [[ $# -ge 2 ]] || usage
      run_fixture "$2"
      ;;
    -h|--help) usage ;;
    *) usage ;;
  esac
}

main "$@"