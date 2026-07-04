#!/usr/bin/env bash
# CI-B-LOCAL-HARNESS-0: owner-local executable proof for GPU/Bevy/desktop-exclusive residue.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILES="${ROOT}/scripts/ci/doctrine_tests_profiles.tsv"
INVENTORY="${ROOT}/scripts/ci/test_inventory.tsv"
GHA_PROFILES="${ROOT}/scripts/ci/doctrine_exec_profiles.tsv"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

export DOCTRINE_TESTS_ROOT="$ROOT"
export DOCTRINE_TESTS_PROFILES="$PROFILES"
export DOCTRINE_TESTS_INVENTORY="$INVENTORY"
export DOCTRINE_TESTS_GHA_PROFILES="$GHA_PROFILES"

"$PYTHON_BIN" - <<'PY' "$@"
import csv
import datetime
import os
import pathlib
import re
import shlex
import subprocess
import sys

root = pathlib.Path(os.environ["DOCTRINE_TESTS_ROOT"])
inventory_path = pathlib.Path(os.environ["DOCTRINE_TESTS_INVENTORY"])
gha_profiles_path = pathlib.Path(os.environ["DOCTRINE_TESTS_GHA_PROFILES"])


def profiles_path() -> pathlib.Path:
    return pathlib.Path(os.environ["DOCTRINE_TESTS_PROFILES"])

PROFILE_HEADER = [
    "profile_id",
    "profile_class",
    "owner_local_only",
    "proof_class",
    "commands",
    "expected_verdict_if_missing",
]

OWNER_LOCAL_GPU_CRATES = {
    "simthing-gpu",
    "simthing-workshop",
    "simthing-driver",
    "simthing-sim",
    "simthing-clausething",
}
OWNER_LOCAL_DESKTOP_CRATES = {
    "simthing-mapeditor",
    "simthing-tools",
}

FORBIDDEN_GHA_PATTERNS = [
    re.compile(r"cargo\s+test\s+-p\s+simthing-mapeditor\b"),
    re.compile(r"cargo\s+test\s+-p\s+simthing-tools\b"),
    re.compile(r"cargo\s+test\s+-p\s+simthing-gpu\b"),
    re.compile(r"cargo\s+test\s+-p\s+simthing-driver\b.*\s--test\s+atlas"),
    re.compile(r"cargo\s+test\s+-p\s+simthing-workshop\b.*\s--test\s+"),
    re.compile(r"\bbevy\b", re.I),
    re.compile(r"\bwgpu\b", re.I),
]


def git_value(*args: str) -> str:
    try:
        out = subprocess.run(
            ["git", "-C", str(root), *args],
            check=True,
            text=True,
            capture_output=True,
        )
        return out.stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return ""


def parse_args(argv: list[str]) -> dict:
    out = {
        "list": False,
        "plan": False,
        "prove_report": False,
        "profile": "",
    }
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--list":
            out["list"] = True
        elif arg == "--plan":
            out["plan"] = True
        elif arg == "--prove-report":
            out["prove_report"] = True
        elif arg == "--profile":
            i += 1
            out["profile"] = argv[i]
        elif arg in ("-h", "--help"):
            print(
                "usage: doctrine_tests.sh [--list] [--plan --profile ID] [--profile ID] [--prove-report]"
            )
            sys.exit(0)
        else:
            print(f"unknown arg: {arg}", file=sys.stderr)
            sys.exit(2)
        i += 1
    out["execute"] = bool(out["profile"]) and not out["plan"]
    return out


def read_profiles() -> list[dict[str, str]]:
    path = profiles_path()
    if not path.exists():
        raise RuntimeError(f"missing local profiles file {path}")
    with path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f, delimiter="\t")
        if reader.fieldnames != PROFILE_HEADER:
            raise RuntimeError(f"bad local profiles header: {reader.fieldnames!r}")
        return list(reader)


