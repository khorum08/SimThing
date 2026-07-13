#!/usr/bin/env bash
# OC-QUERY-0 — queryable anchor library + reach-log observability.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly ANCHORS_TSV="${SCRIPT_DIR}/doctrine_anchors.tsv"
readonly TRIGGERS_TSV="${SCRIPT_DIR}/anchor_triggers.tsv"
readonly REACH_LOG_TSV="${SCRIPT_DIR}/anchor_reach_log.tsv"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/anchor_query"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

MODE=""
DOMAIN_ARG=""
GREP_ARG=""
PRUNE_DAYS=""
PATH_ARGS=()
ROLE_ARG="${ANCHOR_QUERY_ROLE:-coding}"
SELFTEST=0
FIXTURE_DIR=""

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/anchor_query.sh --domain <domain>
  bash scripts/ci/anchor_query.sh --paths <files...>
  bash scripts/ci/anchor_query.sh --grep <term>
  bash scripts/ci/anchor_query.sh --prune <days>
  bash scripts/ci/anchor_query.sh --selftest
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --domain)
        MODE="domain"
        DOMAIN_ARG="${2:-}"
        [[ -n "$DOMAIN_ARG" ]] || usage
        shift 2
        ;;
      --paths)
        MODE="paths"
        shift
        while [[ $# -gt 0 && "$1" != --* ]]; do
          PATH_ARGS+=("$1")
          shift
        done
        [[ "${#PATH_ARGS[@]}" -gt 0 ]] || usage
        ;;
      --grep)
        MODE="grep"
        GREP_ARG="${2:-}"
        [[ -n "$GREP_ARG" ]] || usage
        shift 2
        ;;
      --prune)
        MODE="prune"
        PRUNE_DAYS="${2:-}"
        [[ -n "$PRUNE_DAYS" ]] || usage
        shift 2
        ;;
      --role)
        ROLE_ARG="${2:-}"
        [[ -n "$ROLE_ARG" ]] || usage
        shift 2
        ;;
      --selftest) SELFTEST=1; shift ;;
      --fixture)
        FIXTURE_DIR="${FIXTURES_ROOT}/${2:-}"
        shift 2
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
  if [[ "$SELFTEST" -eq 0 && -z "$MODE" ]]; then
    usage
  fi
}

