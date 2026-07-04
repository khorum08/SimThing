#!/usr/bin/env bash
# CI-LIFECYCLE-BIRTH-TRACK-TRIPWIRE-0: lifecycle expiry tripwire (ledger/text analysis only).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"
TRACKS="${ROOT}/scripts/ci/test_lifecycle_tracks.tsv"
RESIDUE_CLASSES="${ROOT}/scripts/ci/test_residue_classes.tsv"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY" "$TRACKS" "$RESIDUE_CLASSES" "$@"
import contextlib
import csv
import io
import pathlib
import re
import sys
import tempfile

root = pathlib.Path(sys.argv[1])
inventory_path = pathlib.Path(sys.argv[2])
tracks_path = pathlib.Path(sys.argv[3])
residue_classes_path = pathlib.Path(sys.argv[4])
args = sys.argv[5:]

inventory_header = [
    "crate",
    "file",
    "test_name",
    "kind",
    "class",
    "superseding_boundary",
    "verdict",
    "note",
    "promotion_target",
    "birth_track",
]
tracks_header = ["track_id", "status", "closed_at", "source", "note"]

DURABLE_CLASSES = {
    "seal-proof",
    "oracle-parity",
    "golden-byte",
    "invariant-required",
    "stead-required",
    "determinism",
    "dependency-floor",
}
DURABLE_PROMOTION_SUFFIXES = {
    "oracle-parity",
    "golden-byte",
    "seal-proof",
    "determinism",
    "stead-required",
    "dependency-floor",
    "doc-named-invariant",
}


def read_residue_durable_targets(path: pathlib.Path) -> set[str]:
    targets: set[str] = set()
    if not path.exists():
        return targets
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        for row in reader:
            target = row.get("promotion_target", "").strip()
            if not target.startswith("permanent-residue:"):
                continue
            suffix = target.removeprefix("permanent-residue:")
            if suffix in DURABLE_PROMOTION_SUFFIXES:
                targets.add(target)
    return targets


durable_promotion_targets = read_residue_durable_targets(residue_classes_path)


def is_durable(row: dict[str, str]) -> bool:
    if row.get("kind", "") in {"compile_fail", "trybuild"}:
        return True
    if row.get("class", "") in DURABLE_CLASSES:
        return True
    target = row.get("promotion_target", "").strip()
    if target in durable_promotion_targets:
        return True
    return False


def has_downstream_utility(note: str) -> bool:
    return "downstream-utility:" in note


def read_inventory(path: pathlib.Path) -> tuple[list[str] | None, list[dict[str, str]]]:
    if not path.exists():
        return None, []
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        return reader.fieldnames, list(reader)


def read_tracks(path: pathlib.Path) -> tuple[list[str] | None, dict[str, dict[str, str]]]:
    if not path.exists():
        return None, {}
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        fieldnames = reader.fieldnames
        tracks = {row["track_id"]: row for row in reader if row.get("track_id")}
    return fieldnames, tracks


FOOTER_RE = re.compile(
    r"^LIFECYCLE-EXPIRY-VERDICT: (PASS|INSPECT|FAIL) expired=(\d+) mode=(\S+)$",
    re.MULTILINE,
)


def emit_footer(verdict: str, expired: int, mode: str) -> None:
    print(f"LIFECYCLE-EXPIRY-VERDICT: {verdict} expired={expired} mode={mode}")


def parse_footer(output: str) -> tuple[str, int, str] | None:
    matches = FOOTER_RE.findall(output)
    if not matches:
        return None
    verdict, expired_s, mode = matches[-1]
    return verdict, int(expired_s), mode


def assert_prove_expectations(
    label: str,
    output: str,
    rc: int,
    *,
    expect_verdict: str,
    expect_expired: int,
    expect_mode: str,
    expect_exit: int,
) -> list[str]:
    errors: list[str] = []
    if rc != expect_exit:
        errors.append(f"{label}: expected exit {expect_exit} got {rc}")
    parsed = parse_footer(output)
    if parsed is None:
        errors.append(f"{label}: missing footer")
        return errors
    verdict, expired, mode = parsed
    if verdict != expect_verdict:
        errors.append(f"{label}: expected verdict {expect_verdict} got {verdict}")
    if expired != expect_expired:
        errors.append(f"{label}: expected expired={expect_expired} got expired={expired}")
    if mode != expect_mode:
        errors.append(f"{label}: expected mode={expect_mode} got mode={mode}")
    return errors


