#!/usr/bin/env bash
# CI-A-SELF-TEST-0 — exercise committed fixture corpus against doctrine_scan.sh.
set -uo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly CI_SRC="${SCRIPT_DIR}"
readonly FIXTURES="${SCRIPT_DIR}/fixtures"
readonly SCAN_SH="${SCRIPT_DIR}/doctrine_scan.sh"

ROOT_SANDBOX=""
selftest_failures=0
positive_control="FAIL"
rot_test_result="FAIL"
test_budget_result="FAIL"
drift_proof_result="FAIL"

declare -a KB_REPORT=()
declare -a HE_REPORT=()
declare -a TRAP_REPORT=()

cleanup() {
  if [[ -n "${SKELETON:-}" && -d "$SKELETON" ]]; then
    rm -rf "$SKELETON"
  fi
}
trap cleanup EXIT

fail_selftest() {
  selftest_failures=$((selftest_failures + 1))
}

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

SKELETON=""

prepare_skeleton() {
  SKELETON="$(mktemp -d "${TMPDIR:-/tmp}/selftest-skel-XXXXXX")"
  copy_ci_bundle "$SKELETON"
  ensure_minimal_crates "$SKELETON"
  mkdir -p "$SKELETON/crates/simthing-kernel/src"
  mkdir -p "$SKELETON/crates/simthing-sim/src"
  mkdir -p "$SKELETON/crates/simthing-spec/src"
  mkdir -p "$SKELETON/crates/simthing-clausething/src"
  cp "${REPO_ROOT}/crates/simthing-kernel/src/lib.rs" "$SKELETON/crates/simthing-kernel/src/lib.rs" 2>/dev/null || echo '#![forbid(unsafe_code)]' > "$SKELETON/crates/simthing-kernel/src/lib.rs"
  cp "${REPO_ROOT}/crates/simthing-sim/src/lib.rs" "$SKELETON/crates/simthing-sim/src/lib.rs" 2>/dev/null || echo '#![forbid(unsafe_code)]' > "$SKELETON/crates/simthing-sim/src/lib.rs"
  echo '// spec stub' > "$SKELETON/crates/simthing-spec/src/lib.rs"
  echo '// clausething stub' > "$SKELETON/crates/simthing-clausething/src/lib.rs"
}

begin_sandbox() {
  if [[ -z "${SKELETON:-}" || ! -d "$SKELETON" ]]; then
    prepare_skeleton
  fi
  ROOT_SANDBOX="$(mktemp -d "${TMPDIR:-/tmp}/simthing-selftest-XXXXXX")"
  # reliable cross platform tree copy (tar available in git bash)
  (cd "$SKELETON" && tar cf - .) 2>/dev/null | (cd "$ROOT_SANDBOX" && tar xf -) 2>/dev/null || cp -r "$SKELETON/." "$ROOT_SANDBOX/"
}

end_sandbox() {
  if [[ -n "${ROOT_SANDBOX:-}" && -d "$ROOT_SANDBOX" ]]; then
    rm -rf "$ROOT_SANDBOX"
  fi
  ROOT_SANDBOX=""
}

copy_ci_bundle() {
  local root="$1"
  mkdir -p "${root}/scripts/ci/allow"
  cp "${CI_SRC}/doctrine_scan.sh" \
    "${CI_SRC}/scans.tsv" \
    "${CI_SRC}/scan_allowlists.py" \
    "${root}/scripts/ci/"
  cp "${CI_SRC}/allow/"*.txt "${root}/scripts/ci/allow/"
  # copy triage/justif tsv if present to avoid any outer state leakage in scans
  for f in inspect_justifications.tsv triage_log.tsv; do
    if [[ -f "${CI_SRC}/$f" ]]; then
      cp "${CI_SRC}/$f" "${root}/scripts/ci/"
    fi
  done
}

