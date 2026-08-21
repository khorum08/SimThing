#!/usr/bin/env bash
# DOCTRINE-CONSTITUTIONAL-SURFACES-0 — closed constitutional surface census.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

exec "$PYTHON_BIN" - "$ROOT" "${1:---check}" <<'PY'
from __future__ import annotations

import csv
import fnmatch
import pathlib
import re
import sys

ROOT = pathlib.Path(sys.argv[1])
MODE = sys.argv[2]
REGISTRY = ROOT / "scripts/ci/constitutional_surfaces.tsv"


def block(text: str, prefix: str) -> str:
    match = re.search(prefix + r"\s*\{", text, re.MULTILINE)
    if not match:
        raise ValueError(f"missing declaration matching {prefix}")
    start = match.end() - 1
    depth = 0
    for index in range(start, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[start + 1 : index]
    raise ValueError(f"unclosed declaration matching {prefix}")


def top_level_chunks(body: str) -> list[str]:
    chunks: list[str] = []
    current: list[str] = []
    depth = 0
    for char in body:
        if char in "({[<":
            depth += 1
        elif char in ")}]>":
            depth = max(0, depth - 1)
        if char == "," and depth == 0:
            chunks.append("".join(current))
            current = []
        else:
            current.append(char)
    if "".join(current).strip():
        chunks.append("".join(current))
    return chunks


def uncomment(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return "\n".join(line.split("//", 1)[0] for line in text.splitlines())


def enum_members(text: str, name: str) -> set[str]:
    body = uncomment(block(text, rf"\b(?:pub\s+)?enum\s+{re.escape(name)}\b"))
    members: set[str] = set()
    for chunk in top_level_chunks(body):
        chunk = re.sub(r"#\s*\[.*?\]", "", chunk, flags=re.DOTALL).strip()
        match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", chunk)
        if match:
            members.add(match.group(1))
    return members


def struct_fields(text: str, name: str) -> set[str]:
    body = uncomment(block(text, rf"\b(?:pub\s+)?struct\s+{re.escape(name)}\b"))
    fields: set[str] = set()
    for chunk in top_level_chunks(body):
        chunk = re.sub(r"#\s*\[.*?\]", "", chunk, flags=re.DOTALL).strip()
        match = re.match(r"(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:", chunk)
        if match:
            fields.add(match.group(1))
    return fields


def module_consts(text: str, name: str) -> set[str]:
    body = uncomment(block(text, rf"\bpub\s+mod\s+{re.escape(name)}\b"))
    return set(re.findall(r"^\s*pub\s+const\s+([A-Z][A-Z0-9_]*)\s*:", body, re.MULTILINE))


def glob_paths(root: pathlib.Path, pattern: str) -> list[pathlib.Path]:
    pattern = pattern.replace("\\", "/")
    candidates = [path for path in root.glob("crates/*/src/**/*.rs") if path.is_file()]
    if "{" in pattern and "}" in pattern:
        head, rest = pattern.split("{", 1)
        choices, tail = rest.split("}", 1)
        patterns = [head + choice + tail for choice in choices.split(",")]
    else:
        patterns = [pattern]
    return sorted(
        path for path in candidates
        if any(fnmatch.fnmatch(path.relative_to(root).as_posix(), pat.replace("**", "*"))
               or pathlib.PurePosixPath(path.relative_to(root).as_posix()).match(pat)
               for pat in patterns)
    )


def read_sources(root: pathlib.Path) -> dict[str, str]:
    return {
        path.relative_to(root).as_posix(): path.read_text(encoding="utf-8", errors="replace")
        for path in root.glob("crates/*/src/**/*.rs") if path.is_file()
    }


def path_matches(path: str, pattern: str) -> bool:
    if pathlib.PurePosixPath(path).match(pattern):
        return True
    if "**" in pattern:
        prefix = pattern.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, pattern.replace("**", "*"))


def registry_rows() -> list[dict[str, str]]:
    with REGISTRY.open(encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


CROSSING_DECL_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum)\s+"
    r"(\w*(?:BandCrossing|ThresholdCrossing|ActionBandCrossing|CrossingConsequence|SaturationListener)\w*)",
    re.MULTILINE,
)
CROSSING_FN_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+"
    r"(\w*(?:threshold_crossed|band_crossing_evalu|crossing_comparator)\w*)",
    re.MULTILINE,
)


