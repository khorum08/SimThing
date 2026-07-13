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
  bash scripts/ci/anchor_check.sh --resync [--dry-run]
  bash scripts/ci/anchor_check.sh --selftest
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check) MODE="check"; shift ;;
      --anchor-stamp) MODE="anchor-stamp"; shift ;;
      --resync) MODE="resync"; shift ;;
      --dry-run) ANCHOR_RESYNC_DRY_RUN=1; shift ;;
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
  ANCHOR_RESYNC_DRY_RUN="${ANCHOR_RESYNC_DRY_RUN:-0}" \
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
resync_dry_run = os.environ.get("ANCHOR_RESYNC_DRY_RUN", "0") == "1"


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    text = raw.decode("utf-8")
    return text.replace("\r\n", "\n").replace("\r", "\n")


def read_normalized(path: pathlib.Path) -> str:
    return normalize_text(path.read_bytes())


def fail(msg):
    remedy = ""
    if msg == "anchor-hash-drift":
        remedy = " remedy=bash scripts/ci/anchor_check.sh --resync"
    elif msg in ("missing-anchor", "orphaned-anchor"):
        remedy = " remedy=repair doctrine_anchors.tsv section target or run bash scripts/ci/anchor_check.sh --resync"
    print(f"ANCHOR-CHECK-VERDICT: FAIL({msg}){remedy}")
    sys.exit(1 if mode in ("check", "resync") else 0)


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


def resolve_doc(doc_rel: str) -> pathlib.Path:
    if fixture_dir:
        alt = pathlib.Path(fixture_dir) / doc_rel
        if alt.is_file():
            return alt
    return repo / doc_rel


def extract_text(doc_rel: str, section: str) -> str:
    path = resolve_doc(doc_rel)
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


def list_headings(doc_rel: str):
    path = resolve_doc(doc_rel)
    if not path.is_file():
        return []
    out = []
    for line in read_normalized(path).splitlines():
        if line.startswith("#"):
            out.append(line.strip())
    return out


def nearest_headings(doc_rel: str, wanted: str, limit=5):
    wanted_l = wanted.lower()
    heads = list_headings(doc_rel)
    scored = []
    for h in heads:
        hl = h.lower()
        score = 0
        if wanted_l in hl or hl in wanted_l:
            score += 10
        score += sum(1 for tok in re.split(r"\W+", wanted_l) if tok and tok in hl)
        scored.append((score, h))
    scored.sort(key=lambda x: (-x[0], x[1]))
    return [h for s, h in scored if s > 0][:limit] or heads[:limit]


def cmd_resync(rows):
    # Rewrite table in place; never drop rows.
    use = tsv_path
    if fixture_dir:
        alt = pathlib.Path(fixture_dir) / "doctrine_anchors.tsv"
        if alt.is_file():
            use = alt
    orphans = 0
    resynced = 0
    out_rows = []
    for row in rows:
        aid = row["anchor_id"]
        try:
            text = extract_text(row["doc"], row["section"])
            live = hashlib.sha256(text.encode("utf-8")).hexdigest()
            if live != row["content_hash"].lower():
                print(f"RESYNCED {aid}")
                row = dict(row)
                row["content_hash"] = live
                resynced += 1
            else:
                print(f"UNCHANGED {aid}")
        except (FileNotFoundError, KeyError, ValueError) as exc:
            orphans += 1
            print(f"ORPHANED {aid}")
            wanted = row["section"]
            if wanted.startswith("heading:"):
                wanted = wanted[len("heading:"):]
            suggestions = nearest_headings(row["doc"], wanted)
            if suggestions:
                print(f"  suggestions: {' | '.join(suggestions)}")
            else:
                print(f"  suggestions: (none) reason={exc}")
        out_rows.append(row)

    if not resync_dry_run:
        with use.open("w", encoding="utf-8", newline="") as fh:
            writer = csv.DictWriter(
                fh,
                fieldnames=["anchor_id", "doc", "section", "trigger_domains", "content_hash"],
                delimiter="\t",
                lineterminator="\n",
            )
            writer.writeheader()
            writer.writerows(out_rows)

    if orphans:
        fail("orphaned-anchor")
    mode_name = "DRY" if resync_dry_run else "PASS"
    print(f"ANCHOR-RESYNC-VERDICT: {mode_name} resynced={resynced} orphans=0")
    sys.exit(0)


def anchor_stamp(state):
    joined = "|".join(f"{k}:{state[k]['live_hash']}" for k in sorted(state))
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:16]


rows = load_rows()

if mode == "resync":
    cmd_resync(rows)

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
            print("ANCHOR-RESOLVE-VERDICT: FAIL(unknown-anchor) remedy=bash scripts/ci/anchor_query.sh --domain <domain> or --grep <term>")
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

run_resync_selftests() {
  local tmp out
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/anchor-resync-XXXXXX")"
  mkdir -p "$tmp/docs"
  cat >"$tmp/docs/sample.md" <<'EOF'
# Architecture Decision Records

Body line one.

## Other heading

Other body.
EOF
  printf 'anchor_id\tdoc\tsection\ttrigger_domains\tcontent_hash\n' >"$tmp/doctrine_anchors.tsv"
  printf 'sample-anchor\tdocs/sample.md\theading:# Architecture Decision Records\ttest-domain\t0000000000000000000000000000000000000000000000000000000000000000\n' >>"$tmp/doctrine_anchors.tsv"
  FIXTURE_DIR="$tmp"
  export FIXTURE_DIR
  before="$(cat "$tmp/doctrine_anchors.tsv")"
  out="$(ANCHOR_RESYNC_DRY_RUN=1 run_python resync 2>&1 || true)"
  after="$(cat "$tmp/doctrine_anchors.tsv")"
  if [[ "$before" != "$after" ]] || ! printf '%s\n' "$out" | grep -q "ANCHOR-RESYNC-VERDICT: DRY"; then
    echo "FAIL resync_dry_run_no_write"
    echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  else
    echo "PASS resync_dry_run_no_write"
  fi
  out="$(run_python resync 2>&1 || true)"
  if ! printf '%s\n' "$out" | grep -q "RESYNCED sample-anchor"; then
    echo "FAIL resync_edited_section"
    echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  else
    echo "PASS resync_edited_section"
  fi

  printf 'anchor_id\tdoc\tsection\ttrigger_domains\tcontent_hash\n' >"$tmp/doctrine_anchors.tsv"
  printf 'sample-anchor\tdocs/sample.md\theading:# Missing Title That Moved\ttest-domain\t0000000000000000000000000000000000000000000000000000000000000000\n' >>"$tmp/doctrine_anchors.tsv"
  out="$(run_python resync 2>&1 || true)"
  if ! printf '%s\n' "$out" | grep -q "ORPHANED sample-anchor"; then
    echo "FAIL resync_orphaned_heading"
    echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  elif ! printf '%s\n' "$out" | grep -qi "Architecture Decision Records"; then
    echo "FAIL resync_orphan_suggestion"
    echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  else
    echo "PASS resync_orphaned_heading"
  fi
  FIXTURE_DIR=""
  unset FIXTURE_DIR
  rm -rf "$tmp"
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
  run_resync_selftests
  local total=$((${#fixtures[@]} + 2))
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ANCHOR-CHECK-SELFTEST: PASS (${total} fixtures)"
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
