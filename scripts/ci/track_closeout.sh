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
  bash scripts/ci/track_closeout.sh --build-manifest <workplan.md|--track <id>> [--out <path>] [--docs <glob>]...
  bash scripts/ci/track_closeout.sh --check-eval <manifest>
  bash scripts/ci/track_closeout.sh --apply <manifest>
  bash scripts/ci/track_closeout.sh --artifact-expiry
  bash scripts/ci/track_closeout.sh --decommission [--dry-run] [--all]
  bash scripts/ci/track_closeout.sh --deletion-guard <base> <head>
  bash scripts/ci/track_closeout.sh --prove
EOF
  exit 2
}

[[ $# -ge 1 ]] || usage

MODE="$1"; shift || true

case "$MODE" in
  --discover|--build-manifest|--check-eval|--apply|--artifact-expiry|--decommission|--deletion-guard|--prove) ;;
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
import re
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
PARKED = SCRIPT_DIR / "test_lifecycle_parked.tsv"
ACTIVE_TRACK = SCRIPT_DIR / "active_track.txt"
NO_ACTIVE_TRACK = "none"
ACTIVE_TRACK_COMMENT = "# Active track design doc for orientation Next-Rung pointer. Update on track open/close."
PARK_BEGIN = "<!-- SIMTHING-PARKED-TRACK:BEGIN agents: read only when executing --unpark -->"
PARK_END = "<!-- SIMTHING-PARKED-TRACK:END -->"

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
# A parked row is a full inventory row relocated OUT of the live inventory into the
# quarantine pen so test_inventory.tsv only ever holds decided assets.
PARKED_HEADER = INVENTORY_HEADER + ["parked_at", "closeout_track", "park_reason"]
# Legacy (pre-HU-INVENTORY-ONEWRITE-0): if a repo still carries a boundary-row table,
# park/lockstep preserves matching rows. Absent table is the normal post-retirement state.
PARKED_BOUNDARY = SCRIPT_DIR / "test_lifecycle_parked_boundary.tsv"
PARKED_BOUNDARY_HEADER = BOUNDARY_ROWS_HEADER + ["parked_at", "closeout_track"]
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
    "delete",         # remove inventory row now (Necessity Test: owner required)
    "elevate-code",   # relocate source file into a destination crate (target = dest path)
    "elevate-class",  # promote a proof into a permanent-residue class (target = class)
    "keep-durable",   # already durable; retained, no mutation
    "lease",          # undecided; row is PARKED out of the live tables into the pen,
                      # on a wall-clock clock (target = optional reason)
    "needs-disposition",  # build-manifest placeholder; --check-eval refuses these
}

# Wall-clock lease policy (real-time, not survival-count).
LEASE_CRUFT_DAYS = 3
LEASE_HARD_DAYS = 7

DOC_EXTENSIONS = {".md", ".tsv"}
DOC_ARCHIVE_PREFIX = "docs/archive/"
REAPABLE_DOC_SUFFIXES = (
    "_results.md",
    "_review.tsv",
    "_manifest.tsv",
    "_closeout_manifest.tsv",
)


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


def clean_repo_relpath(path: str) -> str:
    rel = (path or "").replace("\\", "/").strip()
    if not rel or rel.startswith("/") or ":" in rel:
        return ""
    parts = pathlib.PurePosixPath(rel).parts
    if not parts or any(p in ("", ".", "..") for p in parts):
        return ""
    return rel


def is_doc_path(path: str) -> bool:
    rel = clean_repo_relpath(path)
    return (
        bool(rel)
        and rel.startswith("docs/")
        and pathlib.PurePosixPath(rel).suffix in DOC_EXTENSIONS
    )


def is_reapable_doc(path: str) -> bool:
    """Only narrow result/review/manifest artifacts are safe for automatic reaping."""
    rel = clean_repo_relpath(path)
    if not (is_doc_path(rel) and rel.startswith("docs/tests/")):
        return False
    name = pathlib.PurePosixPath(rel).name
    if name.endswith("_closeout_report.md"):
        return False
    return any(name.endswith(suffix) for suffix in REAPABLE_DOC_SUFFIXES)


def is_archive_doc_target(path: str) -> bool:
    rel = clean_repo_relpath(path)
    return is_doc_path(rel) and rel.startswith(DOC_ARCHIVE_PREFIX)


def track_doc_prefixes(track: str, source: str = "") -> list:
    def words_from(value: str) -> list:
        value = value.lower().replace("\\", "/")
        stem = pathlib.PurePosixPath(value).stem
        for prefix in ("design_", "docs_", "test_", "tests_"):
            if stem.startswith(prefix):
                stem = stem[len(prefix):]
        stem = stem.replace(".", "_").replace("-", "_")
        return [w for w in stem.split("_") if w and not w.isdigit()]

    words = words_from(track)
    if not words and source:
        words = words_from(source)
    prefixes = []
    if words:
        acronym = "".join(w[0] for w in words if w)
        if len(acronym) >= 2:
            prefixes.extend([f"{acronym}_", f"{acronym}-"])
        if len(words[0]) <= 4:
            prefixes.extend([f"{words[0]}_", f"{words[0]}-"])
        prefixes.append("_".join(words) + "_")
    seen = set()
    out = []
    for p in prefixes:
        if p not in seen:
            seen.add(p)
            out.append(p)
    return out


def discover_track_docs(track: str, source: str) -> list:
    prefixes = track_doc_prefixes(track, source)
    if not prefixes:
        return []
    tests_dir = ROOT / "docs" / "tests"
    if not tests_dir.exists():
        return []
    out = []
    for path in sorted(tests_dir.iterdir()):
        if not path.is_file():
            continue
        rel = path.relative_to(ROOT).as_posix()
        name = path.name.lower()
        if name.endswith("_closeout_manifest.tsv"):
            continue
        if any(name.startswith(p) for p in prefixes) and is_reapable_doc(rel):
            out.append(rel)
    return out


def auto_doc_scope(track: str, track_rows: list) -> set:
    source = next((t.get("source", "") for t in track_rows if t["track_id"] == track), "")
    doc_paths = set()
    if source and (ROOT / source).is_file() and not doc_validation_error(source):
        doc_paths.add(clean_repo_relpath(source))
    doc_paths.update(discover_track_docs(track, source))
    return doc_paths


def read_active_track_pointer() -> dict:
    if not ACTIVE_TRACK.exists():
        return {"exists": False, "path": "", "raw": "", "reason": "missing"}
    text = norm_bytes(ACTIVE_TRACK.read_bytes())
    for line in text.splitlines():
        raw = line.strip()
        if not raw or raw.startswith("#"):
            continue
        if raw == NO_ACTIVE_TRACK:
            return {
                "exists": True,
                "path": NO_ACTIVE_TRACK,
                "raw": raw,
                "reason": "no-active-track",
            }
        rel = clean_repo_relpath(raw)
        return {
            "exists": True,
            "path": rel,
            "raw": raw,
            "reason": "" if rel else "invalid-path",
        }
    return {"exists": True, "path": "", "raw": "", "reason": "empty"}


def active_track_validation_error(info: dict) -> str:
    if not info.get("exists"):
        return ""
    rel = info.get("path", "")
    if rel == NO_ACTIVE_TRACK:
        return ""
    if not rel:
        return info.get("reason") or "empty"
    if not rel.startswith("docs/"):
        return "not-under-docs"
    if pathlib.PurePosixPath(rel).suffix != ".md":
        return "not-markdown"
    if not (ROOT / rel).is_file():
        return "missing-target"
    return ""


def active_track_leading_comment() -> list:
    if ACTIVE_TRACK.exists():
        lines = norm_bytes(ACTIVE_TRACK.read_bytes()).splitlines()
        comments = []
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("#"):
                comments.append(line.rstrip())
                continue
            if not stripped:
                continue
            break
        if comments:
            return comments
    return [ACTIVE_TRACK_COMMENT]


def write_active_track_pointer(rel: str) -> None:
    lines = active_track_leading_comment() + [rel]
    ACTIVE_TRACK.write_bytes(("\n".join(lines) + "\n").encode("utf-8"))


def plan_active_track_retirement(track: str, track_source: str, rows: list, live_doc_paths: set) -> dict:
    info = read_active_track_pointer()
    result = {
        "retired": "no",
        "current": info.get("path", "") or info.get("raw", ""),
        "from": "",
        "to": "",
        "reason": info.get("reason", ""),
    }
    err = active_track_validation_error(info)
    if err:
        result["reason"] = err
        return result
    if not info.get("exists"):
        result["reason"] = "missing"
        return result
    if info.get("path") == NO_ACTIVE_TRACK:
        result["reason"] = "no-active-track"
        return result

    owned = set()
    source_rel = clean_repo_relpath(track_source)
    if source_rel:
        owned.add(source_rel)
    owned.update(live_doc_paths)
    owned.update(
        clean_repo_relpath(r.get("file", ""))
        for r in rows
        if (r.get("asset_kind") or "").strip() == "doc"
    )
    owned.discard("")

    current = info.get("path", "")
    if current in owned:
        result.update({
            "retired": "yes",
            "from": current,
            "to": NO_ACTIVE_TRACK,
            "reason": "owned-by-closing-track",
        })
    else:
        result["reason"] = "current pointer not owned by closing track"
    return result


def apply_active_track_retirement(plan: dict) -> None:
    if plan.get("retired") != "yes":
        return
    target = plan.get("to", NO_ACTIVE_TRACK)
    if target != NO_ACTIVE_TRACK:
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(harness-error) active_track_target_invalid={target!r}",
              file=sys.stderr)
        sys.exit(1)
    write_active_track_pointer(target)
    orient = SCRIPT_DIR / "gen_orientation.sh"
    if not orient.exists():
        print("TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(harness-error) active_track_orientation_regen=missing",
              file=sys.stderr)
        sys.exit(1)
    proc = subprocess.run([BASH, str(orient)], cwd=str(ROOT), capture_output=True, text=True)
    if proc.returncode != 0:
        if proc.stdout:
            print(proc.stdout, file=sys.stderr)
        if proc.stderr:
            print(proc.stderr, file=sys.stderr)
        print("TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(harness-error) active_track_orientation_regen=FAIL",
              file=sys.stderr)
        sys.exit(1)


