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
import datetime
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


UNIFIED_SURFACE_ROLES = {
    "overlay-ingress",
    "costband-ingress",
    "actionband-ingress",
    "mapping-eml-ingress",
}
CANONICAL_RF_ROLE = "rf-triad-resolution"
CANONICAL_EXECUTION_ROLE = "unified-simthing-execution"
PROOF_RF_ROLE = "proof-only-rf-resolution"
POSTURES = {"production", "deferred", "guard", "proof", "terminal"}
RESOLUTION_CALL_RE = re.compile(
    r"(?:\.run_resource_flow_bands\s*\(|\.tick_with_commitment_spec\s*\(|"
    r"(?<!fn )\brun_simulation_fabric_hot_cycle\s*\(|"
    r"\.execute_with_boundary_hook_and_growth\s*\()"
)
FN_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b",
    re.MULTILINE,
)


def route_for(role: str) -> str:
    if role in UNIFIED_SURFACE_ROLES:
        return f"{role}>{CANONICAL_RF_ROLE}>{CANONICAL_EXECUTION_ROLE}"
    if role == CANONICAL_RF_ROLE:
        return f"{CANONICAL_RF_ROLE}>{CANONICAL_EXECUTION_ROLE}"
    if role == CANONICAL_EXECUTION_ROLE:
        return CANONICAL_EXECUTION_ROLE
    return ""


def function_blocks(text: str) -> list[tuple[str, str]]:
    """Return Rust function bodies for structural call-site census.

    This is intentionally a brace-aware source harvester rather than a function-name
    allowlist. Names locate evidence; the law below is stated over the registered
    semantic role and reachability route.
    """
    clean = uncomment(text)
    found: list[tuple[str, str]] = []
    for match in FN_RE.finditer(clean):
        start = clean.find("{", match.end())
        if start < 0:
            continue
        depth = 0
        for index in range(start, len(clean)):
            char = clean[index]
            if char == "{":
                depth += 1
            elif char == "}":
                depth -= 1
                if depth == 0:
                    # Include the signature so type-carried surface ingress is
                    # visible even when the body only forwards an opaque value.
                    found.append((match.group(1), clean[match.start() : index]))
                    break
    return found


