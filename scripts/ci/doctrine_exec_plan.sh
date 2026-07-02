#!/usr/bin/env bash
# Resolve doctrine-exec profile/probe commands without running them (plan mode).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${ROOT}/scripts/ci/doctrine_exec_profiles.tsv"
PROFILE="${DOCTRINE_EXEC_PROFILE:-ci-b-webchat-smoke}"
OWNER_DEEP_ALLOWED="${DOCTRINE_EXEC_OWNER_DEEP_ALLOWED:-false}"

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
while IFS=$'\t' read -r profile_id profile_class risk_class crate_checks tests doc_tests gpu_required expected_verdict; do
  [[ "$profile_id" == "profile_id" ]] && continue
  [[ -z "${profile_id// }" ]] && continue
  if [[ "$profile_id" == "$PROFILE" ]]; then
    match="$profile_id|$profile_class|$risk_class|$crate_checks|$tests|$doc_tests|$gpu_required|$expected_verdict"
    break
  fi
done < "$PROFILES"

if [[ -z "$match" ]]; then
  echo "unknown profile: $PROFILE" >&2
  exit 1
fi

IFS='|' read -r profile_id profile_class risk_class crate_checks tests doc_tests gpu_required expected_verdict <<< "$match"
[[ "$crate_checks" == "-" ]] && crate_checks=""
[[ "$tests" == "-" ]] && tests=""
[[ "$doc_tests" == "-" ]] && doc_tests=""

if [[ "$profile_class" == "owner-deep" && "$OWNER_DEEP_ALLOWED" != "true" ]]; then
  echo "DOCTRINE-EXEC-PLAN-REJECTED profile=$profile_id profile_class=$profile_class owner_deep=false"
  exit 1
fi

echo "DOCTRINE-EXEC-PLAN profile=$profile_id profile_class=$profile_class risk_class=$risk_class gpu_required=$gpu_required expected_verdict_if_gpu_missing=$expected_verdict"
echo "--- crate_checks ---"
if [[ -n "$crate_checks" ]]; then echo "$crate_checks" | tr ',' '\n'; else echo "(none)"; fi
echo "--- tests ---"
if [[ -n "$tests" ]]; then echo "$tests" | tr ';' '\n'; else echo "(none)"; fi
echo "--- doc_tests ---"
if [[ -n "$doc_tests" ]]; then echo "$doc_tests"; else echo "(none)"; fi
echo "--- workspace_check ---"
if [[ "$profile_class" == "owner-deep" ]]; then
  echo "cargo check -p simthing-core -p simthing-kernel -p simthing-gpu -p simthing-feeder -p simthing-sim -p simthing-driver -p simthing-spec -p simthing-workshop -p simthing-clausething -p simthing-mapgenerator -p simthing-tools"
else
  echo "(none)"
fi
echo "--- surface_truth ---"
if [[ "$profile_class" == "smoke" || "$risk_class" == "test-deletion-clausething" ]]; then
  echo "(none)"
else
  echo "bash scripts/ci/doctrine_surface_truth.sh"
fi
