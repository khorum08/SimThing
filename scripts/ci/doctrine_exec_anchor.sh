#!/usr/bin/env bash
# Emit anchor report for GHA /anchor command.
set -euo pipefail

TARGET="${1:-}"
REPORT="${2:-anchor-report.txt}"
PR="${3:-}"

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[[ -n "$TARGET" ]] || { echo "missing anchor target" >&2; exit 1; }

{
  [[ -n "$PR" ]] && echo "pr: $PR"
  bash "${SCRIPT_DIR}/anchor_check.sh" --resolve "$TARGET"
} >"$REPORT"

echo "ANCHOR-REPORT: written to ${REPORT}"