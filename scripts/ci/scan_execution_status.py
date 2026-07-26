#!/usr/bin/env python3
"""EXECUTION-STATUS-TAXONOMY-0 — unclassified execution-surface detector.

Discovers driver/kernel src paths that match the execution-flavored name
heuristic and are missing from scripts/ci/execution_status_taxonomy.tsv.

Stdout: one match line per unclassified path (path:1:basis) for doctrine_scan.
Exit 0 always when data is well-formed; exit 2 on scanner/data errors.
"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

LEGAL_CLASSES = frozenset({"executed", "oracle", "rehearsal", "compile-plan"})

# Basename / relative-path heuristic for "execution-flavored" surfaces.
# Keep aligned with discovery used to seed the taxonomy TSV.
FLAVOR_RE = re.compile(
    r"(oracle|dress_rehearsal|min_plus|gated_rates|_compile\.rs|"
    r"accumulator|resource_economy|resource_flow|arena_allocation|"
    r"recursive_.*rf|runtime_rf_tick|threshold_event|session_resource_flow|"
    r"cpu_oracle|emission_oracle)",
    re.IGNORECASE,
)


def fail(msg: str) -> int:
    print(f"execution-status scan error: {msg}", file=sys.stderr)
    return 2


def load_taxonomy(path: Path) -> dict[str, str]:
    if not path.is_file():
        raise FileNotFoundError(f"missing taxonomy TSV: {path}")
    classified: dict[str, str] = {}
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != 3 or any(p == "" for p in parts):
            raise ValueError(f"{path.name}:{lineno}: expected 3 tab fields (path, class, basis)")
        rel, cls, _basis = parts
        rel = rel.replace("\\", "/").strip()
        if cls not in LEGAL_CLASSES:
            raise ValueError(
                f"{path.name}:{lineno}: illegal class {cls!r}; "
                f"legal={sorted(LEGAL_CLASSES)}"
            )
        if rel in classified:
            raise ValueError(f"{path.name}:{lineno}: duplicate path {rel}")
        classified[rel] = cls
    if not classified:
        raise ValueError(f"{path.name}: empty taxonomy (no data rows)")
    return classified


def discover_flavored(repo: Path) -> list[str]:
    found: list[str] = []
    for crate in ("simthing-driver", "simthing-kernel"):
        base = repo / "crates" / crate / "src"
        if not base.is_dir():
            continue
        for p in base.rglob("*.rs"):
            rel = p.relative_to(repo).as_posix()
            if FLAVOR_RE.search(p.name) or FLAVOR_RE.search(rel):
                found.append(rel)
    return sorted(set(found))


def main(argv: list[str]) -> int:
    # Args: [repo_root] [--delta-list path]
    repo = Path(argv[1] if len(argv) > 1 else ".").resolve()
    delta_list: Path | None = None
    i = 2
    while i < len(argv):
        if argv[i] == "--delta-list" and i + 1 < len(argv):
            delta_list = Path(argv[i + 1])
            i += 2
            continue
        return fail(f"unknown arg: {argv[i]}")

    taxonomy_path = repo / "scripts" / "ci" / "execution_status_taxonomy.tsv"
    try:
        classified = load_taxonomy(taxonomy_path)
    except (OSError, ValueError, FileNotFoundError) as exc:
        return fail(str(exc))

    if delta_list is not None:
        if not delta_list.is_file():
            return fail(f"missing delta list: {delta_list}")
        candidates = []
        for raw in delta_list.read_text(encoding="utf-8").splitlines():
            rel = raw.strip().replace("\\", "/")
            if not rel.endswith(".rs"):
                continue
            if not (
                rel.startswith("crates/simthing-driver/src/")
                or rel.startswith("crates/simthing-kernel/src/")
            ):
                continue
            name = Path(rel).name
            if FLAVOR_RE.search(name) or FLAVOR_RE.search(rel):
                candidates.append(rel)
        candidates = sorted(set(candidates))
    else:
        candidates = discover_flavored(repo)

    # Unclassified-only (handoff): a NEW flavored surface without a taxonomy row.
    # Stale taxonomy rows (path deleted) are not HEURISTIC-INSPECT here — that is
    # a registry hygiene concern, not an execution-posture admission gap.
    for rel in candidates:
        if rel not in classified:
            print(f"{rel}:1:execution-flavored surface unclassified in execution_status_taxonomy.tsv")

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
