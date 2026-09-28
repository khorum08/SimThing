"""The ONE page protocol for everything agents read (Owner, 2026-09-28).

One tool call shows an agent only part of a long output: Codex keeps 10,000 bytes (the first
and last 128 lines), Claude Code 30,000 characters, and Grok documents no limit. So every
agent-facing render (anchor queries, orientation, handoff projections) can be read as verbatim
pages sized for the smallest window:

- a page holds at most PAGE_BYTES bytes and a matching number of lines, header and footer
  included, so it fits Codex's 10,000 bytes and 256 lines;
- pages break between units (an anchor, a section), else before a paragraph, list item or table
  row, else between lines; a line is cut only when it alone overflows a page, and the footer
  says so;
- every page opens `PAGE k/n . <label> . <where it starts>` and ends `PAGE-END k/n sha=<8 hex>`,
  so a page missing its PAGE-END line was truncated; the last page adds `END n/n` and repeats
  the render's foundation ACK line;
- page 0 is the map: every unit's name, size, pages and the verbatim start of its first line.

Pages are never summaries: joining the page bodies gives back the render byte for byte.

usage: python pager.py --page N [--page-size BYTES] [--label TEXT]
                       [--reach-role ROLE --reach-query QUERY]   < render
       python pager.py --note KIND --command CMD [--page-size BYTES] < render
       python pager.py --selftest
"""
import datetime as dt
import hashlib
import os
import pathlib
import re
import sys

PAGE_BYTES = 8_000
_RESERVE = 700  # header, footer and END lines
_ANCHOR = re.compile(r"^--- ([a-z0-9-]+) ---$")
_UNANCHORED = re.compile(r"^--- unanchored: \S+ ---$")
_HEADING = re.compile(r"^#{1,6} ")
_ITEM = re.compile(r"^\s*(?:[-*+]|\d+[.)])\s|^\|")
_META = ("doc: ", "section: ", "content_hash: ", "lifecycle: ")


class PageError(ValueError):
    pass


