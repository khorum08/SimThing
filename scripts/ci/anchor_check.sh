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
if command -v cygpath >/dev/null 2>&1; then
  ANCHOR_BASH="$(cygpath -w "$(command -v bash)" 2>/dev/null || command -v bash)"
else
  ANCHOR_BASH="$(command -v bash)"
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
  bash scripts/ci/anchor_check.sh --pending
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
      --pending) MODE="pending"; shift ;;
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
  ANCHOR_GEN_ORIENTATION="${ANCHOR_GEN_ORIENTATION:-${SCRIPT_DIR}/gen_orientation.sh}" \
  ANCHOR_BASH="$ANCHOR_BASH" \
    "$PYTHON_BIN" - <<'PY'
import csv
import hashlib
import os
import pathlib
import re
import subprocess
import sys

repo = pathlib.Path(os.environ["ANCHOR_REPO_ROOT"])
tsv_path = pathlib.Path(os.environ["ANCHOR_TSV_PATH"])
mode = os.environ["ANCHOR_MODE"]
fixture_dir = os.environ.get("ANCHOR_FIXTURE_DIR", "")
resolve_arg = os.environ.get("ANCHOR_RESOLVE_ARG", "")
resync_dry_run = os.environ.get("ANCHOR_RESYNC_DRY_RUN", "0") == "1"
gen_orientation = pathlib.Path(os.environ["ANCHOR_GEN_ORIENTATION"])
bash_bin = os.environ.get("ANCHOR_BASH", "bash")
ANCHOR_HEADER = ["anchor_id", "doc", "section", "trigger_domains", "content_hash", "lifecycle"]
PENDING_RE = re.compile(r"^pending:([A-Z0-9][A-Z0-9-]*-[0-9]+)$")
CANONIZATION_RUNG = "CORE-CANONIZATION-0"


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
    sys.exit(1 if mode in ("check", "resync", "pending") else 0)


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
        if reader.fieldnames != ANCHOR_HEADER:
            fail("anchor-table")
        seen = set()
        for row in reader:
            if not row.get("anchor_id"):
                continue
            for key in ANCHOR_HEADER:
                if not row.get(key):
                    fail("anchor-table")
            if row["anchor_id"] in seen:
                fail("anchor-table")
            seen.add(row["anchor_id"])
            lifecycle = row["lifecycle"].strip()
            if lifecycle != "canonical" and not PENDING_RE.fullmatch(lifecycle):
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
            "lifecycle": row["lifecycle"],
        }
        if live != row["content_hash"].lower():
            fail("anchor-hash-drift")
    return out


def load_rung_truth():
    try:
        command_path = gen_orientation.relative_to(repo).as_posix()
    except ValueError:
        command_path = str(gen_orientation)
    result = subprocess.run(
        [bash_bin, command_path, "--rung-truth"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=False,
        env=os.environ.copy(),
    )
    if result.returncode != 0 or "RUNG-TRUTH-VERDICT: PASS" not in result.stdout:
        fail("rung-truth")
    states = {}
    for line in result.stdout.splitlines():
        match = re.fullmatch(r"RUNG-TRUTH: id=(\S+) state=(completed|open|superseded)", line.strip())
        if match:
            states[match.group(1)] = match.group(2)
    return states


def pending_dispositions(rows):
    pending = [row for row in rows if row["lifecycle"].startswith("pending:")]
    if not pending:
        return []
    truth = load_rung_truth()
    canonized = truth.get(CANONIZATION_RUNG) == "completed"
    out = []
    for row in pending:
        rung = row["lifecycle"].split(":", 1)[1]
        mint_state = truth.get(rung, "absent")
        if canonized:
            disposition = "STALE-PENDING"
            reason = "canonization-completed"
        elif mint_state == "completed":
            disposition = "PENDING-HEALTHY"
            reason = "minting-rung-graduated"
        else:
            disposition = "ORPHANED"
            reason = f"minting-rung-{mint_state}"
        out.append((row, disposition, rung, reason))
    return out


def emit_pending(dispositions):
    counts = {"PENDING-HEALTHY": 0, "ORPHANED": 0, "STALE-PENDING": 0}
    for row, disposition, rung, reason in dispositions:
        counts[disposition] += 1
        print(
            f"ANCHOR-PENDING: disposition={disposition} anchor_id={row['anchor_id']} "
            f"rung={rung} doc={row['doc']} reason={reason}"
        )
    print(
        "ANCHOR-PENDING-VERDICT: PASS "
        f"healthy={counts['PENDING-HEALTHY']} orphaned={counts['ORPHANED']} "
        f"stale={counts['STALE-PENDING']}"
    )


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
                fieldnames=ANCHOR_HEADER,
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
    joined = "|".join(
        f"{k}:{state[k]['live_hash']}:{state[k]['lifecycle']}" for k in sorted(state)
    )
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:16]


