#!/usr/bin/env bash
# HU-DELTA-SCAN-0 — delta-first coding screen (thin wrapper; no new scan logic).
# RELIABLE hard-FAIL: doctrine_scan machinery (whole-tree reliable in --pr-delta mode).
# HEURISTIC: changed-files/lines only via doctrine_scan --pr-delta.
# Ambient whole-tree HEURISTIC INSPECT never appears in --pr-delta output.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SCAN_SH="${SCRIPT_DIR}/doctrine_scan.sh"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/agent_scan"

# Prefer the same bash that invoked us (Windows git-bash-safe for nested python/subprocess later).
if command -v cygpath >/dev/null 2>&1; then
  AGENT_SCAN_BASH="$(cygpath -w "$(command -v bash)" 2>/dev/null || command -v bash)"
else
  AGENT_SCAN_BASH="$(command -v bash)"
fi
export AGENT_SCAN_BASH

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/agent_scan.sh [--base <sha>] [--head <sha>]
  bash scripts/ci/agent_scan.sh --selftest

Default base: merge-base of origin/master (or master) and HEAD.
Default head: HEAD.

Emits scan report from doctrine_scan --pr-delta plus footer:
  AGENT-SCAN-VERDICT: PASS|FAIL|INSPECT delta_inspect=N elapsed=Ns
EOF
  exit 2
}

normalize_text() {
  # Strip UTF-8 BOM and CR for stable footer parsing on Windows.
  local s="$1"
  s="${s#$'\xEF\xBB\xBF'}"
  s="${s//$'\r'/}"
  printf '%s' "$s"
}

resolve_default_base() {
  local head="$1"
  local mb=""
  if git -C "$REPO_ROOT" rev-parse --verify origin/master >/dev/null 2>&1; then
    mb="$(git -C "$REPO_ROOT" merge-base origin/master "$head" 2>/dev/null || true)"
  fi
  if [[ -z "$mb" ]] && git -C "$REPO_ROOT" rev-parse --verify master >/dev/null 2>&1; then
    mb="$(git -C "$REPO_ROOT" merge-base master "$head" 2>/dev/null || true)"
  fi
  if [[ -z "$mb" ]]; then
    mb="$(git -C "$REPO_ROOT" rev-parse "${head}^" 2>/dev/null || git -C "$REPO_ROOT" rev-parse "$head")"
  fi
  printf '%s' "$mb"
}

run_agent_scan() {
  local base_sha="$1"
  local head_sha="$2"
  local start end elapsed out verdict inspect exit_code agent_verdict

  if ! git -C "$REPO_ROOT" rev-parse --verify "${base_sha}^{commit}" >/dev/null 2>&1; then
    echo "agent_scan: invalid base SHA: ${base_sha}" >&2
    exit 2
  fi
  if ! git -C "$REPO_ROOT" rev-parse --verify "${head_sha}^{commit}" >/dev/null 2>&1; then
    echo "agent_scan: invalid head SHA: ${head_sha}" >&2
    exit 2
  fi

  start="$(date +%s)"
  out="$(mktemp "${TMPDIR:-/tmp}/agent-scan-XXXXXX")"
  set +e
  # Skip inventory/lifecycle stock gates for coding-speed; RELIABLE scan rules still run whole-tree.
  # Drift remains PR CI / maintainer whole-tree doctrine_scan responsibility.
  (
    cd "$REPO_ROOT"
    DOCTRINE_SCAN_SKIP_DRIFT=1 bash "$SCAN_SH" --pr-delta "$base_sha" "$head_sha"
  ) >"$out" 2>&1
  exit_code=$?
  set -e
  end="$(date +%s)"
  elapsed=$((end - start))
  if [[ "$elapsed" -lt 0 ]]; then
    elapsed=0
  fi

  # Show scan report (delta HEURISTIC only; no ambient whole-tree INSPECT lines from HEURISTIC).
  cat "$out"

  verdict_line="$(grep 'DOCTRINE-SCAN-VERDICT:' "$out" | tail -n 1 || true)"
  verdict_line="$(normalize_text "$verdict_line")"
  if [[ "$verdict_line" =~ DOCTRINE-SCAN-VERDICT:[[:space:]]*([A-Z]+) ]]; then
    verdict="${BASH_REMATCH[1]}"
  else
    verdict="FAIL"
  fi
  if [[ "$verdict_line" =~ inspect=([0-9]+) ]]; then
    inspect="${BASH_REMATCH[1]}"
  else
    inspect=0
  fi

  case "$verdict" in
    PASS) agent_verdict="PASS" ;;
    INSPECT) agent_verdict="INSPECT" ;;
    FAIL) agent_verdict="FAIL" ;;
    *) agent_verdict="FAIL" ;;
  esac
  # Nonzero scanner exit forces FAIL even if parse missed.
  if [[ "$exit_code" -ne 0 ]]; then
    agent_verdict="FAIL"
  fi

  printf 'AGENT-SCAN-VERDICT: %s delta_inspect=%s elapsed=%ss\n' \
    "$agent_verdict" "$inspect" "$elapsed"
  rm -f "$out"

  if [[ "$agent_verdict" == "FAIL" ]]; then
    return 1
  fi
  return 0
}

