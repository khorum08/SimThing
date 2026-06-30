#!/usr/bin/env python3
"""Compare kernel_surface.txt against lib.rs exports (audit proof helper)."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LIB = ROOT / "crates/simthing-kernel/src/lib.rs"
SURFACE = ROOT / "scripts/ci/allow/kernel_surface.txt"


def extract_exports(text: str) -> set[str]:
    mods = set(re.findall(r"^pub mod (\w+);", text, re.M))
    syms: set[str] = set()
    for block in re.finditer(r"pub use \w+::\{([^}]+)\}", text, re.S):
        for part in block.group(1).split(","):
            s = part.strip()
            if s and s not in ("pub", "use", "as", "from", "*"):
                syms.add(s)
    for m in re.finditer(r"^pub use \w+::(\w+);", text, re.M):
        syms.add(m.group(1))
    return mods | syms


def listed_symbols() -> set[str]:
    out: set[str] = set()
    for line in SURFACE.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        out.add(line.split(" | ", 1)[0].strip())
    return out


def main() -> int:
    text = LIB.read_text(encoding="utf-8")
    exports = extract_exports(text)
    listed = listed_symbols()
    missing = sorted(exports - listed)
    extra = sorted(listed - exports)
    print(f"lib.rs exports: {len(exports)}")
    print(f"kernel_surface.txt: {len(listed)}")
    print(f"missing: {missing}")
    print(f"extra: {extra}")
    for req in ("build_overlay_deltas", "project_tree_to_values", "ResolvedGpuBuffers"):
        print(f"{req}: {'present' if req in listed else 'MISSING'}")
    grouped = len(re.findall(r"pub use \w+::\{", text))
    single = len(re.findall(r"^pub use \w+::\w+;", text, re.M))
    print(f"forms: grouped={grouped} single-line={single}")
    return 1 if missing or extra else 0


if __name__ == "__main__":
    raise SystemExit(main())
