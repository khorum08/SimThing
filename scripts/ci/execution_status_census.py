#!/usr/bin/env python3
"""EXECUTION-STATUS-TAXONOMY-0 — export-module census completeness proof.

Proves every `pub mod` exported from:
  crates/simthing-driver/src/lib.rs
  crates/simthing-kernel/src/lib.rs

is accounted for by exactly one of:
  1) scripts/ci/execution_status_taxonomy.tsv   (four legal classes)
  2) scripts/ci/execution_status_mixed_posture.tsv  (DA residual; not a fifth class)
  3) scripts/ci/execution_status_non_execution.tsv  (outside RF execution-status envelope)

Directory modules require every descendant `*.rs` leaf to be accounted, unless the
module root itself is listed (re-export-only roots may use non_execution).

Exit 0 on complete census; exit 1 with uncovered paths; exit 2 on data errors.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

LEGAL = frozenset({"executed", "oracle", "rehearsal", "compile-plan"})


def load_tab(path: Path, nfields: int) -> dict[str, tuple]:
    if not path.is_file():
        raise FileNotFoundError(f"missing {path}")
    out: dict[str, tuple] = {}
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != nfields or any(p == "" for p in parts):
            raise ValueError(f"{path.name}:{lineno}: expected {nfields} tab fields")
        rel = parts[0].replace("\\", "/").strip()
        if rel in out:
            raise ValueError(f"{path.name}:{lineno}: duplicate path {rel}")
        out[rel] = tuple(parts[1:])
    return out


def pub_mods(lib: Path) -> list[str]:
    return re.findall(r"^pub mod (\w+);", lib.read_text(encoding="utf-8"), re.M)


def mod_root_path(repo: Path, crate: str, name: str) -> Path:
    p = repo / "crates" / crate / "src" / f"{name}.rs"
    if p.is_file():
        return p
    p2 = repo / "crates" / crate / "src" / name / "mod.rs"
    if p2.is_file():
        return p2
    return p


def main(argv: list[str]) -> int:
    repo = Path(argv[1] if len(argv) > 1 else ".").resolve()
    tax_path = repo / "scripts" / "ci" / "execution_status_taxonomy.tsv"
    mixed_path = repo / "scripts" / "ci" / "execution_status_mixed_posture.tsv"
    non_path = repo / "scripts" / "ci" / "execution_status_non_execution.tsv"
    try:
        tax = load_tab(tax_path, 3)
        mixed = load_tab(mixed_path, 3)
        non_exec = load_tab(non_path, 2)
    except (OSError, ValueError, FileNotFoundError) as exc:
        print(f"CENSUS-ERROR: {exc}", file=sys.stderr)
        return 2

    for rel, (cls, _basis) in tax.items():
        if cls not in LEGAL:
            print(f"CENSUS-ERROR: illegal class {cls!r} for {rel}", file=sys.stderr)
            return 2

    # Paths must not overlap registries.
    for a, b, na, nb in (
        (tax, mixed, "taxonomy", "mixed"),
        (tax, non_exec, "taxonomy", "non_execution"),
        (mixed, non_exec, "mixed", "non_execution"),
    ):
        overlap = set(a) & set(b)
        if overlap:
            print(
                f"CENSUS-ERROR: path(s) in both {na} and {nb}: {sorted(overlap)[:8]}",
                file=sys.stderr,
            )
            return 2

    classified = set(tax) | set(mixed) | set(non_exec)
    uncovered: list[str] = []

    for crate in ("simthing-driver", "simthing-kernel"):
        lib = repo / "crates" / crate / "src" / "lib.rs"
        if not lib.is_file():
            print(f"CENSUS-ERROR: missing {lib}", file=sys.stderr)
            return 2
        for name in pub_mods(lib):
            root = mod_root_path(repo, crate, name)
            rel_root = root.relative_to(repo).as_posix()
            d = repo / "crates" / crate / "src" / name
            if d.is_dir():
                leaves = sorted(p.relative_to(repo).as_posix() for p in d.rglob("*.rs"))
                for leaf in leaves:
                    if leaf not in classified:
                        uncovered.append(leaf)
            else:
                if rel_root not in classified:
                    uncovered.append(rel_root)

    counts = {k: 0 for k in ("executed", "oracle", "rehearsal", "compile-plan")}
    for cls, _ in tax.values():
        counts[cls] += 1

    print(
        "EXECUTION-STATUS-CENSUS: "
        f"classified={len(tax)} "
        f"executed={counts['executed']} oracle={counts['oracle']} "
        f"rehearsal={counts['rehearsal']} compile-plan={counts['compile-plan']} "
        f"mixed_pending_da={len(mixed)} non_execution={len(non_exec)}"
    )
    if uncovered:
        print(f"CENSUS-FAIL: uncovered={len(uncovered)}", file=sys.stderr)
        for u in uncovered[:40]:
            print(f"  {u}", file=sys.stderr)
        if len(uncovered) > 40:
            print(f"  ... +{len(uncovered) - 40} more", file=sys.stderr)
        return 1
    print("CENSUS-VERDICT: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
