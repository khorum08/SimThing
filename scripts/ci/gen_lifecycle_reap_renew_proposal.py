#!/usr/bin/env python3
"""Stage-A2 read-only invariant split under Detachability / Invariant Set law.

Outputs:
  docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv
  docs/tests/tp_purge_successor_census_2026_08_11.tsv

No reap/renew execution. Corpus/fixture/generator coupling never validates survival.
"""
from __future__ import annotations

import csv
import datetime
import pathlib
import re
from collections import Counter

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv"
TP_OUT = ROOT / "docs/tests/tp_purge_successor_census_2026_08_11.tsv"
INV = ROOT / "scripts/ci/test_inventory.tsv"
TRACKS = ROOT / "scripts/ci/test_lifecycle_tracks.tsv"

GRACE = 5
RUNWAY = "2026-08-11"
DSU_RE = re.compile(r"downstream-utility:\s*(.*)", re.I | re.S)

INVARIANTS = (
    "conservation",
    "determinism",
    "cpu-gpu-parity",
    "boundedness",
    "admission-totality",
    "residency-typing",
    "NONE",
)

ENGINE_CRATES = {
    "simthing-core",
    "simthing-spec",
    "simthing-kernel",
    "simthing-sim",
    "simthing-gpu",
    "simthing-feeder",
    "simthing-driver",
}

LIVE_ANCHOR_GUARD_KEYS = {
    (
        "simthing-driver",
        "crates/simthing-driver/src/child_share_eml.rs",
        "child_share_cpu_zero_weight_is_zero_not_nan",
        "unit",
    ),
    (
        "simthing-driver",
        "crates/simthing-driver/tests/phase_m_c0_m4_atlas_protocol_oracle.rs",
        "c0_mapping_profile_default_remains_disabled",
        "integration",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/src/property_expiry.rs",
        "cpu_decay_keeps_registry_live_when_sibling_still_has_property",
        "unit",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/tests/c8b_intensity_eml_parity.rs",
        "c8b_intensity_runs_after_velocity_before_overlay",
        "integration",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/tests/c8c_transfer_accumulator_parity.rs",
        "c8c_conjunctive_transfer_min_across_inputs",
        "integration",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/tests/c8d_emission_accumulator_parity.rs",
        "c8d_mismatched_registration_tree_id_rejected",
        "integration",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/tests/protected_representative_restore.rs",
        "assert_no_hard_trigger_on_soft_aggregate",
        "integration",
    ),
    (
        "simthing-sim",
        "crates/simthing-sim/tests/protected_representative_restore.rs",
        "clone_capability_children",
        "integration",
    ),
}
LIVE_ANCHOR_MARKERS = (
    "invariants.md",
    "stead_spatial_contract",
    "stead_spatial_contract.md",
)

# Explicit Stage-A2 overrides (Remand 4 named identities).
SPECIAL = {
    "s6_threshold_events_match_cpu_golden": {
        "invariant": "cpu-gpu-parity",
        "input_shape": "inline-constructed",
        "disposition": "RENEW-INVARIANT",
        "consumer": "simthing-kernel::WorldGpuState::dispatch_accumulator_threshold_scan / AccumulatorOpSession threshold path",
        "falsifier": (
            "planted defect: drop prepare_threshold_scan (and/or finish_threshold_scan) on "
            "WorldGpuState::dispatch_accumulator_threshold_scan so n_ops=0 and GPU events=0 "
            "while the CPU golden still expects 1 -- must FAIL"
        ),
        "reason": (
            "direct-drive DimensionRegistry+WorldGpuState; proves CPU/GPU parity for the "
            "accepted 5.3c production repair; not TP-coupled"
        ),
    },
    "canonical_tp_gpu_table_matches_admission_totality": {
        "invariant": "admission-totality",
        "input_shape": "TP/corpus-coupled",
        "disposition": "TP-PURGE-SUCCESSOR",
        "consumer": "",
        "falsifier": (
            "catches: ordinary TP field-bearing install losing observation-table totality "
            "or inventing repeated (thing,property) loci -- retain as transition-only 5.3c "
            "rename artifact (dsu_survivals=0); mandatory paired reap/replace in TP purge"
        ),
        "reason": (
            "authorized rename retained; TP hydrate/field-bearing install cannot remain a "
            "substrate gate -- successor must re-home admission totality onto inline input"
        ),
    },
    "picker_0_no_duplicate_parse_or_rebind_path": {
        "invariant": "NONE",
        "input_shape": "TP/corpus-coupled",
        "disposition": "TP-PURGE-SUCCESSOR",
        "consumer": "",
        "falsifier": "NONE (source-string/location grep; not an Invariant Set proof)",
        "reason": "TP studio picker referee; does not name a substrate invariant; not blocking 5.3c",
    },
    "picker_0_no_gamemode_rf_live_run_closeout": {
        "invariant": "NONE",
        "input_shape": "TP/corpus-coupled",
        "disposition": "TP-PURGE-SUCCESSOR",
        "consumer": "",
        "falsifier": "NONE (source-string/location grep; not an Invariant Set proof)",
        "reason": "TP studio picker referee; does not name a substrate invariant; not blocking 5.3c",
    },
}