prepare_selftest_repo() {
  local root="$1"
  mkdir -p "${root}/scripts/ci/allow"
  mkdir -p "${root}/crates/simthing-kernel/src"
  mkdir -p "${root}/crates/simthing-sim/src"
  mkdir -p "${root}/crates/simthing-spec/src"
  mkdir -p "${root}/crates/simthing-clausething/src"

  cp "${SCRIPT_DIR}/doctrine_scan.sh" \
    "${SCRIPT_DIR}/scans.tsv" \
    "${SCRIPT_DIR}/scan_allowlists.py" \
    "${SCRIPT_DIR}/agent_scan.sh" \
    "${root}/scripts/ci/"
  cp "${SCRIPT_DIR}/allow/"*.txt "${root}/scripts/ci/allow/" 2>/dev/null || true
  # Minimal stubs for stock gates the scan may touch; drift skipped.
  if [[ -f "${REPO_ROOT}/crates/simthing-kernel/src/lib.rs" ]]; then
    cp "${REPO_ROOT}/crates/simthing-kernel/src/lib.rs" "${root}/crates/simthing-kernel/src/lib.rs"
  else
    echo '// stub' >"${root}/crates/simthing-kernel/src/lib.rs"
  fi
  if [[ -f "${REPO_ROOT}/crates/simthing-sim/src/lib.rs" ]]; then
    cp "${REPO_ROOT}/crates/simthing-sim/src/lib.rs" "${root}/crates/simthing-sim/src/lib.rs"
  else
    echo '// stub' >"${root}/crates/simthing-sim/src/lib.rs"
  fi
  echo '// stub' >"${root}/crates/simthing-spec/src/lib.rs"
  echo '// stub' >"${root}/crates/simthing-clausething/src/lib.rs"

  git -C "$root" init -q
  git -C "$root" config user.email "agent-scan@simthing.local"
  git -C "$root" config user.name "agent-scan"
  git -C "$root" add -A
  git -C "$root" commit -q -m "base"
}

selftest_case() {
  local label="$1"
  local expect_verdict="$2"
  local expect_inspect="$3"
  local base="$4"
  local head="$5"
  local root="$6"

  local out footer verdict inspect
  out="$(mktemp)"
  set +e
  (
    cd "$root"
    DOCTRINE_SCAN_SKIP_DRIFT=1 bash scripts/ci/agent_scan.sh --base "$base" --head "$head"
  ) >"$out" 2>&1
  set -e
  footer="$(grep '^AGENT-SCAN-VERDICT:' "$out" | tail -n 1 || true)"
  footer="$(normalize_text "$footer")"
  if [[ ! "$footer" =~ ^AGENT-SCAN-VERDICT:[[:space:]]+(PASS|FAIL|INSPECT)[[:space:]]+delta_inspect=[0-9]+[[:space:]]+elapsed=[0-9]+s$ ]]; then
    echo "FAIL ${label}: footer grammar: ${footer}"
    sed -n '1,30p' "$out"
    rm -f "$out"
    return 1
  fi
  verdict="$(echo "$footer" | sed -n 's/^AGENT-SCAN-VERDICT: \([A-Z]*\).*/\1/p')"
  inspect="$(echo "$footer" | sed -n 's/.*delta_inspect=\([0-9]*\).*/\1/p')"
  if [[ "$verdict" != "$expect_verdict" ]]; then
    echo "FAIL ${label}: want verdict=${expect_verdict} got ${verdict}"
    sed -n '1,40p' "$out"
    rm -f "$out"
    return 1
  fi
  if [[ -n "$expect_inspect" && "$inspect" != "$expect_inspect" ]]; then
    echo "FAIL ${label}: want delta_inspect=${expect_inspect} got ${inspect}"
    rm -f "$out"
    return 1
  fi
  # Ambient whole-tree HEURISTIC must not dominate: no huge ambient inspect from unchanged heuristic.
  if grep -E 'scan mode: whole-tree$' "$out" >/dev/null 2>&1; then
    if ! grep -E 'scan mode: PR delta' "$out" >/dev/null 2>&1; then
      echo "FAIL ${label}: expected PR delta mode"
      rm -f "$out"
      return 1
    fi
  fi
  echo "PASS ${label} (${footer})"
  rm -f "$out"
  return 0
}

