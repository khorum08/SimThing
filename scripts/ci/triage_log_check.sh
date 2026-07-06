#!/usr/bin/env bash
# OH-TRIAGE-INDUCTION-0 — triage_log.tsv schema + reason validation.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly DEFAULT_TRIAGE_LOG="${SCRIPT_DIR}/triage_log.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

TRIAGE_LOG="${DEFAULT_TRIAGE_LOG}"
MODE="check"
REASON_ARG=""
SCAN_ARG=""
OUTCOME_ARG=""

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/triage_log_check.sh --check [path]
  bash scripts/ci/triage_log_check.sh --validate-reason <reason>
  bash scripts/ci/triage_log_check.sh --validate-append <scan-id> <outcome> <reason>
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check)
        MODE="check"
        if [[ "${2:-}" != "" && "${2:-}" != --* ]]; then
          TRIAGE_LOG="$2"
          shift
        fi
        shift
        ;;
      --validate-reason)
        MODE="validate-reason"
        REASON_ARG="${2:-}"
        [[ -n "$REASON_ARG" ]] || usage
        shift 2
        ;;
      --validate-append)
        MODE="validate-append"
        SCAN_ARG="${2:-}"
        OUTCOME_ARG="${3:-}"
        REASON_ARG="${4:-}"
        [[ -n "$SCAN_ARG" && -n "$OUTCOME_ARG" && -n "$REASON_ARG" ]] || usage
        shift 4
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
}

run_python() {
  TRIAGE_LOG_PATH="$TRIAGE_LOG" \
  TRIAGE_MODE="$MODE" \
  TRIAGE_REASON="${REASON_ARG:-}" \
  TRIAGE_SCAN="${SCAN_ARG:-}" \
  TRIAGE_OUTCOME="${OUTCOME_ARG:-}" \
    "$PYTHON_BIN" - <<'PY'
import os
import re
import sys

path = os.environ.get("TRIAGE_LOG_PATH", "")
mode = os.environ["TRIAGE_MODE"]
reason = os.environ.get("TRIAGE_REASON", "")
scan = os.environ.get("TRIAGE_SCAN", "")
outcome = os.environ.get("TRIAGE_OUTCOME", "")

PLACEHOLDER_RE = re.compile(
    r"^(?:tbd|todo|n/?a|none|pending|fixme|wip|placeholder|\.{1,3}|-+)$",
    re.IGNORECASE,
)
VALID_OUTCOMES = {"delete", "green", "escalate"}


def reason_valid(text: str) -> tuple[bool, str]:
    text = (text or "").strip()
    if not text:
        return False, "missing-reason"
    if PLACEHOLDER_RE.match(text):
        return False, "placeholder-reason"
    return True, ""


def parse_row(line: str, lineno: int) -> tuple[str, str, str, str, str]:
    line = line.rstrip("\n\r")
    if not line.strip():
        raise ValueError(f"empty row at line {lineno}")
    if "|" in line:
        parts = [p.strip() for p in line.split("|")]
    else:
        parts = [p.strip() for p in line.split("\t")]
    if len(parts) < 5:
        raise ValueError(f"malformed row at line {lineno}: need 5 fields, got {len(parts)}")
    return parts[0], parts[1], parts[2], parts[3], parts[4]


def load_rows(triage_path: str) -> list[tuple[str, str, str, str, str]]:
    rows = []
    with open(triage_path, encoding="utf-8-sig", newline="") as fh:
        for i, line in enumerate(fh, 1):
            line = line.rstrip("\n\r")
            if not line.strip():
                continue
            if i == 1 and ("scan-id" in line.lower() or "scan_id" in line.lower()):
                continue
            rows.append(parse_row(line, i))
    return rows


def check_file(triage_path: str) -> int:
    if not os.path.isfile(triage_path):
        print(f"TRIAGE-LOG-CHECK: FAIL missing file {triage_path}", file=sys.stderr)
        return 1
    try:
        rows = load_rows(triage_path)
    except ValueError as exc:
        print(f"TRIAGE-LOG-CHECK: FAIL({exc})", file=sys.stderr)
        return 1
    errors = []
    covered = set()
    for scan_id, branch, out, why, commit in rows:
        if not scan_id:
            errors.append("empty scan-id")
        if not branch:
            errors.append(f"{scan_id or '?'}: empty branch")
        if out not in VALID_OUTCOMES:
            errors.append(f"{scan_id}: invalid outcome {out!r}")
        ok, detail = reason_valid(why)
        if not ok:
            errors.append(f"{scan_id}: {detail}")
        if not commit:
            errors.append(f"{scan_id}: empty commit field")
        covered.add(scan_id)
    if errors:
        for err in errors:
            print(f"TRIAGE-LOG-CHECK: FAIL {err}", file=sys.stderr)
        return 1
    print(f"TRIAGE-LOG-CHECK: PASS rows={len(rows)}")
    return 0


def covered_scan_ids(triage_path: str) -> set[str]:
    if not os.path.isfile(triage_path):
        return set()
    try:
        rows = load_rows(triage_path)
    except ValueError:
        return set()
    covered = set()
    for scan_id, _branch, outcome, reason, _commit in rows:
        if outcome not in VALID_OUTCOMES:
            continue
        ok, _ = reason_valid(reason)
        if not ok:
            continue
        if scan_id:
            covered.add(scan_id)
    return covered


if mode == "validate-reason":
    ok, detail = reason_valid(reason)
    if ok:
        sys.exit(0)
    print(detail, file=sys.stderr)
    sys.exit(1)

if mode == "validate-append":
    if not scan.strip():
        print("missing-scan-id", file=sys.stderr)
        sys.exit(1)
    if outcome not in VALID_OUTCOMES:
        print("invalid-outcome", file=sys.stderr)
        sys.exit(1)
    ok, detail = reason_valid(reason)
    if not ok:
        print(detail, file=sys.stderr)
        sys.exit(1)
    sys.exit(0)

if mode == "check":
    sys.exit(check_file(path))

sys.exit(2)
PY
}

main() {
  parse_args "$@"
  run_python
}

main "$@"