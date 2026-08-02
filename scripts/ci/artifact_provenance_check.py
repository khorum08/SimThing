#!/usr/bin/env python3
"""ARTIFACT-PROVENANCE-CONTAINMENT-0.

Generated TSVs consumed by the orientation or handoff renderers must expose a
regenerator, reach an executable fixture-backed proof, and never read from the
shipped ``scenarios/`` tree.  The census is derived from the two consumers and
from artifact headers; artifact names are not allowlisted here.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import sys
import tempfile


DEFAULT_ROOT = pathlib.Path(__file__).resolve().parents[2]
CONSUMERS = (
    "scripts/ci/gen_orientation.sh",
    "scripts/ci/handoff_dispatch.sh",
)
TSV_TOKEN_RE = re.compile(r"[A-Za-z0-9_./\\${}-]+\.tsv")
GENERATED_RE = re.compile(r"(?im)^#.*\bGENERATED\b.*(?:do not hand-edit|do not edit)")
REGEN_RE = re.compile(r"(?im)^#\s*Regenerate:\s*(?:(?:bash|python3?|py)\s+)?([^\s]+)")
CARGO_TEST_RE = re.compile(
    r"\bcargo\s+test\b(?P<args>.*?)(?=(?:\bcargo\s+test\b)|$)", re.S
)
PACKAGE_RE = re.compile(r"(?:^|\s)-p\s+([^\s]+)")
TARGET_RE = re.compile(r"(?:^|\s)--test\s+([^\s]+)")
PACKAGE_NAME_RE = re.compile(
    r"(?ms)^\[package\]\s*$.*?^name\s*=\s*['\"]([^'\"]+)['\"]"
)
MOD_RE = re.compile(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
INCLUDE_RE = re.compile(r"include_(?:str|bytes)!\s*\(\s*['\"]([^'\"]+)['\"]\s*\)")
SCENARIO_PATH_RE = re.compile(r"(?:^|[/\\\"'])scenarios(?:[/\\\"']|$)", re.I)


def repo_relative(root: pathlib.Path, path: pathlib.Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def within(root: pathlib.Path, path: pathlib.Path) -> bool:
    try:
        path.resolve().relative_to(root.resolve())
        return True
    except ValueError:
        return False


def strip_comments(text: str) -> str:
    """Remove Rust/shell comments while retaining newlines for diagnostics."""

    def blank_block(match: re.Match[str]) -> str:
        return "".join("\n" if ch == "\n" else " " for ch in match.group(0))

    text = re.sub(r"/\*.*?\*/", blank_block, text, flags=re.S)
    out: list[str] = []
    for line in text.splitlines(keepends=True):
        stripped = line.lstrip()
        if stripped.startswith("#") and not stripped.startswith("#!["):
            out.append("\n" if line.endswith("\n") else "")
            continue
        marker = line.find("//")
        if marker >= 0:
            ending = "\n" if line.endswith("\n") else ""
            line = line[:marker] + ending
        out.append(line)
    return "".join(out)


def consumed_tsvs(root: pathlib.Path) -> tuple[dict[pathlib.Path, set[str]], list[str]]:
    consumed: dict[pathlib.Path, set[str]] = {}
    errors: list[str] = []
    for consumer_rel in CONSUMERS:
        consumer = root / consumer_rel
        if not consumer.is_file():
            errors.append(f"missing-consumer consumer={consumer_rel}")
            continue
        text = consumer.read_text(encoding="utf-8", errors="replace")
        for token in TSV_TOKEN_RE.findall(text):
            token = token.replace("\\", "/")
            candidates: list[pathlib.Path] = []
            if token.startswith("scripts/ci/"):
                candidates.append(root / token)
            candidates.append(root / "scripts/ci" / pathlib.PurePosixPath(token).name)
            for candidate in candidates:
                if candidate.is_file():
                    consumed.setdefault(candidate.resolve(), set()).add(consumer_rel)
                    break
    return consumed, errors


def generated_artifact(path: pathlib.Path) -> bool:
    text = path.read_text(encoding="utf-8", errors="replace")
    return bool(GENERATED_RE.search(text) or REGEN_RE.search(text))


def regenerator(root: pathlib.Path, artifact: pathlib.Path) -> tuple[pathlib.Path | None, str | None]:
    text = artifact.read_text(encoding="utf-8", errors="replace")
    match = REGEN_RE.search(text)
    if not match:
        return None, "missing-regenerator"
    rel = match.group(1).strip("`'\"").replace("\\", "/")
    generator = (root / rel).resolve()
    if not within(root, generator):
        return None, "generator-outside-repo"
    if not generator.is_file():
        return None, f"missing-generator generator={rel}"
    return generator, None


def cargo_packages(root: pathlib.Path) -> dict[str, pathlib.Path]:
    packages: dict[str, pathlib.Path] = {}
    for manifest in sorted(root.glob("crates/*/Cargo.toml")):
        text = manifest.read_text(encoding="utf-8", errors="replace")
        match = PACKAGE_NAME_RE.search(text)
        if match:
            packages[match.group(1)] = manifest.parent
    return packages


def proof_sources(
    root: pathlib.Path, generator: pathlib.Path, packages: dict[str, pathlib.Path]
) -> tuple[set[pathlib.Path], list[str]]:
    text = generator.read_text(encoding="utf-8", errors="replace").replace("\\\n", " ")
    sources: set[pathlib.Path] = {generator.resolve()}
    errors: list[str] = []
    commands = list(CARGO_TEST_RE.finditer(text))
    if not commands:
        return sources, errors
    for command in commands:
        args = command.group("args")
        pkg_match = PACKAGE_RE.search(args)
        target_match = TARGET_RE.search(args)
        if not pkg_match or not target_match:
            errors.append(f"unresolved-generator-command generator={repo_relative(root, generator)}")
            continue
        package = pkg_match.group(1).strip("'\"")
        target = target_match.group(1).strip("'\"")
        crate = packages.get(package)
        if crate is None:
            errors.append(f"missing-generator-package generator={repo_relative(root, generator)} package={package}")
            continue
        source = (crate / "tests" / f"{target}.rs").resolve()
        if not source.is_file():
            errors.append(
                f"missing-generator-proof generator={repo_relative(root, generator)} "
                f"proof={repo_relative(root, source)}"
            )
            continue
        sources.add(source)
    return sources, errors


def reachable_modules(root: pathlib.Path, seeds: set[pathlib.Path]) -> set[pathlib.Path]:
    reached = set(seeds)
    pending = list(seeds)
    while pending:
        source = pending.pop()
        if source.suffix != ".rs":
            continue
        active = strip_comments(source.read_text(encoding="utf-8", errors="replace"))
        for name in MOD_RE.findall(active):
            candidates = (source.parent / f"{name}.rs", source.parent / source.stem / f"{name}.rs")
            for candidate in candidates:
                candidate = candidate.resolve()
                if candidate.is_file() and within(root, candidate) and candidate not in reached:
                    reached.add(candidate)
                    pending.append(candidate)
                    break
    return reached


def fixture_witnesses(root: pathlib.Path, sources: set[pathlib.Path]) -> set[pathlib.Path]:
    witnesses: set[pathlib.Path] = set()
    for source in sources:
        active = strip_comments(source.read_text(encoding="utf-8", errors="replace"))
        for rel in INCLUDE_RE.findall(active):
            candidate = (source.parent / rel).resolve()
            if candidate.is_file() and within(root, candidate) and "fixtures" in candidate.parts:
                witnesses.add(candidate)
    return witnesses


def scenario_reads(root: pathlib.Path, sources: set[pathlib.Path]) -> list[str]:
    hits: list[str] = []
    for source in sorted(sources):
        active = strip_comments(source.read_text(encoding="utf-8", errors="replace"))
        for line_no, line in enumerate(active.splitlines(), 1):
            normalized = line.replace("\\", "/").strip()
            if SCENARIO_PATH_RE.search(normalized):
                hits.append(f"{repo_relative(root, source)}:{line_no}:{normalized}")
    return hits


def check(root: pathlib.Path, *, emit: bool = True) -> tuple[int, list[str]]:
    root = root.resolve()
    consumed, errors = consumed_tsvs(root)
    packages = cargo_packages(root)
    covered: list[tuple[pathlib.Path, set[str], set[pathlib.Path]]] = []

    for artifact, consumers in sorted(consumed.items(), key=lambda item: str(item[0])):
        if not generated_artifact(artifact):
            continue
        generator, error = regenerator(root, artifact)
        if error or generator is None:
            errors.append(f"{error} artifact={repo_relative(root, artifact)}")
            continue
        if artifact.name not in generator.read_text(encoding="utf-8", errors="replace"):
            errors.append(
                f"generator-output-unbound artifact={repo_relative(root, artifact)} "
                f"generator={repo_relative(root, generator)}"
            )
        seeds, proof_errors = proof_sources(root, generator, packages)
        errors.extend(f"{error} artifact={repo_relative(root, artifact)}" for error in proof_errors)
        sources = reachable_modules(root, seeds)
        witnesses = fixture_witnesses(root, sources)
        if not witnesses:
            errors.append(
                f"missing-fixture-witness artifact={repo_relative(root, artifact)} "
                f"generator={repo_relative(root, generator)}"
            )
        for hit in scenario_reads(root, sources):
            errors.append(
                f"scenario-generator-read artifact={repo_relative(root, artifact)} source={hit}"
            )
        covered.append((artifact, consumers, witnesses))

    if not covered:
        errors.append("empty-generated-artifact-census")

    if emit:
        for artifact, consumers, witnesses in covered:
            consumer_text = ",".join(sorted(pathlib.PurePosixPath(value).name for value in consumers))
            witness_text = ",".join(sorted(repo_relative(root, value) for value in witnesses)) or "none"
            print(
                f"  - artifact={repo_relative(root, artifact)} consumers={consumer_text} "
                f"fixtures={witness_text}"
            )
        for error in errors:
            print(f"  - {error}")
        verdict = "FAIL" if errors else "PASS"
        print(
            f"ARTIFACT-PROVENANCE-VERDICT: {verdict} generated={len(covered)} "
            f"consumers={len(CONSUMERS)} errors={len(errors)}"
        )
    return (1 if errors else 0), errors


def selftest(root: pathlib.Path) -> int:
    fixtures = root / "scripts/ci/fixtures/artifact_provenance"
    clean = fixtures / "clean"
    planted = fixtures / "scenario_read"
    failures: list[str] = []

    clean_rc, clean_errors = check(clean, emit=False)
    if clean_rc != 0:
        failures.append(f"clean fixture must pass: {clean_errors}")

    planted_rc, planted_errors = check(planted, emit=False)
    if planted_rc == 0 or not planted_errors or not all(
        error.startswith("scenario-generator-read") for error in planted_errors
    ):
        failures.append(f"planted scenarios/ read must be the only failure: {planted_errors}")

    with tempfile.TemporaryDirectory() as td:
        restored = pathlib.Path(td) / "restored"
        shutil.copytree(planted, restored)
        proof = restored / "crates/fixture-crate/tests/sample_generator.rs"
        proof.write_text(
            proof.read_text(encoding="utf-8").replace(
                '../../../scenarios/input.txt', 'fixtures/input.txt'
            ),
            encoding="utf-8",
        )
        fixture_input = restored / "crates/fixture-crate/tests/fixtures/input.txt"
        fixture_input.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(restored / "scenarios/input.txt", fixture_input)
        restored_rc, restored_errors = check(restored, emit=False)
        if restored_rc != 0:
            failures.append(f"restored fixture read must return green: {restored_errors}")

    if failures:
        for failure in failures:
            print(f"  - {failure}")
        print(f"ARTIFACT-PROVENANCE-SELFTEST: FAIL ({len(failures)})")
        return 1
    print("PASS clean fixture-backed generator")
    print("PASS planted generator scenarios/ read -> RED")
    print("PASS restored generator fixture read -> GREEN")
    print("ARTIFACT-PROVENANCE-SELFTEST: PASS (3 checks, 1 planted defect)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=DEFAULT_ROOT)
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args()
    return selftest(args.root.resolve()) if args.selftest else check(args.root)[0]


if __name__ == "__main__":
    sys.exit(main())