run_selftest() {
  local root failures=0
  root="$(mktemp -d "${TMPDIR:-/tmp}/agent-scan-selftest-XXXXXX")"
  prepare_selftest_repo "$root"
  local base head

  base="$(git -C "$root" rev-parse HEAD)"

  echo "AGENT-SCAN selftest"

  # Case 1: RELIABLE known-bad in changed file -> FAIL
  cat >"${root}/crates/simthing-kernel/src/_agent_scan_unsafe.rs" <<'EOF'
pub unsafe fn agent_scan_prove_unsafe() {}
EOF
  git -C "$root" add crates/simthing-kernel/src/_agent_scan_unsafe.rs
  git -C "$root" commit -q -m "known-bad reliable in delta"
  head="$(git -C "$root" rev-parse HEAD)"
  selftest_case "known-bad in changed file -> FAIL" "FAIL" "" "$base" "$head" "$root" \
    || failures=$((failures + 1))

  # Case 2: HEURISTIC only in UNchanged file -> PASS delta_inspect=0
  git -C "$root" reset --hard "$base" -q
  cat >"${root}/crates/simthing-kernel/src/_agent_scan_existing_heuristic.rs" <<'EOF'
pub struct LaneProbe {
    pub data: [f32; 4],
}
pub fn read_lane(probe: &LaneProbe) -> f32 {
    probe.data[0]
}
EOF
  git -C "$root" add crates/simthing-kernel/src/_agent_scan_existing_heuristic.rs
  git -C "$root" commit -q -m "pre-existing heuristic outside future delta"
  local base2
  base2="$(git -C "$root" rev-parse HEAD)"
  echo '// clean touch only' >"${root}/crates/simthing-kernel/src/_agent_scan_clean.rs"
  git -C "$root" add crates/simthing-kernel/src/_agent_scan_clean.rs
  git -C "$root" commit -q -m "clean delta"
  head="$(git -C "$root" rev-parse HEAD)"
  selftest_case "heuristic only outside delta -> PASS delta_inspect=0" "PASS" "0" "$base2" "$head" "$root" \
    || failures=$((failures + 1))

  # Case 3: footer grammar already checked in every selftest_case; explicit stable re-run clean
  selftest_case "footer grammar stable on clean delta" "PASS" "0" "$base2" "$head" "$root" \
    || failures=$((failures + 1))

  rm -rf "$root"
  if [[ "$failures" -eq 0 ]]; then
    echo "AGENT-SCAN-SELFTEST: PASS (3 fixtures)"
    return 0
  fi
  echo "AGENT-SCAN-SELFTEST: FAIL (${failures})"
  return 1
}

main() {
  local base="" head="HEAD" mode="scan"
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --selftest) mode="selftest"; shift ;;
      --base)
        base="${2:-}"
        [[ -n "$base" ]] || usage
        shift 2
        ;;
      --head)
        head="${2:-}"
        [[ -n "$head" ]] || usage
        shift 2
        ;;
      -h|--help) usage ;;
      *) usage ;;
    esac
  done

  if [[ "$mode" == "selftest" ]]; then
    run_selftest
    exit $?
  fi

  head="$(git -C "$REPO_ROOT" rev-parse "$head")"
  if [[ -z "$base" ]]; then
    base="$(resolve_default_base "$head")"
  else
    base="$(git -C "$REPO_ROOT" rev-parse "$base")"
  fi
  run_agent_scan "$base" "$head"
  exit $?
}

main "$@"
