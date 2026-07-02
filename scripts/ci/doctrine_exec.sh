#!/usr/bin/env bash
# CI-B-GH-CPU-0: GitHub-side CPU executable proof (non-blocking Track B).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${ROOT}/scripts/ci/doctrine_exec_profiles.tsv"
REPORT_JSON="${DOCTRINE_EXEC_REPORT_JSON:-doctrine_exec_report.json}"
REPORT_TXT="${DOCTRINE_EXEC_REPORT_TXT:-doctrine-exec-report.txt}"

PROFILE="${DOCTRINE_EXEC_PROFILE:-full-cpu}"
MODE="${DOCTRINE_EXEC_MODE:-run}"
PROBE="${DOCTRINE_EXEC_PROBE:-}"

PR="${DOCTRINE_EXEC_PR:-}"
HEAD_SHA="${DOCTRINE_EXEC_HEAD_SHA:-}"
BASE_SHA="${DOCTRINE_EXEC_BASE_SHA:-}"
TESTED_REF="${DOCTRINE_EXEC_TESTED_REF:-}"
MERGE_REF_STATUS="${DOCTRINE_EXEC_MERGE_REF_STATUS:-UNAVAILABLE}"
WORKFLOW_RUN_ID="${GITHUB_RUN_ID:-local}"
JOB_ID="${GITHUB_JOB:-local}"

FAILURES=0
INSPECTS=0
FAILURE_LINES=()
INSPECT_LINES=()

run_cmd() {
  local label="$1"
  shift
  local ec=0
  echo "+ $*"
  set +e
  "$@"
  ec=$?
  set -e
  if [[ "$ec" -eq 0 ]]; then
    return 0
  fi
  FAILURE_LINES+=("$label failed (exit $ec)")
  FAILURES=$((FAILURES + 1))
  return 0
}

run_inspect_cmd() {
  local label="$1"
  shift
  local ec=0
  echo "+ (inspect) $*"
  set +e
  "$@"
  ec=$?
  set -e
  if [[ "$ec" -eq 0 ]]; then
    return 0
  fi
  INSPECT_LINES+=("$label skipped or inconclusive (GPU/local)")
  INSPECTS=$((INSPECTS + 1))
  return 0
}

resolve_profile() {
  local want="$1"
  while IFS=$'\t' read -r profile_id risk_class crate_checks tests doc_tests gpu_required expected_verdict; do
    [[ "$profile_id" == "profile_id" ]] && continue
    [[ -z "${profile_id// }" ]] && continue
    if [[ "$profile_id" == "$want" ]]; then
      echo "$profile_id|$risk_class|$crate_checks|$tests|$doc_tests|$gpu_required|$expected_verdict"
      return 0
    fi
  done < "$PROFILES"
  return 1
}

cd "$ROOT"

if [[ "$MODE" == "plan" ]]; then
  DOCTRINE_EXEC_PROFILE="$PROFILE" bash "${ROOT}/scripts/ci/doctrine_exec_plan.sh" --profile "$PROFILE"
  exit 0
fi

if [[ "$MODE" == "probe" ]]; then
  probe_out="$(mktemp)"
  if bash "${ROOT}/scripts/ci/doctrine_exec_probes.sh" "$PROBE" 2>&1 | tee "$probe_out"; then
    VERDICT="PASS"
  else
    VERDICT="FAIL"
  fi
  {
    echo "DOCTRINE EXEC PROBE REPORT (probe ${PROBE}, head ${HEAD_SHA:-unknown})"
    echo "  pr: ${PR:-n/a}"
    echo "  head_sha: ${HEAD_SHA:-n/a}"
    echo "  base_sha: ${BASE_SHA:-n/a}"
    echo "  tested_ref: ${TESTED_REF:-n/a}"
    echo "  merge_ref_status: ${MERGE_REF_STATUS}"
    echo "  workflow_run_id: ${WORKFLOW_RUN_ID}"
    echo "  job_id: ${JOB_ID}"
    cat "$probe_out"
    probe_failures=0
    [[ "$VERDICT" == "FAIL" ]] && probe_failures=1
    echo "DOCTRINE-EXEC-VERDICT: ${VERDICT} failures=${probe_failures} inspect=0 profile=probe:${PROBE} head_sha=${HEAD_SHA:-unknown}"
  } | tee "$REPORT_TXT"
  python3 - <<'PY' "$REPORT_JSON" "$VERDICT" "$PR" "$HEAD_SHA" "$BASE_SHA" "$TESTED_REF" "$MERGE_REF_STATUS" "$WORKFLOW_RUN_ID" "$JOB_ID" "probe:${PROBE}" "$ROOT/scripts/ci/triage_log.tsv"
import json, pathlib, sys
out, verdict, pr, head, base, tested, merge_status, run_id, job_id, profile, triage_path = sys.argv[1:12]
commands = []
report_txt = pathlib.Path(out).with_name("doctrine-exec-report.txt")
if report_txt.exists():
    for line in report_txt.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith("PROBE "):
            commands.append(line.strip())
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
    "failures": 1 if verdict == "FAIL" else 0,
    "inspect_entries": 0,
    "tests": [profile],
    "commands": commands,
    "failure_details": [],
    "inspect_details": [],
    "triage_rows": triage_rows,
}
with open(out, "w", encoding="utf-8") as f:
    json.dump(payload, f, indent=2)