INLINE_NAMES = {
    "s6_threshold_events_match_cpu_golden",
}

TP_TOKEN_RE = re.compile(
    r"(?:^|[_/-])(?:tp_|terran|pirate|foundry_valley|clause_picker|canonical_tp|"
    r"clausething|hydrate_scenario|field_bearing|terran_pirate)",
    re.I,
)


def add_days(iso: str, days: int) -> str:
    return (datetime.date.fromisoformat(iso.strip()) + datetime.timedelta(days=days)).isoformat()


def track_reap_due(track: dict[str, str]) -> str:
    closed = (track.get("closed_at") or "").strip()
    due = add_days(closed, GRACE) if closed and closed != "-" else RUNWAY
    return max(due, RUNWAY)


def is_live_anchor(row: dict[str, str]) -> bool:
    key = (row["crate"], row["file"], row["test_name"], row["kind"])
    if key in LIVE_ANCHOR_GUARD_KEYS:
        return True
    note = row.get("note") or ""
    return any(m in note for m in LIVE_ANCHOR_MARKERS)


def is_durable(row: dict[str, str]) -> bool:
    if row.get("kind") in {"compile_fail", "trybuild"}:
        return True
    if is_live_anchor(row):
        return True
    return False


def biting_falsifier(note: str) -> str:
    """Return concrete planted-defect text; empty if note alone is insufficient."""
    text = (note or "").strip()
    lower = text.lower()
    if lower.startswith("catches:") and len(lower.removeprefix("catches:").strip()) >= 40:
        return text
    return ""


def parse_dsu(note: str) -> str | None:
    match = DSU_RE.search(note or "")
    if not match:
        return None
    consumer = match.group(1).strip()
    return consumer or None


def tier_for(n: int) -> tuple[str, str]:
    if n <= 0:
        return ("none", "n/a")
    if n <= 2:
        return ("advisory-audit", "PASS")
    if n == 3:
        return ("rejustify", "INSPECT")
    return ("promotion-evaluation-required", "FAIL")


def scrub(text: str, limit: int = 320) -> str:
    return (text or "").replace("\t", " ").replace("\n", " ")[:limit]


def is_tp_related(row: dict[str, str]) -> bool:
    blob = " ".join(
        [
            row.get("crate", ""),
            row.get("file", ""),
            row.get("test_name", ""),
            row.get("note", ""),
            row.get("birth_track", ""),
        ]
    )
    if "0.0.8.5-terran-pirate" in (row.get("birth_track") or ""):
        return True
    if row["test_name"] in SPECIAL and SPECIAL[row["test_name"]]["disposition"] == "TP-PURGE-SUCCESSOR":
        return True
    return bool(TP_TOKEN_RE.search(blob))


