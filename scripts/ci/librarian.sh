#!/usr/bin/env bash
# HD-LIBRARIAN-0 - stewardship reports and safe cull front door.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

if command -v cygpath >/dev/null 2>&1; then
  LIBRARIAN_BASH="$(cygpath -w "$(command -v bash)" 2>/dev/null || command -v bash)"
else
  LIBRARIAN_BASH="$(command -v bash)"
fi

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/librarian.sh --staleness
  bash scripts/ci/librarian.sh --cull [--confirm]
  bash scripts/ci/librarian.sh --catalog [--role coding|orchestrator|da]
  bash scripts/ci/librarian.sh --selftest
EOF
  exit 2
}

[[ $# -ge 1 ]] || usage

MODE="$1"; shift || true
case "$MODE" in
  --staleness|--cull|--catalog|--selftest) ;;
  -h|--help) usage ;;
  *) echo "librarian.sh: unknown mode: ${MODE}" >&2; usage ;;
esac

LIBRARIAN_MODE="$MODE" \
LIBRARIAN_REPO_ROOT="$REPO_ROOT" \
LIBRARIAN_SCRIPT_DIR="$SCRIPT_DIR" \
LIBRARIAN_BASH="$LIBRARIAN_BASH" \
  exec "$PYTHON_BIN" - "$@" <<'PY'
import os
import csv
import pathlib
import stat
import subprocess
import sys
import tempfile

MODE = os.environ["LIBRARIAN_MODE"]
ROOT = pathlib.Path(os.environ["LIBRARIAN_REPO_ROOT"])
SCRIPT_DIR = pathlib.Path(os.environ["LIBRARIAN_SCRIPT_DIR"])
argv = sys.argv[1:]
BASH_BIN = os.environ.get("LIBRARIAN_BASH", "bash")
LINE_CAP = int(os.environ.get("LIBRARIAN_LINE_CAP", "60"))
ROLES = {"coding", "orchestrator", "da"}

ANCHOR_CHECK = pathlib.Path(os.environ.get("LIBRARIAN_ANCHOR_CHECK", SCRIPT_DIR / "anchor_check.sh"))
ANCHOR_QUERY = pathlib.Path(os.environ.get("LIBRARIAN_ANCHOR_QUERY", SCRIPT_DIR / "anchor_query.sh"))
DOC_BUDGET = pathlib.Path(os.environ.get("LIBRARIAN_DOC_BUDGET", SCRIPT_DIR / "doc_budget_check.sh"))
HANDOFF_DISPATCH = pathlib.Path(os.environ.get("LIBRARIAN_HANDOFF_DISPATCH", SCRIPT_DIR / "handoff_dispatch.sh"))
ORIENT = pathlib.Path(os.environ.get("LIBRARIAN_ORIENT", SCRIPT_DIR / "orient.sh"))
TRACK_CLOSEOUT = pathlib.Path(os.environ.get("LIBRARIAN_TRACK_CLOSEOUT", SCRIPT_DIR / "track_closeout.sh"))
HANDOFF_PATH = os.environ.get("LIBRARIAN_HANDOFF", "handoffs/HD-LIBRARIAN-0.hd.md")
SELF = pathlib.Path(os.environ.get("LIBRARIAN_SELF", SCRIPT_DIR / "librarian.sh"))


