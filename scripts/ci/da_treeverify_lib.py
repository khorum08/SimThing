#!/usr/bin/env python3
"""DA treeverify advisor library — advisory profile + lifecycle only."""
from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import pathlib
import re
import sys

WEIGHT_RANK = {"relax": 0, "light": 1, "deep": 2}
RANK_WEIGHT = {0: "relax", 1: "light", 2: "deep"}
PROFILE_LABEL = {"relax": "RELAX", "light": "LIGHT-TREE", "deep": "DEEP-TREE"}
LOAD_BEARING_TRIGGERS = {
    "production-crate",
    "engine-crate",
    "long-lifecycle",
    "horizontal-process",
    "gate-wiring",
}
REQUIRED_HEADER = [
    "rule_id",
    "match_kind",
    "match",
    "weight",
    "trigger",
    "focus_hint",
    "core",
    "status",
    "expires_on",
    "owner_track",
    "note",
]


def load_rules(path: pathlib.Path, active_only: bool = True) -> list[dict[str, str]]:
    if not path.is_file():
        raise FileNotFoundError(str(path))
    header: list[str] | None = None
    rows: list[dict[str, str]] = []
    for i, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if header is None:
            header = parts
            if header != REQUIRED_HEADER:
                raise ValueError(f"header-mismatch:{path}:{i}")
            continue
        if len(parts) < len(header):
            raise ValueError(f"malformed-row:{path}:{i}")
        row = dict(zip(header, parts))
        if active_only and row.get("status", "").strip().lower() != "active":
            continue
        rows.append(row)
    return rows


def cmd_lifecycle(profile: pathlib.Path) -> int:
    today = dt.date.today()
    errors: list[str] = []
    warnings: list[str] = []
    active_non_core = 0
    try:
        # load all statuses
        header = None
        with profile.open(encoding="utf-8") as f:
            for i, line in enumerate(f, 1):
                line = line.rstrip("\n\r")
                if not line.strip() or line.startswith("#"):
                    continue
                parts = line.split("\t")
                if header is None:
                    header = parts
                    if header != REQUIRED_HEADER:
                        errors.append(f"header-mismatch:{profile}:{i}")
                    continue
                if len(parts) < len(header):
                    errors.append(f"malformed-row:{profile}:{i}")
                    continue
                row = dict(zip(header, parts))
                rid = row["rule_id"]
                core = row["core"].strip().upper()
                status = row["status"].strip().lower()
                exp = row["expires_on"].strip()
                weight = row["weight"].strip().lower()
                if weight not in WEIGHT_RANK:
                    errors.append(f"bad-weight:{rid}:{weight}")
                if status not in ("active", "suspended", "retired"):
                    errors.append(f"bad-status:{rid}:{status}")
                if core not in ("YES", "NO"):
                    errors.append(f"bad-core:{rid}:{row['core']}")
                if core == "YES":
                    if exp:
                        try:
                            d = dt.date.fromisoformat(exp)
                            if d < today and status == "active":
                                warnings.append(f"core-expired-still-active:{rid}:{exp}")
                        except ValueError:
                            errors.append(f"bad-expires_on:{rid}:{exp}")
                    continue
                if status == "active":
                    active_non_core += 1
                    if not exp:
                        errors.append(f"non-core-missing-expires_on:{rid}")
                        continue
                    try:
                        d = dt.date.fromisoformat(exp)
                    except ValueError:
                        errors.append(f"bad-expires_on:{rid}:{exp}")
                        continue
                    if d < today:
                        errors.append(
                            f"non-core-expired-must-delete:{rid}:expires_on={exp}:delete-or-retire-row"
                        )
    except FileNotFoundError:
        print("DA-TREEVERIFY-LIFECYCLE-VERDICT: FAIL(missing-profile-tsv)", file=sys.stderr)
        return 1

    if errors:
        for e in errors:
            print(f"DA-TREEVERIFY-LIFECYCLE: {e}", file=sys.stderr)
        print(f"DA-TREEVERIFY-LIFECYCLE-VERDICT: FAIL(errors={len(errors)})", file=sys.stderr)
        return 1
    for w in warnings:
        print(f"DA-TREEVERIFY-LIFECYCLE-WARN: {w}")
    print(f"DA-TREEVERIFY-LIFECYCLE-VERDICT: PASS active_non_core={active_non_core}")
    return 0


