#!/usr/bin/env bash
# OH-CLEARANCE-ROUTER-0 — emit CLEARANCE-VERDICT for PR/range evidence.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/clearance"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

GATE_WIRING_PATHS=(
  "scripts/ci/clearance_check.sh"
  "scripts/ci/precedented_classes.tsv"
  "scripts/ci/binding_conditions.tsv"
  "scripts/ci/clearance_ledger.tsv"
  ".github/workflows/doctrine-exec-commands.yml"
  "scripts/ci/doctrine_exec_commands.sh"
  "scripts/ci/doctrine_exec_clearance.sh"
  "scripts/ci/doctrine_exec_clearance_comment.sh"
)

ENGINE_CRATE_PREFIXES=(
  "crates/simthing-kernel/"
  "crates/simthing-sim/src/"
  "crates/simthing-spec/src/"
  "crates/simthing-clausething/src/"
  "crates/simthing-gpu/src/"
  "crates/simthing-driver/src/"
)

VERDICT=""
FIXTURE_MODE=""
FIXTURE_DIR=""
PR_NUMBER=""
RANGE_SPEC=""
SELFTEST_FAILURES=0
REQUESTED_TARGET=0
CHANGED_FILES_RESOLVED=0
CHANGED_FILES_LIST=""
PR_RESOLUTION_FAILED=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/clearance_check.sh --selftest
  bash scripts/ci/clearance_check.sh --fixture <name>
  bash scripts/ci/clearance_check.sh --pr <number>
  bash scripts/ci/clearance_check.sh --range <base>..<head>
  bash scripts/ci/clearance_check.sh <pr-number>
EOF
  exit 1
}

emit_verdict() {
  local kind="$1"
  local detail="${2:-}"
  case "$kind" in
    clearable)
      VERDICT="CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE"
      ;;
    reserve)
      VERDICT="CLEARANCE-VERDICT: DA-RESERVE(${detail})"
      ;;
    fail)
      VERDICT="CLEARANCE-VERDICT: FAIL(${detail})"
      ;;
    *)
      VERDICT="CLEARANCE-VERDICT: DA-RESERVE(harness-error)"
      ;;
  esac
  printf '%s\n' "$VERDICT"
}

glob_matches_any() {
  local file="$1"
  local globs="$2"
  "$PYTHON_BIN" - "$file" "$globs" <<'PY'
import fnmatch
import sys
from pathlib import PurePosixPath

path = sys.argv[1].replace("\\", "/")
globs = sys.argv[2].split("|")
p = PurePosixPath(path)
for g in globs:
    g = g.strip().replace("\\", "/")
    if not g:
        continue
    if p.match(g):
        sys.exit(0)
    if "**" in g:
        prefix = g.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            sys.exit(0)
    if fnmatch.fnmatch(path, g.replace("**", "*")):
        sys.exit(0)
sys.exit(1)
PY
}

load_tsv() {
  local path="$1"
  local min_cols="$2"
  if [[ ! -f "$path" ]]; then
    return 1
  fi
  "$PYTHON_BIN" - "$path" "$min_cols" <<'PY'
import sys

path, min_cols = sys.argv[1], int(sys.argv[2])
rows = []
with open(path, encoding="utf-8", newline="") as f:
    for i, line in enumerate(f, 1):
        line = line.rstrip("\n\r")
        if not line.strip():
            continue
        if i == 1 and "\t" not in line and "class_id" not in line and "rung" not in line and "verdict" not in line:
            continue
        parts = line.split("\t")
        if len(parts) < min_cols:
            print(f"MALFORMED:{path}:{i}:{len(parts)}", file=sys.stderr)
            sys.exit(1)
        rows.append(parts)
print(len(rows))
PY
}

