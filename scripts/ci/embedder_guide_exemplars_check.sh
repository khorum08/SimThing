#!/usr/bin/env bash
# EMBEDDER-GUIDE-EXEMPLARS-0 — guide/exemplar admission gate.
#
# CI-safe structure checks. No cargo. Planted defects RED for a named reason.
set -euo pipefail
cd "$(dirname "$0")/../.."
MODE="${1:---check}"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - "$MODE" <<'PY'
from __future__ import annotations

import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(".").resolve()
GUIDE = ROOT / "docs/embedders_guide.md"
EXEMPLARS = (
    ROOT / "crates/simthing-embedder/tests/finance_toy_0.rs",
    ROOT / "crates/simthing-embedder/tests/network_saturation_triad_0.rs",
)
FIXTURES = ROOT / "scripts/ci/fixtures/embedder_guide"
ENGINE_CRATE = re.compile(
    r"^\s*use\s+(simthing_(?:core|driver|kernel|gpu|sim|spec|clausething|feeder))\b",
    re.M,
)
PATH_RE = re.compile(r"crates/simthing-embedder/tests/[A-Za-z0-9_./-]+\.rs")
FENCE_RE = re.compile(r"```(?:rust)?\n(.*?)```", re.S)
LAW_RE = re.compile(r"eml_exp_pinned_f32\([\s\S]{0,160}?eml_ln_pinned_f32", re.S)
STAIR_RE = re.compile(r"else if [^\n]*<\s*[0-9]", re.M)


def fail(reason: str) -> None:
    print(f"EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL({reason})")
    raise SystemExit(1)


def pass_ok() -> None:
    print("EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS")


def check_tree(root: Path) -> str | None:
    guide = root / "docs/embedders_guide.md"
    if not guide.is_file():
        return "guide-path:docs/embedders_guide.md"
    text = guide.read_text(encoding="utf-8")
    for rel in sorted(set(PATH_RE.findall(text))):
        if not (root / rel).is_file():
            return f"guide-path:{rel}"
    exemplars = [
        root / "crates/simthing-embedder/tests/finance_toy_0.rs",
        root / "crates/simthing-embedder/tests/network_saturation_triad_0.rs",
    ]
    bodies = []
    for path in exemplars:
        if not path.is_file():
            return f"guide-path:{path.as_posix()}"
        body = path.read_text(encoding="utf-8")
        bodies.append(body)
        hit = ENGINE_CRATE.search(body)
        if hit:
            return f"door-import:{hit.group(1)}"
    joined = "\n".join(bodies)
    # Law before guide-drift so a planted staircase REDs for the law, not a
    # missing rust fence that cited the exp/ln composition.
    if not LAW_RE.search(joined):
        if STAIR_RE.search(joined):
            return "authored-law-staircase"
        return "authored-law-missing-composition"
    for block in FENCE_RE.findall(text):
        snippet = block.replace("\r\n", "\n")
        if snippet.strip() and snippet not in joined:
            first = snippet.strip().splitlines()[0][:80]
            return f"guide-drift:{first}"
    return None


def run_check() -> None:
    reason = check_tree(ROOT)
    if reason:
        fail(reason)
    pass_ok()


def write_min_tree(dest: Path, finance: str, network: str, guide: str) -> None:
    tests = dest / "crates/simthing-embedder/tests"
    tests.mkdir(parents=True, exist_ok=True)
    (dest / "docs").mkdir(parents=True, exist_ok=True)
    (tests / "finance_toy_0.rs").write_text(finance, encoding="utf-8")
    (tests / "network_saturation_triad_0.rs").write_text(network, encoding="utf-8")
    (dest / "docs/embedders_guide.md").write_text(guide, encoding="utf-8")


def run_selftest() -> None:
    finance = EXEMPLARS[0].read_text(encoding="utf-8")
    network = EXEMPLARS[1].read_text(encoding="utf-8")
    guide = GUIDE.read_text(encoding="utf-8")
    failures = 0

    def case(name: str, want: str, mutator) -> None:
        nonlocal failures
        tmp = Path(tempfile.mkdtemp(prefix="embedder-guide-"))
        try:
            write_min_tree(tmp, finance, network, guide)
            mutator(tmp)
            reason = check_tree(tmp)
            got = (
                "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS"
                if reason is None
                else f"EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL({reason})"
            )
            if not got.startswith(want):
                print(f"FAIL {name} (got={got}, want prefix {want})")
                failures += 1
            else:
                print(f"PASS {name}")
        finally:
            shutil.rmtree(tmp, ignore_errors=True)

    case("selftest_live_shape", "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS", lambda p: None)

    def plant_stair(p: Path) -> None:
        planted = (FIXTURES / "known_bad_staircase.rs").read_text(encoding="utf-8")
        (p / "crates/simthing-embedder/tests/network_saturation_triad_0.rs").write_text(
            planted, encoding="utf-8"
        )

    case(
        "selftest_staircase",
        "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL(authored-law-staircase)",
        plant_stair,
    )

    def plant_import(p: Path) -> None:
        body = (p / "crates/simthing-embedder/tests/finance_toy_0.rs").read_text(encoding="utf-8")
        body = "use simthing_core::SimThing;\n" + body
        (p / "crates/simthing-embedder/tests/finance_toy_0.rs").write_text(body, encoding="utf-8")

    case(
        "selftest_door_import",
        "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL(door-import:simthing_core)",
        plant_import,
    )

    def plant_path(p: Path) -> None:
        g = (p / "docs/embedders_guide.md").read_text(encoding="utf-8")
        g = g.replace(
            "crates/simthing-embedder/tests/finance_toy_0.rs",
            "crates/simthing-embedder/tests/missing_toy.rs",
        )
        (p / "docs/embedders_guide.md").write_text(g, encoding="utf-8")

    case(
        "selftest_guide_path",
        "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL(guide-path:",
        plant_path,
    )

    if failures:
        print("EMBEDDER-GUIDE-EXEMPLARS-SELFTEST: FAIL")
        raise SystemExit(1)
    print("EMBEDDER-GUIDE-EXEMPLARS-SELFTEST: PASS")
    pass_ok()


mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
if mode == "--check":
    run_check()
elif mode == "--selftest":
    run_selftest()
else:
    print("usage: embedder_guide_exemplars_check.sh --check|--selftest", file=sys.stderr)
    raise SystemExit(2)
PY