def is_auto_doc_candidate(track: str, source: str, path: str) -> bool:
    rel = clean_repo_relpath(path)
    if not is_doc_path(rel):
        return False
    if source and rel == clean_repo_relpath(source):
        return True
    if not (rel.startswith("docs/tests/") and is_reapable_doc(rel)):
        return False
    name = pathlib.PurePosixPath(rel).name.lower()
    return any(name.startswith(p) for p in track_doc_prefixes(track, source))


def doc_validation_error(path: str) -> str:
    rel = clean_repo_relpath(path)
    if not rel:
        return "path must be repo-relative and may not contain drive letters or '..'"
    if not rel.startswith("docs/"):
        return "path must live under docs/"
    if pathlib.PurePosixPath(rel).suffix not in DOC_EXTENSIONS:
        return "path must be a .md or .tsv document"
    return ""


def split_park_block_text(text: str):
    begin_count = text.count(PARK_BEGIN)
    end_count = text.count(PARK_END)
    if begin_count == 0 and end_count == 0:
        return text, False
    if begin_count != 1 or end_count != 1:
        die("track doc has malformed parked block; run gen_orientation.sh --unpark/--park repair", 1)
    begin = text.index(PARK_BEGIN)
    end = text.index(PARK_END) + len(PARK_END)
    suffix = text[end:]
    if suffix not in ("", "\n"):
        die("track doc parked block is not absolute EOF; run gen_orientation.sh --park repair", 1)
    return text[:begin].rstrip() + "\n", True


def status_header_is_parked(text: str) -> bool:
    for line in text.splitlines()[:80]:
        stripped = line.strip().lstrip(">").strip()
        m = re.match(r"\**status\s*:\s*([A-Za-z0-9 +/\-]+)", stripped, re.IGNORECASE)
        if m:
            return bool(re.search(r"\bPARKED\b", m.group(1), re.IGNORECASE))
    return False


def doc_manifest_row(track: str, rel_path: str, disp: str = "needs-disposition",
                     note: str = "") -> dict:
    return {
        "asset_kind": "doc",
        "ref": f"doc::{rel_path}",
        "crate": "-", "file": rel_path,
        "test_name": pathlib.PurePosixPath(rel_path).name, "kind": "doc",
        "current_class": "-", "birth_track": track,
        "disposition": disp, "target": "", "owner": "", "note": note,
    }


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
    _, parked_rows = read_tsv(PARKED)
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

    def aging_count(rows, field):
        n = 0
        for row in rows:
            try:
                if (today - _dt.date.fromisoformat(row[field])).days >= LEASE_CRUFT_DAYS:
                    n += 1
            except (ValueError, KeyError):
                continue
        return n

    art_aging = aging_count(art_rows, "leased_at")
    park_aging = aging_count(parked_rows, "parked_at")
    print(f"  leased artifacts: {len(art_rows)} (aging >= {LEASE_CRUFT_DAYS}d: {art_aging})")
    print(f"  parked rows (pen): {len(parked_rows)} (aging >= {LEASE_CRUFT_DAYS}d: {park_aging})")
    print(f"TRACK-CLOSEOUT-DISCOVER-VERDICT: OK ripe={len(ripe)} "
          f"leased={len(art_rows)} parked={len(parked_rows)}")
    return 0


# ---------- build-manifest ----------

