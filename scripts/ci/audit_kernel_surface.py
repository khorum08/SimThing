#!/usr/bin/env python3
"""One-off kernel_surface.txt audit — run from repo root (documented in ci-a-scan-defs_results.md)."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LIB = ROOT / "crates/simthing-kernel/src/lib.rs"
OUT = ROOT / "scripts/ci/allow/kernel_surface.txt"

SEALED_EXPORT = {
    "CandidateFMagnitudeReport",
    "EmissionRecord",
    "EmissionRecordGpu",
    "PlacedParticipant",
    "PlacedParticipantValidationError",
    "ResolvedWriteAuthority",
    "StructuralGridPlacement",
    "ThresholdEmission",
    "ThresholdEmissionGpu",
    "ThresholdEvent",
    "ThresholdEventGpu",
}

SURFACE_INERT = {
    "AO_WGSL0_ENTRY_POINT",
    "CLAMP_BOUNDED",
    "CLAMP_FLOORED",
    "CLAMP_UNBOUNDED",
    "DEFAULT_EMISSION_CAPACITY",
    "DEFAULT_EML_NODE_CAPACITY",
    "DEFAULT_EML_TREE_CAPACITY",
    "DEFAULT_INPUT_LIST_CAPACITY",
    "DEFAULT_THRESHOLD_EMISSION_CAPACITY",
    "DIR_DOWNWARD",
    "DIR_EITHER",
    "DIR_UPWARD",
    "FORMULA_KIND_CONSTANT",
    "FORMULA_KIND_EVAL_EML",
    "FORMULA_KIND_IDENTITY_FLOOR",
    "NO_CONSTANT",
    "NO_MAX_EMIT",
    "NO_TREE_ID",
    "OP_ADD",
    "OP_MULTIPLY",
    "OP_SET",
    "RULE_FIRST",
    "RULE_MAX",
    "RULE_MEAN",
    "RULE_MIN",
    "RULE_SUM",
    "RULE_WEIGHTED_MEAN",
    "THRESH_BUF_OUTPUT",
    "THRESH_BUF_VALUES",
    "WEIGHT_COL_NONE",
    "WORKGROUP_SIZE",
}

SEALED_PRODUCER_XREF = {
    "cpu_oracle_emission_records",
    "cpu_oracle_threshold_events",
}

PROMO = "retire when kernel export set is closed by type-boundary admission"

RATIONALE_OVERRIDES: dict[str, tuple[str, str]] = {
    "WorldGpuState": (
        "authority-export",
        "Kernel-owned world GPU state surface; authority-bearing runtime handle",
    ),
    "ThresholdEvent": (
        "sealed-export",
        "Sealed event record export; produced only through sanctioned read/cpu-oracle doors",
    ),
    "ResolvedWriteAuthority": (
        "sealed-export",
        "Sealed write authority proof export; no cross-crate minting",
    ),
    "ResolvedGpuBuffers": (
        "authority-export",
        "Kernel-owned resolved GPU buffer bundle; authority-bearing runtime surface",
    ),
    "build_overlay_deltas": (
        "authority-export",
        "Overlay delta builder; authority-adjacent kernel admission surface",
    ),
    "project_tree_to_values": (
        "authority-export",
        "Projection helper over resolved values; authority-adjacent kernel surface",
    ),
    "WORKGROUP_SIZE": ("surface-inert", "Inert public kernel constant"),
}


def extract_exports(text: str) -> list[str]:
    mods = re.findall(r"^pub mod (\w+);", text, re.M)
    syms: list[str] = []
    for block in re.finditer(r"pub use \w+::\{([^}]+)\}", text, re.S):
        for part in block.group(1).split(","):
            s = part.strip()
            if s and s not in ("pub", "use", "as", "from", "*"):
                syms.append(s)
    for m in re.finditer(r"^pub use \w+::(\w+);", text, re.M):
        syms.append(m.group(1))
    return sorted(set(mods + syms))


def classify(symbol: str) -> tuple[str, str]:
    if symbol in RATIONALE_OVERRIDES:
        return RATIONALE_OVERRIDES[symbol]
    if symbol in SEALED_EXPORT:
        return (
            "sealed-export",
            "Sealed record/type export; produced only through sanctioned doors",
        )
    if symbol in SURFACE_INERT:
        return ("surface-inert", "Inert public kernel constant")
    if symbol in SEALED_PRODUCER_XREF:
        return (
            "authority-export",
            f"CPU-oracle authority surface; xref sealed_producers:{symbol}",
        )
    if re.match(r"^pub mod ", f"pub mod {symbol};"):
        pass
    # pub mod names land here via mods list
    if symbol in {
        "accumulator_op",
        "candidate_f_magnitude",
        "context",
        "cpu_oracle",
        "emission_accumulator",
        "emission_oracle",
        "gpu_readback",
        "indexed_scatter",
        "intensity_accumulator",
        "overlay_orderband",
        "overlay_prep",
        "participation",
        "passes",
        "projection",
        "readback",
        "reduction",
        "reduction_orderband",
        "registration",
        "resolved",
        "sealed",
        "slot",
        "transfer_accumulator",
        "velocity_accumulator",
        "world_state",
    }:
        return (
            "authority-export",
            f"Exported kernel module surface; authority-bearing namespace",
        )
    return (
        "authority-export",
        "Exported kernel runtime/planning/oracle surface",
    )


def main() -> int:
    text = LIB.read_text(encoding="utf-8")
    items = extract_exports(text)
    lines = ["# symbol | door-class | rationale | promotion-blocker"]
    for sym in items:
        door, rationale = classify(sym)
        lines.append(f"{sym} | {door} | {rationale} | {PROMO}")
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    print(f"exported {len(items)} symbols -> {OUT.relative_to(ROOT)}")
    for required in ("build_overlay_deltas", "project_tree_to_values", "ResolvedGpuBuffers"):
        if required not in items:
            print(f"MISSING: {required}", file=sys.stderr)
            return 1
    grouped = len(re.findall(r"pub use \w+::\{", text))
    single = len(re.findall(r"^pub use \w+::\w+;", text, re.M))
    print(f"forms: grouped={grouped} single-line={single}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
