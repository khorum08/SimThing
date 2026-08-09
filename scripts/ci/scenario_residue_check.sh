#!/usr/bin/env python3
"""SCENARIO-RESIDUE-PURGE-0 (Owner mandate 2026-08-01).

Two gates that together answer the question this rung was opened by: why did
23,437 lines of dead scenario scaffolding sit in `simthing-driver/src` for
months with nothing to reap it?

The answer was structural. The mortalization regime reaps rows in
`test_inventory.tsv`, and only TESTS get rows. Production `src` has no
lifecycle clock at all, so residue there was not merely unscheduled — it was
INVISIBLE. Things with clocks get cleaned; things without them grow.

  SCENARIO-RESIDUE -> hard FAIL. No scenario or domain-activity vocabulary in
  engine crates, `src` or `tests`. This is the `SEMANTIC-WORDS` heuristic
  promoted to a gate. It could not be promoted before the purge: a gate raised
  ahead of its debt fails against the debt it exists to prevent. The debt is
  now paid (engine measured at zero), so the ratchet lands. Asymmetric by
  doctrine: tightening this list is free, loosening it is reserved to the
  authority that set it.

  DEAD-EXPORT -> INSPECT. An engine module exported from its crate root with
  ZERO references anywhere outside its own crate's `src`, above a line
  threshold; a Rust integration-test target declaring zero test functions; or
  a `tests/support/` module unreachable from every consumer outside support.
  This is the GENERAL gate and the more important of the two: it catches the
  SHAPE of residue rather than its vocabulary. Internal support dependencies
  are followed from live outside roots, and crate-root symbol re-exports keep
  their existing reachability protection.

Usage:
  python3 scripts/ci/scenario_residue_check.sh
  python3 scripts/ci/scenario_residue_check.sh --selftest
"""
import pathlib
import re
import datetime
import os
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]

ENGINE = [
    "simthing-core",
    "simthing-spec",
    "simthing-kernel",
    "simthing-sim",
    "simthing-gpu",
    "simthing-feeder",
    "simthing-driver",
]

# Scenario vocabulary: names of a SHIPPED scenario. A corpus may WITNESS engine
# law but never DEFINE it, so its proper nouns may not appear in engine code.
SCENARIO_WORDS = ["terran", "pirate"]

# Domain-activity vocabulary (Mechanisms-Not-Domains Law, section 4). The engine
# names MECHANISMS; it never names DOMAIN ACTIVITIES. The mechanism behind all
# of these is contested-claim resolution over the RF arena with an AUTHORED
# rule -- proportional, priority-ordered, price-clearing, or attrition. The last
# of those is the case formerly called "combat"; it is one authored instance of
# the mechanism, not the mechanism, and not all contention is adversarial.
DOMAIN_WORDS = ["combat", "diplomac", "siege", "battle"]

# Orphan-export line threshold. Small helper modules exported for future use are
# ordinary; four-thousand-line ones nothing calls are not.
DEAD_EXPORT_MIN_LINES = 200

SUFFIXES = (".rs", ".ron", ".wgsl")
TEST_ATTRIBUTE_RE = re.compile(
    r"#\s*\[\s*(?:test|[A-Za-z_][A-Za-z0-9_]*::test|rstest)\b"
)


