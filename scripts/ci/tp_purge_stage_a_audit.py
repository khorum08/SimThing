#!/usr/bin/env python3
"""TP-PURGE-0 Stage A pair-audit against base→head (remand 5134500978).

For every approved Stage A identity, prove:
  - base inventory row existed
  - head inventory row is absent
  - where a #[test] existed at base, the function or dedicated file is absent at head
  - true ledger-only / non-function identities are explicitly classified
"""
from __future__ import annotations

import csv
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
PROPOSAL = ROOT / "docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv"
REPORT = ROOT / "docs/tests/tp_purge_0_stage_a_reap_report.tsv"


def stage_a_rows() -> list[dict[str, str]]:
    rows = list(csv.DictReader(PROPOSAL.open(encoding="utf-8"), delimiter="\t"))
    return [r for r in rows if r["disposition"] == "PAIR-REAP" or r["invariant"] == "NONE"]


def git_show(ref: str, rel: str) -> str | None:
    try:
        out = subprocess.run(
            ["git", "-C", str(ROOT), "show", f"{ref}:{rel}"],
            capture_output=True,
            check=True,
        )
    except subprocess.CalledProcessError:
        return None
    return out.stdout.decode("utf-8", errors="replace")


def inv_rows(text: str) -> list[dict[str, str]]:
    return list(csv.DictReader(text.splitlines(), delimiter="\t"))


def inv_key(r: dict[str, str]) -> tuple[str, str, str, str]:
    return (r["crate"], r["file"], r["test_name"], r["kind"])


def has_test_fn(src: str, name: str) -> bool:
    return bool(
        re.search(
            rf"(?ms)#\[(?:\w+::)*test[^\]]*\][ \t]*\n(?:[ \t]*#\[[^\]]+\][ \t]*\n)*[ \t]*(?:async\s+)?fn\s+{re.escape(name)}\s*",
            src,
        )
    )


def audit_pairs(base: str, head: str) -> int:
    targets = stage_a_rows()
    base_text = git_show(base, "scripts/ci/test_inventory.tsv")
    head_text = git_show(head, "scripts/ci/test_inventory.tsv")
    if not base_text or not head_text:
        print("FAIL: inventory not resolvable at base/head", file=sys.stderr)
        return 1
    base_inv = {inv_key(r): r for r in inv_rows(base_text)}
    head_inv = {inv_key(r) for r in inv_rows(head_text)}

    report: list[dict[str, str]] = []
    failures = 0
    for t in targets:
        key = (t["crate"], t["file"], t["test_name"], t["kind"])
        file = t["file"]
        name = t["test_name"]
        if key not in base_inv:
            report.append(
                {
                    "file": file,
                    "test_name": name,
                    "action": "BASE-INVENTORY-MISSING",
                    "ok": "0",
                    "class": "FAIL",
                }
            )
            failures += 1
            continue
        if key in head_inv:
            report.append(
                {
                    "file": file,
                    "test_name": name,
                    "action": "HEAD-INVENTORY-PRESENT",
                    "ok": "0",
                    "class": "FAIL",
                }
            )
            failures += 1
            continue

        base_src = git_show(base, file)
        head_src = git_show(head, file)
        if base_src is None:
            report.append(
                {
                    "file": file,
                    "test_name": name,
                    "action": "LEDGER-ONLY-BASE-FILE-ABSENT",
                    "ok": "1",
                    "class": "ledger-only",
                }
            )
            continue

        if has_test_fn(base_src, name):
            if head_src is None:
                report.append(
                    {
                        "file": file,
                        "test_name": name,
                        "action": "DELETE-FILE",
                        "ok": "1",
                        "class": "paired",
                    }
                )
            elif has_test_fn(head_src, name):
                report.append(
                    {
                        "file": file,
                        "test_name": name,
                        "action": "HEAD-TEST-FN-PRESENT",
                        "ok": "0",
                        "class": "FAIL",
                    }
                )
                failures += 1
            else:
                report.append(
                    {
                        "file": file,
                        "test_name": name,
                        "action": "REMOVE-FN",
                        "ok": "1",
                        "class": "paired",
                    }
                )
        else:
            report.append(
                {
                    "file": file,
                    "test_name": name,
                    "action": "LEDGER-ONLY-NO-TEST-FN",
                    "ok": "1",
                    "class": "ledger-only",
                }
            )

    with REPORT.open("w", encoding="utf-8", newline="") as fh:
        fields = ["file", "test_name", "action", "ok", "class"]
        w = csv.DictWriter(fh, fieldnames=fields, delimiter="\t", lineterminator="\n")
        w.writeheader()
        w.writerows(report)

    ok_rows = sum(1 for r in report if r["ok"] == "1")
    fail_rows = sum(1 for r in report if r["ok"] == "0")
    ledger_only = sum(1 for r in report if r["class"] == "ledger-only")
    print(f"targets={len(targets)}")
    print(f"report_rows={len(report)}")
    print(f"ok={ok_rows} fail={fail_rows} ledger_only={ledger_only}")
    print(f"report={REPORT}")
    if len(report) != len(targets):
        print("FAIL: report row count != Stage A union count")
        return 1
    if fail_rows:
        print(f"FAIL: {fail_rows} pair-audit failures")
        return 1
    print("STAGE-A-PAIR-AUDIT-VERDICT: PASS")
    return 0


def main() -> int:
    if len(sys.argv) < 3:
        print(
            "usage: tp_purge_stage_a_audit.py <base> <head>",
            file=sys.stderr,
        )
        return 2
    return audit_pairs(sys.argv[1], sys.argv[2])


if __name__ == "__main__":
    raise SystemExit(main())
