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
STAIR_RE = re.compile(r"else if [^\n]*<\s*[0-9]", re.M)
ADMITTED_EXP = "eml_exp_pinned_f32"
ADMITTED_LN = "eml_ln_pinned_f32"
COMPOSITION_RE = re.compile(
    rf"{ADMITTED_EXP}\s*\([\s\S]{{0,200}}?{ADMITTED_LN}",
    re.S,
)
USE_BRACE_RE = re.compile(
    r"use\s+(?:simthing_embedder::)?populate::\{([^}]+)\}"
)
USE_PATH_RE = re.compile(
    r"use\s+(?:simthing_embedder::)?populate::(eml_(?:exp|ln)_pinned_f32)(?:\s+as\s+(\w+))?"
)
LET_BIND_RE = re.compile(
    r"let\s+(\w+)\s*=\s*(?:populate::)?(eml_(?:exp|ln)_pinned_f32)\s*;"
)


def fail(reason: str) -> None:
    print(f"EMBEDDER-GUIDE-EXEMPLARS-VERDICT: FAIL({reason})")
    raise SystemExit(1)


def pass_ok() -> None:
    print("EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS")


def resolve_admitted_callees(src: str) -> str:
    """Map local aliases / let-bindings onto the admitted EXP/LN callees.

    The law check is composition-shaped after this rewrite, so renaming a
    local identifier does not change the verdict.
    """
    aliases: dict[str, str] = {}
    for block in USE_BRACE_RE.findall(src):
        for item in block.split(","):
            item = item.strip()
            m = re.match(r"(eml_(?:exp|ln)_pinned_f32)(?:\s+as\s+(\w+))?", item)
            if m:
                aliases[m.group(2) or m.group(1)] = m.group(1)
    for m in USE_PATH_RE.finditer(src):
        aliases[m.group(2) or m.group(1)] = m.group(1)
    for m in LET_BIND_RE.finditer(src):
        aliases[m.group(1)] = m.group(2)
    out = re.sub(rf"(?:populate::)?{ADMITTED_EXP}", ADMITTED_EXP, src)
    out = re.sub(rf"(?:populate::)?{ADMITTED_LN}", ADMITTED_LN, out)
    for local, canon in sorted(aliases.items(), key=lambda kv: -len(kv[0])):
        if local in {ADMITTED_EXP, ADMITTED_LN}:
            continue
        out = re.sub(rf"\b{re.escape(local)}\b", canon, out)
    return out


def authored_law_reason(joined: str) -> str | None:
    resolved = resolve_admitted_callees(joined)
    if COMPOSITION_RE.search(resolved):
        return None
    if STAIR_RE.search(joined):
        return "authored-law-staircase"
    return "authored-law-missing-composition"


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
    law = authored_law_reason(joined)
    if law:
        return law
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

    def case(name: str, want: str, mutator, law_only: bool = False) -> None:
        nonlocal failures
        tmp = Path(tempfile.mkdtemp(prefix="embedder-guide-"))
        try:
            write_min_tree(tmp, finance, network, guide)
            mutator(tmp)
            if law_only:
                bodies = [
                    (tmp / "crates/simthing-embedder/tests/finance_toy_0.rs").read_text(
                        encoding="utf-8"
                    ),
                    (tmp / "crates/simthing-embedder/tests/network_saturation_triad_0.rs").read_text(
                        encoding="utf-8"
                    ),
                ]
                reason = authored_law_reason("\n".join(bodies))
            else:
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

    def plant_rename(p: Path) -> None:
        # Local aliases keep exp(k * ln x) and the staircase rival. The
        # admitted callees disappear from call sites; only `as` imports remain.
        path = p / "crates/simthing-embedder/tests/network_saturation_triad_0.rs"
        body = path.read_text(encoding="utf-8")
        body = body.replace(
            "use simthing_embedder::{bind, derive, overlay, populate, run};",
            "use simthing_embedder::{bind, derive, overlay, populate, run};\n"
            "use simthing_embedder::populate::{eml_exp_pinned_f32 as exp, eml_ln_pinned_f32 as ln};",
        )
        body = body.replace("populate::eml_exp_pinned_f32", "exp")
        body = body.replace("populate::eml_ln_pinned_f32", "ln")
        path.write_text(body, encoding="utf-8")

    case(
        "selftest_rename",
        "EMBEDDER-GUIDE-EXEMPLARS-VERDICT: PASS",
        plant_rename,
        True,
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
