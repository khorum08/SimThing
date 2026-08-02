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
            lines = len(module.read_text(encoding="utf-8", errors="replace").splitlines())
            if lines < min_lines:
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


def scan_zero_test_targets(root: pathlib.Path) -> list[str]:
    """Rust files directly under ``tests/`` are Cargo integration targets.

    A target with no test function still compiles and reports ``0 passed``, so
    it looks healthy while proving nothing.  Keep this advisory, like the
    existing orphan-export detector, but make the shape visible.
    """
    dead: list[str] = []
    for crate in ENGINE:
        tests = root / "crates" / crate / "tests"
        if not tests.is_dir():
            continue
        for path in sorted(tests.glob("*.rs")):
            text = path.read_text(encoding="utf-8", errors="replace")
            if not TEST_ATTRIBUTE_RE.search(strip_rust_comments(text)):
                rel = path.relative_to(root).as_posix()
                lines = len(text.splitlines())
                dead.append(f"{rel} ({lines} lines, integration target has zero test functions)")
    return dead


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


def run(root: pathlib.Path, min_lines: int = DEAD_EXPORT_MIN_LINES) -> int:
    scenario = scan_vocabulary(root, SCENARIO_WORDS)
    domain = scan_vocabulary(root, DOMAIN_WORDS)
    dead_modules = scan_dead_exports(root, min_lines)
    dead_test_targets = scan_zero_test_targets(root)
    dead_support = scan_dead_test_support(root)
    dead = dead_modules + dead_test_targets + dead_support

    for row in scenario:
        print(f"  - SCENARIO-RESIDUE: {row}")
    for row in domain:
        print(f"  - DOMAIN-ACTIVITY: {row}")
    for row in dead:
        print(f"  - DEAD-EXPORT (inspect): {row}")

    failed = bool(scenario) or bool(domain)
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
        f"domain={len(domain)} dead_exports={len(dead)}"
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

        # PLANTED DEFECT 3: a Cargo integration target with no test function is
        # advisory residue even when it contains helper-looking code.
        empty_target = tests / "empty_target.rs"
        empty_target.write_text("pub fn helper_only() {}\n", encoding="utf-8")
        zero_targets = scan_zero_test_targets(tmp)
        if not any("empty_target.rs" in row for row in zero_targets):
            failures.append("zero-test integration target should be detected")
        empty_target.write_text(
            "#[test]\nfn executable_proof() { assert!(true); }\n", encoding="utf-8"
        )
        if any("empty_target.rs" in row for row in scan_zero_test_targets(tmp)):
            failures.append("integration target with a test function must stay live")

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
    print("SCENARIO-RESIDUE-SELFTEST: PASS (10 checks, 4 planted defects)")
    return 0


if __name__ == "__main__":
    sys.exit(selftest() if "--selftest" in sys.argv[1:] else run(ROOT))
