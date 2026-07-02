#!/usr/bin/env bash
# Resolve doctrine-exec profile/probe commands without running them (plan mode).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${ROOT}/scripts/ci/doctrine_exec_profiles.tsv"
PROFILE="${DOCTRINE_EXEC_PROFILE:-full-cpu}"

usage() {
  echo "usage: $0 [--profile <id>]"
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      PROFILE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    *)
      echo "unknown arg: $1" >&2
      usage
      ;;
  esac
done

if [[ ! -f "$PROFILES" ]]; then
  echo "missing profiles: $PROFILES" >&2
  exit 1
fi

match=""
while IFS=$'\t' read -r profile_id risk_class crate_checks tests doc_tests gpu_required expected_verdict; do
  [[ "$profile_id" == "profile_id" ]] && continue
  [[ -z "${profile_id// }" ]] && continue
  if [[ "$profile_id" == "$PROFILE" ]]; then
    match="$profile_id|$risk_class|$crate_checks|$tests|$doc_tests|$gpu_required|$expected_verdict"
    break
  fi
done < "$PROFILES"

if [[ -z "$match" ]]; then
  echo "unknown profile: $PROFILE" >&2
  exit 1
fi

IFS='|' read -r profile_id risk_class crate_checks tests doc_tests gpu_required expected_verdict <<< "$match"

echo "DOCTRINE-EXEC-PLAN profile=$profile_id risk_class=$risk_class gpu_required=$gpu_required expected_verdict_if_gpu_missing=$expected_verdict"
echo "--- crate_checks ---"
echo "$crate_checks" | tr ',' '\n'
echo "--- tests ---"
echo "$tests" | tr ';' '\n'
echo "--- doc_tests ---"
echo "$doc_tests"
echo "--- workspace_check ---"
echo "cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-feeder -p simthing-sim -p simthing-driver -p simthing-spec -p simthing-workshop -p simthing-clausething -p simthing-mapgenerator -p simthing-tools"
echo "--- surface_truth ---"
echo "bash scripts/ci/doctrine_surface_truth.sh"