ensure_minimal_crates() {
  local root="$1"
  mkdir -p "${root}/crates/simthing-kernel/src"
  mkdir -p "${root}/crates/simthing-sim/src"
  mkdir -p "${root}/crates/simthing-spec/src"
  mkdir -p "${root}/crates/simthing-clausething/src"
  cat >"${root}/crates/simthing-kernel/src/lib.rs" <<'EOF'
#![forbid(unsafe_code)]
EOF
  cat >"${root}/crates/simthing-sim/src/lib.rs" <<'EOF'
#![forbid(unsafe_code)]
EOF
  cat >"${root}/crates/simthing-spec/src/lib.rs" <<'EOF'
// spec stub for self-test sandbox
EOF
  cat >"${root}/crates/simthing-clausething/src/lib.rs" <<'EOF'
// clausething stub for self-test sandbox
EOF
}

prepare_trap_baseline() {
  local root="$1"
  copy_ci_bundle "$root"
  mkdir -p "${root}/crates/simthing-kernel/src"
  mkdir -p "${root}/crates/simthing-sim/src"
  mkdir -p "${root}/crates/simthing-spec/src"
  mkdir -p "${root}/crates/simthing-clausething/src"
  cp "${REPO_ROOT}/crates/simthing-kernel/src/lib.rs" \
    "${root}/crates/simthing-kernel/src/lib.rs"
  cp "${REPO_ROOT}/crates/simthing-sim/src/lib.rs" \
    "${root}/crates/simthing-sim/src/lib.rs"
  cat >"${root}/crates/simthing-spec/src/lib.rs" <<'EOF'
// spec stub for self-test sandbox
EOF
  cat >"${root}/crates/simthing-clausething/src/lib.rs" <<'EOF'
// clausething stub for self-test sandbox
EOF
}

run_scan_in_sandbox() {
  local root="$1"
  local out_file="$2"
  set +e
  (cd "$root" && DOCTRINE_SCAN_SKIP_DRIFT=1 bash "scripts/ci/doctrine_scan.sh") >"$out_file" 2>&1
  printf '%s' $?
  set -e
}

parse_footer_verdict() {
  local out_file="$1"
  local line
  line="$(grep 'DOCTRINE-SCAN-VERDICT:' "$out_file" | tail -1 || true)"
  if [[ "$line" =~ DOCTRINE-SCAN-VERDICT:[[:space:]]*(PASS|FAIL|INSPECT)[[:space:]]+failures=([0-9]+)[[:space:]]+inspect=([0-9]+) ]]; then
    printf '%s %s %s' "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}" "${BASH_REMATCH[3]}"
    return 0
  fi
  # fallback tolerant parse for lines with trailing fields like selftest=...
  if [[ "$line" =~ DOCTRINE-SCAN-VERDICT:[[:space:]]*([A-Z]+)[[:space:]]+failures=([0-9]+)[[:space:]]+inspect=([0-9]+) ]]; then
    printf '%s %s %s' "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}" "${BASH_REMATCH[3]}"
    return 0
  fi
  printf 'UNKNOWN 0 0'
  return 1
}

scan_line_verdict() {
  local out_file="$1"
  local scan_id="$2"
  local line
  line="$(grep -E "^  ${scan_id}  " "$out_file" | head -1 || true)"
  if [[ -z "$line" ]]; then
    printf 'MISSING 0'
    return 1
  fi
  local rest="${line#  ${scan_id}  }"
  local verdict count
  verdict="$(trim "${rest%% *}")"
  rest="${rest#${verdict}}"
  rest="$(trim "$rest")"
  count="$(trim "${rest%% *}")"
  printf '%s %s' "$verdict" "$count"
}

has_scanner_error() {
  grep -q 'scanner/data error' "$1"
}

setup_kernel_src() {
  local fixture="$1"
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/_selftest_fixture.rs"
}

setup_sim_src() {
  local fixture="$1"
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-sim/src/_selftest_fixture.rs"
}

setup_spec_src() {
  local fixture="$1"
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-spec/src/_selftest_fixture.rs"
}

