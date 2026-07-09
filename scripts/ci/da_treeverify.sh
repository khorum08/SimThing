#!/usr/bin/env bash
# DA treeverify advisor — advisory review-depth profile (not a clearance verdict).
# Emits DA-TREEVERIFY-PROFILE for load-bearing routing; never graduates or clear-merges.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly PROFILE_TSV="${SCRIPT_DIR}/da_review_profile.tsv"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/da_treeverify"
readonly PY_HELPER="${SCRIPT_DIR}/da_treeverify_lib.py"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

MODE=""
PR_NUMBER=""
RANGE_SPEC=""
FIXTURE_NAME=""
FILES_FROM=""
BODY_FILE=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/da_treeverify.sh --selftest
  bash scripts/ci/da_treeverify.sh --check-lifecycle
  bash scripts/ci/da_treeverify.sh --fixture <name>
  bash scripts/ci/da_treeverify.sh --pr <number>
  bash scripts/ci/da_treeverify.sh --range <base>..<head>
  bash scripts/ci/da_treeverify.sh --files-from <path> [--body-file <path>]

Advisory only. Does not emit CLEARANCE-VERDICT or authorize merge.
EOF
  exit 2
}

parse_args() {
  [[ $# -gt 0 ]] || usage
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --selftest) MODE="selftest"; shift ;;
      --check-lifecycle) MODE="lifecycle"; shift ;;
      --fixture)
        [[ $# -ge 2 ]] || usage
        MODE="fixture"; FIXTURE_NAME="$2"; shift 2 ;;
      --pr)
        [[ $# -ge 2 ]] || usage
        MODE="pr"; PR_NUMBER="$2"; shift 2 ;;
      --range)
        [[ $# -ge 2 ]] || usage
        MODE="range"; RANGE_SPEC="$2"; shift 2 ;;
      --files-from)
        [[ $# -ge 2 ]] || usage
        MODE="files"; FILES_FROM="$2"; shift 2 ;;
      --body-file)
        [[ $# -ge 2 ]] || usage
        BODY_FILE="$2"; shift 2 ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
}

py_lifecycle() {
  local tsv="${1:-$PROFILE_TSV}"
  "$PYTHON_BIN" "$PY_HELPER" lifecycle --profile "$tsv"
}

py_profile() {
  local files="$1"
  local body="${2:-}"
  local profile="${3:-$PROFILE_TSV}"
  if [[ -n "$body" ]]; then
    "$PYTHON_BIN" "$PY_HELPER" profile --profile "$profile" --files "$files" --body "$body"
  else
    "$PYTHON_BIN" "$PY_HELPER" profile --profile "$profile" --files "$files"
  fi
}

resolve_pr_files() {
  local pr="$1"
  local out="$2"
  local files=""
  files="$(gh pr diff "$pr" --name-only 2>/dev/null || true)"
  if [[ -z "$files" ]]; then
    local repo
    repo="$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || true)"
    if [[ -n "$repo" ]]; then
      files="$(gh api "repos/${repo}/pulls/${pr}/files" --paginate --jq '.[].filename' 2>/dev/null || true)"
    fi
  fi
  printf '%s\n' "$files" >"$out"
}

resolve_pr_body() {
  local pr="$1"
  local out="$2"
  gh pr view "$pr" --json body -q .body 2>/dev/null >"$out" || true
}

run_fixture_profile() {
  local name="$1"
  local dir="${FIXTURES_ROOT}/${name}"
  local expected="${dir}/expected_profile.txt"
  local files="${dir}/changed_files.txt"
  local body="${dir}/pr_body.txt"
  local profile="${dir}/da_review_profile.tsv"
  [[ -d "$dir" ]] || { echo "FAIL ${name}: missing dir"; return 1; }
  [[ -f "$files" && -f "$expected" ]] || { echo "FAIL ${name}: missing files"; return 1; }
  local profile_arg="$PROFILE_TSV"
  [[ -f "$profile" ]] && profile_arg="$profile"
  local body_arg=""
  [[ -f "$body" ]] && body_arg="$body"

  local got rc
  set +e
  if [[ -n "$body_arg" ]]; then
    got="$(py_profile "$files" "$body_arg" "$profile_arg" 2>&1)"
  else
    got="$(py_profile "$files" "" "$profile_arg" 2>&1)"
  fi
  rc=$?
  set -e

  local exp_profile got_profile exp_rc
  exp_profile="$(grep -E '^DA-TREEVERIFY-PROFILE:' "$expected" | head -n1)"
  got_profile="$(printf '%s\n' "$got" | grep -E '^DA-TREEVERIFY-PROFILE:' | head -n1 || true)"
  exp_rc="$(grep -E '^exit_code:' "$expected" | head -n1 | awk '{print $2}' || true)"
  [[ -n "$exp_rc" ]] || exp_rc=0

  if [[ "$rc" != "$exp_rc" || "$got_profile" != "$exp_profile" ]]; then
    echo "FAIL ${name}"
    echo "  expected: ${exp_profile} exit=${exp_rc}"
    echo "  got:      ${got_profile} exit=${rc}"
    echo "  output: ${got}"
    return 1
  fi

  while IFS= read -r must || [[ -n "$must" ]]; do
    [[ -z "$must" || "$must" =~ ^# ]] && continue
    [[ "$must" =~ ^DA-TREEVERIFY-PROFILE: || "$must" =~ ^exit_code: ]] && continue
    if [[ "$must" =~ ^must_contain: ]]; then
      local needle="${must#must_contain:}"
      needle="${needle#"${needle%%[![:space:]]*}"}"
      if [[ "$got" != *"$needle"* ]]; then
        echo "FAIL ${name}: missing '${needle}'"
        return 1
      fi
    fi
  done <"$expected"

  echo "PASS ${name}"
  return 0
}

run_fixture_lifecycle() {
  local name="$1"
  local dir="${FIXTURES_ROOT}/${name}"
  local expected="${dir}/expected_lifecycle.txt"
  local profile="${dir}/da_review_profile.tsv"
  [[ -f "$profile" && -f "$expected" ]] || { echo "FAIL ${name}: missing lifecycle fixture"; return 1; }
  local got rc
  set +e
  got="$(py_lifecycle "$profile" 2>&1)"
  rc=$?
  set -e
  local exp_rc exp_line got_line
  exp_rc="$(grep -E '^exit_code:' "$expected" | awk '{print $2}' || true)"
  [[ -n "$exp_rc" ]] || exp_rc=0
  exp_line="$(grep -E '^DA-TREEVERIFY-LIFECYCLE-VERDICT:' "$expected" | head -n1)"
  got_line="$(printf '%s\n' "$got" | grep -E '^DA-TREEVERIFY-LIFECYCLE-VERDICT:' | head -n1 || true)"
  local exp_tok got_tok
  exp_tok="${exp_line#DA-TREEVERIFY-LIFECYCLE-VERDICT: }"
  got_tok="${got_line#DA-TREEVERIFY-LIFECYCLE-VERDICT: }"
  if [[ "$rc" != "$exp_rc" ]]; then
    echo "FAIL ${name} exit expected=${exp_rc} got=${rc} out=${got}"
    return 1
  fi
  if [[ "$exp_tok" == PASS* && "$got_tok" == PASS* ]]; then
    echo "PASS ${name}"
    return 0
  fi
  if [[ "$exp_tok" == FAIL* && "$got_tok" == FAIL* ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name} expected=${exp_line} got=${got_line}"
  return 1
}

run_selftest() {
  local profile_fixtures=(
    da_tv_selftest_docs_results_relax
    da_tv_selftest_production_mapeditor_deep
    da_tv_selftest_engine_kernel_deep
    da_tv_selftest_ci_scripts_deep
    da_tv_selftest_workshop_light
    da_tv_selftest_unclassified_deep
    da_tv_selftest_expeditionary_missing_until_fail
    da_tv_selftest_expeditionary_docs_light
    da_tv_selftest_expeditionary_production_stays_deep
  )
  local lifecycle_fixtures=(
    da_tv_selftest_lifecycle_non_core_expired_fail
    da_tv_selftest_lifecycle_core_ok
  )
  local name
  for name in "${profile_fixtures[@]}"; do
    if ! run_fixture_profile "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  for name in "${lifecycle_fixtures[@]}"; do
    if ! run_fixture_lifecycle "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if ! py_lifecycle "$PROFILE_TSV" >/dev/null; then
    echo "FAIL live-lifecycle-check"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  else
    echo "PASS live-lifecycle-check"
  fi
  local total=$((${#profile_fixtures[@]} + ${#lifecycle_fixtures[@]} + 1))
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "DA-TREEVERIFY-SELFTEST: PASS (${total} checks)"
    return 0
  fi
  echo "DA-TREEVERIFY-SELFTEST: FAIL (${SELFTEST_FAILURES})"
  return 1
}

main() {
  parse_args "$@"
  case "$MODE" in
    selftest) run_selftest; exit $? ;;
    lifecycle) py_lifecycle "$PROFILE_TSV"; exit $? ;;
    fixture)
      if [[ -f "${FIXTURES_ROOT}/${FIXTURE_NAME}/expected_lifecycle.txt" ]]; then
        run_fixture_lifecycle "$FIXTURE_NAME"
      else
        run_fixture_profile "$FIXTURE_NAME"
      fi
      exit $?
      ;;
    pr)
      local tmpf tmpb
      tmpf="$(mktemp)"
      tmpb="$(mktemp)"
      resolve_pr_files "$PR_NUMBER" "$tmpf"
      resolve_pr_body "$PR_NUMBER" "$tmpb"
      set +e
      py_profile "$tmpf" "$tmpb" "$PROFILE_TSV"
      local rc=$?
      set -e
      rm -f "$tmpf" "$tmpb"
      exit $rc
      ;;
    range)
      local tmpf base head
      tmpf="$(mktemp)"
      base="${RANGE_SPEC%%..*}"
      head="${RANGE_SPEC##*..}"
      git -C "$REPO_ROOT" diff --name-only "$base" "$head" >"$tmpf"
      set +e
      py_profile "$tmpf" "" "$PROFILE_TSV"
      local rc=$?
      set -e
      rm -f "$tmpf"
      exit $rc
      ;;
    files)
      py_profile "$FILES_FROM" "${BODY_FILE:-}" "$PROFILE_TSV"
      exit $?
      ;;
    *) usage ;;
  esac
}

main "$@"
