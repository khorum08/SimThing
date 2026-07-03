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

if errors:
    print("PROFILE-LINT: FAIL")
    for error in errors:
        print(f"  - {error}")
    sys.exit(1)

print(f"PROFILE-LINT: PASS profiles={len(rows)} default={default_profile}")
PY

bash "$GHA_PROOF_SEAL"
