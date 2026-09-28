"""The ONE definition of what a doctrine anchor's `section` spec selects.

anchor_check hashes this text, anchor_query and `/anchor` render it, and relay_lint checks
ANCHOR-ACK freshness against it. One extractor means the text an agent reads is exactly the
text it acknowledges.

Section specs (the `section` column of doctrine_anchors.tsv):
  heading:<heading>  that heading down to the next `##`-or-deeper heading at the same or a
                     higher level. Subsections stay in; a `#` title spans only its preamble.
  intro:<heading>    only that heading's own text, up to its first subheading. Use it when the
                     subsections carry anchors of their own.
  row:<ID>           the one markdown table row that has a cell reading `ID` (a ladder rung).
  lines:<a>-<b>      a fixed line range. It is fragile under edits; prefer the ones above.

Anchor size is never an admission rule: no anchor is ever narrowed to fit a number (Owner,
2026-09-28, sufficiency over size). Two limits remain, and each bounds a render, not a law:
ANCHOR_REPLY_BYTES is one `/anchor` reply (a GitHub comment holds 65,536 characters including
the report header), so a larger anchor is paged across several comments and flagged as an
advisory; GREP_BUDGET_BYTES bounds what one broad `--grep` renders beyond the foundation.
The advisory keeps the old landmine visible: nine anchors once cited a whole closed-track ladder,
so one query served 365 KB (~92k tokens).
"""
import hashlib
import pathlib
import re

ANCHOR_REPLY_BYTES = 60_000
GREP_BUDGET_BYTES = 60_000
_HEADING = re.compile(r"(#{1,6}) ")


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")


def _level(line: str) -> int:
    match = _HEADING.match(line)
    return len(match.group(1)) if match else 0


def _heading_start(lines, heading: str) -> int:
    start = next(
        (i for i, line in enumerate(lines) if line.strip() == heading or line.strip().startswith(heading)),
        None,
    )
    if start is None:
        raise KeyError(f"missing heading {heading!r}")
    return start


def intro_section(lines, heading: str) -> str:
    start = _heading_start(lines, heading)
    out = [lines[start]]
    for line in lines[start + 1 :]:
        if _level(line) >= 2:
            break
        out.append(line)
    return "\n".join(out).rstrip() + "\n"


def heading_section(lines, heading: str) -> str:
    start = _heading_start(lines, heading)
    deepest_stop = max(_level(lines[start]), 2)
    out = [lines[start]]
    for line in lines[start + 1 :]:
        if 2 <= _level(line) <= deepest_stop:
            break
        out.append(line)
    return "\n".join(out).rstrip() + "\n"


def row_section(lines, row_id: str) -> str:
    cell = f"`{row_id}`"
    rows = [line for line in lines if line.startswith("|") and cell in (c.strip() for c in line.split("|"))]
    if len(rows) != 1:
        raise KeyError(f"row {row_id!r} matches {len(rows)} table rows")
    return rows[0].rstrip() + "\n"


def enclosing_unit(lines, i: int) -> str:
    """The smallest citation around line i: its table row, else its section, else its paragraph.

    `--grep` renders this for a match outside every anchored section, so a term that lives in
    one rung of a long ladder arrives as that rung, never as the whole ladder.
    """
    if lines[i].startswith("|"):
        return lines[i].rstrip() + "\n"
    for j in range(i, -1, -1):
        if _level(lines[j]):
            section = heading_section(lines[j:], lines[j].strip())
            if len(section.encode("utf-8")) <= GREP_BUDGET_BYTES:
                return section
            break
    start, end = i, i + 1
    while start > 0 and lines[start - 1].strip():
        start -= 1
    while end < len(lines) and lines[end].strip():
        end += 1
    return "\n".join(lines[start:end]).rstrip() + "\n"


def lines_slice(lines, span: str) -> str:
    match = re.fullmatch(r"(\d+)-(\d+)", span)
    if not match:
        raise ValueError(f"bad lines spec: {span}")
    start, end = int(match.group(1)), int(match.group(2))
    return "\n".join(lines[start - 1 : end]) + "\n"


def foundation_stamp(hashes) -> str:
    """The one ACK that covers the whole 0.0.8.7 foundation: `ANCHOR-ACK: foundation@<stamp>`.

    `hashes` maps every foundation anchor id to its content hash, so the stamp goes stale on
    any foundation edit exactly as each individual ACK would.
    """
    joined = "|".join(f"{aid}:{hashes[aid]}" for aid in sorted(hashes))
    return hashlib.sha256(joined.encode("utf-8")).hexdigest()[:12]


def extract(path: pathlib.Path, section: str) -> str:
    kind, _, arg = section.partition(":")
    selector = {"heading": heading_section, "intro": intro_section, "row": row_section, "lines": lines_slice}.get(kind)
    if selector is None:
        raise ValueError(f"unsupported section spec: {section}")
    return selector(normalize_text(path.read_bytes()).splitlines(), arg)
