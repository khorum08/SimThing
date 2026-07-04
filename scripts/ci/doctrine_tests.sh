#!/usr/bin/env bash
# CI-B-LOCAL-HARNESS-0 + CI-B-TRIPWIRE-TAGS-0: owner-local executable proof with tripwire tags.
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


FOOTER_PATTERN = re.compile(
    r"^DOCTRINE-TESTS-VERDICT: (PASS|FAIL|INSPECT) "
    r"failures=\d+ inspect=\d+ profile=\S+ owner_local=true head_sha=\S+$"
)


def footer_line(verdict: str, failures: int, inspects: int, profile: str, head_sha: str) -> str:
    return (
        f"DOCTRINE-TESTS-VERDICT: {verdict} failures={failures} inspect={inspects} "
        f"profile={profile} owner_local=true head_sha={head_sha or 'unknown'}"
    )


def footer_is_valid(line: str) -> bool:
    return bool(FOOTER_PATTERN.match(line.strip()))


PASS_TRIPWIRE_TAGS = ("COMPILE_FAIL_PROVEN", "PARITY_BIT_EXACT", "OWNER_LOCAL_PASS")
INSPECT_TRIPWIRE_TAGS = (
    "GPU_SKIPPED",
    "BEVY_SKIPPED",
    "DESKTOP_SKIPPED",
    "OWNER_PREREQ_MISSING",
    "PLAN_ONLY",
    "GITHUB_ACTIONS_REFUSAL",
    "NO_LIVE_LOCAL_PROOF_TARGET",
    "FLAKY",
    "PERF_VARIANCE",
)


def command_needs_gpu(command: str) -> bool:
    lower = command.lower()
    return "simthing-gpu" in lower or re.search(r"\bgpu\b", lower) is not None


def command_needs_bevy(command: str) -> bool:
    return "simthing-mapeditor" in command or re.search(r"\bbevy\b", command, re.I) is not None


def command_needs_desktop(command: str) -> bool:
    return "simthing-mapeditor" in command or "simthing-tools" in command


def classify_command_success_tag(command: str) -> str:
    lower = command.lower()
    if "compile_fail" in lower or "--test compile_fail" in lower:
        return "COMPILE_FAIL_PROVEN"
    if any(token in lower for token in ("parity", "oracle", "bit_exact", "bit-exact", "golden")):
        return "PARITY_BIT_EXACT"
    return "OWNER_LOCAL_PASS"


def derive_tripwire_tags(
    *,
    commands: list[str],
    verdict: str,
    plan_only: bool = False,
    gha_refusal: bool = False,
    gpu_prereq_missing: bool = False,
    no_targets: bool = False,
    run_band: list[str] | None = None,
    synthetic_success_tag: str | None = None,
) -> dict[str, str]:
    tags: dict[str, str] = {}
    has_gpu = any(command_needs_gpu(cmd) for cmd in commands)
    has_bevy = any(command_needs_bevy(cmd) for cmd in commands)
    has_desktop = any(command_needs_desktop(cmd) for cmd in commands)

    if gha_refusal:
        tags["GITHUB_ACTIONS_REFUSAL"] = "INSPECT"
        tags["OWNER_PREREQ_MISSING"] = "INSPECT"
    if plan_only:
        tags["PLAN_ONLY"] = "INSPECT"
        if has_gpu:
            tags["GPU_SKIPPED"] = "INSPECT"
    if no_targets:
        tags["NO_LIVE_LOCAL_PROOF_TARGET"] = "INSPECT"
    if gpu_prereq_missing:
        tags["OWNER_PREREQ_MISSING"] = "INSPECT"
        if has_gpu:
            tags["GPU_SKIPPED"] = "INSPECT"
    skipped_execution = plan_only or gpu_prereq_missing or gha_refusal
    if skipped_execution:
        if has_bevy:
            tags["BEVY_SKIPPED"] = "INSPECT"
        if has_desktop:
            tags["DESKTOP_SKIPPED"] = "INSPECT"

    if run_band:
        normalized = [item.upper() for item in run_band]
        if len(set(normalized)) > 1:
            tags["FLAKY"] = "INSPECT"
        timings = [item for item in run_band if item.endswith("ms")]
        if len(timings) >= 2:
            values = [int(item.removesuffix("ms")) for item in timings]
            if max(values) - min(values) > min(values) * 0.2:
                tags["PERF_VARIANCE"] = "INSPECT"

    if verdict == "PASS":
        if synthetic_success_tag:
            tags[synthetic_success_tag] = "PASS"
        elif commands:
            tags[classify_command_success_tag(commands[0])] = "PASS"
        else:
            tags["OWNER_LOCAL_PASS"] = "PASS"

    return tags


