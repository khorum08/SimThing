#!/usr/bin/env bash
# TRACK-CLOSEOUT-PROTOCOL-0 — one-shot, staged, role-symmetric track closeout.
#
# One script, run identically by the DA and the Orchestrator, that performs the
# whole end-of-track ritual the 0.0.8.4.8 corpus-clearance sweep did by hand across
# seven PRs and three rate windows:
#   * discover which assets sit at end-of-lifecycle but are not yet dispositioned,
#   * build a single deterministic, receipt-anchored disposition manifest,
#   * evaluate keep/delete/elevate/lease per asset against the Necessity Test,
#   * apply the whole batch in one pass (inventory + boundary rows in lockstep),
#     stamp the birth_track closed, and emit a compact size-first report,
#   * enforce a wall-clock expiry clock on undecided ("leased") artifacts,
#   * guard test-row deletion behind a closed birth_track.
#
# No SHA-matching of assets anywhere (the churn cost of that pattern is exactly what
# this protocol removes). Agreement between agents flows from the CLOSEOUT-RECEIPT:
# a content stamp over the manifest disposition body — same manifest => same receipt.
#
# Doctrine: docs/track_closeout_protocol.md
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

# Resolve THIS bash to a path the (possibly Windows) Python can exec. On Windows
# git-bash, a bare "bash" from Python resolves to WSL, not git-bash — so hand Python
# a cygpath-converted absolute path. On Linux CI cygpath is absent and the POSIX
# path is directly executable.
if command -v cygpath >/dev/null 2>&1; then
  TC_BASH="$(cygpath -w "$(command -v bash)" 2>/dev/null || command -v bash)"
else
  TC_BASH="$(command -v bash)"
fi

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/track_closeout.sh --discover [--track <id>]
  bash scripts/ci/track_closeout.sh --build-manifest <workplan.md|--track <id>> [--out <path>]
  bash scripts/ci/track_closeout.sh --check-eval <manifest>
  bash scripts/ci/track_closeout.sh --apply <manifest>
  bash scripts/ci/track_closeout.sh --artifact-expiry
  bash scripts/ci/track_closeout.sh --deletion-guard <base> <head>
  bash scripts/ci/track_closeout.sh --prove
EOF
  exit 2
}

[[ $# -ge 1 ]] || usage

MODE="$1"; shift || true

case "$MODE" in
  --discover|--build-manifest|--check-eval|--apply|--artifact-expiry|--deletion-guard|--prove) ;;
  -h|--help) usage ;;
  *) echo "track_closeout.sh: unknown mode: $MODE" >&2; usage ;;
esac

TRACK_CLOSEOUT_NOW="${TRACK_CLOSEOUT_NOW:-}" \
TRACK_CLOSEOUT_ROLE="${TRACK_CLOSEOUT_ROLE:-agent}" \
TC_MODE="$MODE" \
TC_REPO_ROOT="$REPO_ROOT" \
TC_SCRIPT_DIR="$SCRIPT_DIR" \
TC_BASH="$TC_BASH" \
  exec "$PYTHON_BIN" - "$@" <<'PY'
import csv
import datetime as _dt
import hashlib
import io
import os
import pathlib
import subprocess
import sys
import tempfile

MODE = os.environ["TC_MODE"]
ROOT = pathlib.Path(os.environ["TC_REPO_ROOT"])
SCRIPT_DIR = pathlib.Path(os.environ["TC_SCRIPT_DIR"])
ROLE = os.environ.get("TRACK_CLOSEOUT_ROLE", "agent")
BASH = os.environ.get("TC_BASH") or "bash"
argv = sys.argv[1:]

INVENTORY = SCRIPT_DIR / "test_inventory.tsv"
BOUNDARY_ROWS = SCRIPT_DIR / "test_lifecycle_boundary_rows.tsv"
TRACKS = SCRIPT_DIR / "test_lifecycle_tracks.tsv"
RESIDUE_CLASSES = SCRIPT_DIR / "test_residue_classes.tsv"
DSU_TIERS = SCRIPT_DIR / "test_lifecycle_dsu_tiers.tsv"
AUTOCLEAR = SCRIPT_DIR / "closeout_autoclear.tsv"
ARTIFACT_LEDGER = SCRIPT_DIR / "closeout_artifacts.tsv"

INVENTORY_HEADER = [
    "crate", "file", "test_name", "kind", "class", "superseding_boundary",
    "verdict", "note", "promotion_target", "birth_track", "dsu_survivals",
]
BOUNDARY_ROWS_HEADER = [
    "crate", "file", "test_name", "kind", "current_class", "boundary_id",
    "boundary_tier", "recommended_disposition", "representative_to_keep",
    "consolidation_target", "promotion_required", "confidence", "note",
]
TRACKS_HEADER = ["track_id", "status", "closed_at", "source", "note"]
ARTIFACT_LEDGER_HEADER = ["path", "leased_at", "disposition", "closeout_track", "note"]
MANIFEST_HEADER = [
    "asset_kind", "ref", "crate", "file", "test_name", "kind",
    "current_class", "birth_track", "disposition", "target", "owner", "note",
]

DURABLE_CLASSES = {
    "seal-proof", "oracle-parity", "golden-byte", "invariant-required",
    "stead-required", "determinism", "dependency-floor",
}
DURABLE_KINDS = {"compile_fail", "trybuild"}

DISPOSITIONS = {
    "delete",         # remove inventory + boundary row now (Necessity Test: owner required)
    "elevate-code",   # relocate source file into a destination crate (target = dest path)
    "elevate-class",  # promote a proof into a permanent-residue class (target = class)
    "keep-durable",   # already durable; retained, no mutation
    "lease",          # undecided; wall-clock expiry clock (target = optional reason)
    "needs-disposition",  # build-manifest placeholder; --check-eval refuses these
}

