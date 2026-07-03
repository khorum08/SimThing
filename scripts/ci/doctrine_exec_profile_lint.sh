#!/usr/bin/env bash
# CI-B-WEBCHAT-PR1R: quarantine casual full-crate cargo test batteries.
# GHA-PROOF-SEAL-0: forbid Atlas/Bevy/GPU/desktop proof in non-owner-deep profiles.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${DOCTRINE_EXEC_PROFILE_LINT_FILE:-${ROOT}/scripts/ci/doctrine_exec_profiles.tsv}"
DEFAULT_PROFILE="${DOCTRINE_EXEC_DEFAULT_PROFILE:-ci-b-webchat-smoke}"
GHA_PROOF_SEAL="${ROOT}/scripts/ci/doctrine_exec_gha_proof_seal.sh"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

if [[ "${1:-}" == "--prove-gha-proof-seal" ]]; then
  bash "$GHA_PROOF_SEAL" --prove
  exit 0
fi

"$PYTHON_BIN" - <<'PY' "$PROFILES" "$DEFAULT_PROFILE"
import csv
import pathlib
import shlex
import sys

profiles_path = pathlib.Path(sys.argv[1])
default_profile = sys.argv[2]

errors: list[str] = []

FORBIDDEN_GHA_TOKENS = [
    "alsa",
    "libasound",
    "alsa-sys",
    "libudev",
    "udev",
    "xvfb",
    "x11",
    "wayland",
    "xkbcommon",
    "xcb",
    "egl",
    "glx",
    "mesa",
    "vulkan",
    "display",
    "wayland_display",
    "pulseaudio",
    "pipewire",
    "bevy",
    "winit",
    "wininit",
    "wgpu",
    "apt-get",
    "mapeditor",
    "typeface",
]

FORBIDDEN_GHA_CRATE_COMMANDS = [
    "simthing-driver",
    "simthing-gpu",
    "simthing-mapeditor",
    "simthing-tools",
]

if not profiles_path.exists():
    print(f"PROFILE-LINT: FAIL missing profiles file {profiles_path}")
    sys.exit(1)

with profiles_path.open("r", encoding="utf-8", newline="") as f:
    rows = list(csv.DictReader(f, delimiter="\t"))

by_id = {row.get("profile_id", ""): row for row in rows}
if default_profile not in by_id:
    errors.append(f"default profile `{default_profile}` is not present")
elif by_id[default_profile].get("profile_class") == "owner-deep":
    errors.append(f"default profile `{default_profile}` must not be owner-deep")

def split_commands(value: str) -> list[str]:
    return [part.strip() for part in (value or "").split(";") if part.strip()]

def is_owner_deep(row: dict[str, str]) -> bool:
    return row.get("profile_class", "") == "owner-deep"

def blocked_crate_in_command(command: str) -> str | None:
    try:
        tokens = shlex.split(command)
    except ValueError:
        return None
    for index, token in enumerate(tokens):
        if token in ("-p", "--package") and index + 1 < len(tokens):
            crate = tokens[index + 1]
            if crate in FORBIDDEN_GHA_CRATE_COMMANDS:
                return crate
        elif token.startswith("--package="):
            crate = token.split("=", 1)[1]
            if crate in FORBIDDEN_GHA_CRATE_COMMANDS:
                return crate
        elif token.startswith("-p="):
            crate = token.split("=", 1)[1]
            if crate in FORBIDDEN_GHA_CRATE_COMMANDS:
                return crate
    return None


def forbidden_desktop_dep_errors(profile_id: str, profile_class: str, field: str, command: str) -> list[str]:
    out: list[str] = []
    lowered = command.lower()
    for token in FORBIDDEN_GHA_TOKENS:
        if token in lowered:
            out.append(
                "FORBIDDEN-GHA-DESKTOP-DEPS: owner_deep=false profile contains desktop/audio/windowing/GPU token "
                f"'{token}'. Do not install/probe ALSA, X, Bevy, winit, wgpu, mapeditor, typeface, or desktop/GPU "
                f"dependencies in non-owner-deep GHA. Block/defer the crate instead. "
                f"(profile `{profile_id}` {field}: `{command}`)"
            )
    blocked_crate = blocked_crate_in_command(command)
    if blocked_crate:
        out.append(
            "FORBIDDEN-GHA-DESKTOP-DEPS: owner_deep=false profile contains blocked crate "
            f"`{blocked_crate}` in executable command. Do not probe driver/GPU/mapeditor/tools on non-owner-deep GHA. "
            f"Block/defer the crate instead. (profile `{profile_id}` {field}: `{command}`)"
        )
    return out

