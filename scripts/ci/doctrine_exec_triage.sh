#!/usr/bin/env bash
# Append a §1A triage row to scripts/ci/triage_log.tsv on the PR branch.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TRIAGE_LOG="${ROOT}/scripts/ci/triage_log.tsv"
TRIAGE_CHECK="${SCRIPT_DIR}/triage_log_check.sh"
FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/triage"

SCAN_ID="${1:-}"
OUTCOME="${2:-}"
REASON="${3:-}"
BRANCH="${4:-${GITHUB_HEAD_REF:-}}"
COMMIT="${5:-${GITHUB_SHA:-}}"
FIXTURE_MODE=""
FIXTURE_NAME=""
SELFTEST_FAILURES=0

usage() {
  echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
  echo "usage: $0 <scan-id> <delete|green|escalate> <reason> [branch] [commit]"
  echo "       $0 --selftest"
  echo "       $0 --fixture <name>"
  exit 1
}

print_format() {
  echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
}

validate_append() {
  if ! bash "$TRIAGE_CHECK" --validate-append "$SCAN_ID" "$OUTCOME" "$REASON"; then
    print_format
    return 1
  fi
  return 0
}

append_row() {
  if [[ ! -f "$TRIAGE_LOG" ]]; then
    echo "scan-id | branch | outcome | reason | commit" > "$TRIAGE_LOG"
  fi
  local row="${SCAN_ID} | ${BRANCH:-unknown} | ${OUTCOME} | ${REASON} | ${COMMIT:-unknown}"
  echo "$row" >> "$TRIAGE_LOG"
  echo "TRIAGE-APPEND: OK"
  echo "$row"
}

run_fixture() {
  local name="$1"
  local dir="${FIXTURES_ROOT}/${name}"
  [[ -d "$dir" ]] || { echo "missing fixture: $name" >&2; return 1; }

  local expected_exit expected_contains
  expected_exit="$(tr -d '\r' < "${dir}/expected_exit.txt")"
  expected_contains="$(tr -d '\r' < "${dir}/expected_contains.txt")"

  local arg_scan arg_outcome arg_reason arg_branch arg_commit
  arg_scan="$(sed -n '1p' "${dir}/args.txt" | tr -d '\r')"
  arg_outcome="$(sed -n '2p' "${dir}/args.txt" | tr -d '\r')"
  arg_reason="$(sed -n '3p' "${dir}/args.txt" | tr -d '\r')"
  arg_branch="$(sed -n '4p' "${dir}/args.txt" | tr -d '\r')"
  arg_commit="$(sed -n '5p' "${dir}/args.txt" | tr -d '\r')"

  SCAN_ID="$arg_scan"
  OUTCOME="$arg_outcome"
  REASON="$arg_reason"
  BRANCH="${arg_branch:-fixture-branch}"
  COMMIT="${arg_commit:-fixturecommit}"

  local tmp_log out exit_code
  tmp_log="$(mktemp "${TMPDIR:-/tmp}/triage-log-XXXXXX")"
  printf 'scan-id | branch | outcome | reason | commit\n' >"$tmp_log"
  TRIAGE_LOG="$tmp_log"

  set +e
  if [[ -z "$SCAN_ID" || -z "$OUTCOME" ]]; then
    out="$(print_format)"
    exit_code=1
  elif ! out="$(validate_append 2>&1)"; then
    out="$(print_format)"
    exit_code=1
  else
    out="$(append_row 2>&1)"
    exit_code=0
  fi
  set -e
  rm -f "$tmp_log"

  if [[ "$exit_code" -ne "$expected_exit" ]]; then
    echo "FAIL ${name} (exit=${exit_code}, want ${expected_exit})"
    printf '%s\n' "$out"
    return 1
  fi
  if ! printf '%s\n' "$out" | grep -Fq "$expected_contains"; then
    echo "FAIL ${name} (missing expected output: ${expected_contains})"
    printf '%s\n' "$out"
    return 1
  fi
  echo "PASS ${name}"
  return 0
}

run_selftest() {
  local fixtures=(
    triage_selftest_reject_missing_reason
    triage_selftest_reject_placeholder_reason
    triage_selftest_reject_unknown_outcome
    triage_selftest_accept_valid_reason
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "TRIAGE-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  echo "TRIAGE-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

parse_args() {
  case "${1:-}" in
    --selftest)
      FIXTURE_MODE="selftest"
      ;;
    --fixture)
      [[ $# -ge 2 ]] || usage
      FIXTURE_MODE="fixture"
      FIXTURE_NAME="$2"
      ;;
    -h|--help)
      usage
      ;;
    *)
      ;;
  esac
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    run_fixture "$FIXTURE_NAME"
    exit $?
  fi

  [[ -n "$SCAN_ID" && -n "$OUTCOME" ]] || usage

  if ! validate_append; then
    exit 1
  fi
  append_row
}

main "$@"