parse_args() {
  if [[ $# -eq 0 ]]; then
    usage
  fi
  case "${1:-}" in
    --selftest)
      FIXTURE_MODE="selftest"
      ;;
    --fixture)
      [[ $# -ge 2 ]] || usage
      FIXTURE_MODE="fixture"
      FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
      ;;
    --pr)
      [[ $# -ge 2 ]] || usage
      PR_NUMBER="$2"
      ;;
    --range)
      [[ $# -ge 2 ]] || usage
      RANGE_SPEC="$2"
      ;;
    -h|--help)
      usage
      ;;
    *)
      if [[ "$1" =~ ^[0-9]+$ ]]; then
        PR_NUMBER="$1"
      else
        usage
      fi
      ;;
  esac
}

resolve_paths() {
  local classes="${SCRIPT_DIR}/precedented_classes.tsv"
  local binding="${SCRIPT_DIR}/binding_conditions.tsv"
  local ledger="${SCRIPT_DIR}/clearance_ledger.tsv"
  if [[ -n "$FIXTURE_DIR" && -d "$FIXTURE_DIR" ]]; then
    [[ -f "${FIXTURE_DIR}/precedented_classes.tsv" ]] && classes="${FIXTURE_DIR}/precedented_classes.tsv"
    [[ -f "${FIXTURE_DIR}/binding_conditions.tsv" ]] && binding="${FIXTURE_DIR}/binding_conditions.tsv"
  fi
  printf '%s\n%s\n%s' "$classes" "$binding" "$ledger"
}

read_fixture_file() {
  local name="$1"
  local default="${2:-}"
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/${name}" ]]; then
    cat "${FIXTURE_DIR}/${name}"
    return 0
  fi
  printf '%s' "$default"
}

mark_requested_target() {
  if [[ -n "$PR_NUMBER" || -n "$RANGE_SPEC" ]]; then
    REQUESTED_TARGET=1
    return 0
  fi
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/target_mode.txt" ]]; then
    REQUESTED_TARGET=1
    return 0
  fi
  return 1
}

resolve_pr_changed_files() {
  local files repo
  if ! command -v gh >/dev/null 2>&1; then
    return 1
  fi
  files="$(gh pr diff "$PR_NUMBER" --name-only 2>/dev/null || true)"
  if [[ -n "$files" ]]; then
    CHANGED_FILES_LIST="$files"
    return 0
  fi
  repo="${GITHUB_REPOSITORY:-}"
  if [[ -z "$repo" ]]; then
    repo="$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || true)"
  fi
  if [[ -n "$repo" ]]; then
    files="$(gh api "repos/${repo}/pulls/${PR_NUMBER}/files" --paginate \
      --jq '.[].filename' 2>/dev/null || true)"
    if [[ -n "$files" ]]; then
      CHANGED_FILES_LIST="$files"
      return 0
    fi
  fi
  return 1
}

resolve_changed_files_once() {
  if [[ "$CHANGED_FILES_RESOLVED" -eq 1 ]]; then
    return 0
  fi
  CHANGED_FILES_RESOLVED=1
  mark_requested_target || true

  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/changed_files.txt" ]]; then
    CHANGED_FILES_LIST="$(sed '/^[[:space:]]*$/d' "${FIXTURE_DIR}/changed_files.txt" || true)"
    return 0
  fi
  if [[ -n "$RANGE_SPEC" ]]; then
    local base="${RANGE_SPEC%%..*}"
    local head="${RANGE_SPEC##*..}"
    CHANGED_FILES_LIST="$(git -C "$REPO_ROOT" diff --name-only "${base}" "${head}" 2>/dev/null || true)"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]]; then
    if resolve_pr_changed_files; then
      return 0
    fi
    PR_RESOLUTION_FAILED=1
    return 1
  fi
  return 1
}

changed_files_nonempty() {
  resolve_changed_files_once || true
  printf '%s\n' "$CHANGED_FILES_LIST" | sed '/^[[:space:]]*$/d' | grep -q .
}

changed_files() {
  resolve_changed_files_once || true
  printf '%s\n' "$CHANGED_FILES_LIST"
}

print_pr_resolution_remedy() {
  printf 'Unable to resolve PR diff locally; pass --range <base>..<head> or run in GHA with PR metadata.\n' >&2
}

pr_body_text() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/pr_body.txt" ]]; then
    cat "${FIXTURE_DIR}/pr_body.txt"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]] && command -v gh >/dev/null 2>&1; then
    gh pr view "$PR_NUMBER" --json body -q .body 2>/dev/null || true
    return 0
  fi
  printf ''
}

