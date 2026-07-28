#!/usr/bin/env bash
# PLAN-STRUCT-TYPING-0 production census: typed-column authority + WGSL wire boundary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() {
  echo "FAIL(plan-struct-typing-census): $1" >&2
  shift
  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@" >&2
  fi
  exit 1
}

normalize() {
  tr '\\' '/'
}

# ── 1. from_gpu_round_trip only in door definition + wgsl_encode ─────────────
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'ColumnIndex::from_gpu_round_trip\(' \
    | normalize \
    | grep -Ev 'crates/simthing-core/src/column_index\.rs|crates/simthing-kernel/src/wgsl_encode\.rs' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "production from_gpu_round_trip outside wgsl_encode (+ column_index door definition):" "${hits}"
fi
echo "PASS: zero production from_gpu_round_trip outside wgsl_encode (+ column_index door definition)"

# ── 2. from_raw_for_oracle_or_rehearsal only on oracle/rehearsal/test surfaces ─
# Production src allowlist: door definition; oracle/rehearsal modules; unit-test
# fixture constructors in arena_allocation_plan / child_share_eml.
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'ColumnIndex::from_raw_for_oracle_or_rehearsal\(' \
    | normalize \
    | grep -Ev 'crates/simthing-core/src/column_index\.rs' \
    | grep -Ev 'dress_rehearsal|_oracle\.rs|arena_allocation_oracle\.rs' \
    | grep -Ev 'arena_allocation_plan\.rs|child_share_eml\.rs|emission_accumulator\.rs' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "production from_raw_for_oracle_or_rehearsal outside oracle/rehearsal/test:" "${hits}"
fi
echo "PASS: zero production from_raw_for_oracle_or_rehearsal outside oracle/rehearsal/test"

# ── 3. No public bare-raw StructuralScalarChannel authored ctor ───────────────
hits="$(
  rg -n --glob 'crates/**/*.rs' 'from_authored_channel\(' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "from_authored_channel still present (bare public raw ColumnIndex door):" "${hits}"
fi
echo "PASS: zero from_authored_channel constructors"

# ── 4. No literal column_from_wire(<int>) minting ─────────────────────────────
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'column_from_wire\([[:space:]]*[0-9]+[[:space:]]*\)' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "column_from_wire(<literal>) minting:" "${hits}"
fi
echo "PASS: zero column_from_wire(<literal>) minting"

# ── 5. No fabricated PropertyColumnRange { start: 0 } as registry substitute ──
hits="$(
  rg -n -U --glob 'crates/simthing-driver/src/{arena_hierarchy,arena_pressure,arena_allocation_sync,need_binding,gated_rates}.rs' \
    'PropertyColumnRange\s*\{[^}]*start:\s*0' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "fabricated PropertyColumnRange { start: 0 } as registry substitute:" "${hits}"
fi
echo "PASS: no fabricated PropertyColumnRange { start: 0 } as registry substitute"

# ── 6. Direct plan/WGSL raw_u32 drops outside wgsl_encode (targeted POD fields)
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' '\.raw_u32\(\)' \
    | normalize \
    | grep -Ev 'crates/simthing-core/src/column_index\.rs|crates/simthing-kernel/src/wgsl_encode\.rs' \
    | grep -E 'source_col:.*raw_u32|target_col:.*raw_u32|governed_col:.*raw_u32|governing_col:.*raw_u32|choke_output_col:.*map\(.*raw_u32|choke_output_col:.*raw_u32' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "direct plan/WGSL raw_u32 drops outside wgsl_encode:" "${hits}"
fi
echo "PASS: zero direct plan/WGSL raw_u32 drops outside wgsl_encode (targeted POD fields)"

# ── 7. Family-B compiled/intermediate plan records must not store column ids as u32
# Authored/serde (`RegionFieldSpec`, scenario channels) and WGSL/POD wire structs
# are classified elsewhere; this arm only scans named production compile/plan
# records. Avoid whole-file test exemptions — production lines remain visible.
FAMILY_B_PLAN_FILES=(
  'crates/simthing-spec/src/compile/region_field_admission.rs'
  'crates/simthing-spec/src/compile/resource_economy.rs'
  'crates/simthing-driver/src/arena_hierarchy.rs'
  'crates/simthing-driver/src/arena_allocation_plan.rs'
  'crates/simthing-kernel/src/transfer_accumulator.rs'
  'crates/simthing-kernel/src/emission_accumulator.rs'
  'crates/simthing-kernel/src/intensity_accumulator.rs'
  'crates/simthing-core/src/compiled_accumulator_plan.rs'
)
hits=""
for f in "${FAMILY_B_PLAN_FILES[@]}"; do
  if [[ ! -f "${f}" ]]; then
    continue
  fi
  file_hits="$(
    rg -n --glob "${f}" \
      '^\s*(pub\s+)?(source_col|target_col|child_col|parent_col|urgency_col|weight_col|intrinsic_flow_col|allocated_flow_col|choke_output_col)\s*:\s*(Option<)?u32' \
      "${f}" \
      | normalize \
      || true
  )"
  if [[ -n "${file_hits}" ]]; then
    hits+="${file_hits}"$'\n'
  fi
done
hits="$(printf '%s' "${hits}" | sed '/^$/d' || true)"
if [[ -n "${hits}" ]]; then
  fail "Family-B compiled/plan records still carry column identity as u32/Option<u32>:" "${hits}"
fi
echo "PASS: Family-B compiled/plan column identities are typed (no raw u32 plan fields)"

echo "PASS(plan-struct-typing-census): full 4.2 authority + wire-boundary census green"
