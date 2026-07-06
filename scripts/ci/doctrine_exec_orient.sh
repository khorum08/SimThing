#!/usr/bin/env bash
# Emit orientation digest + ORIENT-RECEIPT for GHA /orient command.
set -euo pipefail

ROLE="${1:-orchestrator}"
REPORT="${2:-orient-report.txt}"
HEAD_SHA="${3:-}"
BASE_SHA="${4:-}"
PR="${5:-}"

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[[ -n "$ROLE" ]] || { echo "missing role" >&2; exit 1; }

{
  echo "ORIENT-REPORT: OK"
  [[ -n "$PR" ]] && echo "pr: $PR"
  [[ -n "$HEAD_SHA" ]] && echo "head_sha: $HEAD_SHA"
  [[ -n "$BASE_SHA" ]] && echo "base_sha: $BASE_SHA"
  bash "${SCRIPT_DIR}/orient.sh" "--role=${ROLE}"
} >"$REPORT"

echo "ORIENT-REPORT: written to ${REPORT}"