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

# Net `{`/`}` delta for one source line (// comments stripped; string-agnostic heuristic).
line_brace_delta() {
  local s="$1"
  if [[ "$s" == *"//"* ]]; then
    s="${s%%//*}"
  fi
  local i ch
  local delta=0
  for ((i = 0; i < ${#s}; i++)); do
    ch="${s:i:1}"
    if [[ "$ch" == "{" ]]; then
      delta=$((delta + 1))
    elif [[ "$ch" == "}" ]]; then
      delta=$((delta - 1))
    fi
  done
  printf '%s' "$delta"
}

# Emit "open_line close_line" pairs for every brace-balanced `#[cfg(test)] mod tests`.
cfg_test_mod_spans() {
  local file="$1"
  [[ -f "$file" ]] || return 0
  local -a lines=()
  mapfile -t lines < "$file"
  local n=${#lines[@]}
  local i=0
  while ((i < n)); do
    local l="${lines[i]}"
    if [[ ! "$l" =~ ^[[:space:]]*#\[[Cc]fg\(test\)\] ]]; then
      i=$((i + 1))
      continue
    fi
    local j
    local mod_idx=-1
    for ((j = i; j < n && j <= i + 4; j++)); do
      if [[ "${lines[j]}" =~ mod[[:space:]]+tests ]]; then
        mod_idx=$j
        break
      fi
    done
    if ((mod_idx < 0)); then
      i=$((i + 1))
      continue
    fi
    local k
    local open_idx=-1
    local depth=0
    for ((k = mod_idx; k < n; k++)); do
      local delta
      delta="$(line_brace_delta "${lines[k]}")"
      if ((open_idx < 0)); then
        if [[ "${lines[k]}" == *"{"* ]]; then
          open_idx=$k
          depth=$delta
          if ((depth <= 0)); then
            printf '%s %s\n' "$((open_idx + 1))" "$((open_idx + 1))"
            break
          fi
        fi
        continue
      fi
      depth=$((depth + delta))
      if ((depth <= 0)); then
        printf '%s %s\n' "$((open_idx + 1))" "$((k + 1))"
        break
      fi
    done
    if ((open_idx >= 0 && depth > 0)); then
      printf '%s %s\n' "$((open_idx + 1))" "$n"
    fi
    i=$((i + 1))
  done
}

# True when path:line sits inside a brace-balanced `#[cfg(test)] mod tests { ... }` body.
# Hits after the module's closing brace remain visible to the census.
in_cfg_test_mod_region() {
  local hit="$1"
  local file line_num
  if [[ "$hit" =~ ^(.+):([0-9]+): ]]; then
    file="${BASH_REMATCH[1]}"
    line_num="${BASH_REMATCH[2]}"
  else
    return 1
  fi
  local open close
  while read -r open close; do
    [[ -z "$open" ]] && continue
    if ((line_num >= open && line_num <= close)); then
      return 0
    fi
  done < <(cfg_test_mod_spans "$file")
  return 1
}

# Drop hits that live inside brace-balanced `#[cfg(test)] mod tests` bodies.
filter_cfg_test_mod_hits() {
  local hits="$1"
  local kept=""
  local hit
  declare -A span_cache=()
  while IFS= read -r hit || [[ -n "$hit" ]]; do
    [[ -z "$hit" ]] && continue
    local file line_num
    if [[ "$hit" =~ ^(.+):([0-9]+): ]]; then
      file="${BASH_REMATCH[1]}"
      line_num="${BASH_REMATCH[2]}"
    else
      kept+="${hit}"$'\n'
      continue
    fi
    if [[ -z "${span_cache[$file]+x}" ]]; then
      span_cache[$file]="$(cfg_test_mod_spans "$file")"
    fi
    local in_mod=1
    local open close
    while read -r open close; do
      [[ -z "$open" ]] && continue
      if ((line_num >= open && line_num <= close)); then
        in_mod=0
        break
      fi
    done <<< "${span_cache[$file]}"
    if [[ "$in_mod" -eq 0 ]]; then
      continue
    fi
    kept+="${hit}"$'\n'
  done <<< "$hits"
  printf '%s' "$kept" | sed '/^$/d' || true
}

run_cfg_test_filter_selftest() {
  local tmp
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/plan-struct-cfg-test-XXXXXX")"
  # shellcheck disable=SC2064
  trap "rm -rf '$tmp'" RETURN
  local fixture="$tmp/oracle_door_cfg_test_span.rs"
  cat >"$fixture" <<'EOF'
fn production_before() {}

#[cfg(test)]
mod tests {
    fn inside_oracle_door() {
        let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(0);
    }
}

fn production_after_closed_test_module() {
    let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(99);
}
EOF
  # Line map (1-based): 6 = inside body, 11 = after closing brace.
  local inside_hit="${fixture}:6:        let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(0);"
  local after_hit="${fixture}:11:    let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(99);"
  if ! in_cfg_test_mod_region "$inside_hit"; then
    fail "selftest: in-module oracle-door hit must be excluded (cfg(test) body)"
  fi
  if in_cfg_test_mod_region "$after_hit"; then
    fail "selftest: post-module oracle-door hit must remain visible after closing brace"
  fi
  local filtered
  filtered="$(filter_cfg_test_mod_hits $'crates/x:1:noop\n'"${inside_hit}"$'\n'"${after_hit}")"
  if [[ "$filtered" == *":6:"* ]]; then
    fail "selftest: filter must drop in-module hit:" "$filtered"
  fi
  if [[ "$filtered" != *":11:"* ]]; then
    fail "selftest: filter must retain post-module hit:" "$filtered"
  fi
  echo "PASS: cfg(test) mod tests filter is brace-balanced (in-module excluded; post-brace visible)"
}

if [[ "${1:-}" == "--selftest" ]]; then
  run_cfg_test_filter_selftest
  echo "PASS(plan-struct-typing-census): --selftest green"
  exit 0
fi

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
# Production src allowlist: door definition + oracle/rehearsal modules.
# Brace-balanced `#[cfg(test)] mod tests { ... }` bodies are excluded structurally.
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'ColumnIndex::from_raw_for_oracle_or_rehearsal\(' \
    | normalize \
    | grep -Ev 'crates/simthing-core/src/column_index\.rs' \
    | grep -Ev 'dress_rehearsal|_oracle\.rs|arena_allocation_oracle\.rs' \
    || true
)"
hits="$(filter_cfg_test_mod_hits "$hits")"
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