def relationship_checks(
    sources: dict[str, str], rows: list[dict[str, str]], counts: dict[str, int]
) -> list[str]:
    errors: list[str] = []
    row_by_surface = {row["surface_id"]: row for row in rows}

    # The paired vendor-facing and compile-side gadget vocabularies are one
    # constitutional shape. Exact row checks catch drift on either side; this
    # additional relation catches a coordinated but non-1:1 edit.
    instance = row_by_surface.get("EML-GADGET-INSTANCE-VOCABULARY")
    kind = row_by_surface.get("EML-GADGET-KIND-VOCABULARY")
    if instance is None or kind is None:
        errors.append("EML-GADGET-PAIR-CLOSURE: paired census row missing")
    else:
        instance_members = {x for x in instance["admitted_members"].split(",") if x}
        kind_members = {x for x in kind["admitted_members"].split(",") if x}
        if instance_members != kind_members or len(instance_members) != 10:
            errors.append(
                "EML-GADGET-PAIR-CLOSURE: "
                f"instance={sorted(instance_members)} kind={sorted(kind_members)}"
            )

    roles_seen: set[str] = set()
    registered_resolution_callers: dict[str, tuple[str, str]] = {}
    surface_tokens: dict[str, str] = {}

    for row in rows:
        surface = row["surface_id"]
        value = lambda key: "" if (row.get(key) or "") == "n/a" else (row.get(key) or "")
        posture = value("producer_posture")
        role = value("semantic_role")
        consumer_path = value("production_consumer_path")
        consumer_pattern = value("production_consumer_pattern")
        route = value("resolution_route")
        deferral_date = value("deferral_date")
        deferral_provenance = value("deferral_provenance")
        deferral_rationale = value("deferral_rationale")

        if posture not in POSTURES:
            errors.append(f"{surface}: unknown producer_posture {posture!r}")
            continue
        if not role:
            errors.append(f"{surface}: semantic_role is required")
            continue
        roles_seen.add(role)

        if posture == "production":
            production_path = (
                consumer_path.startswith("crates/")
                and "/src/" in f"/{consumer_path}"
                and "/tests/" not in f"/{consumer_path}"
            )
            consumer_text = sources.get(consumer_path)
            found = bool(
                production_path
                and consumer_text is not None
                and consumer_pattern
                and consumer_pattern in uncomment(consumer_text)
            )
            if not found:
                errors.append(
                    "GRADUATED-PRODUCER-WITHOUT-PRODUCTION-CONSUMER: "
                    f"{surface} consumer={consumer_path or 'none'}"
                )
            if deferral_date or deferral_provenance or deferral_rationale:
                errors.append(f"{surface}: production row cannot also claim deferral")
        elif posture == "deferred":
            try:
                datetime.date.fromisoformat(deferral_date)
                valid_date = True
            except ValueError:
                valid_date = False
            provenance_path = ROOT / deferral_provenance if deferral_provenance else None
            provenance_text = (
                provenance_path.read_text(encoding="utf-8", errors="replace")
                if provenance_path is not None and provenance_path.is_file()
                else ""
            )
            if (
                not valid_date
                or provenance_path is None
                or not provenance_path.is_file()
                or deferral_date not in provenance_text
                or not deferral_rationale.strip()
            ):
                errors.append(
                    "GRADUATED-PRODUCER-WITHOUT-PRODUCTION-CONSUMER: "
                    f"{surface} invalid-or-undated-deferral"
                )
            if consumer_path or consumer_pattern:
                errors.append(f"{surface}: deferred row cannot claim a production consumer")
        elif posture == "guard":
            if consumer_path or consumer_pattern or deferral_date:
                errors.append(f"{surface}: guard posture cannot claim consumer or deferral")
        elif posture in {"proof", "terminal"}:
            if consumer_path or consumer_pattern or deferral_date:
                errors.append(f"{surface}: {posture} posture cannot claim consumer or deferral")

        expected_route = route_for(role)
        if expected_route and route != expected_route:
            reason = (
                "CONSTITUTIONAL-SURFACE-OUTSIDE-UNIFIED-INGRESS"
                if role in UNIFIED_SURFACE_ROLES
                else "SECOND-PRODUCTION-RESOLUTION-PATH"
            )
            errors.append(f"{reason}: {surface} route={route or 'none'}")
        if posture == "production" and route.endswith(f">{CANONICAL_RF_ROLE}"):
            errors.append(f"SECOND-PRODUCTION-RESOLUTION-PATH: {surface} route={route}")

        declaration = row.get("declaration", "")
        if role in UNIFIED_SURFACE_ROLES and re.fullmatch(r"[A-Z][A-Za-z0-9_]+", declaration):
            surface_tokens[declaration] = role

        if role in {CANONICAL_RF_ROLE, CANONICAL_EXECUTION_ROLE, PROOF_RF_ROLE}:
            for member in (x for x in row["admitted_members"].split(",") if x):
                registered_resolution_callers[member] = (posture, role)

    missing_roles = UNIFIED_SURFACE_ROLES - roles_seen
    if missing_roles:
        errors.append(
            "CONSTITUTIONAL-SURFACE-OUTSIDE-UNIFIED-INGRESS: "
            f"missing_roles={sorted(missing_roles)}"
        )
    if CANONICAL_RF_ROLE not in roles_seen:
        errors.append("SECOND-PRODUCTION-RESOLUTION-PATH: canonical RF Triad role missing")
    if CANONICAL_EXECUTION_ROLE not in roles_seen:
        errors.append("SECOND-PRODUCTION-RESOLUTION-PATH: unified SimThing root missing")

    # Enumerate every in-tree call that reaches a resolution sink. Test files are
    # absent from `sources`; proof helpers under src must be explicitly classified
    # and therefore cannot silently discharge a production obligation.
    for rel, text in sources.items():
        for caller, body in function_blocks(text):
            if not RESOLUTION_CALL_RE.search(body):
                continue
            key = f"{rel}::{caller}"
            registered = registered_resolution_callers.get(key)
            touched_roles = {
                role for token, role in surface_tokens.items() if re.search(rf"\b{re.escape(token)}\b", body)
            }
            if registered is None:
                errors.append(f"SECOND-PRODUCTION-RESOLUTION-PATH: unregistered={key}")
                if touched_roles:
                    errors.append(
                        "CONSTITUTIONAL-SURFACE-OUTSIDE-UNIFIED-INGRESS: "
                        f"caller={key} roles={sorted(touched_roles)}"
                    )
                continue
            posture, role = registered
            if posture == "proof":
                continue
            if role not in {CANONICAL_RF_ROLE, CANONICAL_EXECUTION_ROLE}:
                errors.append(
                    f"SECOND-PRODUCTION-RESOLUTION-PATH: caller={key} role={role}"
                )
            if role == CANONICAL_RF_ROLE and touched_roles:
                errors.append(
                    "CONSTITUTIONAL-SURFACE-OUTSIDE-UNIFIED-INGRESS: "
                    f"caller={key} roles={sorted(touched_roles)}"
                )

    return errors


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


