#!/usr/bin/env bash
# Shared surface-truth output → Doctrine Exec inspect-line mapping.
# Sourced by doctrine_exec.sh and doctrine_surface_truth_reason_test.sh.

surface_truth_inspect_line_from_output() {
  local surface_out="$1"
  local reason=""

  if echo "$surface_out" | grep -q 'SURFACE-TRUTH: PASS'; then
    return 0
  fi

  if ! echo "$surface_out" | grep -q 'SURFACE-TRUTH: INSPECT'; then
    echo "surface-truth unexpected output" >&2
    return 2
  fi

  reason="$(echo "$surface_out" | grep -E '^SURFACE-TRUTH-REASON:' | tail -n 1 | awk '{print $2}')"
  case "$reason" in
    divergence)
      echo "surface-truth divergence"
      ;;
    tooling-gap)
      echo "surface-truth tooling-gap"
      ;;
    *)
      echo "surface-truth inspect unknown-reason"
      ;;
  esac
  return 0
}