def schema_check(inv_path: pathlib.Path, trk_path: pathlib.Path) -> int:
    errors: list[str] = []
    header, rows = read_inventory(inv_path)
    if header != inventory_header:
        errors.append(f"bad inventory header: {header!r}")
    track_header, tracks = read_tracks(trk_path)
    if track_header != tracks_header:
        errors.append(f"bad tracks header: {track_header!r}")
    if not tracks:
        errors.append("empty or missing lifecycle tracks table")

    for line_no, row in enumerate(rows, start=2):
        birth_track = row.get("birth_track", "").strip()
        if not birth_track:
            errors.append(f"inventory line {line_no}: empty birth_track")
        elif birth_track not in tracks:
            errors.append(f"inventory line {line_no}: unknown birth_track {birth_track!r}")

    print("LIFECYCLE-EXPIRY SCHEMA CHECK")
    print(f"  inventory rows: {len(rows)}")
    print(f"  lifecycle tracks: {len(tracks)}")
    if errors:
        for error in errors:
            print(f"  - {error}")
        emit_footer("FAIL", 0, "schema")
        return 1
    emit_footer("PASS", 0, "schema")
    return 0


def scan_expiry(
    inv_path: pathlib.Path,
    trk_path: pathlib.Path,
    *,
    mode: str,
    track_filter: str | None = None,
) -> int:
    errors: list[str] = []
    header, rows = read_inventory(inv_path)
    if header != inventory_header:
        errors.append(f"bad inventory header: {header!r}")
    track_header, tracks = read_tracks(trk_path)
    if track_header != tracks_header:
        errors.append(f"bad tracks header: {track_header!r}")

    if track_filter is not None:
        if track_filter not in tracks:
            errors.append(f"unknown track_id {track_filter!r}")
        elif tracks[track_filter]["status"] != "closed":
            print(f"LIFECYCLE-EXPIRY {mode.upper()} CHECK")
            print(f"  track: {track_filter} (open — no expiry scan)")
            emit_footer("PASS", 0, mode)
            return 0

    expired_candidates: list[tuple[str, str, str, str]] = []
    for line_no, row in enumerate(rows, start=2):
        birth_track = row.get("birth_track", "").strip()
        if not birth_track:
            errors.append(f"inventory line {line_no}: empty birth_track")
            continue
        track = tracks.get(birth_track)
        if track is None:
            errors.append(f"inventory line {line_no}: unknown birth_track {birth_track!r}")
            continue

        if track_filter is not None and birth_track != track_filter:
            continue
        if track_filter is None and track["status"] != "closed":
            continue

        if is_durable(row):
            continue
        if has_downstream_utility(row.get("note", "")):
            continue
        expired_candidates.append(
            (row["crate"], row["file"], row["test_name"], row["kind"])
        )

    print(f"LIFECYCLE-EXPIRY {mode.upper()} CHECK")
    if track_filter is not None:
        print(f"  track: {track_filter}")
    else:
        closed = [tid for tid, t in tracks.items() if t["status"] == "closed"]
        print(f"  closed tracks: {', '.join(sorted(closed))}")
    print(f"  expired candidates: {len(expired_candidates)}")
    if expired_candidates:
        print("  candidates (first 10):")
        for item in expired_candidates[:10]:
            print(f"    {item}")

    if errors:
        for error in errors:
            print(f"  - {error}")
        emit_footer("FAIL", len(expired_candidates), mode)
        return 1

    if expired_candidates:
        emit_footer("INSPECT", len(expired_candidates), mode)
        return 0
    emit_footer("PASS", 0, mode)
    return 0


