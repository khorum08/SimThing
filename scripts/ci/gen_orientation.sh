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
FORCE_OWNER=""
FIXTURE_MODE=""
FIXTURE_DIR=""
SELFTEST_FAILURES=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/gen_orientation.sh
  bash scripts/ci/gen_orientation.sh --check
  bash scripts/ci/gen_orientation.sh --open <track-md>
  bash scripts/ci/gen_orientation.sh --park <track-md>
  bash scripts/ci/gen_orientation.sh --unpark <track-md>
  bash scripts/ci/gen_orientation.sh --open <track-md> --force-owner "<directive text>"
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
      --park)
        [[ $# -ge 2 ]] || usage
        MODE="park"
        OPEN_TARGET="$2"
        shift 2
        ;;
      --unpark)
        [[ $# -ge 2 ]] || usage
        MODE="unpark"
        OPEN_TARGET="$2"
        shift 2
        ;;
      --force-owner)
        [[ $# -ge 2 ]] || usage
        FORCE_OWNER="$2"
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
  mkdir -p "${sb}/scripts/ci" "${sb}/docs" "${sb}/handoffs"
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
  cat >"${sb}/scripts/ci/owner_directives.tsv" <<'EOF'
directive	scope	status	set_by
EOF
  cat >"${sb}/scripts/ci/closeout_artifacts.tsv" <<'EOF'
path	leased_at	disposition	closeout_track	note
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
  local owner_directives="${sb}/scripts/ci/owner_directives.tsv"
  local closeout_artifacts="${sb}/scripts/ci/closeout_artifacts.tsv"
  local handoffs="${sb}/handoffs"
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
    owner_directives="$(cygpath -w "$owner_directives")"
    closeout_artifacts="$(cygpath -w "$closeout_artifacts")"
    handoffs="$(cygpath -w "$handoffs")"
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
  ORIENTATION_OWNER_DIRECTIVES="$owner_directives" \
  ORIENTATION_CLOSEOUT_ARTIFACTS="$closeout_artifacts" \
  ORIENTATION_HANDOFFS_DIR="$handoffs" \
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

# HD-POINTER-LIFECYCLE-GATE-0: fixtures for the outgoing-track lifecycle gate.
seed_gate_outgoing_track() {
  # $1=sandbox $2=doc-relpath $3=STATUS-word
  local sb="$1" rel="$2" status="$3"
  cat >"${sb}/${rel}" <<EOF
# Outgoing Track

> **Status: ${status} (fixture).** gate fixture.

This is a production track / PR ladder workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | \`GATE-RUNG\` | Build it. | fixture state. |
EOF
  cat >"${sb}/scripts/ci/active_track.txt" <<EOF
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
${rel}
EOF
}

# Flipping the pointer off a still-OPEN outgoing track is refused, with no writes.
run_selftest_gate_open_refused() {
  local sandbox rc
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-open-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  seed_gate_outgoing_track "$sandbox" "docs/outgoing.md" "OPEN"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  set +e
  run_gen_sandbox "$sandbox" --open docs/successor.md >"${sandbox}/open.out" 2>"${sandbox}/open.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: FAIL(outgoing-track-open)" "${sandbox}/open.err" \
    || ! cmp -s "${sandbox}/active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || [[ -e "${sandbox}/docs/successor.md" ]]; then
    echo "FAIL orientation_gate_open_refused"
    cat "${sandbox}/open.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_open_refused"
  rm -rf "$sandbox"; return 0
}

# Flipping off a PARKED or CLOSED outgoing track proceeds (pointer moves).
run_selftest_gate_parked_closed_allowed() {
  local sandbox rc state
  for state in PARKED CLOSED; do
    sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-${state}-XXXXXX")"
    seed_orientation_sandbox "$sandbox"
    seed_gate_outgoing_track "$sandbox" "docs/outgoing.md" "$state"
    set +e
    run_gen_sandbox "$sandbox" --open docs/successor.md >"${sandbox}/open.out" 2>"${sandbox}/open.err"
    rc=$?
    set -e
    if [[ "$rc" -ne 0 ]] \
      || ! grep -q "ORIENTATION-OPEN-VERDICT: CREATED" "${sandbox}/open.out" \
      || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/successor.md" ]]; then
      echo "FAIL orientation_gate_${state}_allowed"
      cat "${sandbox}/open.out" "${sandbox}/open.err" 2>/dev/null || true
      rm -rf "$sandbox"; return 1
    fi
    rm -rf "$sandbox"
  done
  echo "PASS orientation_gate_parked_closed_allowed"
  return 0
}

# --force-owner overrides the OPEN refusal AND records an owner_directives.tsv row (no silent force).
run_selftest_gate_force_owner_records() {
  local sandbox rc
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-force-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  seed_gate_outgoing_track "$sandbox" "docs/outgoing.md" "OPEN"
  set +e
  run_gen_sandbox "$sandbox" --open docs/successor.md --force-owner "Owner override: successor authorized" >"${sandbox}/open.out" 2>"${sandbox}/open.err"
  rc=$?
  set -e
  if [[ "$rc" -ne 0 ]] \
    || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/successor.md" ]] \
    || ! grep -q "ORIENTATION-OPEN-FORCE-OWNER: recorded scope=outgoing" "${sandbox}/open.err" \
    || ! grep -qE "Owner override: successor authorized[[:space:]]+outgoing[[:space:]]+active[[:space:]]+Owner-" "${sandbox}/scripts/ci/owner_directives.tsv"; then
    echo "FAIL orientation_gate_force_owner_records"
    cat "${sandbox}/open.out" "${sandbox}/open.err" 2>/dev/null || true
    cat "${sandbox}/scripts/ci/owner_directives.tsv" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_records"
  rm -rf "$sandbox"; return 0
}

# Cross-root env-seam bypass: a FAKE repo root (PARKED outgoing) must not authorize writes to a
# VICTIM checkout's pointer/orientation/directives via the ORIENTATION_* seams. Refuse before any write.
run_selftest_gate_incoherent_root_refused() {
  local victim fake rc
  victim="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-victim-XXXXXX")"
  fake="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-fake-XXXXXX")"
  seed_orientation_sandbox "$victim"
  seed_gate_outgoing_track "$victim" "docs/outgoing.md" "OPEN"
  seed_orientation_sandbox "$fake"
  seed_gate_outgoing_track "$fake" "docs/outgoing.md" "PARKED"
  cp "${victim}/scripts/ci/active_track.txt" "${victim}/active.before"
  cp "${victim}/scripts/ci/owner_directives.tsv" "${victim}/owner.before"
  set +e
  ORIENTATION_REPO_ROOT="$fake" \
  ORIENTATION_CLASSES_TSV="${fake}/scripts/ci/precedented_classes.tsv" \
  ORIENTATION_BINDING_TSV="${fake}/scripts/ci/binding_conditions.tsv" \
  ORIENTATION_LEDGER_TSV="${fake}/scripts/ci/clearance_ledger.tsv" \
  ORIENTATION_ACTIVE_TRACK_FILE="${victim}/scripts/ci/active_track.txt" \
  ORIENTATION_TRACKS_TSV="${fake}/scripts/ci/test_lifecycle_tracks.tsv" \
  ORIENTATION_RELAY_LINT="${fake}/scripts/ci/relay_lint.sh" \
  ORIENTATION_ANCHORS_TSV="${fake}/scripts/ci/doctrine_anchors.tsv" \
  ORIENTATION_OWNER_DIRECTIVES="${victim}/scripts/ci/owner_directives.tsv" \
  ORIENTATION_OUTPUT="${victim}/docs/orchestrator_orientation.md" \
  ORIENTATION_DESIGN_DOC= \
    bash "${SCRIPT_DIR}/gen_orientation.sh" --open docs/successor.md >"${victim}/open.out" 2>"${victim}/open.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: FAIL(incoherent-root)" "${victim}/open.err" \
    || ! cmp -s "${victim}/active.before" "${victim}/scripts/ci/active_track.txt" \
    || ! cmp -s "${victim}/owner.before" "${victim}/scripts/ci/owner_directives.tsv" \
    || [[ -e "${victim}/docs/orchestrator_orientation.md" ]] \
    || [[ -e "${victim}/docs/successor.md" ]]; then
    echo "FAIL orientation_gate_incoherent_root"
    cat "${victim}/open.err" 2>/dev/null || true
    rm -rf "$victim" "$fake"; return 1
  fi
  echo "PASS orientation_gate_incoherent_root"
  rm -rf "$victim" "$fake"; return 0
}

# Two-phase --force-owner: a forced open to an existing non-workplan target must fail with ZERO
# writes — no stray Owner directive, no pointer/orientation change, target untouched.
run_selftest_gate_force_owner_invalid_target_no_write() {
  local sandbox rc tgt_before tgt_after
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-forceinvalid-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  seed_gate_outgoing_track "$sandbox" "docs/outgoing.md" "OPEN"
  cat >"${sandbox}/docs/not_a_workplan.md" <<'EOF'
# Not A Workplan

Just prose. No rung table, no ladder, no workplan language.
EOF
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  tgt_before="$(sha256sum "${sandbox}/docs/not_a_workplan.md" | awk '{print $1}')"
  set +e
  run_gen_sandbox "$sandbox" --open docs/not_a_workplan.md --force-owner "force to invalid target" >"${sandbox}/open.out" 2>"${sandbox}/open.err"
  rc=$?
  set -e
  tgt_after="$(sha256sum "${sandbox}/docs/not_a_workplan.md" | awk '{print $1}')"
  if [[ "$rc" -eq 0 ]] \
    || ! grep -Eqi "non-workplan|FAIL" "${sandbox}/open.err" \
    || ! cmp -s "${sandbox}/active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || ! cmp -s "${sandbox}/owner.before" "${sandbox}/scripts/ci/owner_directives.tsv" \
    || [[ "$tgt_before" != "$tgt_after" ]] \
    || [[ -e "${sandbox}/docs/orchestrator_orientation.md" ]]; then
    echo "FAIL orientation_gate_force_owner_invalid_target"
    cat "${sandbox}/open.err" 2>/dev/null || true
    cat "${sandbox}/scripts/ci/owner_directives.tsv" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_invalid_target"
  rm -rf "$sandbox"; return 0
}

# --force-owner without --open is rejected (no silent, unused Owner directive).
run_selftest_gate_force_owner_requires_open() {
  local sandbox rc
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-forcenoopen-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  set +e
  run_gen_sandbox "$sandbox" --force-owner "force with no open" >"${sandbox}/out" 2>"${sandbox}/err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "force-owner requires --open" "${sandbox}/err" \
    || ! cmp -s "${sandbox}/owner.before" "${sandbox}/scripts/ci/owner_directives.tsv"; then
    echo "FAIL orientation_gate_force_owner_requires_open"
    cat "${sandbox}/err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_requires_open"
  rm -rf "$sandbox"; return 0
}

# Transactional forced open: a VALID target but an invalid directive-table mutation target (a
# directory) under the SAME coherent root must abort atomically — reaching beyond target
# classification, leaving pointer, orientation, directive path, and target unchanged.
run_selftest_gate_force_owner_unwritable_directive_no_write() {
  local sandbox rc tgt_before tgt_after
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-forcedir-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  seed_gate_outgoing_track "$sandbox" "docs/outgoing.md" "OPEN"
  cat >"${sandbox}/docs/valid_target.md" <<'EOF'
# Valid Target

production track / PR ladder workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `R` | build it | TODO: not complete. |
EOF
  # Corrupt the owner-directives mutation target: an existing directory (still under the coherent root).
  rm -f "${sandbox}/scripts/ci/owner_directives.tsv"
  mkdir "${sandbox}/scripts/ci/owner_directives.tsv"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  tgt_before="$(sha256sum "${sandbox}/docs/valid_target.md" | awk '{print $1}')"
  set +e
  run_gen_sandbox "$sandbox" --open docs/valid_target.md --force-owner "force with bad directive target" >"${sandbox}/open.out" 2>"${sandbox}/open.err"
  rc=$?
  set -e
  tgt_after="$(sha256sum "${sandbox}/docs/valid_target.md" | awk '{print $1}')"
  if [[ "$rc" -eq 0 ]] \
    || ! grep -Eq "FAIL\(forced-preflight\)|FAIL\(forced-transition-aborted\)" "${sandbox}/open.err" \
    || ! cmp -s "${sandbox}/active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || [[ "$tgt_before" != "$tgt_after" ]] \
    || [[ -e "${sandbox}/docs/orchestrator_orientation.md" ]] \
    || [[ ! -d "${sandbox}/scripts/ci/owner_directives.tsv" ]] \
    || [[ -n "$(ls -A "${sandbox}/scripts/ci/owner_directives.tsv")" ]]; then
    echo "FAIL orientation_gate_force_owner_unwritable_directive"
    cat "${sandbox}/open.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_unwritable_directive"
  rm -rf "$sandbox"; return 0
}

# Shared helper: seed a forced-open scenario (OPEN outgoing + a valid target) and snapshot surfaces.
seed_gate_force_scenario() {
  local sb="$1"
  seed_orientation_sandbox "$sb"
  seed_gate_outgoing_track "$sb" "docs/outgoing.md" "OPEN"
  cat >"${sb}/docs/valid_target.md" <<'EOF'
# Valid Target

production track / PR ladder workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `R` | build it | TODO: not complete. |
EOF
}

# Assert all four forced-open surfaces are unchanged (pointer, owner-directives, target, orientation).
assert_gate_surfaces_unchanged() {
  local sb="$1" tgt_before="$2"
  cmp -s "${sb}/active.before" "${sb}/scripts/ci/active_track.txt" || return 1
  cmp -s "${sb}/owner.before" "${sb}/scripts/ci/owner_directives.tsv" || return 1
  [[ "$tgt_before" == "$(sha256sum "${sb}/docs/valid_target.md" | awk '{print $1}')" ]] || return 1
  [[ ! -e "${sb}/docs/orchestrator_orientation.md" ]] || return 1
  return 0
}

# Preflight semantic falsifier: an existing 4-column directive row with an invalid status must fail
# the planned-bytes validation before any write.
run_selftest_gate_force_owner_invalid_status_row() {
  local sandbox rc tgt_before
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-badstatus-XXXXXX")"
  seed_gate_force_scenario "$sandbox"
  printf 'directive\tscope\tstatus\tset_by\nsome directive\t0.0.0\tbogus\tOwner-2026-07-14\n' \
    >"${sandbox}/scripts/ci/owner_directives.tsv"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  tgt_before="$(sha256sum "${sandbox}/docs/valid_target.md" | awk '{print $1}')"
  set +e
  run_gen_sandbox "$sandbox" --open docs/valid_target.md --force-owner "new directive" >"${sandbox}/out" 2>"${sandbox}/err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: FAIL(forced-preflight)" "${sandbox}/err" \
    || ! assert_gate_surfaces_unchanged "$sandbox" "$tgt_before"; then
    echo "FAIL orientation_gate_force_owner_invalid_status_row"
    cat "${sandbox}/err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_invalid_status_row"
  rm -rf "$sandbox"; return 0
}

# Preflight semantic falsifier: a planned (directive, outgoing-scope) that duplicates an existing
# ACTIVE pair must fail before any write.
run_selftest_gate_force_owner_duplicate_active_pair() {
  local sandbox rc tgt_before
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-dup-XXXXXX")"
  seed_gate_force_scenario "$sandbox"
  # outgoing_track_id("docs/outgoing.md") == "outgoing"; pre-seed the identical active pair.
  printf 'directive\tscope\tstatus\tset_by\ndup directive\toutgoing\tactive\tOwner-2026-07-14\n' \
    >"${sandbox}/scripts/ci/owner_directives.tsv"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  tgt_before="$(sha256sum "${sandbox}/docs/valid_target.md" | awk '{print $1}')"
  set +e
  run_gen_sandbox "$sandbox" --open docs/valid_target.md --force-owner "dup directive" >"${sandbox}/out" 2>"${sandbox}/err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: FAIL(forced-preflight)" "${sandbox}/err" \
    || ! assert_gate_surfaces_unchanged "$sandbox" "$tgt_before"; then
    echo "FAIL orientation_gate_force_owner_duplicate_active_pair"
    cat "${sandbox}/err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_duplicate_active_pair"
  rm -rf "$sandbox"; return 0
}

# Automated rollback proof: a deterministic selftest-only fault fires on the real commit_forced_open
# path AFTER the pointer + orientation writes; the abort must roll back all four surfaces.
run_selftest_gate_force_owner_rollback_after_writes() {
  local sandbox rc tgt_before
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-gate-rollback-XXXXXX")"
  seed_gate_force_scenario "$sandbox"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/active.before"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  tgt_before="$(sha256sum "${sandbox}/docs/valid_target.md" | awk '{print $1}')"
  if [[ -e "${sandbox}/docs/orchestrator_orientation.md" ]]; then
    echo "FAIL orientation_gate_force_owner_rollback_after_writes (precondition: orientation exists)"
    rm -rf "$sandbox"; return 1
  fi
  set +e
  ORIENTATION_FORCE_FAULT_AFTER_WRITES=1 run_gen_sandbox "$sandbox" --open docs/valid_target.md --force-owner "rollback proof" >"${sandbox}/out" 2>"${sandbox}/err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "ORIENTATION-OPEN-VERDICT: FAIL(forced-transition-aborted)" "${sandbox}/err" \
    || ! assert_gate_surfaces_unchanged "$sandbox" "$tgt_before"; then
    echo "FAIL orientation_gate_force_owner_rollback_after_writes"
    cat "${sandbox}/err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_gate_force_owner_rollback_after_writes"
  rm -rf "$sandbox"; return 0
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

run_selftest_authoritative_park_pointer() {
  local sandbox out
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-park-pointer-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/design_owner_gated_track.md" <<'EOF'
# Owner-Gated Design Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `ORIGINAL-RUNG` | Original work. | REMEDIAL-SUPERSEDED by the repair. |
| REMEDIAL | `REPAIR-RUNG` | Repair. | DA-GRADUATED / merged #2. |
| OWNER | `OWNER-CLOSURE-RUNG` | Owner-only closure. | DEFERRED / Owner-gated. |

| Item | State |
|---|---|
| Active open rung | none — Phase 1 COMPLETE; awaiting Owner direction for the next UI/UX ladder (Phase 2+) |
EOF
  cat >"${sandbox}/scripts/ci/binding_conditions.tsv" <<'EOF'
rung	condition	set_by	status	promotion_blocker
OWNER-CLOSURE-RUNG	track-closeout-blocked-until-explicit-owner-authorization	Owner-fixture	active	OWNER-CLOSURE-RUNG
EOF
  if ! run_gen_sandbox "$sandbox" --open docs/design_owner_gated_track.md >/dev/null; then
    echo "FAIL orientation_authoritative_park_pointer"
    rm -rf "$sandbox"
    return 1
  fi
  out="${sandbox}/docs/orchestrator_orientation.md"
  if ! grep -qF "Active pointer: none — awaiting Owner direction for the next UI/UX ladder" "$out" \
    || grep -qF 'Active pointer: `ORIGINAL-RUNG`' "$out" \
    || grep -qF 'Active pointer: `OWNER-CLOSURE-RUNG`' "$out" \
    || ! grep -qF 'track-closeout-blocked-until-explicit-owner-authorization' "$out" \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null; then
    echo "FAIL orientation_authoritative_park_pointer"
    cat "$out" 2>/dev/null || true
    rm -rf "$sandbox"
    return 1
  fi
  echo "PASS orientation_authoritative_park_pointer"
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

run_selftest_cold_start_spine() {
  local sandbox out
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-spine-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1; then
    echo "FAIL cold_start_spine_generate"
    rm -rf "$sandbox"
    return 1
  fi
  out="${sandbox}/docs/orchestrator_orientation.md"
  local failures=0
  for needle in \
    "## Cold-Start Spine (constitutional pointers)" \
    "field-policy-time-decisions" \
    "spec-fidelity-anti-ceremony" \
    "founding-ontology-invariants" \
    "docs/invariants.md" \
    "drift-detectors-six-line" \
    "anchor_query.sh" \
    "Doctrine lookup entrypoint"
  do
    if ! grep -qF "$needle" "$out"; then
      echo "FAIL cold_start_spine_missing:${needle}"
      failures=$((failures + 1))
    fi
  done
  # Forbidden: raw doctrine prose blocks / §3–§7 paste
  if grep -qE '## 8\. Time, decisions|### 0\.6 Specification Fidelity|## 9\. The drift detectors' "$out"; then
    echo "FAIL cold_start_spine_forbidden_prose"
    failures=$((failures + 1))
  fi
  # Deterministic regen
  local first second
  first="$(tr -d '\r' <"$out")"
  run_gen_sandbox "$sandbox" >/dev/null 2>&1 || true
  second="$(tr -d '\r' <"$out")"
  if [[ "$first" != "$second" ]]; then
    echo "FAIL cold_start_spine_nondeterministic"
    failures=$((failures + 1))
  fi
  rm -rf "$sandbox"
  if [[ "$failures" -eq 0 ]]; then
    echo "PASS cold_start_spine"
    return 0
  fi
  return 1
}

seed_park_track_sandbox() {
  local sb="$1"
  seed_orientation_sandbox "$sb"
  cat >"${sb}/docs/design_1_2_3_studio.md" <<'EOF'
# Studio Fixture

> **Status: OPEN / fixture.**

This is a production track / PR ladder workplan.

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| OWNER | `STUDIO-OWNER-CLOSURE-0` | owner close | TODO | DA |
| 2 | `STUDIO-SECOND-0` | second close | TODO | DA |
EOF
  cat >"${sb}/scripts/ci/binding_conditions.tsv" <<'EOF'
rung	condition	set_by	status	promotion_blocker
STUDIO-OWNER-CLOSURE-0	owner-close	Owner	active	STUDIO-OWNER-CLOSURE-0
other	keep	Owner	active	other
STUDIO-SECOND-0	second-close	Owner	active	STUDIO-SECOND-0
other-2	keep2	Owner	active	other-2
EOF
  cat >"${sb}/scripts/ci/owner_directives.tsv" <<'EOF'
directive	scope	status	set_by
Studio remains parked	1.2.3	active	Owner
Keep me	9.9.9	active	Owner
Second studio directive	1.2.3-addon	active	Owner
Keep me too	9.9.9	active	Owner
EOF
  cat >"${sb}/scripts/ci/test_lifecycle_tracks.tsv" <<'EOF'
track_id	status	closed_at	source	note
1.2.3-studio	open	-	docs/design_1_2_3_studio.md	fixture
9.9.9-other	open	-	docs/other.md	fixture
EOF
  cat >"${sb}/scripts/ci/closeout_artifacts.tsv" <<'EOF'
path	leased_at	disposition	closeout_track	note
docs/tests/studio_manifest.tsv	2026-07-01	lease	1.2.3-studio	moved-a
docs/tests/other_manifest.tsv	2026-07-01	lease	9.9.9-other	keep
docs/tests/studio_second_manifest.tsv	2026-07-01	lease	1.2.3-addon	moved-b
docs/tests/other_second_manifest.tsv	2026-07-01	lease	9.9.9-other	keep-b
docs/tests/studio_vanished_manifest.tsv	2026-07-01	lease	1.2.3-studio	moved-c
EOF
  mkdir -p "${sb}/docs/tests"
  : >"${sb}/docs/tests/studio_manifest.tsv"
  : >"${sb}/docs/tests/other_manifest.tsv"
  : >"${sb}/docs/tests/studio_second_manifest.tsv"
  : >"${sb}/docs/tests/other_second_manifest.tsv"
  : >"${sb}/docs/tests/studio_vanished_manifest.tsv"
  cat >"${sb}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md" <<'EOF'
rung: STUDIO-OWNER-CLOSURE-0
track: 1.2.3
HD-RECEIPT: fixture
EOF
  cat >"${sb}/handoffs/STUDIO-SECOND-0.hd.md" <<'EOF'
rung: STUDIO-SECOND-0
track: 1.2.3
HD-RECEIPT: fixture-second
EOF
  cat >"${sb}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/design_1_2_3_studio.md
EOF
}

run_selftest_park_unpark_lifecycle() {
  local sandbox rc
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-park-life-XXXXXX")"
  seed_park_track_sandbox "$sandbox"
  cp "${sandbox}/scripts/ci/binding_conditions.tsv" "${sandbox}/binding.before"
  cp "${sandbox}/scripts/ci/owner_directives.tsv" "${sandbox}/owner.before"
  cp "${sandbox}/scripts/ci/closeout_artifacts.tsv" "${sandbox}/artifacts.before"
  cp "${sandbox}/scripts/ci/test_lifecycle_tracks.tsv" "${sandbox}/tracks.before"
  cp "${sandbox}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md" "${sandbox}/handoff.before"
  cp "${sandbox}/handoffs/STUDIO-SECOND-0.hd.md" "${sandbox}/handoff_second.before"
  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >"${sandbox}/park.out" 2>"${sandbox}/park.err"; then
    echo "FAIL orientation_park_unpark_lifecycle"
    cat "${sandbox}/park.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  local ok=0
  if grep -q "STUDIO-OWNER-CLOSURE-0" "${sandbox}/scripts/ci/binding_conditions.tsv"; then ok=1; fi
  if grep -q "Studio remains parked" "${sandbox}/scripts/ci/owner_directives.tsv"; then ok=1; fi
  if [[ -e "${sandbox}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md" ]]; then ok=1; fi
  if [[ -e "${sandbox}/handoffs/STUDIO-SECOND-0.hd.md" ]]; then ok=1; fi
  if ! grep -q "SIMTHING-PARKED-TRACK:BEGIN" "${sandbox}/docs/design_1_2_3_studio.md"; then ok=1; fi
  if [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "none" ]]; then ok=1; fi
  if [[ "$ok" -ne 0 ]]; then
    echo "FAIL orientation_park_moves_rows_out"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_park_moves_rows_out"
  echo "PASS orientation_hd_fold"

  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --unpark docs/design_1_2_3_studio.md >"${sandbox}/unpark.out" 2>"${sandbox}/unpark.err"; then
    echo "FAIL orientation_unpark_restores"
    cat "${sandbox}/unpark.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  if ! cmp -s "${sandbox}/binding.before" "${sandbox}/scripts/ci/binding_conditions.tsv" \
    || ! cmp -s "${sandbox}/owner.before" "${sandbox}/scripts/ci/owner_directives.tsv" \
    || ! cmp -s "${sandbox}/artifacts.before" "${sandbox}/scripts/ci/closeout_artifacts.tsv" \
    || ! cmp -s "${sandbox}/tracks.before" "${sandbox}/scripts/ci/test_lifecycle_tracks.tsv" \
    || ! cmp -s "${sandbox}/handoff.before" "${sandbox}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md" \
    || ! cmp -s "${sandbox}/handoff_second.before" "${sandbox}/handoffs/STUDIO-SECOND-0.hd.md" \
    || grep -q "SIMTHING-PARKED-TRACK:BEGIN" "${sandbox}/docs/design_1_2_3_studio.md" \
    || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/design_1_2_3_studio.md" ]]; then
    echo "FAIL orientation_unpark_restores_byte_exact"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_unpark_restores_byte_exact"
  echo "PASS orientation_hd_restore"

  set +e
  ORIENTATION_OPEN_PR_FIXTURE="#1 STUDIO-OWNER-CLOSURE-0 still open" run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >"${sandbox}/blocked.out" 2>"${sandbox}/blocked.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] || ! grep -q "FAIL(open-prs-for-track)" "${sandbox}/blocked.err"; then
    echo "FAIL orientation_park_open_pr_blocks"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_park_open_pr_blocks"

  set +e
  ORIENTATION_OPEN_PR_FIXTURE='[{"number":2,"title":"generic maintenance","headRefName":"generic-maintenance","body":"Rung: STUDIO-SECOND-0"}]' run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >"${sandbox}/blocked_body.out" 2>"${sandbox}/blocked_body.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] || ! grep -q "FAIL(open-prs-for-track)" "${sandbox}/blocked_body.err"; then
    echo "FAIL orientation_park_open_pr_body_blocks"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_park_open_pr_body_blocks"

  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >/dev/null 2>&1; then
    echo "FAIL orientation_repark_first"
    rm -rf "$sandbox"; return 1
  fi
  cat >>"${sandbox}/scripts/ci/binding_conditions.tsv" <<'EOF'
STUDIO-OWNER-CLOSURE-0	owner-close	Owner-2	retired	STUDIO-OWNER-CLOSURE-0
EOF
  cat >>"${sandbox}/scripts/ci/owner_directives.tsv" <<'EOF'
Studio remains parked	1.2.3	retired	Owner-2
Fresh directive	1.2.3	active	Owner
EOF
  cat >>"${sandbox}/scripts/ci/closeout_artifacts.tsv" <<'EOF'
docs/tests/studio_manifest.tsv	2026-07-02	lease	1.2.3-studio	replaced-note
EOF
  cat >"${sandbox}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md" <<'EOF'
rung: STUDIO-OWNER-CLOSURE-0
track: 1.2.3
HD-RECEIPT: fixture-newer
newer handoff content
EOF
  rm -f "${sandbox}/docs/tests/studio_vanished_manifest.tsv"
  awk '
    /SIMTHING-PARKED-TRACK:BEGIN/ { in_block=1 }
    !in_block && index($0, "STUDIO-SECOND-0") { next }
    { print }
  ' "${sandbox}/docs/design_1_2_3_studio.md" >"${sandbox}/docs/design_1_2_3_studio.md.tmp"
  mv "${sandbox}/docs/design_1_2_3_studio.md.tmp" "${sandbox}/docs/design_1_2_3_studio.md"
  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >"${sandbox}/repark.out" 2>&1 \
    || ! grep -q "Fresh directive" "${sandbox}/docs/design_1_2_3_studio.md" \
    || ! grep -q "Studio remains parked" "${sandbox}/docs/design_1_2_3_studio.md" \
    || ! grep -q "newer handoff content" "${sandbox}/docs/design_1_2_3_studio.md" \
    || ! grep -q "drop_superseded_table_rows: 3" "${sandbox}/repark.out" \
    || ! grep -q "drop_superseded_handoffs: 1" "${sandbox}/repark.out" \
    || ! grep -q "drop_vanished_table_rows: 2" "${sandbox}/repark.out" \
    || ! grep -q "drop_stale_handoffs: 1" "${sandbox}/repark.out"; then
    echo "FAIL orientation_repark_supersede"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_repark_supersede"

  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --unpark docs/design_1_2_3_studio.md >"${sandbox}/stale_unpark.out" 2>&1 \
    || grep -q "STUDIO-SECOND-0" "${sandbox}/scripts/ci/binding_conditions.tsv" \
    || [[ -e "${sandbox}/handoffs/STUDIO-SECOND-0.hd.md" ]] \
    || grep -q "studio_vanished_manifest" "${sandbox}/scripts/ci/closeout_artifacts.tsv" \
    || [[ "$(grep -c '^STUDIO-OWNER-CLOSURE-0	owner-close	' "${sandbox}/scripts/ci/binding_conditions.tsv")" != "1" ]] \
    || ! grep -q '^STUDIO-OWNER-CLOSURE-0	owner-close	Owner-2	retired	STUDIO-OWNER-CLOSURE-0$' "${sandbox}/scripts/ci/binding_conditions.tsv" \
    || [[ "$(grep -c '^Studio remains parked	1.2.3	' "${sandbox}/scripts/ci/owner_directives.tsv")" != "1" ]] \
    || ! grep -q '^Studio remains parked	1.2.3	retired	Owner-2$' "${sandbox}/scripts/ci/owner_directives.tsv" \
    || [[ "$(grep -c '^docs/tests/studio_manifest.tsv	' "${sandbox}/scripts/ci/closeout_artifacts.tsv")" != "1" ]] \
    || ! grep -q '^docs/tests/studio_manifest.tsv	2026-07-02	lease	1.2.3-studio	replaced-note$' "${sandbox}/scripts/ci/closeout_artifacts.tsv" \
    || ! grep -q "newer handoff content" "${sandbox}/handoffs/STUDIO-OWNER-CLOSURE-0.hd.md"; then
    echo "FAIL orientation_stale_referents_not_resurrected"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_stale_referents_not_resurrected"

  if ! ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >/dev/null 2>&1; then
    echo "FAIL orientation_repark_for_tamper"
    rm -rf "$sandbox"; return 1
  fi

  perl -0pi -e 's/"park_receipt": "[0-9a-f]{12}"/"park_receipt": "000000000000"/' "${sandbox}/docs/design_1_2_3_studio.md"
  set +e
  run_gen_sandbox "$sandbox" --unpark docs/design_1_2_3_studio.md >"${sandbox}/tamper.out" 2>"${sandbox}/tamper.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] || ! grep -q "receipt drift" "${sandbox}/tamper.err"; then
    echo "FAIL orientation_tampered_receipt_fails"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_tampered_receipt_fails"
  rm -rf "$sandbox"; return 0
}

seed_pointer_divergence_sandbox() {
  # Shared ladder for HC-4 pointer-divergence fixtures.
  # R1 graduated, R2 open (not dispatched), R3 open.
  local sb="$1"
  seed_orientation_sandbox "$sb"
  cat >"${sb}/docs/design_pointer_div.md" <<'EOF'
# Pointer Divergence Fixture Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `GRAD-RUNG` | Graduated work. | DA-GRADUATED / merged #99. |
| 2 | `OPEN-RUNG` | Legitimate next work. | NOT STARTED |
| 3 | `LATER-RUNG` | Later work. | TODO |

| Item | State |
|---|---|
| Active open rung | `OPEN-RUNG` |
EOF
  cat >"${sb}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/design_pointer_div.md
EOF
}

run_selftest_pointer_divergence_lint() {
  local sandbox rc out
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-ptr-div-XXXXXX")"
  seed_pointer_divergence_sandbox "$sandbox"

  # Positive control: legitimate not-yet-dispatched next rung + regenerated digest → --check PASS
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1 \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null 2>&1 \
    || ! grep -qF 'Active pointer: `OPEN-RUNG`' "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_pointer_divergence_open_passes"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_pointer_divergence_open_passes"

  # none-form still coherent (authoritative row may say none while ladder has open rungs)
  cat >"${sandbox}/docs/design_pointer_div.md" <<'EOF'
# Pointer Divergence Fixture Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `GRAD-RUNG` | Graduated work. | DA-GRADUATED / merged #99. |
| 2 | `OPEN-RUNG` | Legitimate next work. | NOT STARTED |

| Item | State |
|---|---|
| Active open rung | none — Phase 1 COMPLETE; awaiting Owner direction for the next UI/UX ladder (Phase 2+) |
EOF
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1 \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null 2>&1 \
    || ! grep -qF "Active pointer: none — awaiting Owner direction for the next UI/UX ladder" \
         "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_pointer_divergence_none_form_passes"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_pointer_divergence_none_form_passes"

  # graduated-rung-named-as-pointer: orientation is byte-fresh after generate attempt
  # fails, so build a coherent open pointer first, then mutate only the Active row
  # to name the graduated rung while keeping orientation from the prior generate —
  # WITHOUT the lint, --check would only compare bytes and stay green; WITH the lint
  # it FAILs even when we re-sync orientation (generate itself refuses).
  cat >"${sandbox}/docs/design_pointer_div.md" <<'EOF'
# Pointer Divergence Fixture Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `GRAD-RUNG` | Graduated work. | DA-GRADUATED / merged #99. |
| 2 | `OPEN-RUNG` | Legitimate next work. | NOT STARTED |

| Item | State |
|---|---|
| Active open rung | `OPEN-RUNG` |
EOF
  run_gen_sandbox "$sandbox" >/dev/null 2>&1 || true
  # Mutate Active open rung → graduated; orientation still names OPEN-RUNG (stale vs would-be generate)
  # Force the divergent authoring state and re-generate: pre-fix generate would succeed and
  # write Active pointer: GRAD-RUNG; fixed generate FAILs pointer-divergence.
  perl -0pi -e 's/Active open rung \| `OPEN-RUNG`/Active open rung | `GRAD-RUNG`/' \
    "${sandbox}/docs/design_pointer_div.md"
  set +e
  run_gen_sandbox "$sandbox" >"${sandbox}/grad_gen.out" 2>"${sandbox}/grad_gen.err"
  local gen_rc=$?
  run_gen_sandbox "$sandbox" --check >"${sandbox}/grad_check.out" 2>"${sandbox}/grad_check.err"
  local check_rc=$?
  set -e
  if [[ "$gen_rc" -eq 0 ]] || [[ "$check_rc" -eq 0 ]] \
    || ! grep -q "FAIL(pointer-divergence)" "${sandbox}/grad_gen.err" \
    || ! grep -q "GRAD-RUNG" "${sandbox}/grad_gen.err"; then
    echo "FAIL orientation_pointer_divergence_graduated_fails"
    cat "${sandbox}/grad_gen.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_pointer_divergence_graduated_fails"

  # unknown-rung: Active open rung names a rung absent from the ladder
  cat >"${sandbox}/docs/design_pointer_div.md" <<'EOF'
# Pointer Divergence Fixture Track

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `GRAD-RUNG` | Graduated work. | DA-GRADUATED / merged #99. |
| 2 | `OPEN-RUNG` | Legitimate next work. | NOT STARTED |

| Item | State |
|---|---|
| Active open rung | `GHOST-RUNG` |
EOF
  set +e
  run_gen_sandbox "$sandbox" >"${sandbox}/ghost_gen.out" 2>"${sandbox}/ghost_gen.err"
  gen_rc=$?
  run_gen_sandbox "$sandbox" --check >"${sandbox}/ghost_check.out" 2>"${sandbox}/ghost_check.err"
  check_rc=$?
  set -e
  if [[ "$gen_rc" -eq 0 ]] || [[ "$check_rc" -eq 0 ]] \
    || ! grep -q "FAIL(pointer-divergence)" "${sandbox}/ghost_gen.err" \
    || ! grep -q "GHOST-RUNG" "${sandbox}/ghost_gen.err" \
    || ! grep -q "absent from the ladder" "${sandbox}/ghost_gen.err"; then
    echo "FAIL orientation_pointer_divergence_unknown_fails"
    cat "${sandbox}/ghost_gen.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_pointer_divergence_unknown_fails"

  # --park refuses divergent pointer (no mutation; same family as open-PR refusal)
  cat >"${sandbox}/docs/design_pointer_div.md" <<'EOF'
# Pointer Divergence Fixture Track

> **Status: OPEN / fixture.**

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `GRAD-RUNG` | Graduated work. | DA-GRADUATED / merged #99. |
| 2 | `OPEN-RUNG` | Legitimate next work. | NOT STARTED |

| Item | State |
|---|---|
| Active open rung | `GRAD-RUNG` |
EOF
  cp "${sandbox}/docs/design_pointer_div.md" "${sandbox}/park_doc.before"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/park_active.before"
  set +e
  ORIENTATION_SKIP_OPEN_PR_CHECK=1 run_gen_sandbox "$sandbox" --park docs/design_pointer_div.md \
    >"${sandbox}/park_div.out" 2>"${sandbox}/park_div.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! grep -q "FAIL(divergent-pointer)" "${sandbox}/park_div.err" \
    || ! cmp -s "${sandbox}/park_doc.before" "${sandbox}/docs/design_pointer_div.md" \
    || ! cmp -s "${sandbox}/park_active.before" "${sandbox}/scripts/ci/active_track.txt"; then
    echo "FAIL orientation_park_refuses_divergent_pointer"
    cat "${sandbox}/park_div.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_park_refuses_divergent_pointer"

  rm -rf "$sandbox"
  return 0
}

run_selftest_scope_cell_not_false_complete() {
  # HC-3/HC-4 case: Scope cell *describes* completion words (DA-GRADUATED / merged [#N])
  # while Exit-proof is still open. Pre-fix next_rung_pointer treated is_completed_exit(deliv)
  # as completion and skipped the rung; fixed path must select it.
  local sandbox
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-scope-false-XXXXXX")"
  seed_orientation_sandbox "$sandbox"
  cat >"${sandbox}/docs/design_scope_false.md" <<'EOF'
# Scope False-Complete Fixture

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `SCOPE-NARRATIVE-RUNG` | Names a rung whose exit-proof is DA-GRADUATED / merged [#1355] — narrative only. | NOT STARTED — awaiting implementer. |
| 2 | `NEXT-AFTER-SCOPE` | Should not be selected while SCOPE-NARRATIVE-RUNG is open. | TODO |
EOF
  cat >"${sandbox}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/design_scope_false.md
EOF
  # Pre-fix bite: old rule (exit OR deliv) would skip SCOPE-NARRATIVE-RUNG.
  local pre_fix
  pre_fix="$(
    python - <<'PY'
import re
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
COMPLETED_EXIT_PREFIX_RE = re.compile(
    r"^[\*\s_]*(?:"
    r"done\b|complete\b|da-graduated\b|orchestrator-graduated\b|da-approved\b|"
    r"da-opened\b|da/owner-cleared\b|owner-cleared\b|graduated\b|merged\b|"
    r"closed\b|parked\b|deferred\b|remedial-superseded\b|resolved predecessor\b"
    r")",
    re.IGNORECASE,
)
def is_completed_exit(text):
    raw = (text or "").strip()
    if not raw:
        return False
    if COMPLETED_EXIT_PREFIX_RE.search(raw):
        head = raw[:160]
        if re.match(r"^[\*\s_]*(done|complete)\b", head, re.IGNORECASE):
            return bool(re.search(
                r"\b(da|owner|orchestrator|approved|cleared|opened|graduated|merged)\b",
                head, re.IGNORECASE,
            ))
        return True
    if COMPLETED_EXIT_STATUS_RE.search(raw[:200]):
        return True
    return False
deliv = "Names a rung whose exit-proof is DA-GRADUATED / merged [#1355] — narrative only."
exit_proof = "NOT STARTED — awaiting implementer."
# old rule
if is_completed_exit(exit_proof) or is_completed_exit(deliv):
    print("NEXT-AFTER-SCOPE")
else:
    print("SCOPE-NARRATIVE-RUNG")
PY
  )"
  if [[ "$pre_fix" != "NEXT-AFTER-SCOPE" ]]; then
    echo "FAIL orientation_scope_cell_pre_fix_would_false_complete (got ${pre_fix})"
    rm -rf "$sandbox"; return 1
  fi
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1 \
    || ! grep -qF 'Active pointer: `SCOPE-NARRATIVE-RUNG`' "${sandbox}/docs/orchestrator_orientation.md" \
    || grep -qF 'Active pointer: `NEXT-AFTER-SCOPE`' "${sandbox}/docs/orchestrator_orientation.md" \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null 2>&1; then
    echo "FAIL orientation_scope_cell_not_false_complete"
    cat "${sandbox}/docs/orchestrator_orientation.md" 2>/dev/null | head -n 40 || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_scope_cell_not_false_complete"
  rm -rf "$sandbox"
  return 0
}

run_selftest_ladder_column_integrity() {
  # HC-5: escaped/bare pipe in a cell shifts columns so parts[3] is not Exit-proof.
  # Pre-fix parse takes the shifted parts[3] and --check stays green; fixed path
  # FAILs(ladder-column-count) naming the row + remedy. Clean row still passes.
  local sandbox pre_exit pre_ncols
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-col-integrity-XXXXXX")"
  seed_orientation_sandbox "$sandbox"

  # --- Falsifier fixture: escaped pipe in Scope cell ---
  cat >"${sandbox}/docs/design_col_integrity.md" <<'EOF'
# Ladder Column Integrity Fixture

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `PIPE-ROW` | Scope has an escaped pipe here \| still scope | REAL-EXIT-PROOF-MARKER |
| 2 | `NEXT-ROW` | Later work. | TODO |
EOF
  cat >"${sandbox}/scripts/ci/active_track.txt" <<'EOF'
# Active track design doc for orientation Next-Rung pointer. Update on track open/close.
docs/design_col_integrity.md
EOF

  # Pre-fix bite: naive split (no column-count assert) mis-assigns Exit-proof and
  # would accept the row because it only bounds too-few columns.
  pre_exit="$(
    python - <<'PY'
row = "| 1 | `PIPE-ROW` | Scope has an escaped pipe here \\| still scope | REAL-EXIT-PROOF-MARKER |"
parts = [p.strip() for p in row.strip().strip("|").split("|")]
# Old parse_rungs: skip only if len(parts) < 4, then take parts[3] as exit_proof.
assert len(parts) >= 4, parts
print(parts[3])
PY
  )"
  pre_ncols="$(
    python - <<'PY'
row = "| 1 | `PIPE-ROW` | Scope has an escaped pipe here \\| still scope | REAL-EXIT-PROOF-MARKER |"
parts = [p.strip() for p in row.strip().strip("|").split("|")]
print(len(parts))
PY
  )"
  if [[ "$pre_exit" == "REAL-EXIT-PROOF-MARKER" ]]; then
    echo "FAIL orientation_ladder_column_pre_fix_would_misread (exit still correct: ${pre_exit})"
    rm -rf "$sandbox"; return 1
  fi
  if [[ "$pre_ncols" -le 4 ]]; then
    echo "FAIL orientation_ladder_column_pre_fix_would_misread (ncols=${pre_ncols}, need >4)"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_ladder_column_pre_fix_misreads_exit (got exit='${pre_exit}' ncols=${pre_ncols})"

  # Fixed path: generate and --check both FAIL(ladder-column-count), name the row.
  set +e
  run_gen_sandbox "$sandbox" >"${sandbox}/pipe_gen.out" 2>"${sandbox}/pipe_gen.err"
  local gen_rc=$?
  run_gen_sandbox "$sandbox" --check >"${sandbox}/pipe_check.out" 2>"${sandbox}/pipe_check.err"
  local check_rc=$?
  set -e
  if [[ "$gen_rc" -eq 0 ]] || [[ "$check_rc" -eq 0 ]] \
    || ! grep -q "FAIL(ladder-column-count)" "${sandbox}/pipe_gen.err" \
    || ! grep -q "PIPE-ROW" "${sandbox}/pipe_gen.err" \
    || ! grep -q "say it without a pipe" "${sandbox}/pipe_gen.err" \
    || ! grep -q "FAIL(ladder-column-count)" "${sandbox}/pipe_check.err"; then
    echo "FAIL orientation_ladder_column_escaped_pipe_fails"
    cat "${sandbox}/pipe_gen.err" 2>/dev/null || true
    cat "${sandbox}/pipe_check.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_ladder_column_escaped_pipe_fails"

  # Clean row: exact header column count → generate + --check PASS; exit-proof correct.
  cat >"${sandbox}/docs/design_col_integrity.md" <<'EOF'
# Ladder Column Integrity Fixture

This document is the authoritative production track and PR ladder / workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `CLEAN-ROW` | Scope has no pipe character at all. | REAL-EXIT-PROOF-MARKER |
| 2 | `NEXT-ROW` | Later work. | TODO |

| Item | State |
|---|---|
| Active open rung | `CLEAN-ROW` |
EOF
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1 \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null 2>&1 \
    || ! grep -qF 'Active pointer: `CLEAN-ROW`' "${sandbox}/docs/orchestrator_orientation.md" \
    || ! grep -qF 'REAL-EXIT-PROOF-MARKER' "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_ladder_column_clean_row_passes"
    cat "${sandbox}/docs/orchestrator_orientation.md" 2>/dev/null | head -n 50 || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_ladder_column_clean_row_passes"

  # 5-column production shape: bare pipe in Scope also fails; clean 5-col passes.
  cat >"${sandbox}/docs/design_col_integrity.md" <<'EOF'
# Ladder Column Integrity Fixture (5-col)

This document is the authoritative production track and PR ladder / workplan.

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| R1 | `BARE-PIPE-ROW` | Mentions a bare | pipe in scope | NOT STARTED | Std |
EOF
  set +e
  run_gen_sandbox "$sandbox" --check >"${sandbox}/bare_check.out" 2>"${sandbox}/bare_check.err"
  check_rc=$?
  set -e
  if [[ "$check_rc" -eq 0 ]] \
    || ! grep -q "FAIL(ladder-column-count)" "${sandbox}/bare_check.err" \
    || ! grep -q "BARE-PIPE-ROW" "${sandbox}/bare_check.err"; then
    echo "FAIL orientation_ladder_column_bare_pipe_5col_fails"
    cat "${sandbox}/bare_check.err" 2>/dev/null || true
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_ladder_column_bare_pipe_5col_fails"

  cat >"${sandbox}/docs/design_col_integrity.md" <<'EOF'
# Ladder Column Integrity Fixture (5-col clean)

This document is the authoritative production track and PR ladder / workplan.

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| R1 | `FIVE-COL-CLEAN` | Scope without any pipe character. | NOT STARTED | Std |

| Item | State |
|---|---|
| Active open rung | `FIVE-COL-CLEAN` |
EOF
  if ! run_gen_sandbox "$sandbox" >/dev/null 2>&1 \
    || ! run_gen_sandbox "$sandbox" --check >/dev/null 2>&1 \
    || ! grep -qF 'Active pointer: `FIVE-COL-CLEAN`' "${sandbox}/docs/orchestrator_orientation.md"; then
    echo "FAIL orientation_ladder_column_5col_clean_passes"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_ladder_column_5col_clean_passes"

  rm -rf "$sandbox"
  return 0
}

run_selftest_park_rollback_and_virgin_unpark() {
  local sandbox rc before
  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-park-rollback-XXXXXX")"
  seed_park_track_sandbox "$sandbox"
  before="$(sha256sum "${sandbox}/scripts/ci/binding_conditions.tsv" "${sandbox}/docs/design_1_2_3_studio.md" | sha256sum | awk '{print $1}')"
  set +e
  ORIENTATION_SKIP_OPEN_PR_CHECK=1 ORIENTATION_FORCE_FAULT_AFTER_WRITES=1 run_gen_sandbox "$sandbox" --park docs/design_1_2_3_studio.md >"${sandbox}/fault.out" 2>"${sandbox}/fault.err"
  rc=$?
  set -e
  local after
  after="$(sha256sum "${sandbox}/scripts/ci/binding_conditions.tsv" "${sandbox}/docs/design_1_2_3_studio.md" | sha256sum | awk '{print $1}')"
  if [[ "$rc" -eq 0 ]] || [[ "$before" != "$after" ]] || ! grep -q "transaction-rolled-back" "${sandbox}/fault.err"; then
    echo "FAIL orientation_park_post_write_rollback"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_park_post_write_rollback"
  perl -0pi -e 's#> \*\*Status: OPEN / fixture\.\*\*#> **Status: PARKED / fixture.**#' "${sandbox}/docs/design_1_2_3_studio.md"
  run_gen_sandbox "$sandbox" >/dev/null
  cp "${sandbox}/docs/design_1_2_3_studio.md" "${sandbox}/virgin.doc.before"
  cp "${sandbox}/scripts/ci/active_track.txt" "${sandbox}/virgin.active.before"
  cp "${sandbox}/docs/orchestrator_orientation.md" "${sandbox}/virgin.orientation.before"
  set +e
  ORIENTATION_FORCE_FAULT_AFTER_WRITES=1 run_gen_sandbox "$sandbox" --unpark docs/design_1_2_3_studio.md >"${sandbox}/virgin_fault.out" 2>"${sandbox}/virgin_fault.err"
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] \
    || ! cmp -s "${sandbox}/virgin.doc.before" "${sandbox}/docs/design_1_2_3_studio.md" \
    || ! cmp -s "${sandbox}/virgin.active.before" "${sandbox}/scripts/ci/active_track.txt" \
    || ! cmp -s "${sandbox}/virgin.orientation.before" "${sandbox}/docs/orchestrator_orientation.md" \
    || ! grep -q "open-transition-aborted" "${sandbox}/virgin_fault.err"; then
    echo "FAIL orientation_virgin_unpark_post_write_rollback"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_virgin_unpark_post_write_rollback"
  if ! run_gen_sandbox "$sandbox" --unpark docs/design_1_2_3_studio.md >"${sandbox}/virgin.out" 2>&1 \
    || ! grep -q "ORIENTATION-UNPARK-VERDICT: OPENED" "${sandbox}/virgin.out" \
    || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/design_1_2_3_studio.md" ]] \
    || ! grep -q "Status: PARKED / fixture" "${sandbox}/docs/design_1_2_3_studio.md"; then
    echo "FAIL orientation_virgin_unpark_unchanged"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_virgin_unpark_unchanged"
  rm -rf "$sandbox"

  sandbox="$(mktemp -d "${TMPDIR:-/tmp}/orient-virgin-open-gate-XXXXXX")"
  seed_park_track_sandbox "$sandbox"
  cat >"${sandbox}/docs/successor.md" <<'EOF'
# Successor Fixture

> **Status: OPEN / fixture.**

This is a production track / PR ladder workplan.

| # | Rung | Deliverable | Exit proof |
|---|---|---|---|
| 1 | `SUCCESSOR-0` | Successor. | TODO. |
EOF
  set +e
  run_gen_sandbox "$sandbox" --open docs/successor.md >"${sandbox}/open_refuse.out" 2>"${sandbox}/open_refuse.err"
  local open_rc=$?
  run_gen_sandbox "$sandbox" --unpark docs/successor.md >"${sandbox}/unpark_refuse.out" 2>"${sandbox}/unpark_refuse.err"
  local unpark_rc=$?
  set -e
  if [[ "$open_rc" -eq 0 ]] || [[ "$unpark_rc" -eq 0 ]] \
    || ! grep -q "FAIL(outgoing-track-open)" "${sandbox}/open_refuse.err" \
    || ! grep -q "FAIL(outgoing-track-open)" "${sandbox}/unpark_refuse.err" \
    || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/design_1_2_3_studio.md" ]]; then
    echo "FAIL orientation_virgin_unpark_open_refusal_parity"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_virgin_unpark_open_refusal_parity"
  if ! run_gen_sandbox "$sandbox" --unpark docs/successor.md --force-owner "Owner override: virgin unpark authorized" >"${sandbox}/unpark_force.out" 2>"${sandbox}/unpark_force.err" \
    || ! grep -q "ORIENTATION-UNPARK-VERDICT: OPENED" "${sandbox}/unpark_force.out" \
    || ! grep -q "ORIENTATION-UNPARK-FORCE-OWNER: recorded scope=1.2.3" "${sandbox}/unpark_force.err" \
    || [[ "$(active_payload_line "${sandbox}/scripts/ci/active_track.txt")" != "docs/successor.md" ]] \
    || ! grep -q '^Owner override: virgin unpark authorized	1.2.3	active	Owner-' "${sandbox}/scripts/ci/owner_directives.tsv"; then
    echo "FAIL orientation_virgin_unpark_force_owner_parity"
    rm -rf "$sandbox"; return 1
  fi
  echo "PASS orientation_virgin_unpark_force_owner_parity"
  rm -rf "$sandbox"; return 0
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
    run_selftest_authoritative_park_pointer
    run_selftest_cold_start_spine
    run_selftest_gate_open_refused
    run_selftest_gate_parked_closed_allowed
    run_selftest_gate_force_owner_records
    run_selftest_gate_incoherent_root_refused
    run_selftest_gate_force_owner_invalid_target_no_write
    run_selftest_gate_force_owner_requires_open
    run_selftest_gate_force_owner_unwritable_directive_no_write
    run_selftest_gate_force_owner_invalid_status_row
    run_selftest_gate_force_owner_duplicate_active_pair
    run_selftest_gate_force_owner_rollback_after_writes
    run_selftest_park_unpark_lifecycle
    run_selftest_park_rollback_and_virgin_unpark
    run_selftest_pointer_divergence_lint
    run_selftest_scope_cell_not_false_complete
    run_selftest_ladder_column_integrity
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
  if [[ -n "$FORCE_OWNER" && "$MODE" != "open" && "$MODE" != "unpark" ]]; then
    echo "gen_orientation: --force-owner requires --open or virgin --unpark" >&2
    usage
  fi
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
  export ORIENTATION_FORCE_OWNER="$FORCE_OWNER"
  export ORIENTATION_OWNER_DIRECTIVES="${ORIENTATION_OWNER_DIRECTIVES:-${SCRIPT_DIR}/owner_directives.tsv}"
  export ORIENTATION_CLOSEOUT_ARTIFACTS="${ORIENTATION_CLOSEOUT_ARTIFACTS:-${SCRIPT_DIR}/closeout_artifacts.tsv}"
  export ORIENTATION_HANDOFFS_DIR="${ORIENTATION_HANDOFFS_DIR:-${REPO_ROOT}/handoffs}"

  exec "$PYTHON_BIN" - <<'PY'
import csv
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
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
FORCE_OWNER = os.environ.get("ORIENTATION_FORCE_OWNER", "").strip()
OWNER_DIRECTIVES = pathlib.Path(
    os.environ.get("ORIENTATION_OWNER_DIRECTIVES", str(REPO_ROOT / "scripts/ci/owner_directives.tsv"))
)
CLOSEOUT_ARTIFACTS = pathlib.Path(
    os.environ.get("ORIENTATION_CLOSEOUT_ARTIFACTS", str(REPO_ROOT / "scripts/ci/closeout_artifacts.tsv"))
)
HANDOFFS_DIR = pathlib.Path(os.environ.get("ORIENTATION_HANDOFFS_DIR", str(REPO_ROOT / "handoffs")))

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
    r"remedial-superseded\b|"
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
PARK_BEGIN = "<!-- SIMTHING-PARKED-TRACK:BEGIN agents: read only when executing --unpark -->"
PARK_END = "<!-- SIMTHING-PARKED-TRACK:END -->"
PARK_FENCE = "```json"


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


def ladder_row_parts(line: str) -> list:
    """Split a markdown table row on bare pipes (no escape awareness)."""
    return [p.strip() for p in (line or "").strip().strip("|").split("|")]


def ladder_header_column_count(line: str):
    """Declared column count for a ladder header line, or None if not a header."""
    if not is_ladder_header(line):
        return None
    return len(ladder_row_parts(line))


def ladder_row_label(parts: list) -> str:
    """Human-readable row name for column-count FAIL messages."""
    if not parts:
        return "<empty-row>"
    if len(parts) == 1:
        return parts[0] or "<empty-row>"
    # Prefer `#`/`Rung` + id cell so HC-5 / `PIPE-ROW` both name cleanly.
    left = (parts[0] or "").strip()
    right = (parts[1] or "").strip()
    if left and right:
        return f"{left} {right}"
    return left or right or "<empty-row>"


def parse_rungs(design_text: str):
    """Parse all PR-ladder / rung tables from a design workplan.

    Supports:
      | # | Rung | Deliverable | Exit proof |
      | Rung | ID | Scope | Exit proof | Tier |

    Collects rows from every matching table in the document (multi-phase ladders).
    Returns list of (num_or_index, rung_id, deliverable_or_scope, exit_proof).

    Column-count invariant (HC-LADDER-COLUMN-INTEGRITY-0): every data row must have
    exactly the column count its own table header declares. A bare or escaped pipe
    inside a cell shifts columns so parts[3] is no longer Exit-proof; FAIL loud at
    this choke point (active gen/--check, --park, --unpark) rather than silent misread.
    """
    rows = []
    in_table = False
    expected_cols = None
    for line in design_text.splitlines():
        stripped = line.strip()
        if is_ladder_header(stripped):
            in_table = True
            expected_cols = ladder_header_column_count(stripped)
            continue
        if not in_table:
            continue
        if not stripped.startswith("|"):
            in_table = False
            expected_cols = None
            continue
        if stripped.startswith("|---") or re.match(r"^\|[\s\-:|]+\|$", stripped):
            continue
        parts = ladder_row_parts(stripped)
        # Skip accidental re-header rows mid-scan (before column-count assert so a
        # repeated header with matching shape does not trip the data-row rule).
        if len(parts) >= 2:
            c0 = parts[0].lower()
            c1 = parts[1].lower()
            if c0 in ("#", "rung") and c1 in ("rung", "id", "deliverable"):
                if expected_cols is not None:
                    expected_cols = len(parts)
                continue
        if expected_cols is not None and len(parts) != expected_cols:
            label = ladder_row_label(parts)
            fail(
                f"FAIL(ladder-column-count): ladder data row {label!r} has "
                f"{len(parts)} columns but the table header declares {expected_cols}; "
                f"remedy: say it without a pipe — backticks do not help; a bare pipe splits too"
            )
        if len(parts) < 4:
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


def read_tsv_dict(path: pathlib.Path, header: list):
    if not path.is_file():
        return header, []
    with path.open(encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh, delimiter="\t")
        fields = reader.fieldnames or header
        return fields, list(reader)


def write_tsv_dict(path: pathlib.Path, header: list, rows: list) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=header, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({k: row.get(k, "") for k in header})


def md_row(values):
    return "| " + " | ".join(v.replace("|", "\\|") for v in values) + " |"


def table(headers, rows):
    lines = [md_row(headers), "| " + " | ".join("---" for _ in headers) + " |"]
    lines.extend(md_row(r) for r in rows)
    return lines


def is_completed_exit(text: str) -> bool:
    """True when an Exit-proof cell marks the rung complete (status language).

    Completion stamps live in the Exit-proof cell only (owner_authoring_guide).
    Scope/deliverable narrative that merely describes completion words must not
    false-complete a rung — see next_rung_pointer.
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


def rung_id_from_cell(rung_cell: str) -> str:
    parts = (rung_cell or "").split("`")
    return parts[1] if len(parts) >= 3 else (rung_cell or "").strip("`").strip()


def next_rung_pointer(rungs):
    for num, rung, deliv, exit_proof in rungs:
        # Completion stamp lives ONLY in the Exit-proof cell. A Scope/deliverable
        # cell that describes completion words (e.g. "names a rung whose exit-proof
        # is DA-GRADUATED / merged [#N]") must not false-complete the rung.
        if is_completed_exit(exit_proof):
            continue
        rid = rung_id_from_cell(rung)
        if rid:
            return rid
    return "none"


def authoritative_active_pointer(design_text: str):
    """Read the explicit park/open posture when the design publishes one."""
    match = re.search(
        r"^\|\s*Active open rung\s*\|\s*(.*?)\s*\|\s*$",
        design_text,
        re.IGNORECASE | re.MULTILINE,
    )
    if not match:
        return None
    state = match.group(1).strip()
    if re.match(r"^none\b", state, re.IGNORECASE):
        if re.search(
            r"awaiting Owner direction for the next UI/UX ladder",
            state,
            re.IGNORECASE,
        ):
            return "none — awaiting Owner direction for the next UI/UX ladder"
        return NO_ACTIVE_TRACK
    rung = re.search(r"`([^`]+)`", state)
    return rung.group(1) if rung else None


def active_pointer_line(pointer: str) -> str:
    if pointer.startswith(f"{NO_ACTIVE_TRACK} "):
        return f"Active pointer: {pointer}"
    return f"Active pointer: `{pointer}`"


def is_no_active_pointer(pointer: str) -> bool:
    return pointer == NO_ACTIVE_TRACK or pointer.startswith(f"{NO_ACTIVE_TRACK} ")


def ladder_exit_proof_by_rung(rungs) -> dict:
    """Map rung_id -> exit_proof cell text (first occurrence wins)."""
    out = {}
    for _num, rung, _deliv, exit_proof in rungs:
        rid = rung_id_from_cell(rung)
        if rid and rid not in out:
            out[rid] = exit_proof
    return out


def divergent_pointer_reason(design_text: str, rungs: list):
    """Return a FAIL reason when the authoritative Active open rung row diverges.

    Coherent cases (no reason):
      - no Active open rung row (ladder scan is the sole source)
      - none-form / awaiting-Owner-direction form
      - named rung present on the ladder with a non-completed Exit-proof cell

    Divergent cases:
      - named rung absent from the ladder
      - named rung whose Exit-proof already carries a graduation/finished stamp
    """
    pointer = authoritative_active_pointer(design_text)
    if pointer is None or is_no_active_pointer(pointer):
        return None
    by_rung = ladder_exit_proof_by_rung(rungs)
    if pointer not in by_rung:
        return (
            f"FAIL(pointer-divergence): authoritative Active open rung names "
            f"{pointer!r} which is absent from the ladder; "
            f"remedy: set Active open rung to a live ladder rung id (or `none`), "
            f"then `bash scripts/ci/gen_orientation.sh`"
        )
    if is_completed_exit(by_rung[pointer]):
        return (
            f"FAIL(pointer-divergence): authoritative Active open rung names "
            f"{pointer!r} whose Exit-proof cell is already graduation/finished-stamped; "
            f"remedy: advance Active open rung to the next open ladder rung (or `none`) "
            f"when stamping graduation, then `bash scripts/ci/gen_orientation.sh`"
        )
    return None


def assert_authoritative_pointer_coherent(design_text: str, rungs: list) -> None:
    reason = divergent_pointer_reason(design_text, rungs)
    if reason:
        fail(reason)


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


def park_payload_receipt(payload: dict) -> str:
    body = dict(payload)
    body.pop("park_receipt", None)
    raw = json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(raw.encode("utf-8")).hexdigest()[:12]


def park_block_for_payload(payload: dict) -> str:
    payload = dict(payload)
    payload["park_receipt"] = park_payload_receipt(payload)
    return (
        f"{PARK_BEGIN}\n"
        f"{PARK_FENCE}\n"
        f"{json.dumps(payload, indent=2, sort_keys=True, ensure_ascii=False)}\n"
        "```\n"
        f"{PARK_END}\n"
    )


def split_park_block(text: str):
    begin_count = text.count(PARK_BEGIN)
    end_count = text.count(PARK_END)
    if begin_count == 0 and end_count == 0:
        return text, None
    if begin_count != 1 or end_count != 1:
        fail("parked track block must appear exactly once")
    begin = text.index(PARK_BEGIN)
    end = text.index(PARK_END) + len(PARK_END)
    suffix = text[end:]
    if suffix not in ("", "\n"):
        fail("parked track block must be the absolute EOF block")
    block = text[begin:end]
    fence_start = block.find(PARK_FENCE)
    fence_end = block.rfind("```")
    if fence_start < 0 or fence_end <= fence_start:
        fail("parked track block missing json fence")
    raw_json = block[fence_start + len(PARK_FENCE):fence_end].strip()
    try:
        payload = json.loads(raw_json)
    except json.JSONDecodeError as exc:
        fail(f"parked track block has invalid json: {exc}")
    receipt = payload.get("park_receipt", "")
    if not re.fullmatch(r"[0-9a-f]{12}", receipt):
        fail("parked track block has invalid PARK-RECEIPT")
    if receipt != park_payload_receipt(payload):
        fail("parked track block receipt drift; refuse tampered block")
    return text[:begin].rstrip() + "\n", payload


def validate_park_block_shape(text: str) -> None:
    split_park_block(text)


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
        validate_park_block_shape(design_text)
        rungs = parse_rungs(design_text)
        assert_authoritative_pointer_coherent(design_text, rungs)
        next_rung = authoritative_active_pointer(design_text) or next_rung_pointer(rungs)
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
    "| `DA-RESERVE(harness-error)` / `FAIL(<remedy>)` | remedy the harness/PR; **not** a DA review. |",
    "| `DA-RESERVE(unclassified-scope)` | **classify before DA relay** (below) — not automatic design escalation. |",
    "| `DA-RESERVE(admitted-scope-router-gap)` | router debt inside admitted envelope — **class-hardening**, not fresh DA design. |",
    "| `DA-RESERVE(class-envelope-violation)` | if design already marks the rung clearable → **class-harden** `class_predicates.tsv` (under-width); escalate only when forbidden surfaces (spec/clausething/driver/gpu/wgsl/gate) are real. |",
    "| other `DA-RESERVE(<reason>)` | quote verbatim; escalate only for true DA residue (novelty, gate-wiring, seal, binding, …). |",
    "",
    "**Empty-class split** (CLEARANCE-ADMITTED-SCOPE-GAP-0 / #1242 Option A):",
    "1) `novelty_claim: YES` + `novelty_basis` → `DA-RESERVE(novelty)` (checked before class match).",
    "2) `admitted_envelope: YES` + admitting_pr/rung + surfaces + proofs → `DA-RESERVE(admitted-scope-router-gap)`.",
    "3) else → `DA-RESERVE(unclassified-scope)` (true unadmitted residue only).",
    "Missing admitted-scope claim/proof fields → `FAIL(missing-admitted-scope-router-gap-fields...)`.",
    "`DA-RESERVE(admitted-scope-router-gap)` means admitted envelope + proof-present + missing class. It is router debt, not a fresh DA design question. Repeated admitted-scope router gaps should be closed with class-hardening.",
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
        # End-state / fully-complete ladders: keep the digest thin — last few rows only.
        # Open ladders: show from one completed predecessor through remaining incomplete rungs.
        indexed = list(enumerate(rungs))
        if is_no_active_pointer(next_rung) or track_state in {"closed", "parked", "end-state"}:
            selected = indexed[-5:] if len(indexed) > 5 else indexed
            if len(indexed) > 5:
                lines.append(
                    f"> Compact view: showing last {len(selected)} of {len(indexed)} rungs "
                    f"(track `{track_state}`); full ladder in the design doc."
                )
                lines.append("")
        else:
            first_open = next(
                (
                    i
                    for i, (_n, _r, _deliv, exit_proof) in indexed
                    if not is_completed_exit(exit_proof)
                ),
                0,
            )
            start = max(0, first_open - 1)
            selected = indexed[start:]
        for _i, (num, rung, deliverable, exit_proof) in selected:
            short = exit_proof
            if len(short) > 120:
                short = short[:117] + "..."
            rung_table.append((num, rung.strip("`"), deliverable[:80], short))
        lines.extend(table(["#", "rung", "deliverable", "exit proof"], rung_table))
        lines.extend([
            "",
            "## Next Rung Pointer",
            "",
            active_pointer_line(next_rung),
            "",
        ])
    lines.extend([
    "",
    "## Cold-Start Spine (constitutional pointers)",
    "",
    "Pointers only — resolve verbatim doctrine via `anchor_query.sh`; do not raw-grep doctrine docs.",
    "",
    "- FIELD_POLICY / CPU-only job → `field-policy-time-decisions` (`bash scripts/ci/anchor_query.sh --domain field-policy`)",
    "- Spec fidelity §0.6 → `spec-fidelity-anti-ceremony` (`bash scripts/ci/anchor_query.sh --grep spec-fidelity`)",
    "- Founding ontology §0.2 (allocation is recursive) → `founding-ontology-invariants`",
    "- Founding ontology §0.3 (all conflict is resource flow) → `founding-ontology-invariants`",
    "- Invariants registry → `docs/invariants.md` (via `founding-ontology-invariants`)",
    "- Drift detectors §9 → `drift-detectors-six-line` (`bash scripts/ci/anchor_query.sh --domain drift-detectors`)",
    "- Doctrine lookup entrypoint: `bash scripts/ci/anchor_query.sh --domain <d> --paths <files...> --grep <term>`",
    "- Cold-start receipt: `bash scripts/ci/orient.sh --role=coding|orchestrator|da` → carry `ORIENT-RECEIPT`",
    "",
    "## Clearance Router Verdict Meanings",
    "",
    "| verdict | meaning |",
    "| --- | --- |",
    "| `CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE` | precedented class matched; binding conditions discharged; required proof fields present |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(unclassified-scope)` | no class match and no valid admitted-envelope claim (narrowed; not novelty) |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(admitted-scope-router-gap)` | admitted envelope + proof-present + missing class — router debt; class-harden, not fresh DA design |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(novelty)` | explicit `novelty_claim: YES` + `novelty_basis`; **overrides** matched-class clearance; not a generic unmatched-diff fallback |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(class-envelope-violation)` | matched class but diff leaves path envelope — under-width when design says clearable → class-harden; forbidden-surface hit → DA |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(engine-scope-violation)` | matched class forbids engine crate/src and the diff touches engine scope |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(module-marker-shape-mismatch)` | corpus-module-marker-sweep shape fails inventory deletion rules |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(harness-error)` | malformed data, ambiguous class, empty/unresolved requested target, or script error |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` | PR touches router/lint/harness gate surfaces (self-application refusal) |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(binding-conditions)` | open binding condition blocks clearance for matched class |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(class-suspended)` | precedented class row status=suspended |",
    "| `CLEARANCE-VERDICT: DA-RESERVE(triage-missing)` | INSPECT delta without landed /triage row (check 7 live) |",
    "| `CLEARANCE-VERDICT: FAIL(missing-novelty-basis...)` | `novelty_claim: YES` without valid `novelty_basis` |",
    "| `CLEARANCE-VERDICT: FAIL(missing-admitted-scope-router-gap-fields...)` | `admitted_envelope: YES` missing admitting_pr/rung, surfaces, or proof fields |",
    "| `CLEARANCE-VERDICT: FAIL(remedy)` | named fix required before re-attempt (CI not green, missing proof fields, etc.) |",
    "",
    "`DA-RESERVE(novelty)` is explicit-claim-only and overrides matched-class clearance. A novelty claim must",
    "include `novelty_basis` naming the unanticipated implementation discovery or substrate improvement.",
    "Without `novelty_basis`, clearance fails. Empty-class diffs with a valid admitted-envelope claim emit",
    "`DA-RESERVE(admitted-scope-router-gap)` (router debt). True unknown empty-class diffs emit",
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
    "- `unclassified-scope` → **only** no class match and no valid admitted-envelope claim (not automatic DA ceremony).",
    "- `admitted-scope-router-gap` → admitted envelope + proof-present + missing class; class-hardening follow-up; not a fresh DA design question.",
    "- `class-envelope-violation` → if design already marks clearable, treat as class-hardening (widen scope/`match_any`); escalate to DA only for real forbidden surfaces (spec/clausething/driver/kernel/sim/gpu/wgsl/gate) or workshop_only breach.",
    "- engine-scope-violation, module-marker-shape-mismatch → DA review (precise reason; not novelty rhetoric).",
    "- Novelty (`novelty_claim: YES` + `novelty_basis`) overrides matched-class clearance → DA review routing.",
    "- `novelty_claim: YES` without `novelty_basis` → FAIL(missing-novelty-basis); not clearable.",
    "- `tp-workshop-candidate-proof` → workshop-homed 0.0.8.5 TP candidate proofs only (not mapeditor API / sealed crates / GPU / picker / closeout).",
    "- `tp-admitted-clause-api-composition` → DA-admitted limited ClauseScript composition (StructuralRebindReady) only; not picker/runtime/GameMode/RF/closeout.",
    "- `tp-studio-clause-picker` → DA-admitted narrow Studio/mapeditor `.clause` picker UI (#1239 shape); production API only; not TP defaults / dual parse-rebind / GameMode-RF-live-run-closeout / runtime-GPU-kernel.",
    "- binding-conditions, class-suspended, triage-missing → DA review routing.",
    "- gate-wiring → deep audit; harness surfaces are never self-mergeable.",
    "- harness-error → fix data/target resolution before re-run.",
    "- FAIL(remedy) → apply named remedy and re-run clearance.",
    "- DA exit-proof stamp (binding): after DA pass (graduate-merge / formal admission / denial), DA updates active workplan Exit proof + results COMPLETE + orientation regen and merges the stamp PR; not orchestrator residual (see agent_onboarding).",
    "- DA verify-the-tree (weighted): require for code-facing / long-lifecycle / horizontally impactful load-bearing; relaxed for pure policy, stamps, light residual (see agent_onboarding).",
    "- DA treeverify advisor (advisory): `bash scripts/ci/da_treeverify.sh --pr <n>|--range <base>..<head>` emits `DA-TREEVERIFY-PROFILE: RELAX|LIGHT-TREE|DEEP-TREE` + focus paths — not CLEARANCE-VERDICT; table `scripts/ci/da_review_profile.tsv` (core rows permanent; non-core require expires_on and default-delete after expiry).",
    "- Expeditionary escape (anti-abuse): body may set `expeditionary: YES` + `expedition_charter` + `expeditionary_until: YYYY-MM-DD`; never silent RELAX; cannot downgrade production/engine/long-lifecycle to RELAX.",
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
    "- `da` — rung table, binding conditions, escalation posture; run da_treeverify on load-bearing escalations; after pass, exit-proof stamp + merge (agent_onboarding)",
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
    "Unconditional steps (HU-DELTA-SCAN-0) — ≤4:",
    "",
    "```bash",
    "bash scripts/ci/orient.sh --role=coding   # once per fresh session",
    "cargo check -p <touched-crate>",
    "bash scripts/ci/agent_scan.sh             # delta HEURISTIC + RELIABLE hard FAIL",
    "cargo test -p <crate> --test <focused>   # when tests changed / named by handoff",
    "```",
    "",
    "Maintainer/CI only (not coding default): whole-tree `doctrine_scan.sh`, clearance/relay/da_treeverify selftests,",
    "`doctrine_selftest.sh` (scanner surface only).",
    "",
    "## HD Owner Interface",
    "",
    "- Scribe Owner prose into `.hd.md`, echo the exact diff before push; \"current handoff approved, implement\" means resolve active `.hd.md`, render your projection, verify `owner_approved: true`; `/handoff approve|amend: <text>|hold` is OWNER-only, collaborator attempts route to owner-review, `/handoff status` is read-only, and `owner_notes` must render verbatim.",
    "",
    "",
    "## GHA Comment Commands",
    "",
    "- `/clearance` — M1 router verdict",
    "- `/relay-lint` — M3 relay lint verdict",
    "- `/orient` — M2 orientation digest (this page)",
    "- `/orient role=orchestrator|coding|da` — role-filtered subset",
    "- `/anchor <anchor_id|trigger_domain>` — verbatim anchored doctrine text",
    "- DA treeverify is **local/on-demand** (`da_treeverify.sh`); not a required PR gate or merge authority",
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


def status_header_closed_or_parked(design_text: str) -> bool:
    """The outgoing track's status header must declare CLOSED or PARKED for a pointer flip to be
    allowed. Inspect only the STATE token immediately after `Status:` (up to the first delimiter),
    never the trailing prose — a status line may mention another track being 'parked' in prose
    (e.g. HD board: `Status: OPEN / ... Owner parked 0.0.8.6 ...`) without itself being parked.
    Absence of a status header is treated as still-open (refuse)."""
    for line in design_text.splitlines()[:60]:
        stripped = line.strip().lstrip(">").strip()
        m = re.match(r"\**status\s*:\s*([A-Za-z0-9 +\-]+)", stripped, re.IGNORECASE)
        if m:
            return bool(re.search(r"\b(CLOSED|PARKED)\b", m.group(1), re.IGNORECASE))
    return False


def outgoing_track_id(rel: str) -> str:
    """Derive the dotted track id from a track doc path (design_0_0_8_4_8_4_... -> 0.0.8.4.8.4);
    fall back to the file stem when the numeric prefix is absent."""
    stem = pathlib.PurePosixPath(rel).stem
    m = re.match(r"design_(\d+(?:_\d+)*)", stem)
    if m:
        return m.group(1).replace("_", ".")
    return stem


_OWNER_DIRECTIVE_HEADER = ["directive", "scope", "status", "set_by"]
_VALID_DIRECTIVE_STATUS = {"active", "retired"}


def plan_owner_directives_bytes(directive_text: str, outgoing_rel: str) -> str:
    """Plan + fully validate the COMPLETE post-operation `owner_directives.tsv` content during the
    forced-open preflight. Enforces the authority-table semantics on both the existing rows AND the
    normalized planned row — 4 tab fields; status exactly active/retired; nonempty directive/scope/
    set_by; no duplicate active (directive, scope) pair. Returns the exact bytes the commit must
    write verbatim (no reconstruct-at-commit); raises ValueError with a reason on any violation.
    Performs no writes."""
    import datetime

    directive_clean = directive_text.replace("\t", " ").replace("\n", " ").strip()
    if not directive_clean:
        raise ValueError("--force-owner directive text is empty")
    scope = outgoing_track_id(outgoing_rel).strip()
    if not scope:
        raise ValueError("could not derive a nonempty outgoing track scope")
    set_by = f"Owner-{datetime.date.today().isoformat()}"

    existing_rows = []
    active_pairs = set()
    if OWNER_DIRECTIVES.is_file():
        dtext = OWNER_DIRECTIVES.read_text(encoding="utf-8")
        lines = [ln for ln in dtext.splitlines() if ln.strip() != ""]
        if lines:
            if lines[0].split("\t") != _OWNER_DIRECTIVE_HEADER:
                raise ValueError("owner_directives header schema mismatch (want directive/scope/status/set_by)")
            for row in lines[1:]:
                fields = row.split("\t")
                if len(fields) != 4:
                    raise ValueError("owner_directives row is not 4 tab-separated fields")
                d, s, st, sb = fields
                if st not in _VALID_DIRECTIVE_STATUS:
                    raise ValueError(f"owner_directives row has invalid status {st!r} (want active/retired)")
                if not d.strip() or not s.strip() or not sb.strip():
                    raise ValueError("owner_directives row has an empty directive/scope/set_by field")
                if st == "active":
                    pair = (d, s)
                    if pair in active_pairs:
                        raise ValueError(f"owner_directives has a duplicate active (directive, scope) pair {pair!r}")
                    active_pairs.add(pair)
                existing_rows.append(row)

    if (directive_clean, scope) in active_pairs:
        raise ValueError(
            f"planned directive duplicates an active (directive, scope) pair {(directive_clean, scope)!r}"
        )
    planned_row = "\t".join([directive_clean, scope, "active", set_by])
    return "\n".join(["\t".join(_OWNER_DIRECTIVE_HEADER)] + existing_rows + [planned_row]) + "\n"


def assert_coherent_open_root() -> None:
    """`--open` mutates ONE coherent root. The active-pointer file, generated orientation, and
    owner-directives table must all resolve under `ORIENTATION_REPO_ROOT` — the same root whose
    documents supply the outgoing/target tracks the gate validates. This closes the cross-root
    env-seam bypass: a fake root's PARKED outgoing doc must never authorize a write to a *victim*
    checkout's pointer/orientation. No new bypass flag; sandbox tests stay valid because every
    sandbox mutation path is under its sandbox root."""
    root = REPO_ROOT.resolve()
    for label, p in (
        ("ORIENTATION_ACTIVE_TRACK_FILE", ACTIVE_TRACK),
        ("ORIENTATION_OUTPUT", OUTPUT),
        ("ORIENTATION_OWNER_DIRECTIVES", OWNER_DIRECTIVES),
        ("ORIENTATION_CLOSEOUT_ARTIFACTS", CLOSEOUT_ARTIFACTS),
        ("ORIENTATION_HANDOFFS_DIR", HANDOFFS_DIR),
    ):
        try:
            p.resolve().relative_to(root)
        except ValueError:
            print("ORIENTATION-OPEN-VERDICT: FAIL(incoherent-root)", file=sys.stderr)
            print(
                f"gen_orientation: FAIL(incoherent-root): {label}={p} is not under "
                f"ORIENTATION_REPO_ROOT={root}; --open requires one coherent mutation root.",
                file=sys.stderr,
            )
            sys.exit(1)


class _FileTxn:
    """Snapshot/restore for the forced-open commit: stage every mutation target before writing, then
    restore original bytes/existence on any failure so a failed forced attempt leaves the tree byte-
    identical."""

    def __init__(self):
        self._snap = {}

    def stage(self, path) -> None:
        path = pathlib.Path(path)
        key = str(path)
        if key in self._snap:
            return
        existed = path.exists()
        data = path.read_bytes() if path.is_file() else None
        self._snap[key] = (path, existed, data)

    def rollback(self) -> None:
        for _, (path, existed, data) in self._snap.items():
            try:
                if existed and data is not None:
                    path.write_bytes(data)
                elif not existed and path.exists():
                    path.unlink()
            except OSError:
                pass


def _fail_forced_preflight(detail: str):
    print("ORIENTATION-OPEN-VERDICT: FAIL(forced-preflight)", file=sys.stderr)
    print(f"gen_orientation: FAIL(forced-preflight): {detail}", file=sys.stderr)
    sys.exit(1)


def preflight_forced_open(target: pathlib.Path, created_planned: bool, outgoing_rel: str) -> str:
    """Zero-write admission of a forced open: validates the mutation targets' type/writability, the
    target's creatability, and (via `plan_owner_directives_bytes`) the FULL directive-table semantics
    plus the normalized planned row. Returns the exact directive-table bytes the commit must write.
    No byte is written; any violation aborts before mutation."""
    for label, p in (
        ("active_track", ACTIVE_TRACK),
        ("orientation", OUTPUT),
        ("owner_directives", OWNER_DIRECTIVES),
    ):
        if p.is_dir():
            _fail_forced_preflight(f"{label} target {p} is a directory, not a writable file")
        if p.exists():
            if not os.access(p, os.W_OK):
                _fail_forced_preflight(f"{label} target {p} is not writable")
        else:
            if not p.parent.exists() or not os.access(p.parent, os.W_OK):
                _fail_forced_preflight(f"{label} parent {p.parent} is not writable")
    if created_planned:
        anc = target.parent
        while not anc.exists():
            anc = anc.parent
        if not os.access(anc, os.W_OK):
            _fail_forced_preflight(f"target parent {target.parent} is not creatable")
    try:
        return plan_owner_directives_bytes(FORCE_OWNER, outgoing_rel)
    except (ValueError, OSError, UnicodeDecodeError) as exc:
        _fail_forced_preflight(str(exc))
        raise  # unreachable (sys.exit above); satisfies static "always returns"


def _forced_fault_after_writes() -> None:
    """Deterministic selftest-only fault injected AFTER the pointer + orientation writes to prove the
    rollback path restores all four surfaces. Fail-only by construction: it can only RAISE (forcing an
    abort + rollback), never grant or bypass admission — so it is useless as a production gate bypass
    and weakens neither normal nor forced admission."""
    if os.environ.get("ORIENTATION_FORCE_FAULT_AFTER_WRITES") == "1":
        raise RuntimeError("selftest fault injection: forced post-write failure (rollback proof)")


_BINDING_HEADER = ["rung", "condition", "set_by", "status", "promotion_blocker"]
_OWNER_HEADER = ["directive", "scope", "status", "set_by"]
_ARTIFACT_HEADER = ["path", "leased_at", "disposition", "closeout_track", "note"]


def _repo_head_short() -> str:
    try:
        proc = subprocess.run(
            ["git", "-C", str(REPO_ROOT), "rev-parse", "--short=12", "HEAD"],
            capture_output=True, text=True, check=True,
        )
        return proc.stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def _today() -> str:
    override = os.environ.get("ORIENTATION_PARKED_AT", "").strip()
    if override:
        return override
    return datetime.datetime.now(datetime.timezone.utc).date().isoformat()


def _track_handoff_match(path: pathlib.Path, text: str, track_id: str, rung_ids: set) -> bool:
    if re.search(rf"(?m)^track:\s*{re.escape(track_id)}(?:\b|$)", text):
        return True
    stem = path.stem
    return stem in rung_ids


def _open_pr_hits_for_track(track_id: str, rung_ids: set) -> list:
    fixture = os.environ.get("ORIENTATION_OPEN_PR_FIXTURE", "")
    if fixture:
        try:
            prs = json.loads(fixture)
            hits = []
            for pr in prs:
                blob = "\n".join(str(pr.get(k, "")) for k in ("number", "title", "headRefName", "body"))
                if track_id in blob or any(rung and rung in blob for rung in rung_ids):
                    hits.append(f"#{pr.get('number')} {pr.get('title', '')}".strip())
            return hits
        except json.JSONDecodeError:
            lines = [ln for ln in fixture.splitlines() if ln.strip()]
            return [
                ln for ln in lines
                if track_id in ln or any(rung and rung in ln for rung in rung_ids)
            ]
    if os.environ.get("ORIENTATION_SKIP_OPEN_PR_CHECK") == "1":
        return []
    try:
        proc = subprocess.run(
            [
                "gh", "pr", "list", "--repo", "khorum08/SimThing", "--state", "open",
                "--limit", "100", "--json", "number,title,headRefName,body",
            ],
            capture_output=True, text=True, check=True,
        )
        prs = json.loads(proc.stdout or "[]")
    except (OSError, subprocess.CalledProcessError, json.JSONDecodeError) as exc:
        fail(f"ORIENTATION-PARK-VERDICT: FAIL(open-pr-check-unavailable): {exc}")
    hits = []
    for pr in prs:
        blob = "\n".join(str(pr.get(k, "")) for k in ("number", "title", "headRefName", "body"))
        if track_id in blob or any(rung and rung in blob for rung in rung_ids):
            hits.append(f"#{pr.get('number')} {pr.get('title', '')}".strip())
    return hits


def _replace_status_header(text: str, state: str) -> str:
    replacement = f"> **Status: {state} / harness lifecycle.**"
    lines = text.splitlines()
    for idx, line in enumerate(lines[:80]):
        stripped = line.strip().lstrip(">").strip()
        if re.match(r"\**status\s*:", stripped, re.IGNORECASE):
            lines[idx] = replacement
            return "\n".join(lines).rstrip() + "\n"
    insert_at = 1 if lines else 0
    lines.insert(insert_at, replacement)
    return "\n".join(lines).rstrip() + "\n"


def _append_unique(existing: list, incoming: list, header: list) -> list:
    seen = {tuple((row.get(k, "") for k in header)) for row in existing}
    out = list(existing)
    for row in incoming:
        key = tuple(row.get(k, "") for k in header)
        if key not in seen:
            seen.add(key)
            out.append(row)
    return out


def _row_key(row: dict, header: list) -> tuple:
    return tuple(row.get(k, "") for k in header if k != "__park_index")


def _table_identity_key(name: str, row: dict, header: list) -> tuple:
    if name == "binding_conditions.tsv":
        return (
            name,
            row.get("rung", ""),
            row.get("condition", ""),
            row.get("promotion_blocker", ""),
        )
    if name == "owner_directives.tsv":
        return (name, row.get("directive", ""), row.get("scope", ""))
    if name == "closeout_artifacts.tsv":
        return (name, clean_repo_relpath(row.get("path", "")))
    return (name, _row_key(row, header))


def _indexed_row(row: dict, default: int = 1000000) -> int:
    try:
        return int(row.get("__park_index", default))
    except (TypeError, ValueError):
        return default


def _filter_superseded_old_rows(name: str, old_rows: list, moved_rows: list, header: list, drops: dict) -> list:
    moved_keys = {_table_identity_key(name, row, header) for row in moved_rows}
    kept = []
    for row in old_rows:
        if _table_identity_key(name, row, header) in moved_keys:
            drops["superseded_table_rows"] = drops.get("superseded_table_rows", 0) + 1
            continue
        kept.append(row)
    return kept


def _merge_handoffs_by_path(old_handoffs: list, new_handoffs: list, drops: dict) -> list:
    merged = []
    positions = {}
    for handoff in old_handoffs:
        path = clean_repo_relpath(handoff.get("path", ""))
        if not path:
            drops["stale_handoffs"] = drops.get("stale_handoffs", 0) + 1
            continue
        normalized = {"path": path, "content": handoff.get("content", "")}
        positions[path] = len(merged)
        merged.append(normalized)
    for handoff in new_handoffs:
        path = clean_repo_relpath(handoff.get("path", ""))
        if not path:
            continue
        normalized = {"path": path, "content": handoff.get("content", "")}
        if path in positions:
            if merged[positions[path]].get("content", "") != normalized["content"]:
                drops["superseded_handoffs"] = drops.get("superseded_handoffs", 0) + 1
            merged[positions[path]] = normalized
        else:
            positions[path] = len(merged)
            merged.append(normalized)
    return merged


def _merge_at_original_positions(live_rows: list, parked_rows: list, header: list) -> list:
    by_index = {}
    for row in parked_rows:
        idx = _indexed_row(row)
        by_index.setdefault(idx, []).append(row)
    total = len(live_rows) + len(parked_rows)
    restored = []
    live_idx = 0
    for idx in range(total):
        bucket = by_index.get(idx)
        if bucket:
            for row in bucket:
                restored.append({k: row.get(k, "") for k in header})
            continue
        if live_idx < len(live_rows):
            restored.append({k: live_rows[live_idx].get(k, "") for k in header})
            live_idx += 1
    while live_idx < len(live_rows):
        restored.append({k: live_rows[live_idx].get(k, "") for k in header})
        live_idx += 1
    return restored


def _handoff_referent_live(handoff: dict, rung_ids: set) -> bool:
    path = clean_repo_relpath(handoff.get("path", ""))
    if not path:
        return False
    content = handoff.get("content", "")
    stem = pathlib.PurePosixPath(path).stem
    if stem in rung_ids:
        return True
    return any(rung and re.search(rf"(?m)^rung:\s*{re.escape(rung)}(?:\b|$)", content) for rung in rung_ids)


def _artifact_referent_live(row: dict, handoff_paths: set) -> bool:
    rel = clean_repo_relpath(row.get("path", ""))
    if not rel:
        return False
    if rel.startswith("handoffs/"):
        return rel in handoff_paths or (REPO_ROOT / rel).is_file()
    return (REPO_ROOT / rel).is_file()


def _filter_table_referents(name: str, rows: list, header: list, track_id: str,
                            rung_ids: set, handoff_paths: set, drops: dict) -> list:
    kept = []
    for row in rows:
        ok = True
        if name == "binding_conditions.tsv":
            ok = row.get("rung", "") in rung_ids or row.get("promotion_blocker", "") in rung_ids
        elif name == "owner_directives.tsv":
            scope = row.get("scope", "")
            ok = scope == track_id or scope.startswith(f"{track_id}-")
        elif name == "closeout_artifacts.tsv":
            ct = row.get("closeout_track", "")
            ok = ct == track_id or ct.startswith(f"{track_id}-")
            if ok:
                ok = _artifact_referent_live(row, handoff_paths)
        if not ok:
            drops["vanished_table_rows"] = drops.get("vanished_table_rows", 0) + 1
            continue
        kept.append(row)
    return kept


def _collect_park_payload(rel: str, base_text: str, payload_old=None) -> tuple:
    track_id = outgoing_track_id(rel)
    rung_ids = {rung for _num, rung, _scope, _exit in parse_rungs(base_text)}
    rung_ids = {r.split("`")[1] if "`" in r and len(r.split("`")) >= 3 else r.strip("`").strip()
                for r in rung_ids}
    hits = _open_pr_hits_for_track(track_id, rung_ids)
    if hits:
        print("ORIENTATION-PARK-VERDICT: FAIL(open-prs-for-track)", file=sys.stderr)
        for hit in hits[:10]:
            print(f"  - {hit}", file=sys.stderr)
        sys.exit(1)

    b_hdr, binding = read_tsv_dict(BINDING_TSV, _BINDING_HEADER)
    o_hdr, owners = read_tsv_dict(OWNER_DIRECTIVES, _OWNER_HEADER)
    a_hdr, artifacts = read_tsv_dict(CLOSEOUT_ARTIFACTS, _ARTIFACT_HEADER)

    def binding_match(row):
        return row.get("rung", "") in rung_ids or row.get("promotion_blocker", "") in rung_ids

    def owner_match(row):
        scope = row.get("scope", "")
        return scope == track_id or scope.startswith(f"{track_id}-")

    def artifact_match(row):
        ct = row.get("closeout_track", "")
        p = row.get("path", "").replace("\\", "/")
        return ct == track_id or ct.startswith(f"{track_id}-") or (
            p.startswith("handoffs/") and any(rung and rung in p for rung in rung_ids)
        )

    def moved_with_index(rows, pred):
        out = []
        for idx, row in enumerate(rows):
            if pred(row):
                entry = dict(row)
                entry["__park_index"] = str(idx)
                out.append(entry)
        return out

    moved_binding = moved_with_index(binding, binding_match)
    moved_owners = moved_with_index(owners, owner_match)
    moved_artifacts = moved_with_index(artifacts, artifact_match)
    remaining_binding = [r for r in binding if not binding_match(r)]
    remaining_owners = [r for r in owners if not owner_match(r)]
    remaining_artifacts = [r for r in artifacts if not artifact_match(r)]

    handoffs = []
    if HANDOFFS_DIR.is_dir():
        for path in sorted(HANDOFFS_DIR.glob("*.hd.md")):
            text = path.read_text(encoding="utf-8")
            if _track_handoff_match(path, text, track_id, rung_ids):
                try:
                    hrel = path.relative_to(REPO_ROOT).as_posix()
                except ValueError:
                    hrel = f"handoffs/{path.name}"
                handoffs.append({"path": hrel, "content": text})

    drop_counts = {}
    old_handoffs = (payload_old or {}).get("handoffs", [])
    kept_old_handoffs = []
    for h in old_handoffs:
        if _handoff_referent_live(h, rung_ids):
            kept_old_handoffs.append(h)
        else:
            drop_counts["stale_handoffs"] = drop_counts.get("stale_handoffs", 0) + 1
    handoffs = _merge_handoffs_by_path(kept_old_handoffs, handoffs, drop_counts)
    handoff_path_set = {h.get("path", "") for h in handoffs}

    old_tables = (payload_old or {}).get("tables", {})
    def carried(name, header, moved):
        old = old_tables.get(name, {})
        old_rows = old.get("rows", []) if isinstance(old, dict) else []
        old_header = old.get("header", header) if isinstance(old, dict) else header
        effective_header = old_header or header
        filtered_old = _filter_table_referents(
            name, old_rows, effective_header, track_id, rung_ids, handoff_path_set, drop_counts
        )
        filtered_old = _filter_superseded_old_rows(name, filtered_old, moved, effective_header, drop_counts)
        return {"header": effective_header, "rows": _append_unique(filtered_old, moved, effective_header)}

    payload = {
        "schema": "simthing.parked-track.v1",
        "track_doc": rel,
        "track_id": track_id,
        "rung_ids": sorted(rung_ids),
        "parked_at": _today(),
        "parked_from_head": _repo_head_short(),
        "pointer": first_payload_line(ACTIVE_TRACK),
        "tables": {
            "binding_conditions.tsv": carried("binding_conditions.tsv", b_hdr, moved_binding),
            "owner_directives.tsv": carried("owner_directives.tsv", o_hdr, moved_owners),
            "closeout_artifacts.tsv": carried("closeout_artifacts.tsv", a_hdr, moved_artifacts),
        },
        "handoffs": handoffs,
        "drop_counts": drop_counts,
    }
    tables_after = {
        BINDING_TSV: (b_hdr, remaining_binding),
        OWNER_DIRECTIVES: (o_hdr, remaining_owners),
        CLOSEOUT_ARTIFACTS: (a_hdr, remaining_artifacts),
    }
    return payload, tables_after, [REPO_ROOT / h["path"] for h in handoffs]


def park_track() -> int:
    if not OPEN_TARGET:
        fail("--park requires exactly one track doc")
    assert_coherent_open_root()
    rel = normalize_track_doc_arg(OPEN_TARGET, must_exist=True)
    target = REPO_ROOT / rel
    original_text = target.read_text(encoding="utf-8")
    base_text, old_payload = split_park_block(original_text)
    ok, reason = classify_workplan(rel, base_text)
    if not ok:
        fail_non_workplan(f"{rel}: {reason}")
    # §3a cascade: refuse a divergent authoritative pointer so --unpark can never
    # restore one (same family as open-PR refusal; zero writes before exit).
    rungs = parse_rungs(base_text)
    diverged = divergent_pointer_reason(base_text, rungs)
    if diverged:
        print("ORIENTATION-PARK-VERDICT: FAIL(divergent-pointer)", file=sys.stderr)
        print(f"gen_orientation: {diverged}", file=sys.stderr)
        sys.exit(1)
    payload, tables_after, handoff_paths = _collect_park_payload(rel, base_text, old_payload)
    parked_text = _replace_status_header(base_text, "PARKED").rstrip() + "\n\n" + park_block_for_payload(payload)
    txn = _FileTxn()
    for path in [target, ACTIVE_TRACK, OUTPUT, BINDING_TSV, OWNER_DIRECTIVES, CLOSEOUT_ARTIFACTS, *handoff_paths]:
        txn.stage(path)
    try:
        for path, (hdr, rows) in tables_after.items():
            write_tsv_dict(path, hdr, rows)
        for path in handoff_paths:
            if path.exists():
                path.unlink()
        target.write_text(parked_text, encoding="utf-8", newline="\n")
        current = read_active_track_pointer()
        if current.get("path") == rel:
            write_active_track_pointer(NO_ACTIVE_TRACK)
        generated, _track_state, _next_rung = render_orientation(active_pointer_for_render(strict=False))
        write_orientation(generated)
        if os.environ.get("ORIENTATION_FORCE_FAULT_AFTER_WRITES") == "1":
            raise RuntimeError("selftest fault injection: park post-write failure")
    except BaseException:
        txn.rollback()
        print("ORIENTATION-PARK-VERDICT: FAIL(transaction-rolled-back)", file=sys.stderr)
        sys.exit(1)
    print(f"ORIENTATION-PARK-VERDICT: PARKED receipt={payload['park_receipt'] if 'park_receipt' in payload else park_payload_receipt(payload)}")
    print(f"track_doc: {rel}")
    print(f"track_id: {payload['track_id']}")
    print(f"moved_rows: {sum(len(v['rows']) for v in payload['tables'].values())}")
    print(f"folded_handoffs: {len(payload['handoffs'])}")
    for name, count in sorted(payload.get("drop_counts", {}).items()):
        print(f"drop_{name}: {count}")
    return 0


def unpark_track() -> int:
    if not OPEN_TARGET:
        fail("--unpark requires exactly one track doc")
    assert_coherent_open_root()
    rel = normalize_track_doc_arg(OPEN_TARGET, must_exist=True)
    target = REPO_ROOT / rel
    original_text = target.read_text(encoding="utf-8")
    base_text, payload = split_park_block(original_text)
    if payload is None:
        return open_track_core(verdict_prefix="UNPARK", require_existing=True)
    if FORCE_OWNER:
        fail("--force-owner is only valid for virgin --unpark with no parked block")
    if payload.get("track_doc") != rel:
        fail(f"parked block is for {payload.get('track_doc')!r}, not {rel!r}")
    txn = _FileTxn()
    table_paths = {
        "binding_conditions.tsv": BINDING_TSV,
        "owner_directives.tsv": OWNER_DIRECTIVES,
        "closeout_artifacts.tsv": CLOSEOUT_ARTIFACTS,
    }
    for path in [target, ACTIVE_TRACK, OUTPUT, *table_paths.values()]:
        txn.stage(path)
    track_id = payload.get("track_id", outgoing_track_id(rel))
    rung_ids = {rung for _num, rung, _scope, _exit in parse_rungs(base_text)}
    rung_ids = {r.split("`")[1] if "`" in r and len(r.split("`")) >= 3 else r.strip("`").strip()
                for r in rung_ids}
    drop_counts = {}
    handoffs = []
    for h in payload.get("handoffs", []):
        if _handoff_referent_live(h, rung_ids):
            handoffs.append(h)
        else:
            drop_counts["stale_handoffs"] = drop_counts.get("stale_handoffs", 0) + 1
    handoffs = _merge_handoffs_by_path([], handoffs, drop_counts)
    for h in handoffs:
        txn.stage(REPO_ROOT / h.get("path", ""))
    dropped = []
    try:
        for name, path in table_paths.items():
            table = payload.get("tables", {}).get(name, {})
            hdr = table.get("header") or []
            rows = table.get("rows") or []
            live_hdr, live_rows = read_tsv_dict(path, hdr)
            handoff_paths = {h.get("path", "") for h in handoffs}
            rows = _filter_table_referents(
                name, rows, live_hdr or hdr, track_id, rung_ids, handoff_paths, drop_counts
            )
            restored = _merge_at_original_positions(live_rows, rows, live_hdr)
            write_tsv_dict(path, live_hdr, restored)
        for h in handoffs:
            hrel = clean_repo_relpath(h.get("path", ""))
            if not hrel:
                dropped.append(h.get("path", ""))
                continue
            hpath = REPO_ROOT / hrel
            hpath.parent.mkdir(parents=True, exist_ok=True)
            hpath.write_text(h.get("content", ""), encoding="utf-8", newline="\n")
        target.write_text(_replace_status_header(base_text, "OPEN"), encoding="utf-8", newline="\n")
        write_active_track_pointer(rel)
        generated, _track_state, _next_rung = render_orientation(active_pointer_for_render(strict=True))
        write_orientation(generated)
        if os.environ.get("ORIENTATION_FORCE_FAULT_AFTER_WRITES") == "1":
            raise RuntimeError("selftest fault injection: unpark post-write failure")
    except BaseException:
        txn.rollback()
        print("ORIENTATION-UNPARK-VERDICT: FAIL(transaction-rolled-back)", file=sys.stderr)
        sys.exit(1)
    print(f"ORIENTATION-UNPARK-VERDICT: UNPARKED receipt={payload.get('park_receipt')}")
    print(f"track_doc: {rel}")
    print(f"restored_rows: {sum(len(v.get('rows', [])) for v in payload.get('tables', {}).values())}")
    print(f"restored_handoffs: {len(handoffs) - len(dropped)}")
    for name, count in sorted(drop_counts.items()):
        print(f"drop_{name}: {count}")
    if dropped:
        print(f"dropped_stale_referents: {len(dropped)}")
    return 0


def commit_forced_open(rel: str, target: pathlib.Path, created_planned: bool, old_info: dict,
                       changed_pointer: bool, verdict_prefix: str = "OPEN"):
    """One admitted operation: preflight (zero writes; plans the exact directive bytes) -> stage all
    four mutation targets up front -> apply skeleton/pointer/orientation writes then the admitted
    directive bytes -> roll back to original bytes/existence on ANY failure. The pointer transition
    and the Owner authority row succeed or fail together."""
    outgoing_rel = old_info["path"]
    planned_directive_bytes = preflight_forced_open(target, created_planned, outgoing_rel)
    scope = outgoing_track_id(outgoing_rel)
    txn = _FileTxn()
    # Stage every mutation target BEFORE any write so rollback covers all four regardless of where a
    # failure occurs.
    txn.stage(target)
    txn.stage(ACTIVE_TRACK)
    txn.stage(OUTPUT)
    txn.stage(OWNER_DIRECTIVES)
    try:
        if created_planned:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(skeleton_doc(rel), encoding="utf-8", newline="\n")
        if changed_pointer:
            write_active_track_pointer(rel)
        active_info = {
            "state": "doc", "path": rel, "raw": rel, "reason": "",
            "design_doc": REPO_ROOT / rel,
        }
        generated, track_state, next_rung = render_orientation(active_info)
        regenerated = write_orientation(generated)
        _forced_fault_after_writes()
        # Write exactly the preflight-admitted directive-table bytes (no reconstruct-at-commit).
        OWNER_DIRECTIVES.parent.mkdir(parents=True, exist_ok=True)
        OWNER_DIRECTIVES.write_text(planned_directive_bytes, encoding="utf-8", newline="\n")
    except BaseException:
        txn.rollback()
        print(f"ORIENTATION-{verdict_prefix}-VERDICT: FAIL(forced-transition-aborted)", file=sys.stderr)
        print(
            "gen_orientation: FAIL(forced-transition-aborted): staged forced open rolled back; "
            "pointer, orientation, directive table, and target left unchanged.",
            file=sys.stderr,
        )
        sys.exit(1)
    print(f"ORIENTATION-{verdict_prefix}-FORCE-OWNER: recorded scope={scope}", file=sys.stderr)
    if created_planned:
        verdict = "CREATED"
    elif track_state in {"closed", "parked", "end-state"}:
        verdict = "OPENED-WARN(closed-or-parked)"
    else:
        verdict = "OPENED"
    return verdict, track_state, next_rung, regenerated, created_planned


def commit_normal_open(rel: str, target: pathlib.Path, created_planned: bool, design_text: str,
                       old_info: dict, changed_pointer: bool, verdict_prefix: str = "OPEN"):
    txn = _FileTxn()
    if created_planned:
        txn.stage(target)
    txn.stage(ACTIVE_TRACK)
    txn.stage(OUTPUT)
    try:
        if created_planned:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(design_text, encoding="utf-8", newline="\n")
        if changed_pointer:
            write_active_track_pointer(rel)
        active_info = active_pointer_for_render(strict=True)
        generated, track_state, next_rung = render_orientation(active_info)
        regenerated = write_orientation(generated)
        _forced_fault_after_writes()
    except BaseException:
        txn.rollback()
        print(f"ORIENTATION-{verdict_prefix}-VERDICT: FAIL(open-transition-aborted)", file=sys.stderr)
        print(
            "gen_orientation: FAIL(open-transition-aborted): staged open rolled back; "
            "pointer, orientation, and target left unchanged.",
            file=sys.stderr,
        )
        sys.exit(1)
    if created_planned:
        verdict = "CREATED"
    elif track_state in {"closed", "parked", "end-state"}:
        verdict = "OPENED-WARN(closed-or-parked)"
    else:
        verdict = "OPENED"
    return verdict, track_state, next_rung, regenerated, created_planned


def open_track_core(verdict_prefix: str = "OPEN", require_existing: bool = False) -> int:
    if not OPEN_TARGET:
        fail(f"--{verdict_prefix.lower()} requires exactly one track doc")
    assert_coherent_open_root()
    rel = normalize_track_doc_arg(OPEN_TARGET, must_exist=require_existing)
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

    # Pointer-lifecycle gate (HD-POINTER-LIFECYCLE-GATE-0): refuse abandoning a still-open outgoing
    # track. Only fires when the pointer actually moves off a live workplan doc whose status header
    # does not declare CLOSED/PARKED. `--force-owner "<text>"` proceeds AND records the override —
    # but the directive row is DEFERRED until the transition is admitted (two-phase; see below), so a
    # forced open that later fails classification leaves no stray Owner directive.
    force_needed = False
    force_outgoing_rel = ""
    if old_info.get("state") == "doc" and old_info.get("path") != rel:
        outgoing_rel = old_info["path"]
        outgoing_text = (REPO_ROOT / outgoing_rel).read_text(encoding="utf-8")
        if not status_header_closed_or_parked(outgoing_text):
            if FORCE_OWNER:
                force_needed = True
                force_outgoing_rel = outgoing_rel
            else:
                print("ORIENTATION-OPEN-VERDICT: FAIL(outgoing-track-open)", file=sys.stderr)
                print(
                    f"gen_orientation: FAIL(outgoing-track-open): outgoing active track "
                    f"{outgoing_rel} status header lacks CLOSED/PARKED; close or park it first, "
                    f'or pass --force-owner "<directive text>" to override.',
                    file=sys.stderr,
                )
                sys.exit(1)

    # Classify the target WITHOUT writing (skeleton content is classified in memory for a new track),
    # so a non-workplan target aborts before any mutation on both the normal and forced paths.
    created_planned = not target.exists()
    if not created_planned and not target.is_file():
        fail(f"track doc target is not a file: {rel}")
    design_text = skeleton_doc(rel) if created_planned else target.read_text(encoding="utf-8")
    ok, reason = classify_workplan(rel, design_text)
    if not ok:
        fail_non_workplan(f"{rel}: {reason}")

    changed_pointer = old_info.get("path") != rel

    if force_needed:
        # Forced open is one admitted, rollback-guarded operation (skeleton + pointer + orientation +
        # Owner directive commit together or not at all).
        verdict, track_state, next_rung, regenerated, created = commit_forced_open(
            rel, target, created_planned, old_info, changed_pointer, verdict_prefix
        )
    else:
        verdict, track_state, next_rung, regenerated, created = commit_normal_open(
            rel, target, created_planned, design_text, old_info, changed_pointer, verdict_prefix
        )

    if verdict == "CREATED":
        next_action = "populate production track ladder/rungs before coding work"
    elif verdict == "OPENED-WARN(closed-or-parked)":
        next_action = "owner/DA must clarify whether this is a reopen, audit, or new successor track before production coding"
    else:
        next_action = "orientation aligned"

    print(f"ORIENTATION-{verdict_prefix}-VERDICT: {verdict}")
    print(f"active_track_from: {old}")
    print(f"active_track_to: {rel}")
    print(f"orientation_regenerated: {'yes' if regenerated else 'no'}")
    print(f"track_state: {'new' if created else track_state}")
    print(f"next_rung: {next_rung}")
    print(f"next_action: {next_action}")
    return 0


def open_track() -> int:
    return open_track_core(verdict_prefix="OPEN", require_existing=False)


if MODE == "check":
    sys.exit(check_orientation())
if MODE == "open":
    sys.exit(open_track())
if MODE == "park":
    sys.exit(park_track())
if MODE == "unpark":
    sys.exit(unpark_track())
sys.exit(generate_orientation())
PY
}

main "$@"
