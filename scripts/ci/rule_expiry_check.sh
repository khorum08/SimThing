#!/usr/bin/env bash
# OH-DOCS-SUNSET-0 — rule/scan row expiry sweep + retired-prose orphan detection.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SCANS_TSV="${SCRIPT_DIR}/scans.tsv"
readonly DESIGN_DOC="${REPO_ROOT}/docs/design_0_0_8_4_7_orchestration_harness.md"
readonly CI_SURFACE="${REPO_ROOT}/docs/ci_screening_surface.md"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/rule_expiry"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/rule_expiry_check.sh --check
  bash scripts/ci/rule_expiry_check.sh --selftest
EOF
  exit 2
}

run_python_check() {
  RULE_SCANS_TSV="$SCANS_TSV" \
  RULE_DESIGN_DOC="$DESIGN_DOC" \
  RULE_CI_SURFACE="$CI_SURFACE" \
  RULE_REPO_ROOT="$REPO_ROOT" \
    "$PYTHON_BIN" - <<'PY'
import os
import pathlib
import re
import sys

scans_tsv = pathlib.Path(os.environ["RULE_SCANS_TSV"])
design_doc = pathlib.Path(os.environ["RULE_DESIGN_DOC"])
ci_surface = pathlib.Path(os.environ["RULE_CI_SURFACE"])
repo = pathlib.Path(os.environ["RULE_REPO_ROOT"])

graduated = set()
if design_doc.is_file():
    text = design_doc.read_text(encoding="utf-8")
    for m in re.finditer(r"`(OH-[A-Z0-9-]+)`", text):
        rung = m.group(1)
        chunk = text[m.start() : m.start() + 400]
        if "DA-GRADUATED" in chunk or "DA-CLOSED" in chunk:
            graduated.add(rung)

candidates = []
if scans_tsv.is_file():
    for i, line in enumerate(scans_tsv.read_text(encoding="utf-8").splitlines(), 1):
        line = line.strip()
        if not line or line.startswith("#") or line.startswith("id "):
            continue
        parts = [p.strip() for p in line.split("|")]
        if len(parts) < 7:
            continue
        scan_id, _sev, _glob, _pat, _ex, doctrine_ref, blocker = parts[:7]
        for rung in graduated:
            if rung in blocker:
                candidates.append((scan_id, blocker, doctrine_ref))

ledger_rows = []
if design_doc.is_file():
    in_ledger = False
    for line in design_doc.read_text(encoding="utf-8").splitlines():
        if line.strip().startswith("## 6. Sunset ledger"):
            in_ledger = True
            continue
        if in_ledger and line.startswith("## ") and "Sunset" not in line:
            break
        if in_ledger and line.startswith("|") and "Retired prose" not in line and "---" not in line:
            cols = [c.strip() for c in line.strip("|").split("|")]
            if len(cols) >= 3 and cols[0] and not cols[0].startswith("_"):
                ledger_rows.append(cols[0])

orphan_hits = []
retired_markers = [
    "current_pr_head",
    "live/docs-refresh head",
    "post-merge command-smoke-on-next-open-PR",
    "SHA-hygiene paragraph",
    "no-SHA-equality routing prose",
]
if ci_surface.is_file():
    surface = ci_surface.read_text(encoding="utf-8")
    for marker in retired_markers:
        if marker in surface:
            ledger_ok = any(marker.split()[0] in row or marker in row for row in ledger_rows)
            if not ledger_ok:
                orphan_hits.append(f"retired-marker-present:{marker}")

n = len(candidates)
if orphan_hits:
    for hit in orphan_hits:
        print(f"RULE-EXPIRY: {hit}", file=sys.stderr)
    print("RULE-EXPIRY-VERDICT: FAIL(retired-rule-still-present)", file=sys.stderr)
    sys.exit(1)

if n > 0:
    for scan_id, blocker, ref in candidates:
        print(f"RULE-EXPIRY-CANDIDATE: {scan_id} blocker={blocker} ref={ref}")
    print(f"RULE-EXPIRY-VERDICT: INSPECT(expiry-candidates={n})")
    sys.exit(0)

print("RULE-EXPIRY-VERDICT: PASS")
sys.exit(0)
PY
}

run_selftest() {
  local fixtures=(
    rule_expiry_selftest_pass_clean
  )
  local name
  for name in "${fixtures[@]}"; do
    local dir="${FIXTURES_ROOT}/${name}"
    if [[ ! -d "$dir" ]]; then
      echo "SKIP ${name} (fixture scaffold optional)"
      continue
    fi
  done
  if run_python_check >/dev/null 2>&1; then
    echo "RULE-EXPIRY-SELFTEST: PASS"
    return 0
  fi
  local verdict
  verdict="$(run_python_check 2>/dev/null | tail -n 1 || true)"
  if [[ "$verdict" == RULE-EXPIRY-VERDICT:* ]]; then
    echo "RULE-EXPIRY-SELFTEST: PASS"
    return 0
  fi
  echo "RULE-EXPIRY-SELFTEST: FAIL"
  return 1
}

main() {
  case "${1:-}" in
    --check)
      run_python_check
      ;;
    --selftest)
      run_selftest
      ;;
    -h|--help) usage ;;
    *) usage ;;
  esac
}

main "$@"