rows = load_rows()

if mode == "resync":
    cmd_resync(rows)

state = live_hashes(rows)

dispositions = pending_dispositions(rows)

if mode == "pending":
    emit_pending(dispositions)
    sys.exit(0)

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
    emit_pending(dispositions)
    stale = [row for row, disposition, _, _ in dispositions if disposition == "STALE-PENDING"]
    orphaned = [row for row, disposition, _, _ in dispositions if disposition == "ORPHANED"]
    if stale:
        fail("stale-pending")
    if orphaned:
        fail("orphaned-pending")
    # COVERAGE, not integrity. Everything above verifies that rows which EXIST
    # still point at live headings with unchanged hashes. Nothing asked whether
    # doctrine exists that NO row points at -- and the anchor library was a
    # one-time hand-enumerated catalogue (OC-ANCHOR-CATALOG-0), so a doc nobody
    # listed simply never entered it. AGENTS.md routes doctrine through
    # anchor_query.sh, which makes an unanchored doc operationally invisible
    # while still looking authoritative in the tree.
    #
    # Settled doctrine is the class that goes dark: a track actively pushing new
    # doctrine gets it anchored, while invariants that are already true have no
    # advocate. That is how `AccumulatorRole is compile-time metadata only` fell
    # out of reach and got re-derived from scratch two rungs running.
    #
    # ADVISORY ONLY. This never fails the build -- an unanchored doc may be
    # archive, a worklog, or genuinely superseded, and a gate that forces every
    # markdown file into the anchor table would be exactly the ceremony the
    # anti-kabuki floor forbids. It reports; a human dispositions.
    # Fixture sandboxes carry a miniature corpus; coverage is a statement about
    # the REAL docs tree, so it is meaningless there and would break fixtures
    # that assert exact stdout.
    if fixture_dir:
        pass_ok()
    covered = {r["doc"].replace("\\", "/") for r in rows}
    corpus = sorted(
        str(p).replace("\\", "/")
        for p in list(repo.glob("docs/*.md")) + list(repo.glob("docs/adr/*.md"))
    )
    prefix = str(repo).replace("\\", "/") + "/"
    dark = [c[len(prefix):] if c.startswith(prefix) else c for c in corpus]
    dark = [d for d in dark if d not in covered]
    if dark:
        print(f"ANCHOR-COVERAGE: INSPECT unanchored={len(dark)}/{len(dark) + len(covered)}")
        for d in dark:
            print(f"  unanchored: {d}")
        print("  disposition: anchor it (doctrine agents must reach) or delete it (superseded).")
    else:
        print("ANCHOR-COVERAGE: PASS every docs/ and docs/adr/ file is anchored")

    # ANCHOR-CURATION: anchors grow monotonically by design -- settled doctrine does
    # not stop being true when a phase closes, and reaping it is how 6.4/6.5 came to
    # re-derive an invariant adjudicated months earlier. But growth without
    # re-curation lets superseded rows sit indefinitely looking authoritative, so
    # this flags them for edit/resync. Advisory: it reports, a human dispositions.
    #
    # Two signals, deliberately separate because they mean different things:
    #   SUPERSEDED - the anchored doc now lives under an archive/superseded path.
    #                Definitionally stale; the row should be re-pointed or removed.
    #   UNREACHED  - the anchor has never been served in the reach log. Usually a
    #                wrong or too-narrow trigger domain rather than obsolescence,
    #                and freshly added anchors appear here until first served, so
    #                this is a review prompt, NEVER a deletion signal.
    superseded = sorted(
        r["anchor_id"] for r in rows
        if "/archive/" in r["doc"].replace("\\", "/")
        or "superseded" in r["doc"].replace("\\", "/")
    )
    reach = repo / "scripts" / "ci" / "anchor_reach_log.tsv"
    reach_text = reach.read_text(encoding="utf-8", errors="replace") if reach.is_file() else ""
    unreached = sorted(r["anchor_id"] for r in rows if r["anchor_id"] not in reach_text)
    if superseded or unreached:
        print(
            f"ANCHOR-CURATION: INSPECT superseded={len(superseded)} "
            f"unreached={len(unreached)}/{len(rows)}"
        )
        for a in superseded:
            print(f"  superseded (doc archived; re-point or remove): {a}")
        for a in unreached:
            print(f"  unreached (check trigger_domains; not a deletion signal): {a}")
    else:
        print(f"ANCHOR-CURATION: PASS rows={len(rows)}")
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
  printf 'anchor_id\tdoc\tsection\ttrigger_domains\tcontent_hash\tlifecycle\n' >"$tmp/doctrine_anchors.tsv"
  printf 'sample-anchor\tdocs/sample.md\theading:# Architecture Decision Records\ttest-domain\t0000000000000000000000000000000000000000000000000000000000000000\tcanonical\n' >>"$tmp/doctrine_anchors.tsv"
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

  printf 'anchor_id\tdoc\tsection\ttrigger_domains\tcontent_hash\tlifecycle\n' >"$tmp/doctrine_anchors.tsv"
  printf 'sample-anchor\tdocs/sample.md\theading:# Missing Title That Moved\ttest-domain\t0000000000000000000000000000000000000000000000000000000000000000\tcanonical\n' >>"$tmp/doctrine_anchors.tsv"
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