def script_arg(path: pathlib.Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return str(path)


def run_script(path: pathlib.Path, *args, env=None):
    merged = os.environ.copy()
    if env:
        merged.update(env)
    return subprocess.run(
        [BASH_BIN, script_arg(path), *args],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        env=merged,
        check=False,
    )


def clean_lines(text: str):
    return [line.rstrip() for line in text.splitlines() if line.strip()]


def verdict_line(text: str, marker: str) -> str:
    for line in reversed(clean_lines(text)):
        if marker in line:
            return line
    return f"{marker}: MISSING"


def first_line(text: str, marker: str) -> str:
    for line in clean_lines(text):
        if marker in line:
            return line
    return f"{marker}: MISSING"


def emit(lines, verdict_prefix):
    if len(lines) > LINE_CAP:
        print(f"{verdict_prefix}: FAIL(report-line-cap lines={len(lines)} max={LINE_CAP})")
        return 1
    print("\n".join(lines))
    return 0


def append_owner_block(lines, title, result, keep=None):
    lines.append(f"{title}: exit={result.returncode}")
    source = clean_lines(result.stdout)
    if keep:
        source = [line for line in source if keep(line)]
    lines.extend(source)


def owner_ok(result, allow_inspect=False):
    text = result.stdout.upper()
    if result.returncode != 0:
        return False
    if "FAIL(" in text or "MISSING" in text:
        return False
    if not allow_inspect and "INSPECT" in text:
        return False
    return True


def read_tsv(path: pathlib.Path):
    with path.open(encoding="utf-8", newline="") as fh:
        return list(csv.DictReader(fh, delimiter="\t"))


def split_csv(value: str):
    return [item.strip() for item in (value or "").split(",") if item.strip()]


def harness_fixture_count():
    rows = read_tsv(ROOT / "scripts/ci/test_inventory.tsv")
    return sum(1 for row in rows if (row.get("birth_track") or "").strip() == "harness-fixture")


def cmd_staleness():
    lines = ["LIBRARIAN STALENESS"]
    lines.append(f"harness-fixture-count: {harness_fixture_count()}")
    anchor = run_script(ANCHOR_CHECK, "--resync", "--dry-run")
    append_owner_block(lines, "anchor-resync-preview", anchor)
    dead = run_script(ANCHOR_QUERY, "--dead-listeners")
    append_owner_block(lines, "dead-listeners", dead)
    prune = run_script(ANCHOR_QUERY, "--prune", "--dry-run")
    append_owner_block(lines, "reach-log-prune-preview", prune)
    expiry = run_script(TRACK_CLOSEOUT, "--artifact-expiry")
    append_owner_block(lines, "lease-aging", expiry)
    discover = run_script(TRACK_CLOSEOUT, "--discover")
    append_owner_block(lines, "closeout-discover", discover)
    budget = run_script(DOC_BUDGET, "--headroom")
    append_owner_block(lines, "doc-budget-headroom", budget)
    verdict = "PASS" if all(owner_ok(r) for r in (anchor, dead, prune, expiry, discover, budget)) else "INSPECT"
    lines.append(f"LIBRARIAN-STALENESS-VERDICT: {verdict} lines={len(lines) + 1}")
    return emit(lines, "LIBRARIAN-STALENESS-VERDICT")


def decommission_items(text: str):
    items = []
    for line in clean_lines(text):
        stripped = line.strip()
        if stripped.startswith("- rm "):
            items.append(f"cull-item: REAP source=track_closeout target={stripped[5:]}")
        elif stripped.startswith("! manual:"):
            items.append(f"cull-item: DA-ROUTE source=track_closeout target={stripped[len('! manual:'):].strip()}")
    if not items:
        items.append("cull-item: KEEP source=track_closeout reason=no-expired-safe-items")
    return items


def prune_items(text: str):
    items = []
    for line in clean_lines(text):
        if line.startswith("ANCHOR-QUERY-PRUNE-ITEM:"):
            items.append("cull-item: " + line.split(":", 1)[1].strip() + " source=anchor_query")
    if not items:
        items.append("cull-item: KEEP source=anchor_query reason=no-reach-log-prune-candidates")
    return items


def orphan_items(text: str):
    items = []
    for line in clean_lines(text):
        if line.startswith("ORPHANED "):
            items.append(f"cull-item: DA-ROUTE source=anchor_check target={line.split(' ', 1)[1]} reason=orphan-anchor")
        elif line.startswith("RESYNCED "):
            items.append(f"cull-item: DA-ROUTE source=anchor_check target={line.split(' ', 1)[1]} reason=authority-resync")
    if not items:
        items.append("cull-item: KEEP source=anchor_check reason=no-anchor-retirement-candidates")
    return items


def anchor_preview_ok(result):
    if owner_ok(result):
        return True, []
    evidence = [
        line for line in clean_lines(result.stdout)
        if line.startswith("ORPHANED ") or line.startswith("RESYNCED ")
    ]
    if evidence:
        return True, []
    detail = verdict_line(result.stdout, "ANCHOR-RESYNC-VERDICT")
    if "MISSING" in detail:
        detail = verdict_line(result.stdout, "ANCHOR-CHECK-VERDICT")
    return False, [f"cull-item: ERROR source=anchor_check reason=anchor-preview-failed detail={detail}"]


def cmd_cull():
    confirm = False
    for arg in argv:
        if arg == "--confirm":
            confirm = True
        else:
            print(f"librarian.sh: unexpected --cull arg: {arg}", file=sys.stderr)
            return 2
    lines = ["LIBRARIAN CULL", f"mode: {'confirm' if confirm else 'dry-run'}"]
    discover = run_script(TRACK_CLOSEOUT, "--discover")
    lines.append("track-closeout-discover: " + verdict_line(discover.stdout, "TRACK-CLOSEOUT-DISCOVER-VERDICT"))
    decommission_plan = run_script(TRACK_CLOSEOUT, "--decommission", "--dry-run")
    lines.extend(decommission_items(decommission_plan.stdout))
    lines.append("track-closeout-decommission: " + verdict_line(decommission_plan.stdout, "TRACK-CLOSEOUT-DECOMMISSION-VERDICT"))
    prune_plan = run_script(ANCHOR_QUERY, "--prune", "--dry-run")
    prune = prune_plan
    lines.extend(prune_items(prune.stdout))
    lines.append("reach-log-prune: " + verdict_line(prune.stdout, "ANCHOR-QUERY-PRUNE:"))
    anchor = run_script(ANCHOR_CHECK, "--resync", "--dry-run")
    lines.extend(orphan_items(anchor.stdout))
    lines.append("orphan-anchor-preview: " + verdict_line(anchor.stdout, "ANCHOR-RESYNC-VERDICT"))
    anchor_ok, anchor_errors = anchor_preview_ok(anchor)
    lines.extend(anchor_errors)
    ok = all(owner_ok(r) for r in (discover, decommission_plan, prune_plan)) and anchor_ok
    preflight_verdict = ("OK" if confirm else "DRY") if ok else "ERROR"
    mutation_tail = []
    if confirm and ok:
        mutation_tail = [
            "track-closeout-apply: PENDING",
            "reach-log-prune-apply: PENDING",
        ]
    trial = lines + mutation_tail + [f"LIBRARIAN-CULL-VERDICT: {preflight_verdict} lines={len(lines) + len(mutation_tail) + 1}"]
    if len(trial) > LINE_CAP:
        print(f"LIBRARIAN-CULL-VERDICT: FAIL(report-line-cap lines={len(trial)} max={LINE_CAP})")
        return 1
    if confirm and ok:
        decommission_apply = run_script(TRACK_CLOSEOUT, "--decommission")
        prune_apply = run_script(ANCHOR_QUERY, "--prune")
        lines.append("track-closeout-apply: " + verdict_line(decommission_apply.stdout, "TRACK-CLOSEOUT-DECOMMISSION-VERDICT"))
        lines.append("reach-log-prune-apply: " + verdict_line(prune_apply.stdout, "ANCHOR-QUERY-PRUNE:"))
        ok = ok and owner_ok(decommission_apply) and owner_ok(prune_apply)
    verdict = ("OK" if confirm else "DRY") if ok else "ERROR"
    lines.append(f"LIBRARIAN-CULL-VERDICT: {verdict} lines={len(lines) + 1}")
    rc = emit(lines, "LIBRARIAN-CULL-VERDICT")
    if rc != 0:
        return rc
    return 0 if ok else 1


def trigger_domains():
    rows = read_tsv(ROOT / "scripts/ci/anchor_triggers.tsv")
    out = set()
    for row in rows:
        out.update(split_csv(row.get("trigger_domains", "")))
    return sorted(out)


def parse_anchor_ids(text: str):
    for line in clean_lines(text):
        if line.startswith("anchors:"):
            raw = line.split(":", 1)[1].strip()
            if raw == "none":
                return []
            return [item.strip() for item in raw.split(",") if item.strip()]
    return []


def domain_anchor_hits(domains):
    hits = {domain: set() for domain in domains}
    ok = True
    with tempfile.TemporaryDirectory(prefix="librarian-catalog-domain-reach-") as raw:
        reach_log = str(pathlib.Path(raw) / "anchor_reach_log.tsv")
        for domain in domains:
            result = run_script(
                ANCHOR_QUERY,
                "--domain",
                domain,
                env={"ANCHOR_REACH_LOG_PATH": reach_log},
            )
            ok = ok and result.returncode == 0
            hits[domain].update(parse_anchor_ids(result.stdout))
    return hits, ok


def catalog_anchor_lines():
    domains = trigger_domains()
    hits, ok = domain_anchor_hits(domains)
    anchor_rows = read_tsv(ROOT / "scripts/ci/doctrine_anchors.tsv")
    lines = []
    for row in anchor_rows:
        aid = row.get("anchor_id", "").strip()
        declared = split_csv(row.get("trigger_domains", ""))
        reachable = [domain for domain in declared if aid in hits.get(domain, set())]
        shown = reachable or declared
        lines.append(f"anchor: {aid} domains={','.join(shown) if shown else 'none'}")
    return lines, ok, len(anchor_rows), len(domains)


def probe_handoff_path(tmp: pathlib.Path):
    path = tmp / "CATALOG-PROBE.hd.md"
    path.write_text(
        """---
rung: CATALOG-PROBE
kind: rung
track: catalog-probe
base_sha: 0000000000000000000000000000000000000000
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: catalog probe
surfaces: ["scripts/ci/librarian.sh"]
forbidden: ["crates/**"]
required_checks: ["librarian-selftest"]
stop_conditions: ["catalog-probe-stop"]
---
## BUILD
- catalog probe build
## FENCES
- catalog probe fence
## EXIT-PROOF
- catalog probe proof
""",
        encoding="utf-8",
        newline="\n",
    )
    return path


def role_payload_sections(role: str):
    labels = []
    with tempfile.TemporaryDirectory(prefix="librarian-catalog-probe-") as raw:
        handoff = probe_handoff_path(pathlib.Path(raw))
        result = run_script(HANDOFF_DISPATCH, "--render", role, str(handoff))
    prefixes = (
        "required_checks:",
        "forbidden_surfaces:",
        "routing:",
        "audit_targets:",
        "risk_class:",
        "expected_residue:",
    )
    for line in clean_lines(result.stdout):
        if line.startswith("## "):
            labels.append(line.removeprefix("## "))
        elif any(line.startswith(prefix) for prefix in prefixes):
            labels.append(line.split(":", 1)[0])
    return labels, result.returncode


def orientation_spine(orient_text: str):
    anchor_rows = read_tsv(ROOT / "scripts/ci/doctrine_anchors.tsv")
    anchor_ids = {row.get("anchor_id", "").strip() for row in anchor_rows}
    found = []
    for line in clean_lines(orient_text):
        if not line.startswith("- "):
            continue
        for aid in anchor_ids:
            if f"`{aid}`" in line and aid not in found:
                found.append(aid)
    return found


def role_catalog(role: str, anchor_lines, anchor_count, domain_count):
    orient = run_script(ORIENT, f"--role={role}")
    payload, payload_code = role_payload_sections(role)
    spine = orientation_spine(orient.stdout)
    out = [f"LIBRARIAN CATALOG role={role}"]
    out.append(f"library-source: anchors={anchor_count} trigger_domains={domain_count}")
    out.append("always-on-spine: " + (",".join(spine) if spine else "none"))
    out.append("payload-sections: " + (",".join(payload) if payload else "none"))
    out.append("orientation: " + first_line(orient.stdout, "ORIENT-RECEIPT"))
    out.extend(anchor_lines)
    out.append(f"LIBRARIAN-CATALOG-VERDICT: PASS role={role} lines={len(out) + 1}")
    return out, (orient.returncode, payload_code)


def cmd_catalog():
    roles = ["coding", "orchestrator", "da"]
    if argv:
        if len(argv) == 2 and argv[0] == "--role" and argv[1] in ROLES:
            roles = [argv[1]]
        else:
            print("librarian.sh: expected --catalog [--role coding|orchestrator|da]", file=sys.stderr)
            return 2
    anchor_lines, anchor_ok, anchor_count, domain_count = catalog_anchor_lines()
    lines = []
    ok = True
    for ix, role in enumerate(roles):
        if ix:
            lines.append("")
        role_lines, codes = role_catalog(role, anchor_lines, anchor_count, domain_count)
        if len(role_lines) > LINE_CAP:
            print(f"LIBRARIAN-CATALOG-VERDICT: FAIL(report-line-cap role={role} lines={len(role_lines)} max={LINE_CAP})")
            return 1
        lines.extend(role_lines)
        ok = ok and all(code == 0 for code in codes)
    ok = ok and anchor_ok
    if not ok:
        lines.append("LIBRARIAN-CATALOG-VERDICT: ERROR(owner-command-failed)")
    print("\n".join(lines))
    return 0 if ok else 1


def write_exe(path: pathlib.Path, text: str):
    path.write_text(text, encoding="utf-8", newline="\n")
    path.chmod(path.stat().st_mode | stat.S_IXUSR)


def fake_suite(tmp: pathlib.Path):
    fake_dir = tmp / "fake"
    fake_dir.mkdir()
    write_exe(fake_dir / "track_closeout.sh", r'''#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --discover)
    echo "TRACK-CLOSEOUT-DISCOVER-VERDICT: OK ripe=1 leased=1 parked=1"
    ;;
  --artifact-expiry)
    if [[ "${FAKE_EXPIRY_VERDICT:-PASS}" == "FAIL" ]]; then
      echo "ARTIFACT-EXPIRY-VERDICT: FAIL expired=1 cruft=0 malformed=0"
      exit 1
    elif [[ "${FAKE_EXPIRY_VERDICT:-PASS}" == "INSPECT" ]]; then
      echo "ARTIFACT-EXPIRY-VERDICT: INSPECT expired=0 cruft=1 malformed=0"
      exit 0
    fi
    echo "ARTIFACT-EXPIRY-VERDICT: PASS expired=0 cruft=0 malformed=0"
    ;;
  --decommission)
    echo "TRACK-CLOSEOUT DECOMMISSION"
    if [[ "$*" == *"--dry-run"* ]]; then
      if [[ "${FAKE_CULL_ITEMS:-1}" -gt 1 ]]; then
        i=1
        while [[ "$i" -le "${FAKE_CULL_ITEMS}" ]]; do
          echo "    ! manual: crates/x/src/lib${i}.rs::unit - inline/src or shared file"
          i=$((i+1))
        done
      fi
      echo "    ! manual: crates/x/src/lib.rs::unit — inline/src or shared file"
      echo "TRACK-CLOSEOUT-DECOMMISSION-VERDICT: DRY reaped=0 files=0 manual=1"
    else
      if [[ -n "${FAKE_GUARD_FILE:-}" ]]; then
        printf 'mutated\n' >"${FAKE_GUARD_FILE}"
      fi
      echo "    ! manual: crates/x/src/lib.rs::unit — inline/src or shared file"
      echo "TRACK-CLOSEOUT-DECOMMISSION-VERDICT: OK reaped=0 files=0 manual=1"
    fi
    ;;
esac
''')
    write_exe(fake_dir / "anchor_query.sh", r'''#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --dead-listeners)
    n="${FAKE_DEAD_COUNT:-1}"
    i=1
    while [[ "$i" -le "$n" ]]; do
      echo "ANCHOR-QUERY-DEAD-LISTENER-ITEM: DA-ROUTE glob=missing/${i}/** domains=dead-${i}"
      i=$((i+1))
    done
    echo "ANCHOR-QUERY-DEAD-LISTENERS-VERDICT: PASS count=${n}"
    ;;
  --prune)
    if [[ "$*" == *"--dry-run"* ]]; then
      echo "ANCHOR-QUERY-PRUNE-ITEM: REAP date=2020-01-01T00:00:00Z role=coding query=--grep-old anchors=none hit=none"
      echo "ANCHOR-QUERY-PRUNE: DRY removed=1 kept=0 days=30"
    else
      echo "ANCHOR-QUERY-PRUNE-ITEM: REAP date=2020-01-01T00:00:00Z role=coding query=--grep-old anchors=none hit=none"
      echo "ANCHOR-QUERY-PRUNE: PASS removed=1 kept=0 days=30"
    fi
    ;;
  --paths)
    log="${ANCHOR_REACH_LOG_PATH:-${FAKE_LIVE_REACH_LOG:-}}"
    if [[ -n "${log}" ]]; then
      printf 'catalog path touched\n' >>"${log}"
    fi
    echo "ANCHOR-QUERY-VERDICT: PASS ids=1"
    echo "anchors: orientation-harness-core ${FAKE_CATALOG_MARKER:-one}"
    ;;
  --domain)
    log="${ANCHOR_REACH_LOG_PATH:-${FAKE_LIVE_REACH_LOG:-}}"
    if [[ -n "${log}" ]]; then
      printf 'catalog domain touched\n' >>"${log}"
    fi
    echo "ANCHOR-QUERY-VERDICT: PASS ids=1"
    echo "anchors: orientation-harness-core"
    ;;
esac
''')
    write_exe(fake_dir / "anchor_check.sh", r'''#!/usr/bin/env bash
set -euo pipefail
if [[ "${FAKE_ANCHOR_VERDICT:-orphan}" == "harness-fail" ]]; then
  echo "ANCHOR-CHECK-VERDICT: FAIL(malformed-authority)"
  exit 1
fi
echo "ORPHANED sample-anchor"
echo "ANCHOR-CHECK-VERDICT: FAIL(orphaned-anchor) remedy=repair doctrine_anchors.tsv section target or run bash scripts/ci/anchor_check.sh --resync"
''')
    write_exe(fake_dir / "doc_budget_check.sh", r'''#!/usr/bin/env bash
set -euo pipefail
echo "DOC-BUDGET-HEADROOM-ITEM: path=docs/x.md lines=1/9 headroom=8"
echo "DOC-BUDGET-HEADROOM-VERDICT: PASS over=0 rows=1"
''')
    write_exe(fake_dir / "handoff_dispatch.sh", r'''#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--render" ]]; then
  role="${2:-coding}"
  echo "HD-RECEIPT: fake-${FAKE_CATALOG_MARKER:-one}"
  echo "REQUIRED-ANCHORS: orientation-harness-core"
  case "${role}" in
    coding)
      echo "required_checks:"
      echo "forbidden_surfaces:"
      ;;
    orchestrator)
      echo "routing:"
      ;;
    da)
      echo "audit_targets:"
      echo "risk_class: gate-wiring"
      echo "expected_residue: DA deep audit only"
      echo "forbidden_surfaces:"
      ;;
  esac
  if [[ "${FAKE_MANY_PAYLOAD:-0}" -eq 1 ]]; then
    i=1
    while [[ "$i" -le 12 ]]; do
      echo "## PAYLOAD-${i}-${FAKE_CATALOG_MARKER:-one}"
      i=$((i+1))
    done
  fi
  echo "## BUILD"
  echo "## FENCES"
  echo "## EXIT-PROOF"
fi
''')
    write_exe(fake_dir / "orient.sh", r'''#!/usr/bin/env bash
set -euo pipefail
echo "ORIENT-RECEIPT: ${FAKE_CATALOG_MARKER:-one}"
''')
    return fake_dir


def run_self_with(fake_dir: pathlib.Path, mode: str, *extra, env=None):
    merged = {
        "LIBRARIAN_TRACK_CLOSEOUT": str(fake_dir / "track_closeout.sh"),
        "LIBRARIAN_ANCHOR_QUERY": str(fake_dir / "anchor_query.sh"),
        "LIBRARIAN_ANCHOR_CHECK": str(fake_dir / "anchor_check.sh"),
        "LIBRARIAN_DOC_BUDGET": str(fake_dir / "doc_budget_check.sh"),
        "LIBRARIAN_HANDOFF_DISPATCH": str(fake_dir / "handoff_dispatch.sh"),
        "LIBRARIAN_ORIENT": str(fake_dir / "orient.sh"),
    }
    if env:
        merged.update(env)
    return run_script(SELF, mode, *extra, env=merged)


def run_selftest():
    failures = []
    with tempfile.TemporaryDirectory(prefix="librarian-selftest-") as raw:
        tmp = pathlib.Path(raw)
        fake_dir = fake_suite(tmp)
        sentinel = tmp / "sentinel.tsv"
        sentinel.write_text("unchanged\n", encoding="utf-8")

        before = sentinel.read_bytes()
        dry = run_self_with(fake_dir, "--cull")
        after = sentinel.read_bytes()
        if dry.returncode == 0 and before == after and "LIBRARIAN-CULL-VERDICT: DRY" in dry.stdout:
            print("PASS cull-dry-run-default")
        else:
            print("FAIL cull-dry-run-default")
            failures.append("cull-dry-run-default")

        confirm = run_self_with(fake_dir, "--cull", "--confirm")
        if confirm.returncode == 0 and before == sentinel.read_bytes() and "cull-item: DA-ROUTE source=track_closeout target=crates/x/src/lib.rs::unit" in confirm.stdout:
            print("PASS src-path-routes-to-DA")
        else:
            print("FAIL src-path-routes-to-DA")
            failures.append("src-path-routes-to-DA")

        cap = run_self_with(fake_dir, "--staleness", env={"FAKE_DEAD_COUNT": "70"})
        if cap.returncode != 0 and "LIBRARIAN-STALENESS-VERDICT: FAIL(report-line-cap" in cap.stdout:
            print("PASS staleness-report-cap")
        else:
            print("FAIL staleness-report-cap")
            failures.append("staleness-report-cap")

        guarded = tmp / "confirm-guard.tsv"
        guarded.write_text("unchanged\n", encoding="utf-8")
        preflight = run_self_with(
            fake_dir,
            "--cull",
            "--confirm",
            env={"FAKE_CULL_ITEMS": "70", "FAKE_GUARD_FILE": str(guarded)},
        )
        if preflight.returncode != 0 and guarded.read_text(encoding="utf-8") == "unchanged\n" and "FAIL(report-line-cap" in preflight.stdout:
            print("PASS confirm-preflight-cap-no-write")
        else:
            print("FAIL confirm-preflight-cap-no-write")
            failures.append("confirm-preflight-cap-no-write")

        staleness_truth = run_self_with(fake_dir, "--staleness", env={"FAKE_EXPIRY_VERDICT": "FAIL"})
        if "LIBRARIAN-STALENESS-VERDICT: PASS" not in staleness_truth.stdout and "LIBRARIAN-STALENESS-VERDICT: INSPECT" in staleness_truth.stdout:
            print("PASS staleness-owner-failure-not-pass")
        else:
            print("FAIL staleness-owner-failure-not-pass")
            failures.append("staleness-owner-failure-not-pass")

        staleness_fixture_gauge = run_self_with(fake_dir, "--staleness")
        if "harness-fixture-count:" in staleness_fixture_gauge.stdout:
            print("PASS staleness-harness-fixture-count")
        else:
            print("FAIL staleness-harness-fixture-count")
            failures.append("staleness-harness-fixture-count")

        catalog_one = run_self_with(fake_dir, "--catalog", "--role", "coding", env={"FAKE_CATALOG_MARKER": "one"})
        catalog_two = run_self_with(fake_dir, "--catalog", "--role", "coding", env={"FAKE_CATALOG_MARKER": "two"})
        catalog_coding = run_self_with(fake_dir, "--catalog", "--role", "coding")
        catalog_orch = run_self_with(fake_dir, "--catalog", "--role", "orchestrator")
        catalog_da = run_self_with(fake_dir, "--catalog", "--role", "da")
        all_roles = all(result.returncode == 0 for result in (catalog_coding, catalog_orch, catalog_da))
        roles_differ = len({catalog_coding.stdout, catalog_orch.stdout, catalog_da.stdout}) == 3
        content_assertions = (
            "payload-sections: required_checks,forbidden_surfaces,BUILD,FENCES,EXIT-PROOF" in catalog_coding.stdout
            and "payload-sections: routing" in catalog_orch.stdout
            and "payload-sections: audit_targets,risk_class,expected_residue,forbidden_surfaces" in catalog_da.stdout
        )
        if all_roles and roles_differ and content_assertions and catalog_one.stdout != catalog_two.stdout and "one" in catalog_one.stdout and "two" in catalog_two.stdout:
            print("PASS per-role-catalogs-differ")
        else:
            print("FAIL per-role-catalogs-differ")
            failures.append("per-role-catalogs-differ")

        catalog_many = run_self_with(fake_dir, "--catalog", "--role", "coding", env={"FAKE_MANY_PAYLOAD": "1"})
        if "PAYLOAD-11" in catalog_many.stdout or "LIBRARIAN-CATALOG-VERDICT: FAIL(report-line-cap" in catalog_many.stdout:
            print("PASS catalog-no-silent-slice")
        else:
            print("FAIL catalog-no-silent-slice")
            failures.append("catalog-no-silent-slice")

        live_log = tmp / "live-anchor-reach-log.tsv"
        live_log.write_text("live\n", encoding="utf-8")
        missing_log = tmp / "missing-anchor-reach-log.tsv"
        catalog_live = run_self_with(fake_dir, "--catalog", env={"FAKE_LIVE_REACH_LOG": str(live_log)})
        catalog_missing = run_self_with(fake_dir, "--catalog", env={"FAKE_LIVE_REACH_LOG": str(missing_log)})
        if (
            catalog_live.returncode == 0
            and catalog_missing.returncode == 0
            and live_log.read_text(encoding="utf-8") == "live\n"
            and not missing_log.exists()
        ):
            print("PASS catalog-readonly-reach-log")
        else:
            print("FAIL catalog-readonly-reach-log")
            failures.append("catalog-readonly-reach-log")

        anchor_guard = tmp / "anchor-guard.tsv"
        anchor_guard.write_text("unchanged\n", encoding="utf-8")
        anchor_fail = run_self_with(
            fake_dir,
            "--cull",
            "--confirm",
            env={"FAKE_ANCHOR_VERDICT": "harness-fail", "FAKE_GUARD_FILE": str(anchor_guard)},
        )
        if (
            anchor_fail.returncode != 0
            and anchor_guard.read_text(encoding="utf-8") == "unchanged\n"
            and "cull-item: ERROR source=anchor_check reason=anchor-preview-failed" in anchor_fail.stdout
        ):
            print("PASS anchor-harness-failure-no-mutation")
        else:
            print("FAIL anchor-harness-failure-no-mutation")
            failures.append("anchor-harness-failure-no-mutation")

        workflow = (ROOT / ".github" / "workflows" / "doctrine-exec-commands.yml").read_text(encoding="utf-8")
        if (
            "id: librarian" in workflow
            and "echo \"result=${rc}\" >> \"$GITHUB_OUTPUT\"" in workflow
            and "steps.librarian.outputs.result == '0'" in workflow
        ):
            print("PASS failed-confirm-no-commit")
        else:
            print("FAIL failed-confirm-no-commit")
            failures.append("failed-confirm-no-commit")

        if (
            "LIBRARIAN-OWNER-REVIEW:" in workflow
            and "requested_by=@${requester}" in workflow
            and "action=/librarian cull --confirm" in workflow
            and "needs.parse-command.outputs.librarian_action != 'cull' ||" in workflow
        ):
            print("PASS non-owner-confirm-routes-to-owner-review")
        else:
            print("FAIL non-owner-confirm-routes-to-owner-review")
            failures.append("non-owner-confirm-routes-to-owner-review")

        if (
            "COMMENT_ID=\"issue_comment-${{ github.event.comment.id }}\"" in workflow
            and "COMMENT_ID=\"pull_request_review-${{ github.event.review.id }}\"" in workflow
            and "COMMENT_ID=\"pull_request_review_comment-${{ github.event.comment.id }}\"" in workflow
            and "<!-- librarian-report:${{ needs.parse-command.outputs.comment_id }} -->" in workflow
        ):
            print("PASS librarian-report-identity-per-event")
        else:
            print("FAIL librarian-report-identity-per-event")
            failures.append("librarian-report-identity-per-event")

    if failures:
        print(f"LIBRARIAN-SELFTEST-VERDICT: FAIL count={len(failures)}")
        return 1
    print("LIBRARIAN-SELFTEST-VERDICT: PASS")
    return 0


if MODE == "--staleness":
    sys.exit(cmd_staleness())
if MODE == "--cull":
    sys.exit(cmd_cull())
if MODE == "--catalog":
    sys.exit(cmd_catalog())
if MODE == "--selftest":
    sys.exit(run_selftest())
print(f"librarian.sh: unhandled mode {MODE}", file=sys.stderr)
sys.exit(2)
PY
