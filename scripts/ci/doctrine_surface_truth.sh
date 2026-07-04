#!/usr/bin/env bash
# CI-B-SURFACE-TRUTH-0: compiler-derived public API diff for simthing-kernel (report-only).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASELINE="${ROOT}/scripts/ci/kernel_public_api_baseline.txt"
CURRENT="${ROOT}/scripts/ci/.doctrine_exec_public_api_current.txt"
PROBE_MODE="${DOCTRINE_SURFACE_TRUTH_PROBE:-}"
SYNTHETIC_ALLOWED="${DOCTRINE_SURFACE_TRUTH_SYNTHETIC_ALLOWED:-0}"

cd "$ROOT"

emit_reason() {
  echo "SURFACE-TRUTH-REASON: $1"
}

inspect_tooling_gap() {
  echo "SURFACE-TRUTH: INSPECT $1"
  emit_reason tooling-gap
  exit 0
}

inspect_divergence() {
  echo "SURFACE-TRUTH: INSPECT public API diverges from baseline"
  emit_reason divergence
  if [[ -n "${2:-}" ]]; then
    diff -u "$1" "$2" | head -n 80 || true
  fi
  exit 0
}

evaluate_listing() {
  if [[ ! -f "$BASELINE" ]]; then
    inspect_tooling_gap "missing baseline $BASELINE"
  fi

  if [[ ! -s "$CURRENT" ]]; then
    inspect_tooling_gap "empty current public API listing"
  fi

  if diff -u "$BASELINE" "$CURRENT" >/dev/null 2>&1; then
    echo "SURFACE-TRUTH: PASS public API matches baseline"
    emit_reason match
    exit 0
  fi

  inspect_divergence "$BASELINE" "$CURRENT"
}

emit_public_api() {
  local dir="$1"
  (
    cd "$dir"
    cargo +nightly public-api -p simthing-kernel 2>/dev/null | grep '^pub ' | sed -E 's/\(.*/(...)/' | sort -u || true
  )
}

run_synthetic_probe() {
  case "$PROBE_MODE" in
    synthetic-match)
      cp "$BASELINE" "$CURRENT"
      ;;
    synthetic-divergence)
      printf 'pub fn synthetic_divergence_probe(...)\n' > "$CURRENT"
      ;;
    synthetic-tooling-gap-empty)
      : > "$CURRENT"
      ;;
    synthetic-tooling-gap-missing-baseline)
      BASELINE="${ROOT}/scripts/ci/.synthetic_missing_baseline.txt"
      cp "${ROOT}/scripts/ci/kernel_public_api_baseline.txt" "$CURRENT"
      ;;
    *)
      return 1
      ;;
  esac
  evaluate_listing
}

if [[ "$SYNTHETIC_ALLOWED" == "1" && "$PROBE_MODE" == synthetic-* ]]; then
  run_synthetic_probe
fi

if ! command -v cargo-public-api >/dev/null 2>&1; then
  inspect_tooling_gap "cargo-public-api not installed"
fi

if [[ "$PROBE_MODE" == "invisible-pub-use" ]]; then
  workdir="$(mktemp -d)"
  trap 'rm -rf "$workdir"' EXIT
  cp -r crates/simthing-kernel "$workdir/simthing-kernel"
  echo 'pub use crate::sealed::threshold_event::ThresholdEvent as InvisibleProbeExport;' >> "$workdir/simthing-kernel/src/lib.rs"
  emit_public_api "$workdir/simthing-kernel" > "$CURRENT"
else
  emit_public_api "$ROOT" > "$CURRENT"
fi

evaluate_listing