setup_any_crate_src() {
  setup_kernel_src "$1"
}

setup_unsafe_allow_both_libs() {
  copy_ci_bundle "$ROOT_SANDBOX"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-kernel/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-sim/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-spec/src"
  cp "${FIXTURES}/known_bad/unsafe_allow_attr.rs" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/lib.rs"
  cp "${FIXTURES}/known_bad/unsafe_allow_attr.rs" \
    "${ROOT_SANDBOX}/crates/simthing-sim/src/lib.rs"
  echo '// spec stub' >"${ROOT_SANDBOX}/crates/simthing-spec/src/lib.rs"
}

setup_unsafe_forbid_both_libs() {
  copy_ci_bundle "$ROOT_SANDBOX"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-kernel/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-sim/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-spec/src"
  cp "${FIXTURES}/known_bad/unsafe_forbid_missing.rs" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/lib.rs"
  cp "${FIXTURES}/known_bad/unsafe_forbid_missing.rs" \
    "${ROOT_SANDBOX}/crates/simthing-sim/src/lib.rs"
  echo '// spec stub' >"${ROOT_SANDBOX}/crates/simthing-spec/src/lib.rs"
}

setup_heuristic_kernel() {
  local fixture="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/_selftest_fixture.rs"
}

setup_heuristic_sim() {
  local fixture="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-sim/src/_selftest_fixture.rs"
}

setup_heuristic_spec() {
  local fixture="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-spec/src/_selftest_fixture.rs"
}

setup_heuristic_role_resolution_exclude_site_spec() {
  prepare_trap_baseline "$ROOT_SANDBOX"
  cat >"${ROOT_SANDBOX}/crates/simthing-spec/src/_selftest_fixture.rs" <<'EOF'
// CI selftest: deleted generic marker must not suppress SPEC-LOWERER-KIND-READ.
use simthing_core::SimThingKind;

pub fn generic_role_resolution_label(kind: &SimThingKind) -> String {
    match kind { SimThingKind::Fleet => "fleet".into(), other => format!("{other:?}") } // role-resolution-exclude-site
}
EOF
}

setup_heuristic_clausething() {
  local fixture="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/crates/simthing-clausething/src/_selftest_fixture.rs"
}

setup_deny_toml() {
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/deny_toml_stub.txt" "${ROOT_SANDBOX}/deny.toml"
}

setup_kernel_lib_surface() {
  copy_ci_bundle "$ROOT_SANDBOX"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-kernel/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-sim/src"
  mkdir -p "${ROOT_SANDBOX}/crates/simthing-spec/src"
  cp "${FIXTURES}/known_bad/allow_kernel_surface_lib.rs" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/lib.rs"
  cat >"${ROOT_SANDBOX}/crates/simthing-sim/src/lib.rs" <<'EOF'
#![forbid(unsafe_code)]
EOF
  echo '// spec stub' >"${ROOT_SANDBOX}/crates/simthing-spec/src/lib.rs"
}

setup_malformed_allowlist() {
  local fixture="$1"
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  cp "${FIXTURES}/known_bad/${fixture}" \
    "${ROOT_SANDBOX}/scripts/ci/allow/sealed_producers.txt"
}

setup_trap() {
  local trap_file="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/${trap_file}" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/_selftest_trap.rs"
}

setup_trap_spec() {
  local trap_file="$1"
  prepare_trap_baseline "$ROOT_SANDBOX"
  cp "${FIXTURES}/${trap_file}" \
    "${ROOT_SANDBOX}/crates/simthing-spec/src/_selftest_trap.rs"
}

setup_rot_neutralized_spec_lowerer_kind_read() {
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  python - "$ROOT_SANDBOX/scripts/ci/scans.tsv" <<'PY'
import sys
from pathlib import Path
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
old = "match .*\\.kind|\\.kind\\s*(==|!=)|match\\s+(?:&)?kind\\s*\\{[\\s\\S]*?SimThingKind::"
new = "match __NEVER_MATCH__"
if old not in text:
    raise SystemExit("rot-test: SPEC-LOWERER-KIND-READ pattern not found in scans.tsv copy")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
PY
  cp "${FIXTURES}/known_bad/spec_fleet_cohort_kind_branch.rs" \
    "${ROOT_SANDBOX}/crates/simthing-spec/src/_selftest_fixture.rs"
}

