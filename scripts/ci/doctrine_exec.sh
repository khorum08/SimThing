#!/usr/bin/env bash
# CI-B-GH-CPU-0: GitHub-side executable proof (non-blocking Track B).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${ROOT}/scripts/ci/doctrine_exec_profiles.tsv"
REPORT_JSON="${DOCTRINE_EXEC_REPORT_JSON:-doctrine_exec_report.json}"
REPORT_TXT="${DOCTRINE_EXEC_REPORT_TXT:-doctrine-exec-report.txt}"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

PROFILE="${DOCTRINE_EXEC_PROFILE:-ci-b-webchat-smoke}"
MODE="${DOCTRINE_EXEC_MODE:-run}"
PROBE="${DOCTRINE_EXEC_PROBE:-}"
OWNER_DEEP_ALLOWED="${DOCTRINE_EXEC_OWNER_DEEP_ALLOWED:-false}"
COMMAND_TIMEOUT="${DOCTRINE_EXEC_COMMAND_TIMEOUT_SECONDS:-300}"
SMOKE_TIMEOUT="${DOCTRINE_EXEC_SMOKE_TIMEOUT_SECONDS:-120}"
EXHAUSTIVE="${DOCTRINE_EXEC_EXHAUSTIVE:-0}"

PR="${DOCTRINE_EXEC_PR:-}"
HEAD_SHA="${DOCTRINE_EXEC_HEAD_SHA:-}"
BASE_SHA="${DOCTRINE_EXEC_BASE_SHA:-}"
TESTED_REF="${DOCTRINE_EXEC_TESTED_REF:-}"
MERGE_REF_STATUS="${DOCTRINE_EXEC_MERGE_REF_STATUS:-UNAVAILABLE}"
WORKFLOW_RUN_ID="${GITHUB_RUN_ID:-local}"
JOB_ID="${GITHUB_JOB:-local}"

PROFILE_CLASS=""
RISK_CLASS=""
GPU_REQUIRED="no"
EXPECTED_VERDICT="PASS"
ACTIVE_TIMEOUT="$COMMAND_TIMEOUT"
FAIL_FAST="${DOCTRINE_EXEC_FAIL_FAST:-1}"

FAILURES=0
INSPECTS=0
STOP_LAUNCHING=0
FAILURE_LINES=()
INSPECT_LINES=()
SURFACE_TRUTH_REQUIRED="yes"

cd "$ROOT"

log() {
  echo "$*" | tee -a "$REPORT_TXT"
}

resolve_profile() {
  local want="$1"
  while IFS=$'\t' read -r profile_id profile_class risk_class crate_checks tests doc_tests gpu_required expected_verdict; do
    [[ "$profile_id" == "profile_id" ]] && continue
    [[ -z "${profile_id// }" ]] && continue
    if [[ "$profile_id" == "$want" ]]; then
      echo "$profile_id|$profile_class|$risk_class|$crate_checks|$tests|$doc_tests|$gpu_required|$expected_verdict"
      return 0
    fi
  done < "$PROFILES"
  return 1
}

