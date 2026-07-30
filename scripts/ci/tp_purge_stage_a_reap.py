#!/usr/bin/env python3
"""TP-PURGE-0 Stage A: paired reap of PAIR-REAP + invariant=NONE rows.

Removes inventory rows and matching #[test] functions (or fixture files).
Never touches non-test production functions. Never renames in place of deleting.
"""
from __future__ import annotations

import csv
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
PROPOSAL = ROOT / "docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv"
STAGE_B_TARGETS = ROOT / "docs/tests/tp_purge_0_stage_b_replace_targets.tsv"
INV = ROOT / "scripts/ci/test_inventory.tsv"
REPORT = ROOT / "docs/tests/tp_purge_0_stage_a_reap_report.tsv"


def stage_a_rows() -> list[dict[str, str]]:
    rows = list(csv.DictReader(PROPOSAL.open(encoding="utf-8"), delimiter="\t"))
    return [r for r in rows if r["disposition"] == "PAIR-REAP" or r["invariant"] == "NONE"]


def stage_b_rows() -> list[dict[str, str]]:
    return list(csv.DictReader(STAGE_B_TARGETS.open(encoding="utf-8"), delimiter="\t"))


def remove_rust_fn(src: str, name: str) -> tuple[str, bool]:
    """Remove a #[test] fn <name> including leading attrs/docs. Never touches non-tests."""
    pattern = re.compile(
        rf"(?m)^(?P<indent>[ \t]*)(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+{re.escape(name)}\s*[(<]"
    )
    matches = list(pattern.finditer(src))
    if not matches:
        return src, False

    chosen = None
    item_start = None
    for m in matches:
        start = m.start()
        indent = m.group("indent")
        line_start = src.rfind("\n", 0, start) + 1
        prefix = src[:line_start]
        lines = prefix.splitlines(keepends=True)
        keep_upto = len(lines)
        attrs: list[str] = []
        while keep_upto > 0:
            raw = lines[keep_upto - 1]
            stripped = raw.strip()
            if stripped == "":
                keep_upto -= 1
                continue
            if stripped.startswith("///") or stripped.startswith("//"):
                if len(raw) - len(raw.lstrip(" \t")) < len(indent) and stripped.startswith("//"):
                    break
                keep_upto -= 1
                continue
            if stripped.startswith("#["):
                attrs.append(stripped)
                keep_upto -= 1
                continue
            break
        if not any(
            a.startswith("#[test") or a.startswith("#[tokio::test") or "::test" in a for a in attrs
        ):
            continue
        chosen = m
        item_start = sum(len(x) for x in lines[:keep_upto])
        break
    if chosen is None or item_start is None:
        return src, False
    m = chosen

    i = m.end()
    while i < len(src) and src[i] != "{":
        if src[i] == ";":
            item_end = i + 1
            if item_end < len(src) and src[item_end] == "\n":
                item_end += 1
            return src[:item_start] + src[item_end:], True
        i += 1
    if i >= len(src) or src[i] != "{":
        return src, False

    depth = 0
    in_str = None
    escape = False
    j = i
    while j < len(src):
        ch = src[j]
        if in_str:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == in_str:
                in_str = None
        else:
            if ch == "/" and j + 1 < len(src) and src[j + 1] == "/":
                nl = src.find("\n", j)
                j = len(src) if nl < 0 else nl
                continue
            if ch == "/" and j + 1 < len(src) and src[j + 1] == "*":
                end = src.find("*/", j + 2)
                j = len(src) if end < 0 else end + 1
                continue
            if ch == "'" and j + 1 < len(src) and (src[j + 1].isalpha() or src[j + 1] == "_"):
                j += 1
                while j < len(src) and (src[j].isalnum() or src[j] == "_"):
                    j += 1
                continue
            if ch == '"':
                in_str = '"'
            elif ch == "'":
                in_str = "'"
            elif ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    item_end = j + 1
                    if item_end < len(src) and src[item_end] == "\n":
                        item_end += 1
                    if item_end < len(src) and src[item_end : item_end + 1] == "\n":
                        item_end += 1
                    return src[:item_start] + src[item_end:], True
        j += 1
    return src, False