def cmd_build_manifest():
    a = list(argv)
    track = None
    out_path = None
    positional = None
    doc_globs = []
    while a:
        if a[0] == "--track" and len(a) >= 2:
            track = resolve_track(a[1]); a = a[2:]
        elif a[0] == "--out" and len(a) >= 2:
            out_path = pathlib.Path(a[1]); a = a[2:]
        elif a[0] == "--docs" and len(a) >= 2:
            doc_globs.append(a[1]); a = a[2:]
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

    # P1-4: docs are scoped assets too — result docs and the design doc are exactly
    # what clutters low-context agents. The track's design doc (tracks.source) is
    # auto-included; rung result docs come in via --docs globs.
    _, track_rows = read_tsv(TRACKS)
    doc_paths = sorted(auto_doc_scope(track, track_rows))
    for g in doc_globs:
        hits = sorted(p.relative_to(ROOT).as_posix() for p in ROOT.glob(g) if p.is_file())
        if not hits:
            die(f"--docs glob matched nothing: {g}")
        for h in hits:
            err = doc_validation_error(h)
            if err:
                die(f"--docs matched unsupported document {h!r}: {err}")
            if h not in doc_paths:
                doc_paths.append(h)

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
    for dp in doc_paths:
        manifest_rows.append(doc_manifest_row(track, dp))

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
        "# doc rows: delete=git rm | elevate-code=git mv to target (e.g. docs/archive/...) |\n"
        "#   lease=wall-clock ledger entry | keep-durable=stays. elevate-class is invalid for docs.\n"
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
    print(f"  scoped assets: {len(manifest_rows)} (docs: {len(doc_paths)})")
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
        asset_kind = (r.get("asset_kind") or "inventory-row").strip()
        if asset_kind not in {"inventory-row", "doc"}:
            errors.append(f"row {i} ({r.get('ref','?')}): unknown asset_kind {asset_kind!r}")
            continue
        disp = (r.get("disposition") or "").strip()
        if disp not in DISPOSITIONS:
            errors.append(f"row {i} ({r.get('ref','?')}): unknown disposition {disp!r}")
            continue
        if disp == "needs-disposition":
            errors.append(f"row {i} ({r.get('ref','?')}): unresolved needs-disposition")
        if disp == "delete" and not (r.get("owner") or "").strip():
            errors.append(f"row {i} ({r.get('ref','?')}): delete lacks a named Necessity-Test owner")
        if asset_kind == "doc":
            doc_path = clean_repo_relpath(r.get("file", ""))
            doc_err = doc_validation_error(doc_path)
            if doc_err:
                errors.append(f"row {i} ({r.get('ref','?')}): doc {doc_err}")
                continue
            if disp in {"delete", "lease"} and not is_reapable_doc(doc_path):
                errors.append(f"row {i} ({r.get('ref','?')}): {disp} is only allowed for "
                              f"docs/tests result/review/manifest artifacts; keep or archive this doc")
            if disp == "elevate-code":
                target = clean_repo_relpath(r.get("target", ""))
                if not is_archive_doc_target(target):
                    errors.append(f"row {i} ({r.get('ref','?')}): doc elevate-code target must be "
                                  f"a .md/.tsv path under {DOC_ARCHIVE_PREFIX}")
                elif target == doc_path:
                    errors.append(f"row {i} ({r.get('ref','?')}): doc elevate-code target matches source")
            if disp == "elevate-class":
                errors.append(f"row {i} ({r.get('ref','?')}): elevate-class is invalid for doc rows "
                              f"(use elevate-code with an archive target)")
            continue
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
    # Boundary audit ledger retired (HU-INVENTORY-ONEWRITE-0). Tolerate absence;
    # if a legacy table is present, keep park/lockstep correct without recreating it.
    boundary_present = BOUNDARY_ROWS.exists()
    b_hdr, b_rows = read_tsv(BOUNDARY_ROWS)
    if not boundary_present:
        b_rows = []
    trk_hdr, tracks = read_tsv(TRACKS)

    # P0-3: the birth_track close-stamp is the rubber-stamp everything downstream
    # keys on; an unknown track must be a hard harness-error, never a silent no-stamp.
    if not any(t.get("track_id") == track for t in tracks):
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(harness-error) unknown track {track!r} "
              f"in {TRACKS.name}", file=sys.stderr)
        sys.exit(1)

    # P0-1: scope-freshness — the manifest must cover exactly the live rows carrying
    # this birth_track. A row born or removed between --build-manifest and --apply
    # would otherwise be silently undisposed under a closed track. Set comparison,
    # no SHA-matching.
    live_keys = {inv_key(r) for r in inv if r.get("birth_track", "").strip() == track}
    manifest_keys = {
        (r["crate"], r["file"], r["test_name"], r["kind"])
        for r in rows if r.get("asset_kind", "inventory-row") == "inventory-row"
    }
    unscoped = live_keys - manifest_keys
    vanished = manifest_keys - live_keys
    if unscoped or vanished:
        for k in sorted(unscoped)[:10]:
            print(f"  - live row not in manifest: {k}", file=sys.stderr)
        for k in sorted(vanished)[:10]:
            print(f"  - manifest row no longer live: {k}", file=sys.stderr)
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(stale-manifest) unscoped={len(unscoped)} "
              f"vanished={len(vanished)}; re-run --build-manifest", file=sys.stderr)
        sys.exit(1)

    # P0-1 also covers docs. Auto scope catches source docs plus track-shaped
    # docs/tests artifacts; explicit --docs rows outside that shape must still
    # exist at apply time so delete/lease cannot silently no-op on a missing path.
    track_source = next((t.get("source", "") for t in tracks if t["track_id"] == track), "")
    strip_park_block_on_close = None
    source_rel = clean_repo_relpath(track_source)
    if source_rel and (ROOT / source_rel).is_file():
        source_text = norm_bytes((ROOT / source_rel).read_bytes())
        source_without_park, has_park_block = split_park_block_text(source_text)
        if has_park_block and status_header_is_parked(source_text):
            print("TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(track-parked-unpark-first) "
                  f"track={track} source={source_rel}; run `bash scripts/ci/gen_orientation.sh --unpark {source_rel}` first",
                  file=sys.stderr)
            sys.exit(1)
        if has_park_block:
            strip_park_block_on_close = (ROOT / source_rel, source_without_park)
    live_doc_paths = auto_doc_scope(track, tracks)
    manifest_doc_paths = {
        clean_repo_relpath(r.get("file", ""))
        for r in rows
        if (r.get("asset_kind") or "").strip() == "doc"
        and is_auto_doc_candidate(track, track_source, r.get("file", ""))
    }
    doc_unscoped = live_doc_paths - manifest_doc_paths
    doc_vanished = manifest_doc_paths - live_doc_paths
    if doc_unscoped or doc_vanished:
        for p in sorted(doc_unscoped)[:10]:
            print(f"  - live doc not in manifest: {p}", file=sys.stderr)
        for p in sorted(doc_vanished)[:10]:
            print(f"  - manifest doc no longer live: {p}", file=sys.stderr)
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(stale-manifest) doc_unscoped={len(doc_unscoped)} "
              f"doc_vanished={len(doc_vanished)}; re-run --build-manifest", file=sys.stderr)
        sys.exit(1)
    explicit_doc_vanished = {
        clean_repo_relpath(r.get("file", ""))
        for r in rows
        if (r.get("asset_kind") or "").strip() == "doc"
        and not is_auto_doc_candidate(track, track_source, r.get("file", ""))
        and not (ROOT / clean_repo_relpath(r.get("file", ""))).exists()
    }
    if explicit_doc_vanished:
        for p in sorted(explicit_doc_vanished)[:10]:
            print(f"  - explicit manifest doc missing: {p}", file=sys.stderr)
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(stale-manifest) "
              f"explicit_doc_vanished={len(explicit_doc_vanished)}; re-run --build-manifest",
              file=sys.stderr)
        sys.exit(1)

    active_info = read_active_track_pointer()
    active_err = active_track_validation_error(active_info)
    if active_err:
        print(f"TRACK-CLOSEOUT-APPLY-VERDICT: FAIL(harness-error) "
              f"active_track_reason={active_err}", file=sys.stderr)
        sys.exit(1)
    active_track_plan = plan_active_track_retirement(track, track_source, rows, live_doc_paths)

    art_hdr, art_rows = read_tsv(ARTIFACT_LEDGER)
    if art_hdr is None:
        art_rows = []
    parked_hdr, parked_rows = read_tsv(PARKED)
    if parked_hdr is None:
        parked_rows = []
    inv_before, b_before = len(inv), len(b_rows)
    inv_by_key = {inv_key(row): row for row in inv}

    delete_keys = set()
    park_keys = set()
    park_reason = {}
    class_updates = {}
    code_moves = []
    doc_deletes, doc_moves, doc_leases = [], [], []
    tally = {"delete": 0, "elevate-code": 0, "elevate-class": 0, "keep-durable": 0, "lease": 0}
    survivors = []

    today = now_date()
    wall = (today + _dt.timedelta(days=LEASE_HARD_DAYS)).isoformat()
    for r in rows:
        disp = r["disposition"].strip()
        tally[disp] = tally.get(disp, 0) + 1
        if (r.get("asset_kind") or "").strip() == "doc":
            doc_path = clean_repo_relpath(r["file"])
            # P1-4: docs follow the same law — delete, relocate (archive), lease, or stay
            if disp == "delete":
                doc_deletes.append(doc_path)
            elif disp == "elevate-code":
                r["file"] = doc_path
                r["target"] = clean_repo_relpath(r["target"])
                doc_moves.append(r)
                survivors.append((r, f"moved -> {r['target'].strip()}"))
            elif disp == "lease":
                r["file"] = doc_path
                doc_leases.append(r)
                survivors.append((r, f"lease (ledger, delete/relocate by {wall})"))
            elif disp == "keep-durable":
                r["file"] = doc_path
                survivors.append((r, "keep-durable"))
            continue
        if disp == "delete":
            delete_keys.add((r["crate"], r["file"], r["test_name"], r["kind"]))
        elif disp == "elevate-class":
            class_updates[(r["crate"], r["file"], r["test_name"], r["kind"])] = r["target"].strip()
            survivors.append((r, f"class -> {r['target'].strip()}"))
        elif disp == "elevate-code":
            code_moves.append(r)
            survivors.append((r, f"code -> {r['target'].strip()}"))
        elif disp == "lease":
            key = (r["crate"], r["file"], r["test_name"], r["kind"])
            park_keys.add(key)
            park_reason[key] = (r.get("target") or r.get("note") or "").strip()
            survivors.append((r, f"PARKED -> pen (delete/elevate by {wall})"))
        elif disp == "keep-durable":
            survivors.append((r, "keep-durable"))

    removed_keys = delete_keys | park_keys
    # 1. delete + park: relocate rows OUT of live inventory (and legacy boundary table if present)
    new_inv = [row for row in inv if inv_key(row) not in removed_keys]
    new_b = [row for row in b_rows if inv_key(row) not in removed_keys] if boundary_present else []
    # 1b. parked inventory rows move into the quarantine pen with a wall-clock stamp.
    # Legacy boundary rows (if any) are preserved in parked_boundary for lossless un-park.
    pb_hdr, parked_b_rows = read_tsv(PARKED_BOUNDARY)
    if pb_hdr is None:
        parked_b_rows = []
    for key in park_keys:
        src_row = inv_by_key.get(key)
        if src_row is None:
            continue
        entry = dict(src_row)
        entry["parked_at"] = today.isoformat()
        entry["closeout_track"] = track
        entry["park_reason"] = park_reason.get(key, "")
        parked_rows.append(entry)
    if boundary_present:
        for row in b_rows:
            if inv_key(row) in park_keys:
                b_entry = dict(row)
                b_entry["parked_at"] = today.isoformat()
                b_entry["closeout_track"] = track
                parked_b_rows.append(b_entry)
    # 2. elevate-class: stamp durable class on the surviving inventory row
    for row in new_inv:
        key = inv_key(row)
        if key in class_updates:
            cls = class_updates[key].removeprefix("permanent-residue:")
            row["class"] = cls
            row["verdict"] = "KEEP"
            row["promotion_target"] = class_updates[key]

    def repo_move(src_rel, dst_rel):
        src = ROOT / src_rel
        dst = ROOT / dst_rel
        if not src.exists():
            die(f"elevate-code source missing: {src_rel}", 1)
        dst.parent.mkdir(parents=True, exist_ok=True)
        try:
            subprocess.run(["git", "-C", str(ROOT), "mv", src_rel, dst_rel],
                           check=True, capture_output=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            src.replace(dst)

    moved_notes = []
    dest_crates = set()
    for r in code_moves:
        tgt = r["target"].strip()
        repo_move(r["file"], tgt)
        parts = pathlib.PurePosixPath(tgt).parts
        new_crate = parts[1] if len(parts) >= 2 and parts[0] == "crates" else r["crate"]
        if len(parts) >= 2 and parts[0] == "crates":
            dest_crates.add(parts[1])
        # the ledger follows the code: retarget surviving inventory (+ legacy boundary) rows
        moved_key = (r["crate"], r["file"], r["test_name"], r["kind"])
        for row in new_inv:
            if inv_key(row) == moved_key:
                row["file"] = tgt
                row["crate"] = new_crate
        if boundary_present:
            for row in new_b:
                if inv_key(row) == moved_key:
                    row["file"] = tgt
                    row["crate"] = new_crate
        art_rows.append({
            "path": tgt, "leased_at": today.isoformat(),
            "disposition": "elevate-code-rehome-pending",
            "closeout_track": track,
            "note": f"moved from {r['file']}; add mod decl + confirm cargo check, then delete this ledger row",
        })
        moved_notes.append(f"{r['file']} -> {tgt}")

    # 2b. doc mutations (P1-4): delete, relocate, or lease the track's documents
    for f in doc_deletes:
        try:
            subprocess.run(["git", "-C", str(ROOT), "rm", "-q", "--", f],
                           check=True, capture_output=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            p = ROOT / f
            if p.exists():
                p.unlink()
    for r in doc_moves:
        repo_move(r["file"], r["target"].strip())
        moved_notes.append(f"{r['file']} -> {r['target'].strip()}")
    for r in doc_leases:
        art_rows.append({
            "path": r["file"], "leased_at": today.isoformat(), "disposition": "lease",
            "closeout_track": track,
            "note": (r.get("target") or r.get("note") or "").strip(),
        })

    # 2c. the closeout manifest itself is track residue (P1-5): lease it so the
    # reaper clears it after the audit window. The report is the durable record.
    try:
        man_rel = path.resolve().relative_to(ROOT.resolve()).as_posix()
    except ValueError:
        man_rel = ""
    if is_reapable_doc(man_rel) and not any(a.get("path") == man_rel for a in art_rows):
        art_rows.append({
            "path": man_rel, "leased_at": today.isoformat(), "disposition": "lease",
            "closeout_track": track,
            "note": "closeout manifest; audit window then reap via --decommission",
        })

    # 3. write mutated tables — never recreate a retired boundary ledger
    write_tsv(INVENTORY, INVENTORY_HEADER, new_inv)
    if boundary_present:
        write_tsv(BOUNDARY_ROWS, BOUNDARY_ROWS_HEADER, new_b)
    if art_rows:
        write_tsv(ARTIFACT_LEDGER, ARTIFACT_LEDGER_HEADER, art_rows)
    if park_keys or PARKED.exists():
        write_tsv(PARKED, PARKED_HEADER, parked_rows)
    if PARKED_BOUNDARY.exists() or (boundary_present and parked_b_rows):
        write_tsv(PARKED_BOUNDARY, PARKED_BOUNDARY_HEADER, parked_b_rows)

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
    if strip_park_block_on_close is not None:
        strip_park_block_on_close[0].write_bytes(strip_park_block_on_close[1].encode("utf-8"))

    # 5. Retire the orientation pointer if it still points at this closing track.
    apply_active_track_retirement(active_track_plan)

    # 6. gate battery (incl. cargo check of elevate-code destination crates, P1-6b)
    gates = run_gate_battery(track, dest_crates)

    # 7. compact, size-first report
    inv_after = len(new_inv)
    b_after = len(new_b) if boundary_present else 0
    grew = inv_after > inv_before or (boundary_present and b_after > b_before)
    report = render_report(track, live_receipt, tally, survivors,
                           inv_before, inv_after, b_before, b_after, gates, closed, moved_notes,
                           active_track_plan)
    report_path = ROOT / "docs" / "tests" / f"{track}_closeout_report.md"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_bytes(report.encode("utf-8"))

    print("TRACK-CLOSEOUT APPLY")
    print(f"  track: {track}  (birth_track closed: {'yes' if closed else 'no'})")
    if active_track_plan.get("retired") == "yes":
        print(f"  active_track_retired: yes ({active_track_plan['from']} -> {active_track_plan['to']})")
    else:
        print(f"  active_track_retired: no "
              f"({active_track_plan.get('current') or '-'}; {active_track_plan.get('reason') or '-'})")
    print(f"  inventory rows: {inv_before} -> {inv_after} (delta {inv_after - inv_before})")
    if boundary_present:
        print(f"  boundary rows:  {b_before} -> {b_after} (delta {b_after - b_before})")
    else:
        print("  boundary rows:  (ledger retired / absent — not written)")
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


def run_gate_battery(track: str, dest_crates=()) -> dict:
    gates = {}
    checks = [
        ("drift", [BASH, str(SCRIPT_DIR / "test_inventory_drift_check.sh")]),
        ("lifecycle-schema", [BASH, str(SCRIPT_DIR / "test_lifecycle_expiry_check.sh"), "--schema"]),
        ("track-expiry", [BASH, str(SCRIPT_DIR / "test_lifecycle_expiry_check.sh"),
                          "--track-closeout", track]),
    ]
    # P1-6b: an elevate-code move that doesn't compile must surface in the closeout
    # verdict, not days later. Escape hatch for environments without a toolchain.
    if os.environ.get("TRACK_CLOSEOUT_SKIP_CARGO", "") == "1":
        for crate in sorted(dest_crates):
            gates[f"cargo-check-{crate}"] = "SKIP"
    else:
        for crate in sorted(dest_crates):
            checks.append((f"cargo-check-{crate}", ["cargo", "check", "-q", "-p", crate]))
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


def render_report(track, receipt, tally, survivors, inv_b, inv_a, b_b, b_a, gates, closed, moved,
                  active_track):
    # Boundary table growth only counts when a legacy table is still in play (non-zero side).
    grew = inv_a > inv_b or (b_a > b_b and (b_b > 0 or b_a > 0))
    lines = []
    lines.append(f"# {track} — Track Closeout Report")
    lines.append("")
    lines.append("## Status")
    lines.append("")
    lines.append(f"birth_track closed: **{'yes' if closed else 'no'}**  ·  "
                 f"CLOSEOUT-RECEIPT: `{receipt}`  ·  role: {ROLE}")
    if active_track.get("retired") == "yes":
        lines.append(f"active_track_retired: **yes**  ·  active_track_from: "
                     f"`{active_track.get('from', '')}`  ·  active_track_to: "
                     f"`{active_track.get('to', '')}`")
    else:
        lines.append(f"active_track_retired: **no**  ·  active_track_current: "
                     f"`{active_track.get('current') or '-'}`  ·  active_track_reason: "
                     f"`{active_track.get('reason') or '-'}`")
    lines.append("")
    lines.append("## TSV table size (primary success metric — growth is the fail state)")
    lines.append("")
    lines.append("| table | before | after | delta |")
    lines.append("| --- | --- | --- | --- |")
    lines.append(f"| test_inventory.tsv | {inv_b} | {inv_a} | {inv_a - inv_b} |")
    if b_b or b_a:
        lines.append(f"| test_lifecycle_boundary_rows.tsv | {b_b} | {b_a} | {b_a - b_b} |")
    else:
        lines.append("| test_lifecycle_boundary_rows.tsv | retired | retired | 0 |")
    lines.append("")
    if tally.get("lease"):
        lines.append(f"_{tally['lease']} row(s) relocated to the parking pen "
                     f"(`test_lifecycle_parked.tsv`) — out of the live inventory, on a "
                     f"{LEASE_HARD_DAYS}-day wall-clock to delete-or-elevate._")
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
    _, parked = read_tsv(PARKED)
    today = now_date()
    cruft, expired, bad = [], [], []

    def account(ident, date_str):
        try:
            leased = _dt.date.fromisoformat(date_str)
        except ValueError:
            bad.append(ident)
            return
        age = (today - leased).days
        if age >= LEASE_HARD_DAYS:
            expired.append((ident, age))
        elif age >= LEASE_CRUFT_DAYS:
            cruft.append((ident, age))

    for row in rows:
        account(row.get("path", "?"), row.get("leased_at", ""))
    for row in parked:
        ident = "::".join((row.get("crate", ""), row.get("file", ""),
                           row.get("test_name", ""), row.get("kind", "")))
        account(f"parked:{ident}", row.get("parked_at", ""))

    print("TRACK-CLOSEOUT ARTIFACT-EXPIRY (wall-clock)")
    print(f"  leased artifacts: {len(rows)}  parked rows: {len(parked)}  now: {today.isoformat()}")
    for path, age in expired:
        print(f"  - EXPIRED ({age}d >= {LEASE_HARD_DAYS}d, must delete or elevate): {path}")
    for path, age in cruft:
        print(f"  - CRUFT ({age}d >= {LEASE_CRUFT_DAYS}d): {path}")
    for path in bad:
        print(f"  - MALFORMED leased_at: {path}")
    if expired or bad:
        print(f"  remedy: run `track_closeout.sh --decommission` to reap the safe expired assets, "
              f"or elevate/delete the rest by hand")
        print(f"ARTIFACT-EXPIRY-VERDICT: FAIL expired={len(expired)} cruft={len(cruft)} malformed={len(bad)}")
        return 1
    if cruft:
        print(f"ARTIFACT-EXPIRY-VERDICT: INSPECT expired=0 cruft={len(cruft)} malformed=0")
        return 0
    print(f"ARTIFACT-EXPIRY-VERDICT: PASS expired=0 cruft=0 malformed=0")
    return 0


# ---------- decommission (reaper) ----------

def cmd_decommission():
    """Actually delete expired parked/leased assets — but only the unambiguously safe
    ones. Refuses (and reports) anything whose deletion could bulldoze a live asset:
    inline/src unit tests, shared test files, and code awaiting rehome."""
    dry = "--dry-run" in argv
    reap_all = "--all" in argv  # reap every parked/leased row, not just those past the wall
    today = now_date()

    _, parked = read_tsv(PARKED)
    _, art_rows = read_tsv(ARTIFACT_LEDGER)
    art_rows = art_rows or []
    _, inv = read_tsv(INVENTORY)

    live_file_refs = {}
    for r in inv:
        live_file_refs[r["file"]] = live_file_refs.get(r["file"], 0) + 1
    parked_file_refs = {}
    for r in parked:
        parked_file_refs[r["file"]] = parked_file_refs.get(r["file"], 0) + 1

    def past_wall(date_str):
        try:
            return (today - _dt.date.fromisoformat(date_str)).days >= LEASE_HARD_DAYS
        except ValueError:
            return False

    def is_dedicated_test_file(path):
        return path.startswith("crates/") and "/tests/" in path and path.endswith(".rs")

    reaped_ids, rm_files, manual, kept = set(), [], [], []
    for r in parked:
        ident = "::".join((r["crate"], r["file"], r["test_name"], r["kind"]))
        if not (reap_all or past_wall(r.get("parked_at", ""))):
            kept.append(r)
            continue
        tn, f = r.get("test_name", ""), r.get("file", "")
        if tn.startswith("cfg_test_mod::"):
            reaped_ids.add(ident)  # ledger-only marker: drop the row, touch no source
        elif is_dedicated_test_file(f) and live_file_refs.get(f, 0) == 0 and parked_file_refs.get(f, 0) == 1:
            rm_files.append(f)
            reaped_ids.add(ident)
        else:
            manual.append((ident, "inline/src or shared file — remove the test by hand, then drop the pen row"))
            kept.append(r)

    new_art, art_rm = [], []
    for r in art_rows:
        due = reap_all or past_wall(r.get("leased_at", ""))
        disp, p = r.get("disposition", ""), r.get("path", "")
        if due and disp == "elevate-code-rehome-pending":
            manual.append((p, "code awaiting rehome — reaping would lose elevated code; rehome or delete by hand"))
            new_art.append(r)
        elif due and disp == "lease" and (is_dedicated_test_file(p) or is_reapable_doc(p)):
            art_rm.append(p)
        elif due and disp == "lease":
            manual.append((p, "expired lease on a non-reapable path — delete or relocate by hand"))
            new_art.append(r)
        else:
            new_art.append(r)

    if not dry:
        for f in rm_files + art_rm:
            try:
                subprocess.run(["git", "-C", str(ROOT), "rm", "-q", "--", f], check=True, capture_output=True)
            except (subprocess.CalledProcessError, FileNotFoundError):
                p = ROOT / f
                if p.exists():
                    p.unlink()
        write_tsv(PARKED, PARKED_HEADER, [
            r for r in parked
            if "::".join((r["crate"], r["file"], r["test_name"], r["kind"])) not in reaped_ids
        ])
        # reaped pen rows take their preserved boundary rows with them (P1-6a)
        pb_hdr, parked_b = read_tsv(PARKED_BOUNDARY)
        if pb_hdr is not None:
            write_tsv(PARKED_BOUNDARY, PARKED_BOUNDARY_HEADER, [
                r for r in parked_b
                if "::".join((r["crate"], r["file"], r["test_name"], r["kind"])) not in reaped_ids
            ])
        if art_rm:
            write_tsv(ARTIFACT_LEDGER, ARTIFACT_LEDGER_HEADER, new_art)

    print("TRACK-CLOSEOUT DECOMMISSION" + (" (dry-run)" if dry else ""))
    print(f"  now: {today.isoformat()}  wall: {LEASE_HARD_DAYS}d  "
          f"mode: {'all-parked' if reap_all else 'expired-only'}")
    print(f"  parked rows reaped: {len(reaped_ids)}")
    print(f"  files deleted: {len(rm_files) + len(art_rm)}")
    for f in rm_files + art_rm:
        print(f"    - rm {f}")
    for ident, reason in manual:
        print(f"    ! manual: {ident} — {reason}")
    verdict = "DRY" if dry else "OK"
    print(f"TRACK-CLOSEOUT-DECOMMISSION-VERDICT: {verdict} "
          f"reaped={len(reaped_ids)} files={len(rm_files) + len(art_rm)} manual={len(manual)}")
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

    # P0-2: closure must PREDATE the PR (status at base), or the PR must itself be a
    # lawful closeout (track closed at head AND closeout report+manifest are in the diff).
    # Otherwise an agent could hand-flip status=closed and delete rows in one PR,
    # bypassing the whole protocol.
    trk_rel = "scripts/ci/test_lifecycle_tracks.tsv"
    base_status = {t["track_id"]: t["status"] for t in (git_show_tsv(base, trk_rel) or [])}
    head_status = {t["track_id"]: t["status"] for t in (git_show_tsv(head, trk_rel) or [])}
    try:
        diff_out = subprocess.run(["git", "-C", str(ROOT), "diff", "--name-only", base, head],
                                  capture_output=True, check=True)
        changed_files = set(norm_bytes(diff_out.stdout).splitlines())
    except subprocess.CalledProcessError:
        changed_files = set()

    violations = []
    for r in removed:
        bt = r.get("birth_track", "").strip()
        if is_cfg_marker_deletion_candidate(r):
            continue  # ledger-only residue has its own sanctioned sweep route
        if base_status.get(bt) == "closed":
            continue
        if (head_status.get(bt) == "closed"
                and f"docs/tests/{bt}_closeout_report.md" in changed_files
                and f"docs/tests/{bt}_closeout_manifest.tsv" in changed_files):
            continue  # this PR IS the closeout apply
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

    def first_payload_line(path: pathlib.Path) -> str:
        for line in norm_bytes(path.read_bytes()).splitlines():
            stripped = line.strip()
            if stripped and not stripped.startswith("#"):
                return stripped
        return ""

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
    check("doc-lease-design-rejected", bool(validate_dispositions(
        [{"asset_kind": "doc", "ref": "d", "file": "docs/track_closeout_protocol.md",
          "disposition": "lease", "owner": "", "target": ""}])))
    check("doc-archive-target-ok", not validate_dispositions(
        [{"asset_kind": "doc", "ref": "d", "file": "docs/track_closeout_protocol.md",
          "disposition": "elevate-code", "owner": "", "target": "docs/archive/track_closeout_protocol.md"}]))
    check("doc-bad-archive-target", bool(validate_dispositions(
        [{"asset_kind": "doc", "ref": "d", "file": "docs/track_closeout_protocol.md",
          "disposition": "elevate-code", "owner": "", "target": "docs/tests/track_closeout_protocol.md"}])))
    old_skip = os.environ.get("TRACK_CLOSEOUT_SKIP_CARGO")
    os.environ["TRACK_CLOSEOUT_SKIP_CARGO"] = "1"
    try:
        cargo_skip_gates = run_gate_battery("pre-lifecycle", {"simthing-core"})
    finally:
        if old_skip is None:
            os.environ.pop("TRACK_CLOSEOUT_SKIP_CARGO", None)
        else:
            os.environ["TRACK_CLOSEOUT_SKIP_CARGO"] = old_skip
    check("cargo-gate-skip-recorded",
          cargo_skip_gates.get("cargo-check-simthing-core") == "SKIP")

    def write_min_closeout_sandbox(root: pathlib.Path, parked_status: bool):
        (root / "scripts/ci").mkdir(parents=True)
        (root / "docs/tests").mkdir(parents=True)
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", root / "scripts/ci/track_closeout.sh")
        status = "PARKED" if parked_status else "OPEN"
        block = (
            "\n<!-- SIMTHING-PARKED-TRACK:BEGIN agents: read only when executing --unpark -->\n"
            "```json\n"
            "{\"park_receipt\":\"000000000000\"}\n"
            "```\n"
            "<!-- SIMTHING-PARKED-TRACK:END -->\n"
        )
        (root / "docs/min_track.md").write_text(
            f"# min\n\n> **Status: {status} / fixture.**\n\nworkplan\n" + block,
            encoding="utf-8",
        )
        write_tsv(root / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [
            {"crate": "c", "file": "crates/c/tests/min.rs", "test_name": "golden",
             "kind": "integration", "class": "golden-byte", "superseding_boundary": "B",
             "verdict": "KEEP", "note": "keep", "promotion_target": "permanent-residue:golden-byte",
             "birth_track": "min-track", "dsu_survivals": "0"},
        ])
        write_tsv(root / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [
            {"track_id": "min-track", "status": "open", "closed_at": "-",
             "source": "docs/min_track.md", "note": "fixture"},
        ])
        write_tsv(root / "scripts/ci/test_residue_classes.tsv", ["promotion_target"], [
            {"promotion_target": "permanent-residue:golden-byte"},
        ])
        write_tsv(root / "scripts/ci/closeout_artifacts.tsv", ARTIFACT_LEDGER_HEADER, [])
        (root / "scripts/ci/active_track.txt").write_text(
            f"{ACTIVE_TRACK_COMMENT}\nnone\n", encoding="utf-8")
        rows = [
            {"asset_kind": "inventory-row", "ref": "c::crates/c/tests/min.rs::golden::integration",
             "crate": "c", "file": "crates/c/tests/min.rs", "test_name": "golden",
             "kind": "integration", "current_class": "golden-byte", "birth_track": "min-track",
             "disposition": "keep-durable", "target": "", "owner": "", "note": "keep"},
            doc_manifest_row("min-track", "docs/min_track.md", "keep-durable", "source"),
        ]
        body = io.StringIO()
        writer = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow(row)
        receipt = closeout_receipt(body.getvalue())
        (root / "docs/tests/min-track_closeout_manifest.tsv").write_text(
            "# track_closeout manifest\n"
            "# track: min-track\n"
            f"# CLOSEOUT-RECEIPT: {receipt}\n"
            "# role: prove\n"
            + body.getvalue(),
            encoding="utf-8",
        )

    with tempfile.TemporaryDirectory() as ptmp:
        pr = pathlib.Path(ptmp)
        write_min_closeout_sandbox(pr, parked_status=True)
        r_parked = subprocess.run(
            [BASH, "scripts/ci/track_closeout.sh", "--apply", "docs/tests/min-track_closeout_manifest.tsv"],
            cwd=str(pr), capture_output=True, text=True,
            env={**os.environ, "TRACK_CLOSEOUT_NOW": "2026-07-20", "TRACK_CLOSEOUT_SKIP_CARGO": "1"},
        )
        check("closeout-refuses-while-parked",
              r_parked.returncode != 0 and "FAIL(track-parked-unpark-first)" in r_parked.stderr)

    with tempfile.TemporaryDirectory() as ctmp:
        cr = pathlib.Path(ctmp)
        write_min_closeout_sandbox(cr, parked_status=False)
        r_close = subprocess.run(
            [BASH, "scripts/ci/track_closeout.sh", "--apply", "docs/tests/min-track_closeout_manifest.tsv"],
            cwd=str(cr), capture_output=True, text=True,
            env={**os.environ, "TRACK_CLOSEOUT_NOW": "2026-07-20", "TRACK_CLOSEOUT_SKIP_CARGO": "1"},
        )
        check("closeout-deletes-block",
              "TRACK-CLOSEOUT-APPLY-VERDICT:" in r_close.stdout
              and PARK_BEGIN not in norm_bytes((cr / "docs/min_track.md").read_bytes()))

    # full build -> check -> apply roundtrip in a sandbox
    with tempfile.TemporaryDirectory() as tmp:
        sb = pathlib.Path(tmp)
        (sb / "scripts" / "ci").mkdir(parents=True)
        (sb / "docs" / "tests").mkdir(parents=True)
        (sb / "docs" / "sb.md").write_text("# sb design\n", encoding="utf-8")
        (sb / "docs" / "tests" / "sb_results.md").write_text("# sb results\n", encoding="utf-8")
        (sb / "docs" / "tests" / "manual_results.md").write_text("# manual\n", encoding="utf-8")
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
            {"crate": "c", "file": "crates/c/tests/park.rs", "test_name": "park_me",
             "kind": "integration", "class": "behavior-regression", "superseding_boundary": "B-T5",
             "verdict": "AUDIT", "note": "undecided", "promotion_target": "permanent-residue:behavior-regression",
             "birth_track": "sb-track", "dsu_survivals": "0"},
        ]
        b_rows = [
            {"crate": "c", "file": "crates/c/src/a.rs", "test_name": "cfg_test_mod::tests",
             "kind": "unit", "current_class": "deletion-candidate", "boundary_id": "B-T6-MODULE-MARKER-EXPANSION",
             "boundary_tier": "TIER6_PROMOTION_REQUIRED", "recommended_disposition": "", "representative_to_keep": "",
             "consolidation_target": "", "promotion_required": "", "confidence": "high", "note": "marker"},
            {"crate": "c", "file": "crates/c/tests/park.rs", "test_name": "park_me",
             "kind": "integration", "current_class": "behavior-regression", "boundary_id": "B-T5",
             "boundary_tier": "TIER5_BEHAVIOR", "recommended_disposition": "", "representative_to_keep": "",
             "consolidation_target": "", "promotion_required": "", "confidence": "medium", "note": "park"},
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
        (sb / "scripts/ci/gen_orientation.sh").write_text(
            "#!/usr/bin/env bash\n"
            "set -euo pipefail\n"
            "mkdir -p docs\n"
            "printf '# generated\\n' > docs/orchestrator_orientation.md\n",
            encoding="utf-8",
        )

        def run(*a, now="2026-07-07"):
            return subprocess.run(
                [BASH, "scripts/ci/track_closeout.sh", *a],
                capture_output=True, text=True, cwd=str(sb),
                env={**os.environ, "TRACK_CLOSEOUT_NOW": now},
            )

        manifest_rel = "docs/tests/sb-track_closeout_manifest.tsv"
        r_build = run("--build-manifest", "--track", "sb-track", "--docs", "docs/tests/manual_results.md")
        check("build-manifest-ok", "BUILD-MANIFEST-VERDICT: OK" in r_build.stdout)
        man_path = sb / manifest_rel
        man = norm_bytes(man_path.read_bytes())
        # marker auto-deletes; golden is keep-durable; the non-durable row => needs-disposition
        check("build-auto-clears-marker", "\tdelete\t" in man)
        check("build-keeps-durable", "keep-durable" in man)
        check("build-flags-needs-disposition", "\tneeds-disposition\t" in man)
        check("build-includes-source-doc", "doc::docs/sb.md" in man)
        check("build-auto-discovers-result-doc", "doc::docs/tests/sb_results.md" in man)
        check("build-includes-explicit-doc", "doc::docs/tests/manual_results.md" in man)

        # check-eval must REFUSE the unresolved needs-disposition
        r_eval_bad = run("--check-eval", manifest_rel)
        check("check-eval-refuses-unresolved", "CHECK-EVAL-VERDICT: FAIL" in r_eval_bad.stdout)

        # resolve inventory and doc rows independently.
        _, _, fields, manifest_rows = load_manifest(man_path)
        for row in manifest_rows:
            if row["asset_kind"] == "inventory-row" and row["test_name"] == "park_me":
                row["disposition"] = "lease"
                row["target"] = "undecided-audit"
            elif row["asset_kind"] == "doc" and row["file"] == "docs/sb.md":
                row["disposition"] = "elevate-code"
                row["target"] = "docs/archive/sb.md"
            elif row["asset_kind"] == "doc" and row["file"] == "docs/tests/sb_results.md":
                row["disposition"] = "lease"
                row["target"] = "result-doc-audit"
            elif row["asset_kind"] == "doc" and row["file"] == "docs/tests/manual_results.md":
                row["disposition"] = "lease"
                row["target"] = "explicit-doc-audit"
        body = io.StringIO()
        w = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
        w.writeheader()
        for row in manifest_rows:
            w.writerow(row)
        receipt = closeout_receipt(body.getvalue())
        man_path.write_bytes((
            "# track_closeout manifest\n"
            "# track: sb-track\n"
            f"# CLOSEOUT-RECEIPT: {receipt}\n"
            "# role: prove\n"
            + body.getvalue()
        ).encode("utf-8"))

        r_eval = run("--check-eval", manifest_rel)
        check("check-eval-pass", "CHECK-EVAL-VERDICT: PASS" in r_eval.stdout)

        # P0-1: a row born into the track after --build-manifest must refuse apply
        write_tsv(sb / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv_rows + [
            {"crate": "c", "file": "crates/c/tests/late.rs", "test_name": "late_arrival",
             "kind": "integration", "class": "behavior-regression", "superseding_boundary": "B-T5",
             "verdict": "AUDIT", "note": "born after manifest",
             "promotion_target": "permanent-residue:behavior-regression",
             "birth_track": "sb-track", "dsu_survivals": "0"},
        ])
        r_stale = run("--apply", manifest_rel)
        check("apply-stale-manifest-fail",
              r_stale.returncode != 0 and "FAIL(stale-manifest)" in r_stale.stderr)
        write_tsv(sb / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv_rows)

        # P0-1: auto-scoped docs get the same build->apply freshness guard.
        (sb / "docs/tests/sb_extra_results.md").write_text("# late doc\n", encoding="utf-8")
        r_doc_born = run("--apply", manifest_rel)
        check("apply-stale-doc-born-fail",
              r_doc_born.returncode != 0
              and "FAIL(stale-manifest)" in r_doc_born.stderr
              and "doc_unscoped=1" in r_doc_born.stderr)
        (sb / "docs/tests/sb_extra_results.md").unlink()
        (sb / "docs/tests/sb_results.md").unlink()
        r_doc_vanished = run("--apply", manifest_rel)
        check("apply-stale-doc-vanished-fail",
              r_doc_vanished.returncode != 0
              and "FAIL(stale-manifest)" in r_doc_vanished.stderr
              and "doc_vanished=1" in r_doc_vanished.stderr)
        (sb / "docs/tests/sb_results.md").write_text("# sb results\n", encoding="utf-8")
        (sb / "docs/tests/manual_results.md").unlink()
        r_explicit_doc_vanished = run("--apply", manifest_rel)
        check("apply-stale-explicit-doc-vanished-fail",
              r_explicit_doc_vanished.returncode != 0
              and "FAIL(stale-manifest)" in r_explicit_doc_vanished.stderr
              and "explicit_doc_vanished=1" in r_explicit_doc_vanished.stderr)
        (sb / "docs/tests/manual_results.md").write_text("# manual\n", encoding="utf-8")

        active_path = sb / "scripts/ci/active_track.txt"
        active_path.write_text(f"{ACTIVE_TRACK_COMMENT}\nC:/not/repo.md\n", encoding="utf-8")
        inv_before_bad_active = norm_bytes((sb / "scripts/ci/test_inventory.tsv").read_bytes())
        tracks_before_bad_active = norm_bytes((sb / "scripts/ci/test_lifecycle_tracks.tsv").read_bytes())
        active_before_bad_active = norm_bytes(active_path.read_bytes())
        r_bad_active = run("--apply", manifest_rel)
        check("apply-invalid-active-track-harness-error",
              r_bad_active.returncode != 0
              and "FAIL(harness-error)" in r_bad_active.stderr
              and "active_track_reason=invalid-path" in r_bad_active.stderr)
        check("apply-invalid-active-track-pre-mutation",
              norm_bytes((sb / "scripts/ci/test_inventory.tsv").read_bytes()) == inv_before_bad_active
              and norm_bytes((sb / "scripts/ci/test_lifecycle_tracks.tsv").read_bytes()) == tracks_before_bad_active
              and norm_bytes(active_path.read_bytes()) == active_before_bad_active)
        active_path.write_text(f"{ACTIVE_TRACK_COMMENT}\ndocs/sb.md\n", encoding="utf-8")

        r_apply = run("--apply", manifest_rel)
        check("apply-ok", "APPLY-VERDICT: OK" in r_apply.stdout or "APPLY-VERDICT: INSPECT" in r_apply.stdout
              or "APPLY-VERDICT:" in r_apply.stdout)
        _, inv_after = read_tsv(sb / "scripts/ci/test_inventory.tsv")
        check("apply-deleted-marker-row", all(r["test_name"] != "cfg_test_mod::tests" for r in inv_after))
        check("apply-kept-golden-row", any(r["test_name"] == "golden" for r in inv_after))
        # parked row left the live inventory and landed in the pen with a wall-clock stamp
        check("apply-parked-row-left-inventory", all(r["test_name"] != "park_me" for r in inv_after))
        _, parked_after = read_tsv(sb / "scripts/ci/test_lifecycle_parked.tsv")
        park_hit = next((r for r in parked_after if r["test_name"] == "park_me"), None)
        check("apply-parked-row-in-pen", park_hit is not None)
        check("apply-parked-row-stamped", bool(park_hit) and park_hit.get("parked_at") == "2026-07-07")
        _, b_after = read_tsv(sb / "scripts/ci/test_lifecycle_boundary_rows.tsv")
        check("apply-deleted-boundary-lockstep", len(b_after) == 0)
        _, parked_b_after = read_tsv(sb / "scripts/ci/test_lifecycle_parked_boundary.tsv")
        check("apply-parked-boundary-preserved",
              any(r["test_name"] == "park_me" for r in parked_b_after))
        check("apply-legacy-boundary-table-present",
              (sb / "scripts/ci/test_lifecycle_boundary_rows.tsv").exists())
        _, trk_after = read_tsv(sb / "scripts/ci/test_lifecycle_tracks.tsv")
        check("apply-closed-birth-track", any(t["track_id"] == "sb-track" and t["status"] == "closed" for t in trk_after))
        check("apply-report-written", (sb / "docs/tests/sb-track_closeout_report.md").exists())
        report_text = norm_bytes((sb / "docs/tests/sb-track_closeout_report.md").read_bytes())
        check("apply-active-track-retired",
              first_payload_line(active_path) == NO_ACTIVE_TRACK
              and "active_track_retired: **yes**" in report_text
              and f"active_track_to: `{NO_ACTIVE_TRACK}`" in report_text)
        check("apply-active-track-regenerated-orientation",
              (sb / "docs/orchestrator_orientation.md").exists())
        check("apply-source-doc-archived", (sb / "docs/archive/sb.md").exists())
        _, art_apply = read_tsv(sb / "scripts/ci/closeout_artifacts.tsv")
        check("apply-result-doc-leased",
              any(r["path"] == "docs/tests/sb_results.md" and r["disposition"] == "lease" for r in art_apply))
        check("apply-explicit-doc-leased",
              any(r["path"] == "docs/tests/manual_results.md" and r["disposition"] == "lease" for r in art_apply))
        check("apply-manifest-self-leased",
              any(r["path"] == manifest_rel and r["disposition"] == "lease" for r in art_apply))

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

        # the parking pen is on the same wall-clock: isolate it and push past the hard wall
        write_tsv(sb / "scripts/ci/closeout_artifacts.tsv", ARTIFACT_LEDGER_HEADER, [])
        r_park_exp = run("--artifact-expiry", now="2026-07-20")
        check("parked-pen-wall-clock-fail",
              "ARTIFACT-EXPIRY-VERDICT: FAIL" in r_park_exp.stdout and "parked:" in r_park_exp.stdout)

        # P0-3: apply against a track missing from tracks.tsv is a hard harness-error
        r_b2 = run("--build-manifest", "--track", "sb-track", "--out", "m2.tsv")
        check("build-manifest-2-ok", "BUILD-MANIFEST-VERDICT: OK" in r_b2.stdout)
        _, _, _, m2_rows = load_manifest(sb / "m2.tsv")
        for row in m2_rows:
            if row["asset_kind"] == "doc":
                row["disposition"] = "keep-durable"
        body2 = io.StringIO()
        w2 = csv.DictWriter(body2, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
        w2.writeheader()
        for row in m2_rows:
            w2.writerow(row)
        receipt2 = closeout_receipt(body2.getvalue())
        (sb / "m2.tsv").write_bytes((
            "# track_closeout manifest\n"
            "# track: sb-track\n"
            f"# CLOSEOUT-RECEIPT: {receipt2}\n"
            "# role: prove\n"
            + body2.getvalue()
        ).encode("utf-8"))
        run("--check-eval", "m2.tsv")
        write_tsv(sb / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [])
        r_unk = run("--apply", "m2.tsv")
        check("apply-unknown-track-harness-error",
              r_unk.returncode != 0 and "FAIL(harness-error)" in r_unk.stderr)

    # HU-INVENTORY-ONEWRITE-0: apply with boundary ledger ABSENT — must not recreate it.
    with tempfile.TemporaryDirectory() as btmp:
        br = pathlib.Path(btmp)
        (br / "scripts/ci").mkdir(parents=True)
        (br / "docs/tests").mkdir(parents=True)
        (br / "docs/ba.md").write_text("# ba design\n", encoding="utf-8")
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", br / "scripts/ci/track_closeout.sh")
        (br / "scripts/ci/gen_orientation.sh").write_text(
            "#!/usr/bin/env bash\nset -euo pipefail\nmkdir -p docs\n"
            "printf '# generated\\n' > docs/orchestrator_orientation.md\n",
            encoding="utf-8",
        )
        write_tsv(br / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [
            {"crate": "c", "file": "crates/c/tests/keep.rs", "test_name": "golden",
             "kind": "integration", "class": "golden-byte", "superseding_boundary": "B-T7",
             "verdict": "KEEP", "note": "keep", "promotion_target": "permanent-residue:golden-byte",
             "birth_track": "ba-track", "dsu_survivals": "0"},
            {"crate": "c", "file": "crates/c/tests/gone.rs", "test_name": "delete_me",
             "kind": "integration", "class": "behavior-regression", "superseding_boundary": "B-T5",
             "verdict": "AUDIT", "note": "x", "promotion_target": "permanent-residue:behavior-regression",
             "birth_track": "ba-track", "dsu_survivals": "0"},
            {"crate": "c", "file": "crates/c/tests/park.rs", "test_name": "park_me",
             "kind": "integration", "class": "behavior-regression", "superseding_boundary": "B-T5",
             "verdict": "AUDIT", "note": "u", "promotion_target": "permanent-residue:behavior-regression",
             "birth_track": "ba-track", "dsu_survivals": "0"},
        ])
        # intentionally NO test_lifecycle_boundary_rows.tsv
        write_tsv(br / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [
            {"track_id": "ba-track", "status": "open", "closed_at": "-", "source": "docs/ba.md", "note": "x"},
        ])
        write_tsv(br / "scripts/ci/test_residue_classes.tsv", ["promotion_target"], [
            {"promotion_target": "permanent-residue:golden-byte"},
        ])
        (br / "scripts/ci/active_track.txt").write_text(
            f"{ACTIVE_TRACK_COMMENT}\ndocs/ba.md\n", encoding="utf-8")

        def brun(*a, now="2026-07-07"):
            return subprocess.run(
                [BASH, "scripts/ci/track_closeout.sh", *a],
                capture_output=True, text=True, cwd=str(br),
                env={**os.environ, "TRACK_CLOSEOUT_NOW": now},
            )

        man_rel = "docs/tests/ba-track_closeout_manifest.tsv"
        r_bbuild = brun("--build-manifest", "--track", "ba-track")
        check("boundary-absent-build-ok", "BUILD-MANIFEST-VERDICT: OK" in r_bbuild.stdout)
        man_path = br / man_rel
        if not man_path.exists():
            check("boundary-absent-manifest-written", False)
        else:
            _, _, _, mrows = load_manifest(man_path)
            for row in mrows:
                if row["asset_kind"] == "inventory-row" and row["test_name"] == "delete_me":
                    row["disposition"] = "delete"
                    row["owner"] = "prove-boundary-absent: necessity delete sample"
                elif row["asset_kind"] == "inventory-row" and row["test_name"] == "park_me":
                    row["disposition"] = "lease"
                    row["target"] = "undecided"
                elif row["asset_kind"] == "doc":
                    row["disposition"] = "keep-durable"
            body = io.StringIO()
            w = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
            w.writeheader()
            for row in mrows:
                w.writerow(row)
            receipt = closeout_receipt(body.getvalue())
            man_path.write_bytes((
                "# track_closeout manifest\n# track: ba-track\n"
                f"# CLOSEOUT-RECEIPT: {receipt}\n# role: prove\n" + body.getvalue()
            ).encode("utf-8"))
            r_beval = brun("--check-eval", man_rel)
            check("boundary-absent-check-eval", "CHECK-EVAL-VERDICT: PASS" in r_beval.stdout)
            r_ba = brun("--apply", man_rel)
            check("apply-boundary-absent-ok", "APPLY-VERDICT:" in r_ba.stdout)
            check("apply-boundary-absent-no-recreate",
                  not (br / "scripts/ci/test_lifecycle_boundary_rows.tsv").exists())
            check("apply-boundary-absent-no-parked-boundary",
                  not (br / "scripts/ci/test_lifecycle_parked_boundary.tsv").exists())
            _, inv_ba = read_tsv(br / "scripts/ci/test_inventory.tsv")
            check("apply-boundary-absent-deleted-row",
                  all(r["test_name"] != "delete_me" for r in inv_ba))
            _, parked_ba = read_tsv(br / "scripts/ci/test_lifecycle_parked.tsv")
            check("apply-boundary-absent-parked-inventory",
                  all(r["test_name"] != "park_me" for r in inv_ba)
                  and any(r["test_name"] == "park_me" for r in parked_ba))
            rep_path = br / "docs/tests/ba-track_closeout_report.md"
            report_ba = norm_bytes(rep_path.read_bytes()) if rep_path.exists() else ""
            check("apply-boundary-absent-report-retired",
                  "retired" in report_ba)
    # active pointer already unset is a valid successful no-op.
    with tempfile.TemporaryDirectory() as ntmp:
        nr = pathlib.Path(ntmp)
        (nr / "scripts/ci").mkdir(parents=True)
        (nr / "docs/tests").mkdir(parents=True)
        (nr / "docs/none_track.md").write_text("# none track\n", encoding="utf-8")
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", nr / "scripts/ci/track_closeout.sh")
        write_tsv(nr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [
            {"crate": "c", "file": "crates/c/tests/none.rs", "test_name": "golden",
             "kind": "integration", "class": "golden-byte", "superseding_boundary": "B",
             "verdict": "KEEP", "note": "keep", "promotion_target": "permanent-residue:golden-byte",
             "birth_track": "none-track", "dsu_survivals": "0"},
        ])
        # boundary ledger absent (post-retirement path)
        write_tsv(nr / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [
            {"track_id": "none-track", "status": "open", "closed_at": "-", "source": "docs/none_track.md", "note": "x"},
        ])
        write_tsv(nr / "scripts/ci/test_residue_classes.tsv", ["promotion_target"], [
            {"promotion_target": "permanent-residue:golden-byte"},
        ])
        (nr / "scripts/ci/active_track.txt").write_text(
            f"{ACTIVE_TRACK_COMMENT}\n{NO_ACTIVE_TRACK}\n", encoding="utf-8")
        n_rows = [
            {"asset_kind": "inventory-row", "ref": "c::crates/c/tests/none.rs::golden::integration",
             "crate": "c", "file": "crates/c/tests/none.rs", "test_name": "golden", "kind": "integration",
             "current_class": "golden-byte", "birth_track": "none-track", "disposition": "keep-durable",
             "target": "", "owner": "", "note": "keep"},
            doc_manifest_row("none-track", "docs/none_track.md", "keep-durable", "source"),
        ]
        body = io.StringIO()
        w = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
        w.writeheader()
        for row in n_rows:
            w.writerow(row)
        n_receipt = closeout_receipt(body.getvalue())
        n_manifest = nr / "docs/tests/none-track_closeout_manifest.tsv"
        n_manifest.write_bytes((
            "# track_closeout manifest\n"
            "# track: none-track\n"
            f"# CLOSEOUT-RECEIPT: {n_receipt}\n"
            "# role: prove\n"
            + body.getvalue()
        ).encode("utf-8"))

        def nrun(*a):
            return subprocess.run([BASH, "scripts/ci/track_closeout.sh", *a],
                                  capture_output=True, text=True, cwd=str(nr),
                                  env={**os.environ, "TRACK_CLOSEOUT_NOW": "2026-07-07"})

        nrun("--check-eval", "docs/tests/none-track_closeout_manifest.tsv")
        r_none = nrun("--apply", "docs/tests/none-track_closeout_manifest.tsv")
        n_report = nr / "docs/tests/none-track_closeout_report.md"
        n_report_text = norm_bytes(n_report.read_bytes()) if n_report.exists() else ""
        _, n_tracks_after = read_tsv(nr / "scripts/ci/test_lifecycle_tracks.tsv")
        check("apply-none-active-track-noop",
              "APPLY-VERDICT:" in r_none.stdout
              and first_payload_line(nr / "scripts/ci/active_track.txt") == NO_ACTIVE_TRACK
              and any(t["track_id"] == "none-track" and t["status"] == "closed" for t in n_tracks_after))
        check("apply-none-active-track-report",
              "active_track_retired: **no**" in n_report_text
              and "active_track_reason: `no-active-track`" in n_report_text)

    # active pointer not owned by the closing manifest must not be hijacked.
    with tempfile.TemporaryDirectory() as atmp:
        ar = pathlib.Path(atmp)
        (ar / "scripts/ci").mkdir(parents=True)
        (ar / "docs/tests").mkdir(parents=True)
        (ar / "docs/quiet.md").write_text("# quiet\n", encoding="utf-8")
        (ar / "docs/other_active.md").write_text("# other\n", encoding="utf-8")
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", ar / "scripts/ci/track_closeout.sh")
        write_tsv(ar / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [
            {"crate": "c", "file": "crates/c/tests/quiet.rs", "test_name": "golden",
             "kind": "integration", "class": "golden-byte", "superseding_boundary": "B",
             "verdict": "KEEP", "note": "keep", "promotion_target": "permanent-residue:golden-byte",
             "birth_track": "quiet-track", "dsu_survivals": "0"},
        ])
        write_tsv(ar / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, [
            {"track_id": "quiet-track", "status": "open", "closed_at": "-", "source": "docs/quiet.md", "note": "x"},
        ])
        write_tsv(ar / "scripts/ci/test_residue_classes.tsv", ["promotion_target"], [
            {"promotion_target": "permanent-residue:golden-byte"},
        ])
        (ar / "scripts/ci/active_track.txt").write_text(
            f"{ACTIVE_TRACK_COMMENT}\ndocs/other_active.md\n", encoding="utf-8")
        manifest_rows = [
            {"asset_kind": "inventory-row", "ref": "c::crates/c/tests/quiet.rs::golden::integration",
             "crate": "c", "file": "crates/c/tests/quiet.rs", "test_name": "golden", "kind": "integration",
             "current_class": "golden-byte", "birth_track": "quiet-track", "disposition": "keep-durable",
             "target": "", "owner": "", "note": "keep"},
            doc_manifest_row("quiet-track", "docs/quiet.md", "keep-durable", "source"),
        ]
        body = io.StringIO()
        w = csv.DictWriter(body, fieldnames=MANIFEST_HEADER, delimiter="\t", lineterminator="\n")
        w.writeheader()
        for row in manifest_rows:
            w.writerow(row)
        q_receipt = closeout_receipt(body.getvalue())
        q_manifest = ar / "docs/tests/quiet-track_closeout_manifest.tsv"
        q_manifest.write_bytes((
            "# track_closeout manifest\n"
            "# track: quiet-track\n"
            f"# CLOSEOUT-RECEIPT: {q_receipt}\n"
            "# role: prove\n"
            + body.getvalue()
        ).encode("utf-8"))

        def arun(*a):
            return subprocess.run([BASH, "scripts/ci/track_closeout.sh", *a],
                                  capture_output=True, text=True, cwd=str(ar),
                                  env={**os.environ, "TRACK_CLOSEOUT_NOW": "2026-07-07"})

        arun("--check-eval", "docs/tests/quiet-track_closeout_manifest.tsv")
        r_unowned = arun("--apply", "docs/tests/quiet-track_closeout_manifest.tsv")
        q_report = ar / "docs/tests/quiet-track_closeout_report.md"
        check("apply-unowned-active-track-noop",
              "docs/other_active.md" in norm_bytes((ar / "scripts/ci/active_track.txt").read_bytes())
              and (ar / "docs/other_active.md").exists())
        check("apply-unowned-active-track-report",
              q_report.exists()
              and "active_track_retired: **no**" in norm_bytes(q_report.read_bytes())
              and "current pointer not owned by closing track" in norm_bytes(q_report.read_bytes())
              and "APPLY-VERDICT:" in r_unowned.stdout)

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

        # P0-2: hand-closing the track IN THE SAME PR as the deletion is a bypass -> FAIL;
        # it becomes lawful only when the closeout report and manifest are part of the same diff.
        for t in trk:
            if t["track_id"] == "open-track":
                t["status"] = "open"; t["closed_at"] = "-"
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, inv0)
        write_tsv(gr / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, trk)
        grun("add", "-A"); grun("commit", "-q", "-m", "base3-open")
        base3 = grun("rev-parse", "HEAD").stdout.strip()
        for t in trk:
            if t["track_id"] == "open-track":
                t["status"] = "closed"; t["closed_at"] = "2026-07-07"
        write_tsv(gr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [])
        write_tsv(gr / "scripts/ci/test_lifecycle_tracks.tsv", TRACKS_HEADER, trk)
        grun("add", "-A"); grun("commit", "-q", "-m", "same-pr-close-no-report")
        head3 = grun("rev-parse", "HEAD").stdout.strip()
        r_bypass = subprocess.run([BASH, "scripts/ci/track_closeout.sh", "--deletion-guard", base3, head3],
                                  capture_output=True, text=True, cwd=str(gr), env={**os.environ})
        check("deletion-guard-blocks-same-pr-close",
              "DELETION-GUARD-VERDICT: FAIL unauthorized=1" in r_bypass.stdout)
        (gr / "docs" / "tests").mkdir(parents=True, exist_ok=True)
        (gr / "docs/tests/open-track_closeout_report.md").write_text("# closeout\n", encoding="utf-8")
        grun("add", "-A"); grun("commit", "-q", "-m", "add-closeout-report")
        head4 = grun("rev-parse", "HEAD").stdout.strip()
        r_report_only = subprocess.run([BASH, "scripts/ci/track_closeout.sh", "--deletion-guard", base3, head4],
                                       capture_output=True, text=True, cwd=str(gr), env={**os.environ})
        check("deletion-guard-blocks-report-only-closeout-pr",
              "DELETION-GUARD-VERDICT: FAIL unauthorized=1" in r_report_only.stdout)
        (gr / "docs/tests/open-track_closeout_manifest.tsv").write_text("# manifest\n", encoding="utf-8")
        grun("add", "-A"); grun("commit", "-q", "-m", "add-closeout-manifest")
        head5 = grun("rev-parse", "HEAD").stdout.strip()
        r_lawful = subprocess.run([BASH, "scripts/ci/track_closeout.sh", "--deletion-guard", base3, head5],
                                  capture_output=True, text=True, cwd=str(gr), env={**os.environ})
        check("deletion-guard-allows-closeout-pr-with-manifest",
              "DELETION-GUARD-VERDICT: PASS" in r_lawful.stdout)

    # decommission reaper: deletes only unambiguously-safe expired assets; refuses the rest.
    with tempfile.TemporaryDirectory() as rtmp:
        rr = pathlib.Path(rtmp)
        (rr / "scripts/ci").mkdir(parents=True)
        (rr / "crates/c/tests").mkdir(parents=True)
        (rr / "crates/c/src").mkdir(parents=True)
        (rr / "docs/tests").mkdir(parents=True)
        shutil.copy(SCRIPT_DIR / "track_closeout.sh", rr / "scripts/ci/track_closeout.sh")
        (rr / "crates/c/tests/dead.rs").write_text("#[test]\nfn dead() {}\n", encoding="utf-8")
        (rr / "crates/c/src/live.rs").write_text("#[cfg(test)]\nmod t { #[test] fn u() {} }\n", encoding="utf-8")
        (rr / "docs/tests/old_results.md").write_text("# old\n", encoding="utf-8")
        (rr / "docs/tests/current_evidence_index.md").write_text("# durable\n", encoding="utf-8")
        write_tsv(rr / "scripts/ci/test_inventory.tsv", INVENTORY_HEADER, [])

        def parked(file, test_name, kind, at):
            return {"crate": "c", "file": file, "test_name": test_name, "kind": kind,
                    "class": "behavior-regression", "superseding_boundary": "B-T5", "verdict": "AUDIT",
                    "note": "u", "promotion_target": "permanent-residue:behavior-regression",
                    "birth_track": "pre-lifecycle", "dsu_survivals": "0",
                    "parked_at": at, "closeout_track": "pre-lifecycle", "park_reason": "undecided"}
        write_tsv(rr / "scripts/ci/test_lifecycle_parked.tsv", PARKED_HEADER, [
            parked("crates/c/tests/dead.rs", "dead", "integration", "2026-07-01"),   # expired dedicated -> reap
            parked("crates/c/src/marker.rs", "cfg_test_mod::tests", "unit", "2026-07-01"),  # expired marker -> drop row
            parked("crates/c/src/live.rs", "u", "unit", "2026-07-01"),               # expired inline src -> manual
            parked("crates/c/tests/fresh.rs", "fresh", "integration", "2026-07-18"), # fresh -> kept
        ])
        write_tsv(rr / "scripts/ci/test_lifecycle_parked_boundary.tsv", PARKED_BOUNDARY_HEADER, [
            {"crate": "c", "file": "crates/c/tests/dead.rs", "test_name": "dead", "kind": "integration",
             "current_class": "behavior-regression", "boundary_id": "B", "boundary_tier": "T",
             "recommended_disposition": "", "representative_to_keep": "", "consolidation_target": "",
             "promotion_required": "", "confidence": "high", "note": "",
             "parked_at": "2026-07-01", "closeout_track": "pre-lifecycle"},
            {"crate": "c", "file": "crates/c/src/marker.rs", "test_name": "cfg_test_mod::tests", "kind": "unit",
             "current_class": "deletion-candidate", "boundary_id": "B", "boundary_tier": "T",
             "recommended_disposition": "", "representative_to_keep": "", "consolidation_target": "",
             "promotion_required": "", "confidence": "high", "note": "",
             "parked_at": "2026-07-01", "closeout_track": "pre-lifecycle"},
            {"crate": "c", "file": "crates/c/src/live.rs", "test_name": "u", "kind": "unit",
             "current_class": "behavior-regression", "boundary_id": "B", "boundary_tier": "T",
             "recommended_disposition": "", "representative_to_keep": "", "consolidation_target": "",
             "promotion_required": "", "confidence": "high", "note": "",
             "parked_at": "2026-07-01", "closeout_track": "pre-lifecycle"},
        ])
        write_tsv(rr / "scripts/ci/closeout_artifacts.tsv", ARTIFACT_LEDGER_HEADER, [
            {"path": "crates/c/src/moved.rs", "leased_at": "2026-07-01",
             "disposition": "elevate-code-rehome-pending", "closeout_track": "t", "note": "rehome"},
            {"path": "docs/tests/old_results.md", "leased_at": "2026-07-01",
             "disposition": "lease", "closeout_track": "t", "note": "result doc"},
            {"path": "docs/tests/current_evidence_index.md", "leased_at": "2026-07-01",
             "disposition": "lease", "closeout_track": "t", "note": "not auto-reapable"},
        ])

        def drun(*a, now="2026-07-20"):
            return subprocess.run([BASH, "scripts/ci/track_closeout.sh", *a],
                                  capture_output=True, text=True, cwd=str(rr),
                                  env={**os.environ, "TRACK_CLOSEOUT_NOW": now})

        r_dry = drun("--decommission", "--dry-run")
        check("decommission-dry-verdict", "DECOMMISSION-VERDICT: DRY" in r_dry.stdout)
        check("decommission-dry-noop", (rr / "crates/c/tests/dead.rs").exists())

        r_reap = drun("--decommission")
        check("decommission-reaps-dedicated-file", not (rr / "crates/c/tests/dead.rs").exists())
        check("decommission-reaps-result-doc", not (rr / "docs/tests/old_results.md").exists())
        check("decommission-spares-nonreapable-doc", (rr / "docs/tests/current_evidence_index.md").exists())
        check("decommission-spares-src", (rr / "crates/c/src/live.rs").exists())
        _, pen_after = read_tsv(rr / "scripts/ci/test_lifecycle_parked.tsv")
        names = {r["test_name"] for r in pen_after}
        check("decommission-drops-marker-row", "cfg_test_mod::tests" not in names)
        check("decommission-drops-reaped-row", "dead" not in names)
        check("decommission-keeps-inline-manual", "u" in names)
        check("decommission-keeps-fresh", "fresh" in names)
        _, pen_b_after = read_tsv(rr / "scripts/ci/test_lifecycle_parked_boundary.tsv")
        b_names = {r["test_name"] for r in pen_b_after}
        check("decommission-drops-reaped-boundary-rows",
              "dead" not in b_names and "cfg_test_mod::tests" not in b_names and "u" in b_names)
        _, art_after = read_tsv(rr / "scripts/ci/closeout_artifacts.tsv")
        check("decommission-refuses-rehome-pending",
              any(r["path"] == "crates/c/src/moved.rs" for r in art_after)
              and "moved.rs" in r_reap.stdout)
        check("decommission-refuses-nonreapable-doc",
              any(r["path"] == "docs/tests/current_evidence_index.md" for r in art_after)
              and "current_evidence_index.md" in r_reap.stdout)
        check("decommission-verdict", "DECOMMISSION-VERDICT: OK reaped=2 files=2 manual=3" in r_reap.stdout)

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
    "--decommission": cmd_decommission,
    "--deletion-guard": cmd_deletion_guard,
    "--prove": cmd_prove,
}
sys.exit(DISPATCH[MODE]())
PY