def write_tsv(path: pathlib.Path, header: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=header, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def prove_cases() -> int:
    failures: list[str] = []

    def run_case(
        label: str,
        inventory_rows: list[dict[str, str]],
        track_rows: list[dict[str, str]],
        *,
        mode: str,
        track_filter: str | None = None,
        expect_verdict: str,
        expect_expired: int,
    ) -> None:
        expect_exit = 0 if expect_verdict in {"PASS", "INSPECT"} else 1
        with tempfile.TemporaryDirectory() as tmp:
            inv = pathlib.Path(tmp) / "inventory.tsv"
            trk = pathlib.Path(tmp) / "tracks.tsv"
            write_tsv(inv, inventory_header, inventory_rows)
            write_tsv(trk, tracks_header, track_rows)
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                if mode == "schema":
                    rc = schema_check(inv, trk)
                else:
                    rc = scan_expiry(inv, trk, mode=mode, track_filter=track_filter)
            failures.extend(
                assert_prove_expectations(
                    label,
                    buf.getvalue(),
                    rc,
                    expect_verdict=expect_verdict,
                    expect_expired=expect_expired,
                    expect_mode=mode,
                    expect_exit=expect_exit,
                )
            )

    base_track_rows = [
        {
            "track_id": "pre-lifecycle",
            "status": "closed",
            "closed_at": "2026-07-04",
            "source": "legacy",
            "note": "proof fixture",
        },
        {
            "track_id": "open-track",
            "status": "open",
            "closed_at": "-",
            "source": "proof",
            "note": "proof fixture",
        },
    ]

    def row(
        *,
        birth_track: str,
        klass: str = "behavior-regression",
        kind: str = "unit",
        note: str = "catches: synthetic proof fixture row for lifecycle expiry",
        promotion_target: str = "permanent-residue:behavior-regression",
    ) -> dict[str, str]:
        return {
            "crate": "proof-crate",
            "file": "crates/proof-crate/tests/proof.rs",
            "test_name": f"proof_{birth_track}_{klass}",
            "kind": kind,
            "class": klass,
            "superseding_boundary": "B-PROOF",
            "verdict": "KEEP",
            "note": note,
            "promotion_target": promotion_target,
            "birth_track": birth_track,
        }

    # closed-track non-durable without downstream-utility -> INSPECT
    run_case(
        "expired-non-durable",
        [row(birth_track="pre-lifecycle")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="INSPECT",
        expect_expired=1,
    )

    # durable class on closed track -> PASS
    run_case(
        "durable-immune",
        [row(birth_track="pre-lifecycle", klass="seal-proof", promotion_target="permanent-residue:seal-proof")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
    )

    # downstream-utility on closed track non-durable -> PASS
    run_case(
        "downstream-utility-immune",
        [row(birth_track="pre-lifecycle", note="downstream-utility: feeds proof harness only")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
    )

    # open-track non-durable -> PASS
    run_case(
        "open-track-immune",
        [row(birth_track="open-track")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
    )

    # unknown birth_track -> FAIL (schema)
    run_case(
        "unknown-birth-track",
        [row(birth_track="missing-track")],
        base_track_rows,
        mode="schema",
        expect_verdict="FAIL",
        expect_expired=0,
    )

    # empty birth_track -> FAIL (schema)
    empty_row = row(birth_track="pre-lifecycle")
    empty_row["birth_track"] = ""
    run_case(
        "empty-birth-track",
        [empty_row],
        base_track_rows,
        mode="schema",
        expect_verdict="FAIL",
        expect_expired=0,
    )

    # compile_fail kind immune
    run_case(
        "compile-fail-immune",
        [row(birth_track="pre-lifecycle", kind="compile_fail", klass="unknown", promotion_target="")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
    )

    # track-closeout on closed track
    run_case(
        "track-closeout-inspect",
        [row(birth_track="pre-lifecycle")],
        base_track_rows,
        mode="track-closeout",
        track_filter="pre-lifecycle",
        expect_verdict="INSPECT",
        expect_expired=1,
    )

    # meta-proof: harness rejects PASS footer when INSPECT is expected
    meta_output = (
        "LIFECYCLE-EXPIRY SCHEDULED CHECK\n"
        "  closed tracks: pre-lifecycle\n"
        "  expired candidates: 0\n"
        "LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 mode=scheduled\n"
    )
    meta_errors = assert_prove_expectations(
        "meta-false-green-lane",
        meta_output,
        rc=0,
        expect_verdict="INSPECT",
        expect_expired=1,
        expect_mode="scheduled",
        expect_exit=0,
    )
    if not meta_errors:
        failures.append("meta-false-green-lane: prove harness failed to reject PASS when INSPECT expected")

    print("LIFECYCLE-EXPIRY PROVE REPORT")
    if failures:
        for failure in failures:
            print(f"  FAIL: {failure}")
        emit_footer("FAIL", len(failures), "prove")
        return 1
    print("  all synthetic prove cases passed")
    emit_footer("PASS", 0, "prove")
    return 0


if not args:
    print("usage: test_lifecycle_expiry_check.sh --schema|--track-closeout <track_id>|--scheduled|--prove", file=sys.stderr)
    sys.exit(2)

if args == ["--schema"]:
    sys.exit(schema_check(inventory_path, tracks_path))
if args == ["--scheduled"]:
    sys.exit(scan_expiry(inventory_path, tracks_path, mode="scheduled"))
if args == ["--prove"]:
    sys.exit(prove_cases())
if len(args) == 2 and args[0] == "--track-closeout":
    sys.exit(scan_expiry(inventory_path, tracks_path, mode="track-closeout", track_filter=args[1]))

print(f"unknown arg(s): {' '.join(args)}", file=sys.stderr)
sys.exit(2)
PY