#!/usr/bin/env bash
# OH-ANCHOR-INTEGRITY-0 — doctrine anchor table verification + anchor stamp emission.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly ANCHORS_TSV="${SCRIPT_DIR}/doctrine_anchors.tsv"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/anchor_integrity"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

MODE="check"
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/anchor_check.sh --check
  bash scripts/ci/anchor_check.sh --anchor-stamp
  bash scripts/ci/anchor_check.sh --resolve <anchor_id|trigger_domain>
  bash scripts/ci/anchor_check.sh --selftest
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check) MODE="check"; shift ;;
      --anchor-stamp) MODE="anchor-stamp"; shift ;;
      --resolve)
        MODE="resolve"
        RESOLVE_ARG="${2:-}"
        [[ -n "$RESOLVE_ARG" ]] || usage
        shift 2
        ;;
      --selftest) FIXTURE_MODE="selftest"; shift ;;
      --fixture)
        [[ $# -ge 2 ]] || usage
        FIXTURE_MODE="fixture"
        FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
        shift 2
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
}

run_python() {
  ANCHOR_REPO_ROOT="$REPO_ROOT" \
  ANCHOR_TSV_PATH="$ANCHORS_TSV" \
  ANCHOR_FIXTURE_DIR="${FIXTURE_DIR:-}" \
  ANCHOR_MODE="$1" \
  ANCHOR_RESOLVE_ARG="${RESOLVE_ARG:-}" \
    "$PYTHON_BIN" - <<'PY'
import csv
import hashlib
import os
import pathlib
import re
import sys

repo = pathlib.Path(os.environ["ANCHOR_REPO_ROOT"])
tsv_path = pathlib.Path(os.environ["ANCHOR_TSV_PATH"])
mode = os.environ["ANCHOR_MODE"]
fixture_dir = os.environ.get("ANCHOR_FIXTURE_DIR", "")
resolve_arg = os.environ.get("ANCHOR_RESOLVE_ARG", "")


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    text = raw.decode("utf-8")
    return text.replace("\r\n", "\n").replace("\r", "\n")


def read_normalized(path: pathlib.Path) -> str:
    return normalize_text(path.read_bytes())


def fail(msg):
    print(f"ANCHOR-CHECK-VERDICT: FAIL({msg})")
    sys.exit(1 if mode == "check" else 0)


def pass_ok(detail=""):
    if detail:
        print(f"ANCHOR-CHECK-VERDICT: PASS {detail}".rstrip())
    else:
        print("ANCHOR-CHECK-VERDICT: PASS")
    sys.exit(0)


def lines_slice(path: pathlib.Path, spec: str) -> str:
    m = re.match(r"lines:(\d+)-(\d+)$", spec)
    if not m:
        raise ValueError(f"bad lines spec: {spec}")
    start, end = int(m.group(1)), int(m.group(2))
    lines = read_normalized(path).splitlines()
    return "\n".join(lines[start - 1 : end]) + "\n"


def heading_section(path: pathlib.Path, heading: str) -> str:
    h = heading.removeprefix("heading:")
    lines = read_normalized(path).splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.strip() == h or line.strip().startswith(h):
            start = i
            break
    if start is None:
        raise KeyError(f"missing heading {h!r} in {path}")
    out = [lines[start]]
    for line in lines[start + 1 :]:
        if line.startswith("## ") and not line.startswith("###"):
            break
        out.append(line)
    return "\n".join(out).rstrip() + "\n"


def extract_text(doc_rel: str, section: str) -> str:
    path = repo / doc_rel
    if not path.is_file():
        raise FileNotFoundError(doc_rel)
    if section.startswith("heading:"):
        return heading_section(path, section)
    if section.startswith("lines:"):
        return lines_slice(path, section)
    raise ValueError(f"unsupported section spec: {section}")


def load_rows():
    if fixture_dir:
        alt = pathlib.Path(fixture_dir) / "doctrine_anchors.tsv"
        use = alt if alt.is_file() else tsv_path
    else:
        use = tsv_path
    if not use.is_file():
        fail("missing-anchor")
    rows = []
    with use.open(encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh, delimiter="\t")
        if not reader.fieldnames or "anchor_id" not in reader.fieldnames:
            fail("anchor-table")
        for row in reader:
            if not row.get("anchor_id"):
                continue
            for key in ("anchor_id", "doc", "section", "trigger_domains", "content_hash"):
                if not row.get(key):
                    fail("anchor-table")
            rows.append(row)
    if not rows:
        fail("missing-anchor")
    return rows


def live_hashes(rows):
    out = {}
    for row in rows:
        try:
            text = extract_text(row["doc"], row["section"])
        except (FileNotFoundError, KeyError, ValueError):
            fail("missing-anchor")
        live = hashlib.sha256(text.encode("utf-8")).hexdigest()
        out[row["anchor_id"]] = {
            "live_hash": live,
            "expected": row["content_hash"].lower(),
            "short": live[:12],
            "domains": [d.strip() for d in row["trigger_domains"].split(",") if d.strip()],
            "doc": row["doc"],
            "section": row["section"],
            "text": text,
        }
        if live != row["content_hash"].lower():
            fail("anchor-hash-drift")
    return out


def anchor_stamp(state):
    joined = "|".join(f"{k}:{state[k]['live_hash']}" for k in sorted(state))
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:16]


rows = load_rows()
state = live_hashes(rows)

if mode == "anchor-stamp":
    print(anchor_stamp(state))
    sys.exit(0)

if mode == "resolve":
    arg = resolve_arg.lower().strip()
    exact = [(aid, meta) for aid, meta in state.items() if aid.lower() == arg]
    if exact:
        aid, meta = exact[0]
    else:
        domain = [(aid, meta) for aid, meta in state.items() if arg in meta["domains"]]
        if not domain:
            print("ANCHOR-RESOLVE-VERDICT: FAIL(unknown-anchor)")
            sys.exit(1)
        aid, meta = domain[0]
    print("ANCHOR-REPORT: OK")
    print(f"anchor_id: {aid}")
    print(f"doc: {meta['doc']}")
    print(f"section: {meta['section']}")
    print(f"content_hash: {meta['live_hash']}")
    print("--- verbatim anchored text ---")
    print(meta["text"].rstrip())
    sys.exit(0)

if mode == "check":
    pass_ok()

print("ANCHOR-CHECK-VERDICT: FAIL(harness-error)")
sys.exit(1)
PY
}