setup_rot_neutralized_b3() {
  copy_ci_bundle "$ROOT_SANDBOX"
  ensure_minimal_crates "$ROOT_SANDBOX"
  python - "$ROOT_SANDBOX/scripts/ci/scans.tsv" <<'PY'
import sys
from pathlib import Path
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
old = "pub fn [a-z_]+\\(&self\\) *-> *&"
new = "pub fn __NEVER_MATCH__"
if old not in text:
    raise SystemExit("rot-test: B3 pattern not found in scans.tsv copy")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
PY
  cp "${FIXTURES}/known_bad/b3_buffer_escape.rs" \
    "${ROOT_SANDBOX}/crates/simthing-kernel/src/_selftest_fixture.rs"
}

expect_reliable_fail() {
  local label="$1"
  local scan_id="$2"
  shift 2
  begin_sandbox
  "$@"
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code verdict hard inspect sv count
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "$scan_id")"
  if [[ "$verdict" == "FAIL" && "$hard" -gt 0 && "$sv" == "FAIL" && "$count" -gt 0 ]]; then
    KB_REPORT+=("${scan_id} (${label})  PASS")
  else
    KB_REPORT+=("${scan_id} (${label})  FAIL (verdict=${verdict} scan=${sv} count=${count} exit=${exit_code})")
    fail_selftest
  fi
  end_sandbox
}

expect_heuristic_inspect() {
  local label="$1"
  local scan_id="$2"
  shift 2
  begin_sandbox
  "$@"
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code verdict hard inspect sv count
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "$scan_id")"
  if [[ "$verdict" == "INSPECT" && "$inspect" -gt 0 && "$sv" == "INSPECT" && "$count" -gt 0 && "$exit_code" -eq 0 ]]; then
    HE_REPORT+=("${scan_id} (${label})  PASS")
  else
    HE_REPORT+=("${scan_id} (${label})  FAIL (verdict=${verdict} scan=${sv} count=${count} exit=${exit_code})")
    fail_selftest
  fi
  end_sandbox
}

expect_trap_pass() {
  local label="$1"
  local trap_file="$2"
  begin_sandbox
  setup_trap "$trap_file"
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code verdict hard inspect
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  if [[ "$hard" -eq 0 && "$exit_code" -eq 0 ]]; then
    TRAP_REPORT+=("${label}  PASS")
  else
    TRAP_REPORT+=("${label}  FAIL (verdict=${verdict} hard=${hard} exit=${exit_code})")
    fail_selftest
  fi
  end_sandbox
}

expect_trap_pass_spec() {
  local label="$1"
  local trap_file="$2"
  begin_sandbox
  setup_trap_spec "$trap_file"
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code verdict hard inspect
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  if [[ "$hard" -eq 0 && "$exit_code" -eq 0 ]]; then
    TRAP_REPORT+=("${label}  PASS")
  else
    TRAP_REPORT+=("${label}  FAIL (verdict=${verdict} hard=${hard} exit=${exit_code})")
    fail_selftest
  fi
  end_sandbox
}

expect_scanner_error() {
  local label="$1"
  shift
  begin_sandbox
  "$@"
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  if [[ "$exit_code" -ne 0 ]] && has_scanner_error "$out"; then
    KB_REPORT+=("allowlist validation (${label})  PASS")
  else
    KB_REPORT+=("allowlist validation (${label})  FAIL (exit=${exit_code})")
    fail_selftest
  fi
  end_sandbox
}

