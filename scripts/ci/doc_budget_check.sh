#!/usr/bin/env bash
# OH-DOCS-SUNSET-0 — DOC-BUDGET tripwire for protected guidance prose files.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly BASELINE_TSV="${SCRIPT_DIR}/doc_budget_baseline.tsv"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/doc_budget"

MODE="check"
FIXTURE_DIR=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/doc_budget_check.sh --check
  bash scripts/ci/doc_budget_check.sh --headroom
  bash scripts/ci/doc_budget_check.sh --selftest
  bash scripts/ci/doc_budget_check.sh --fixture <name>
EOF
  exit 2
}

emit_verdict() {
  local kind="$1"
  case "$kind" in
    pass) printf 'DOC-BUDGET-VERDICT: PASS\n' ;;
    fail) printf 'DOC-BUDGET-VERDICT: FAIL(prose-growth)\n' >&2 ;;
  esac
}

count_lines() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    printf '0'
    return 0
  fi
  wc -l <"$path" | tr -d ' \r'
}

run_check() {
  local baseline="$1"
  local root="$2"
  local path max current
  local failures=0
  while IFS=$'\t' read -r path max; do
    path="${path//$'\r'/}"
    max="${max//$'\r'/}"
    [[ -z "${path:-}" || "$path" == "path" ]] && continue
    current="$(count_lines "${root}/${path}")"
    if [[ "$current" -gt "$max" ]]; then
      echo "DOC-BUDGET: ${path} lines=${current} max=${max}" >&2
      failures=$((failures + 1))
    fi
  done <"$baseline"
  if [[ "$failures" -gt 0 ]]; then
    emit_verdict fail
    return 1
  fi
  emit_verdict pass
  return 0
}

run_headroom() {
  local baseline="$1"
  local root="$2"
  local path max current room
  local failures=0
  local count=0
  while IFS=$'\t' read -r path max; do
    path="${path//$'\r'/}"
    max="${max//$'\r'/}"
    [[ -z "${path:-}" || "$path" == "path" ]] && continue
    current="$(count_lines "${root}/${path}")"
    room=$((max - current))
    if [[ "$room" -lt 0 ]]; then
      failures=$((failures + 1))
    fi
    count=$((count + 1))
    echo "DOC-BUDGET-HEADROOM-ITEM: path=${path} lines=${current}/${max} headroom=${room}"
  done <"$baseline"
  if [[ "$failures" -gt 0 ]]; then
    echo "DOC-BUDGET-HEADROOM-VERDICT: FAIL over=${failures} rows=${count}"
    return 1
  fi
  echo "DOC-BUDGET-HEADROOM-VERDICT: PASS over=0 rows=${count}"
  return 0
}

run_fixture() {
  local name="$1"
  local dir="${FIXTURES_ROOT}/${name}"
  [[ -d "$dir" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${dir}/expected_verdict.txt" | head -n 1)"
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/doc-budget-XXXXXX")"
  cp "${dir}/doc_budget_baseline.tsv" "${sandbox}/baseline.tsv"
  mkdir -p "${sandbox}/docs"
  cp "${dir}/ci_screening_surface.md" "${sandbox}/docs/ci_screening_surface.md" 2>/dev/null || true
  cp "${dir}/design.md" "${sandbox}/docs/design_0_0_8_4_7_orchestration_harness.md" 2>/dev/null || true
  local got exit_code
  set +e
  got="$(run_check "${sandbox}/baseline.tsv" "${sandbox}" 2>&1 | tail -n 1)"
  exit_code=$?
  set -e
  rm -rf "$sandbox"
  if [[ "$exit_code" -eq 0 && "$got" == "DOC-BUDGET-VERDICT: PASS" ]] || \
     [[ "$exit_code" -ne 0 && "$got" == "DOC-BUDGET-VERDICT: FAIL(prose-growth)" ]]; then
    if [[ "$expected" == "$got" ]]; then
      echo "PASS ${name}"
      return 0
    fi
  fi
  if [[ "$expected" == "DOC-BUDGET-VERDICT: FAIL(prose-growth)" && "$got" == *FAIL* ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name} (got=${got} exit=${exit_code}, want ${expected})"
  return 1
}

run_selftest() {
  local fixtures=(
    doc_budget_selftest_pass_at_ceiling
    doc_budget_selftest_fail_prose_growth
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    emit_verdict pass
    echo "DOC-BUDGET-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  emit_verdict fail
  echo "DOC-BUDGET-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  case "${1:-}" in
    --check)
      run_check "$BASELINE_TSV" "$REPO_ROOT"
      ;;
    --headroom)
      run_headroom "$BASELINE_TSV" "$REPO_ROOT"
      ;;
    --selftest)
      run_selftest
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
