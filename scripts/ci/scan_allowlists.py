#!/usr/bin/env python3
"""Closed-set allowlist scans for doctrine_scan.sh (stdlib only)."""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
KERNEL_SRC = ROOT / "crates/simthing-kernel/src"
LIB_RS = KERNEL_SRC / "lib.rs"
SEALED_TYPES_FILE = ROOT / "scripts/ci/allow/sealed_types.txt"
FIELD_SWEEP_LEGACY_FILE = ROOT / "scripts/ci/allow/field_sweep_legacy_shaders.txt"
FIELD_SWEEP_SHADER_DIRS = (
    ROOT / "crates/simthing-gpu/src/shaders",
    ROOT / "crates/simthing-kernel/src/shaders",
)
FIELD_SWEEP_CANONICAL_INTERPRETER = "crates/simthing-kernel/src/shaders/field_sweep.wgsl"
FIELD_SWEEP_LEGACY_OPERATOR_NAMES = (
    "AtlasMaskGpuOp",
    "MinPlusStencilOp",
    "MinPlusTraversalDProbeOp",
    "SaturatingFluxChokeThresholdOp",
    "StressComposeOp",
    "StructuredFieldStencilOp",
    "WImpedanceComposeOp",
)
FIELD_SWEEP_LEGACY_OPERATOR_RE = re.compile(
    r"\b(?:" + "|".join(FIELD_SWEEP_LEGACY_OPERATOR_NAMES) + r")\b"
)
FIELD_SWEEP_LEGACY_ORACLE_FILES = frozenset(
    {
        "crates/simthing-gpu/src/atlas_mask.rs",
        "crates/simthing-gpu/src/min_plus_stencil.rs",
        "crates/simthing-gpu/src/min_plus_traversal_d_probe.rs",
        "crates/simthing-gpu/src/saturating_flux_choke_threshold.rs",
        "crates/simthing-gpu/src/stress_compose.rs",
        "crates/simthing-gpu/src/structured_field_stencil.rs",
        "crates/simthing-gpu/src/w_impedance_compose.rs",
        # Retained source is deliberately absent from the crate module graph.
        "crates/simthing-gpu/src/scheduled_w_palma_batch.rs",
        "crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_pack_gpu.rs",
    }
)
FIELD_SWEEP_DENSE_CAP_RE = re.compile(
    r"\b(?:REGION_FIELD_STANDARD_MAX_GRID|REGION_FIELD_EXTENDED_MAX_GRID|REGION_FIELD_MAX_CELL_COUNT)\b"
)
FIELD_SWEEP_DENSE_CAP_PATHS = (
    "crates/simthing-kernel/src/field_sweep.rs",
    "crates/simthing-driver/src/structural_link_accumulator_compile.rs",
)
FIELD_SWEEP_EXPLICIT_NON_FIELD_SHADERS = frozenset(
    {
        "crates/simthing-gpu/src/shaders/accumulator_op_generic.wgsl",
        "crates/simthing-gpu/src/shaders/candidate_f_magnitude.wgsl",
        "crates/simthing-gpu/src/shaders/structural_validation.wgsl",
        "crates/simthing-kernel/src/shaders/accumulator_op.wgsl",
        "crates/simthing-kernel/src/shaders/action_band_execution.wgsl",
        "crates/simthing-kernel/src/shaders/anchor_table_magnitude_values.wgsl",
        "crates/simthing-kernel/src/shaders/anchor_table_maintain.wgsl",
        "crates/simthing-kernel/src/shaders/anchor_table_remap.wgsl",
        "crates/simthing-kernel/src/shaders/candidate_f_magnitude.wgsl",
        "crates/simthing-kernel/src/shaders/snapshot.wgsl",
        "crates/simthing-kernel/src/shaders/values_fill.wgsl",
    }
)


def load_sealed_types() -> tuple[str, ...]:
    """Load the sealed-type name set from data (no invariant hard-coded in the engine).

    Fails loudly: a missing or empty/unparseable data file is a scanner/data error,
    not a silent pass — doctrine_scan.sh surfaces the non-zero exit as a FAIL verdict.
    """
    if not SEALED_TYPES_FILE.exists():
        sys.stderr.write(f"scan_allowlists: missing sealed-types data file {SEALED_TYPES_FILE}\n")
        raise SystemExit(2)
    names = tuple(
        line.strip()
        for line in SEALED_TYPES_FILE.read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.strip().startswith("#")
    )
    if not names:
        sys.stderr.write(f"scan_allowlists: sealed-types data file is empty {SEALED_TYPES_FILE}\n")
        raise SystemExit(2)
    return names


SEALED_TYPES = load_sealed_types()

CONSTRUCTOR_NAMES = frozenset({"new", "default"})

# Inherent-impl targets whose `pub fn … -> Self` are sealed producers.
SEALED_IMPL_TARGETS = frozenset(SEALED_TYPES)

IMPL_INHERENT_RE = re.compile(r"^\s*impl(?:<[^>]*>)?\s+(\w+)\s*(?:\{|$)")


def read_allowlist_symbols(path: Path) -> set[str]:
    symbols: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        symbols.add(line.split(" | ", 1)[0].strip())
    return symbols