run_positive_control() {
  local out
  out="$(mktemp "${TMPDIR:-/tmp}/selftest-positive-XXXXXX")"
  local exit_code verdict hard inspect
  set +e
  bash "${SCAN_SH}" >"$out" 2>&1
  exit_code=$?
  set -e
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  rm -f "$out"
  if [[ "$exit_code" -eq 0 && "$hard" -eq 0 && ( "$verdict" == "PASS" || "$verdict" == "INSPECT" ) ]]; then
    positive_control="PASS"
  else
    positive_control="FAIL (verdict=${verdict} exit=${exit_code})"
    fail_selftest
  fi
}

run_rot_test() {
  begin_sandbox
  setup_rot_neutralized_b3
  local out="${ROOT_SANDBOX}/scan.out"
  local exit_code sv count
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "B3-BUFFER-ESCAPE")"
  if [[ "$sv" == "PASS" && "$count" -eq 0 ]]; then
    rot_test_result="PASS"
  else
    rot_test_result="FAIL (scan=${sv} count=${count} exit=${exit_code})"
    fail_selftest
  fi
  end_sandbox

  begin_sandbox
  setup_rot_neutralized_spec_lowerer_kind_read
  out="${ROOT_SANDBOX}/scan.out"
  exit_code="$(run_scan_in_sandbox "$ROOT_SANDBOX" "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "SPEC-LOWERER-KIND-READ")"
  if [[ "$sv" == "PASS" && "$count" -eq 0 ]]; then
    :
  else
    rot_test_result="FAIL (SPEC-LOWERER-KIND-READ scan=${sv} count=${count} exit=${exit_code})"
    fail_selftest
  fi
  end_sandbox
}

run_test_budget_proof() {
  local root out base head exit_code verdict hard inspect sv count
  root="$(mktemp -d "${TMPDIR:-/tmp}/test-budget-proof-XXXXXX")"
  copy_ci_bundle "$root"
  ensure_minimal_crates "$root"
  mkdir -p "${root}/crates/simthing-spec/tests"
  (
    cd "$root" &&
    git init -q &&
    git config user.email "ci@example.invalid" &&
    git config user.name "Doctrine Selftest" &&
    git add . &&
    git commit -q -m baseline
  )
  base="$(git -C "$root" rev-parse HEAD)"
  cp "${FIXTURES}/test_budget/enumeration_burst.rs" "${root}/crates/simthing-spec/tests/budget.rs"
  (
    cd "$root" &&
    git add . &&
    git commit -q -m enumeration-burst
  )
  head="$(git -C "$root" rev-parse HEAD)"
  out="${root}/scan-enum.out"
  set +e
  (cd "$root" && DOCTRINE_SCAN_SKIP_DRIFT=1 DOCTRINE_SCAN_ONLY_TEST_BUDGET=1 bash "scripts/ci/doctrine_scan.sh" --pr-delta "$base" "$head") >"$out" 2>&1
  exit_code=$?
  set -e
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "TEST-BUDGET")"
  if [[ "$verdict" != "INSPECT" || "$sv" != "INSPECT" || "$count" -le 0 || "$exit_code" -ne 0 ]]; then
    test_budget_result="FAIL (enumeration burst verdict=${verdict} scan=${sv} count=${count} exit=${exit_code})"
    fail_selftest
    rm -rf "$root"
    return
  fi
  rm -rf "$root"

  root="$(mktemp -d "${TMPDIR:-/tmp}/test-budget-trap-XXXXXX")"
  copy_ci_bundle "$root"
  ensure_minimal_crates "$root"
  mkdir -p "${root}/crates/simthing-spec/tests"
  (
    cd "$root" &&
    git init -q &&
    git config user.email "ci@example.invalid" &&
    git config user.name "Doctrine Selftest" &&
    git add . &&
    git commit -q -m baseline
  )
  base="$(git -C "$root" rev-parse HEAD)"
  cp "${FIXTURES}/test_budget/table_driven_trap.rs" "${root}/crates/simthing-spec/tests/budget.rs"
  (
    cd "$root" &&
    git add . &&
    git commit -q -m table-driven-trap
  )
  head="$(git -C "$root" rev-parse HEAD)"
  out="${root}/scan-table.out"
  set +e
  (cd "$root" && DOCTRINE_SCAN_SKIP_DRIFT=1 DOCTRINE_SCAN_ONLY_TEST_BUDGET=1 bash "scripts/ci/doctrine_scan.sh" --pr-delta "$base" "$head") >"$out" 2>&1
  exit_code=$?
  set -e
  read -r verdict hard inspect <<<"$(parse_footer_verdict "$out")"
  read -r sv count <<<"$(scan_line_verdict "$out" "TEST-BUDGET")"
  if [[ "$hard" -eq 0 && "$sv" == "PASS" && "$count" -eq 0 && "$exit_code" -eq 0 ]]; then
    test_budget_result="PASS"
  else
    test_budget_result="FAIL (table trap verdict=${verdict} scan=${sv} count=${count} exit=${exit_code})"
    fail_selftest
  fi
  rm -rf "$root"
}