def lint_forbidden_desktop_deps(rows: list[dict[str, str]]) -> list[str]:
    out: list[str] = []
    for row in rows:
        profile_id = row.get("profile_id", "")
        profile_class = row.get("profile_class", "")
        if is_owner_deep(row):
            continue
        for field in ("tests", "doc_tests"):
            for command in split_commands(row.get(field, "")):
                out.extend(forbidden_desktop_dep_errors(profile_id, profile_class, field, command))
        crate_checks = row.get("crate_checks", "").strip()
        if crate_checks and crate_checks != "-":
            for crate in crate_checks.split(","):
                crate = crate.strip()
                if crate in FORBIDDEN_GHA_CRATE_COMMANDS:
                    out.append(
                        "FORBIDDEN-GHA-DESKTOP-DEPS: owner_deep=false profile contains blocked crate "
                        f"`{crate}` in crate_checks (the engine executes `cargo check -p` per entry). "
                        f"Do not check driver/GPU/mapeditor/tools on non-owner-deep GHA. "
                        f"(profile `{profile_id}` crate_checks: `{crate_checks}`)"
                    )
    return out

def prove_forbidden_desktop_dep_guard() -> list[str]:
    out: list[str] = []
    bad_cases = [
        (
            "blocked crate in crate_checks",
            {
                "profile_id": "prove-bad-crate-checks-mapeditor",
                "profile_class": "targeted",
                "crate_checks": "simthing-core,simthing-mapeditor",
                "tests": "-",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "apt-get libasound",
            {
                "profile_id": "prove-bad-apt-alsa",
                "profile_class": "targeted",
                "tests": "apt-get install -y libasound2-dev",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "driver -p compile floor",
            {
                "profile_id": "prove-bad-driver-p",
                "profile_class": "targeted",
                "tests": "cargo check -p simthing-driver --tests",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "driver --package space",
            {
                "profile_id": "prove-bad-driver-package-space",
                "profile_class": "targeted",
                "tests": "cargo check --package simthing-driver --tests",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "gpu --package= form",
            {
                "profile_id": "prove-bad-gpu-package-eq",
                "profile_class": "targeted",
                "tests": "cargo check --package=simthing-gpu --tests",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "mapeditor -p form",
            {
                "profile_id": "prove-bad-mapeditor-p",
                "profile_class": "targeted",
                "tests": "cargo check -p simthing-mapeditor --tests",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "tools -p= form",
            {
                "profile_id": "prove-bad-tools-p-eq",
                "profile_class": "targeted",
                "tests": "cargo check -p=simthing-tools --tests",
                "doc_tests": "-",
            },
            False,
        ),
        (
            "clean compile floor",
            {
                "profile_id": "prove-good-floor",
                "profile_class": "targeted",
                "tests": "cargo check -p simthing-core --tests;cargo check -p simthing-kernel --tests",
                "doc_tests": "-",
            },
            True,
        ),
    ]
    for label, row, should_pass in bad_cases:
        hits = lint_forbidden_desktop_deps([row])
        if should_pass and hits:
            out.append(f"forbidden-desktop-deps prove `{label}` expected PASS got FAIL: {hits[0]}")
        if not should_pass and not hits:
            out.append(f"forbidden-desktop-deps prove `{label}` expected FAIL got PASS")
    return out

def cargo_test_occurrences(command: str) -> list[list[str]]:
    try:
        tokens = shlex.split(command)
    except ValueError as exc:
        errors.append(f"cannot parse command `{command}`: {exc}")
        return []
    found: list[list[str]] = []
    for i, token in enumerate(tokens[:-1]):
        if token == "cargo" and tokens[i + 1] == "test":
            found.append(tokens[i:])
    return found

def has_exact_selector(tokens: list[str]) -> bool:
    if "--doc" in tokens or "--test" in tokens:
        return True
    package_index = None
    for index, token in enumerate(tokens):
        if token in ("-p", "--package") and index + 1 < len(tokens):
            package_index = index + 1
            break
        if token.startswith("--package="):
            package_index = index
            break
    if package_index is None:
        return False
    for token in tokens[package_index + 1:]:
        if token == "--":
            break
        if token.startswith("-"):
            continue
        return True
    return False

for row in rows:
    profile_id = row.get("profile_id", "")
    profile_class = row.get("profile_class", "")
    if profile_class not in {"smoke", "targeted", "probe", "owner-deep"}:
        errors.append(f"profile `{profile_id}` has invalid profile_class `{profile_class}`")
    if profile_id == "full-cpu":
        errors.append("friendly `full-cpu` profile name is forbidden; use owner-deep-full-cpu-quarantined")
    for field in ("tests", "doc_tests"):
        for command in split_commands(row.get(field, "")):
            for tokens in cargo_test_occurrences(command):
                if profile_class == "owner-deep":
                    continue
                if not has_exact_selector(tokens):
                    errors.append(
                        f"profile `{profile_id}` ({profile_class}) has casual full-crate cargo test: `{command}`"
                    )
                if "--lib" in tokens:
                    errors.append(
                        f"profile `{profile_id}` ({profile_class}) uses broad `--lib` cargo test without an exact selector: `{command}`"
                    )

errors.extend(lint_forbidden_desktop_deps(rows))
errors.extend(prove_forbidden_desktop_dep_guard())

if errors:
    print("PROFILE-LINT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

print(f"PROFILE-LINT: PASS profiles={len(rows)} default={default_profile}")
PY

bash "$GHA_PROOF_SEAL"