def page_lines(page_bytes):
    # Codex also cuts at 256 lines; 200 leaves room at the default size, and larger pages
    # (for harnesses with larger windows) scale the line bound with them.
    return max(200, page_bytes // 40)


def units(lines):
    """(first line, name) of each unit. An anchor render's units are its anchors (their inner
    headings stay inside them); any other render's units are its markdown sections."""
    found = [(0, "start")]

    def anchor_at(i):
        # An anchor render opens every anchor with `--- <id> ---` then its `doc:` line; other
        # renders may use the same rule shape as a plain separator.
        match = _ANCHOR.match(lines[i])
        return match if match and i + 1 < len(lines) and lines[i + 1].startswith("doc: ") else None

    anchored = any(anchor_at(i) or _UNANCHORED.match(line) for i, line in enumerate(lines))
    for i, line in enumerate(lines):
        match = anchor_at(i)
        if match:
            detail = " ".join(l.split(": ", 1)[1] for l in lines[i + 1:i + 3] if l.startswith(("doc: ", "section: ")))
            found.append((i, f"anchor {match.group(1)}" + (f" ({detail})" if detail else "")))
        elif _UNANCHORED.match(line):
            found.append((i, "unanchored match " + line[len("--- unanchored: "):-len(" ---")]))
        elif not anchored and _HEADING.match(line):
            found.append((i, line.strip()))
    if len(found) > 1 and found[1][0] == 0:
        found.pop(0)
    return found


def paginate(text, page_bytes=PAGE_BYTES):
    """Split `text` into pages of (line index, text, ends_line) segments, losslessly."""
    lines = text.split("\n")
    ends = [True] * len(lines)
    if lines and lines[-1] == "":
        lines.pop()
        ends.pop()
    elif lines:
        ends[-1] = False  # the render's last line had no newline
    budget_bytes = page_bytes - _RESERVE
    budget_lines = page_lines(page_bytes) - 5
    if budget_bytes < 200:
        raise PageError(f"page size {page_bytes} is too small")
    starts = {i for i, _ in units(lines)}
    pages, current, best = [], [], None

    def size(segments):
        return sum(len(t.encode("utf-8")) + (1 if e else 0) for _, t, e in segments)

    for i, line in enumerate(lines):
        width = len(line.encode("utf-8")) + 1
        if width > budget_bytes:
            # One line alone overflows a page: cut it at character boundaries, never lose a byte.
            if current:
                pages.append(current)
            current, best = [], None
            data, cut = line.encode("utf-8"), 0
            while cut < len(data):
                end = min(cut + budget_bytes - 1, len(data))
                while end < len(data) and (data[end] & 0xC0) == 0x80:
                    end -= 1
                pages.append([(i, data[cut:end].decode("utf-8"), end >= len(data) and ends[i])])
                cut = end
            continue
        if current and (size(current) + width > budget_bytes or len(current) + 1 > budget_lines):
            if best and size(current[:best]) >= budget_bytes // 2:
                pages.append(current[:best])
                current = current[best:]
            else:
                pages.append(current)
                current = []
            best = None
        breakable = i in starts or _HEADING.match(line) or _ITEM.match(line) or (
            i > 0 and line.strip() and not lines[i - 1].strip())
        if breakable and current:
            best = len(current)
        current.append((i, line, ends[i]))
    if current or not pages:
        pages.append(current)
    return pages, lines


def _body(page):
    return "".join(t + ("\n" if e else "") for _, t, e in page)


def _unit_at(unit_list, line_index):
    name = unit_list[0][1]
    for start, unit in unit_list:
        if start > line_index:
            break
        name = unit
    return name


def _map(pages, lines, page_bytes, label, total):
    unit_list = units(lines)
    n = len(pages)
    first_page = {}
    last_page = {}
    for k, page in enumerate(pages, 1):
        for index, _, _ in page:
            name = _unit_at(unit_list, index)
            first_page.setdefault(name, k)
            last_page[name] = k
    rows, anchored_extra = [], []
    for position, (start, name) in enumerate(unit_list):
        end = unit_list[position + 1][0] if position + 1 < len(unit_list) else len(lines)
        body = lines[start:end]
        bytes_ = sum(len(l.encode("utf-8")) + 1 for l in body)
        opener = next((l.strip() for l in body[1:] if l.strip() and not l.startswith(_META)), "")
        if name.startswith("unanchored match "):
            anchored_extra.append((first_page.get(name, 1), last_page.get(name, n), bytes_))
            continue
        rows.append((name, bytes_, first_page.get(name, 1), last_page.get(name, n), opener))
    budget = page_bytes - _RESERVE - 400
    fixed = sum(len(f"- {r[0]} . {r[1]} B . p{r[2]}-{r[3]} . ".encode("utf-8")) + 1 for r in rows)
    room = max(0, (budget - fixed) // max(1, len(rows)))
    lines_out = [
        f"PAGE 0/{n} . MAP . {label}",
        f"{total} bytes in {n} pages of at most {page_bytes} bytes. Read pages 1 to {n} in order; each "
        f"ends with its PAGE-END line, and a page without one was truncated: re-read it with a smaller --page-size.",
    ]
    shown = 0
    for name, bytes_, a, b, opener in rows:
        prefix = opener if len(opener) <= room else opener[:max(0, room - 3)].rstrip() + "..."
        line = f"- {name} . {bytes_} B . p{a}-{b}" + (f" . {prefix}" if room >= 20 and prefix else "")
        if sum(len(l.encode("utf-8")) + 1 for l in lines_out) + len(line.encode("utf-8")) > budget or len(lines_out) >= page_lines(page_bytes) - 8:
            break
        lines_out.append(line)
        shown += 1
    if shown < len(rows):
        rest = rows[shown:]
        lines_out.append(f"- ... and {len(rest)} more units (p{rest[0][2]}-{rest[-1][3]})")
    if anchored_extra:
        lines_out.append(f"- unanchored matches . {len(anchored_extra)} units . {sum(e[2] for e in anchored_extra)} B . "
                         f"p{min(e[0] for e in anchored_extra)}-{max(e[1] for e in anchored_extra)}")
    body = "\n".join(lines_out[1:]) + "\n"
    return "\n".join(lines_out) + f"\nPAGE-END 0/{n} sha={hashlib.sha256(body.encode('utf-8')).hexdigest()[:8]}\n", []


def render_page(text, k, page_bytes=PAGE_BYTES, label="render"):
    """Page `k` of `text` (0 = map) as it is shown to an agent, the page count, and the names of
    the units the page carries."""
    pages, lines = paginate(text, page_bytes)
    n = len(pages)
    if k == 0:
        rendered, names = _map(pages, lines, page_bytes, label, len(text.encode("utf-8")))
        return rendered, n, names
    if not 1 <= k <= n:
        raise PageError(f"page {k} is out of range 0..{n}")
    page = pages[k - 1]
    unit_list = units(lines)
    names = []
    for index, _, _ in page:
        name = _unit_at(unit_list, index)
        if name not in names:
            names.append(name)
    first = page[0][0] if page else len(lines)
    previous_last = pages[k - 2][-1] if k > 1 and pages[k - 2] else None
    continued = (first not in {s for s, _ in unit_list}) or (previous_last is not None and not previous_last[2])
    where = _unit_at(unit_list, first) + (", continued" if continued else "")
    header = f"PAGE {k}/{n} . {label} . {where}"
    if len(header) > 300:
        header = header[:297] + "..."
    body = _body(page)
    cut = page and not page[-1][2]
    footer = f"PAGE-END {k}/{n} sha={hashlib.sha256(body.encode('utf-8')).hexdigest()[:8]}"
    if cut and k < n:
        footer += " . the last line continues on the next page"
    out = header + "\n" + body + ("\n" if cut else "") + footer + "\n"
    if k == n:
        out += f"END {n}/{n}: the whole render has been read\n"
        ack = next((l for l in lines if l.startswith("foundation-ack: ")), None)
        if ack:
            out += ack + "\n"
    return out, n, names


def size_note(text, page_bytes, kind, command):
    """One line telling an agent a render is longer than a page, or None when it fits."""
    total = len(text.encode("utf-8"))
    if total <= page_bytes - _RESERVE:
        return None
    n = len(paginate(text, page_bytes)[0])
    return (f"{kind}-SIZE: {total} bytes = {n} pages of {page_bytes}; if your tool shows less than all of it, "
            f"read it page by page: {command} --page 0 (map), then --page 1 to {n}")


def log_read(role, query, served, hit):
    path = pathlib.Path(os.environ.get("ANCHOR_REACH_LOG_PATH", "").strip()
                        or pathlib.Path(__file__).resolve().parent / "anchor_reach_log.tsv")
    if not path.is_file():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b"date\trole\tquery\tanchors_served\thit\n")
    stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    with path.open("a", encoding="utf-8", newline="\n") as fh:
        fh.write(f"{stamp}\t{role}\t{query}\t{served or 'none'}\t{hit}\n")


def selftest():
    checks = {}
    text = "".join(
        [f"--- anchor-{u} ---\ndoc: docs/x.md\nsection: heading:## U{u}\ncontent_hash: 0\nlifecycle: canonical\n"
         + "".join(f"Paragraph {u}.{p} opens a thought.\n" + "word " * 60 + "\n\n" for p in range(12))
         for u in range(6)]
    ) + "- bullet list item\n" + ("x" * 20000) + "\nfoundation-ack: ANCHOR-ACK: foundation@0123456789ab\n"
    pages, lines = paginate(text)
    checks["pager_is_lossless"] = "".join(_body(p) for p in pages) == text
    rendered = [render_page(text, k)[0] for k in range(1, len(pages) + 1)]
    checks["pager_every_page_fits_codex"] = all(
        len(r.encode("utf-8")) <= PAGE_BYTES and r.count("\n") <= page_lines(PAGE_BYTES) for r in rendered)
    checks["pager_pages_are_framed"] = all(
        r.startswith(f"PAGE {k}/{len(pages)} . ") and f"PAGE-END {k}/{len(pages)} sha=" in r for k, r in enumerate(rendered, 1))
    checks["pager_last_page_ends_with_ack"] = rendered[-1].rstrip().endswith("foundation-ack: ANCHOR-ACK: foundation@0123456789ab") \
        and f"END {len(pages)}/{len(pages)}" in rendered[-1]
    checks["pager_overlong_line_cut_and_flagged"] = any("the last line continues on the next page" in r for r in rendered)
    second = next(r for r in rendered[1:] if " . anchor anchor-" in r.split("\n", 1)[0])
    checks["pager_header_names_the_unit"] = " . anchor anchor-" in second.split("\n", 1)[0]
    page_map = render_page(text, 0)[0]
    checks["pager_map_fits_and_lists_units"] = len(page_map.encode("utf-8")) <= PAGE_BYTES and all(
        f"- anchor anchor-{u} (docs/x.md heading:## U{u})" in page_map for u in range(6)) and "PAGE-END 0/" in page_map
    small = "one short line\n"
    checks["pager_small_render_is_one_page"] = len(paginate(small)[0]) == 1 and size_note(small, PAGE_BYTES, "X", "cmd") is None
    for name, ok in checks.items():
        print(("PASS " if ok else "FAIL ") + name)
    good = all(checks.values())
    print("PAGER-SELFTEST: " + ("PASS" if good else "FAIL"))
    return 0 if good else 1


def main(argv):
    if argv[:1] == ["--selftest"]:
        return selftest()
    args = dict(zip(argv[::2], argv[1::2]))
    if "--page" not in args and "--note" not in args:
        print(__doc__, file=sys.stderr)
        return 2
    # A render piped on Windows arrives with CRLF; the page protocol is defined over LF text.
    text = sys.stdin.buffer.read().decode("utf-8").replace("\r\n", "\n")
    if "--note" in args:
        note = size_note(text, int(args.get("--page-size", PAGE_BYTES)), args["--note"], args.get("--command", ""))
        if note:
            print(note)
        return 0
    try:
        out, n, names = render_page(text, int(args["--page"]), int(args.get("--page-size", PAGE_BYTES)), args.get("--label", "render"))
    except (PageError, ValueError) as exc:
        print(f"PAGER: FAIL({exc})")
        return 1
    sys.stdout.buffer.write(out.encode("utf-8"))
    if args.get("--reach-query"):
        log_read(args.get("--reach-role", "coding"), f"{args['--reach-query']} --page {args['--page']}/{n}", "none", "page")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