def check_sources(
    sources: dict[str, str], rows: list[dict[str, str]] | None = None
) -> tuple[list[str], dict[str, int]]:
    errors: list[str] = []
    counts: dict[str, int] = {}
    rows = registry_rows() if rows is None else rows
    for row in rows:
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
    errors.extend(relationship_checks(sources, rows, counts))
    return errors, counts


def selftest(sources: dict[str, str]) -> int:
    rows = registry_rows()
    baseline, _ = check_sources(sources, rows)
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
        ("producer-consumer-law", "crates/simthing-driver/src/session.rs", ""),
        ("resolution-bypass", "crates/simthing-driver/src/lib.rs", ""),
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
        elif label == "producer-consumer-law":
            mutated[path] = mutated[path].replace(
                ".dispatch_sealed_and_apply(",
                ".dispatch_sealed_and_apply_removed(",
                1,
            )
            # A child proof retains the old call spelling. Tests, fixtures, and
            # proof helpers are intentionally absent from production-consumer
            # evidence and therefore cannot close the parent obligation.
            mutated["crates/simthing-driver/tests/child_rung_proof.rs"] = (
                "fn child_rung_proof_only() { let _ = dispatch.dispatch_sealed_and_apply(); }\n"
            )
        elif label == "resolution-bypass":
            # The planted function uses no canonical ingress/caller spelling.
            # Its structure is the violation: a censused Overlay reaches the RF
            # resolution sink from an unregistered production caller.
            mutated[path] += (
                "\npub fn opaque_peer(\n"
                "    state: &mut simthing_gpu::WorldGpuState,\n"
                "    _surface: &simthing_core::Overlay,\n"
                ") { state.run_resource_flow_bands(1, 1.0); }\n"
            )
        else:
            mutated[path] = mutated[path] + plant
        errors, _ = check_sources(mutated, rows)
        if not errors:
            failures.append(label)
        elif label == "producer-consumer-law" and not any(
            error.startswith("GRADUATED-PRODUCER-WITHOUT-PRODUCTION-CONSUMER:")
            for error in errors
        ):
            failures.append(f"{label}-wrong-reason")
        elif label == "resolution-bypass":
            if not any(
                error.startswith("SECOND-PRODUCTION-RESOLUTION-PATH:") for error in errors
            ):
                failures.append(f"{label}-missing-second-path")
            if not any(
                error.startswith("CONSTITUTIONAL-SURFACE-OUTSIDE-UNIFIED-INGRESS:")
                for error in errors
            ):
                failures.append(f"{label}-missing-surface-bypass")
    bound = dict(sources)
    bound_path = "crates/simthing-core/src/lib.rs"
    bound[bound_path] += (
        "\n// EML-TELEMETRY-LIFECYCLE: session-observation-snapshot\n"
        "pub struct EmlPerSlotTelemetry;\n"
    )
    bound_errors, _ = check_sources(bound, rows)
    if any("unbound telemetry" in error for error in bound_errors):
        failures.append("telemetry-valid-binding")

    # Rename-only control: change the source symbol and its evidence locator,
    # preserving the semantic role and route. Exclusivity must remain green.
    renamed = dict(sources)
    old_name = "run_resource_flow_bands_if_active"
    new_name = "dispatch_primary_rf_lane"
    renamed_path = "crates/simthing-driver/src/simulation_fabric.rs"
    renamed[renamed_path] = renamed[renamed_path].replace(old_name, new_name)
    renamed_rows = [dict(row) for row in rows]
    for row in renamed_rows:
        if row["surface_id"] == "RF-TRIAD-RESOURCE-FLOW-RESOLUTION":
            row["declaration"] = row["declaration"].replace(old_name, new_name)
            row["admitted_members"] = row["admitted_members"].replace(old_name, new_name)
            row["production_consumer_pattern"] = row[
                "production_consumer_pattern"
            ].replace(old_name, new_name)
    rename_errors, _ = check_sources(renamed, renamed_rows)
    if rename_errors:
        failures.append("semantic-rename-control")
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