def strip_comments(text: str) -> str:
    out: list[str] = []
    for line in text.splitlines():
        if re.match(r"^\s*//", line):
            continue
        out.append(line)
    return "\n".join(out)


def extract_pub_functions(source: str) -> list[tuple[int, str, str, bool]]:
    """Return (line_1based, fn_name, return_type, is_pub_crate)."""
    lines = source.splitlines()
    results: list[tuple[int, str, str, bool]] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if not re.search(r"\bpub\s+(?:\(crate\)\s+)?fn\s+\w+", line):
            i += 1
            continue
        start = i
        sig_parts = [line.rstrip()]
        while i + 1 < len(lines):
            joined = " ".join(p.strip() for p in sig_parts)
            if "{" in joined or joined.rstrip().endswith(";"):
                break
            i += 1
            sig_parts.append(lines[i].rstrip())
        sig = " ".join(p.strip() for p in sig_parts)
        is_crate = bool(re.search(r"\bpub\s*\(\s*crate\s*\)\s+fn\b", sig))
        name_m = re.search(r"\bpub\s+(?:\(crate\)\s+)?fn\s+(\w+)", sig)
        if not name_m:
            i += 1
            continue
        name = name_m.group(1)
        ret_m = re.search(r"->\s*(.+?)(?:\s*\{|\s*;|$)", sig)
        rtype = ret_m.group(1).strip() if ret_m else ""
        results.append((start + 1, name, rtype, is_crate))
        i += 1
    return results


def return_type_is_sealed(rtype: str, impl_sealed_type: str | None = None) -> bool:
    if not rtype:
        return False
    if rtype.strip() == "Self" and impl_sealed_type:
        return True
    for t in SEALED_TYPES:
        if re.search(rf"\b{re.escape(t)}\b", rtype):
            return True
    return False


def build_sealed_impl_line_map(lines: list[str]) -> dict[int, str]:
    """Map 1-based line numbers to sealed inherent-impl type when inside such a block."""
    result: dict[int, str] = {}
    brace_depth = 0
    pending_sealed: str | None = None
    active_sealed: str | None = None

    for i, line in enumerate(lines):
        line_no = i + 1
        if brace_depth == 0:
            m = IMPL_INHERENT_RE.match(line)
            if m and m.group(1) in SEALED_IMPL_TARGETS:
                pending_sealed = m.group(1)

        if active_sealed:
            result[line_no] = active_sealed

        for ch in line:
            if ch == "{":
                if pending_sealed and brace_depth == 0:
                    active_sealed = pending_sealed
                    pending_sealed = None
                brace_depth += 1
            elif ch == "}":
                brace_depth -= 1
                if brace_depth == 0:
                    active_sealed = None
                    pending_sealed = None

    return result


def scan_sealed_producers(allow_path: Path) -> list[str]:
    allowed = read_allowlist_symbols(allow_path)
    violations: list[str] = []
    for path in sorted(KERNEL_SRC.rglob("*.rs")):
        text = strip_comments(path.read_text(encoding="utf-8"))
        lines = text.splitlines()
        impl_map = build_sealed_impl_line_map(lines)
        rel = path.relative_to(ROOT).as_posix()
        for line_no, name, rtype, is_crate in extract_pub_functions(text):
            if is_crate:
                continue
            impl_sealed = impl_map.get(line_no)
            if name in CONSTRUCTOR_NAMES and impl_sealed is None:
                continue
            if not return_type_is_sealed(rtype, impl_sealed):
                continue
            display_rtype = rtype if rtype.strip() != "Self" else f"Self ({impl_sealed})"
            if name not in allowed:
                violations.append(
                    f"{rel}:{line_no}: unsanctioned sealed producer `{name}` -> {display_rtype}"
                )
    return violations


def scan_buffer_handles(allow_path: Path) -> list[str]:
    allowed = read_allowlist_symbols(allow_path)
    violations: list[str] = []
    for path in sorted(KERNEL_SRC.rglob("*.rs")):
        text = strip_comments(path.read_text(encoding="utf-8"))
        rel = path.relative_to(ROOT).as_posix()
        for line_no, name, _rtype, is_crate in extract_pub_functions(text):
            if is_crate:
                continue
            if name in allowed:
                continue
            # Reconstruct approximate signature window for this function.
            lines = text.splitlines()
            sig = lines[line_no - 1] if line_no - 1 < len(lines) else ""
            j = line_no
            while j < len(lines) and "{" not in sig and ";" not in sig:
                sig += " " + lines[j].strip()
                j += 1
            if "pub(crate)" in sig:
                continue
            qual = name
            if name == "dispatch" and "impl IndexedScatterOp" in text:
                qual = "IndexedScatterOp::dispatch"
            if qual in allowed:
                continue
            if re.search(r"->\s*&(?:wgpu::)?Buffer\b", sig):
                violations.append(
                    f"{rel}:{line_no}: unsanctioned public buffer handle `{qual}`"
                )
            elif re.search(r"->\s*(?:wgpu::)?Buffer\b", sig):
                violations.append(
                    f"{rel}:{line_no}: unsanctioned public buffer handle `{qual}`"
                )
            elif re.search(r"->\s*BindingResource\b", sig):
                violations.append(
                    f"{rel}:{line_no}: unsanctioned public buffer handle `{qual}`"
                )
        for m in re.finditer(r"^\s*pub\s+(\w+)\s*:\s*(?:wgpu::)?Buffer\b", text, re.M):
            if "pub(crate)" in m.group(0):
                continue
            sym = m.group(1)
            if sym in allowed:
                continue
            line_no = text[: m.start()].count("\n") + 1
            violations.append(
                f"{rel}:{line_no}: unsanctioned public buffer field `{sym}`"
            )
    return sorted(set(violations))