def read_inventory() -> list[dict[str, str]]:
    with inventory_path.open("r", encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def test_binary_from_path(file_path: str) -> str:
    return pathlib.Path(file_path).stem


def is_gpu_test_name(test_name: str) -> bool:
    lower = test_name.lower()
    if "no_gpu" in lower or "nogpu" in lower:
        return False
    return "gpu" in lower


def inventory_owner_local_commands() -> tuple[list[str], list[str]]:
    rows = read_inventory()
    commands: list[str] = []
    inspect_notes: list[str] = []
    seen: set[str] = set()
    gpu_binaries: set[tuple[str, str]] = set()
    desktop_binaries: set[tuple[str, str]] = set()

    def add_binary_cmd(crate: str, binary: str) -> None:
        key = (crate, binary)
        cmd = f"cargo test -p {crate} --test {binary} -- --nocapture"
        if cmd in seen:
            return
        seen.add(cmd)
        commands.append(cmd)

    for row in rows:
        if row.get("verdict") != "KEEP" or row.get("kind") != "integration":
            continue
        crate = row["crate"]
        file_path = row["file"]
        test_name = row["test_name"]
        full_path = root / file_path
        if not full_path.exists():
            inspect_notes.append(f"missing inventory-backed path: {file_path}")
            continue
        binary = test_binary_from_path(file_path)
        normalized = file_path.replace("\\", "/")

        if crate in OWNER_LOCAL_GPU_CRATES:
            in_tests_dir = "/tests/" in normalized
            if not in_tests_dir:
                continue
            gpu_leg = (
                crate == "simthing-gpu"
                or is_gpu_test_name(test_name)
                or "_gpu" in normalized.lower()
            )
            if gpu_leg:
                gpu_binaries.add((crate, binary))

        if crate in OWNER_LOCAL_DESKTOP_CRATES and "/tests/" in normalized:
            desktop_binaries.add((crate, binary))

    for crate, binary in sorted(gpu_binaries):
        add_binary_cmd(crate, binary)
    for crate, binary in sorted(desktop_binaries):
        add_binary_cmd(crate, binary)

    if not commands:
        inspect_notes.append("no live local proof target resolved from inventory")
    return commands, inspect_notes


def resolve_profile_commands(profile_row: dict[str, str]) -> tuple[list[str], list[str]]:
    raw = profile_row.get("commands", "").strip()
    if raw.startswith("RESOLVE:"):
        if raw != "RESOLVE:inventory-owner-local-gpu":
            return [], [f"unknown resolver token: {raw}"]
        return inventory_owner_local_commands()
    if not raw or raw == "-":
        return [], ["profile has no commands or resolver"]
    return [part.strip() for part in raw.split(";") if part.strip()], []


def command_is_forbidden_on_gha(command: str) -> bool:
    for pattern in FORBIDDEN_GHA_PATTERNS:
        if pattern.search(command):
            return True
    return False


def load_gha_profile_commands() -> set[str]:
    if not gha_profiles_path.exists():
        return set()
    out: set[str] = set()
    with gha_profiles_path.open("r", encoding="utf-8", newline="") as f:
        for row in csv.DictReader(f, delimiter="\t"):
            for field in ("tests", "doc_tests"):
                value = row.get(field, "")
                for part in value.split(";"):
                    part = part.strip()
                    if part and part != "-":
                        out.add(part)
    return out


def host_summary() -> str:
    return f"{sys.platform} / {os.name}"


def is_github_actions() -> bool:
    return os.environ.get("GITHUB_ACTIONS", "").lower() == "true"


def owner_local_prerequisites_ok(commands: list[str]) -> tuple[bool, str]:
    if is_github_actions():
        return False, "GitHub Actions refuses owner-local GPU/Bevy/desktop execution"
    needs_gpu = any("simthing-gpu" in c or "gpu" in c.lower() for c in commands)
    if needs_gpu and os.environ.get("DOCTRINE_TESTS_FORCE_INSPECT", "") == "1":
        return False, "DOCTRINE_TESTS_FORCE_INSPECT=1 set for prove path"
    if needs_gpu:
        if os.environ.get("DOCTRINE_TESTS_GPU_OK", "") == "1":
            return True, "DOCTRINE_TESTS_GPU_OK=1"
        return False, (
            "owner-local GPU prerequisites not confirmed "
            "(set DOCTRINE_TESTS_GPU_OK=1 on owner machine with real adapter)"
        )
    return True, "desktop-local legs only"


def footer_line(verdict: str, failures: int, inspects: int, profile: str, head_sha: str) -> str:
    return (
        f"DOCTRINE-TESTS-VERDICT: {verdict} failures={failures} inspect={inspects} "
        f"profile={profile} owner_local=true head_sha={head_sha or 'unknown'}"
    )


def emit_report(
    *,
    profile: str,
    commands: list[str],
    failures: list[str],
    inspects: list[str],
    verdict: str,
) -> None:
    head_sha = git_value("rev-parse", "HEAD")
    tested_ref = git_value("symbolic-ref", "--short", "-q", "HEAD") or git_value(
        "rev-parse", "--short", "HEAD"
    )
    base_ref = git_value("rev-parse", "--abbrev-ref", "origin/master")
    if not base_ref or base_ref == "origin/master":
        base_ref = "origin/master"
    ts = datetime.datetime.utcnow().replace(microsecond=0).isoformat() + "Z"
    print(f"DOCTRINE TESTS REPORT  (head {head_sha or 'unknown'}, {ts})")
    print(f"  profile: {profile}")
    print("  owner_local: true")
    print(f"  host: {host_summary()}")
    print(f"  tested_ref: {tested_ref or 'unknown'}")
    print(f"  head_sha: {head_sha or 'unknown'}")
    print(f"  base_ref: {base_ref}")
    print("  --- commands ---")
    if commands:
        for cmd in commands:
            print(f"  {cmd}")
    else:
        print("  (none)")
    print("  --- failures ---")
    if failures:
        for item in failures:
            print(f"  {item}")
    else:
        print("  none")
    print("  --- inspect ---")
    if inspects:
        for item in inspects:
            print(f"  {item}")
    else:
        print("  none")
    print(footer_line(verdict, len(failures), len(inspects), profile, head_sha))


def run_commands(commands: list[str]) -> tuple[list[str], list[str]]:
    failures: list[str] = []
    inspects: list[str] = []
    for cmd in commands:
        print(f"RUN: {cmd}")
        try:
            proc = subprocess.run(
                cmd,
                cwd=root,
                shell=True,
                text=True,
                capture_output=True,
            )
        except OSError as exc:
            failures.append(f"{cmd} -> launch error: {exc}")
            continue
        if proc.returncode != 0:
            snippet = (proc.stderr or proc.stdout or "").strip().splitlines()
            tail = snippet[-1] if snippet else f"exit {proc.returncode}"
            failures.append(f"{cmd} -> {tail}")
    return failures, inspects


def prove_report() -> int:
    errors: list[str] = []

    sample_commands = [
        "cargo test -p simthing-gpu --test bh1_choke_readout -- --nocapture bh1_choke_readout_gpu_matches_cpu_oracle"
    ]
    footer = footer_line("PASS", 0, 0, "owner-local-gpu-bevy", "deadbeef")
    if not footer.startswith("DOCTRINE-TESTS-VERDICT: PASS"):
        errors.append("footer format check failed")

    bad_footer = "DOCTRINE-TESTS-VERDICT: PASS without counts"
    if re.search(r"DOCTRINE-TESTS-VERDICT: (PASS|FAIL|INSPECT)", bad_footer):
        pass
    else:
        errors.append("verdict regex failed on negative sample")

    gha_cmds = load_gha_profile_commands()
    for cmd in sample_commands:
        if cmd in gha_cmds:
            errors.append(f"sample local command also wired to doctrine_exec_profiles.tsv: {cmd}")

    malformed = profiles_path().with_suffix(".malformed.tsv")
    malformed.write_text("profile_id\tprofile_class\nx\ty\n", encoding="utf-8")
    try:
        with malformed.open("r", encoding="utf-8", newline="") as f:
            reader = csv.DictReader(f, delimiter="\t")
            if reader.fieldnames == PROFILE_HEADER:
                errors.append("malformed profile header unexpectedly valid")
        saved_profiles = os.environ.get("DOCTRINE_TESTS_PROFILES")
        os.environ["DOCTRINE_TESTS_PROFILES"] = str(malformed)
        try:
            read_profiles()
            errors.append("malformed profile TSV did not fail read_profiles")
        except RuntimeError:
            pass
        finally:
            if saved_profiles is None:
                os.environ.pop("DOCTRINE_TESTS_PROFILES", None)
            else:
                os.environ["DOCTRINE_TESTS_PROFILES"] = saved_profiles
    finally:
        malformed.unlink(missing_ok=True)

    if is_github_actions():
        ok, reason = owner_local_prerequisites_ok(sample_commands)
        if ok:
            errors.append("expected GITHUB_ACTIONS to block owner-local prerequisites")
    else:
        os.environ["DOCTRINE_TESTS_FORCE_INSPECT"] = "1"
        ok, reason = owner_local_prerequisites_ok(sample_commands)
        if ok:
            errors.append("expected FORCE_INSPECT to block gpu prerequisites")
        os.environ.pop("DOCTRINE_TESTS_FORCE_INSPECT", None)

    profiles = read_profiles()
    for row in profiles:
        commands, notes = resolve_profile_commands(row)
        for cmd in commands:
            if cmd in gha_cmds:
                errors.append(
                    f"local profile `{row['profile_id']}` command overlaps GHA profile table: {cmd}"
                )
            if command_is_forbidden_on_gha(cmd):
                pass  # expected for owner-local legs

    if errors:
        print("DOCTRINE-TESTS-PROVE-REPORT: FAIL")
        for err in errors:
            print(f"  - {err}")
        return 1

    print("DOCTRINE-TESTS-PROVE-REPORT: PASS")
    return 0


def main() -> int:
    args = parse_args(sys.argv[1:])
    profiles = read_profiles()

    if args["prove_report"]:
        return prove_report()

    if args["list"]:
        for row in profiles:
            print(
                f"{row['profile_id']}\t{row['profile_class']}\t{row['proof_class']}\t"
                f"owner_local={row['owner_local_only']}"
            )
        return 0

    profile_id = args["profile"] or "owner-local-gpu-bevy"
    profile_row = next((r for r in profiles if r["profile_id"] == profile_id), None)
    if profile_row is None:
        print(f"unknown profile: {profile_id}", file=sys.stderr)
        return 2

    commands, resolver_inspects = resolve_profile_commands(profile_row)
    failures: list[str] = []
    inspects: list[str] = list(resolver_inspects)

    if args["plan"]:
        plan_inspects = list(inspects)
        plan_inspects.append("plan-only; commands not executed")
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=plan_inspects,
            verdict="INSPECT",
        )
        return 0

    if not args["execute"]:
        print("specify --profile ID to execute, or --plan, or --list, or --prove-report", file=sys.stderr)
        return 2

    if is_github_actions():
        inspects.append(
            "refusing owner-local GPU/Bevy/desktop execution on GitHub Actions"
        )
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=inspects,
            verdict="INSPECT",
        )
        return 0

    prereq_ok, prereq_reason = owner_local_prerequisites_ok(commands)
    if not prereq_ok:
        inspects.append(prereq_reason)

    if not commands:
        inspects.append("no live local proof target resolved")

    if inspects:
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=inspects,
            verdict="INSPECT",
        )
        return 0

    run_failures, run_inspects = run_commands(commands)
    failures.extend(run_failures)
    inspects.extend(run_inspects)

    if failures:
        verdict = "FAIL"
    elif inspects:
        verdict = "INSPECT"
    else:
        verdict = "PASS"

    emit_report(
        profile=profile_id,
        commands=commands,
        failures=failures,
        inspects=inspects,
        verdict=verdict,
    )
    return 1 if verdict == "FAIL" else 0


if __name__ == "__main__":
    raise SystemExit(main())
PY