run_drift_proof() {
  local out exit_code
  out="$(mktemp "${TMPDIR:-/tmp}/drift-proof-XXXXXX")"
  set +e
  bash "${REPO_ROOT}/scripts/ci/test_inventory_drift_check.sh" --prove >"$out" 2>&1
  exit_code=$?
  set -e
  if [[ "$exit_code" -eq 0 ]] && grep -q 'TEST-INVENTORY-DRIFT-PROVE-VERDICT: PASS' "$out"; then
    drift_proof_result="PASS"
  else
    drift_proof_result="FAIL (exit=${exit_code})"
    fail_selftest
  fi
  rm -f "$out"
}

run_all_cases() {
  expect_reliable_fail "b3_buffer_escape" "B3-BUFFER-ESCAPE" \
    setup_kernel_src b3_buffer_escape.rs
  expect_reliable_fail "forge_minter" "FORGE-MINTERS" \
    setup_kernel_src forge_minter.rs
  expect_reliable_fail "unsafe_fn" "UNSAFE-FN" \
    setup_kernel_src unsafe_fn.rs
  expect_reliable_fail "unsafe_allow_attr" "UNSAFE-ALLOW-ATTR" \
    setup_unsafe_allow_both_libs
  expect_reliable_fail "unsafe_forbid_missing" "UNSAFE-FORBID-ATTR" \
    setup_unsafe_forbid_both_libs
  expect_reliable_fail "deny_toml_stub" "DENY-TOML-STUB" setup_deny_toml

  expect_reliable_fail "allow_sealed_producer" "ALLOW-SEALED-PRODUCERS" \
    setup_kernel_src allow_sealed_producer.rs
  expect_reliable_fail "allow_sealed_producer_split" "ALLOW-SEALED-PRODUCERS" \
    setup_kernel_src allow_sealed_producer_split.rs
  expect_reliable_fail "allow_sealed_producer_self" "ALLOW-SEALED-PRODUCERS" \
    setup_kernel_src allow_sealed_producer_self.rs
  expect_reliable_fail "allow_sealed_constructor_new" "ALLOW-SEALED-PRODUCERS" \
    setup_kernel_src allow_sealed_constructor_new.rs
  expect_reliable_fail "allow_sealed_producer_doc_hidden" "ALLOW-SEALED-PRODUCERS" \
    setup_kernel_src allow_sealed_producer_doc_hidden.rs
  expect_reliable_fail "allow_buffer_handle" "ALLOW-BUFFER-HANDLES" \
    setup_kernel_src allow_buffer_handle.rs
  expect_reliable_fail "allow_kernel_surface_lib" "ALLOW-KERNEL-SURFACE" \
    setup_kernel_lib_surface

  expect_scanner_error "malformed_wrong_door" \
    setup_malformed_allowlist malformed_allowlist_wrong_door.txt
  expect_scanner_error "malformed_missing_rationale" \
    setup_malformed_allowlist malformed_allowlist_missing_rationale.txt

  expect_heuristic_inspect "column_index_mint" "COLUMN-INDEX-MINT" \
    setup_heuristic_kernel column_index_mint.rs
  expect_heuristic_inspect "sim_kind_read" "SIM-KIND-READ" \
    setup_heuristic_sim sim_kind_read.rs  expect_heuristic_inspect "semantic_words_production" "SEMANTIC-WORDS" \
    setup_heuristic_kernel semantic_words_production.rs
  expect_heuristic_inspect "spec_string_channel" "SPEC-STRING-CHANNEL" \
    setup_heuristic_spec spec_string_channel.rs
  expect_heuristic_inspect "spec_fleet_cohort_kind_branch" "SPEC-LOWERER-KIND-READ" \
    setup_heuristic_spec spec_fleet_cohort_kind_branch.rs
  expect_heuristic_inspect "clausething_kind_branch" "SPEC-LOWERER-KIND-READ" \
    setup_heuristic_clausething clausething_kind_branch.rs
  expect_heuristic_inspect "clausething_param_kind_branch" "SPEC-LOWERER-KIND-READ" \
    setup_heuristic_clausething clausething_param_kind_branch.rs
  expect_heuristic_inspect "role_resolution_exclude_site_kind_param_match" "SPEC-LOWERER-KIND-READ" \
    setup_heuristic_role_resolution_exclude_site_spec

  expect_trap_pass "jomini_write" "traps/jomini_write.rs"
  expect_trap_pass "studio_antialiasing" "traps/studio_antialiasing.rs"
  expect_trap_pass "pub_crate_sealed_accessor" "traps/pub_crate_sealed_accessor.rs"
  expect_trap_pass "comment_semantic_words" "traps/comment_semantic_words.rs"
  expect_trap_pass "cfg_test_semantic_words" "traps/cfg_test_semantic_words.rs"
  expect_trap_pass "cfg_test_kind_read" "traps/cfg_test_kind_read.rs"
  expect_trap_pass_spec "role_resolution_kind_param_match" \
    traps/role_resolution_kind_param_match.rs
}

