#!/usr/bin/env bash
# Reject stale doctrine-exec reports when head_sha != current PR head.
set -euo pipefail

REPORT="${1:-doctrine_exec_report.json}"
CURRENT_HEAD="${2:-}"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

[[ -f "$REPORT" ]] || { echo "STALE-CHECK: FAIL missing report"; exit 1; }
[[ -n "$CURRENT_HEAD" ]] || { echo "STALE-CHECK: FAIL missing current head"; exit 1; }

reported="$("$PYTHON_BIN" - <<'PY' "$REPORT"
import json, sys
print(json.load(open(sys.argv[1], encoding="utf-8")).get("head_sha") or "")
PY
)"

if [[ "$reported" != "$CURRENT_HEAD" ]]; then
  echo "STALE-CHECK: FAIL reported head_sha=$reported current=$CURRENT_HEAD"
  exit 1
fi

echo "STALE-CHECK: PASS head_sha matches current PR head"
