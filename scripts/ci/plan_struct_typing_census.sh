#!/usr/bin/env bash
# PLAN-STRUCT-TYPING-0 production census: GPU round-trip remints live only in wgsl_encode.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

# Normalize Windows backslashes so allowlist matching is portable.
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'ColumnIndex::from_gpu_round_trip\(' \
    | tr '\\' '/' \
    | grep -Ev 'crates/simthing-core/src/column_index\.rs|crates/simthing-kernel/src/wgsl_encode\.rs' \
    || true
)"

if [[ -n "${hits}" ]]; then
  echo "FAIL(plan-struct-typing-census): production from_gpu_round_trip outside wgsl_encode:" >&2
  echo "${hits}" >&2
  exit 1
fi

echo "PASS(plan-struct-typing-census): zero production from_gpu_round_trip outside wgsl_encode (+ column_index door definition)"