emit_report() {
  echo "DOCTRINE SELFTEST REPORT"
  echo "  positive control: ${positive_control}"
  echo "  known-bad:"
  local line
  for line in "${KB_REPORT[@]}"; do
    echo "    ${line}"
  done
  echo "  heuristic controls:"
  for line in "${HE_REPORT[@]}"; do
    echo "    ${line}"
  done
  echo "  traps:"
  for line in "${TRAP_REPORT[@]}"; do
    echo "    ${line}"
  done
  echo "  rot test: ${rot_test_result}"
  echo "  test budget proof: ${test_budget_result}"
  echo "  inventory drift proof: ${drift_proof_result}"
  if [[ "$selftest_failures" -eq 0 ]]; then
    echo "DOCTRINE-SELFTEST-VERDICT: PASS"
  else
    echo "DOCTRINE-SELFTEST-VERDICT: FAIL"
  fi
}

main() {
  if ! command -v rg >/dev/null 2>&1; then
    echo "doctrine_selftest: ripgrep (rg) not found on PATH" >&2
    fail_selftest   # the self-test could not run; it must not report PASS (§0.6.6 false-confidence)
    emit_report
    exit 1
  fi
  if ! command -v python >/dev/null 2>&1; then
    echo "doctrine_selftest: python not found on PATH" >&2
    fail_selftest   # the self-test could not run; it must not report PASS (§0.6.6 false-confidence)
    emit_report
    exit 1
  fi

  if ! bash "${REPO_ROOT}/scripts/ci/doctrine_exec_profile_lint.sh"; then
    fail_selftest
    emit_report
    exit 1
  fi

  run_positive_control
  run_all_cases
  run_rot_test
  run_test_budget_proof
  run_drift_proof
  emit_report

  if [[ "$selftest_failures" -gt 0 ]]; then
    exit 1
  fi
  exit 0
}

main "$@"