def extract_lib_exports(text: str) -> set[str]:
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


def scan_kernel_surface(allow_path: Path) -> list[str]:
    text = LIB_RS.read_text(encoding="utf-8")
    exports = extract_lib_exports(text)
    listed = read_allowlist_symbols(allow_path)
    violations: list[str] = []
    for sym in sorted(exports - listed):
        violations.append(
            f"crates/simthing-kernel/src/lib.rs: export `{sym}` missing from kernel_surface.txt"
        )
    for sym in sorted(listed - exports):
        violations.append(
            f"scripts/ci/allow/kernel_surface.txt: stale export `{sym}` not in lib.rs"
        )
    return violations


def scan_field_sweep_shaders(allow_path: Path) -> list[str]:
    """Reject bespoke field shaders in either production shader tree."""
    allowed = read_allowlist_symbols(allow_path)
    violations: list[str] = []
    for shader_dir in FIELD_SWEEP_SHADER_DIRS:
        if not shader_dir.exists():
            continue
        for path in sorted(shader_dir.rglob("*.wgsl")):
            rel = path.relative_to(ROOT).as_posix()
            if rel == FIELD_SWEEP_CANONICAL_INTERPRETER:
                continue
            if rel in FIELD_SWEEP_EXPLICIT_NON_FIELD_SHADERS:
                continue
            if rel not in allowed:
                violations.append(
                    f"{rel}: bespoke field shader is outside the retiring seven-file catalogue"
                )
    return violations


def strip_rust_reexports(text: str) -> str:
    """Remove public re-export declarations; they expose referees but do not call them."""
    return re.sub(r"^\s*pub\s+use\b.*?;\s*$", "", text, flags=re.M | re.S)


def scan_field_sweep_legacy_callers() -> list[str]:
    """Reject every legacy operator type reach from compiled production source."""
    violations: list[str] = []
    crates = ROOT / "crates"
    for path in sorted(crates.glob("*/src/**/*.rs")):
        rel = path.relative_to(ROOT).as_posix()
        if rel in FIELD_SWEEP_LEGACY_ORACLE_FILES:
            continue
        text = strip_rust_reexports(strip_comments(path.read_text(encoding="utf-8")))
        for match in FIELD_SWEEP_LEGACY_OPERATOR_RE.finditer(text):
            line = text[: match.start()].count("\n") + 1
            violations.append(
                f"{rel}:{line}: production reach to retiring field operator `{match.group(0)}`"
            )
    return violations


def scan_field_sweep_dense_caps() -> list[str]:
    """Keep dense REGION_FIELD caps out of sparse/generic adjacency admission."""
    violations: list[str] = []
    for rel in FIELD_SWEEP_DENSE_CAP_PATHS:
        path = ROOT / rel
        if not path.exists():
            # Doctrine selftests intentionally materialize minimal crate trees;
            # scan every authoritative path that exists in that sandbox.
            continue
        text = strip_comments(path.read_text(encoding="utf-8"))
        for match in FIELD_SWEEP_DENSE_CAP_RE.finditer(text):
            line = text[: match.start()].count("\n") + 1
            violations.append(
                f"{rel}:{line}: dense REGION_FIELD cap reached field adjacency path `{match.group(0)}`"
            )
    return violations


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "mode",
        choices=(
            "sealed-producers",
            "buffer-handles",
            "kernel-surface",
            "field-sweep-shaders",
            "field-sweep-legacy-callers",
            "field-sweep-dense-caps",
        ),
    )
    args = parser.parse_args()
    allow_dir = ROOT / "scripts/ci/allow"
    if args.mode == "sealed-producers":
        violations = scan_sealed_producers(allow_dir / "sealed_producers.txt")
    elif args.mode == "buffer-handles":
        violations = scan_buffer_handles(allow_dir / "inert_buffer_handles.txt")
    elif args.mode == "kernel-surface":
        violations = scan_kernel_surface(allow_dir / "kernel_surface.txt")
    elif args.mode == "field-sweep-shaders":
        violations = scan_field_sweep_shaders(FIELD_SWEEP_LEGACY_FILE)
    elif args.mode == "field-sweep-legacy-callers":
        violations = scan_field_sweep_legacy_callers()
    else:
        violations = scan_field_sweep_dense_caps()
    for v in violations:
        print(v)
    return 1 if violations else 0


if __name__ == "__main__":
    raise SystemExit(main())