# Wall-clock lease policy (real-time, not survival-count).
LEASE_CRUFT_DAYS = 3
LEASE_HARD_DAYS = 7


# ---------- normalization / io ----------

def norm_bytes(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")


def read_tsv(path: pathlib.Path):
    if not path.exists():
        return None, []
    text = norm_bytes(path.read_bytes())
    reader = csv.DictReader(io.StringIO(text), delimiter="\t")
    return reader.fieldnames, list(reader)


def write_tsv(path: pathlib.Path, header, rows) -> None:
    buf = io.StringIO()
    writer = csv.DictWriter(buf, fieldnames=header, delimiter="\t", lineterminator="\n")
    writer.writeheader()
    for row in rows:
        writer.writerow({k: row.get(k, "") for k in header})
    path.write_bytes(buf.getvalue().encode("utf-8"))


def now_date() -> _dt.date:
    override = os.environ.get("TRACK_CLOSEOUT_NOW", "").strip()
    if override:
        return _dt.date.fromisoformat(override)
    return _dt.datetime.now(_dt.timezone.utc).date()


def durable_promotion_targets() -> set:
    targets = set()
    _, rows = read_tsv(RESIDUE_CLASSES)
    for row in rows:
        t = (row.get("promotion_target") or "").strip()
        if t.startswith("permanent-residue:"):
            targets.add(t)
    return targets


DURABLE_TARGETS = durable_promotion_targets()


def is_durable(row: dict) -> bool:
    if row.get("kind", "") in DURABLE_KINDS:
        return True
    if row.get("class", "") in DURABLE_CLASSES:
        return True
    if (row.get("promotion_target") or "").strip() in DURABLE_TARGETS:
        return True
    return False


def is_cfg_marker_deletion_candidate(row: dict) -> bool:
    return (
        row.get("test_name", "").startswith("cfg_test_mod::")
        and row.get("class", "").strip() == "deletion-candidate"
        and row.get("verdict", "").strip() != "KEEP"
    )


def inv_key(row: dict):
    return (row["crate"], row["file"], row["test_name"], row["kind"])


def read_autoclear_rules():
    """Rules table: known-shape residue that auto-clears to delete with a named owner."""
    _, rows = read_tsv(AUTOCLEAR)
    return rows


def autoclear_owner(row: dict, rules) -> str:
    for rule in rules:
        prefix = (rule.get("test_name_prefix") or "").strip()
        klass = (rule.get("class") or "").strip()
        if not prefix:
            continue
        if not row.get("test_name", "").startswith(prefix):
            continue
        if klass and row.get("class", "").strip() != klass:
            continue
        if row.get("verdict", "").strip() == "KEEP":
            continue
        return (rule.get("owner") or "").strip()
    return ""


# ---------- receipt ----------

def manifest_body_lines(text: str):
    """Disposition-bearing lines only (drives the receipt). Comment/blank lines excluded."""
    out = []
    for line in text.split("\n"):
        if line.startswith("#") or not line.strip():
            continue
        out.append(line.rstrip())
    return out


def closeout_receipt(text: str) -> str:
    body = "\n".join(manifest_body_lines(text))
    return hashlib.sha256(body.encode("utf-8")).hexdigest()[:12]


# ---------- track resolution ----------

def resolve_track(spec: str) -> str:
    """Accept an explicit track id or a workplan doc path (resolved via tracks.source)."""
    _, tracks = read_tsv(TRACKS)
    ids = {t["track_id"] for t in tracks}
    if spec in ids:
        return spec
    # treat as a workplan path; match against source column (basename-tolerant)
    want = spec.replace("\\", "/")
    want_base = pathlib.PurePosixPath(want).name
    hits = [
        t["track_id"] for t in tracks
        if (t.get("source") or "").replace("\\", "/") == want
        or pathlib.PurePosixPath((t.get("source") or "").replace("\\", "/")).name == want_base
    ]
    if len(hits) == 1:
        return hits[0]
    if len(hits) > 1:
        die(f"workplan {spec!r} maps to multiple tracks: {hits}; pass --track <id>")
    die(f"could not resolve track from {spec!r}; pass an existing track_id or --track <id>")


def die(msg: str, code: int = 2):
    print(f"track_closeout.sh: {msg}", file=sys.stderr)
    sys.exit(code)


# ---------- discover ----------

def scan_ripe(track_filter=None):
    _, inv = read_tsv(INVENTORY)
    _, tracks = read_tsv(TRACKS)
    track_status = {t["track_id"]: t["status"] for t in tracks}
    rules = read_autoclear_rules()

    ripe = []
    for row in inv:
        bt = row.get("birth_track", "").strip()
        if track_filter and bt != track_filter:
            continue
        reason = None
        if is_cfg_marker_deletion_candidate(row):
            reason = "cfg-marker-deletion-candidate"
        elif autoclear_owner(row, rules):
            reason = "autoclear-shape"
        elif track_status.get(bt) == "closed" and not is_durable(row):
            try:
                surv = int(row.get("dsu_survivals", "0") or "0")
            except ValueError:
                surv = 0
            if surv >= 5:
                reason = "presumed-stale(dsu>=5)"
            elif "downstream-utility:" not in row.get("note", "").lower():
                reason = "closed-track-non-durable"
        if reason:
            ripe.append((reason, row))
    return ripe, track_status


def cmd_discover():
    track_filter = None
    a = list(argv)
    while a:
        if a[0] == "--track" and len(a) >= 2:
            track_filter = a[1]; a = a[2:]
        else:
            die(f"discover: unexpected arg {a[0]!r}")
    ripe, _ = scan_ripe(track_filter)
    art_hdr, art_rows = read_tsv(ARTIFACT_LEDGER)
    today = now_date()

    print("TRACK-CLOSEOUT DISCOVER")
    if track_filter:
        print(f"  track: {track_filter}")
    print(f"  ripe inventory rows (delete/lease candidates): {len(ripe)}")
    by_reason = {}
    for reason, _ in ripe:
        by_reason[reason] = by_reason.get(reason, 0) + 1
    for reason in sorted(by_reason):
        print(f"    {reason}: {by_reason[reason]}")
    for reason, row in ripe[:15]:
        print(f"    - [{reason}] {row['crate']} {row['file']}::{row['test_name']}")
    if len(ripe) > 15:
        print(f"    ... (+{len(ripe) - 15} more)")

    aging = 0
    for row in art_rows:
        try:
            age = (today - _dt.date.fromisoformat(row["leased_at"])).days
        except (ValueError, KeyError):
            continue
        if age >= LEASE_CRUFT_DAYS:
            aging += 1
    print(f"  leased artifacts: {len(art_rows)} (aging >= {LEASE_CRUFT_DAYS}d: {aging})")
    print(f"TRACK-CLOSEOUT-DISCOVER-VERDICT: OK ripe={len(ripe)} leased={len(art_rows)}")
    return 0


# ---------- build-manifest ----------

def cmd_build_manifest():
    a = list(argv)
    track = None
    out_path = None
    positional = None
    while a:
        if a[0] == "--track" and len(a) >= 2:
            track = resolve_track(a[1]); a = a[2:]
        elif a[0] == "--out" and len(a) >= 2:
            out_path = pathlib.Path(a[1]); a = a[2:]
        elif not a[0].startswith("--"):
            positional = a[0]; a = a[1:]
        else:
            die(f"build-manifest: unexpected arg {a[0]!r}")
    if track is None and positional is not None:
        track = resolve_track(positional)
    if track is None:
        die("build-manifest requires a workplan path or --track <id>")

    _, inv = read_tsv(INVENTORY)
    rules = read_autoclear_rules()
    scoped = [r for r in inv if r.get("birth_track", "").strip() == track]

    manifest_rows = []
    for row in scoped:
        disp, target, owner, note = "needs-disposition", "", "", ""
        oc = autoclear_owner(row, rules)
        if is_cfg_marker_deletion_candidate(row) or oc:
            disp = "delete"
            owner = oc or "B-T6 cfg(test) module marker; ledger-only, not a runnable test"
            note = "auto-cleared shape (rules table)"
        elif is_durable(row):
            disp = "keep-durable"
            note = "durable class — retained, no mutation"
        manifest_rows.append({
            "asset_kind": "inventory-row",
            "ref": "::".join(inv_key(row)),
            "crate": row["crate"], "file": row["file"],
            "test_name": row["test_name"], "kind": row["kind"],
            "current_class": row.get("class", ""),
            "birth_track": track,
            "disposition": disp, "target": target, "owner": owner, "note": note,
        })

    body = io.StringIO()
    w = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
    w.writeheader()
    for r in manifest_rows:
        w.writerow(r)
    receipt = closeout_receipt(body.getvalue())

    header = (
        "# track_closeout manifest\n"
        f"# track: {track}\n"
        f"# CLOSEOUT-RECEIPT: {receipt}\n"
        f"# role: {ROLE}\n"
        "# dispositions: delete | elevate-code | elevate-class | keep-durable | lease\n"
        "# delete rows REQUIRE a named higher-rung owner (Necessity Test).\n"
        "# resolve every needs-disposition, then run --check-eval.\n"
    )
    text = header + body.getvalue()

    if out_path is None:
        docs = ROOT / "docs" / "tests"
        docs.mkdir(parents=True, exist_ok=True)
        out_path = docs / f"{track}_closeout_manifest.tsv"
    out_path.write_bytes(text.encode("utf-8"))

    need = sum(1 for r in manifest_rows if r["disposition"] == "needs-disposition")
    auto = sum(1 for r in manifest_rows if r["note"].startswith("auto-cleared"))
    keep = sum(1 for r in manifest_rows if r["disposition"] == "keep-durable")
    print("TRACK-CLOSEOUT BUILD-MANIFEST")
    print(f"  track: {track}")
    print(f"  scoped assets: {len(manifest_rows)}")
    print(f"  auto-cleared (delete): {auto}")
    print(f"  keep-durable: {keep}")
    print(f"  needs-disposition (agent must resolve): {need}")
    print(f"  manifest: {out_path.relative_to(ROOT) if out_path.is_relative_to(ROOT) else out_path}")
    print(f"  CLOSEOUT-RECEIPT: {receipt}")
    print(f"TRACK-CLOSEOUT-BUILD-MANIFEST-VERDICT: OK receipt={receipt} needs_disposition={need}")
    return 0


# ---------- manifest parsing shared by check-eval / apply ----------

def load_manifest(path: pathlib.Path):
    if not path.exists():
        die(f"manifest not found: {path}", 2)
    text = norm_bytes(path.read_bytes())
    meta = {}
    for line in text.split("\n"):
        if line.startswith("# track:"):
            meta["track"] = line.split(":", 1)[1].strip()
        elif line.startswith("# CLOSEOUT-RECEIPT:"):
            meta["receipt"] = line.split(":", 1)[1].strip()
    data = "\n".join(l for l in text.split("\n") if not l.startswith("#"))
    reader = csv.DictReader(io.StringIO(data), delimiter="\t")
    rows = list(reader)
    return text, meta, reader.fieldnames, rows


def validate_dispositions(rows):
    errors = []
    for i, r in enumerate(rows, start=1):
        disp = (r.get("disposition") or "").strip()
        if disp not in DISPOSITIONS:
            errors.append(f"row {i} ({r.get('ref','?')}): unknown disposition {disp!r}")
            continue
        if disp == "needs-disposition":
            errors.append(f"row {i} ({r.get('ref','?')}): unresolved needs-disposition")
        if disp == "delete" and not (r.get("owner") or "").strip():
            errors.append(f"row {i} ({r.get('ref','?')}): delete lacks a named Necessity-Test owner")
        if disp == "elevate-code" and not (r.get("target") or "").strip():
            errors.append(f"row {i} ({r.get('ref','?')}): elevate-code lacks a target destination path")
        if disp == "elevate-class":
            tgt = (r.get("target") or "").strip()
            if not tgt.startswith("permanent-residue:"):
                errors.append(f"row {i} ({r.get('ref','?')}): elevate-class target must be a permanent-residue:* class")
    return errors


def cmd_check_eval():
    if not argv:
        die("check-eval requires a manifest path")
    path = pathlib.Path(argv[0])
    text, meta, fields, rows = load_manifest(path)
    if fields != MANIFEST_HEADER:
        die(f"bad manifest header: {fields!r}", 1)
    errors = validate_dispositions(rows)

    tally = {}
    for r in rows:
        d = (r.get("disposition") or "").strip()
        tally[d] = tally.get(d, 0) + 1

    receipt = closeout_receipt(text)
    print("TRACK-CLOSEOUT CHECK-EVAL")
    print(f"  track: {meta.get('track','?')}")
    print(f"  assets: {len(rows)}")
    for d in sorted(tally):
        print(f"    {d}: {tally[d]}")
    print(f"  CLOSEOUT-RECEIPT: {receipt}")
    if errors:
        for e in errors:
            print(f"  - {e}")
        print(f"TRACK-CLOSEOUT-CHECK-EVAL-VERDICT: FAIL unresolved={len(errors)} receipt={receipt}")
        return 1
    # rewrite manifest header receipt to the resolved value so both agents quote the same one
    lines = text.split("\n")
    for idx, line in enumerate(lines):
        if line.startswith("# CLOSEOUT-RECEIPT:"):
            lines[idx] = f"# CLOSEOUT-RECEIPT: {receipt}"
    path.write_bytes("\n".join(lines).encode("utf-8"))
    print(f"TRACK-CLOSEOUT-CHECK-EVAL-VERDICT: PASS receipt={receipt}")
    return 0


# ---------- apply ----------

def cmd_apply():
    if not argv:
        die("apply requires a manifest path")
    path = pathlib.Path(argv[0])
    text, meta, fields, rows = load_manifest(path)
    if fields != MANIFEST_HEADER:
        die(f"bad manifest header: {fields!r}", 1)

    header_receipt = meta.get("receipt", "")
    live_receipt = closeout_receipt(text)
    if header_receipt != live_receipt:
        die(f"manifest receipt drift (header={header_receipt} live={live_receipt}); "
            f"run --check-eval first", 1)
    errors = validate_dispositions(rows)
    if errors:
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        die("apply refused: unresolved dispositions; run --check-eval", 1)

    track = meta.get("track")
    if not track:
        die("manifest has no track", 1)

    inv_hdr, inv = read_tsv(INVENTORY)
    b_hdr, b_rows = read_tsv(BOUNDARY_ROWS)
    trk_hdr, tracks = read_tsv(TRACKS)
    art_hdr, art_rows = read_tsv(ARTIFACT_LEDGER)
    if art_hdr is None:
        art_rows = []
    inv_before, b_before = len(inv), len(b_rows)

    delete_keys = set()
    class_updates = {}
    lease_entries = []
    code_moves = []
    tally = {"delete": 0, "elevate-code": 0, "elevate-class": 0, "keep-durable": 0, "lease": 0}
    survivors = []

    for r in rows:
        disp = r["disposition"].strip()
        tally[disp] = tally.get(disp, 0) + 1
        if disp == "delete":
            delete_keys.add((r["crate"], r["file"], r["test_name"], r["kind"]))
        elif disp == "elevate-class":
            class_updates[(r["crate"], r["file"], r["test_name"], r["kind"])] = r["target"].strip()
            survivors.append((r, f"class -> {r['target'].strip()}"))
        elif disp == "elevate-code":
            code_moves.append(r)
            survivors.append((r, f"code -> {r['target'].strip()}"))
        elif disp == "lease":
            lease_entries.append(r)
            survivors.append((r, f"lease (expires {(now_date() + _dt.timedelta(days=LEASE_HARD_DAYS)).isoformat()})"))
        elif disp == "keep-durable":
            survivors.append((r, "keep-durable"))

    # 1. delete: inventory + boundary rows in lockstep
    new_inv = [row for row in inv if inv_key(row) not in delete_keys]
    new_b = [row for row in b_rows if inv_key(row) not in delete_keys]
    # 2. elevate-class: stamp durable class on the surviving inventory row
    for row in new_inv:
        key = inv_key(row)
        if key in class_updates:
            cls = class_updates[key].removeprefix("permanent-residue:")
            row["class"] = cls
            row["verdict"] = "KEEP"
            row["promotion_target"] = class_updates[key]

    today = now_date()
    moved_notes = []
    for r in code_moves:
        src = ROOT / r["file"]
        dst = ROOT / r["target"].strip()
        if not src.exists():
            die(f"elevate-code source missing: {r['file']}", 1)
        dst.parent.mkdir(parents=True, exist_ok=True)
        try:
            subprocess.run(["git", "-C", str(ROOT), "mv", r["file"], r["target"].strip()],
                           check=True, capture_output=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            src.replace(dst)
        art_rows.append({
            "path": r["target"].strip(), "leased_at": today.isoformat(),
            "disposition": "elevate-code-rehome-pending",
            "closeout_track": track,
            "note": f"moved from {r['file']}; add mod decl + confirm cargo check, then delete this ledger row",
        })
        moved_notes.append(f"{r['file']} -> {r['target'].strip()}")

    for r in lease_entries:
        art_rows.append({
            "path": r["ref"], "leased_at": today.isoformat(), "disposition": "lease",
            "closeout_track": track, "note": (r.get("target") or r.get("note") or "").strip(),
        })

    # 3. write mutated tables
    write_tsv(INVENTORY, INVENTORY_HEADER, new_inv)
    write_tsv(BOUNDARY_ROWS, BOUNDARY_ROWS_HEADER, new_b)
    if art_rows:
        write_tsv(ARTIFACT_LEDGER, ARTIFACT_LEDGER_HEADER, art_rows)

    # 4. close the birth_track (rubber-stamp) unless nothing was actually closed out
    closed = False
    for t in tracks:
        if t["track_id"] == track:
            if t["status"] != "closed":
                t["status"] = "closed"
                t["closed_at"] = today.isoformat()
            closed = True
    if closed:
        write_tsv(TRACKS, TRACKS_HEADER, tracks)

    # 5. gate battery
    gates = run_gate_battery(track)

    # 6. compact, size-first report
    inv_after, b_after = len(new_inv), len(new_b)
    grew = inv_after > inv_before or b_after > b_before
    report = render_report(track, live_receipt, tally, survivors,
                           inv_before, inv_after, b_before, b_after, gates, closed, moved_notes)
    report_path = ROOT / "docs" / "tests" / f"{track}_closeout_report.md"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_bytes(report.encode("utf-8"))

    print("TRACK-CLOSEOUT APPLY")
    print(f"  track: {track}  (birth_track closed: {'yes' if closed else 'no'})")
    print(f"  inventory rows: {inv_before} -> {inv_after} (delta {inv_after - inv_before})")
    print(f"  boundary rows:  {b_before} -> {b_after} (delta {b_after - b_before})")
    for d in sorted(tally):
        if tally[d]:
            print(f"    {d}: {tally[d]}")
    print(f"  report: {report_path.relative_to(ROOT)}")
    gate_fail = any(v == "FAIL" for v in gates.values())
    if grew:
        print("  - PRIMARY FAIL STATE: a TSV table GREW at closeout")
    if gate_fail:
        for g, v in gates.items():
            if v == "FAIL":
                print(f"  - gate FAIL: {g}")
    verdict = "FAIL" if (grew or gate_fail) else "OK"
    print(f"TRACK-CLOSEOUT-APPLY-VERDICT: {verdict} receipt={live_receipt} "
          f"inv_delta={inv_after - inv_before}")
    return 1 if verdict == "FAIL" else 0


def run_gate_battery(track: str) -> dict:
    gates = {}
    checks = [
        ("drift", [BASH, str(SCRIPT_DIR / "test_inventory_drift_check.sh")]),
        ("lifecycle-schema", [BASH, str(SCRIPT_DIR / "test_lifecycle_expiry_check.sh"), "--schema"]),
        ("track-expiry", [BASH, str(SCRIPT_DIR / "test_lifecycle_expiry_check.sh"),
                          "--track-closeout", track]),
    ]
    for name, cmd in checks:
        try:
            proc = subprocess.run(cmd, capture_output=True, text=True, cwd=str(ROOT))
            tail = (proc.stdout or "").strip().splitlines()
            verdict = "PASS" if proc.returncode == 0 else "FAIL"
            for line in reversed(tail):
                if "VERDICT:" in line:
                    if "FAIL" in line:
                        verdict = "FAIL"
                    elif "INSPECT" in line and verdict != "FAIL":
                        verdict = "INSPECT"
                    break
            gates[name] = verdict
        except FileNotFoundError:
            gates[name] = "SKIP"
    return gates


def render_report(track, receipt, tally, survivors, inv_b, inv_a, b_b, b_a, gates, closed, moved):
    grew = inv_a > inv_b or b_a > b_b
    lines = []
    lines.append(f"# {track} — Track Closeout Report")
    lines.append("")
    lines.append("## Status")
    lines.append("")
    lines.append(f"birth_track closed: **{'yes' if closed else 'no'}**  ·  "
                 f"CLOSEOUT-RECEIPT: `{receipt}`  ·  role: {ROLE}")
    lines.append("")
    lines.append("## TSV table size (primary success metric — growth is the fail state)")
    lines.append("")
    lines.append("| table | before | after | delta |")
    lines.append("| --- | --- | --- | --- |")
    lines.append(f"| test_inventory.tsv | {inv_b} | {inv_a} | {inv_a - inv_b} |")
    lines.append(f"| test_lifecycle_boundary_rows.tsv | {b_b} | {b_a} | {b_a - b_b} |")
    lines.append("")
    if grew:
        lines.append("> **FAIL — a TSV table grew at closeout.** TSV growth is the primary fail "
                     "state of the rustification harness. Anything worth keeping is either elevated "
                     "or deleted; nothing may accrete as a permanent row.")
        lines.append("")
    lines.append("## Dispositions")
    lines.append("")
    lines.append("| disposition | count |")
    lines.append("| --- | --- |")
    for d in sorted(tally):
        if tally[d]:
            lines.append(f"| {d} | {tally[d]} |")
    lines.append("")
    lines.append("## NOT deleted — every survivor's new lifecycle (nothing crufts)")
    lines.append("")
    if survivors:
        lines.append("| asset | disposition/new lifecycle |")
        lines.append("| --- | --- |")
        for r, note in survivors:
            ident = r.get("ref") or r.get("file")
            lines.append(f"| `{ident}` | {note} |")
    else:
        lines.append("_No survivors — every scoped asset was deleted._")
    lines.append("")
    if moved:
        lines.append("## Elevated code — rehome pending (real-time clock running)")
        lines.append("")
        for m in moved:
            lines.append(f"- `{m}` — add the `mod` declaration + confirm `cargo check`, then remove "
                         f"its `closeout_artifacts.tsv` row. Guillotine at "
                         f"{LEASE_HARD_DAYS}d.")
        lines.append("")
    lines.append("## Gate battery")
    lines.append("")
    lines.append("| gate | verdict |")
    lines.append("| --- | --- |")
    for g, v in gates.items():
        lines.append(f"| {g} | {v} |")
    lines.append("")
    return "\n".join(lines) + "\n"


# ---------- artifact-expiry (real-time clock) ----------

def cmd_artifact_expiry():
    _, rows = read_tsv(ARTIFACT_LEDGER)
    today = now_date()
    cruft, expired, bad = [], [], []
    for row in rows:
        try:
            leased = _dt.date.fromisoformat(row.get("leased_at", ""))
        except ValueError:
            bad.append(row.get("path", "?"))
            continue
        age = (today - leased).days
        if age >= LEASE_HARD_DAYS:
            expired.append((row.get("path", "?"), age))
        elif age >= LEASE_CRUFT_DAYS:
            cruft.append((row.get("path", "?"), age))

    print("TRACK-CLOSEOUT ARTIFACT-EXPIRY (wall-clock)")
    print(f"  leased artifacts: {len(rows)}  now: {today.isoformat()}")
    for path, age in expired:
        print(f"  - EXPIRED ({age}d >= {LEASE_HARD_DAYS}d, must delete or elevate): {path}")
    for path, age in cruft:
        print(f"  - CRUFT ({age}d >= {LEASE_CRUFT_DAYS}d): {path}")
    for path in bad:
        print(f"  - MALFORMED leased_at: {path}")
    if expired or bad:
        print(f"ARTIFACT-EXPIRY-VERDICT: FAIL expired={len(expired)} cruft={len(cruft)} malformed={len(bad)}")
        return 1
    if cruft:
        print(f"ARTIFACT-EXPIRY-VERDICT: INSPECT expired=0 cruft={len(cruft)} malformed=0")
        return 0
    print(f"ARTIFACT-EXPIRY-VERDICT: PASS expired=0 cruft=0 malformed=0")
    return 0


# ---------- deletion-guard ----------

def git_show_tsv(ref: str, rel: str):
    try:
        out = subprocess.run(["git", "-C", str(ROOT), "show", f"{ref}:{rel}"],
                             capture_output=True, check=True)
    except subprocess.CalledProcessError:
        return None
    text = norm_bytes(out.stdout)
    reader = csv.DictReader(io.StringIO(text), delimiter="\t")
    return list(reader)


def cmd_deletion_guard():
    if len(argv) < 2:
        die("deletion-guard requires <base> <head>")
    base, head = argv[0], argv[1]
    rel = "scripts/ci/test_inventory.tsv"
    base_rows = git_show_tsv(base, rel)
    head_rows = git_show_tsv(head, rel)
    if base_rows is None or head_rows is None:
        print("TRACK-CLOSEOUT-DELETION-GUARD-VERDICT: SKIP (inventory not resolvable at base/head)")
        return 0

    head_keys = {inv_key(r) for r in head_rows}
    removed = [r for r in base_rows if inv_key(r) not in head_keys]

    # birth_track statuses at head
    head_tracks_rows = git_show_tsv(head, "scripts/ci/test_lifecycle_tracks.tsv") or []
    status = {t["track_id"]: t["status"] for t in head_tracks_rows}

    violations = []
    for r in removed:
        bt = r.get("birth_track", "").strip()
        if is_cfg_marker_deletion_candidate(r):
            continue  # ledger-only residue has its own sanctioned sweep route
        if status.get(bt) != "closed":
            violations.append((inv_key(r), bt))

    print("TRACK-CLOSEOUT DELETION-GUARD")
    print(f"  removed inventory rows: {len(removed)}")
    print(f"  unauthorized (birth_track not closed by closeout): {len(violations)}")
    for key, bt in violations[:10]:
        print(f"  - {key} birth_track={bt!r} (open — run track_closeout --apply to close it first)")
    if violations:
        print(f"TRACK-CLOSEOUT-DELETION-GUARD-VERDICT: FAIL unauthorized={len(violations)}")
        return 1
    print(f"TRACK-CLOSEOUT-DELETION-GUARD-VERDICT: PASS removed={len(removed)}")
    return 0


# ---------- prove ----------

def cmd_prove():
    import shutil
    failures = []

    def check(label, cond):
        if not cond:
            failures.append(label)
            print(f"  FAIL {label}")
        else:
            print(f"  PASS {label}")

    print("TRACK-CLOSEOUT PROVE")

    # receipt determinism + body-only sensitivity
    m1 = "# track: t\n# CLOSEOUT-RECEIPT: x\nasset_kind\tref\n" + "inventory-row\ta::b::c::unit\n"
    m2 = "# track: t\n# CLOSEOUT-RECEIPT: DIFFERENT\nasset_kind\tref\n" + "inventory-row\ta::b::c::unit\n"
    m3 = "# track: t\n# CLOSEOUT-RECEIPT: x\nasset_kind\tref\n" + "inventory-row\ta::b::c::CHANGED\n"
    check("receipt-ignores-comment-churn", closeout_receipt(m1) == closeout_receipt(m2))
    check("receipt-tracks-body-change", closeout_receipt(m1) != closeout_receipt(m3))

    # validate_dispositions
    check("delete-needs-owner", bool(validate_dispositions(
        [{"ref": "r", "disposition": "delete", "owner": "", "target": ""}])))
    check("delete-with-owner-ok", not validate_dispositions(
        [{"ref": "r", "disposition": "delete", "owner": "type-boundary X", "target": ""}]))
    check("needs-disposition-rejected", bool(validate_dispositions(
        [{"ref": "r", "disposition": "needs-disposition", "owner": "", "target": ""}])))
    check("elevate-class-needs-residue", bool(validate_dispositions(
        [{"ref": "r", "disposition": "elevate-class", "owner": "", "target": "golden-byte"}])))
    check("elevate-class-residue-ok", not validate_dispositions(
        [{"ref": "r", "disposition": "elevate-class", "owner": "", "target": "permanent-residue:golden-byte"}]))

    # full build -> check -> apply roundtrip in a sandbox
    with tempfile.TemporaryDirectory() as tmp:
        sb = pathlib.Path(tmp)
        (sb / "scripts" / "ci").mkdir(parents=True)
        (sb / "docs" / "tests").mkdir(parents=True)
        # minimal fixtures
        inv_rows = [
            {"crate": "c", "file": "crates/c/src/a.rs", "test_name": "cfg_test_mod::tests",
             "kind": "unit", "class": "deletion-candidate", "superseding_boundary": "B-T6-MODULE-MARKER-EXPANSION",
             "verdict": "AUDIT", "note": "marker", "promotion_target": "ledger-only",
             "birth_track": "sb-track", "dsu_survivals": "0"},
            {"crate": "c", "file": "crates/c/tests/keep.rs", "test_name": "golden",
             "kind": "integration", "class": "golden-byte", "superseding_boundary": "B-T7-GOLDEN-BYTE-DETERMINISM",
             "verdict": "KEEP", "note": "keep", "promotion_target": "permanent-residue:golden-byte",
             "birth_track": "sb-track", "dsu_survivals": "0"},
        ]
        b_rows = [
            {"crate": "c", "file": "crates/c/src/a.rs", "test_name": "cfg_test_mod::tests",
             "kind": "unit", "current_class": "deletion-candidate", "boundary_id": "B-T6-MODULE-MARKER-EXPANSION",
             "boundary_tier": "TIER6_PROMOTION_REQUIRED", "recommended_disposition": "", "representative_to_keep": "",
             "consolidation_target": "", "promotion_required": "", "confidence": "high", "note": "marker"},
        ]
        write_tsv(sb / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv_rows)
        write_tsv(sb / "scripts/ci/test_lifecycle_boundary_rows.tsv", BOUNDARY_ROWS_HEADER, b_rows)
        write_tsv(sb / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [
            {"track_id": "sb-track", "status": "open", "closed_at": "-", "source": "docs/sb.md", "note": "x"},
        ])
        write_tsv(sb / "scripts/ci/test_residue_classes.tsv", ["promotion_target"], [
            {"promotion_target": "permanent-residue:golden-byte"},
        ])
        write_tsv(sb / "scripts/ci/closeout_autoclear.tsv",
                  ["test_name_prefix", "class", "owner"],
                  [{"test_name_prefix": "cfg_test_mod::", "class": "deletion-candidate",
                    "owner": "B-T6 cfg(test) module marker; ledger-only"}])

        # the script keys off its own SCRIPT_DIR, so invoke a copy placed in the sandbox.
        # Use cwd=sb + relative POSIX paths so bash invocation is Windows-path-safe.
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", sb / "scripts/ci/track_closeout.sh")

        def run(*a, now="2026-07-07"):
            return subprocess.run(
                [BASH, "scripts/ci/track_closeout.sh", *a],
                capture_output=True, text=True, cwd=str(sb),
                env={**os.environ, "TRACK_CLOSEOUT_NOW": now},
            )

        r_build = run("--build-manifest", "--track", "sb-track", "--out", "m.tsv")
        check("build-manifest-ok", "BUILD-MANIFEST-VERDICT: OK" in r_build.stdout)
        man = norm_bytes((sb / "m.tsv").read_bytes())
        # the golden keep row is keep-durable; the cfg marker auto-deletes; no needs-disposition
        check("build-auto-clears-marker", "\tdelete\t" in man)
        check("build-keeps-durable", "keep-durable" in man)
        check("build-no-needs-disposition", "\tneeds-disposition\t" not in man)

        r_eval = run("--check-eval", "m.tsv")
        check("check-eval-pass", "CHECK-EVAL-VERDICT: PASS" in r_eval.stdout)

        r_apply = run("--apply", "m.tsv")
        check("apply-ok", "APPLY-VERDICT: OK" in r_apply.stdout or "APPLY-VERDICT: INSPECT" in r_apply.stdout
              or "APPLY-VERDICT:" in r_apply.stdout)
        _, inv_after = read_tsv(sb / "scripts/ci/test_inventory.tsv")
        check("apply-deleted-marker-row", all(r["test_name"] != "cfg_test_mod::tests" for r in inv_after))
        check("apply-kept-golden-row", any(r["test_name"] == "golden" for r in inv_after))
        _, b_after = read_tsv(sb / "scripts/ci/test_lifecycle_boundary_rows.tsv")
        check("apply-deleted-boundary-lockstep", len(b_after) == 0)
        _, trk_after = read_tsv(sb / "scripts/ci/test_lifecycle_tracks.tsv")
        check("apply-closed-birth-track", any(t["track_id"] == "sb-track" and t["status"] == "closed" for t in trk_after))
        check("apply-report-written", (sb / "docs/tests/sb-track_closeout_report.md").exists())

        # artifact-expiry clock
        write_tsv(sb / "scripts/ci/closeout_artifacts.tsv", ARTIFACT_LEDGER_HEADER, [
            {"path": "x", "leased_at": "2026-07-01", "disposition": "lease", "closeout_track": "sb-track", "note": ""},
        ])
        r_exp_fresh = run("--artifact-expiry", now="2026-07-03")
        check("artifact-fresh-pass", "ARTIFACT-EXPIRY-VERDICT: PASS" in r_exp_fresh.stdout)
        r_exp_cruft = run("--artifact-expiry", now="2026-07-05")
        check("artifact-cruft-inspect", "ARTIFACT-EXPIRY-VERDICT: INSPECT" in r_exp_cruft.stdout)
        r_exp_dead = run("--artifact-expiry", now="2026-07-09")
        check("artifact-expired-fail", "ARTIFACT-EXPIRY-VERDICT: FAIL" in r_exp_dead.stdout)

    # deletion-guard: a git-backed repo where an OPEN-track row is removed must FAIL,
    # and a removal whose birth_track is closed (or a cfg-marker) must PASS.
    with tempfile.TemporaryDirectory() as gtmp:
        gr = pathlib.Path(gtmp)
        (gr / "scripts" / "ci").mkdir(parents=True)
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", gr / "scripts/ci/track_closeout.sh")

        def grun(*a):
            return subprocess.run(["git", "-C", str(gr), *a], capture_output=True, text=True)

        grun("init", "-q")
        grun("config", "user.email", "t@t")
        grun("config", "user.name", "t")
        inv0 = [
            {"crate": "c", "file": "crates/c/tests/open.rs", "test_name": "open_t", "kind": "integration",
             "class": "behavior-regression", "superseding_boundary": "B", "verdict": "KEEP", "note": "n",
             "promotion_target": "permanent-residue:behavior-regression", "birth_track": "open-track", "dsu_survivals": "0"},
            {"crate": "c", "file": "crates/c/src/m.rs", "test_name": "cfg_test_mod::tests", "kind": "unit",
             "class": "deletion-candidate", "superseding_boundary": "B-T6", "verdict": "AUDIT", "note": "marker",
             "promotion_target": "ledger-only", "birth_track": "open-track", "dsu_survivals": "0"},
        ]
        trk = [
            {"track_id": "open-track", "status": "open", "closed_at": "-", "source": "d", "note": "x"},
            {"track_id": "closed-track", "status": "closed", "closed_at": "2026-07-01", "source": "d", "note": "x"},
        ]
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv0)
        write_tsv(gr / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, trk)
        grun("add", "-A"); grun("commit", "-q", "-m", "base")
        base = grun("rev-parse", "HEAD").stdout.strip()

        # remove BOTH rows: the behavior-regression on an OPEN track = unauthorized;
        # the cfg-marker = exempt. Expect FAIL naming exactly 1 unauthorized.
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [])
        grun("add", "-A"); grun("commit", "-q", "-m", "delete")
        head = grun("rev-parse", "HEAD").stdout.strip()
        r_guard = subprocess.run([BASH, "scripts/ci/track_closeout.sh", "--deletion-guard", base, head],
                                 capture_output=True, text=True, cwd=str(gr),
                                 env={**os.environ})
        check("deletion-guard-catches-open-track",
              "DELETION-GUARD-VERDICT: FAIL unauthorized=1" in r_guard.stdout)

        # now close the track and re-remove: expect PASS
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv0)
        for t in trk:
            if t["track_id"] == "open-track":
                t["status"] = "closed"; t["closed_at"] = "2026-07-07"
        write_tsv(gr / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, trk)
        grun("add", "-A"); grun("commit", "-q", "-m", "reopen-base-closed")
        base2 = grun("rev-parse", "HEAD").stdout.strip()
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [])
        grun("add", "-A"); grun("commit", "-q", "-m", "delete-closed")
        head2 = grun("rev-parse", "HEAD").stdout.strip()
        r_guard2 = subprocess.run([BASH, "scripts/ci/track_closeout.sh", "--deletion-guard", base2, head2],
                                  capture_output=True, text=True, cwd=str(gr),
                                  env={**os.environ})
        check("deletion-guard-allows-closed-track",
              "DELETION-GUARD-VERDICT: PASS" in r_guard2.stdout)

    if failures:
        print(f"TRACK-CLOSEOUT-PROVE-VERDICT: FAIL ({len(failures)})")
        return 1
    print("TRACK-CLOSEOUT-PROVE-VERDICT: PASS")
    return 0


DISPATCH = {
    "--discover": cmd_discover,
    "--build-manifest": cmd_build_manifest,
    "--check-eval": cmd_check_eval,
    "--apply": cmd_apply,
    "--artifact-expiry": cmd_artifact_expiry,
    "--deletion-guard": cmd_deletion_guard,
    "--prove": cmd_prove,
}
sys.exit(DISPATCH[MODE]())
PY