def strip_rust_comments(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//.*$", "", text, flags=re.M)


def scan_vocabulary(root: pathlib.Path, words: list[str]) -> list[str]:
    hits: list[str] = []
    pattern = re.compile("|".join(re.escape(w) for w in words), re.I)
    for crate in ENGINE:
        for sub in ("src", "tests"):
            base = root / "crates" / crate / sub
            if not base.is_dir():
                continue
            for path in sorted(base.rglob("*")):
                if not path.is_file() or path.suffix not in SUFFIXES:
                    continue
                text = path.read_text(encoding="utf-8", errors="replace")
                for n, line in enumerate(text.splitlines(), 1):
                    m = pattern.search(line)
                    if m:
                        rel = path.relative_to(root).as_posix()
                        hits.append(f"{rel}:{n}: {m.group(0).lower()}")
    return hits


def reexported_symbols(lib_text: str, module: str) -> set[str]:
    """Symbols a crate root re-exports FROM `module`.

    A live module is usually reached by its SYMBOLS, not its name: `lib.rs`
    says `pub use spec_session::{SpecSessionState, ...}` and downstream code
    writes `use simthing_driver::SpecSessionState`, so the module name never
    appears outside the crate. Matching names alone therefore reports every
    healthy re-exported module as dead. Follow the re-exports instead."""
    out: set[str] = set()
    for m in re.finditer(rf"^pub use (?:crate::)?{re.escape(module)}::(.+?);", lib_text, re.M | re.S):
        body = m.group(1)
        body = body.replace("{", " ").replace("}", " ").replace("*", " ")
        for token in re.split(r"[,\s]+", body):
            token = token.strip()
            if token and token not in {"as", "self"}:
                out.add(token)
    return out


HORIZON_ENTRY_RE = re.compile(r"HORIZON-ENTRY\((\d{4})-(\d{2})-(\d{2})\):\s*\S")
HORIZON_ENTRY_STALE_DAYS = int(os.environ.get("HORIZON_ENTRY_STALE_DAYS", "90"))


def fresh_horizon_entry(text: str) -> bool:
    """True when text carries a dated HORIZON-ENTRY still inside the stale window."""
    today = datetime.date.today()
    for y, m, d in HORIZON_ENTRY_RE.findall(text):
        try:
            stamped = datetime.date(int(y), int(m), int(d))
        except ValueError:
            continue
        if (today - stamped).days <= HORIZON_ENTRY_STALE_DAYS:
            return True
    return False


def scan_dead_exports(root: pathlib.Path, min_lines: int) -> list[str]:
    """An engine module exported from its crate root that nothing outside its
    own crate's src references -- by module name OR by any symbol the crate
    root re-exports from it. Consumers are searched repo-wide, so a module used
    by the authoring layer or by the crate's own tests is NOT dead."""
    dead: list[str] = []
    mod_re = re.compile(r"^pub mod ([a-z0-9_]+);", re.M)

    corpus: list[tuple[str, str]] = []
    for path in sorted((root / "crates").rglob("*.rs")):
        corpus.append((path.relative_to(root).as_posix(), path.read_text(encoding="utf-8", errors="replace")))

    for crate in ENGINE:
        lib = root / "crates" / crate / "src" / "lib.rs"
        if not lib.exists():
            continue
        lib_text = lib.read_text(encoding="utf-8", errors="replace")
        own_src = f"crates/{crate}/src/"
        for name in mod_re.findall(lib_text):
            module = root / "crates" / crate / "src" / f"{name}.rs"
            if not module.exists():
                continue
            module_text = module.read_text(encoding="utf-8", errors="replace")
            lines = len(module_text.splitlines())
            if lines < min_lines:
                continue
            # A FRESH dated HORIZON-ENTRY exempts a consumerless module, exactly as
            # it exempts GUARD-KABUKI (doctrine_scan.sh HC-HORIZON-ENTRY-CONVENTION-0,
            # 90-day window). The repo already ruled that dated + assessable is the
            # sanctioned way to hold API ahead of its consumer; this scan simply did
            # not honour that ruling, so a module whose consumer is being built read
            # identically to one whose consumer will never exist. Stale markers fall
            # through and are reported as before -- the marker buys time, not silence.
            if fresh_horizon_entry(module_text):
                continue
            needles = {name} | reexported_symbols(lib_text, name)
            pattern = re.compile(r"\b(" + "|".join(re.escape(n) for n in sorted(needles)) + r")\b")
            referenced = False
            for rel, text in corpus:
                if rel.startswith(own_src):
                    continue
                if pattern.search(text):
                    referenced = True
                    break
            if not referenced:
                dead.append(f"crates/{crate}/src/{name}.rs ({lines} lines, no consumer outside {crate}/src)")
    return dead


def scan_zero_test_targets(root: pathlib.Path) -> tuple[list[str], list[str]]:
    """Rust files directly under ``tests/`` are Cargo integration targets.

    A target with no test function still compiles and reports ``0 passed``, so
    it looks healthy while proving nothing.  Two DIFFERENT shapes hide there and
    conflating them is why this stayed advisory and unactioned:

    * ``mod <name>;`` from a sibling test -> a LIVE fixture that is merely
      misplaced.  Cargo auto-discovers ``tests/*.rs`` but not ``tests/support/``,
      so the fix is to move it, never to delete it.  Advisory.
    * no test function AND no ``mod`` consumer -> nothing can ever reach it.
      There is no horizon in which a test file that tests nothing and is
      included by nothing becomes correct, so this shape is a hard FAIL and
      carries deletion authority.

    Returns ``(unreachable, misplaced)``.
    """
    unreachable: list[str] = []
    misplaced: list[str] = []
    for crate in ENGINE:
        tests = root / "crates" / crate / "tests"
        if not tests.is_dir():
            continue
        siblings = {
            q: q.read_text(encoding="utf-8", errors="replace")
            for q in sorted(tests.glob("*.rs"))
        }
        for path, text in siblings.items():
            if TEST_ATTRIBUTE_RE.search(strip_rust_comments(text)):
                continue
            rel = path.relative_to(root).as_posix()
            lines = len(text.splitlines())
            decl = re.compile(r"(?m)^[ 	]*mod[ 	]+" + re.escape(path.stem) + r"[ 	]*;")
            consumed = any(
                decl.search(body) for q, body in siblings.items() if q != path
            )
            if consumed:
                misplaced.append(
                    f"{rel} ({lines} lines, zero test functions but included via "
                    f"`mod {path.stem};` — MOVE to tests/support/, do not delete)"
                )
            else:
                unreachable.append(
                    f"{rel} ({lines} lines, zero test functions and no `mod` "
                    f"consumer — unreachable, reap it)"
                )
    return unreachable, misplaced


def scan_dead_test_support(root: pathlib.Path) -> list[str]:
    """Find support modules unreachable from any Rust source outside support/.

    Internal support-to-support edges are followed from externally referenced
    roots, so a live fixture chain remains live.  A module mentioned only by
    another unreachable support module is residue, not a consumer.
    """
    dead: list[str] = []
    for crate in ENGINE:
        tests = root / "crates" / crate / "tests"
        support = tests / "support"
        if not support.is_dir():
            continue
        modules = {
            path.stem: path
            for path in sorted(support.rglob("*.rs"))
            if path.name != "mod.rs"
        }
        if not modules:
            continue

        outside: list[str] = []
        for path in sorted(tests.rglob("*.rs")):
            if support in path.parents:
                continue
            outside.append(path.read_text(encoding="utf-8", errors="replace"))
        outside_text = "\n".join(outside)

        roots: set[str] = set()
        for name in modules:
            if re.search(rf"\b{re.escape(name)}\b", outside_text):
                roots.add(name)

        # Preserve the same symbol-reach protection as crate-root exports for
        # support/mod.rs re-exports (``use support::TheFixture``).
        support_mod = support / "mod.rs"
        mod_text = (
            support_mod.read_text(encoding="utf-8", errors="replace")
            if support_mod.is_file()
            else ""
        )
        for name in modules:
            symbols = reexported_symbols(mod_text, name)
            if symbols and any(
                re.search(rf"\b{re.escape(symbol)}\b", outside_text)
                for symbol in symbols
            ):
                roots.add(name)

        graph: dict[str, set[str]] = {name: set() for name in modules}
        for name, path in modules.items():
            text = path.read_text(encoding="utf-8", errors="replace")
            for candidate in modules:
                if candidate != name and re.search(rf"\b{re.escape(candidate)}\b", text):
                    graph[name].add(candidate)

        live = set(roots)
        pending = list(roots)
        while pending:
            name = pending.pop()
            for dependency in graph[name]:
                if dependency not in live:
                    live.add(dependency)
                    pending.append(dependency)

        for name, path in modules.items():
            if name not in live:
                rel = path.relative_to(root).as_posix()
                lines = len(path.read_text(encoding="utf-8", errors="replace").splitlines())
                dead.append(f"{rel} ({lines} lines, no consumer outside tests/support/)")
    return dead


UNPINNED_SEAL_BASELINE = 95


def scan_unpinned_seals(root: pathlib.Path) -> tuple[int, int]:
    """`compile_fail` doctests that do not pin their expected error code.

    A `compile_fail` seal passes when the snippet fails to compile for ANY
    reason -- including "the type it seals was renamed or deleted".  Such a
    seal is VACUOUS: green, and proving nothing.  Pinning the code
    (```compile_fail,E0599) makes the seal assert WHY it fails, which is the
    difference between a referee and decoration.

    Measured 2026-08-05 after a full-suite run: 100 seals, 5 pinned.  None was
    vacuous at that point, so this ratchets rather than fails the backlog --
    the count may only go DOWN.
    """
    total = pinned = 0
    for path in sorted(root.glob("crates/*/src/**/*.rs")):
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            s = line.strip()
            if s.startswith("///") or s.startswith("//!"):
                s = s.lstrip("/!").strip()
            if s.startswith("```compile_fail"):
                total += 1
                if s.startswith("```compile_fail,E"):
                    pinned += 1
    return total - pinned, total


def run(root: pathlib.Path, min_lines: int = DEAD_EXPORT_MIN_LINES) -> int:
    scenario = scan_vocabulary(root, SCENARIO_WORDS)
    domain = scan_vocabulary(root, DOMAIN_WORDS)
    dead_modules = scan_dead_exports(root, min_lines)
    unreachable_targets, misplaced_targets = scan_zero_test_targets(root)
    dead_support = scan_dead_test_support(root)
    dead = dead_modules + misplaced_targets + dead_support

    for row in scenario:
        print(f"  - SCENARIO-RESIDUE: {row}")
    for row in domain:
        print(f"  - DOMAIN-ACTIVITY: {row}")
    for row in dead:
        print(f"  - DEAD-EXPORT (inspect): {row}")

    for row in unreachable_targets:
        print(f"  - DEAD-TARGET (fail): {row}")
    if unreachable_targets:
        print(
            "  note: a tests/ target with zero test functions AND no `mod` "
            "consumer is unreachable by construction — there is no horizon in "
            "which it becomes correct. Reap it; this shape carries deletion "
            "authority. A zero-test target that IS consumed via `mod` is a "
            "misplaced fixture — move it to tests/support/, never delete it."
        )
    unpinned, seal_total = scan_unpinned_seals(root)
    seal_regressed = unpinned > UNPINNED_SEAL_BASELINE
    if seal_regressed:
        print(
            f'  - UNPINNED-SEAL (fail): {unpinned} of {seal_total} compile_fail '
            f'doctests do not pin an error code (baseline {UNPINNED_SEAL_BASELINE}). '
            'A seal that passes for ANY compile error is vacuous once its type '
            'is renamed. Pin the new one: ```compile_fail,E0599'
        )
    failed = (
        bool(scenario) or bool(domain) or bool(unreachable_targets) or seal_regressed
    )
    if scenario:
        print(
            "  remedy: a corpus may WITNESS engine law but never DEFINE it. Move "
            "the scenario name out of the engine, or express the proof over "
            "inline-constructed input."
        )
    if domain:
        print(
            "  remedy: the engine names MECHANISMS, never DOMAIN ACTIVITIES. The "
            "mechanism is contested-claim resolution with an AUTHORED rule "
            "(proportional / priority / price-clearing / attrition)."
        )
    if dead:
        print(
            "  note: DEAD-EXPORT is advisory. An exported module, empty test "
            "target, or unreachable support fixture is either a missing "
            "consumer or unreaped residue; say which, in the PR."
        )

    verdict = "FAIL" if failed else ("INSPECT" if dead else "PASS")
    print(
        f"SCENARIO-RESIDUE-VERDICT: {verdict} scenario={len(scenario)} "
        f"domain={len(domain)} dead_exports={len(dead)} unpinned_seals={unpinned}"
    )
    return 1 if failed else 0


def selftest() -> int:
    """Every gate lands with a planted defect that must turn it red. A referee
    that does not fail when its defect is planted is decoration, not a proof."""
    import tempfile

    failures: list[str] = []
    with tempfile.TemporaryDirectory() as td:
        tmp = pathlib.Path(td)
        src = tmp / "crates" / "simthing-core" / "src"
        src.mkdir(parents=True)
        (src / "lib.rs").write_text("pub mod clean;\n", encoding="utf-8")
        (src / "clean.rs").write_text("pub fn f() {}\n" * 300, encoding="utf-8")

        # A large orphan export is INSPECT, never FAIL.
        if run(tmp) != 0:
            failures.append("orphan export must be INSPECT (exit 0), not FAIL")

        # PLANTED DEFECT 1: scenario vocabulary must turn it red.
        (src / "clean.rs").write_text("// pirate fleet helper\n", encoding="utf-8")
        if run(tmp) == 0:
            failures.append("planted scenario word should FAIL")

        # PLANTED DEFECT 2: domain-activity vocabulary must turn it red.
        (src / "clean.rs").write_text("pub fn resolve_combat() {}\n", encoding="utf-8")
        if run(tmp) == 0:
            failures.append("planted domain word should FAIL")

        # Clean tree with a consumer outside the crate passes.
        (src / "clean.rs").write_text("pub fn f() {}\n" * 300, encoding="utf-8")
        other = tmp / "crates" / "simthing-mapeditor" / "src"
        other.mkdir(parents=True)
        (other / "use_it.rs").write_text("use simthing_core::clean;\n", encoding="utf-8")
        if run(tmp) != 0:
            failures.append("clean tree with external consumer should PASS")

        # Threshold is honoured: a short orphan is not reported.
        (src / "clean.rs").write_text("pub fn f() {}\n", encoding="utf-8")
        if run(tmp) != 0:
            failures.append("short orphan should not be reported")

        # A module reached ONLY through a re-exported SYMBOL is alive. Without
        # this, every healthy re-exported module reports as dead -- the false
        # positive that `spec_session` exposed when this gate was first run.
        (src / "lib.rs").write_text(
            "pub mod clean;\npub use clean::{TheThing};\n", encoding="utf-8"
        )
        (src / "clean.rs").write_text("pub struct TheThing;\n" * 300, encoding="utf-8")
        (other / "use_it.rs").write_text("use simthing_core::TheThing;\n", encoding="utf-8")
        out: list[str] = scan_dead_exports(tmp, DEAD_EXPORT_MIN_LINES)
        if out:
            failures.append(f"symbol-reached module must not be dead, got {out}")

        tests = tmp / "crates" / "simthing-core" / "tests"
        support = tests / "support"
        support.mkdir(parents=True)

        # PLANTED DEFECT 3: zero test functions AND no `mod` consumer =
        # unreachable by construction -> hard FAIL, deletion authority.
        empty_target = tests / "empty_target.rs"
        empty_target.write_text("pub fn helper_only() {}\n", encoding="utf-8")
        unreachable, misplaced = scan_zero_test_targets(tmp)
        if not any("empty_target.rs" in r for r in unreachable):
            failures.append("unreachable zero-test target should FAIL")
        if any("empty_target.rs" in r for r in misplaced):
            failures.append("unreachable target must not be filed as misplaced")

        # PLANTED DEFECT 3b: the SAME shape, but consumed via `mod` from a
        # sibling, is a LIVE fixture in the wrong directory. Deleting it would
        # break its consumer -- this is the case that kept the detector
        # advisory, and conflating the two is why 5 rows sat unactioned.
        consumer = tests / "consumer_of_fixture.rs"
        consumer.write_text(
            "mod empty_target;\n#[test]\nfn t() { empty_target::helper_only(); }\n",
            encoding="utf-8",
        )
        unreachable, misplaced = scan_zero_test_targets(tmp)
        if any("empty_target.rs" in r for r in unreachable):
            failures.append("mod-consumed fixture must NOT FAIL as unreachable")
        if not any("empty_target.rs" in r for r in misplaced):
            failures.append("mod-consumed zero-test fixture should be misplaced")
        consumer.unlink()

        empty_target.write_text(
            "#[test]\nfn executable_proof() { assert!(true); }\n", encoding="utf-8"
        )
        if any("empty_target.rs" in r for r in scan_zero_test_targets(tmp)[0]):
            failures.append("integration target with a test function must stay live")

        # PLANTED DEFECT 3c: an UNPINNED compile_fail seal is counted; a pinned
        # one is not. A seal that passes for any compile error is vacuous the
        # moment its type is renamed.
        seal_src = tmp / "crates" / "simthing-core" / "src"
        seal_src.mkdir(parents=True, exist_ok=True)
        (seal_src / "seals.rs").write_text(
            "/// ```compile_fail\n/// let _ = 1;\n/// ```\n"
            "/// ```compile_fail,E0599\n/// let _ = 2;\n/// ```\n",
            encoding="utf-8",
        )
        unp, tot = scan_unpinned_seals(tmp)
        if (unp, tot) != (1, 2):
            failures.append(f"seal counter should read (1, 2), got ({unp}, {tot})")

        # PLANTED DEFECT 4: an unconsumed support module is dead, while a
        # sibling named by a source outside support/ is a live root.
        (support / "orphan_fixture.rs").write_text(
            "pub fn orphan_fixture() {}\n", encoding="utf-8"
        )
        (support / "live_fixture.rs").write_text(
            "pub fn live_fixture() {}\n", encoding="utf-8"
        )
        (tests / "live_consumer.rs").write_text(
            '#[path = "support/live_fixture.rs"]\nmod live_fixture;\n'
            "#[test]\nfn consumes_fixture() { live_fixture::live_fixture(); }\n",
            encoding="utf-8",
        )
        support_dead = scan_dead_test_support(tmp)
        if not any("orphan_fixture.rs" in row for row in support_dead):
            failures.append("support module without an outside consumer should be detected")
        if any("live_fixture.rs" in row for row in support_dead):
            failures.append("externally consumed support module must stay live")

    if failures:
        for f in failures:
            print(f"  - {f}")
        print(f"SCENARIO-RESIDUE-SELFTEST: FAIL ({len(failures)})")
        return 1
    print("SCENARIO-RESIDUE-SELFTEST: PASS (14 checks, 6 planted defects)")
    return 0


if __name__ == "__main__":
    sys.exit(selftest() if "--selftest" in sys.argv[1:] else run(ROOT))