run_query_python() {
  ANCHOR_REPO_ROOT="$REPO_ROOT" \
  ANCHOR_TSV_PATH="${FIXTURE_DIR:+$FIXTURE_DIR/doctrine_anchors.tsv}" \
  ANCHOR_TRIGGERS_PATH="${FIXTURE_DIR:+$FIXTURE_DIR/anchor_triggers.tsv}" \
  ANCHOR_REACH_LOG_PATH="${ANCHOR_REACH_LOG_PATH:-${FIXTURE_DIR:+$FIXTURE_DIR/anchor_reach_log.tsv}}" \
  ANCHOR_QUERY_MODE="$1" \
  ANCHOR_QUERY_DOMAIN="${DOMAIN_ARG:-}" \
  ANCHOR_QUERY_GREP="${GREP_ARG:-}" \
  ANCHOR_QUERY_PRUNE="${PRUNE_DAYS:-}" \
  ANCHOR_QUERY_ROLE="$ROLE_ARG" \
  ANCHOR_QUERY_PATHS="$(printf '%s\n' "${PATH_ARGS[@]:-}")" \
    "$PYTHON_BIN" - <<'PY'
import csv
import datetime as dt
import fnmatch
import hashlib
import os
import pathlib
import re
import sys
from pathlib import PurePosixPath

repo = pathlib.Path(os.environ["ANCHOR_REPO_ROOT"])
mode = os.environ["ANCHOR_QUERY_MODE"]
role = os.environ.get("ANCHOR_QUERY_ROLE", "coding")
domain_arg = os.environ.get("ANCHOR_QUERY_DOMAIN", "").strip()
grep_arg = os.environ.get("ANCHOR_QUERY_GREP", "").strip()
prune_days = os.environ.get("ANCHOR_QUERY_PRUNE", "").strip()
paths_blob = os.environ.get("ANCHOR_QUERY_PATHS", "")

def pick(env_key, default_rel):
    override = os.environ.get(env_key, "").strip()
    if override:
        p = pathlib.Path(override)
        if p.is_file() or env_key.endswith("REACH_LOG_PATH"):
            return p
    return repo / "scripts" / "ci" / default_rel

anchors_tsv = pick("ANCHOR_TSV_PATH", "doctrine_anchors.tsv")
if not anchors_tsv.is_file():
    anchors_tsv = repo / "scripts" / "ci" / "doctrine_anchors.tsv"
triggers_tsv = pick("ANCHOR_TRIGGERS_PATH", "anchor_triggers.tsv")
if not triggers_tsv.is_file():
    triggers_tsv = repo / "scripts" / "ci" / "anchor_triggers.tsv"
reach_log = pick("ANCHOR_REACH_LOG_PATH", "anchor_reach_log.tsv")
if str(reach_log).endswith("anchor_reach_log.tsv") and not reach_log.parent.exists():
    reach_log = repo / "scripts" / "ci" / "anchor_reach_log.tsv"


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")


def read_normalized(path: pathlib.Path) -> str:
    return normalize_text(path.read_bytes())


def lines_slice(path: pathlib.Path, spec: str) -> str:
    m = re.match(r"lines:(\d+)-(\d+)$", spec)
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
        raise KeyError(h)
    out = [lines[start]]
    for line in lines[start + 1 :]:
        if line.startswith("## ") and not line.startswith("###"):
            break
        out.append(line)
    return "\n".join(out).rstrip() + "\n"


def extract_text(doc_rel: str, section: str) -> str:
    path = repo / doc_rel
    if section.startswith("heading:"):
        return heading_section(path, section)
    if section.startswith("lines:"):
        return lines_slice(path, section)
    raise ValueError(section)


def glob_match(path: str, pattern: str) -> bool:
    g = pattern.strip().replace("\\", "/")
    if not g:
        return False
    p = PurePosixPath(path)
    if p.match(g):
        return True
    if "**" in g:
        prefix = g.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, g.replace("**", "*"))


def load_anchors():
    rows = []
    with anchors_tsv.open(encoding="utf-8", newline="") as fh:
        for row in csv.DictReader(fh, delimiter="\t"):
            if not row.get("anchor_id"):
                continue
            domains = [d.strip() for d in (row.get("trigger_domains") or "").split(",") if d.strip()]
            text = extract_text(row["doc"], row["section"])
            rows.append({
                "anchor_id": row["anchor_id"],
                "doc": row["doc"],
                "section": row["section"],
                "domains": domains,
                "text": text,
                "hash": hashlib.sha256(text.encode("utf-8")).hexdigest(),
            })
    return rows


def domains_from_paths(files):
    domains = set()
    if not triggers_tsv.is_file():
        return domains
    with triggers_tsv.open(encoding="utf-8", newline="") as fh:
        for row in csv.DictReader(fh, delimiter="\t"):
            glob_pat = (row.get("glob") or "").strip()
            if not glob_pat:
                continue
            if any(glob_match(path, glob_pat) for path in files):
                for d in (row.get("trigger_domains") or "").split(","):
                    d = d.strip()
                    if d:
                        domains.add(d)
    return domains


def ensure_reach_log():
    if not reach_log.is_file():
        reach_log.parent.mkdir(parents=True, exist_ok=True)
        with reach_log.open("w", encoding="utf-8", newline="\n") as fh:
            fh.write("date\trole\tquery\tanchors_served\thit\n")


def append_reach(query: str, ids, hit: str):
    ensure_reach_log()
    served = ",".join(ids) if ids else "none"
    stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    with reach_log.open("a", encoding="utf-8", newline="\n") as fh:
        fh.write(f"{stamp}\t{role}\t{query}\t{served}\t{hit}\n")


def emit_hits(ids, rows_by_id):
    print(f"ANCHOR-QUERY-VERDICT: PASS ids={len(ids)}")
    print(f"anchors: {','.join(ids) if ids else 'none'}")
    for aid in ids:
        meta = rows_by_id[aid]
        print(f"--- {aid} ---")
        print(f"doc: {meta['doc']}")
        print(f"section: {meta['section']}")
        print(f"content_hash: {meta['hash']}")
        print(meta["text"].rstrip())
        print("")