def check_sources(sources: dict[str, str]) -> tuple[list[str], dict[str, int]]:
    errors: list[str] = []
    counts: dict[str, int] = {}
    for row in registry_rows():
        surface = row["surface_id"]
        parser = row["parser"]
        path = row["path"]
        expected = {item for item in row["admitted_members"].split(",") if item}
        actual: set[str] = set()
        if parser in {"enum", "struct-fields", "module-const"}:
            text = sources.get(path)
            if text is None:
                errors.append(f"{surface}: missing {path}")
                continue
            try:
                if parser == "enum":
                    actual = enum_members(text, row["declaration"])
                elif parser == "struct-fields":
                    actual = struct_fields(text, row["declaration"])
                else:
                    actual = module_consts(text, row["declaration"])
            except ValueError as exc:
                errors.append(f"{surface}: {exc}")
                continue
        elif parser == "regex-symbol":
            regex = re.compile(row["declaration"], re.MULTILINE)
            for rel, text in sources.items():
                if not path_matches(rel, path):
                    continue
                actual.update(f"{rel}::{match.group(1)}" for match in regex.finditer(text))
        elif parser == "crossing-symbols":
            for rel, text in sources.items():
                if "/tests/" in f"/{rel}/":
                    continue
                actual.update(f"{rel}::{m.group(1)}" for m in CROSSING_DECL_RE.finditer(text))
                actual.update(f"{rel}::{m.group(1)}" for m in CROSSING_FN_RE.finditer(text))
        elif parser == "marker-bound-regex":
            regex = re.compile(row["declaration"], re.IGNORECASE)
            marker = row["lifecycle_binding"]
            for rel, text in sources.items():
                lines = text.splitlines()
                for index, line in enumerate(lines):
                    if line.lstrip().startswith(("//", "///", "//!")):
                        continue
                    if not regex.search(line):
                        continue
                    context = "\n".join(lines[max(0, index - 4) : index + 1])
                    actual.add(f"{rel}:{index + 1}")
                    if marker not in context:
                        errors.append(f"{surface}: unbound telemetry at {rel}:{index + 1}")
        elif parser == "root-admission":
            text = sources.get(path, "")
            if not re.search(r"ValidationFailedAt\s*\{\s*site:\s*&'static\s+str\s*\}", text):
                errors.append(f"{surface}: ValidationFailedAt must carry a static site")
            if re.search(r"\b(?:ValidationFailed|AdmissionFailed|InvalidSpec|InvalidConfiguration|InvalidState)\s*,", text):
                errors.append(f"{surface}: generic nullary admission error is forbidden")
            stale_uses = []
            for rel, body in sources.items():
                if re.search(r"SpecError::ValidationFailed\b(?!At)", body):
                    stale_uses.append(rel)
            if stale_uses:
                errors.append(f"{surface}: untagged ValidationFailed uses: {','.join(sorted(stale_uses))}")
            actual = {"ValidationFailedAt"}
            counts["root_tagged_sites"] = sum(
                len(
                    re.findall(
                        r"SpecError::ValidationFailedAt\s*\{\s*site:\s*\"[^\"]+\"\s*,?\s*\}",
                        body,
                    )
                )
                for body in sources.values()
            )
            if counts["root_tagged_sites"] == 0:
                errors.append(f"{surface}: no tagged admission-error sites discovered")
        else:
            errors.append(f"{surface}: unknown parser {parser}")
            continue
        counts[surface] = len(actual)
        if parser != "marker-bound-regex" and actual != expected:
            added = sorted(actual - expected)
            removed = sorted(expected - actual)
            errors.append(f"{surface}: registry drift added={added} removed={removed}")
    return errors, counts


def selftest(sources: dict[str, str]) -> int:
    baseline, _ = check_sources(sources)
    if baseline:
        print(f"CONSTITUTIONAL-SURFACE-SELFTEST: FAIL(baseline) {baseline}")
        return 1
    cases: list[tuple[str, str, str]] = [
        ("eml-opcode", "crates/simthing-core/src/eml_nodes.rs", "\npub mod planted { pub const NEW_EML_OPCODE: u32 = 99; }\n"),
        ("eml-stack", "crates/simthing-core/src/property.rs", "\npub fn planted_eml_stack() {}\n"),
        ("target-form", "crates/simthing-spec/src/spec/action_band.rs", "\npub enum ActionBandTargetPlant { Cylinder }\n"),
        ("cap-widening", "crates/simthing-spec/src/spec/action_band.rs", ""),
        ("crossing-rival", "crates/simthing-sim/src/lib.rs", "\npub struct SecondBandCrossingRecord;\n"),
        ("telemetry-unbound", "crates/simthing-core/src/lib.rs", "\npub struct EmlPerSlotTelemetry;\n"),
        ("root-nullary", "crates/simthing-spec/src/error.rs", "\npub enum Planted { ValidationFailed, }\n"),
    ]
    failures: list[str] = []
    for label, path, plant in cases:
        mutated = dict(sources)
        if label == "eml-opcode":
            mutated[path] = mutated[path].replace(
                "    pub const RETURN_TOP: u32 = 50;",
                "    pub const RETURN_TOP: u32 = 50;\n    pub const NEW_EML_OPCODE: u32 = 99;",
            )
        elif label == "target-form":
            mutated[path] = mutated[path].replace(
                "    EmlProjectedSet {",
                "    PlantedCylinder { radius: f32 },\n    EmlProjectedSet {",
            )
        elif label == "cap-widening":
            mutated[path] = mutated[path].replace(
                "    pub emission_binding_count: u32,",
                "    pub emission_binding_count: u32,\n    pub runtime_axis_growth: u32,",
            )
        else:
            mutated[path] = mutated[path] + plant
        errors, _ = check_sources(mutated)
        if not errors:
            failures.append(label)
    bound = dict(sources)
    bound_path = "crates/simthing-core/src/lib.rs"
    bound[bound_path] += (
        "\n// EML-TELEMETRY-LIFECYCLE: session-observation-snapshot\n"
        "pub struct EmlPerSlotTelemetry;\n"
    )
    bound_errors, _ = check_sources(bound)
    if any("unbound telemetry" in error for error in bound_errors):
        failures.append("telemetry-valid-binding")
    if failures:
        print(f"CONSTITUTIONAL-SURFACE-SELFTEST: FAIL cases={','.join(failures)}")
        return 1
    print(f"CONSTITUTIONAL-SURFACE-SELFTEST: PASS planted={len(cases)} valid_binding=1")
    return 0


sources = read_sources(ROOT)
if MODE == "--selftest":
    raise SystemExit(selftest(sources))
if MODE != "--check":
    print("usage: constitutional_surface_check.sh [--check|--selftest]", file=sys.stderr)
    raise SystemExit(2)
errors, counts = check_sources(sources)
if errors:
    print(f"CONSTITUTIONAL-SURFACE-VERDICT: FAIL errors={len(errors)}")
    for error in errors:
        print(f"  - {error}")
    raise SystemExit(1)
summary = " ".join(f"{key}={value}" for key, value in sorted(counts.items()))
print(f"CONSTITUTIONAL-SURFACE-VERDICT: PASS {summary}")
PY