def file_is_only_reaped_tests(path: pathlib.Path, names: set[str]) -> bool:
    text = path.read_text(encoding="utf-8")
    test_fns = set()
    for m in re.finditer(
        r"(?ms)^[ \t]*#\[(?:\w+::)*test[^\]]*\][ \t]*\n(?:[ \t]*#\[[^\]]+\][ \t]*\n)*[ \t]*(?:async\s+)?fn\s+(\w+)\s*",
        text,
    ):
        test_fns.add(m.group(1))
    if not test_fns:
        return False
    return test_fns <= names


def still_has_test_fn(text: str, name: str) -> bool:
    return bool(
        re.search(
            rf"(?ms)#\[(?:\w+::)*test[^\]]*\][ \t]*\n(?:[ \t]*#\[[^\]]+\][ \t]*\n)*[ \t]*(?:async\s+)?fn\s+{re.escape(name)}\s*",
            text,
        )
    )


def reap_sources(by_file: dict[str, set[str]]) -> tuple[list[dict[str, str]], list[tuple[str, str]], list[str]]:
    report: list[dict[str, str]] = []
    missing_fn: list[tuple[str, str]] = []
    deleted_files: list[str] = []
    for rel, names in sorted(by_file.items()):
        path = ROOT / rel
        if not path.exists():
            for n in sorted(names):
                report.append({"file": rel, "test_name": n, "action": "MISSING-FILE", "ok": "0"})
            continue
        if ("/tests/" in rel.replace("\\", "/") or rel.endswith("_fixture.rs")) and file_is_only_reaped_tests(
            path, names
        ):
            path.unlink()
            deleted_files.append(rel)
            for n in sorted(names):
                report.append({"file": rel, "test_name": n, "action": "DELETE-FILE", "ok": "1"})
            continue
        text = path.read_text(encoding="utf-8")
        new = text
        for name in sorted(names):
            new2, ok = remove_rust_fn(new, name)
            if ok:
                report.append({"file": rel, "test_name": name, "action": "REMOVE-FN", "ok": "1"})
                new = new2
            elif still_has_test_fn(new, name):
                report.append({"file": rel, "test_name": name, "action": "FN-NOT-FOUND", "ok": "0"})
                missing_fn.append((rel, name))
            else:
                # Fixture marker / already gone / non-test identity — inventory pair is enough.
                report.append({"file": rel, "test_name": name, "action": "NO-TEST-FN", "ok": "1"})
        if new != text:
            path.write_text(new, encoding="utf-8", newline="\n")
    return report, missing_fn, deleted_files


def main() -> int:
    source_only = len(sys.argv) > 1 and sys.argv[1] == "--source-only"
    stage_b = len(sys.argv) > 1 and sys.argv[1] == "--stage-b"
    if stage_b:
        targets = stage_b_rows()
        report_path = ROOT / "docs/tests/tp_purge_0_stage_b_reap_report.tsv"
    else:
        targets = stage_a_rows()
        report_path = REPORT
    by_file: dict[str, set[str]] = {}
    keys = set()
    for r in targets:
        by_file.setdefault(r["file"], set()).add(r["test_name"])
        keys.add((r["crate"], r["file"], r["test_name"], r["kind"]))

    report, missing_fn, deleted_files = reap_sources(by_file)

    removed = 0
    if not source_only:
        inv_rows = list(csv.DictReader(INV.open(encoding="utf-8"), delimiter="\t"))
        header = list(inv_rows[0].keys()) if inv_rows else []
        kept = []
        for r in inv_rows:
            key = (r["crate"], r["file"], r["test_name"], r["kind"])
            if key in keys:
                removed += 1
                continue
            kept.append(r)
        with INV.open("w", encoding="utf-8", newline="") as fh:
            w = csv.DictWriter(fh, fieldnames=header, delimiter="\t", lineterminator="\n")
            w.writeheader()
            w.writerows(kept)

    with report_path.open("w", encoding="utf-8", newline="") as fh:
        fields = ["file", "test_name", "action", "ok"]
        w = csv.DictWriter(fh, fieldnames=fields, delimiter="\t", lineterminator="\n")
        w.writeheader()
        w.writerows(report)

    print(f"mode={'stage-b' if stage_b else ('source-only' if source_only else 'stage-a')}")
    print(f"targets={len(targets)}")
    print(f"inventory_removed={removed}")
    print(f"files_deleted={len(deleted_files)}")
    print(f"fn_not_found={len(missing_fn)}")
    for rel, name in missing_fn[:30]:
        print(f"  MISSING #[test] fn {name} in {rel}")
    print(f"report={report_path}")
    return 0 if not missing_fn else 1


if __name__ == "__main__":
    sys.exit(main())