def match_rule(path: str, rule: dict[str, str]) -> bool:
    kind = rule["match_kind"].strip()
    pat = rule["match"].strip().replace("\\", "/")
    if kind == "path_exact":
        return path == pat
    if kind == "path_prefix":
        base = pat.rstrip("/")
        return path == base or path.startswith(base + "/")
    if kind == "glob":
        return fnmatch.fnmatch(path, pat) or fnmatch.fnmatch(path, pat.replace("**", "*"))
    return False


def body_field(body: str, name: str) -> str:
    m = re.search(rf"(?im)^{re.escape(name)}\s*:\s*(.+)$", body)
    return m.group(1).strip() if m else ""


def cmd_profile(profile: pathlib.Path, files_path: pathlib.Path, body_path: pathlib.Path | None) -> int:
    files = [
        ln.strip().replace("\\", "/")
        for ln in files_path.read_text(encoding="utf-8").splitlines()
        if ln.strip() and not ln.strip().startswith("#")
    ]
    body = ""
    if body_path is not None and body_path.is_file():
        body = body_path.read_text(encoding="utf-8")

    try:
        rules = load_rules(profile, active_only=True)
    except FileNotFoundError:
        print("DA-TREEVERIFY-PROFILE: DEEP-TREE")
        print("load_bearing: YES")
        print("triggers: missing-profile-tsv")
        print("relax_ok: NO")
        print("escape: none")
        print("authority: advisory-only (not CLEARANCE-VERDICT)")
        return 0
    except ValueError as e:
        print(f"DA-TREEVERIFY-PROFILE: DEEP-TREE")
        print("load_bearing: YES")
        print(f"triggers: profile-error:{e}")
        print("relax_ok: NO")
        print("escape: none")
        print("authority: advisory-only (not CLEARANCE-VERDICT)")
        return 0

    matched: list[tuple[str, dict[str, str]]] = []
    for path in files:
        for rule in rules:
            if match_rule(path, rule):
                matched.append((path, rule))

    max_rank = -1
    triggers: list[str] = []
    focus: list[str] = []
    matched_ids: list[str] = []
    for path, rule in matched:
        w = rule["weight"].strip().lower()
        rank = WEIGHT_RANK.get(w, 1)
        if rank > max_rank:
            max_rank = rank
        trig = rule["trigger"].strip()
        if trig and trig not in triggers:
            triggers.append(trig)
        rid = rule["rule_id"]
        if rid not in matched_ids:
            matched_ids.append(rid)
        if path not in focus:
            focus.append(path)

    expeditionary = body_field(body, "expeditionary").upper() in ("YES", "TRUE", "1")
    novelty = body_field(body, "novelty_claim").upper() in ("YES", "TRUE", "1")
    charter = body_field(body, "expedition_charter") or body_field(body, "novelty_basis")
    until_s = body_field(body, "expeditionary_until")
    escape = "none"
    escape_notes: list[str] = []
    today = dt.date.today()

    if expeditionary or (novelty and max_rank < 0):
        escape = "expeditionary" if expeditionary else "unclassified"
        if not charter:
            print("DA-TREEVERIFY-ESCAPE: FAIL(missing-charter-or-novelty_basis)", file=sys.stderr)
            print("DA-TREEVERIFY-PROFILE: DEEP-TREE")
            print("load_bearing: YES")
            print("triggers: escape-incomplete")
            print("relax_ok: NO")
            print(f"escape: {escape}")
            print("authority: advisory-only (not CLEARANCE-VERDICT)")
            print("note: expeditionary/unclassified requires expedition_charter or novelty_basis")
            return 1
        if expeditionary:
            if not until_s:
                print("DA-TREEVERIFY-ESCAPE: FAIL(missing-expeditionary_until)", file=sys.stderr)
                print("DA-TREEVERIFY-PROFILE: DEEP-TREE")
                print("load_bearing: YES")
                print("triggers: escape-incomplete")
                print("relax_ok: NO")
                print("escape: expeditionary")
                print("authority: advisory-only (not CLEARANCE-VERDICT)")
                print("note: expeditionary_until: YYYY-MM-DD required (time-boxed; anti-abuse)")
                return 1
            try:
                until = dt.date.fromisoformat(until_s)
            except ValueError:
                print("DA-TREEVERIFY-ESCAPE: FAIL(bad-expeditionary_until)", file=sys.stderr)
                print("DA-TREEVERIFY-PROFILE: DEEP-TREE")
                print("load_bearing: YES")
                print("triggers: escape-incomplete")
                print("relax_ok: NO")
                print("escape: expeditionary")
                print("authority: advisory-only (not CLEARANCE-VERDICT)")
                return 1
            if until < today:
                print("DA-TREEVERIFY-ESCAPE: FAIL(expeditionary-expired)", file=sys.stderr)
                print("DA-TREEVERIFY-PROFILE: DEEP-TREE")
                print("load_bearing: YES")
                print("triggers: escape-expired")
                print("relax_ok: NO")
                print("escape: expeditionary")
                print("authority: advisory-only (not CLEARANCE-VERDICT)")
                print(f"note: expeditionary_until {until_s} past; remove flag or re-charter")
                return 1
            escape_notes.append(f"expeditionary_until={until_s}")

    if files and max_rank < 0 and escape == "none":
        max_rank = 2
        triggers.append("unclassified-surface")
        escape = "unclassified"
        escape_notes.append("no-profile-rule-matched; default DEEP-TREE (never silent RELAX)")

    if max_rank < 0:
        max_rank = 0
        triggers.append("empty-diff")

    load_bearing = any(t in LOAD_BEARING_TRIGGERS for t in triggers) or max_rank >= 2

    if escape in ("expeditionary", "unclassified"):
        if load_bearing or any(
            t in ("production-crate", "engine-crate", "long-lifecycle") for t in triggers
        ):
            max_rank = max(max_rank, 2)
            if "expeditionary-load-bearing" not in triggers:
                triggers.append("expeditionary-load-bearing")
            escape_notes.append("escape cannot downgrade load-bearing surfaces")
        else:
            max_rank = max(max_rank, 1)
            if "expeditionary-light-floor" not in triggers:
                triggers.append("expeditionary-light-floor")

    weight = RANK_WEIGHT[max_rank]
    profile_label = PROFILE_LABEL[weight]
    relax_ok = "YES" if weight == "relax" and escape == "none" and not load_bearing else "NO"

    suggested: list[str] = []
    if weight == "deep":
        for p in focus[:8]:
            if p.endswith(".rs") and "/tests/" in p and p.startswith("crates/"):
                crate = p.split("/")[1]
                tname = pathlib.Path(p).stem
                suggested.append(f"cargo test -p {crate} --test {tname}")
            elif p.endswith(".rs"):
                suggested.append(f"read {p}")
        if any(t == "long-lifecycle" for t in triggers):
            suggested.append("bash scripts/ci/clearance_check.sh --selftest")
            suggested.append("bash scripts/ci/da_treeverify.sh --check-lifecycle")
    elif weight == "light":
        suggested.append("confirm named deliverables on changed paths")
        for p in focus[:5]:
            suggested.append(f"read {p}")
    else:
        suggested.append("relay+CI sufficient unless new load-bearing path appears")
        suggested.append("confirm results/exit-proof cells only")

    print(f"DA-TREEVERIFY-PROFILE: {profile_label}")
    print(f"load_bearing: {'YES' if load_bearing else 'NO'}")
    print("triggers: " + (",".join(triggers) if triggers else "none"))
    print(f"relax_ok: {relax_ok}")
    print(f"escape: {escape}")
    print("authority: advisory-only (not CLEARANCE-VERDICT)")
    print("matched_rules: " + (",".join(matched_ids) if matched_ids else "none"))
    print("focus_paths:")
    if focus:
        for p in focus[:20]:
            print(f"  - {p}")
    else:
        print("  - (none)")
    print("suggested_checks:")
    for s in suggested[:12]:
        print(f"  - {s}")
    if escape_notes:
        print("escape_notes: " + "; ".join(escape_notes))
    print(
        "usage: DA runs this before load-bearing graduate/admit; RELAX does not waive CI or clearance"
    )
    return 0


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="da_treeverify_lib")
    sub = p.add_subparsers(dest="cmd", required=True)

    lc = sub.add_parser("lifecycle")
    lc.add_argument("--profile", required=True)

    pr = sub.add_parser("profile")
    pr.add_argument("--profile", required=True)
    pr.add_argument("--files", required=True)
    pr.add_argument("--body", default="")

    args = p.parse_args(argv)
    if args.cmd == "lifecycle":
        return cmd_lifecycle(pathlib.Path(args.profile))
    body = pathlib.Path(args.body) if args.body else None
    return cmd_profile(pathlib.Path(args.profile), pathlib.Path(args.files), body)


if __name__ == "__main__":
    sys.exit(main())