write_json() {
  local verdict="$1"
  "$PYTHON_BIN" - <<'PY' "$REPORT_JSON" "$REPORT_TXT" "$verdict" "$PR" "$HEAD_SHA" "$BASE_SHA" "$TESTED_REF" "$MERGE_REF_STATUS" "$WORKFLOW_RUN_ID" "$JOB_ID" "$PROFILE" "$PROFILE_CLASS" "$FAILURES" "$INSPECTS" "$ROOT/scripts/ci/triage_log.tsv" "$OWNER_DEEP_ALLOWED" "$ACTIVE_TIMEOUT" "$FAIL_FAST"
import json
import pathlib
import sys

(
    out,
    report_txt,
    verdict,
    pr,
    head,
    base,
    tested,
    merge_status,
    run_id,
    job_id,
    profile,
    profile_class,
    failures,
    inspects,
    triage_path,
    owner_deep,
    timeout_seconds,
    fail_fast,
) = sys.argv[1:19]

commands = []
tests = []
failure_rows = []
inspect_rows = []
section = None
report = pathlib.Path(report_txt)
if report.exists():
    for line in report.read_text(encoding="utf-8", errors="replace").splitlines():
        stripped = line.strip()
        if stripped == "--- failures ---":
            section = "failures"
            continue
        if stripped == "--- inspect ---":
            section = "inspect"
            continue
        if line.startswith("DOCTRINE-EXEC-VERDICT:"):
            section = None
            continue
        if line.startswith("+ "):
            cmd = line[2:].strip()
            commands.append(cmd)
            tests.append(cmd)
        elif section == "failures" and stripped and stripped != "(none)":
            failure_rows.append(stripped)
        elif section == "inspect" and stripped and stripped != "(none)":
            inspect_rows.append(stripped)

triage_rows = []
triage_file = pathlib.Path(triage_path)
if triage_file.exists():
    for line in triage_file.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith("scan-id") or not line.strip():
            continue
        triage_rows.append(line.strip())

payload = {
    "artifact_version": "doctrine-exec.v1",
    "verdict": verdict,
    "pr": int(pr) if pr and pr.isdigit() else (pr or None),
    "head_sha": head or None,
    "base_sha": base or None,
    "tested_ref": tested or None,
    "merge_ref_status": merge_status,
    "workflow_run_id": str(run_id),
    "job_id": job_id,
    "profile": profile,
    "profile_class": profile_class or None,
    "owner_deep": owner_deep == "true",
    "command_timeout_seconds": int(timeout_seconds) if timeout_seconds.isdigit() else timeout_seconds,
    "fail_fast": fail_fast == "1",
    "failures": int(failures),
    "inspect_entries": int(inspects),
    "tests": tests,
    "commands": commands,
    "failure_details": failure_rows,
    "inspect_details": inspect_rows,
    "triage_rows": triage_rows,
}
with open(out, "w", encoding="utf-8") as f:
    json.dump(payload, f, indent=2)
PY
}

emit_footer_and_json() {
  local verdict="$1"
  {
    echo "  --- failures ---"
    if [[ "${#FAILURE_LINES[@]}" -eq 0 ]]; then
      echo "  (none)"
    else
      printf '  %s\n' "${FAILURE_LINES[@]}"
    fi
    echo "  --- inspect ---"
    if [[ "${#INSPECT_LINES[@]}" -eq 0 ]]; then
      echo "  (none)"
    else
      printf '  %s\n' "${INSPECT_LINES[@]}"
    fi
    echo "DOCTRINE-EXEC-VERDICT: ${verdict} failures=${FAILURES} inspect=${INSPECTS} profile=${PROFILE} profile_class=${PROFILE_CLASS:-unknown} owner_deep=${OWNER_DEEP_ALLOWED} head_sha=${HEAD_SHA:-unknown}"
  } | tee -a "$REPORT_TXT"
  write_json "$verdict"
}

finish() {
  local verdict="PASS"
  if [[ "$FAILURES" -gt 0 ]]; then
    verdict="FAIL"
  elif [[ "$INSPECTS" -gt 0 ]] || [[ "$MERGE_REF_STATUS" == "UNAVAILABLE" ]]; then
    verdict="INSPECT"
  fi
  emit_footer_and_json "$verdict"
  [[ "$verdict" == "FAIL" ]] && exit 1
  exit 0
}

run_with_timeout() {
  local seconds="$1"
  shift
  if command -v timeout >/dev/null 2>&1; then
    timeout "${seconds}s" "$@"
  else
    "$@"
  fi
}

record_failure() {
  local message="$1"
  FAILURE_LINES+=("$message")
  FAILURES=$((FAILURES + 1))
  if [[ "$FAIL_FAST" == "1" && "$EXHAUSTIVE" != "1" ]]; then
    STOP_LAUNCHING=1
  fi
}