def guess_invariant(row: dict[str, str]) -> str:
    name = row["test_name"].lower()
    note = (row.get("note") or "").lower()
    klass = (row.get("class") or "").lower()
    file = (row.get("file") or "").lower()
    blob = f"{name} {note} {klass} {file}"

    if any(k in blob for k in ("conservation", "rf_conservation", "rf1", "mass imbalance", "balance rf")):
        return "conservation"
    if any(k in blob for k in ("replay", "determin", "golden-byte", "bit-exact", "byte-identical")):
        return "determinism"
    if any(
        k in blob
        for k in (
            "oracle-parity",
            "cpu_golden",
            "gpu_matches_cpu",
            "cpu oracle",
            "cpu/gpu",
            "parity",
        )
    ):
        return "cpu-gpu-parity"
    if any(k in blob for k in ("nan", "inf", "clamp", "bounded", "finite")):
        return "boundedness"
    if any(
        k in blob
        for k in (
            "admission",
            "totality",
            "anchored",
            "unobserved",
            "hostless",
            "observation-table",
        )
    ):
        return "admission-totality"
    if any(
        k in blob
        for k in (
            "compile_fail",
            "trybuild",
            "seal-proof",
            "type-boundary",
            "unrepresentable",
            "typed",
            "pod",
        )
    ):
        return "residency-typing"
    return "NONE"


def guess_input_shape(row: dict[str, str]) -> str:
    name = row["test_name"]
    if name in INLINE_NAMES or (
        name in SPECIAL and SPECIAL[name]["input_shape"] == "inline-constructed"
    ):
        return "inline-constructed"
    if is_tp_related(row):
        return "TP/corpus-coupled"
    file = row.get("file") or ""
    kind = row.get("kind") or ""
    if "fixtures/" in file or file.endswith(".md") or kind == "fixture":
        return "fixture/generator-coupled"
    if kind in {"compile_fail", "trybuild"}:
        return "type-boundary/eliminated"
    # Conservative default: unknown integration proofs are treated as fixture/generator-coupled
    # until an inline replacement is named (Detachability Law).
    if "/src/" in file.replace("\\", "/") and kind == "unit":
        return "inline-constructed"
    return "fixture/generator-coupled"


def default_consumer_for(invariant: str, row: dict[str, str]) -> str:
    name = row["test_name"]
    file = row["file"]
    if name in SPECIAL and SPECIAL[name].get("consumer"):
        return SPECIAL[name]["consumer"]
    if "rf_conservation" in file:
        return "simthing-driver::rf_conservation_oracle / RF-1 conservation path"
    if "write_door_band_delta" in file:
        return "simthing-kernel::BandCrossingDelta write door + boundary remap"
    if "anchor_table_surface" in file or "canonical_anchor_materialization" in file:
        return "simthing-kernel::anchor_table GPU observation surface"
    if "arena_participant_elimination" in file and "rf1" in name:
        return "simthing-driver::sparse RF-1 execute + replay exactness"
    if invariant == "cpu-gpu-parity" and "s6_threshold" in file:
        return "simthing-kernel::WorldGpuState threshold dispatch"
    return parse_dsu(row.get("note", "")) or ""


