# CI lifecycle schema PR gate: run schema validation only for inventory/lifecycle TSV diffs.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/lifecycle_schema_gate"

SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/lifecycle_schema_pr_gate.sh <base> <head>
  bash scripts/ci/lifecycle_schema_pr_gate.sh --fixture <name>
  bash scripts/ci/lifecycle_schema_pr_gate.sh --selftest
EOF
  exit 2
}

schema_relevant() {
  local file
  while IFS= read -r file; do
    case "$file" in
      scripts/ci/test_inventory.tsv|scripts/ci/test_lifecycle_*.tsv|scripts/ci/test_lifecycle_boundary_rows.tsv)
        return 0
        ;;
    esac
  done
  return 1
}

run_gate_for_files() {
  local changed_files="$1"
  local inventory_override="${2:-}"
  if ! printf '%s\n' "$changed_files" | schema_relevant; then
    echo "LIFECYCLE-SCHEMA-PR-GATE: SKIP (no inventory/lifecycle TSV diff)"
    return 0
  fi

  echo "LIFECYCLE-SCHEMA-PR-GATE: RUN"
  if [[ -n "$inventory_override" ]]; then
    TEST_LIFECYCLE_INVENTORY="$inventory_override" \
      bash "${SCRIPT_DIR}/test_lifecycle_expiry_check.sh" --schema
  else
    bash "${SCRIPT_DIR}/test_lifecycle_expiry_check.sh" --schema
  fi
}

run_range() {
  local base="$1"
  local head="$2"
  local changed_files
  changed_files="$(git -C "$REPO_ROOT" diff --name-only "$base" "$head")"
  run_gate_for_files "$changed_files"
}

run_fixture() {
  local name="$1"
  local fix="${FIXTURES_ROOT}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected got rc inventory_override
  expected="$(tr -d '\r' <"${fix}/expected_verdict.txt" | head -n 1)"
  inventory_override=""
  if [[ -f "${fix}/test_inventory.tsv" ]]; then
    inventory_override="${fix}/test_inventory.tsv"
  fi
  rc=0
  got="$(run_gate_for_files "$(cat "${fix}/changed_files.txt")" "$inventory_override" 2>&1 | tail -n 1)" || rc=$?
  case "$expected" in
    LIFECYCLE-SCHEMA-PR-GATE:\ FAIL)
      if [[ "$rc" -ne 0 && "$got" == LIFECYCLE-EXPIRY-VERDICT:\ FAIL* ]]; then
        echo "PASS ${name}"
        return 0
      fi
      ;;
    *)
      if [[ "$rc" -eq 0 && "$got" == "$expected" ]]; then
        echo "PASS ${name}"
        return 0
      fi
      ;;
  esac
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  echo "  exit:     ${rc}"
  return 1
}

run_selftest() {
  local fixtures=(
    lifecycle_schema_gate_selftest_fail_invalid_birth_track
    lifecycle_schema_gate_selftest_pass_clean_inventory
    lifecycle_schema_gate_selftest_skip_unrelated
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "LIFECYCLE-SCHEMA-PR-GATE-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  echo "LIFECYCLE-SCHEMA-PR-GATE-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

case "${1:-}" in
  --selftest)
    run_selftest
    ;;
  --fixture)
    [[ $# -eq 2 ]] || usage
    run_fixture "$2"
    ;;
  "")
    usage
    ;;
  *)
    [[ $# -eq 2 ]] || usage
    run_range "$1" "$2"
    ;;
esac
