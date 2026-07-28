#!/usr/bin/env bash
# FIRST-CITIZEN-SPECIALISTS-0 — generate/check citizen-count TSV from the
# canonical TP authority install (SpecSessionState.specialization.citizen_counts).
# Never hand-edit scripts/ci/specialization_citizen_counts.tsv.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
OUT_DEFAULT="${SCRIPT_DIR}/specialization_citizen_counts.tsv"

MODE="write"
OUT="${OUT_DEFAULT}"

usage() {
  cat <<'EOF'
usage: gen_specialization_citizen_counts.sh [--check] [--output PATH]
  (default)  hydrate+install canonical TP; write specialization_citizen_counts.tsv
  --check    fail if PATH is missing or stale vs live citizen_counts()
  --output   override output/check path (default: scripts/ci/specialization_citizen_counts.tsv)
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --check) MODE="check"; shift ;;
    --output)
      OUT="${2:-}"
      if [[ -z "$OUT" ]]; then
        echo "gen_specialization_citizen_counts: --output requires PATH" >&2
        exit 2
      fi
      shift 2
      ;;
    -h|--help) usage; exit 0 ;;
    *)
      echo "gen_specialization_citizen_counts: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

cd "$REPO_ROOT"
export FCS_CITIZEN_COUNTS_MODE="$MODE"
export FCS_CITIZEN_COUNTS_OUT="$OUT"

cargo test -p simthing-clausething --test first_citizen_specialists_0 generator_cli \
  -- --exact --ignored --nocapture