run_selftest() {
  local fixtures=(
    anchor_integrity_selftest_pass_valid_table
    anchor_integrity_selftest_fail_hash_drift
    anchor_integrity_selftest_fail_missing_anchor
    anchor_integrity_selftest_fail_malformed_table
    anchor_integrity_selftest_receipt_stales_on_anchor_change
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_anchor_selftest_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ANCHOR-CHECK-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  echo "ANCHOR-CHECK-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

run_anchor_selftest_fixture() {
  local name="$1"
  local fix="${FIXTURES_ROOT}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  if [[ "$name" == "anchor_integrity_selftest_receipt_stales_on_anchor_change" ]]; then
    local live drift
    live="$(bash "${SCRIPT_DIR}/anchor_check.sh" --anchor-stamp)"
    FIXTURE_DIR="$fix"
    export FIXTURE_DIR
    drift="$(run_python anchor-stamp)"
    FIXTURE_DIR=""
    unset FIXTURE_DIR
    if [[ -n "$live" && -n "$drift" && "$live" != "$drift" ]]; then
      echo "PASS ${name}"
      return 0
    fi
    echo "FAIL ${name}"
    echo "  expected: anchor_stamp drift between live and fixture table"
    echo "  live:     ${live}"
    echo "  drift:    ${drift}"
    return 1
  fi
  local expected
  expected="$(tr -d '\r' <"${fix}/expected_verdict.txt" | head -n 1)"
  FIXTURE_DIR="$fix"
  export FIXTURE_DIR
  local got
  got="$(run_python check 2>&1 | head -n 1 || true)"
  FIXTURE_DIR=""
  unset FIXTURE_DIR
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    FIXTURE_DIR="${FIXTURE_DIR:-}"
    run_anchor_selftest_fixture "$(basename "$FIXTURE_DIR")"
    exit $?
  fi
  run_python "$MODE"
}

main "$@"