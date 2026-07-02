#!/usr/bin/env bash
# Append a §1A triage row to scripts/ci/triage_log.tsv on the PR branch.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TRIAGE_LOG="${ROOT}/scripts/ci/triage_log.tsv"

SCAN_ID="${1:-}"
OUTCOME="${2:-}"
REASON="${3:-}"
BRANCH="${4:-${GITHUB_HEAD_REF:-}}"
COMMIT="${5:-${GITHUB_SHA:-}}"

usage() {
  echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
  echo "usage: $0 <scan-id> <delete|green|escalate> <reason> [branch] [commit]"
  exit 1
}

[[ -n "$SCAN_ID" && -n "$OUTCOME" && -n "$REASON" ]] || usage

case "$OUTCOME" in
  delete|green|escalate) ;;
  *)
    echo "FORMAT: /triage <scan-id> <delete|green|escalate> <reason>"
    exit 1
    ;;
esac

if [[ ! -f "$TRIAGE_LOG" ]]; then
  echo "scan-id | branch | outcome | reason | commit" > "$TRIAGE_LOG"
fi

row="${SCAN_ID} | ${BRANCH:-unknown} | ${OUTCOME} | ${REASON} | ${COMMIT:-unknown}"
echo "$row" >> "$TRIAGE_LOG"
echo "TRIAGE-APPEND: OK"
echo "$row"