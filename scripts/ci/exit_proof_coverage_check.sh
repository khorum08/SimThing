#!/usr/bin/env python3
"""EXIT-PROOF-COVERAGE — ladder rows whose scope outgrew their proof.

Recorded because it happened TWICE in one day. Rung 6.1 was amended into the
seam primitive (second carrier, async-as-ordinary, recorded schedule) and its
exit-proof still read only "Events carry stamps; forced observer lag honors
declared backpressure". Rung 6.1b was amended into two halves (CostBand + the
EML instruction set) and its exit-proof covered only the first. In both cases a
coder satisfying the proof LITERALLY would ship a fraction of the scope and be
CORRECT to call it done. Both were caught by a DA reading the row; neither was
caught by anything mechanical.

THE SIGNAL. An amendment block in a description declares NEW SCOPE and is
self-identifying: `**DA AMENDMENT <date>`, `**<HEADING> (Owner-directed <date>`,
`(DA review <date>`, `**BINDS <RUNG-ID>`. A row that gained scope on a date its
exit-proof never mentions is the drift. This is a PARITY check, not a semantic
one: it does not judge whether the proof is good, only whether the proof was
touched when the scope moved.

WHY PARITY IS THE RIGHT TIER. Judging proof adequacy needs a reader. Judging
whether the author EDITED the proof cell when they added scope is mechanical,
and it is exactly the step both drifts skipped. Cheap, falsifiable, no false
sense of semantic coverage.

SCOPE. TODO rows only. A graduated row's proof is settled and its stamp text
legitimately post-dates its amendments -- flagging those would be pure noise.

usage:
  python3 scripts/ci/exit_proof_coverage_check.sh            # check
  python3 scripts/ci/exit_proof_coverage_check.sh --selftest # planted defects
"""
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]
SELF_CHECK_NAME = "exit-proof-coverage"

# Scope-declaring markers. Each carries a date or a rung id we can look for in
# the proof cell.
DATED = re.compile(
    r"(?:DA AMENDMENT|Owner-directed|Owner-approved|Owner mandate|DA review|DA \d{4})[^.]{0,40}?"
    r"(\d{4}-\d{2}-\d{2})"
)
BINDS = re.compile(r"BINDS\s+(?:\d+\.\d+\w*\s+)?`([A-Z][A-Z0-9-]+-\d)`")


def normalize(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")


def ladder_rows(text: str):
    """Yield (rung_num, rung_id, description, exit_proof, status) for ladder rows."""
    for line in text.splitlines():
        if not line.startswith("| "):
            continue
        cells = line.split("|")
        if len(cells) != 8:
            continue
        num, rid, desc, proof, _lane, status = (c.strip() for c in cells[1:7])
        if not re.fullmatch(r"\d+\.\d+\w*", num):
            continue
        yield num, rid, desc, proof, status


def findings_for(text: str):
    out = []
    for num, rid, desc, proof, status in ladder_rows(text):
        if "DA-GRADUATED" in status or "DA-GRADUATED" in proof:
            continue  # settled; stamp text legitimately post-dates amendments
        missing_dates = sorted(
            {d for d in DATED.findall(desc)} - {d for d in DATED.findall(proof)}
            - set(re.findall(r"\d{4}-\d{2}-\d{2}", proof))
        )
        missing_binds = sorted(
            {b for b in BINDS.findall(desc)}
            - {b for b in BINDS.findall(proof)}
            - {b for b in re.findall(r"`([A-Z][A-Z0-9-]+-\d)`", proof)}
        )
        if missing_dates or missing_binds:
            out.append((num, rid, missing_dates, missing_binds))
    return out


def run_check(doc: pathlib.Path) -> int:
    if not doc.is_file():
        print(f"EXIT-PROOF-COVERAGE-VERDICT: FAIL(missing-doc) {doc}")
        return 1
    findings = findings_for(normalize(doc.read_bytes()))
    for num, rid, dates, binds in findings:
        detail = []
        if dates:
            detail.append("scope dated " + ",".join(dates))
        if binds:
            detail.append("BINDS " + ",".join(binds))
        print(f"  - EXIT-PROOF-COVERAGE (inspect): {num} `{rid}` — {'; '.join(detail)} absent from exit-proof")
    if findings:
        print(
            "  note: the description gained scope on a date the exit-proof never mentions. "
            "Either widen the proof or cite the amendment in it — a proof that predates its "
            "own scope lets a coder ship a fraction and be correct."
        )
    print(f"EXIT-PROOF-COVERAGE-VERDICT: INSPECT rows={len(findings)}")
    return 0


ROW = "| {n} | `{r}` | {d} | {p} | Frontier — coder | {s} |"


def selftest() -> int:
    fails = []

    def case(name, rows, want_flagged):
        text = "\n".join(rows)
        got = {f[0] for f in findings_for(text)}
        if got != want_flagged:
            fails.append(f"{name}: flagged={sorted(got)} want={sorted(want_flagged)}")

    # clean: proof cites the same amendment date
    case(
        "clean_dated_parity",
        [ROW.format(n="9.9", r="CLEAN-0", d="**DA AMENDMENT 2026-08-03 — new scope.**",
                    p="proof widened (DA review 2026-08-03) for the new scope.", s="TODO")],
        set(),
    )
    # PLANTED: description amended, proof untouched  (the 6.1 defect)
    case(
        "planted_dated_drift",
        [ROW.format(n="9.8", r="DRIFT-0", d="**DA AMENDMENT 2026-08-03 — second carrier added.**",
                    p="Events carry stamps.", s="TODO")],
        {"9.8"},
    )
    # PLANTED: BINDS declared in scope, absent from proof
    case(
        "planted_binds_drift",
        [ROW.format(n="9.7", r="BDRIFT-0", d="**BINDS 6.0b `SIMTHING-AUTOMATON-INTRINSIC-0`** origin required.",
                    p="unrelated proof text.", s="TODO")],
        {"9.7"},
    )
    # clean: BINDS echoed in the proof
    case(
        "clean_binds_parity",
        [ROW.format(n="9.6", r="BOK-0", d="**BINDS 6.0b `SIMTHING-AUTOMATON-INTRINSIC-0`**",
                    p="routed arrival preserved per `SIMTHING-AUTOMATON-INTRINSIC-0`.", s="TODO")],
        set(),
    )
    # graduated rows are exempt even when drifted
    case(
        "graduated_exempt",
        [ROW.format(n="9.5", r="GRAD-0", d="**DA AMENDMENT 2026-08-03 — scope.**",
                    p="**DA-GRADUATED / merged #1 @ abc** proof.", s="DA-GRADUATED / merged #1 @ abc")],
        set(),
    )
    # non-ladder lines must not parse as rows
    case("ignores_non_rows", ["| Item | State |", "prose line", "| a | b | c |"], set())

    for f in fails:
        print(f"  FAIL {f}")
    print(
        f"EXIT-PROOF-COVERAGE-SELFTEST: {'PASS' if not fails else 'FAIL'} "
        f"(6 cases, 3 planted defects)"
    )
    return 1 if fails else 0


if __name__ == "__main__":
    if "--selftest" in sys.argv:
        sys.exit(selftest())
    docs = sorted((REPO / "docs").glob("design_0_0_8_7*.md"))
    sys.exit(max((run_check(d) for d in docs), default=0))
