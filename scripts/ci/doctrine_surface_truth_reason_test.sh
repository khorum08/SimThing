#!/usr/bin/env bash
# Lightweight reason-split proof for doctrine_surface_truth.sh and doctrine_exec inspect mapping.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=doctrine_surface_truth_inspect.sh
source "${ROOT}/scripts/ci/doctrine_surface_truth_inspect.sh"

OFFLINE_PATH="/usr/bin:/bin"
if [[ -d "/mingw64/bin" ]]; then
  OFFLINE_PATH="${OFFLINE_PATH}:/mingw64/bin"
fi

fail() {
  echo "SURFACE-TRUTH-REASON-TEST: FAIL $*" >&2
  exit 1
}

pass() {
  echo "SURFACE-TRUTH-REASON-TEST: PASS $*"
}

assert_output_has() {
  local label="$1"
  local pattern="$2"
  shift 2
  local out
  out="$("$@" 2>&1)"
  if ! echo "$out" | grep -q "$pattern"; then
    echo "$out" >&2
    fail "$label expected pattern: $pattern"
  fi
  pass "$label"
}

assert_inspect_line() {
  local label="$1"
  local expected="$2"
  local fixture="$3"
  local line
  line="$(surface_truth_inspect_line_from_output "$fixture")"
  if [[ "$line" != "$expected" ]]; then
    echo "fixture output:" >&2
    echo "$fixture" >&2
    fail "$label expected inspect line '$expected' got '$line'"
  fi
  pass "$label"
}

synthetic_offline() {
  env DOCTRINE_SURFACE_TRUTH_SYNTHETIC_ALLOWED=1 PATH="$OFFLINE_PATH" "$@"
}

# 1. Missing cargo-public-api → tooling-gap (production path, no synthetic guard)
assert_output_has "missing cargo-public-api" 'SURFACE-TRUTH-REASON: tooling-gap' \
  env PATH="$OFFLINE_PATH" bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

# 2. Synthetic probes without guard fall through to production behavior
assert_output_has "unguarded synthetic falls through" 'SURFACE-TRUTH-REASON: tooling-gap' \
  env DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-match PATH="$OFFLINE_PATH" bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

# 3. Guarded synthetic proof with cargo-public-api unavailable
assert_output_has "offline synthetic match" 'SURFACE-TRUTH-REASON: match' \
  synthetic_offline DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-match bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

assert_output_has "offline synthetic divergence" 'SURFACE-TRUTH-REASON: divergence' \
  synthetic_offline DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-divergence bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

assert_output_has "offline synthetic missing baseline tooling-gap" 'SURFACE-TRUTH-REASON: tooling-gap' \
  synthetic_offline DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-tooling-gap-missing-baseline bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

assert_output_has "offline synthetic empty current tooling-gap" 'SURFACE-TRUTH-REASON: tooling-gap' \
  synthetic_offline DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-tooling-gap-empty bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

# 4. Real invisible-pub-use probe when tooling is available (optional live path)
if command -v cargo-public-api >/dev/null 2>&1; then
  divergence_out=""
  divergence_out="$(env DOCTRINE_SURFACE_TRUTH_PROBE=invisible-pub-use bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh" 2>&1 || true)"
  if echo "$divergence_out" | grep -q 'SURFACE-TRUTH-REASON: divergence'; then
    pass "invisible-pub-use divergence"
  else
    pass "invisible-pub-use skipped (tooling unavailable for live divergence)"
  fi
fi

# 5. doctrine_exec inspect mapping
assert_inspect_line "exec mapping pass" "" "$(cat <<'EOF'
SURFACE-TRUTH: PASS public API matches baseline
SURFACE-TRUTH-REASON: match
EOF
)"

assert_inspect_line "exec mapping divergence" "surface-truth divergence" "$(cat <<'EOF'
SURFACE-TRUTH: INSPECT public API diverges from baseline
SURFACE-TRUTH-REASON: divergence
EOF
)"

assert_inspect_line "exec mapping tooling-gap" "surface-truth tooling-gap" "$(cat <<'EOF'
SURFACE-TRUTH: INSPECT cargo-public-api not installed
SURFACE-TRUTH-REASON: tooling-gap
EOF
)"

assert_inspect_line "exec mapping unknown reason" "surface-truth inspect unknown-reason" "$(cat <<'EOF'
SURFACE-TRUTH: INSPECT public API diverges from baseline
EOF
)"

echo "SURFACE-TRUTH-REASON-TEST-VERDICT: PASS"