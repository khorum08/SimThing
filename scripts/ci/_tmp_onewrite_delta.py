#!/usr/bin/env python3
"""HU-INVENTORY-ONEWRITE-0: pure derivation attempt + delta report (no force-fit)."""
from __future__ import annotations

import csv
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INV = ROOT / "scripts/ci/test_inventory.tsv"
BR = ROOT / "scripts/ci/test_lifecycle_boundary_rows.tsv"
BND = ROOT / "scripts/ci/test_lifecycle_boundaries.tsv"

INV_HEADER = [
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
BR_HEADER = [
    "crate",
    "file",
    "test_name",
    "kind",
    "current_class",
    "boundary_id",
    "boundary_tier",
    "recommended_disposition",
    "representative_to_keep",
    "consolidation_target",
    "promotion_required",
    "confidence",
    "note",
]


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def is_never_pare_inventory(row: dict[str, str]) -> bool:
    return (
        row["kind"] in {"compile_fail", "trybuild"}
        or row["class"]
        in {
            "seal-proof",
            "oracle-parity",
            "golden-byte",
            "invariant-required",
            "stead-required",
        }
        or row["test_name"] == "custom_layout_ethics_axis"
    )


# Best-effort canned notes by (class, boundary_id) — modal note from live table where uniform enough.
# Used only for delta analysis, not as force-fit authority.
CLASS_NOTE_FALLBACK = {
    "seal-proof": "seal proof fixture/test remains protected",
    "oracle-parity": "CPU oracle/parity proof remains protected",
    "golden-byte": "golden-byte / determinism proof remains protected",
    "stead-required": "STEAD-required proof remains protected",
    "behavior-regression": "owned behavior regression retained unless a future stronger boundary is named",
    "dependency-floor": "dependency-floor remains protected",
}


def pure_derive_row(inv: dict[str, str], policy: dict[str, str]) -> dict[str, str]:
    """Derive boundary-row fields from inventory + policy only (no free-text carry)."""
    tier = policy["boundary_tier"]
    # Disposition from tier / never-pare inventory heuristics (mirrors boundary_check spirit).
    if is_never_pare_inventory(inv) or tier == "TIER7_NEVER_PARE":
        disp = "NEVER_PARE"
        promo = ""
        conf = "high"
    elif tier == "TIER6_PROMOTION_REQUIRED":
        disp = "KEEP"
        promo = "promotion-target:protected-oracle-review"
        conf = "medium"
    else:
        disp = "KEEP"
        promo = ""
        conf = "high"
    note = CLASS_NOTE_FALLBACK.get(inv["class"], f"derived from {inv['superseding_boundary']}")
    # compile_fail special modal note on seal-proof
    if inv["kind"] in {"compile_fail", "trybuild"}:
        note = "compile_fail/trybuild proof is sentinel-core never-pare"
    return {
        "crate": inv["crate"],
        "file": inv["file"],
        "test_name": inv["test_name"],
        "kind": inv["kind"],
        "current_class": inv["class"],
        "boundary_id": inv["superseding_boundary"],
        "boundary_tier": tier,
        "recommended_disposition": disp,
        "representative_to_keep": "",
        "consolidation_target": "",
        "promotion_required": promo,
        "confidence": conf,
        "note": note,
    }


def main() -> int:
    inv_rows = read_tsv(INV)
    br_rows = read_tsv(BR)
    policies = {r["boundary_id"]: r for r in read_tsv(BND)}

    br_by = {(r["crate"], r["file"], r["test_name"], r["kind"]): r for r in br_rows}

    inv_policy = [r for r in inv_rows if r["superseding_boundary"] in policies]
    inv_nonpolicy = [r for r in inv_rows if r["superseding_boundary"] not in policies]

    derived = []
    for r in inv_policy:
        derived.append(pure_derive_row(r, policies[r["superseding_boundary"]]))
    der_by = {(r["crate"], r["file"], r["test_name"], r["kind"]): r for r in derived}

    only_committed = sorted(set(br_by) - set(der_by))
    only_derived = sorted(set(der_by) - set(br_by))
    both = sorted(set(br_by) & set(der_by))

    field_mismatches: list[tuple] = []
    for k in both:
        a, b = br_by[k], der_by[k]
        diffs = {f: (a[f], b[f]) for f in BR_HEADER if a[f] != b[f]}
        if diffs:
            field_mismatches.append((k, diffs))

    # Classify field mismatches
    note_only = sum(1 for _, d in field_mismatches if set(d) == {"note"})
    conf_note = sum(1 for _, d in field_mismatches if set(d) <= {"note", "confidence"})
    structural = [
        (k, d)
        for k, d in field_mismatches
        if set(d) - {"note", "confidence", "promotion_required", "recommended_disposition"}
    ]

    print("=== HU-INVENTORY-ONEWRITE-0 STOP DELTA ===")
    print(f"inventory rows:                 {len(inv_rows)}")
    print(f"committed boundary_rows:        {len(br_rows)}")
    print(f"policy table (B-T*):            {len(policies)}")
    print(f"inventory with policy boundary: {len(inv_policy)}")
    print(f"inventory non-policy boundary:  {len(inv_nonpolicy)}")
    print(f"pure-derived rows:              {len(derived)}")
    print()
    print(f"in committed NOT pure-derived:  {len(only_committed)}")
    print(f"in pure-derived NOT committed:  {len(only_derived)}")
    print(f"intersection with field diffs:  {len(field_mismatches)}")
    print(f"  note-only diffs:              {note_only}")
    print(f"  note+confidence only:         {conf_note}")
    print(f"  structural field diffs:       {len(structural)}")
    print()
    print("--- non-policy superseding_boundary (no B-T* policy row) ---")
    for bid, n in Counter(r["superseding_boundary"] for r in inv_nonpolicy).most_common():
        print(f"  {n:4d}  {bid}")
    print()
    print("--- pure-derived NOT in committed (would ADD on regen) ---")
    add_by_b = Counter(der_by[k]["boundary_id"] for k in only_derived)
    for bid, n in add_by_b.most_common():
        print(f"  {n:4d}  {bid}")
    print(f"  samples ({min(8, len(only_derived))}):")
    for k in only_derived[:8]:
        print(f"    {k[0]} | {k[1]} | {k[2]}")
    print()
    print("--- intersection field-diff summary (would REWRITE on regen) ---")
    field_hit = Counter()
    for _, d in field_mismatches:
        for f in d:
            field_hit[f] += 1
    for f, n in field_hit.most_common():
        print(f"  {n:4d}  field {f}")
    print()
    print("--- structural mismatches (not just note/confidence) ---")
    for k, d in structural[:15]:
        print(f"  {k[1]}::{k[2]}")
        for f, (old, new) in d.items():
            if f in ("note",):
                print(f"    {f}: {old[:60]!r} -> {new[:60]!r}")
            else:
                print(f"    {f}: {old!r} -> {new!r}")
    print()
    print("--- note variance within committed (proves free-text not policy-keyed) ---")
    notes = Counter()
    for r in br_rows:
        notes[(r["current_class"], r["boundary_id"], r["note"])] += 1
    by_cb: dict[tuple[str, str], int] = Counter()
    for (c, b, _n), n in notes.items():
        by_cb[(c, b)] += 1
    multi = [(k, v) for k, v in by_cb.items() if v > 1]
    multi.sort(key=lambda x: -x[1])
    for (c, b), v in multi[:10]:
        print(f"  {v} distinct notes for class={c} boundary={b}")
    print()
    if only_committed or only_derived or field_mismatches:
        print("VERDICT: NO-OP REGEN IMPOSSIBLE — STOP (do not force-fit)")
        return 1
    print("VERDICT: pure regen is no-op")
    return 0


if __name__ == "__main__":
    sys.exit(main())
