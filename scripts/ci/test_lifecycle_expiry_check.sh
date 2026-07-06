#!/usr/bin/env bash
# CI-LIFECYCLE-BIRTH-TRACK-TRIPWIRE-0: lifecycle expiry tripwire (ledger/text analysis only).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
INVENTORY="${TEST_LIFECYCLE_INVENTORY:-${ROOT}/scripts/ci/test_inventory.tsv}"
TRACKS="${TEST_LIFECYCLE_TRACKS:-${ROOT}/scripts/ci/test_lifecycle_tracks.tsv}"
RESIDUE_CLASSES="${TEST_LIFECYCLE_RESIDUE_CLASSES:-${ROOT}/scripts/ci/test_residue_classes.tsv}"
DSU_TIERS="${TEST_LIFECYCLE_DSU_TIERS:-${ROOT}/scripts/ci/test_lifecycle_dsu_tiers.tsv}"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$ROOT" "$INVENTORY" "$TRACKS" "$RESIDUE_CLASSES" "$DSU_TIERS" "$@"
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
dsu_tiers_path = pathlib.Path(sys.argv[5])
args = sys.argv[6:]

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
    "dsu_survivals",
]
tracks_header = ["track_id", "status", "closed_at", "source", "note"]
dsu_tiers_header = ["min_survivals", "max_survivals", "tier", "verdict", "note"]

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
LIVE_ANCHOR_MARKERS = (
    "invariants.md",
    "stead_spatial_contract",
    "stead_spatial_contract.md",
)
DSU_RE = re.compile(r"downstream-utility:\s*(.*)", re.IGNORECASE | re.DOTALL)
FOOTER_RE = re.compile(
    r"^LIFECYCLE-EXPIRY-VERDICT: (PASS|INSPECT|FAIL) expired=(\d+) audit=(\d+)(?: max_dsu_survivals=(\d+))? mode=(\S+)$",
    re.MULTILINE,
)

