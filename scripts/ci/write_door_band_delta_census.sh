#!/usr/bin/env bash
# WRITE-DOOR-BAND-DELTA-0 census: fused write-door + structural remap authority.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() {
  echo "FAIL(write-door-band-delta-census): $1" >&2
  shift
  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@" >&2
  fi
  exit 1
}

normalize() {
  tr '\\' '/'
}

# ── 1. BandCrossingDelta mint only via sealed kernel door ────────────────────
hits="$(
  rg -n --glob 'crates/**/*.rs' 'BandCrossingDelta\s*\{' \
    | normalize \
    | grep -Ev 'crates/simthing-kernel/src/sealed/band_crossing_delta\.rs' \
    | grep -Ev 'compile_fail|doctest|docs/' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "BandCrossingDelta struct literal outside sealed mint door:" "${hits}"
fi
echo "PASS: BandCrossingDelta literals confined to sealed door module"

# ── 2. No production CPU post-hoc invent of threshold crossings (hot path) ───
# Forbid inventing crossings from bare value compares outside oracle/rehearsal/
# sealed twins. Known residues (era-0080 schedules) must stay named here if present.
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' 'patrol_threshold_crossed|cpu_invent_band_crossing|infer_threshold_crossing_from_values' \
    | normalize \
    | grep -Ev 'dress_rehearsal|_oracle\.rs|sealed/|cpu_oracle\.rs|default_schedule_0080|gradient_follow_0080' \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "production CPU post-hoc crossing invent outside fenced residues:" "${hits}"
fi
echo "PASS: no unexplained production CPU post-hoc crossing invent symbols"

# ── 3. Structural encode gate symbol must exist ──────────────────────────────
if ! rg -q 'validate_anchor_remap_for_encode' crates/simthing-sim/src crates/simthing-core/src; then
  fail "missing validate_anchor_remap_for_encode encode gate"
fi
echo "PASS: validate_anchor_remap_for_encode present"

# ── 4. Boundary flush must consult the remap encode gate ─────────────────────
# Production call site uses gate_structural_gpu_encode (wraps validate_*).
if ! rg -q 'gate_structural_gpu_encode|validate_anchor_remap_for_encode' \
  crates/simthing-sim/src/boundary.rs; then
  fail "boundary.rs does not call remap encode gate before GPU sync"
fi
echo "PASS: boundary.rs gates structural GPU sync on anchor remap"

# ── 5. Remap-free relocation doors must not bypass the gate ──────────────────
# Direct sync_gpu_buffers from tree_mutation/fission (skipping boundary gate) is forbidden.
hits="$(
  rg -n --glob 'crates/simthing-sim/src/{fission,tree_mutation}.rs' 'sync_gpu_buffers\(' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "remap-free relocation: sync_gpu_buffers called outside boundary gate:" "${hits}"
fi
echo "PASS: fission/tree_mutation do not call sync_gpu_buffers directly"

# ── 6. Fused mint door must exist on AccumulatorOpSession readback ───────────
if ! rg -q 'fn readback_band_crossing_deltas' \
  crates/simthing-kernel/src/accumulator_op/session.rs; then
  fail "missing AccumulatorOpSession::readback_band_crossing_deltas fused mint door"
fi
if ! rg -q 'band_crossing_deltas_from_fused_emissions' \
  crates/simthing-kernel/src/accumulator_op/session.rs; then
  fail "readback mint door does not join fused emissions"
fi
echo "PASS: fused BandCrossingDelta mint door on AccumulatorOpSession"

# ── 7. Structural encode inventory (load-bearing map; must remain named) ─────
for path in \
  crates/simthing-sim/src/fission.rs \
  crates/simthing-sim/src/tree_mutation.rs \
  crates/simthing-sim/src/boundary.rs \
  crates/simthing-sim/src/gpu_sync.rs
do
  if [[ ! -f "$path" ]]; then
    fail "missing structural encode inventory path: $path"
  fi
done
echo "PASS: structural encode inventory paths present (fission/tree_mutation/boundary/gpu_sync)"

echo "PASS(write-door-band-delta-census): fused write-door + structural remap census green"
