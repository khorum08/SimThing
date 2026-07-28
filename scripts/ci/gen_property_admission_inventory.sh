#!/usr/bin/env bash
# ANCHOR-DISPOSITION-ADMISSION-0 - generate/check the canonical TP property
# disposition inventory from SpecSessionState.property_admission.
# Never hand-edit scripts/ci/property_admission_inventory.tsv.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUT_DEFAULT="${SCRIPT_DIR}/property_admission_inventory.tsv"

MODE="write"
OUT="${OUT_DEFAULT}"

usage() {
  cat <<'EOF'
usage: gen_property_admission_inventory.sh [--check] [--output PATH]
  (default)  hydrate+install canonical TP properties; write property_admission_inventory.tsv
  --check    fail if PATH is missing or stale vs live property admission
  --output   override output/check path (default: scripts/ci/property_admission_inventory.tsv)
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check) MODE="check"; shift ;;
    --output)
      OUT="${2:-}"
      if [[ -z "$OUT" ]]; then
        echo "gen_property_admission_inventory: --output requires PATH" >&2
        exit 2
      fi
      shift 2
      ;;
    -h|--help) usage; exit 0 ;;
    *)
      echo "gen_property_admission_inventory: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

cd "$REPO_ROOT"
export PROPERTY_ADMISSION_INVENTORY_MODE="$MODE"
export PROPERTY_ADMISSION_INVENTORY_OUT="$OUT"

cargo test -p simthing-clausething --test anchor_disposition_admission_0 generator_cli \
  -- --exact --ignored --nocapture
