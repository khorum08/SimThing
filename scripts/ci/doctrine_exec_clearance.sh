#!/usr/bin/env bash
# Append a clearance ledger row after a GHA-side /clearance verdict.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LEDGER="${ROOT}/scripts/ci/clearance_ledger.tsv"

VERDICT="${1:-}"
CLASS="${2:-unknown}"
PR="${3:-}"
SHA="${4:-${GITHUB_SHA:-unknown}}"
SKETCH="${5:-}"

[[ -n "$VERDICT" ]] || { echo "missing verdict" >&2; exit 1; }

if [[ ! -f "$LEDGER" ]]; then
  printf 'verdict\tclass\tpr\tsha\tdate\tsketch\n' >"$LEDGER"
fi

date="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u)"
row="${VERDICT}	${CLASS}	${PR}	${SHA}	${date}	${SKETCH}"
echo "$row" >>"$LEDGER"
echo "CLEARANCE-LEDGER-APPEND: OK"
echo "$row"