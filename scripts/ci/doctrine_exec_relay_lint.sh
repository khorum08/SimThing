#!/usr/bin/env bash
# Run relay lint for GHA /relay-lint command; write report file.
set -euo pipefail

PR="${1:-}"
REPORT="${2:-relay-lint-report.txt}"

[[ -n "$PR" ]] || { echo "missing PR number" >&2; exit 1; }

export GITHUB_REPOSITORY="${GITHUB_REPOSITORY:?}"
export GH_TOKEN="${GH_TOKEN:-}"

bash "$(dirname "$0")/relay_lint.sh" --pr "$PR" >"$REPORT"
echo "RELAY-LINT-REPORT: OK"