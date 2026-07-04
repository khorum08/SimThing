#!/usr/bin/env bash
# Lightweight reason-split proof for doctrine_surface_truth.sh and doctrine_exec inspect mapping.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=doctrine_surface_truth_inspect.sh
source "${ROOT}/scripts/ci/doctrine_surface_truth_inspect.sh"

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

# 1. Missing cargo-public-api → tooling-gap
assert_output_has "missing cargo-public-api" 'SURFACE-TRUTH-REASON: tooling-gap' \
  env PATH=/usr/bin:/bin bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

# 2. PASS → match (synthetic probe avoids nightly/public-api dependency)
assert_output_has "synthetic match" 'SURFACE-TRUTH-REASON: match' \
  env DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-match bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

# 3. Divergence probe (prefer live invisible-pub-use when nightly public-api is available)
divergence_out=""
divergence_out="$(env DOCTRINE_SURFACE_TRUTH_PROBE=invisible-pub-use bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh" 2>&1 || true)"
if echo "$divergence_out" | grep -q 'SURFACE-TRUTH-REASON: divergence'; then
  pass "invisible-pub-use divergence"
else
  assert_output_has "synthetic divergence" 'SURFACE-TRUTH-REASON: divergence' \
    env DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-divergence bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"
fi

# 4. Additional tooling-gap paths
assert_output_has "missing baseline tooling-gap" 'SURFACE-TRUTH-REASON: tooling-gap' \
  env DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-tooling-gap-missing-baseline bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

assert_output_has "empty current tooling-gap" 'SURFACE-TRUTH-REASON: tooling-gap' \
  env DOCTRINE_SURFACE_TRUTH_PROBE=synthetic-tooling-gap-empty bash "${ROOT}/scripts/ci/doctrine_surface_truth.sh"

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