def tripwire_tags_valid(tags: dict[str, str], verdict: str) -> tuple[bool, str]:
    if not tags:
        return False, "missing tripwire tags"
    for name, value in tags.items():
        if value not in {"PASS", "INSPECT"}:
            return False, f"invalid tag value {name}={value}"
    if verdict == "PASS":
        if not any(tags.get(name) == "PASS" for name in PASS_TRIPWIRE_TAGS):
            return False, "PASS verdict requires a PASS tripwire tag"
    if verdict == "INSPECT":
        if not any(value == "INSPECT" for value in tags.values()):
            return False, "INSPECT verdict requires an INSPECT tripwire tag"
    return True, ""


def format_tripwire_tags(tags: dict[str, str]) -> list[str]:
    lines = ["  --- tripwire-tags ---"]
    for name in sorted(tags):
        lines.append(f"  {name}: {tags[name]}")
    return lines


def emit_report(
    *,
    profile: str,
    commands: list[str],
    failures: list[str],
    inspects: list[str],
    verdict: str,
    tripwire_tags: dict[str, str] | None = None,
    run_band: list[str] | None = None,
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
    tags = tripwire_tags or {}
    ok, reason = tripwire_tags_valid(tags, verdict)
    if not ok:
        failures = list(failures)
        failures.append(f"tripwire contract: {reason}")
        verdict = "FAIL"
    for line in format_tripwire_tags(tags):
        print(line)
    if run_band:
        print("  --- run-band ---")
        print(f"  samples: {', '.join(run_band)}")
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
    valid_footer = footer_line("PASS", 0, 0, "owner-local-gpu-bevy", "deadbeef")
    if not footer_is_valid(valid_footer):
        errors.append(f"valid footer rejected by parser: {valid_footer}")

    malformed_footers = [
        "DOCTRINE-TESTS-VERDICT: PASS without counts",
        "DOCTRINE-TESTS-VERDICT: PASS failures=0 profile=owner-local-gpu-bevy owner_local=true head_sha=deadbeef",
        "DOCTRINE-TESTS-VERDICT: PASS failures=0 inspect=0 owner_local=true head_sha=deadbeef",
        "DOCTRINE-TESTS-VERDICT: PASS failures=0 inspect=0 profile=owner-local-gpu-bevy head_sha=deadbeef",
        "DOCTRINE-TESTS-VERDICT: PASS failures=0 inspect=0 profile=owner-local-gpu-bevy owner_local=true",
    ]
    for sample in malformed_footers:
        if footer_is_valid(sample):
            errors.append(f"malformed footer incorrectly accepted: {sample}")

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

    gpu_cmd = "cargo test -p simthing-gpu --test bh1_choke_readout -- --nocapture"
    desktop_cmd = "cargo test -p simthing-mapeditor --test terran_pirate_skeleton -- --nocapture"
    mixed_cmds = [gpu_cmd, desktop_cmd]

    def expect_tag(case: str, tags: dict[str, str], name: str, value: str) -> None:
        if tags.get(name) != value:
            errors.append(f"{case}: expected {name}={value} got {tags.get(name)!r}")

    gha_tags = derive_tripwire_tags(
        commands=mixed_cmds, verdict="INSPECT", gha_refusal=True
    )
    expect_tag("gha-refusal", gha_tags, "GITHUB_ACTIONS_REFUSAL", "INSPECT")

    gpu_missing_tags = derive_tripwire_tags(
        commands=mixed_cmds,
        verdict="INSPECT",
        gpu_prereq_missing=True,
    )
    expect_tag("gpu-missing", gpu_missing_tags, "GPU_SKIPPED", "INSPECT")
    expect_tag("gpu-missing", gpu_missing_tags, "OWNER_PREREQ_MISSING", "INSPECT")
    expect_tag("gpu-missing", gpu_missing_tags, "DESKTOP_SKIPPED", "INSPECT")

    plan_tags = derive_tripwire_tags(
        commands=mixed_cmds, verdict="INSPECT", plan_only=True
    )
    expect_tag("plan-only", plan_tags, "PLAN_ONLY", "INSPECT")
    expect_tag("plan-only", plan_tags, "GPU_SKIPPED", "INSPECT")

    no_target_tags = derive_tripwire_tags(
        commands=[], verdict="INSPECT", no_targets=True
    )
    expect_tag("no-target", no_target_tags, "NO_LIVE_LOCAL_PROOF_TARGET", "INSPECT")

    compile_tags = derive_tripwire_tags(
        commands=["cargo test -p x --test compile_fail_case -- --nocapture"],
        verdict="PASS",
        synthetic_success_tag="COMPILE_FAIL_PROVEN",
    )
    expect_tag("compile-fail", compile_tags, "COMPILE_FAIL_PROVEN", "PASS")

    parity_tags = derive_tripwire_tags(
        commands=["cargo test -p simthing-gpu --test oracle_parity -- --nocapture"],
        verdict="PASS",
        synthetic_success_tag="PARITY_BIT_EXACT",
    )
    expect_tag("parity", parity_tags, "PARITY_BIT_EXACT", "PASS")

    owner_pass_tags = derive_tripwire_tags(
        commands=["cargo test -p simthing-tools --test typeface_lr4 -- --nocapture"],
        verdict="PASS",
        synthetic_success_tag="OWNER_LOCAL_PASS",
    )
    expect_tag("owner-local-pass", owner_pass_tags, "OWNER_LOCAL_PASS", "PASS")

    flaky_tags = derive_tripwire_tags(
        commands=[gpu_cmd],
        verdict="INSPECT",
        run_band=["PASS", "FAIL", "PASS"],
    )
    expect_tag("flaky", flaky_tags, "FLAKY", "INSPECT")

    perf_tags = derive_tripwire_tags(
        commands=[gpu_cmd],
        verdict="INSPECT",
        run_band=["120ms", "180ms", "175ms"],
    )
    expect_tag("perf-variance", perf_tags, "PERF_VARIANCE", "INSPECT")

    fake_pass_ok, fake_pass_reason = tripwire_tags_valid({}, "PASS")
    if fake_pass_ok:
        errors.append("fake PASS without tags should fail validation")

    fake_inspect_ok, _ = tripwire_tags_valid({"PLAN_ONLY": "INSPECT"}, "INSPECT")
    if not fake_inspect_ok:
        errors.append("valid INSPECT tags rejected")

    buf_tags = derive_tripwire_tags(
        commands=mixed_cmds, verdict="INSPECT", plan_only=True
    )
    import io as _io

    capture = _io.StringIO()
    import contextlib as _ctx

    with _ctx.redirect_stdout(capture):
        emit_report(
            profile="owner-local-gpu-bevy",
            commands=mixed_cmds,
            failures=[],
            inspects=["plan-only; commands not executed"],
            verdict="INSPECT",
            tripwire_tags=buf_tags,
        )
    report_text = capture.getvalue()
    if "--- tripwire-tags ---" not in report_text:
        errors.append("emit_report missing tripwire-tags section")
    if "PLAN_ONLY: INSPECT" not in report_text:
        errors.append("emit_report missing PLAN_ONLY tag line")

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
        tags = derive_tripwire_tags(
            commands=commands,
            verdict="INSPECT",
            plan_only=True,
            gpu_prereq_missing=any(command_needs_gpu(c) for c in commands),
            no_targets=not commands,
        )
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=plan_inspects,
            verdict="INSPECT",
            tripwire_tags=tags,
        )
        return 0

    if not args["execute"]:
        print("specify --profile ID to execute, or --plan, or --list, or --prove-report", file=sys.stderr)
        return 2

    if is_github_actions():
        inspects.append(
            "refusing owner-local GPU/Bevy/desktop execution on GitHub Actions"
        )
        tags = derive_tripwire_tags(
            commands=commands,
            verdict="INSPECT",
            gha_refusal=True,
            no_targets=not commands,
        )
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=inspects,
            verdict="INSPECT",
            tripwire_tags=tags,
        )
        return 0

    prereq_ok, prereq_reason = owner_local_prerequisites_ok(commands)
    if not prereq_ok:
        inspects.append(prereq_reason)

    if not commands:
        inspects.append("no live local proof target resolved")

    if inspects:
        tags = derive_tripwire_tags(
            commands=commands,
            verdict="INSPECT",
            gpu_prereq_missing=not prereq_ok,
            no_targets=not commands,
        )
        emit_report(
            profile=profile_id,
            commands=commands,
            failures=failures,
            inspects=inspects,
            verdict="INSPECT",
            tripwire_tags=tags,
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

    tags = derive_tripwire_tags(commands=commands, verdict=verdict)
    emit_report(
        profile=profile_id,
        commands=commands,
        failures=failures,
        inspects=inspects,
        verdict=verdict,
        tripwire_tags=tags,
    )
    return 1 if verdict == "FAIL" else 0


if __name__ == "__main__":
    raise SystemExit(main())
PY