PY
  rm -f "$probe_out"
  [[ "$VERDICT" == "FAIL" ]] && exit 1
  exit 0
fi

profile_line="$(resolve_profile "$PROFILE" || true)"
if [[ -z "$profile_line" ]]; then
  echo "unknown profile: $PROFILE" >&2
  exit 1
fi

IFS='|' read -r _profile_id _risk_class crate_checks tests doc_tests gpu_required expected_verdict <<< "$profile_line"

{
  echo "DOCTRINE EXEC REPORT  (head ${HEAD_SHA:-unknown}, $(date -u +%Y-%m-%dT%H:%M:%SZ))"
  echo "  profile: $PROFILE"
  echo "  pr: ${PR:-n/a}"
  echo "  head_sha: ${HEAD_SHA:-n/a}"
  echo "  base_sha: ${BASE_SHA:-n/a}"
  echo "  tested_ref: ${TESTED_REF:-n/a}"
  echo "  merge_ref_status: ${MERGE_REF_STATUS}"
  echo "  workflow_run_id: ${WORKFLOW_RUN_ID}"
  echo "  job_id: ${JOB_ID}"
  echo "  --- commands ---"
} | tee "$REPORT_TXT"

IFS=',' read -ra CHECK_CRATES <<< "$crate_checks"
for crate in "${CHECK_CRATES[@]}"; do
  crate="${crate// /}"
  [[ -n "$crate" ]] || continue
  run_cmd "cargo check -p $crate" cargo check -p "$crate"
done

IFS=';' read -ra TEST_CMDS <<< "$tests"
for cmd in "${TEST_CMDS[@]}"; do
  cmd="${cmd#"${cmd%%[![:space:]]*}"}"
  cmd="${cmd%"${cmd##*[![:space:]]}"}"
  [[ -n "$cmd" ]] || continue
  if [[ "$gpu_required" == "yes" ]] && [[ "$cmd" == *"simthing-gpu"* ]]; then
    run_inspect_cmd "$cmd" bash -lc "$cmd" || true
  else
    run_cmd "$cmd" bash -lc "$cmd"
  fi
done

if [[ -n "$doc_tests" ]]; then
  run_cmd "$doc_tests" bash -lc "$doc_tests"
fi

if [[ "$PROFILE" == "full-cpu" ]]; then
  workspace_check="cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-feeder -p simthing-sim -p simthing-driver -p simthing-spec -p simthing-workshop -p simthing-clausething -p simthing-mapgenerator -p simthing-tools"
  run_cmd "workspace check (minus mapeditor)" bash -lc "$workspace_check"
fi

surface_out="$(bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh" 2>&1 | tee -a "$REPORT_TXT")"
echo "+ bash scripts/ci/doctrine_surface_truth.sh" | tee -a "$REPORT_TXT"
if echo "$surface_out" | grep -q 'SURFACE-TRUTH: PASS'; then
  :
elif echo "$surface_out" | grep -q 'SURFACE-TRUTH: INSPECT'; then
  INSPECT_LINES+=("surface-truth divergence or tooling gap")
  INSPECTS=$((INSPECTS + 1))
else
  FAILURE_LINES+=("surface-truth unexpected output")
  FAILURES=$((FAILURES + 1))
fi

VERDICT="PASS"
if [[ "$FAILURES" -gt 0 ]]; then
  VERDICT="FAIL"
elif [[ "$INSPECTS" -gt 0 ]] || [[ "$MERGE_REF_STATUS" == "UNAVAILABLE" ]]; then
  VERDICT="INSPECT"
fi

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
  echo "DOCTRINE-EXEC-VERDICT: ${VERDICT} failures=${FAILURES} inspect=${INSPECTS} profile=${PROFILE} head_sha=${HEAD_SHA:-unknown}"
} | tee -a "$REPORT_TXT"

python3 - <<'PY' "$REPORT_JSON" "$VERDICT" "$PR" "$HEAD_SHA" "$BASE_SHA" "$TESTED_REF" "$MERGE_REF_STATUS" "$WORKFLOW_RUN_ID" "$JOB_ID" "$PROFILE" "$FAILURES" "$INSPECTS" "$ROOT/scripts/ci/triage_log.tsv"
import json, pathlib, sys
out, verdict, pr, head, base, tested, merge_status, run_id, job_id, profile, failures, inspects, triage_path = sys.argv[1:14]
commands = []
tests = []
failure_rows = []
inspect_rows = []
section = None
report_txt = pathlib.Path(out).with_name("doctrine-exec-report.txt")
if report_txt.exists():
    for line in report_txt.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.strip() == "--- failures ---":
            section = "failures"
            continue
        if line.strip() == "--- inspect ---":
            section = "inspect"
            continue
        if line.startswith("+ "):
            cmd = line[2:].strip()
            commands.append(cmd)
            tests.append(cmd.split(" ", 2)[0:2][-1] if cmd.startswith("cargo") else cmd)
        elif section == "failures" and line.strip() and line.strip() != "(none)":
            failure_rows.append(line.strip())
        elif section == "inspect" and line.strip() and line.strip() != "(none)":
            inspect_rows.append(line.strip())
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

if [[ "$VERDICT" == "FAIL" ]]; then
  exit 1
fi
exit 0