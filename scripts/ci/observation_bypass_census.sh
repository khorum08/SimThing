#!/usr/bin/env bash
# ANCHOR-TABLE-SURFACE-0 census: sole observation door + no consumer bypass.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() {
  echo "FAIL(observation-bypass-census): $1" >&2
  shift
  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@" >&2
  fi
  exit 1
}

normalize() {
  tr '\\' '/'
}

# Reuse WRITE-DOOR remand-4 brace-balanced cfg(test) filter (do not reinvent).
# shellcheck source=lib_cfg_test_mod_spans.sh
source "$(cd "$(dirname "$0")" && pwd)/lib_cfg_test_mod_spans.sh"

# ── 1. Production consumers must not call GpuValuesSnapshot::from_session ────
hits="$(
  rg -n --glob 'crates/**/*.rs' 'GpuValuesSnapshot::from_session' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "production GpuValuesSnapshot::from_session bypass:" "${hits}"
fi
echo "PASS: no production GpuValuesSnapshot::from_session"

# ── 2. Studio / hosted observation must use AnchorTableSnapshot ──────────────
if ! rg -q 'AnchorTableSnapshot::from_session' \
  crates/simthing-mapeditor/src/studio_live_session_bridge.rs \
  crates/simthing-driver/src/hosted_property_observation.rs; then
  fail "missing AnchorTableSnapshot::from_session on Studio/hosted observation door"
fi
hits="$(
  rg -n --glob 'crates/simthing-mapeditor/src/**/*.rs' 'read_values\(\)' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "mapeditor production read_values observation bypass:" "${hits}"
fi
echo "PASS: Studio/hosted observation uses AnchorTableSnapshot"

# ── 3. No CPU band reconstruction symbols in production consumers ───────────
hits="$(
  rg -n --glob 'crates/**/src/**/*.rs' \
    'reconstruct_band_from_values|infer_band_from_raw|cpu_band_from_matrix' \
    | normalize \
    || true
)"
hits="$(filter_cfg_test_mod_hits "${hits}")"
if [[ -n "${hits}" ]]; then
  fail "CPU band reconstruction in production:" "${hits}"
fi
echo "PASS: no production CPU band reconstruction symbols"

# ── 4. Anchor table must not enter wire/replay delta enum ────────────────────
hits="$(
  rg -n 'AnchorTable|AnchorTableRow|anchor_table_row' \
    crates/simthing-sim/src/delta_log.rs \
    crates/simthing-sim/src/replay.rs \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "anchor table must not enter wire/replay authority:" "${hits}"
fi
echo "PASS: anchor table absent from wire/replay delta authority"

# ── 5. Falloff params must not land in 5.3 table schema (DA sharpening) ──────
hits="$(
  rg -n --glob 'crates/simthing-core/src/anchor_table.rs' \
    'falloff_params|AdmittedFalloff|pub falloff' \
    | normalize \
    || true
)"
if [[ -n "${hits}" ]]; then
  fail "falloff fields must be absent from 5.3 typed table (DA 5120052669):" "${hits}"
fi
if ! rg -q 'Option<BandIndex>' crates/simthing-core/src/anchor_table.rs; then
  fail "core row must carry Option<BandIndex> (sentinel only at POD encode)"
fi
if ! rg -q 'ANCHOR_BAND_NONE_POD' crates/simthing-kernel/src/sealed/anchor_table.rs; then
  fail "POD sentinel ANCHOR_BAND_NONE_POD missing at encode boundary"
fi
echo "PASS: typed Option<BandIndex>; no falloff fields; POD sentinel at encode"

# ── 6. Mapeditor surface present (5.1 surfaces-omission precedent) ───────────
if ! rg -q 'simthing-mapeditor' handoffs/ANCHOR-TABLE-SURFACE-0.hd.md; then
  fail "handoff surfaces must include crates/simthing-mapeditor"
fi
echo "PASS: handoff lists mapeditor surfaces"

echo "PASS(observation-bypass-census): all arms green"