def classify_row(row: dict[str, str]) -> dict[str, str]:
    name = row["test_name"]
    note = row.get("note", "")
    cur = int((row.get("dsu_survivals") or "0").strip() or "0")
    special = SPECIAL.get(name)

    if special:
        inv = special["invariant"]
        shape = special["input_shape"]
        disposition = special["disposition"]
        consumer = special["consumer"]
        falsifier = special["falsifier"]
        reason = special["reason"]
        proposed = cur
        if disposition == "RENEW-INVARIANT":
            if cur >= 3:
                disposition = "PROMOTION-EVAL"
                reason = "fourth-or-later renewal -- PROMOTION-EVAL (never automatic)"
            else:
                proposed = cur + 1
        return _pack(row, inv, shape, disposition, consumer, falsifier, cur, proposed, reason)

    inv = guess_invariant(row)
    shape = guess_input_shape(row)
    bite = biting_falsifier(note)
    consumer = default_consumer_for(inv, row)

    if is_tp_related(row):
        # Owner-directed: no TP item may renew/promote as substrate proof.
        return _pack(
            row,
            inv if inv != "NONE" else "NONE",
            "TP/corpus-coupled",
            "TP-PURGE-SUCCESSOR",
            "",
            bite or "NONE (TP/corpus-coupled; cannot validate substrate law)",
            cur,
            cur,
            "TP-related identity -- successor paired reap + inline re-home if invariant is real",
        )

    if inv == "NONE":
        return _pack(
            row,
            "NONE",
            shape,
            "PAIR-REAP",
            "",
            bite or "NONE",
            cur,
            cur,
            "no Invariant Set membership -- paired reap (test + inventory row)",
        )

    if shape in {"TP/corpus-coupled", "fixture/generator-coupled"}:
        return _pack(
            row,
            inv,
            shape,
            "REPLACE-INLINE-INVARIANT",
            consumer,
            bite
            or f"NONE -- propose inline-constructed {inv} proof; current coupling cannot validate",
            cur,
            cur,
            "invariant may be real but corpus/fixture/generator coupling never establishes validity",
        )

    if shape == "type-boundary/eliminated":
        return _pack(
            row,
            inv,
            shape,
            "PAIR-REAP" if not bite else "RENEW-INVARIANT",
            consumer if bite else "",
            bite or "NONE",
            cur,
            (cur + 1) if bite and consumer and cur < 3 else cur,
            "type-boundary residue -- renew only with biting falsifier + named consumer",
        )

    # inline-constructed
    if bite and consumer:
        if cur >= 3:
            return _pack(
                row,
                inv,
                shape,
                "PROMOTION-EVAL",
                consumer,
                bite,
                cur,
                cur,
                "fourth-or-later renewal -- PROMOTION-EVAL",
            )
        return _pack(
            row,
            inv,
            shape,
            "RENEW-INVARIANT",
            consumer,
            bite,
            cur,
            cur + 1,
            "inline-constructed Invariant Set proof with biting falsifier + named consumer",
        )

    return _pack(
        row,
        inv,
        shape,
        "PAIR-REAP",
        consumer,
        bite or "NONE",
        cur,
        cur,
        "invariant-shaped but missing biting falsifier and/or lifecycle-legal named consumer",
    )


def _pack(
    row: dict[str, str],
    inv: str,
    shape: str,
    disposition: str,
    consumer: str,
    falsifier: str,
    cur: int,
    proposed: int,
    reason: str,
) -> dict[str, str]:
    assert inv in INVARIANTS
    tier, tverdict = tier_for(proposed if disposition == "RENEW-INVARIANT" else cur)
    return {
        "disposition": disposition,
        "invariant": inv,
        "input_shape": shape,
        "crate": row["crate"],
        "file": row["file"],
        "test_name": row["test_name"],
        "kind": row["kind"],
        "class": row["class"],
        "birth_track": row.get("birth_track", ""),
        "current_dsu_survivals": str(cur),
        "proposed_dsu_survivals": str(proposed),
        "tier": tier,
        "tier_verdict": tverdict,
        "live_named_consumer": scrub(consumer),
        "planted_defect_falsifier": scrub(falsifier),
        "reason": scrub(reason),
    }