run_pending_selftests() {
  local tmp hash out rc
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/anchor-pending-XXXXXX")"
  mkdir -p "$tmp/docs"
  printf '# Pending anchor\nBody.\n' >"$tmp/docs/sample.md"
  hash="$("$PYTHON_BIN" -c 'import hashlib; print(hashlib.sha256(b"# Pending anchor\nBody.\n").hexdigest())')"

  write_pending_row() {
    local anchor_id="$1" rung="$2"
    printf 'anchor_id\tdoc\tsection\ttrigger_domains\tcontent_hash\tlifecycle\n' >"$tmp/doctrine_anchors.tsv"
    printf '%s\tdocs/sample.md\tlines:1-2\ttest-domain\t%s\tpending:%s\n' \
      "$anchor_id" "$hash" "$rung" >>"$tmp/doctrine_anchors.tsv"
  }
  write_design() {
    local mint="$1" canon="$2"
    printf '# Fixture workplan\n\nProduction track PR ladder.\n\n| # | Rung | Deliverable | Exit proof |\n|---|---|---|---|\n| 1 | `HEALTHY-RUNG-0` | fixture | %s |\n| 2 | `CORE-CANONIZATION-0` | fixture | %s |\n' \
      "$mint" "$canon" >"$tmp/design.md"
  }

  FIXTURE_DIR="$tmp"
  export FIXTURE_DIR
  export ORIENTATION_DESIGN_DOC="$tmp/design.md"

  write_pending_row healthy-anchor HEALTHY-RUNG-0
  write_design 'DA-GRADUATED / merged #1 @ abcdef0' 'TODO'
  out="$(run_python pending 2>&1 || true)"
  if printf '%s\n' "$out" | grep -q 'disposition=PENDING-HEALTHY anchor_id=healthy-anchor'; then
    echo "PASS pending_healthy_advisory"
  else
    echo "FAIL pending_healthy_advisory"; echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi
  if out="$(run_python check 2>&1)" && printf '%s\n' "$out" | grep -q 'ANCHOR-CHECK-VERDICT: PASS'; then
    echo "PASS pending_healthy_check_green"
  else
    echo "FAIL pending_healthy_check_green"; echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi

  write_pending_row orphan-anchor MISSING-RUNG-0
  out="$(run_python pending 2>&1 || true)"
  if printf '%s\n' "$out" | grep -q 'disposition=ORPHANED anchor_id=orphan-anchor'; then
    echo "PASS pending_missing_rung_orphaned"
  else
    echo "FAIL pending_missing_rung_orphaned"; echo "  got: $out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi

  write_pending_row stale-anchor HEALTHY-RUNG-0
  write_design 'DA-GRADUATED / merged #1 @ abcdef0' 'DA-GRADUATED / merged #2 @ abcdef1'
  out="$(run_python pending 2>&1 || true)"
  set +e
  local check_out
  check_out="$(run_python check 2>&1)"
  rc=$?
  set -e
  if printf '%s\n' "$out" | grep -q 'disposition=STALE-PENDING anchor_id=stale-anchor' \
      && [[ "$rc" -ne 0 ]] && printf '%s\n' "$check_out" | grep -q 'FAIL(stale-pending)'; then
    echo "PASS pending_canonization_live_stale"
  else
    echo "FAIL pending_canonization_live_stale"; echo "  got: $out / $check_out"
    SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
  fi

  FIXTURE_DIR=""
  unset FIXTURE_DIR ORIENTATION_DESIGN_DOC
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
  run_pending_selftests
  local total=$((${#fixtures[@]} + 7))
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
  got="$(run_python check 2>&1 | grep 'ANCHOR-CHECK-VERDICT:' | tail -n 1 || true)"
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