check_self_application() {
  local file
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    local gate
    for gate in "${GATE_WIRING_PATHS[@]}"; do
      if [[ "$file" == "$gate" ]]; then
        return 0
      fi
    done
    if [[ "$file" == scripts/ci/fixtures/clearance/* ]]; then
      return 0
    fi
  done < <(changed_files 2>/dev/null || true)
  return 1
}

detect_classes() {
  local classes_tsv="$1"
  local files
  files="$(changed_files 2>/dev/null || true)"
  "$PYTHON_BIN" - "$classes_tsv" "$files" <<'PY'
import csv
import fnmatch
import sys
from pathlib import PurePosixPath

classes_tsv, files_blob = sys.argv[1], sys.argv[2]
files = [f.strip().replace("\\", "/") for f in files_blob.splitlines() if f.strip()]

primary = ""
for f in files:
    if "tp_fleet_movement_0.rs" in f:
        primary = "tp-fleet-movement-rung"
        break
    if "tp_palma_reach_0.rs" in f:
        primary = "tp-palma-reach-rung"
        break
    if "tp_fronts_authoring_0.rs" in f:
        primary = "tp-fronts-authoring-rung"
        break
    if "tp_diplomacy_flow_0.rs" in f:
        primary = "tp-diplomacy-flow-rung"
        break
    if "suspended_demo" in f:
        primary = "tp-suspended-demo"
        break

def glob_match(path: str, pattern: str) -> bool:
    p = PurePosixPath(path)
    if p.match(pattern):
        return True
    if "**" in pattern:
        prefix = pattern.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, pattern.replace("**", "*"))

rows = []
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id", "rung", "verdict"):
            continue
        rows.append(row)

class_ids = set()
for row in rows:
    if len(row) < 6:
        continue
    class_id, scope_globs, _env, _reqs, status, _blocker = row[:6]
    if status == "retired":
        continue
    if primary and class_id != primary:
        continue
    for path in files:
        for glob_pat in scope_globs.split("|"):
            glob_pat = glob_pat.strip()
            if glob_pat and glob_match(path, glob_pat):
                class_ids.add(class_id)
                break

for cid in sorted(class_ids):
    print(cid)
PY
}

class_for_rung() {
  local class_id="$1"
  case "$class_id" in
    tp-fleet-movement-rung) printf 'TP-FLEET-MOVEMENT-0' ;;
    tp-palma-reach-rung) printf 'TP-PALMA-REACH-0' ;;
    tp-fronts-authoring-rung) printf 'TP-FRONTS-AUTHORING-0' ;;
    tp-diplomacy-flow-rung) printf 'TP-DIPLOMACY-FLOW-0' ;;
    tp-workshop-scenario-rung)
      local files
      files="$(changed_files 2>/dev/null || true)"
      if echo "$files" | grep -q 'tp_fleet_movement'; then printf 'TP-FLEET-MOVEMENT-0'
      elif echo "$files" | grep -q 'tp_palma_reach'; then printf 'TP-PALMA-REACH-0'
      elif echo "$files" | grep -q 'tp_fronts_authoring'; then printf 'TP-FRONTS-AUTHORING-0'
      elif echo "$files" | grep -q 'tp_diplomacy_flow'; then printf 'TP-DIPLOMACY-FLOW-0'
      else printf 'TP-WORKSHOP-SCENARIO-RUNG'
      fi
      ;;
    *) printf "$class_id" ;;
  esac
}

check_binding_conditions() {
  local binding_tsv="$1"
  local rung="$2"
  "$PYTHON_BIN" - "$binding_tsv" "$rung" <<'PY'
import csv
import sys

binding_tsv, rung = sys.argv[1], sys.argv[2]
with open(binding_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("rung", "verdict"):
            continue
        if len(row) < 4:
            continue
        cond_rung, _cond, _set_by, status = row[:4]
        if cond_rung == rung and status == "open":
            sys.exit(0)
sys.exit(1)
PY
}

class_status() {
  local classes_tsv="$1"
  local class_id="$2"
  "$PYTHON_BIN" - "$classes_tsv" "$class_id" <<'PY'
import csv
import sys

classes_tsv, class_id = sys.argv[1], sys.argv[2]
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id",):
            continue
        if row[0] == class_id and len(row) >= 5:
            print(row[4])
            sys.exit(0)
sys.exit(0)
PY
}

class_requirements() {
  local classes_tsv="$1"
  local class_id="$2"
  "$PYTHON_BIN" - "$classes_tsv" "$class_id" <<'PY'
import csv
import sys

classes_tsv, class_id = sys.argv[1], sys.argv[2]
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id",):
            continue
        if row[0] == class_id and len(row) >= 4:
            print(row[3])
            sys.exit(0)
sys.exit(0)
PY
}

check_workshop_only() {
  local file
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
      crates/simthing-workshop/*|docs/tests/*|scripts/ci/test_inventory.tsv|scripts/ci/test_lifecycle_boundary_rows.tsv)
        ;;
      *)
        return 1
        ;;
    esac
  done < <(changed_files 2>/dev/null || true)
  return 0
}

check_no_engine_crate() {
  local file prefix
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    for prefix in "${ENGINE_CRATE_PREFIXES[@]}"; do
      if [[ "$file" == "$prefix"* ]]; then
        return 1
      fi
    done
  done < <(changed_files 2>/dev/null || true)
  return 0
}

check_required_pr_body_fields() {
  local body="$1"
  if ! echo "$body" | grep -qiE 'tested_code_sha:[[:space:]]*[0-9a-f]{8,}'; then
    emit_verdict fail "missing-tested-code-sha: add tested_code_sha and coverage_basis"
    return 1
  fi
  if ! echo "$body" | grep -qiE 'coverage_basis:[[:space:]]*PASS'; then
    emit_verdict fail "missing-tested-code-sha: add tested_code_sha and coverage_basis"
    return 1
  fi
  return 0
}

check_recorded_gpu_proof() {
  local body="$1"
  if echo "$body" | grep -q 'DOCTRINE-TESTS-VERDICT:[[:space:]]*PASS'; then
    return 0
  fi
  if echo "$body" | grep -qiE 'DOCTRINE-TESTS-VERDICT'; then
    emit_verdict fail "missing-gpu-proof: add citable DOCTRINE-TESTS-VERDICT bound to tested_code_sha"
    return 1
  fi
  emit_verdict fail "missing-gpu-proof: add citable DOCTRINE-TESTS-VERDICT bound to tested_code_sha"
  return 1
}

check_ci_status() {
  if [[ -n "$FIXTURE_DIR" ]]; then
    local status
    status="$(read_fixture_file ci_status.txt green)"
    if [[ "$status" != "green" ]]; then
      emit_verdict fail "ci-not-green: rerun/fix failing checks before clearance"
      return 1
    fi
    return 0
  fi
  if [[ -z "$PR_NUMBER" ]] || ! command -v gh >/dev/null 2>&1; then
    return 0
  fi
  local failing
  failing="$(gh pr checks "$PR_NUMBER" 2>/dev/null | awk '$2 != "pass" && $2 != "skipping" && NF {print}' || true)"
  if [[ -n "$failing" ]]; then
    emit_verdict fail "ci-not-green: rerun/fix failing checks before clearance"
    return 1
  fi
  return 0
}

append_ledger() {
  local ledger="$1"
  local class="$2"
  local pr="$3"
  local sha="$4"
  local sketch="${5:-}"
  local date
  date="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u)"
  if [[ ! -f "$ledger" ]]; then
    printf 'verdict\tclass\tpr\tsha\tdate\tsketch\n' >"$ledger"
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$VERDICT" "$class" "$pr" "$sha" "$date" "$sketch" >>"$ledger"
}

route_clearance() {
  local classes_tsv="$1"
  local binding_tsv="$2"
  local ledger_tsv="$3"

  resolve_changed_files_once || true

  if ! load_tsv "$classes_tsv" 6 >/dev/null 2>&1; then
    emit_verdict reserve "harness-error"
    return 0
  fi
  if ! load_tsv "$binding_tsv" 5 >/dev/null 2>&1; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  if [[ "$PR_RESOLUTION_FAILED" -eq 1 ]]; then
    print_pr_resolution_remedy
    emit_verdict reserve "harness-error"
    return 0
  fi

  if [[ "$REQUESTED_TARGET" -eq 1 ]] && ! changed_files_nonempty; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  if check_self_application; then
    emit_verdict reserve "gate-wiring"
    return 0
  fi

  local classes
  classes="$(detect_classes "$classes_tsv")"
  if [[ -z "$classes" ]]; then
    emit_verdict reserve "novelty"
    return 0
  fi

  local class_count
  class_count="$(printf '%s\n' "$classes" | sed '/^$/d' | wc -l | tr -d ' ')"
  if [[ "$class_count" -gt 1 ]]; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  local class_id
  class_id="$(printf '%s\n' "$classes" | head -n 1)"
  local status
  status="$(class_status "$classes_tsv" "$class_id")"
  if [[ "$status" == "suspended" ]]; then
    emit_verdict reserve "class-suspended"
    return 0
  fi

  local rung
  rung="$(class_for_rung "$class_id")"
  if check_binding_conditions "$binding_tsv" "$rung"; then
    emit_verdict reserve "binding-conditions"
    return 0
  fi

  local reqs body
  reqs="$(class_requirements "$classes_tsv" "$class_id")"
  body="$(pr_body_text)"

  if [[ "$reqs" == *workshop_only* ]] && ! check_workshop_only; then
    emit_verdict reserve "novelty"
    return 0
  fi
  if [[ "$reqs" == *no_engine_crate* ]] && ! check_no_engine_crate; then
    emit_verdict reserve "novelty"
    return 0
  fi

  if [[ "$reqs" == *tested_code_sha* || "$reqs" == *coverage_basis* ]]; then
    if ! check_required_pr_body_fields "$body"; then
      return 0
    fi
  fi

  if [[ "$reqs" == *gpu_proof* ]]; then
    if ! check_recorded_gpu_proof "$body"; then
      return 0
    fi
  fi

  if [[ "$reqs" == *ci_green* ]]; then
    if ! check_ci_status; then
      return 0
    fi
  fi

  emit_verdict clearable
}

reset_clearance_state() {
  REQUESTED_TARGET=0
  CHANGED_FILES_RESOLVED=0
  CHANGED_FILES_LIST=""
  PR_RESOLUTION_FAILED=0
}

run_fixture() {
  local name="$1"
  reset_clearance_state
  FIXTURE_DIR="${FIXTURES_ROOT}/${name}"
  [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected got
  expected="$(cat "${FIXTURE_DIR}/expected_verdict.txt" | tr -d '\r' | head -n 1)"
  local paths
  paths="$(resolve_paths)"
  local classes_tsv binding_tsv ledger_tsv
  classes_tsv="$(echo "$paths" | sed -n '1p')"
  binding_tsv="$(echo "$paths" | sed -n '2p')"
  ledger_tsv="$(mktemp "${TMPDIR:-/tmp}/clearance-ledger-XXXXXX")"
  printf 'verdict\tclass\tpr\tsha\tdate\tsketch\n' >"$ledger_tsv"
  got="$(route_clearance "$classes_tsv" "$binding_tsv" "$ledger_tsv" | tail -n 1)"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  return 1
}

run_selftest() {
  local fixtures=(
    clearance_selftest_clearable_1150_shape
    clearance_selftest_clearable_1151_shape
    clearance_selftest_clearable_1152_shape
    clearance_selftest_reserve_1154_binding_conditions
    clearance_selftest_fail_closed_malformed_tsv
    clearance_selftest_fail_closed_ambiguous_class
    clearance_selftest_gate_wiring_self_application
    clearance_selftest_suspended_class
    clearance_selftest_missing_required_proof_fields
    clearance_selftest_fail_closed_empty_requested_diff
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    emit_verdict clearable
    echo "CLEARANCE-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  emit_verdict reserve "harness-error"
  echo "CLEARANCE-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    reset_clearance_state
    local paths
    paths="$(resolve_paths)"
    route_clearance "$(echo "$paths" | sed -n '1p')" "$(echo "$paths" | sed -n '2p')" "$(echo "$paths" | sed -n '3p')"
    exit 0
  fi

  reset_clearance_state
  local paths
  paths="$(resolve_paths)"
  local classes_tsv binding_tsv ledger_tsv
  classes_tsv="$(echo "$paths" | sed -n '1p')"
  binding_tsv="$(echo "$paths" | sed -n '2p')"
  ledger_tsv="$(echo "$paths" | sed -n '3p')"
  route_clearance "$classes_tsv" "$binding_tsv" "$ledger_tsv"

  if [[ "${CLEARANCE_LEDGER_APPEND:-}" == "1" && -n "$PR_NUMBER" ]]; then
    local sha class_id
    sha="${GITHUB_SHA:-$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)}"
    class_id="$(detect_classes "$classes_tsv" | head -n 1 || echo unknown)"
    append_ledger "$ledger_tsv" "$class_id" "$PR_NUMBER" "$sha" ""
  fi
}

main "$@"