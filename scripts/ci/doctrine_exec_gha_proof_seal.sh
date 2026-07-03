#!/usr/bin/env bash
# GHA-PROOF-SEAL-0: forbid Atlas/Bevy/GPU/desktop proof in non-owner-deep profiles.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${DOCTRINE_EXEC_PROFILE_LINT_FILE:-${ROOT}/scripts/ci/doctrine_exec_profiles.tsv}"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

"$PYTHON_BIN" - <<'PY' "$PROFILES" "$@"
import csv
import pathlib
import re
import sys

profiles_path = pathlib.Path(sys.argv[1])
mode = sys.argv[2] if len(sys.argv) > 2 else "check"

FORBIDDEN_SUBSTRINGS = [
    "atlas_0080_0",
    "mapeditor_linux_cargo_check.sh",
    "studio_ingestion",
    "apt-get",
    "x11",
    "wayland",
]

FORBIDDEN_PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    ("cargo test -p simthing-mapeditor", re.compile(r"cargo\s+test\s+-p\s+simthing-mapeditor\b")),
    ("cargo test -p simthing-tools", re.compile(r"cargo\s+test\s+-p\s+simthing-tools\b")),
    ("cargo test -p simthing-gpu", re.compile(r"cargo\s+test\s+-p\s+simthing-gpu\b")),
    (
        "cargo test -p simthing-driver --test atlas",
        re.compile(r"cargo\s+test\s+-p\s+simthing-driver\b.*\s--test\s+atlas"),
    ),
    (
        "cargo test -p simthing-workshop --test",
        re.compile(r"cargo\s+test\s+-p\s+simthing-workshop\b.*\s--test\s+"),
    ),
    ("typeface runtime binary", re.compile(r"cargo\s+test\b.*\s--test\s+typeface")),
    ("wgpu runtime proof", re.compile(r"\bwgpu\b", re.IGNORECASE)),
    ("bevy runtime proof", re.compile(r"\bbevy\b", re.IGNORECASE)),
]


def split_commands(value: str) -> list[str]:
    return [part.strip() for part in (value or "").split(";") if part.strip()]


def command_fields(row: dict[str, str]) -> list[tuple[str, str]]:
    out: list[tuple[str, str]] = []
    for field in ("tests", "doc_tests"):
        for command in split_commands(row.get(field, "")):
            out.append((field, command))
    return out


def gha_proof_errors(row: dict[str, str]) -> list[str]:
    profile_id = row.get("profile_id", "")
    profile_class = row.get("profile_class", "")
    if profile_class == "owner-deep":
        return []

    errors: list[str] = []
    for field, command in command_fields(row):
        lowered = command.lower()
        for token in FORBIDDEN_SUBSTRINGS:
            if token in lowered:
                errors.append(
                    f"profile `{profile_id}` ({profile_class}) {field} contains forbidden GHA token `{token}`: `{command}`"
                )
        for label, pattern in FORBIDDEN_PATTERNS:
            if pattern.search(command):
                errors.append(
                    f"profile `{profile_id}` ({profile_class}) {field} matches forbidden GHA pattern `{label}`: `{command}`"
                )
    return errors


def lint_profiles(rows: list[dict[str, str]]) -> list[str]:
    errors: list[str] = []
    for row in rows:
        errors.extend(gha_proof_errors(row))
    return errors


def prove_cases() -> list[tuple[str, dict[str, str], bool]]:
    base = {
        "profile_id": "prove-fixture",
        "risk_class": "prove",
        "crate_checks": "-",
        "doc_tests": "-",
        "gpu_required": "no",
        "expected_verdict_if_gpu_missing": "PASS",
    }
    return [
        (
            "BAD atlas_0080_0",
            {
                **base,
                "profile_class": "targeted",
                "tests": "cargo test -p simthing-driver --test atlas_0080_0 -- --nocapture",
            },
            False,
        ),
        (
            "BAD mapeditor_linux_cargo_check.sh",
            {
                **base,
                "profile_class": "targeted",
                "tests": "bash scripts/ci/mapeditor_linux_cargo_check.sh",
            },
            False,
        ),
        (
            "BAD typeface_lr4 runtime",
            {
                **base,
                "profile_class": "targeted",
                "tests": "cargo test -p simthing-tools --test typeface_lr4 --no-run",
            },
            False,
        ),
        (
            "BAD bh1_choke_readout gpu runtime",
            {
                **base,
                "profile_class": "targeted",
                "tests": "cargo test -p simthing-gpu --test bh1_choke_readout --no-run",
            },
            False,
        ),
        (
            "GOOD cargo check floor simthing-gpu",
            {
                **base,
                "profile_class": "targeted",
                "crate_checks": "simthing-gpu",
                "tests": "-",
            },
            True,
        ),
        (
            "GOOD cargo check floor simthing-driver",
            {
                **base,
                "profile_class": "targeted",
                "crate_checks": "simthing-driver",
                "tests": "-",
            },
            True,
        ),
        (
            "GOOD clausething CPU representative",
            {
                **base,
                "profile_class": "targeted",
                "tests": "cargo test -p simthing-clausething --test ct_1a_entity -- --nocapture",
            },
            True,
        ),
        (
            "GOOD mapgenerator CPU representative",
            {
                **base,
                "profile_class": "targeted",
                "tests": "cargo test -p simthing-mapgenerator --test params -- --nocapture",
            },
            True,
        ),
        (
            "GOOD owner-deep atlas artillery allowed",
            {
                **base,
                "profile_class": "owner-deep",
                "tests": "cargo test -p simthing-driver --test atlas_0080_0 -- --nocapture",
            },
            True,
        ),
    ]


if mode == "--prove":
    failures: list[str] = []
    for name, row, should_pass in prove_cases():
        errors = gha_proof_errors(row)
        ok = (not errors) if should_pass else bool(errors)
        status = "PASS" if ok else "FAIL"
        print(f"  {status}: {name}")
        if not ok:
            failures.append(name)
            for error in errors:
                print(f"    - {error}")
    if failures:
        print("GHA-PROOF-SEAL: FAIL prove")
        sys.exit(1)
    print("GHA-PROOF-SEAL: PASS prove")
    sys.exit(0)

if not profiles_path.exists():
    print(f"GHA-PROOF-SEAL: FAIL missing profiles file {profiles_path}")
    sys.exit(1)

with profiles_path.open("r", encoding="utf-8", newline="") as f:
    rows = list(csv.DictReader(f, delimiter="\t"))

errors = lint_profiles(rows)
if errors:
    print("GHA-PROOF-SEAL: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

print(f"GHA-PROOF-SEAL: PASS profiles={len(rows)}")
PY