anchors = load_anchors()
by_id = {r["anchor_id"]: r for r in anchors}

if mode == "prune":
    days = int(prune_days)
    ensure_reach_log()
    cutoff = dt.datetime.now(dt.timezone.utc).replace(tzinfo=None) - dt.timedelta(days=days)
    kept = ["date\trole\tquery\tanchors_served\thit"]
    removed = 0
    with reach_log.open(encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh, delimiter="\t")
        for row in reader:
            raw = (row.get("date") or "").strip()
            try:
                when = dt.datetime.strptime(raw, "%Y-%m-%dT%H:%M:%SZ")
            except ValueError:
                kept.append("\t".join([
                    row.get("date", ""),
                    row.get("role", ""),
                    row.get("query", ""),
                    row.get("anchors_served", ""),
                    row.get("hit", ""),
                ]))
                continue
            if when >= cutoff:
                kept.append("\t".join([
                    row.get("date", ""),
                    row.get("role", ""),
                    row.get("query", ""),
                    row.get("anchors_served", ""),
                    row.get("hit", ""),
                ]))
            else:
                removed += 1
    reach_log.write_bytes(("\n".join(kept) + "\n").encode("utf-8"))
    print(f"ANCHOR-QUERY-PRUNE: PASS removed={removed} kept={len(kept)-1}")
    sys.exit(0)

if mode == "domain":
    ids = sorted(r["anchor_id"] for r in anchors if domain_arg in r["domains"])
    append_reach(f"--domain {domain_arg}", ids, "hit" if ids else "none")
    emit_hits(ids, by_id)
    sys.exit(0)

if mode == "paths":
    files = [ln.strip().replace("\\", "/") for ln in paths_blob.splitlines() if ln.strip()]
    domains = domains_from_paths(files)
    ids = sorted(r["anchor_id"] for r in anchors if domains.intersection(r["domains"]))
    q = "--paths " + " ".join(files)
    append_reach(q, ids, "hit" if ids else "none")
    print(f"domains: {','.join(sorted(domains)) if domains else 'none'}")
    emit_hits(ids, by_id)
    sys.exit(0)

if mode == "grep":
    term = grep_arg.lower()
    ids = []
    for r in anchors:
        blob = f"{r['anchor_id']}\n{r['doc']}\n{r['section']}\n{','.join(r['domains'])}\n{r['text']}".lower()
        if term in blob:
            ids.append(r["anchor_id"])
    ids = sorted(ids)
    append_reach(f"--grep {grep_arg}", ids, "hit" if ids else "none")
    emit_hits(ids, by_id)
    sys.exit(0)

print("ANCHOR-QUERY-VERDICT: FAIL(harness-error)")
sys.exit(1)
PY
}