run_cmd() {
  local label="$1"
  shift
  [[ "$STOP_LAUNCHING" -eq 0 ]] || return 0
  log "+ $*"
  local ec=0
  set +e
  run_with_timeout "$ACTIVE_TIMEOUT" "$@"
  ec=$?
  set -e
  if [[ "$ec" -eq 0 ]]; then
    return 0
  fi
  if [[ "$ec" -eq 124 ]]; then
    record_failure "$label timed out after ${ACTIVE_TIMEOUT}s"
  else
    record_failure "$label failed (exit $ec)"
  fi
  return 0
}

run_inspect_cmd() {
  local label="$1"
  shift
  [[ "$STOP_LAUNCHING" -eq 0 ]] || return 0
  log "+ (inspect) $*"
  local ec=0
  set +e
  run_with_timeout "$ACTIVE_TIMEOUT" "$@"
  ec=$?
  set -e
  if [[ "$ec" -eq 0 ]]; then
    return 0
  fi
  if [[ "$ec" -eq 124 ]]; then
    record_failure "$label timed out after ${ACTIVE_TIMEOUT}s"
  else
    INSPECT_LINES+=("$label skipped or inconclusive (GPU/local)")
    INSPECTS=$((INSPECTS + 1))
  fi
  return 0
}

run_semicolon_commands() {
  local value="$1"
  local cmd
  IFS=';' read -ra CMDS <<< "$value"
  for cmd in "${CMDS[@]}"; do
    cmd="${cmd#"${cmd%%[![:space:]]*}"}"
    cmd="${cmd%"${cmd##*[![:space:]]}"}"
    [[ -n "$cmd" ]] || continue
    if [[ "$GPU_REQUIRED" == "yes" && "$cmd" == *"simthing-gpu"* ]]; then
      run_inspect_cmd "$cmd" bash -c "$cmd"
    else
      run_cmd "$cmd" bash -c "$cmd"
    fi
  done
}

start_report() {
  : > "$REPORT_TXT"
  {
    echo "DOCTRINE EXEC REPORT  (head ${HEAD_SHA:-unknown}, $(date -u +%Y-%m-%dT%H:%M:%SZ))"
    echo "  profile: $PROFILE"
    echo "  profile_class: ${PROFILE_CLASS:-unknown}"
    echo "  owner_deep: ${OWNER_DEEP_ALLOWED}"
    echo "  command_timeout_seconds: ${ACTIVE_TIMEOUT}"
    echo "  fail_fast: ${FAIL_FAST}"
    echo "  pr: ${PR:-n/a}"
    echo "  head_sha: ${HEAD_SHA:-n/a}"
    echo "  base_sha: ${BASE_SHA:-n/a}"
    echo "  tested_ref: ${TESTED_REF:-n/a}"
    echo "  merge_ref_status: ${MERGE_REF_STATUS}"
    echo "  workflow_run_id: ${WORKFLOW_RUN_ID}"
    echo "  job_id: ${JOB_ID}"
    echo "  --- commands ---"
  } | tee -a "$REPORT_TXT"
}

if [[ "$MODE" == "probe" ]]; then
  PROFILE="probe:${PROBE}"
  PROFILE_CLASS="probe"
  ACTIVE_TIMEOUT="${DOCTRINE_EXEC_PROBE_TIMEOUT_SECONDS:-$COMMAND_TIMEOUT}"
  start_report
  probe_out="$(mktemp)"
  set +e
  run_with_timeout "$ACTIVE_TIMEOUT" bash "${ROOT}/scripts/ci/doctrine_exec_probes.sh" "$PROBE" 2>&1 | tee "$probe_out"
  probe_ec=${PIPESTATUS[0]}
  set -e
  cat "$probe_out" | tee -a "$REPORT_TXT"
  if [[ "$probe_ec" -eq 124 ]]; then
    record_failure "probe:${PROBE} timed out after ${ACTIVE_TIMEOUT}s"
  elif [[ "$probe_ec" -ne 0 ]]; then
    record_failure "probe:${PROBE} failed (exit $probe_ec)"
  fi
  rm -f "$probe_out"
  finish
fi

if [[ "$MODE" == "plan" ]]; then
  DOCTRINE_EXEC_PROFILE="$PROFILE" DOCTRINE_EXEC_OWNER_DEEP_ALLOWED="$OWNER_DEEP_ALLOWED" \
    bash "${ROOT}/scripts/ci/doctrine_exec_plan.sh" --profile "$PROFILE"
  exit 0
fi

profile_line="$(resolve_profile "$PROFILE" || true)"
if [[ -z "$profile_line" ]]; then
  PROFILE_CLASS="unknown"
  start_report
  record_failure "unknown profile: $PROFILE"
  finish
fi

IFS='|' read -r _profile_id PROFILE_CLASS RISK_CLASS crate_checks tests doc_tests GPU_REQUIRED EXPECTED_VERDICT <<< "$profile_line"
[[ "$crate_checks" == "-" ]] && crate_checks=""
[[ "$tests" == "-" ]] && tests=""
[[ "$doc_tests" == "-" ]] && doc_tests=""
if [[ "$RISK_CLASS" == test-deletion-* ]]; then
  SURFACE_TRUTH_REQUIRED="no"
fi
if [[ "$PROFILE_CLASS" == "smoke" ]]; then
  ACTIVE_TIMEOUT="$SMOKE_TIMEOUT"
fi
if [[ "$PROFILE_CLASS" == "owner-deep" ]]; then
  FAIL_FAST="${DOCTRINE_EXEC_FAIL_FAST:-0}"
fi

start_report

if [[ "$PROFILE_CLASS" == "owner-deep" && "$OWNER_DEEP_ALLOWED" != "true" ]]; then
  record_failure "owner-deep profile ${PROFILE} rejected: workflow_dispatch owner_deep=true is required"
  finish
fi

IFS=',' read -ra CHECK_CRATES <<< "$crate_checks"
for crate in "${CHECK_CRATES[@]}"; do
  crate="${crate// /}"
  [[ -n "$crate" ]] || continue
  run_cmd "cargo check -p $crate" cargo check -p "$crate"
done

run_semicolon_commands "$tests"
run_semicolon_commands "$doc_tests"

if [[ "$PROFILE_CLASS" == "owner-deep" && "$STOP_LAUNCHING" -eq 0 ]]; then
  workspace_check="cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-feeder -p simthing-sim -p simthing-driver -p simthing-spec -p simthing-workshop -p simthing-clausething -p simthing-mapgenerator -p simthing-tools"
  run_cmd "workspace check (minus mapeditor)" bash -c "$workspace_check"
fi

if [[ "$PROFILE_CLASS" != "smoke" && "$SURFACE_TRUTH_REQUIRED" == "yes" && "$STOP_LAUNCHING" -eq 0 ]]; then
  # shellcheck source=doctrine_surface_truth_inspect.sh
  source "${ROOT}/scripts/ci/doctrine_surface_truth_inspect.sh"
  log "+ bash scripts/ci/doctrine_surface_truth.sh"
  surface_out="$(bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh" 2>&1 | tee -a "$REPORT_TXT")"
  inspect_line=""
  set +e
  inspect_line="$(surface_truth_inspect_line_from_output "$surface_out")"
  inspect_map_ec=$?
  set -e
  if [[ "$inspect_map_ec" -eq 0 && -z "$inspect_line" ]]; then
    :
  elif [[ "$inspect_map_ec" -eq 0 && -n "$inspect_line" ]]; then
    INSPECT_LINES+=("$inspect_line")
    INSPECTS=$((INSPECTS + 1))
  else
    record_failure "${inspect_line:-surface-truth unexpected output}"
  fi
fi

finish
