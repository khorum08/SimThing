#!/usr/bin/env python3
"""Exercise the real Resolve PR context workflow block with hostile review JSON."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import textwrap


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/doctrine-exec-commands.yml"
FIXTURE = ROOT / "scripts/ci/fixtures/doctrine_exec_review_context/review_events.json"
SAFE_ASSIGNMENT = 'JSON="$(jq -c \'.pull_request\' "$GITHUB_EVENT_PATH")"'
TO_JSON_EXPRESSION = "toJson(github.event.pull_request)"

JQ_FALLBACK = r'''#!/usr/bin/env python
import json
import sys

arguments = sys.argv[1:]
raw = "-r" in arguments
arguments = [argument for argument in arguments if argument not in {"-c", "-r"}]
expression = arguments[0]
if len(arguments) == 2:
    with open(arguments[1], encoding="utf-8") as handle:
        value = json.load(handle)
else:
    value = json.load(sys.stdin)
expression = expression.removesuffix(" // empty")
for component in expression.removeprefix(".").split("."):
    value = value[component]
if raw:
    if value is None:
        print("")
    elif isinstance(value, bool):
        print(str(value).lower())
    else:
        print(value)
else:
    print(json.dumps(value, separators=(",", ":")))
'''


def resolve_context_script() -> str:
    lines = WORKFLOW.read_text(encoding="utf-8").splitlines()
    step_index = next(
        index for index, line in enumerate(lines) if line.strip() == "- name: Resolve PR context"
    )
    run_index = next(
        index for index in range(step_index + 1, len(lines)) if lines[index].strip() == "run: |"
    )
    run_indent = len(lines[run_index]) - len(lines[run_index].lstrip())
    script_lines: list[str] = []
    for line in lines[run_index + 1 :]:
        indent = len(line) - len(line.lstrip())
        if line.strip() and indent <= run_indent:
            break
        script_lines.append(line)
    return textwrap.dedent("\n".join(script_lines)) + "\n"


def expression_values(case: dict[str, object]) -> dict[str, str]:
    event = case["event"]
    assert isinstance(event, dict)
    pull_request = event["pull_request"]
    assert isinstance(pull_request, dict)
    review = event.get("review", {})
    comment = event.get("comment", {})
    repository = event.get("repository", {})
    assert isinstance(review, dict)
    assert isinstance(comment, dict)
    assert isinstance(repository, dict)
    return {
        "github.event.repository.default_branch": str(repository.get("default_branch", "master")),
        "github.event_name": str(case["event_name"]),
        "github.event.issue.number": "0",
        "github.event.comment.id": str(comment.get("id", 0)),
        "github.event.issue.pull_request != null": "false",
        "github.repository": "example/simthing",
        "github.event.pull_request.number": str(pull_request["number"]),
        "github.event.review.id": str(review.get("id", 0)),
        TO_JSON_EXPRESSION: json.dumps(pull_request, separators=(",", ":")),
    }


EXPRESSION = re.compile(r"\$\{\{\s*(.*?)\s*\}\}")


def render_script(script: str, case: dict[str, object]) -> str:
    values = expression_values(case)

    def replace(match: re.Match[str]) -> str:
        expression = match.group(1)
        if expression not in values:
            raise AssertionError(f"unhandled workflow expression: {expression}")
        return values[expression]

    return EXPRESSION.sub(replace, script)


def parse_outputs(path: Path) -> dict[str, str]:
    outputs: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        key, separator, value = line.partition("=")
        if not separator:
            raise AssertionError(f"malformed GITHUB_OUTPUT line: {line!r}")
        outputs[key] = value
    return outputs


def run_case(
    script: str,
    case: dict[str, object],
    temp_dir: Path,
    bash_program: str,
    environment: dict[str, str],
) -> None:
    event_name = str(case["event_name"])
    case_dir = temp_dir / event_name
    case_dir.mkdir()
    sentinel = case_dir / "command-injection-ran"
    sentinel_shell_path = sentinel.relative_to(ROOT).as_posix()
    event_path = case_dir / "event.json"
    output_path = case_dir / "github-output.txt"

    event = json.dumps(case["event"], separators=(",", ":"))
    event_path.write_text(event.replace("__SENTINEL__", sentinel_shell_path), encoding="utf-8")
    rendered = render_script(script, case).replace("__SENTINEL__", sentinel_shell_path)

    environment = environment.copy()
    environment["GITHUB_EVENT_PATH"] = event_path.relative_to(ROOT).as_posix()
    environment["GITHUB_OUTPUT"] = output_path.relative_to(ROOT).as_posix()
    result = subprocess.run(
        [bash_program, "-c", rendered],
        cwd=ROOT,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    if sentinel.exists():
        raise AssertionError(f"{event_name}: hostile PR payload executed a shell command")
    if result.returncode != 0:
        raise AssertionError(
            f"{event_name}: Resolve PR context failed ({result.returncode})\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )

    expected = case["expected"]
    assert isinstance(expected, dict)
    actual = parse_outputs(output_path)
    if actual != expected:
        raise AssertionError(f"{event_name}: output mismatch\nexpected={expected}\nactual={actual}")


def main() -> int:
    if sys.argv[1:] != ["--selftest"]:
        print(f"usage: {Path(sys.argv[0]).name} --selftest", file=sys.stderr)
        return 2

    script = resolve_context_script()
    transport_errors: list[str] = []
    if script.count(SAFE_ASSIGNMENT) != 2:
        transport_errors.append("both review branches must read .pull_request from GITHUB_EVENT_PATH")
    if TO_JSON_EXPRESSION in script:
        transport_errors.append("review PR JSON must not be interpolated into shell source")

    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    cases = fixture["cases"]
    if {case["event_name"] for case in cases} != {
        "pull_request_review",
        "pull_request_review_comment",
    }:
        raise AssertionError("fixture must cover exactly both review event branches")

    with tempfile.TemporaryDirectory(prefix=".review-context-selftest-", dir=ROOT) as directory:
        temp_dir = Path(directory)
        environment = os.environ.copy()
        if shutil.which("jq", path=environment.get("PATH")) is None:
            bin_dir = temp_dir / "bin"
            bin_dir.mkdir()
            jq_fallback = bin_dir / "jq"
            jq_fallback.write_text(JQ_FALLBACK, encoding="utf-8", newline="\n")
            jq_fallback.chmod(0o755)
            environment["PATH"] = str(bin_dir) + os.pathsep + environment.get("PATH", "")
        bash_program = shutil.which("bash", path=environment.get("PATH"))
        if bash_program is None:
            raise AssertionError("bash is required for the workflow selftest")
        for case in cases:
            run_case(script, case, temp_dir, bash_program, environment)

    if transport_errors:
        raise AssertionError("; ".join(transport_errors))
    print("DOCTRINE-EXEC-REVIEW-CONTEXT-SELFTEST: PASS branches=2 hostile_payloads=2")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