run_selftest() {
  local failures=0
  local tmp out
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/anchor-query-XXXXXX")"
  cp "$ANCHORS_TSV" "$tmp/doctrine_anchors.tsv"
  cp "$TRIGGERS_TSV" "$tmp/anchor_triggers.tsv"
  printf 'date\trole\tquery\tanchors_served\thit\n' >"$tmp/anchor_reach_log.tsv"
  printf '2020-01-01T00:00:00Z\tcoding\t--grep old\tnone\tnone\n' >>"$tmp/anchor_reach_log.tsv"
  local now
  now="$("$PYTHON_BIN" -c 'import datetime as dt;print(dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))')"
  printf '%s\tcoding\t--grep fresh\tnone\tnone\n' "$now" >>"$tmp/anchor_reach_log.tsv"

  FIXTURE_DIR="$tmp"
  out="$(DOMAIN_ARG=gate-wiring run_query_python domain || true)"
  if ! printf '%s\n' "$out" | grep -q "orientation-harness-core"; then
    echo "FAIL query_domain_gate_wiring"; failures=$((failures+1))
  else
    echo "PASS query_domain_gate_wiring"
  fi
  PATH_ARGS=("crates/simthing-kernel/src/lib.rs")
  out="$(run_query_python paths || true)"
  if ! printf '%s\n' "$out" | grep -q "seal-residue-cross-crate"; then
    echo "FAIL query_paths_kernel"; failures=$((failures+1))
  else
    echo "PASS query_paths_kernel"
  fi
  PATH_ARGS=("crates/simthing-gpu/src/shaders/foo.wgsl")
  out="$(run_query_python paths || true)"
  if ! printf '%s\n' "$out" | grep -qE "field-policy-time-decisions|eml-extension-ladder"; then
    echo "FAIL query_paths_wgsl"; failures=$((failures+1))
  else
    echo "PASS query_paths_wgsl"
  fi
  GREP_ARG="Candidate F"
  out="$(run_query_python grep || true)"
  if ! printf '%s\n' "$out" | grep -q "exact-numeric-candidate-f"; then
    echo "FAIL query_grep_hit"; failures=$((failures+1))
  else
    echo "PASS query_grep_hit"
  fi
  GREP_ARG="zzznomatchterm999"
  out="$(run_query_python grep || true)"
  if ! printf '%s\n' "$out" | grep -q "anchors: none"; then
    echo "FAIL query_grep_miss"; failures=$((failures+1))
  else
    echo "PASS query_grep_miss"
  fi
  local rows_before rows_after
  rows_before="$(wc -l <"$tmp/anchor_reach_log.tsv" | tr -d ' ')"
  GREP_ARG="Movement-Front"
  run_query_python grep >/dev/null || true
  rows_after="$(wc -l <"$tmp/anchor_reach_log.tsv" | tr -d ' ')"
  if [[ "$rows_after" -le "$rows_before" ]]; then
    echo "FAIL reach_log_append"; failures=$((failures+1))
  else
    echo "PASS reach_log_append"
  fi
  PRUNE_DAYS="30"
  run_query_python prune >/dev/null || true
  if grep -q "2020-01-01T00:00:00Z" "$tmp/anchor_reach_log.tsv"; then
    echo "FAIL reach_log_prune"; failures=$((failures+1))
  else
    echo "PASS reach_log_prune"
  fi
  if ! head -n 1 "$tmp/anchor_reach_log.tsv" | grep -q $'date\trole\tquery\tanchors_served\thit'; then
    echo "FAIL reach_log_header"; failures=$((failures+1))
  else
    echo "PASS reach_log_header"
  fi

  # DA rider r1: LF-clean reach-log writes (no CR bytes)
  local lf_tmp
  lf_tmp="$(mktemp -d "${TMPDIR:-/tmp}/anchor-query-lf-XXXXXX")"
  cp "$ANCHORS_TSV" "$lf_tmp/doctrine_anchors.tsv"
  cp "$TRIGGERS_TSV" "$lf_tmp/anchor_triggers.tsv"
  FIXTURE_DIR="$lf_tmp"
  DOMAIN_ARG=gate-wiring run_query_python domain >/dev/null || true
  GREP_ARG="zzznomatchterm999"
  run_query_python grep >/dev/null || true
  printf '2020-01-01T00:00:00Z\tcoding\t--grep old\tnone\tnone\n' >>"$lf_tmp/anchor_reach_log.tsv"
  PRUNE_DAYS="30"
  run_query_python prune >/dev/null || true
  if "$PYTHON_BIN" -c '
import pathlib, sys
p = pathlib.Path(sys.argv[1])
raw = p.read_bytes()
sys.exit(0 if (b"\r" not in raw and raw.startswith(b"date\trole\tquery\tanchors_served\thit\n")) else 1)
' "$lf_tmp/anchor_reach_log.tsv"; then
    echo "PASS reach_log_lf_clean"
  else
    echo "FAIL reach_log_lf_clean"; failures=$((failures+1))
  fi
  rm -rf "$lf_tmp"

  rm -rf "$tmp"
  FIXTURE_DIR=""
  if [[ "$failures" -eq 0 ]]; then
    echo "ANCHOR-QUERY-SELFTEST: PASS (9 fixtures)"
    return 0
  fi
  echo "ANCHOR-QUERY-SELFTEST: FAIL (${failures} fixtures)"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$SELFTEST" -eq 1 ]]; then
    run_selftest
    exit $?
  fi
  case "$MODE" in
    domain) run_query_python domain ;;
    paths) run_query_python paths ;;
    grep) run_query_python grep ;;
    prune) run_query_python prune ;;
    *) usage ;;
  esac
}

main "$@"