LIVE_ANCHOR_GUARD_KEYS = {
    ("simthing-driver", "crates/simthing-driver/src/child_share_eml.rs", "child_share_cpu_zero_weight_is_zero_not_nan", "unit"),
    ("simthing-driver", "crates/simthing-driver/tests/phase_m_c0_m4_atlas_protocol_oracle.rs", "c0_mapping_profile_default_remains_disabled", "integration"),
    ("simthing-sim", "crates/simthing-sim/src/property_expiry.rs", "cpu_decay_keeps_registry_live_when_sibling_still_has_property", "unit"),
    ("simthing-sim", "crates/simthing-sim/tests/c8b_intensity_eml_parity.rs", "c8b_intensity_runs_after_velocity_before_overlay", "integration"),
    ("simthing-sim", "crates/simthing-sim/tests/c8c_transfer_accumulator_parity.rs", "c8c_conjunctive_transfer_min_across_inputs", "integration"),
    ("simthing-sim", "crates/simthing-sim/tests/c8d_emission_accumulator_parity.rs", "c8d_mismatched_registration_tree_id_rejected", "integration"),
    ("simthing-sim", "crates/simthing-sim/tests/protected_representative_restore.rs", "assert_no_hard_trigger_on_soft_aggregate", "integration"),
    ("simthing-sim", "crates/simthing-sim/tests/protected_representative_restore.rs", "clone_capability_children", "integration"),
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


def parse_dsu_consumer(note: str) -> str | None:
    match = DSU_RE.search(note)
    if not match:
        return None
    consumer = match.group(1).strip()
    if not consumer:
        return None
    return consumer


def has_structured_dsu(note: str) -> bool:
    return parse_dsu_consumer(note) is not None


def is_survivor_set_row(row: dict[str, str]) -> bool:
    return row.get("verdict") == "KEEP" and row.get("class") != "dependency-floor"


def is_live_anchor_survivor(row: dict[str, str]) -> bool:
    if row.get("class") != "behavior-regression" or not is_survivor_set_row(row):
        return False
    note = row.get("note", "").strip()
    if not note.lower().startswith("catches:"):
        return False
    lower = note.lower()
    if any(marker in lower for marker in LIVE_ANCHOR_MARKERS):
        return True
    if row.get("promotion_target", "").strip() == "permanent-residue:escaped-bug":
        detail = lower.removeprefix("catches:").strip()
        if len(detail) >= 40:
            return True
    return False


def is_durable(row: dict[str, str]) -> bool:
    if row.get("kind", "") in {"compile_fail", "trybuild"}:
        return True
    if row.get("class", "") in DURABLE_CLASSES:
        return True
    target = row.get("promotion_target", "").strip()
    if target in durable_promotion_targets:
        return True
    if is_live_anchor_survivor(row):
        return True
    return False


def parse_dsu_survivals(value: str) -> int | None:
    text = value.strip()
    if not text:
        return None
    try:
        parsed = int(text)
    except ValueError:
        return None
    if parsed < 0:
        return None
    return parsed


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


def read_dsu_tiers(path: pathlib.Path) -> tuple[list[str] | None, list[dict[str, str]]]:
    if not path.exists():
        return None, []
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        return reader.fieldnames, list(reader)


def emit_footer(
    verdict: str,
    expired: int,
    mode: str,
    *,
    audit: int = 0,
    max_dsu_survivals: int = 0,
) -> None:
    print(
        f"LIFECYCLE-EXPIRY-VERDICT: {verdict} expired={expired} audit={audit} "
        f"max_dsu_survivals={max_dsu_survivals} mode={mode}"
    )


def parse_footer(output: str) -> tuple[str, int, int, int, str] | None:
    matches = FOOTER_RE.findall(output)
    if not matches:
        return None
    verdict, expired_s, audit_s, max_dsu_s, mode = matches[-1]
    max_dsu = int(max_dsu_s) if max_dsu_s else 0
    return verdict, int(expired_s), int(audit_s), max_dsu, mode


def assert_prove_expectations(
    label: str,
    output: str,
    rc: int,
    *,
    expect_verdict: str,
    expect_expired: int,
    expect_mode: str,
    expect_exit: int,
    expect_audit: int | None = None,
    expect_max_dsu_survivals: int | None = None,
) -> list[str]:
    errors: list[str] = []
    if rc != expect_exit:
        errors.append(f"{label}: expected exit {expect_exit} got {rc}")
    parsed = parse_footer(output)
    if parsed is None:
        errors.append(f"{label}: missing footer")
        return errors
    verdict, expired, audit, max_dsu, mode = parsed
    if verdict != expect_verdict:
        errors.append(f"{label}: expected verdict {expect_verdict} got {verdict}")
    if expired != expect_expired:
        errors.append(f"{label}: expected expired={expect_expired} got expired={expired}")
    if expect_audit is not None and audit != expect_audit:
        errors.append(f"{label}: expected audit={expect_audit} got audit={audit}")
    if expect_max_dsu_survivals is not None and max_dsu != expect_max_dsu_survivals:
        errors.append(
            f"{label}: expected max_dsu_survivals={expect_max_dsu_survivals} got max_dsu_survivals={max_dsu}"
        )
    if mode != expect_mode:
        errors.append(f"{label}: expected mode={expect_mode} got mode={mode}")
    return errors


def tier_for_survivals(survivals: int, tiers: list[dict[str, str]]) -> tuple[str, str, str]:
    if survivals <= 0:
        return "first-renewal", "PASS", "first renewal audit"
    for row in tiers:
        min_s = int(row["min_survivals"])
        max_raw = row.get("max_survivals", "").strip()
        max_s = int(max_raw) if max_raw else None
        if survivals >= min_s and (max_s is None or survivals <= max_s):
            tier = row["tier"]
            verdict = row["verdict"]
            if tier == "presumed-stale":
                return tier, verdict, "delete-or-promote unless DA affirmatively renews"
            if tier == "rejustify":
                return tier, verdict, "rejustify with fresh verified named consumer"
            return tier, verdict, "advisory audit"
    return "unknown", "INSPECT", "unmapped dsu_survivals tier"


def schema_check(
    inv_path: pathlib.Path,
    trk_path: pathlib.Path,
    tiers_path: pathlib.Path,
) -> int:
    errors: list[str] = []
    header, rows = read_inventory(inv_path)
    if header != inventory_header:
        errors.append(f"bad inventory header: {header!r}")
    track_header, tracks = read_tracks(trk_path)
    if track_header != tracks_header:
        errors.append(f"bad tracks header: {track_header!r}")
    if not tracks:
        errors.append("empty or missing lifecycle tracks table")
    tier_header, tiers = read_dsu_tiers(tiers_path)
    if tier_header != dsu_tiers_header:
        errors.append(f"bad dsu tiers header: {tier_header!r}")
    if not tiers:
        errors.append("empty or missing dsu tiers table")

    for line_no, row in enumerate(rows, start=2):
        birth_track = row.get("birth_track", "").strip()
        if not birth_track:
            errors.append(f"inventory line {line_no}: empty birth_track")
        elif birth_track not in tracks:
            errors.append(f"inventory line {line_no}: unknown birth_track {birth_track!r}")
        dsu = parse_dsu_survivals(row.get("dsu_survivals", ""))
        if dsu is None:
            errors.append(f"inventory line {line_no}: invalid dsu_survivals {row.get('dsu_survivals')!r}")

    print("LIFECYCLE-EXPIRY SCHEMA CHECK")
    print(f"  inventory rows: {len(rows)}")
    print(f"  lifecycle tracks: {len(tracks)}")
    print(f"  dsu tiers: {len(tiers)}")
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
    justified_audit: list[dict[str, str]] = []
    survivor_expired = 0

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

        note = row.get("note", "")
        if is_durable(row):
            continue
        consumer = parse_dsu_consumer(note)
        if consumer is not None:
            justified_audit.append(
                {
                    "crate": row["crate"],
                    "file": row["file"],
                    "test_name": row["test_name"],
                    "kind": row["kind"],
                    "birth_track": birth_track,
                    "dsu_survivals": row.get("dsu_survivals", "0"),
                    "consumer": consumer,
                }
            )
            continue
        key = (row["crate"], row["file"], row["test_name"], row["kind"])
        expired_candidates.append(key)
        if is_survivor_set_row(row):
            survivor_expired += 1

    print(f"LIFECYCLE-EXPIRY {mode.upper()} CHECK")
    if track_filter is not None:
        print(f"  track: {track_filter}")
    else:
        closed = [tid for tid, t in tracks.items() if t["status"] == "closed"]
        print(f"  closed tracks: {', '.join(sorted(closed))}")
    print(f"  expired candidates: {len(expired_candidates)}")
    print(f"  survivor-set expired: {survivor_expired}")
    print(f"  justified-closed (audit): {len(justified_audit)}")
    if expired_candidates:
        print("  candidates (first 10):")
        for item in expired_candidates[:10]:
            print(f"    {item}")
    if justified_audit:
        print("  justified-closed rows (first 10):")
        for item in justified_audit[:10]:
            print(
                f"    {item['crate']}\t{item['file']}\t{item['test_name']}\t{item['kind']}\t"
                f"{item['birth_track']}\t{item['dsu_survivals']}\t{item['consumer'][:80]}"
            )

    if errors:
        for error in errors:
            print(f"  - {error}")
        emit_footer("FAIL", len(expired_candidates), mode, audit=len(justified_audit))
        return 1

    if expired_candidates:
        emit_footer("INSPECT", len(expired_candidates), mode, audit=len(justified_audit))
        return 0
    emit_footer("PASS", 0, mode, audit=len(justified_audit))
    return 0


def closure_gate_scan(
    inv_path: pathlib.Path,
    trk_path: pathlib.Path,
    tiers_path: pathlib.Path,
    track_id: str,
) -> int:
    errors: list[str] = []
    header, rows = read_inventory(inv_path)
    if header != inventory_header:
        errors.append(f"bad inventory header: {header!r}")
    track_header, tracks = read_tracks(trk_path)
    if track_header != tracks_header:
        errors.append(f"bad tracks header: {track_header!r}")
    if track_id not in tracks:
        errors.append(f"unknown track_id {track_id!r}")
    tier_header, tiers = read_dsu_tiers(tiers_path)
    if tier_header != dsu_tiers_header or not tiers:
        errors.append("missing or invalid dsu tiers table")

    dsu_rows: list[dict[str, str]] = []
    for line_no, row in enumerate(rows, start=2):
        consumer = parse_dsu_consumer(row.get("note", ""))
        if consumer is None:
            continue
        survivals = parse_dsu_survivals(row.get("dsu_survivals", ""))
        if survivals is None:
            errors.append(f"inventory line {line_no}: invalid dsu_survivals")
            continue
        birth_track = row.get("birth_track", "").strip()
        tier, tier_verdict, action = tier_for_survivals(survivals, tiers)
        dsu_rows.append(
            {
                "crate": row["crate"],
                "file": row["file"],
                "test_name": row["test_name"],
                "kind": row["kind"],
                "birth_track": birth_track,
                "dsu_survivals": str(survivals),
                "tier": tier,
                "required_action": action,
                "consumer": consumer,
                "tier_verdict": tier_verdict,
            }
        )

    dsu_rows.sort(key=lambda item: int(item["dsu_survivals"]), reverse=True)
    max_dsu = max((int(item["dsu_survivals"]) for item in dsu_rows), default=0)
    inspect_hits = [row for row in dsu_rows if row["tier_verdict"] == "INSPECT"]

    print("LIFECYCLE-EXPIRY CLOSURE-GATE CHECK")
    print(f"  track: {track_id}")
    print(f"  downstream-utility renewal audit: {len(dsu_rows)}")
    print("  promotion pressure: sanctioned exit from rising DSU debt is promotion, not perpetual renewal")
    if dsu_rows:
        print("  downstream-utility rows (dsu_survivals desc, first 10):")
        for item in dsu_rows[:10]:
            print(
                f"    {item['crate']}\t{item['file']}\t{item['test_name']}\t{item['kind']}\t"
                f"{item['birth_track']}\t{item['dsu_survivals']}\t{item['tier']}\t"
                f"{item['required_action']}\t{item['consumer'][:80]}"
            )

    if errors:
        for error in errors:
            print(f"  - {error}")
        emit_footer("FAIL", 0, "closure-gate", audit=len(dsu_rows), max_dsu_survivals=max_dsu)
        return 1

    if inspect_hits:
        emit_footer("INSPECT", 0, "closure-gate", audit=len(dsu_rows), max_dsu_survivals=max_dsu)
        return 0
    emit_footer("PASS", 0, "closure-gate", audit=len(dsu_rows), max_dsu_survivals=max_dsu)
    return 0


def write_tsv(path: pathlib.Path, header: list[str], rows: list[dict[str, str]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=header, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def prove_cases() -> int:
    failures: list[str] = []
    _, tiers = read_dsu_tiers(dsu_tiers_path)

    def run_case(
        label: str,
        inventory_rows: list[dict[str, str]],
        track_rows: list[dict[str, str]],
        *,
        mode: str,
        track_filter: str | None = None,
        expect_verdict: str,
        expect_expired: int,
        expect_audit: int | None = None,
        expect_max_dsu_survivals: int | None = None,
        tiers_rows: list[dict[str, str]] | None = None,
    ) -> str:
        expect_exit = 0 if expect_verdict in {"PASS", "INSPECT"} else 1
        with tempfile.TemporaryDirectory() as tmp:
            inv = pathlib.Path(tmp) / "inventory.tsv"
            trk = pathlib.Path(tmp) / "tracks.tsv"
            tier_file = pathlib.Path(tmp) / "dsu_tiers.tsv"
            write_tsv(inv, inventory_header, inventory_rows)
            write_tsv(trk, tracks_header, track_rows)
            write_tsv(tier_file, dsu_tiers_header, tiers_rows or tiers)
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                if mode == "schema":
                    rc = schema_check(inv, trk, tier_file)
                elif mode == "closure-gate":
                    rc = closure_gate_scan(inv, trk, tier_file, track_filter or "pre-lifecycle")
                else:
                    rc = scan_expiry(inv, trk, mode=mode, track_filter=track_filter)
            output = buf.getvalue()
            failures.extend(
                assert_prove_expectations(
                    label,
                    output,
                    rc,
                    expect_verdict=expect_verdict,
                    expect_expired=expect_expired,
                    expect_mode=mode,
                    expect_exit=expect_exit,
                    expect_audit=expect_audit,
                    expect_max_dsu_survivals=expect_max_dsu_survivals,
                )
            )
            return output

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
        dsu_survivals: str = "0",
        test_name: str | None = None,
    ) -> dict[str, str]:
        return {
            "crate": "proof-crate",
            "file": "crates/proof-crate/tests/proof.rs",
            "test_name": test_name or f"proof_{birth_track}_{klass}_{dsu_survivals}",
            "kind": kind,
            "class": klass,
            "superseding_boundary": "B-PROOF",
            "verdict": "KEEP",
            "note": note,
            "promotion_target": promotion_target,
            "birth_track": birth_track,
            "dsu_survivals": dsu_survivals,
        }

    run_case(
        "expired-non-durable",
        [row(birth_track="pre-lifecycle")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="INSPECT",
        expect_expired=1,
        expect_audit=0,
        expect_max_dsu_survivals=0,
    )

    run_case(
        "durable-immune",
        [row(birth_track="pre-lifecycle", klass="seal-proof", promotion_target="permanent-residue:seal-proof")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=0,
    )

    run_case(
        "structured-dsu-audited",
        [row(birth_track="pre-lifecycle", note="downstream-utility: feeds proof harness consumer")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=1,
    )

    run_case(
        "bare-dsu-flags",
        [row(birth_track="pre-lifecycle", note="downstream-utility:")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="INSPECT",
        expect_expired=1,
        expect_audit=0,
    )

    run_case(
        "open-track-immune",
        [row(birth_track="open-track")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=0,
    )

    run_case(
        "unknown-birth-track",
        [row(birth_track="missing-track")],
        base_track_rows,
        mode="schema",
        expect_verdict="FAIL",
        expect_expired=0,
        expect_audit=0,
    )

    empty_row = row(birth_track="pre-lifecycle")
    empty_row["birth_track"] = ""
    run_case(
        "empty-birth-track",
        [empty_row],
        base_track_rows,
        mode="schema",
        expect_verdict="FAIL",
        expect_expired=0,
        expect_audit=0,
    )

    run_case(
        "compile-fail-immune",
        [row(birth_track="pre-lifecycle", kind="compile_fail", klass="unknown", promotion_target="")],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=0,
    )

    run_case(
        "track-closeout-inspect",
        [row(birth_track="pre-lifecycle")],
        base_track_rows,
        mode="track-closeout",
        track_filter="pre-lifecycle",
        expect_verdict="INSPECT",
        expect_expired=1,
        expect_audit=0,
    )

    anchor_row = row(
        birth_track="pre-lifecycle",
        note="catches: registry tombstoning regression (invariants.md registry tombstoning row)",
        promotion_target="permanent-residue:escaped-bug",
    )
    run_case(
        "live-anchor-not-flagged",
        [anchor_row],
        base_track_rows,
        mode="scheduled",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=0,
    )

    run_case(
        "closure-gate-dsu-audit",
        [row(birth_track="pre-lifecycle", note="downstream-utility: closure gate consumer", dsu_survivals="2")],
        base_track_rows,
        mode="closure-gate",
        track_filter="pre-lifecycle",
        expect_verdict="PASS",
        expect_expired=0,
        expect_audit=1,
        expect_max_dsu_survivals=2,
    )

    run_case(
        "closure-gate-presumed-stale",
        [row(birth_track="pre-lifecycle", note="downstream-utility: stale consumer", dsu_survivals="5")],
        base_track_rows,
        mode="closure-gate",
        track_filter="pre-lifecycle",
        expect_verdict="INSPECT",
        expect_expired=0,
        expect_audit=1,
        expect_max_dsu_survivals=5,
    )

    cross_track_out = run_case(
        "closure-gate-cross-track",
        [
            row(
                birth_track="pre-lifecycle",
                note="downstream-utility: consumer A",
                dsu_survivals="1",
                test_name="proof_cross_pre_lifecycle",
            ),
            row(
                birth_track="open-track",
                note="downstream-utility: consumer B",
                dsu_survivals="5",
                test_name="proof_cross_open_track",
            ),
        ],
        base_track_rows,
        mode="closure-gate",
        track_filter="pre-lifecycle",
        expect_verdict="INSPECT",
        expect_expired=0,
        expect_audit=2,
        expect_max_dsu_survivals=5,
    )
    if "proof_cross_open_track" not in cross_track_out:
        failures.append("closure-gate-cross-track: missing open-track DSU row")
    if "proof_cross_pre_lifecycle" not in cross_track_out:
        failures.append("closure-gate-cross-track: missing pre-lifecycle DSU row")
    open_pos = cross_track_out.find("proof_cross_open_track")
    pre_pos = cross_track_out.find("proof_cross_pre_lifecycle")
    if open_pos < 0 or pre_pos < 0 or open_pos > pre_pos:
        failures.append("closure-gate-cross-track: expected dsu_survivals=5 row before dsu_survivals=1 row")

    cross_track_meta = (
        "LIFECYCLE-EXPIRY CLOSURE-GATE CHECK\n"
        "  track: pre-lifecycle\n"
        "  downstream-utility renewal audit: 1\n"
        "LIFECYCLE-EXPIRY-VERDICT: INSPECT expired=0 audit=1 max_dsu_survivals=5 mode=closure-gate\n"
    )
    meta_cross_errors = assert_prove_expectations(
        "meta-cross-track-audit-count",
        cross_track_meta,
        rc=0,
        expect_verdict="INSPECT",
        expect_expired=0,
        expect_mode="closure-gate",
        expect_exit=0,
        expect_audit=2,
        expect_max_dsu_survivals=5,
    )
    if not meta_cross_errors:
        failures.append("meta-cross-track-audit-count: prove harness failed to reject audit=1 when audit=2 expected")

    simulated_survivals = 0
    for cycle in range(1, 6):
        simulated_survivals += 1
        tier, tier_verdict, action = tier_for_survivals(simulated_survivals, tiers)
        if cycle <= 2:
            expect_verdict = "PASS"
            expect_action_substr = "advisory"
        elif cycle <= 4:
            expect_verdict = "INSPECT"
            expect_action_substr = "rejustify"
        else:
            expect_verdict = "INSPECT"
            expect_action_substr = "delete-or-promote"
        if expect_action_substr not in action:
            failures.append(
                f"closure-cycle-{cycle}: simulated survivals {simulated_survivals} "
                f"expected action containing {expect_action_substr!r} got {action!r}"
            )
        out = run_case(
            f"closure-cycle-{cycle}",
            [
                row(
                    birth_track="pre-lifecycle",
                    note="downstream-utility: cycle consumer",
                    dsu_survivals=str(simulated_survivals),
                )
            ],
            base_track_rows,
            mode="closure-gate",
            track_filter="pre-lifecycle",
            expect_verdict=expect_verdict,
            expect_expired=0,
            expect_audit=1,
            expect_max_dsu_survivals=simulated_survivals,
        )
        if cycle == 5 and "delete-or-promote" not in out:
            failures.append("closure-cycle-5: output missing delete-or-promote pressure language")

    if inventory_path.exists():
        _, live_rows = read_inventory(inventory_path)
        missing_anchors = []
        for key in LIVE_ANCHOR_GUARD_KEYS:
            match = next(
                (
                    row
                    for row in live_rows
                    if (row["crate"], row["file"], row["test_name"], row["kind"]) == key
                ),
                None,
            )
            if match is None:
                failures.append(f"live-anchor-missing: {key}")
                continue
            if not is_live_anchor_survivor(match):
                failures.append(f"live-anchor-not-durable: {key}")
            buf = io.StringIO()
            with tempfile.TemporaryDirectory() as tmp:
                inv = pathlib.Path(tmp) / "inventory.tsv"
                trk = pathlib.Path(tmp) / "tracks.tsv"
                tier_file = pathlib.Path(tmp) / "dsu_tiers.tsv"
                write_tsv(inv, inventory_header, [match])
                write_tsv(trk, tracks_header, base_track_rows)
                write_tsv(tier_file, dsu_tiers_header, tiers)
                with contextlib.redirect_stdout(buf):
                    rc = scan_expiry(inv, trk, mode="scheduled")
            if rc != 0 or "expired candidates: 1" in buf.getvalue():
                failures.append(f"live-anchor-flagged: {key}")
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            rc = scan_expiry(inventory_path, tracks_path, mode="scheduled")
        output = buf.getvalue()
        if rc != 0 and "INSPECT" in output:
            pass
        survivor_hits = 0
        for row in live_rows:
            if not is_survivor_set_row(row) or row.get("birth_track") != "pre-lifecycle":
                continue
            if is_durable(row):
                continue
            if has_structured_dsu(row.get("note", "")):
                continue
            key = (row["crate"], row["file"], row["test_name"], row["kind"])
            if key in LIVE_ANCHOR_GUARD_KEYS:
                continue
            if row.get("class") == "behavior-regression" and row.get("verdict") == "KEEP":
                survivor_hits += 1
        m = re.search(r"survivor-set expired: (\d+)", output)
        if not m or int(m.group(1)) != 0:
            failures.append(f"live-survivor-set-expired: expected 0 got {m.group(1) if m else 'missing'}")

    meta_output = (
        "LIFECYCLE-EXPIRY SCHEDULED CHECK\n"
        "  closed tracks: pre-lifecycle\n"
        "  expired candidates: 0\n"
        "  survivor-set expired: 1\n"
        "  justified-closed (audit): 0\n"
        "LIFECYCLE-EXPIRY-VERDICT: PASS expired=0 audit=0 max_dsu_survivals=0 mode=scheduled\n"
    )
    meta_errors = assert_prove_expectations(
        "meta-false-green-lane",
        meta_output,
        rc=0,
        expect_verdict="INSPECT",
        expect_expired=1,
        expect_mode="scheduled",
        expect_exit=0,
        expect_audit=0,
    )
    if not meta_errors:
        failures.append("meta-false-green-lane: prove harness failed to reject PASS when INSPECT expected")

    print("LIFECYCLE-EXPIRY PROVE REPORT")
    print("  8 live-anchor guard keys:")
    for key in sorted(LIVE_ANCHOR_GUARD_KEYS):
        print(f"    {key}")
    if failures:
        for failure in failures:
            print(f"  FAIL: {failure}")
        emit_footer("FAIL", len(failures), "prove")
        return 1
    print("  all synthetic and live prove cases passed")
    emit_footer("PASS", 0, "prove")
    return 0


if not args:
    print(
        "usage: test_lifecycle_expiry_check.sh --schema|--track-closeout <track_id>|"
        "--scheduled|--closure-gate <track_id>|--prove",
        file=sys.stderr,
    )
    sys.exit(2)

if args == ["--schema"]:
    sys.exit(schema_check(inventory_path, tracks_path, dsu_tiers_path))
if args == ["--scheduled"]:
    sys.exit(scan_expiry(inventory_path, tracks_path, mode="scheduled"))
if args == ["--prove"]:
    sys.exit(prove_cases())
if len(args) == 2 and args[0] == "--track-closeout":
    sys.exit(scan_expiry(inventory_path, tracks_path, mode="track-closeout", track_filter=args[1]))
if len(args) == 2 and args[0] == "--closure-gate":
    sys.exit(closure_gate_scan(inventory_path, tracks_path, dsu_tiers_path, args[1]))

print(f"unknown arg(s): {' '.join(args)}", file=sys.stderr)
sys.exit(2)
PY
