"""The ONE definition of what a doctrine anchor's `section` spec selects.

anchor_check hashes this text, anchor_query and `/anchor` render it, and relay_lint checks
ANCHOR-ACK freshness against it. One extractor means the text an agent reads is exactly the
text it acknowledges.

Section specs (the `section` column of doctrine_anchors.tsv):
  heading:<heading>  that heading down to the next `##`-or-deeper heading at the same or a
                     higher level. Subsections stay in; a `#` title spans only its preamble.
  row:<ID>           the one markdown table row that has a cell reading `ID` (a ladder rung).
  lines:<a>-<b>      a fixed line range. It is fragile under edits; prefer the two above.

An anchor is a bounded citation, not a document. MAX_ANCHOR_BYTES admits every section an
agent can read in one sitting, and it fits a single `/anchor` reply comment (GitHub caps a
comment at 65,536 characters). Nine anchors once cited a whole closed-track ladder, so one
query served 365 KB (~92k tokens).
"""
import pathlib
import re

MAX_ANCHOR_BYTES = 40_000
_HEADING = re.compile(r"(#{1,6}) ")


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return raw.decode("utf-8").replace("\r\n", "\n").replace("\r", "\n")


def _level(line: str) -> int:
    match = _HEADING.match(line)
    return len(match.group(1)) if match else 0


def heading_section(lines, heading: str) -> str:
    start = next(
        (i for i, line in enumerate(lines) if line.strip() == heading or line.strip().startswith(heading)),
        None,
    )
    if start is None:
        raise KeyError(f"missing heading {heading!r}")
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


def lines_slice(lines, span: str) -> str:
    match = re.fullmatch(r"(\d+)-(\d+)", span)
    if not match:
        raise ValueError(f"bad lines spec: {span}")
    start, end = int(match.group(1)), int(match.group(2))
    return "\n".join(lines[start - 1 : end]) + "\n"


def extract(path: pathlib.Path, section: str) -> str:
    kind, _, arg = section.partition(":")
    selector = {"heading": heading_section, "row": row_section, "lines": lines_slice}.get(kind)
    if selector is None:
        raise ValueError(f"unsupported section spec: {section}")
    return selector(normalize_text(path.read_bytes()).splitlines(), arg)
