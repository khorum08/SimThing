#!/usr/bin/env python3
"""Index `#[cfg(test)]` regions for the HEURISTIC test-region filter.

WHY THIS EXISTS. `heuristic_in_cfg_test_region` in doctrine_scan.sh answered
"is this hit inside a test region?" by re-reading the hit's file in a pure-bash
`while read` loop, once per hit. Measured on the real tree: 701 calls reading
604,160 lines, which was 60 of the scanner's 69 seconds -- 87% of total runtime
spent re-reading a handful of files.

CI-A-SELFTEST-0R (47ec2469) chose that bash loop deliberately, to escape `rg -n`
behaviour that "varied with rg build/invocation". It bought real robustness and
paid for it per hit. This script keeps the robustness (one implementation, same
on Windows and the runner, no regex-dialect surface) and drops the per-hit cost
to a single pass per file.

CONTRACT -- byte-identical to the bash predicate it replaces:
  * a region opens at the LAST line matching `^\\s*#\\[cfg(test)\\]` (either case
    of `cfg`) strictly BEFORE the hit line;
  * that opener qualifies only if `mod<space>tests` appears within its own line
    through opener+4 inclusive;
  * the hit is in a test region iff the nearest preceding opener qualifies.
    An earlier qualifying opener does NOT rescue a nearer non-qualifying one --
    the bash version picked the nearest first and then judged only that one, and
    this reproduces that exactly.

usage: cfg_test_regions.py <repo_root>   # newline-separated rel paths on stdin
       cfg_test_regions.py --selftest
emits: <relpath>\\t<openline>:<0|1> <openline>:<0|1> ...   (one line per file)
"""
import re
import sys

OPENER = re.compile(r"^[ \t]*#\[[Cc]fg\(test\)\]")
MOD_TESTS = re.compile(r"mod[ \t]+tests")
WINDOW = 4  # opener .. opener+4 inclusive, matching the bash cfg_line+4 bound


def index_lines(lines):
    """Return [(open_line_1based, qualifies_bool)] in ascending line order."""
    openers = [i for i, l in enumerate(lines) if OPENER.search(l)]
    out = []
    for i in openers:
        window = lines[i : i + WINDOW + 1]
        out.append((i + 1, any(MOD_TESTS.search(l) for l in window)))
    return out


def encode(index):
    return " ".join("%d:%d" % (ln, 1 if q else 0) for ln, q in index)


def main(argv):
    # Emit LF regardless of platform; the bash reader also strips CR, so neither
    # end alone is load-bearing.
    try:
        sys.stdout.reconfigure(newline="\n")
    except AttributeError:  # pragma: no cover - python < 3.7
        pass
    root = argv[1].rstrip("/\\")
    for rel in sys.stdin.read().splitlines():
        rel = rel.strip()
        if not rel:
            continue
        try:
            with open(root + "/" + rel, "r", encoding="utf-8", errors="replace") as fh:
                lines = fh.read().split("\n")
        except OSError:
            # Unreadable file -> empty index; the caller then treats every hit in
            # it as NOT in a test region, which is what the bash version did when
            # the `[[ -f ]]` guard failed.
            print("%s\t" % rel)
            continue
        print("%s\t%s" % (rel, encode(index_lines(lines))))
    return 0


def selftest():
    fails = []

    def check(name, got, want):
        if got != want:
            fails.append("  FAIL %s: got %r want %r" % (name, got, want))

    # A qualifying opener and a bare one must be distinguished.
    src = [
        "pub fn a() {}",          # 1
        "#[cfg(test)]",           # 2  qualifies (mod tests on line 3)
        "mod tests {",            # 3
        "    fn t() {}",          # 4
        "}",                      # 5
        "#[cfg(test)]",           # 6  bare -- no `mod tests` within 6..10
        "fn helper() {}",         # 7
    ]
    check("mixed openers", index_lines(src), [(2, True), (6, False)])

    # The window is inclusive of opener+4 and excludes opener+5.
    check(
        "window edge +4",
        index_lines(["#[cfg(test)]", "a", "b", "c", "mod tests {"]),
        [(1, True)],
    )
    check(
        "window edge +5",
        index_lines(["#[cfg(test)]", "a", "b", "c", "d", "mod tests {"]),
        [(1, False)],
    )

    # Indentation and the `Cfg` spelling both open a region, per the bash regex.
    check("indented", index_lines(["\t  #[cfg(test)]", "mod tests {"]), [(1, True)])
    check("Cfg spelling", index_lines(["#[Cfg(test)]", "mod tests {"]), [(1, True)])

    # A near non-qualifying opener must NOT be rescued by an earlier qualifying
    # one -- this is the exact bash "nearest wins, then judge" semantics, and the
    # single easiest thing to get wrong when rewriting the predicate.
    idx = index_lines(["#[cfg(test)]", "mod tests {", "}", "#[cfg(test)]", "fn h() {}"])
    check("nearest wins", idx, [(1, True), (4, False)])
    nearest = [q for ln, q in idx if ln < 5][-1]
    check("hit after bare opener is NOT in region", nearest, False)

    # `#[cfg(feature = "test")]` is not an opener; the anchored regex must not
    # widen into ordinary cfg attributes.
    check("non-opener cfg", index_lines(['#[cfg(feature = "test")]', "mod tests {"]), [])

    check("encode", encode([(2, True), (6, False)]), "2:1 6:0")
    check("encode empty", encode([]), "")

    if fails:
        print("\n".join(fails))
        print("CFG-TEST-REGIONS-SELFTEST: FAIL (%d)" % len(fails))
        return 1
    print("CFG-TEST-REGIONS-SELFTEST: PASS (9 checks)")
    return 0


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--selftest":
        sys.exit(selftest())
    sys.exit(main(sys.argv))