def main() -> None:
    inv = list(csv.DictReader(INV.open(encoding="utf-8"), delimiter="\t"))
    tracks = {
        r["track_id"]: r
        for r in csv.DictReader(TRACKS.open(encoding="utf-8"), delimiter="\t")
    }

    # --- lifecycle due set ---
    due_rows: list[dict[str, str]] = []
    for r in inv:
        bt = (r.get("birth_track") or "").strip()
        track = tracks.get(bt)
        if not track or track.get("status") != "closed":
            continue
        if is_durable(r):
            continue
        due = track_reap_due(track)
        if due > RUNWAY:
            continue
        classified = classify_row(r)
        classified["reap_due"] = due
        due_rows.append(classified)

    fields = [
        "disposition",
        "invariant",
        "input_shape",
        "crate",
        "file",
        "test_name",
        "kind",
        "class",
        "birth_track",
        "reap_due",
        "current_dsu_survivals",
        "proposed_dsu_survivals",
        "tier",
        "tier_verdict",
        "live_named_consumer",
        "planted_defect_falsifier",
        "reason",
    ]
    with OUT.open("w", encoding="utf-8", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=fields, delimiter="\t", lineterminator="\n")
        w.writeheader()
        for row in sorted(due_rows, key=lambda x: (x["disposition"], x["invariant"], x["file"], x["test_name"])):
            w.writerow(row)

    # --- TP purge successor census (separate; not conflated with mortalization backlog) ---
    tp_rows: list[dict[str, str]] = []
    for r in inv:
        if not is_tp_related(r):
            continue
        # Focus: identities that currently serve as core/driver structural gates,
        # plus mapeditor TP pickers named by Remand 4, plus any TP birth-track rows.
        crate = r["crate"]
        file = r["file"]
        gateish = (
            crate in ENGINE_CRATES
            or "tp_studio_clause_picker" in file
            or r["test_name"] in SPECIAL
            or "0.0.8.5-terran-pirate" in (r.get("birth_track") or "")
        )
        if not gateish:
            continue
        c = classify_row(r)
        tp_rows.append(
            {
                "disposition": "TP-PURGE-SUCCESSOR",
                "invariant": c["invariant"],
                "input_shape": c["input_shape"],
                "crate": c["crate"],
                "file": c["file"],
                "test_name": c["test_name"],
                "kind": c["kind"],
                "birth_track": c["birth_track"],
                "engine_structural_gate": (
                    "YES"
                    if crate in ENGINE_CRATES
                    or c["test_name"]
                    in {
                        "canonical_tp_gpu_table_matches_admission_totality",
                        "picker_0_no_duplicate_parse_or_rebind_path",
                        "picker_0_no_gamemode_rf_live_run_closeout",
                    }
                    else "NO"
                ),
                "planted_defect_falsifier": c["planted_defect_falsifier"],
                "reason": c["reason"],
            }
        )

    # Always record the two metered engine proof couplings (detachability ceiling).
    coupling_rows = [
        {
            "disposition": "TP-PURGE-SUCCESSOR",
            "invariant": "NONE",
            "input_shape": "TP/corpus-coupled",
            "crate": "simthing-driver",
            "file": "crates/simthing-driver/Cargo.toml",
            "test_name": "dev-dependency:simthing-clausething",
            "kind": "dev-dependency",
            "birth_track": "detachability-law",
            "engine_structural_gate": "YES",
            "planted_defect_falsifier": (
                "proof coupling meter: engine tests reach authoring layer; ceiling=2 may only decrease"
            ),
            "reason": (
                "Detachability Law proof-coupling edge #1 -- TP purge successor must drive ceiling toward 0"
            ),
        },
        {
            "disposition": "TP-PURGE-SUCCESSOR",
            "invariant": "NONE",
            "input_shape": "TP/corpus-coupled",
            "crate": "simthing-driver",
            "file": "crates/simthing-driver/Cargo.toml",
            "test_name": "dev-dependency:simthing-mapeditor",
            "kind": "dev-dependency",
            "birth_track": "detachability-law",
            "engine_structural_gate": "YES",
            "planted_defect_falsifier": (
                "proof coupling meter: engine tests reach authoring layer; ceiling=2 may only decrease"
            ),
            "reason": (
                "Detachability Law proof-coupling edge #2 -- TP purge successor must drive ceiling toward 0"
            ),
        },
    ]
    tp_fields = [
        "disposition",
        "invariant",
        "input_shape",
        "crate",
        "file",
        "test_name",
        "kind",
        "birth_track",
        "engine_structural_gate",
        "planted_defect_falsifier",
        "reason",
    ]
    with TP_OUT.open("w", encoding="utf-8", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=tp_fields, delimiter="\t", lineterminator="\n")
        w.writeheader()
        for row in coupling_rows + sorted(
            tp_rows, key=lambda x: (x["crate"], x["file"], x["test_name"])
        ):
            w.writerow(row)

    d_counts = Counter(r["disposition"] for r in due_rows)
    i_counts = Counter(r["invariant"] for r in due_rows)
    print(f"WROTE {OUT} rows={len(due_rows)}")
    print("disposition_counts", dict(sorted(d_counts.items())))
    print("invariant_counts", dict(sorted(i_counts.items())))
    print(f"WROTE {TP_OUT} rows={len(tp_rows) + len(coupling_rows)} (incl. 2 coupling edges)")
    print("tp_engine_structural_gates", sum(1 for r in tp_rows if r["engine_structural_gate"] == "YES") + 2)


if __name__ == "__main__":
    main()
