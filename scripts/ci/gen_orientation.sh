#!/usr/bin/env bash
# OH-ORIENTATION-DIGEST-0 — generate orchestrator orientation digest from live harness data.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/orientation_digest"
readonly OUTPUT_PATH="${REPO_ROOT}/docs/orchestrator_orientation.md"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

MODE="generate"
OPEN_TARGET=""
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/gen_orientation.sh
  bash scripts/ci/gen_orientation.sh --check
  bash scripts/ci/gen_orientation.sh --open <track-md>
  bash scripts/ci/gen_orientation.sh --selftest
  bash scripts/ci/gen_orientation.sh --fixture <name>
EOF
  exit 2
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --check) MODE="check"; shift ;;
      --open)
        [[ $# -ge 2 ]] || usage
        MODE="open"
        OPEN_TARGET="$2"
        shift 2
        ;;
      --selftest) FIXTURE_MODE="selftest"; shift ;;
      --fixture)
        [[ $# -ge 2 ]] || usage
        FIXTURE_MODE="fixture"
        FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
        shift 2
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done
}

run_selftest_fixture() {
  local name="$1"
  local fix="${FIXTURES_ROOT}/${name}"
  [[ -d "$fix" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected
  expected="$(tr -d '\r' <"${fix}/expected_result.txt" | head -n 1)"
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-selftest-XXXXXX")"
  local classes="${SCRIPT_DIR}/precedented_classes.tsv"
  local binding="${SCRIPT_DIR}/binding_conditions.tsv"
  local ledger="${SCRIPT_DIR}/clearance_ledger.tsv"
  local design="${REPO_ROOT}/docs/design_0_0_8_4_7_orchestration_harness.md"
  local relay="${SCRIPT_DIR}/relay_lint.sh"
  [[ -f "${fix}/precedented_classes.tsv" ]] && classes="${fix}/precedented_classes.tsv"
  [[ -f "${fix}/binding_conditions.tsv" ]] && binding="${fix}/binding_conditions.tsv"
  [[ -f "${fix}/clearance_ledger.tsv" ]] && ledger="${fix}/clearance_ledger.tsv"
  cp "$classes" "$sandbox/precedented_classes.tsv"
  cp "$binding" "$sandbox/binding_conditions.tsv"
  cp "$ledger" "$sandbox/clearance_ledger.tsv"
  cp "$design" "$sandbox/design.md"
  cp "$relay" "$sandbox/relay_lint.sh"
  local out="${sandbox}/orientation.md"
  if [[ "$name" == "orientation_digest_selftest_stale_digest" ]]; then
    if [[ ! -f "${fix}/orientation.md" ]]; then
      echo "FAIL ${name}: missing stale orientation.md"
      return 1
    fi
    cp "${fix}/orientation.md" "$out"
  elif [[ "$name" == "orientation_digest_selftest_live_tsv_change" ]]; then
    ORIENTATION_CLASSES_TSV="${sandbox}/precedented_classes.tsv" \
    ORIENTATION_BINDING_TSV="${sandbox}/binding_conditions.tsv" \
    ORIENTATION_LEDGER_TSV="${sandbox}/clearance_ledger.tsv" \
    ORIENTATION_DESIGN_DOC="${sandbox}/design.md" \
    ORIENTATION_RELAY_LINT="${sandbox}/relay_lint.sh" \
    ORIENTATION_OUTPUT="$out" \
    bash "${SCRIPT_DIR}/gen_orientation.sh" >/dev/null
    printf 'stale-marker-row\n' >>"$sandbox/precedented_classes.tsv"
  else
    echo "FAIL ${name}: unknown fixture"
    return 1
  fi
  set +e
  ORIENTATION_CLASSES_TSV="${sandbox}/precedented_classes.tsv" \
  ORIENTATION_BINDING_TSV="${sandbox}/binding_conditions.tsv" \
  ORIENTATION_LEDGER_TSV="${sandbox}/clearance_ledger.tsv" \
  ORIENTATION_DESIGN_DOC="${sandbox}/design.md" \
  ORIENTATION_RELAY_LINT="${sandbox}/relay_lint.sh" \
  ORIENTATION_OUTPUT="$out" \
  bash "${SCRIPT_DIR}/gen_orientation.sh" --check >/dev/null 2>&1
  local rc=$?
  set -e
  local got="PASS"
  [[ "$rc" -ne 0 ]] && got="FAIL"
  if [[ "$got" == "$expected" ]]; then
    echo "PASS ${name}"
    rm -rf "$sandbox"
    return 0
  fi
  echo "FAIL ${name}"
  echo "  expected: ${expected}"
  echo "  got:      ${got}"
  rm -rf "$sandbox"
  return 1
}

seed_orientation_sandbox() {
  local sb="$1"
  mkdir -p "${sb}/scripts/ci" "${sb}/docs"
  cat >"${sb}/scripts/ci/precedented_classes.tsv" <<'EOF'
class_id	envelope	requirements	status	promotion_blocker	note
demo-class	docs/demo.md	tested_code_sha|coverage_basis	active	none	fixture
EOF
  cat >"${sb}/scripts/ci/binding_conditions.tsv" <<'EOF'
rung	condition	set_by	status	promotion_blocker
demo	none	fixture	closed	none
EOF
  cat >"${sb}/scripts/ci/clearance_ledger.tsv" <<'EOF'
verdict	class	pr	sha	date
PASS	demo-class	#1	abcdef12	2026-07-08
EOF
  cat >"${sb}/scripts/ci/doctrine_anchors.tsv" <<'EOF'
anchor_id	doc	section	trigger_domains	content_hash
demo	docs/demo.md	§0	fixture	abc123
EOF
  cat >"${sb}/scripts/ci/test_lifecycle_tracks.tsv" <<'EOF'
track_id	status	closed_at	source	note
EOF
  cat >"${sb}/scripts/ci/relay_lint.sh" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
  cat >"${sb}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
none
EOF
}

run_gen_sandbox() {
  local sb="$1"; shift
  local root="$sb"
  local classes="${sb}/scripts/ci/precedented_classes.tsv"
  local binding="${sb}/scripts/ci/binding_conditions.tsv"
  local ledger="${sb}/scripts/ci/clearance_ledger.tsv"
  local active="${sb}/scripts/ci/active_track.txt"
  local tracks="${sb}/scripts/ci/test_lifecycle_tracks.tsv"
  local relay="${sb}/scripts/ci/relay_lint.sh"
  local anchors="${sb}/scripts/ci/doctrine_anchors.tsv"
  local output="${sb}/docs/orchestrator_orientation.md"
  if command -v cygpath >/dev/null 2>&1; then
    root="$(cygpath -w "$root")"
    classes="$(cygpath -w "$classes")"
    binding="$(cygpath -w "$binding")"
    ledger="$(cygpath -w "$ledger")"
    active="$(cygpath -w "$active")"
    tracks="$(cygpath -w "$tracks")"
    relay="$(cygpath -w "$relay")"
    anchors="$(cygpath -w "$anchors")"
    output="$(cygpath -w "$output")"
  fi
  ORIENTATION_REPO_ROOT="$root" \
  ORIENTATION_CLASSES_TSV="$classes" \
  ORIENTATION_BINDING_TSV="$binding" \
  ORIENTATION_LEDGER_TSV="$ledger" \
  ORIENTATION_ACTIVE_TRACK_FILE="$active" \
  ORIENTATION_TRACKS_TSV="$tracks" \
  ORIENTATION_RELAY_LINT="$relay" \
  ORIENTATION_ANCHORS_TSV="$anchors" \
  ORIENTATION_OUTPUT="$output" \
  ORIENTATION_DESIGN_DOC= \
    bash "${SCRIPT_DIR}/gen_orientation.sh" "$@"
}

active_payload_line() {
  tr -d '\r' <"$1" | grep -v '^[[:space:]]*#' | grep -v '^[[:space:]]*$' | head -n 1
}

run_selftest_active_none() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-none-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  if ! run_gen_sandbox "$sandbox" >/dev/null; then
    echo "FAIL orientation_open_active_none"
    rm -rf "$sandbox"
    return 1
  fi
  if ! grep -q "No active production track is set" "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_open_active_none"
    rm -rf "$sandbox"
    return 1
  fi
  if ! run_gen_sandbox "$sandbox" --check >/dev/null; then
    echo "FAIL orientation_open_active_none"
    rm -rf "$sandbox"
    return 1
  fi
  if grep -q "orchestration_harness" "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_open_active_none"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_active_none"
  rm -rf "$sandbox"
  return 0
}

run_selftest_open_new_track() {
  local sandbox payload before_hash after_hash
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-new-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  if ! run_gen_sandbox "$sandbox" --open 0.0.8.4.9.5_new_idea.md >"${sandbox}/open.out"; then
    echo "FAIL orientation_open_new_track"
    rm -rf "$sandbox"
    return 1
  fi
  payload="$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")"
  before_hash="$(sha256sum "${sandbox}/docs/0.0.8.4.9.5_new_idea.md" | awk '{print $1}')"
  printf '\nPOPULATED-SENTINEL\n' >>"${sandbox}/docs/0.0.8.4.9.5_new_idea.md"
  if ! run_gen_sandbox "$sandbox" --open 0.0.8.4.9.5_new_idea.md >"${sandbox}/open2.out"; then
    echo "FAIL orientation_open_new_track"
    rm -rf "$sandbox"
    return 1
  fi
  after_hash="$(sha256sum "${sandbox}/docs/0.0.8.4.9.5_new_idea.md" | awk '{print $1}')"
  if [[ "$payload" != "docs/0.0.8.4.9.5_new_idea.md" ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: CREATED" "${sandbox}/open.out" \
    || ! grep -q "| # | Rung | Deliverable | Exit proof |" "${sandbox}/docs/0.0.8.4.9.5_new_idea.md" \
    || ! grep -q "OWNER POPULATION REQUIRED" "${sandbox}/docs/0.0.8.4.9.5_new_idea.md" \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: OPENED" "${sandbox}/open2.out" \
    || [[ "$before_hash" == "$after_hash" ]] \
    || ! grep -q "POPULATED-SENTINEL" "${sandbox}/docs/0.0.8.4.9.5_new_idea.md"; then
    echo "FAIL orientation_open_new_track"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_new_track"
  rm -rf "$sandbox"
  return 0
}

run_selftest_existing_open() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-existing-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/existing_open.md" <<'EOF'
# Existing Open

This is a production track / PR ladder workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `OPEN-RUNG` | Build it. | TODO: not complete. |
EOF
  if ! run_gen_sandbox "$sandbox" --open docs/existing_open.md >"${sandbox}/open.out"; then
    echo "FAIL orientation_open_existing_open"
    rm -rf "$sandbox"
    return 1
  fi
  if [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/existing_open.md" ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: OPENED" "${sandbox}/open.out" \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null; then
    echo "FAIL orientation_open_existing_open"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_existing_open"
  rm -rf "$sandbox"
  return 0
}

run_selftest_existing_closed_warns() {
  local sandbox before
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-closed-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/existing_closed.md" <<'EOF'
# Existing Closed

This is a closed production track / design track PR ladder.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `DONE-RUNG` | Built. | merged and closed. |
EOF
  cat >>"${sandbox}/scripts/ci/test_lifecycle_tracks.tsv" <<'EOF'
closed-track	closed	2026-07-08	docs/existing_closed.md	fixture
EOF
  before="$(cat "${sandbox}/scripts/ci/test_lifecycle_tracks.tsv")"
  if ! run_gen_sandbox "$sandbox" --open docs/existing_closed.md >"${sandbox}/open.out"; then
    echo "FAIL orientation_open_existing_closed"
    rm -rf "$sandbox"
    return 1
  fi
  if [[ "$(cat "${sandbox}/scripts/ci/test_lifecycle_tracks.tsv")" != "$before" ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: OPENED-WARN(closed-or-parked)" "${sandbox}/open.out"; then
    echo "FAIL orientation_open_existing_closed"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_existing_closed"
  rm -rf "$sandbox"
  return 0
}

# A. --open rejects evidence index under docs/tests (non-workplan; no mutation)
run_selftest_reject_evidence_index_open() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-evidence-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  mkdir -p "${sandbox}/docs/tests"
  cat >"${sandbox}/docs/tests/current_evidence_index.md" <<'EOF'
# Current Evidence Index

This is a LIVE LEDGER of current evidence. Not a production workplan.

| Guardrail | Live test | What it proves |
|---|---|---|
| example | crates/x/tests/y.rs | proves something |
EOF
  run_gen_sandbox "$sandbox" >/dev/null
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/docs/orchestrator_orientation.md" "${sandbox}/orientation.before"
  set +e
  run_gen_sandbox "$sandbox" --open docs/tests/current_evidence_index.md >"${sandbox}/open.out" 2>"${sandbox}/open.err"
  local rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -Eqi "non-workplan|FAIL\(non-workplan\)" "${sandbox}/open.out" "${sandbox}/open.err" \
    || ! cmp -s "${sandbox}/active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || ! cmp -s "${sandbox}/orientation.before" "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_open_reject_evidence_index"
    cat "${sandbox}/open.out" 2>/dev/null || true
    cat "${sandbox}/open.err" 2>/dev/null || true
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_reject_evidence_index"
  rm -rf "$sandbox"
  return 0
}

# B. active_track already points at evidence index → --check fails
run_selftest_active_evidence_index_check_fails() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-active-evidence-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  mkdir -p "${sandbox}/docs/tests"
  cat >"${sandbox}/docs/tests/current_evidence_index.md" <<'EOF'
# Current Evidence Index

This is a LIVE LEDGER of current evidence.

| Guardrail | Live test | What it proves |
|---|---|---|
| example | crates/x/tests/y.rs | proves something |
EOF
  cat >"${sandbox}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/tests/current_evidence_index.md
EOF
  set +e
  run_gen_sandbox "$sandbox" --check >"${sandbox}/check.out" 2>"${sandbox}/check.err"
  local rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -Eqi "invalid-active-track|non-workplan" "${sandbox}/check.out" "${sandbox}/check.err"; then
    echo "FAIL orientation_active_evidence_index_check"
    cat "${sandbox}/check.out" 2>/dev/null || true
    cat "${sandbox}/check.err" 2>/dev/null || true
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_active_evidence_index_check"
  rm -rf "$sandbox"
  return 0
}

# C. real workplan opens and check passes with next rung parsed
run_selftest_valid_workplan_opens() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-valid-workplan-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/design_valid_track.md" <<'EOF'
# Valid Design Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `DONE-RUNG` | Done deliverable. | DA-GRADUATED / merged #1. |
| 2 | `NEXT-RUNG` | Next deliverable. | TODO: still open. |
EOF
  if ! run_gen_sandbox "$sandbox" --open docs/design_valid_track.md >"${sandbox}/open.out"; then
    echo "FAIL orientation_valid_workplan_open"
    rm -rf "$sandbox"
    return 1
  fi
  if [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/design_valid_track.md" ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: OPENED" "${sandbox}/open.out" \
    || ! grep -q "Active pointer: \`NEXT-RUNG\`" "${sandbox}/docs/orchestrator_orientation.md" \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null; then
    echo "FAIL orientation_valid_workplan_open"
    cat "${sandbox}/open.out" 2>/dev/null || true
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_valid_workplan_open"
  rm -rf "$sandbox"
  return 0
}

run_selftest_invalid_open_path() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-invalid-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  run_gen_sandbox "$sandbox" >/dev/null
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/docs/orchestrator_orientation.md" "${sandbox}/orientation.before"
  set +e
  run_gen_sandbox "$sandbox" --open ../bad.md >/dev/null 2>&1
  local rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! cmp -s "${sandbox}/active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || ! cmp -s "${sandbox}/orientation.before" "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_open_invalid_path"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_invalid_path"
  rm -rf "$sandbox"
  return 0
}

run_selftest_active_pointer_stale_check() {
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-open-stale-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/track_a.md" <<'EOF'
# Track A

This is a production track / PR ladder.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `A-RUNG` | A. | TODO. |
EOF
  cat >"${sandbox}/docs/track_b.md" <<'EOF'
# Track B

This is a production track / PR ladder.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `B-RUNG` | B. | TODO. |
EOF
  run_gen_sandbox "$sandbox" --open docs/track_a.md >/dev/null
  cat >"${sandbox}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/track_b.md
EOF
  set +e
  run_gen_sandbox "$sandbox" --check >/dev/null 2>&1
  local rc=$?
  set -e
  if [[ "$rc" -eq 0 ]]; then
    echo "FAIL orientation_open_active_pointer_stale_check"
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_open_active_pointer_stale_check"
  rm -rf "$sandbox"
  return 0
}

run_selftest() {
  local fixtures=(
    orientation_digest_selftest_stale_digest
    orientation_digest_selftest_live_tsv_change
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_selftest_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  local open_fns=(
    run_selftest_active_none
    run_selftest_open_new_track
    run_selftest_existing_open
    run_selftest_existing_closed_warns
    run_selftest_invalid_open_path
    run_selftest_active_pointer_stale_check
    run_selftest_reject_evidence_index_open
    run_selftest_active_evidence_index_check_fails
    run_selftest_valid_workplan_opens
  )
  local fn
  for fn in "${open_fns[@]}"; do
    if ! "$fn"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  local total=$(( ${#fixtures[@]} + ${#open_fns[@]} ))
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    echo "ORIENTATION-DIGEST-SELFTEST: PASS (${total} fixtures)"
    return 0
  fi
  echo "ORIENTATION-DIGEST-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture dir" >&2; exit 1; }
    run_selftest_fixture "$(basename "$FIXTURE_DIR")"
    exit $?
  fi

  export ORIENTATION_CLASSES_TSV="${ORIENTATION_CLASSES_TSV:-${SCRIPT_DIR}/precedented_classes.tsv}"
  export ORIENTATION_BINDING_TSV="${ORIENTATION_BINDING_TSV:-${SCRIPT_DIR}/binding_conditions.tsv}"
  export ORIENTATION_LEDGER_TSV="${ORIENTATION_LEDGER_TSV:-${SCRIPT_DIR}/clearance_ledger.tsv}"
  export ORIENTATION_REPO_ROOT="${ORIENTATION_REPO_ROOT:-${REPO_ROOT}}"
  export ORIENTATION_ACTIVE_TRACK_FILE="${ORIENTATION_ACTIVE_TRACK_FILE:-${SCRIPT_DIR}/active_track.txt}"
  export ORIENTATION_TRACKS_TSV="${ORIENTATION_TRACKS_TSV:-${SCRIPT_DIR}/test_lifecycle_tracks.tsv}"
  export ORIENTATION_RELAY_LINT="${ORIENTATION_RELAY_LINT:-${SCRIPT_DIR}/relay_lint.sh}"
  export ORIENTATION_ANCHORS_TSV="${ORIENTATION_ANCHORS_TSV:-${SCRIPT_DIR}/doctrine_anchors.tsv}"
  export ORIENTATION_OUTPUT="${ORIENTATION_OUTPUT:-${OUTPUT_PATH}}"
  export ORIENTATION_MODE="$MODE"
  export ORIENTATION_OPEN_TARGET="$OPEN_TARGET"

  exec "$PYTHON_BIN" - <<'PY'
import hashlib
import csv
import os
import pathlib
import re
import sys
import tempfile

REPO_ROOT = pathlib.Path(os.environ["ORIENTATION_REPO_ROOT"])
CLASSES_TSV = pathlib.Path(os.environ["ORIENTATION_CLASSES_TSV"])
BINDING_TSV = pathlib.Path(os.environ["ORIENTATION_BINDING_TSV"])
LEDGER_TSV = pathlib.Path(os.environ["ORIENTATION_LEDGER_TSV"])
DESIGN_DOC_OVERRIDE = os.environ.get("ORIENTATION_DESIGN_DOC", "").strip()
ACTIVE_TRACK = pathlib.Path(os.environ["ORIENTATION_ACTIVE_TRACK_FILE"])
TRACKS_TSV = pathlib.Path(os.environ["ORIENTATION_TRACKS_TSV"])
RELAY_LINT = pathlib.Path(os.environ["ORIENTATION_RELAY_LINT"])
OUTPUT = pathlib.Path(os.environ["ORIENTATION_OUTPUT"])
MODE = os.environ.get("ORIENTATION_MODE", "generate")
OPEN_TARGET = os.environ.get("ORIENTATION_OPEN_TARGET", "").strip()

GENERATED_MARKER = "<!-- GENERATED by scripts/ci/gen_orientation.sh; do not edit by hand. -->"
NO_ACTIVE_TRACK = "none"
ACTIVE_TRACK_COMMENT = "# Active track design doc for orientation Next-Rung pointer. Update on track open/close."
# Leading / status-shaped completion phrases only.
# Do NOT use bare substrings like "closed" or "merged" — they false-positive on
# incomplete exit proofs that say e.g. "reuse the closed save/load battery".
COMPLETED_EXIT_PREFIX_RE = re.compile(
    r"^[\*\s_]*(?:"
    r"done\b|"
    r"complete\b|"
    r"da-graduated\b|"
    r"orchestrator-graduated\b|"
    r"da-approved\b|"
    r"da-opened\b|"
    r"da/owner-cleared\b|"
    r"owner-cleared\b|"
    r"graduated\b|"
    r"merged\b|"
    r"closed\b|"
    r"parked\b|"
    r"deferred\b|"
    r"resolved predecessor\b"
    r")",
    re.IGNORECASE,
)
COMPLETED_EXIT_STATUS_RE = re.compile(
    r"(?:"
    r"\bda-graduated\b|"
    r"\borchestrator-graduated\b|"
    r"\bda-approved\b|"
    r"\bda-opened\b|"
    r"\bda/owner-cleared\b|"
    r"\bowner-cleared\b|"
    r"\bda-equivalent\b|"
    r"merged\s+and\s+closed|"
    r"\bgraduated\s*/\s*merged\b|"
    r"\bmerged\s*\[#\d+"
    r")",
    re.IGNORECASE,
)
# Path prefixes that are never production workplans (evidence, archive, workshop).
FORBIDDEN_WORKPLAN_PREFIXES = (
    "docs/tests/",
    "docs/archive/",
    "docs/workshop/",
)
WORKPLAN_LANGUAGE_RE = re.compile(
    r"production\s+track|PR\s+ladder|workplan|design\s+track",
    re.IGNORECASE,
)
# Skeleton created by --open; must still classify as a workplan.
WORKPLAN_SKELETON_RE = re.compile(
    r"OWNER POPULATION REQUIRED|TODO-RUNG-\d+|populate the production ladder",
    re.IGNORECASE,
)
NON_WORKPLAN_REMEDY = (
    "Use a production workplan/design ladder, e.g. "
    "docs/design_0_0_8_5_clausescript_terran_pirate_galaxy.md, "
    "or set active_track.txt to none."
)


def fail(msg):
    print(f"gen_orientation: {msg}", file=sys.stderr)
    sys.exit(1)


def fail_non_workplan(detail: str) -> None:
    """Hard-fail before mutating active_track / orientation for non-workplan docs."""
    print(f"ORIENTATION-OPEN-VERDICT: FAIL(non-workplan)", file=sys.stderr)
    print(
        f"gen_orientation: FAIL(non-workplan): {detail}; remedy: {NON_WORKPLAN_REMEDY}",
        file=sys.stderr,
    )
    sys.exit(1)


def fail_invalid_active_track(detail: str) -> None:
    print(
        f"gen_orientation: FAIL(invalid-active-track: non-workplan): {detail}; "
        f"remedy: {NON_WORKPLAN_REMEDY}",
        file=sys.stderr,
    )
    sys.exit(1)


def normalize_text(raw: bytes) -> str:
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    text = raw.decode("utf-8")
    return text.replace("\r\n", "\n").replace("\r", "\n")


def clean_repo_relpath(path: str) -> str:
    rel = (path or "").replace("\\", "/").strip()
    if not rel or rel.startswith("/") or ":" in rel:
        return ""
    parts = pathlib.PurePosixPath(rel).parts
    if not parts or any(p in ("", ".", "..") for p in parts):
        return ""
    return rel


def docs_relpath(path: pathlib.Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def first_payload_line(path: pathlib.Path) -> str:
    if not path.exists():
        return ""
    for line in normalize_text(path.read_bytes()).splitlines():
        stripped = line.strip()
        if stripped and not stripped.startswith("#"):
            return stripped
    return ""


def normalize_track_doc_arg(value: str, must_exist: bool = False) -> str:
    raw = (value or "").replace("\\", "/").strip()
    if not raw:
        fail("--open requires a non-empty track doc")
    rel = clean_repo_relpath(raw)
    if not rel:
        fail(f"invalid track doc path: {value!r}")
    if pathlib.PurePosixPath(rel).suffix != ".md":
        fail("track doc path must end in .md")
    if "/" not in rel:
        docs_dir = REPO_ROOT / "docs"
        exact = docs_dir / rel
        matches = sorted(
            p.relative_to(REPO_ROOT).as_posix()
            for p in docs_dir.rglob(rel)
            if p.is_file()
        ) if docs_dir.exists() else []
        if exact.is_file():
            rel = f"docs/{rel}"
        elif len(matches) == 1:
            rel = matches[0]
        elif len(matches) > 1:
            fail(f"ambiguous track doc {value!r}: {', '.join(matches)}")
        else:
            rel = f"docs/{rel}"
    if not rel.startswith("docs/"):
        fail("track doc path must live under docs/")
    if must_exist and not (REPO_ROOT / rel).is_file():
        fail(f"track doc does not exist: {rel}")
    return rel


def is_forbidden_workplan_path(rel: str) -> bool:
    """Evidence/archive/workshop paths are never production workplans."""
    rel_norm = (rel or "").replace("\\", "/").strip()
    if not rel_norm.startswith("docs/"):
        return True
    for prefix in FORBIDDEN_WORKPLAN_PREFIXES:
        if rel_norm.startswith(prefix):
            return True
    parts = pathlib.PurePosixPath(rel_norm).parts
    # docs/<tests|archive|workshop>/...
    if len(parts) >= 2 and parts[1] in ("tests", "archive", "workshop"):
        return True
    return False


def is_ladder_header(line: str) -> bool:
    """Recognize production ladder tables in both skeleton and design-track layouts."""
    norm = re.sub(r"\s+", " ", (line or "").strip().lower())
    if not norm.startswith("|"):
        return False
    # Skeleton / orientation harness: | # | Rung | Deliverable | Exit proof |
    if norm.startswith("| # | rung |"):
        return True
    # Production design tracks: | Rung | ID | Scope | Exit proof | Tier |
    if norm.startswith("| rung | id |"):
        return True
    return False


def parse_rungs(design_text: str):
    """Parse all PR-ladder / rung tables from a design workplan.

    Supports:
      | # | Rung | Deliverable | Exit proof |
      | Rung | ID | Scope | Exit proof | Tier |

    Collects rows from every matching table in the document (multi-phase ladders).
    Returns list of (num_or_index, rung_id, deliverable_or_scope, exit_proof).
    """
    rows = []
    in_table = False
    for line in design_text.splitlines():
        stripped = line.strip()
        if is_ladder_header(stripped):
            in_table = True
            continue
        if not in_table:
            continue
        if not stripped.startswith("|"):
            in_table = False
            continue
        if stripped.startswith("|---") or re.match(r"^\|[\s\-:|]+\|$", stripped):
            continue
        parts = [p.strip() for p in stripped.strip("|").split("|")]
        if len(parts) < 4:
            continue
        # Skip accidental re-header rows mid-scan.
        c0 = parts[0].lower()
        c1 = parts[1].lower()
        if c0 in ("#", "rung") and c1 in ("rung", "id", "deliverable"):
            continue
        rows.append((parts[0], parts[1], parts[2], parts[3]))
    return rows


def has_workplan_language(text: str) -> bool:
    return bool(WORKPLAN_LANGUAGE_RE.search(text) or WORKPLAN_SKELETON_RE.search(text))


def classify_workplan(rel: str, text: str) -> tuple:
    """Return (ok: bool, reason: str) for whether rel/text is a production workplan."""
    rel_norm = (rel or "").replace("\\", "/").strip()
    if not rel_norm.startswith("docs/"):
        return False, "not-under-docs"
    if pathlib.PurePosixPath(rel_norm).suffix != ".md":
        return False, "not-markdown"
    if is_forbidden_workplan_path(rel_norm):
        return False, f"forbidden-path ({rel_norm})"
    rungs = parse_rungs(text)
    if not rungs:
        return False, "no-rung-table (parse_rungs returned empty)"
    if not has_workplan_language(text):
        return False, "missing-workplan-language (need production track / PR ladder / workplan / design track)"
    return True, ""


def read_active_track_pointer() -> dict:
    if not ACTIVE_TRACK.exists():
        return {"state": "missing", "path": "", "raw": "", "reason": "missing"}
    raw = first_payload_line(ACTIVE_TRACK)
    if not raw:
        return {"state": "empty", "path": "", "raw": "", "reason": "empty"}
    if raw == NO_ACTIVE_TRACK:
        return {"state": "none", "path": NO_ACTIVE_TRACK, "raw": raw, "reason": "no-active-track"}
    rel = clean_repo_relpath(raw)
    if not rel:
        return {"state": "invalid", "path": "", "raw": raw, "reason": "invalid-path"}
    if not rel.startswith("docs/"):
        return {"state": "invalid", "path": rel, "raw": raw, "reason": "not-under-docs"}
    if pathlib.PurePosixPath(rel).suffix != ".md":
        return {"state": "invalid", "path": rel, "raw": raw, "reason": "not-markdown"}
    if not (REPO_ROOT / rel).is_file():
        return {"state": "invalid", "path": rel, "raw": raw, "reason": "missing-target"}
    # Existing pointer at a non-workplan (e.g. evidence index) is invalid-active-track.
    text = (REPO_ROOT / rel).read_text(encoding="utf-8")
    ok, reason = classify_workplan(rel, text)
    if not ok:
        return {
            "state": "invalid",
            "path": rel,
            "raw": raw,
            "reason": f"non-workplan ({reason})",
        }
    return {"state": "doc", "path": rel, "raw": raw, "reason": ""}


def write_active_track_pointer(rel: str) -> None:
    comments = []
    if ACTIVE_TRACK.exists():
        for line in normalize_text(ACTIVE_TRACK.read_bytes()).splitlines():
            stripped = line.strip()
            if stripped.startswith("#"):
                comments.append(line.rstrip())
            elif stripped:
                break
    if not comments:
        comments = [ACTIVE_TRACK_COMMENT]
    ACTIVE_TRACK.parent.mkdir(parents=True, exist_ok=True)
    ACTIVE_TRACK.write_text("\n".join(comments + [rel]) + "\n", encoding="utf-8", newline="\n")


def active_pointer_for_render(strict: bool = False) -> dict:
    if DESIGN_DOC_OVERRIDE:
        design = pathlib.Path(DESIGN_DOC_OVERRIDE)
        if not design.is_file():
            fail(f"missing design doc override: {design}")
        return {"state": "doc", "path": docs_relpath(design) if design.is_relative_to(REPO_ROOT) else design.name,
                "raw": str(design), "reason": "", "design_doc": design}
    info = read_active_track_pointer()
    if info["state"] == "invalid" and str(info.get("reason", "")).startswith("non-workplan"):
        # Preferred: hard-fail for both generate and check (safer for CI / operators).
        fail_invalid_active_track(
            f"active_track.txt points at {info.get('path') or info.get('raw')!r}: {info['reason']}"
        )
    if strict and info["state"] in {"missing", "empty", "invalid"}:
        fail(f"active_track.txt is {info['reason']}; remedy: "
             "run `bash scripts/ci/gen_orientation.sh --open docs/<track>.md` "
             "or set active_track.txt to `none`")
    if info["state"] == "doc":
        info["design_doc"] = REPO_ROOT / info["path"]
    else:
        info["design_doc"] = None
    return info


def sha256_file(path: pathlib.Path) -> str:
    return hashlib.sha256(normalize_text(path.read_bytes()).encode("utf-8")).hexdigest()


def read_tsv(path: pathlib.Path):
    if not path.is_file():
        fail(f"missing source: {path}")
    rows = []
    with path.open(encoding="utf-8", newline="") as fh:
        for row in csv.reader(fh, delimiter="\t"):
            if not row or row[0] in ("class_id", "rung", "verdict"):
                continue
            rows.append(row)
    return rows


def md_row(values):
    return "| " + " | ".join(v.replace("|", "\\|") for v in values) + " |"


def table(headers, rows):
    lines = [md_row(headers), "| " + " | ".join("---" for _ in headers) + " |"]
    lines.extend(md_row(r) for r in rows)
    return lines


def is_completed_exit(text: str) -> bool:
    """True when a ladder cell marks the rung complete (status language).

    Production design ladders put DONE/COMPLETE/DA-GRADUATED in Scope and/or Exit-proof.
    Require status-shaped markers so narrative phrases like "closed save/load battery"
    do not false-complete an open rung.
    """
    raw = (text or "").strip()
    if not raw:
        return False
    if COMPLETED_EXIT_PREFIX_RE.search(raw):
        # Leading DONE/COMPLETE still requires a clearance/graduation cue nearby,
        # so narrative "Complete Scope Ledger…" alone does not graduate a closeout row.
        head = raw[:160]
        if re.match(r"^[\*\s_]*(done|complete)\b", head, re.IGNORECASE):
            return bool(
                re.search(
                    r"\b(da|owner|orchestrator|approved|cleared|opened|graduated|merged)\b",
                    head,
                    re.IGNORECASE,
                )
            )
        return True
    if COMPLETED_EXIT_STATUS_RE.search(raw[:200]):
        return True
    return False


def next_rung_pointer(rungs):
    for num, rung, deliv, exit_proof in rungs:
        # Production design ladders put DONE/COMPLETE status in either Scope (deliv)
        # or Exit-proof cells; treat either as completion so the first incomplete
        # rung is selected (e.g. TP-FULL-TRANSPILE-0 after graduated phases).
        if is_completed_exit(exit_proof) or is_completed_exit(deliv):
            continue
        parts = rung.split("`")
        return parts[1] if len(parts) >= 3 else rung.strip("`").strip()
    return "none"


def ledger_summary(rows, limit=5):
    if not rows:
        return ["> clearance ledger empty"]
    tail = rows[-limit:]
    out = table(["verdict", "class", "pr", "sha", "date"], [r[:5] for r in tail if len(r) >= 5])
    return out


def read_tracks():
    if not TRACKS_TSV.is_file():
        return []
    rows = []
    with TRACKS_TSV.open(encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh, delimiter="\t")
        rows.extend(reader)
    return rows


def track_state_for_doc(rel: str, design_text: str, rungs: list) -> str:
    for row in read_tracks():
        source = (row.get("source") or "").replace("\\", "/").strip()
        if source == rel or pathlib.PurePosixPath(source).name == pathlib.PurePosixPath(rel).name:
            status = (row.get("status") or "").strip().lower()
            if status in {"closed", "parked"}:
                return status
            if status == "open":
                return "open"
            if status:
                return status
    if rungs and next_rung_pointer(rungs) == NO_ACTIVE_TRACK:
        return "end-state"
    if rungs:
        return "open"
    return "unknown-open-assumed"


def skeleton_doc(rel: str) -> str:
    title = pathlib.PurePosixPath(rel).stem.replace("_", " ").replace("-", " ").title()
    return "\n".join([
        f"# {title}",
        "",
        "Status: DRAFT / OWNER POPULATION REQUIRED",
        "",
        "## Purpose",
        "",
        "TODO: owner/DA/operator states why this production track exists.",
        "",
        "## Production Target",
        "",
        "TODO: name the production subsystem, user-facing path, or invariant ladder this track will change.",
        "",
        "## Ladder",
        "",
        "| # | Rung | Deliverable | Exit proof |",
        "|---|---|---|---|",
        "| 1 | `TODO-RUNG-1` | TODO: populate the first production rung before assigning coding work. | TODO: owner/DA/operator must define the proof before coding begins. |",
        "",
        "## Operator Notes",
        "",
        "Owner/DA/operator must populate the production ladder/rungs before coding agents begin.",
        "",
        "References:",
        "- `docs/orchestrator_orientation.md`",
        "- `scripts/ci/gen_orientation.sh --open`",
        "- `docs/handoff_template.md`",
        "- `docs/agent_onboarding.md`",
        "- PROJECT SPINE: TODO",
        "",
    ]) + "\n"


def render_orientation(active_info: dict) -> tuple:
    classes = read_tsv(CLASSES_TSV)
    binding = read_tsv(BINDING_TSV)
    ledger_rows = read_tsv(LEDGER_TSV)
    anchors_tsv = pathlib.Path(os.environ["ORIENTATION_ANCHORS_TSV"])

    design_doc = active_info.get("design_doc")
    design_text = ""
    rungs = []
    next_rung = NO_ACTIVE_TRACK
    track_state = active_info.get("reason", "")
    if design_doc is not None:
        design_text = design_doc.read_text(encoding="utf-8")
        rungs = parse_rungs(design_text)
        next_rung = next_rung_pointer(rungs)
        track_state = track_state_for_doc(active_info.get("path", ""), design_text, rungs)

    sources = [
        ("precedented_classes.tsv", CLASSES_TSV),
        ("binding_conditions.tsv", BINDING_TSV),
        ("clearance_ledger.tsv", LEDGER_TSV),
    ]
    if ACTIVE_TRACK.exists():
        sources.append(("active_track.txt", ACTIVE_TRACK))
    if design_doc is not None:
        sources.append((design_doc.name, design_doc))
    sources.extend([
        ("relay_lint.sh", RELAY_LINT),
        ("doctrine_anchors.tsv", anchors_tsv),
    ])
    manifest = [(name, sha256_file(path)) for name, path in sources]

    class_rows = []
    for row in classes:
        if len(row) < 6:
            continue
        class_rows.append((row[0], row[1], row[2], row[3], row[4], row[5]))

    binding_rows = []
    for row in binding:
        if len(row) < 5:
            continue
        binding_rows.append(tuple(row[:5]))

    lines = [
    "# Orchestrator Orientation",
    "",
    GENERATED_MARKER,
    "",
    "> Operational orientation generated from live harness TSVs. Not a doctrine anchor summary.",
    "> Regenerate: `bash scripts/ci/gen_orientation.sh`",
    "",
    "## MANDATORY (ORCHESTRATOR burden): run `/clearance`, then respond to the state it emits",
    "",
    "Do NOT relay a DA-review / graduation handoff without first running the clearance router yourself for the",
    "current PR -- a verdict quoted in someone else's report does NOT satisfy this. Run `/clearance` (GHA) or",
    "`bash scripts/ci/clearance_check.sh --pr <n>`. It emits exactly one state; **respond to it, do not interpret**",
    "it -- the router already codifies freshness/routing, so there is nothing for you to judge:",
    "",
    "| emitted state | your action |",
    "| --- | --- |",
    "| no `CLEARANCE-VERDICT` line / `CLEARANCE-STATUS: PENDING` | run in flight -- **WAIT and re-read.** Not a mismatch, not a failure, not a handoff. |",
    "| `ORCHESTRATOR-CLEARABLE` | **merge it yourself. Do NOT escalate to DA.** |",
    "| `DA-RESERVE(<reason>)` | the ONLY valid basis for a DA relay -- quote it verbatim. |",
    "| `DA-RESERVE(harness-error)` / `FAIL(<remedy>)` | remedy the harness/PR; **not** a DA review. |",
    "",
    "`relay_lint` FAILs a DA relay lacking a fresh PR-head-bound verdict (`FAIL(missing-clearance-verdict)`); a",
    "chat handoff is outside CI, same rule on your honor. **Never SHA-match** (`tested_code_sha`, or a stale",
    "sticky `head_sha`) in place of the router -- that is the recurring kabuki whenever the mechanism is skipped.",
    "",
    "**DA side:** the DA does NOT re-run `/clearance` as a required pass -- a green `relay_lint` is",
    "DA-equivalent for routing (the orchestrator already paid this cost). The DA runs the router only on",
    "spot-audit or when a relay is genuinely suspect. See design 0.0.8.4.8 section 4C.",
    "",
    "## Source Stamps",
    "",
    ]
    lines.extend(table(["source", "sha256"], manifest))
    lines.extend([
    "",
])
    if design_doc is None:
        lines.extend([
            "## Active Track / Rung Summary",
            "",
            "No active production track is set.",
            "",
            "Run:",
            "",
            "```bash",
            "bash scripts/ci/gen_orientation.sh --open docs/<track>.md",
            "```",
            "",
            "to open or create a production track before assigning coding work.",
            "",
            "## Next Rung Pointer",
            "",
            f"Active pointer: `{NO_ACTIVE_TRACK}`",
            "",
        ])
    else:
        lines.extend([
            f"## Active Track / Rung Summary (`{design_doc.name}`)",
            "",
            f"Track state: `{track_state}`",
            "",
        ])
        rung_table = []
        for num, rung, deliverable, exit_proof in rungs:
            short = exit_proof
            if len(short) > 120:
                short = short[:117] + "..."
            rung_table.append((num, rung.strip("`"), deliverable[:80], short))
        lines.extend(table(["#", "rung", "deliverable", "exit proof"], rung_table))
        lines.extend([
            "",
            "## Next Rung Pointer",
            "",
            f"Active pointer: `{next_rung}`",
            "",
        ])
    lines.extend([
    "",
    "## Cold-Start Entrypoint",
    "",
    "Cold-start entrypoint: run `bash scripts/ci/orient.sh --role=coding|orchestrator|da` and carry the emitted ORIENT-RECEIPT.",
    "",
    "## Clearance Router Verdict Meanings",
    "",
    "| verdict | meaning |",
    "| --- | --- |",
    "| `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE` | precedented class matched; binding conditions discharged; required proof fields present |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(unclassified-scope)` | resolved non-empty diff with no precedented class match (not novelty) |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(novelty)` | explicit `novelty_claim: YES` + `novelty_basis`; **overrides** matched-class clearance; not a generic unmatched-diff fallback |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(class-envelope-violation)` | matched class but diff violates workshop_only / class path envelope |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(engine-scope-violation)` | matched class forbids engine crate/src and the diff touches engine scope |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(module-marker-shape-mismatch)` | corpus-module-marker-sweep shape fails inventory deletion rules |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(harness-error)` | malformed data, ambiguous class, empty/unresolved requested target, or script error |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` | PR touches router/lint/harness gate surfaces (self-application refusal) |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(binding-conditions)` | open binding condition blocks clearance for matched class |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(class-suspended)` | precedented class row status=suspended |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(triage-missing)` | INSPECT delta without landed /triage row (check 7 live) |",
    "| `CLEARANCE-VERDICT: FAIL(missing-novelty-basis...)` | `novelty_claim: YES` without valid `novelty_basis` |",
    "| `CLEARANCE-VERDICT: FAIL(remedy)` | named fix required before re-attempt (CI not green, missing proof fields, etc.) |",
    "",
    "`DA-RESERVE(novelty)` is explicit-claim-only and overrides matched-class clearance. A novelty claim must",
    "include `novelty_basis` naming the unanticipated implementation discovery or substrate improvement.",
    "Without `novelty_basis`, clearance fails. Unclassified diffs without `novelty_claim` route to",
    "`DA-RESERVE(unclassified-scope)`. Scope-envelope violations route to their specific reason",
    "(`class-envelope-violation`, `engine-scope-violation`, `module-marker-shape-mismatch`).",
    "",
    "## Precedented Classes (active)",
    "",
    ])
    active_classes = [r for r in class_rows if len(r) > 4 and r[4] != "retired"]
    lines.extend(table(["class_id", "envelope", "requirements", "status", "promotion_blocker"], [r[:5] for r in active_classes]))
    lines.extend([
    "",
    "## Binding Conditions",
    "",
    ])
    lines.extend(table(["rung", "condition", "set_by", "status", "promotion_blocker"], binding_rows))
    lines.extend([
    "",
    "## Clearance Ledger (recent)",
    "",
    ])
    lines.extend(ledger_summary(ledger_rows))
    lines.extend([
    "",
    "## Relay Lint Required Blocks",
    "",
    "Required relay/handoff sections (M3): Status; PR/branch/merge; What changed; Load-bearing proofs; Scope Ledger; Conformance; Known gaps; Graduation routing.",
    "",
    "Graduation routing must name: CI verdict, triage entries, risk class, falsification check, recommended posture.",
    "",
    "Proof identity fields required in relay body:",
    "- `tested_code_sha: <8+ hex>`",
    "- `coverage_basis: PASS` (or explicit coverage basis)",
    "",
    f"relay_lint.sh schema stamp: `{sha256_file(RELAY_LINT)[:12]}`",
    "",
    "## tested_code_sha + coverage_basis Rule",
    "",
    "Clearance classes requiring workshop/production proof must carry citable `tested_code_sha` and `coverage_basis` in the PR/relay body.",
    "GPU/desktop/bevy proof is owner-local execution with recorded `DOCTRINE-TESTS-VERDICT: PASS` bound to the same SHA — GHA validates binding, never executes GPU legs.",
    "",
    "## Escalation / DA-RESERVE Posture",
    "",
    "- unclassified-scope, class-envelope-violation, engine-scope-violation, module-marker-shape-mismatch → DA review (precise reason; not novelty rhetoric).",
    "- Novelty (`novelty_claim: YES` + `novelty_basis`) overrides matched-class clearance → DA review routing.",
    "- `novelty_claim: YES` without `novelty_basis` → FAIL(missing-novelty-basis); not clearable.",
    "- `tp-workshop-candidate-proof` → workshop-homed 0.0.8.5 TP candidate proofs only (not mapeditor API / sealed crates / GPU / picker / closeout).",
    "- `tp-admitted-clause-api-composition` → DA-admitted limited ClauseScript composition (StructuralRebindReady) only; not picker/runtime/GameMode/RF/closeout.",
    "- binding-conditions, class-suspended, triage-missing → DA review routing.",
    "- gate-wiring → deep audit; harness surfaces are never self-mergeable.",
    "- harness-error → fix data/target resolution before re-run.",
    "- FAIL(remedy) → apply named remedy and re-run clearance.",
    "- DA exit-proof stamp (binding): after DA pass (graduate-merge / formal admission / denial), DA updates active workplan Exit proof + results COMPLETE + orientation regen and merges the stamp PR; not orchestrator residual (see agent_onboarding).",
    "- DA verify-the-tree (weighted): require for code-facing / long-lifecycle / horizontally impactful load-bearing; relaxed for pure policy, stamps, light residual (see agent_onboarding).",
    "",
    "## Orientation Receipt (ORIENT-RECEIPT)",
    "",
    "Run `bash scripts/ci/orient.sh --role=coding|orchestrator|da` to emit a rule-source-bound receipt.",
    "",
    "Schema:",
    "- `ORIENT-RECEIPT: <12-char hash>` - stable hash over role + orientation_rule_stamp",
    "- `role: coding|orchestrator|da`",
    "- `orientation_rule_stamp: <16-char hash>` - hash over `precedented_classes.tsv`, `binding_conditions.tsv`, and `doctrine_anchors.tsv`",
    "- `orientation_digest_sha: <sha256 of docs/orchestrator_orientation.md>` (informational only; prose digest churn does not stale receipts)",
    "- `generated_at: source-bound` (non-authoritative; validation uses the rule stamp)",
    "",
    "Role meanings:",
    "- `coding` — clearance contract, inner-loop commands, precedented classes",
    "- `orchestrator` — full orientation digest; ORCHESTRATOR-GRADUATED status-stamp residual only",
    "- `da` — rung table, binding conditions, escalation posture; after a passed verdict, exit-proof stamp + merge is DA duty (agent_onboarding)",
    "",
    "Receipt freshness: relay-lint compares claimed `orientation_rule_stamp` to the live rule stamp; mismatch -> `FAIL(stale-orient-receipt)`.",
    "Relay-lint receipt rule: gate-wiring handoffs require a valid receipt for the declared role.",
    "Rule-source edits, including `doctrine_anchors.tsv`, stale `ORIENT-RECEIPT` values.",
    "",
    "## Doctrine Anchors (ANCHOR-ACK)",
    "",
    "Table: `scripts/ci/doctrine_anchors.tsv` (`anchor_id | doc | section | trigger_domains | content_hash`).",
    "",
    "ANCHOR-ACK schema: `ANCHOR-ACK: <anchor_id>@<12-char content_hash>`",
    "",
    "Trigger-domain rule: relays touching a domain must ack anchors listing that domain (e.g. `movement-front`, `gate-wiring`).",
    "",
    "Relay-lint failures: `missing-anchor-ack`, `stale-anchor-ack`, `unknown-anchor`.",
    "",
    "Run `bash scripts/ci/anchor_check.sh --check` after anchor table edits.",
    "",
    "## Inner Loop (coding agent)",
    "",
    "```bash",
    "bash scripts/ci/orient.sh --role=coding",
    "bash scripts/ci/anchor_check.sh --check",
    "bash scripts/ci/clearance_check.sh --selftest",
    "bash scripts/ci/relay_lint.sh --selftest",
    "bash scripts/ci/gen_orientation.sh --check",
    "bash scripts/ci/doctrine_selftest.sh",
    "bash scripts/ci/doctrine_scan.sh",
    "```",
    "",
    "## GHA Comment Commands",
    "",
    "- `/clearance` — M1 router verdict",
    "- `/relay-lint` — M3 relay lint verdict",
    "- `/orient` — M2 orientation digest (this page)",
    "- `/orient role=orchestrator|coding|da` — role-filtered subset",
    "- `/anchor <anchor_id|trigger_domain>` — verbatim anchored doctrine text",
    "",
    ])
    return "\n".join(lines).rstrip() + "\n", track_state, next_rung


def write_orientation(generated: str) -> bool:
    current = OUTPUT.read_text(encoding="utf-8") if OUTPUT.is_file() else ""
    if current == generated:
        return False
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(generated, encoding="utf-8", newline="\n")
    return True


def check_orientation() -> int:
    active_info = active_pointer_for_render(strict=True)
    generated, _track_state, _next_rung = render_orientation(active_info)
    if not OUTPUT.is_file():
        fail(f"{OUTPUT} is missing; remedy: bash scripts/ci/gen_orientation.sh")
    current = OUTPUT.read_text(encoding="utf-8")
    if GENERATED_MARKER not in current:
        fail("orientation digest missing generated marker; do not hand-edit")
    if current != generated:
        with tempfile.NamedTemporaryFile("w", encoding="utf-8", delete=False, suffix=".md") as tmp:
            tmp.write(generated)
            tmp_path = tmp.name
        fail(
            f"{OUTPUT} is stale; expected output written to {tmp_path}; "
            "remedy: run `bash scripts/ci/gen_orientation.sh` or "
            "`bash scripts/ci/gen_orientation.sh --open docs/<track>.md` and commit docs/orchestrator_orientation.md"
        )
    print("gen_orientation --check: PASS")
    return 0


def generate_orientation() -> int:
    active_info = active_pointer_for_render(strict=False)
    if active_info["state"] == "invalid":
        fail(f"active_track.txt is {active_info['reason']}; remedy: "
             "run `bash scripts/ci/gen_orientation.sh --open docs/<track>.md` "
             "or set active_track.txt to `none`")
    generated, _track_state, _next_rung = render_orientation(active_info)
    write_orientation(generated)
    rel = OUTPUT
    try:
        rel = OUTPUT.relative_to(REPO_ROOT)
    except ValueError:
        pass
    print(f"generated {rel}")
    return 0


def open_track() -> int:
    if not OPEN_TARGET:
        fail("--open requires exactly one track doc")
    rel = normalize_track_doc_arg(OPEN_TARGET)
    # Reject forbidden paths before any create/mutate (evidence index under docs/tests/, etc.).
    if is_forbidden_workplan_path(rel):
        fail_non_workplan(
            f"{rel} is under a forbidden non-workplan path "
            f"(docs/tests|docs/archive|docs/workshop)"
        )
    target = REPO_ROOT / rel
    old_info = read_active_track_pointer()
    # When old pointer is non-workplan invalid, still report its raw path for diagnostics.
    old = old_info.get("raw") or old_info.get("path") or old_info.get("reason") or "missing"
    created = False
    if not target.exists():
        # New skeleton may not be created under forbidden paths (already rejected above).
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(skeleton_doc(rel), encoding="utf-8", newline="\n")
        created = True
    elif not target.is_file():
        fail(f"track doc target is not a file: {rel}")

    design_text = target.read_text(encoding="utf-8")
    # Existing non-workplan markdown must FAIL before mutating active_track or orientation.
    ok, reason = classify_workplan(rel, design_text)
    if not ok:
        fail_non_workplan(f"{rel}: {reason}")

    rungs = parse_rungs(design_text)
    track_state = track_state_for_doc(rel, design_text, rungs)

    changed_pointer = old_info.get("path") != rel
    if changed_pointer:
        write_active_track_pointer(rel)
    # Re-read after pointer write; must now classify as a real workplan.
    active_info = active_pointer_for_render(strict=True)
    generated, track_state, next_rung = render_orientation(active_info)
    regenerated = write_orientation(generated)

    if created:
        verdict = "CREATED"
        next_action = "populate production track ladder/rungs before coding work"
    elif track_state in {"closed", "parked", "end-state"}:
        verdict = "OPENED-WARN(closed-or-parked)"
        next_action = "owner/DA must clarify whether this is a reopen, audit, or new successor track before production coding"
    else:
        verdict = "OPENED"
        next_action = "orientation aligned"

    print(f"ORIENTATION-OPEN-VERDICT: {verdict}")
    print(f"active_track_from: {old}")
    print(f"active_track_to: {rel}")
    print(f"orientation_regenerated: {'yes' if regenerated else 'no'}")
    print(f"track_state: {'new' if created else track_state}")
    print(f"next_rung: {next_rung}")
    print(f"next_action: {next_action}")
    return 0


if MODE == "check":
    sys.exit(check_orientation())
if MODE == "open":
    sys.exit(open_track())
sys.exit(generate_orientation())
PY
}

main "$@"
