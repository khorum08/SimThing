#!/usr/bin/env bash
# OH-CLEARANCE-ROUTER-0 — emit CLEARANCE-VERDICT for PR/range evidence.
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly FIXTURES_ROOT="${SCRIPT_DIR}/fixtures/clearance"

PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN="python"
fi

GATE_WIRING_PATHS=(
  "scripts/ci/clearance_check.sh"
  "scripts/ci/precedented_classes.tsv"
  "scripts/ci/class_predicates.tsv"
  "scripts/ci/binding_conditions.tsv"
  "scripts/ci/clearance_ledger.tsv"
  "scripts/ci/doctrine_anchors.tsv"
  "scripts/ci/anchor_triggers.tsv"
  "scripts/ci/anchor_reach_log.tsv"
  "scripts/ci/anchor_query.sh"
  "scripts/ci/anchor_check.sh"
  ".github/workflows/doctrine-exec-commands.yml"
  ".github/workflows/clearance.yml"
  "scripts/ci/doctrine_exec_commands.sh"
  "scripts/ci/doctrine_exec_clearance.sh"
  "scripts/ci/doctrine_exec_clearance_comment.sh"
  "scripts/ci/doctrine_exec_triage.sh"
  "scripts/ci/clearance_comment.sh"
  "scripts/ci/triage_log_check.sh"
  "scripts/ci/doc_budget_check.sh"
  "scripts/ci/doc_budget_baseline.tsv"
  "scripts/ci/rule_expiry_check.sh"
  "scripts/ci/agents_stub_check.sh"
  "scripts/ci/da_treeverify.sh"
  "scripts/ci/da_treeverify_lib.py"
  "scripts/ci/da_review_profile.tsv"
  "scripts/ci/agent_scan.sh"
  "scripts/ci/relay_lint.sh"
  "docs/handoff_template.md"
  "docs/agent_onboarding.md"
  "AGENTS.md"
)

ENGINE_CRATE_PREFIXES=(
  "crates/simthing-kernel/"
  "crates/simthing-sim/src/"
  "crates/simthing-spec/src/"
  "crates/simthing-clausething/src/"
  "crates/simthing-gpu/src/"
  "crates/simthing-driver/src/"
)

VERDICT=""
FIXTURE_MODE=""
FIXTURE_DIR=""
PR_NUMBER=""
RANGE_SPEC=""
SELFTEST_FAILURES=0
REQUESTED_TARGET=0
CHANGED_FILES_RESOLVED=0
CHANGED_FILES_LIST=""
PR_RESOLUTION_FAILED=0

usage() {
  cat <<'EOF'
usage:
  bash scripts/ci/clearance_check.sh --selftest
  bash scripts/ci/clearance_check.sh --fixture <name>
  bash scripts/ci/clearance_check.sh --pr <number>
  bash scripts/ci/clearance_check.sh --range <base>..<head>
  bash scripts/ci/clearance_check.sh <pr-number>
EOF
  exit 1
}

emit_treeverify_profile_line() {
  # Advisory fold: same changed-file list → da_treeverify_lib (no router duplication).
  # Only called for DA-RESERVE; never for CLEARABLE/FAIL.
  local files_tmp body_tmp profile_tsv
  profile_tsv="${SCRIPT_DIR}/da_review_profile.tsv"
  [[ -f "$profile_tsv" ]] || {
    printf 'DA-TREEVERIFY-PROFILE: DEEP-TREE\n'
    return 0
  }
  files_tmp="$(mktemp "${TMPDIR:-/tmp}/clearance-tv-files-XXXXXX")"
  body_tmp="$(mktemp "${TMPDIR:-/tmp}/clearance-tv-body-XXXXXX")"
  changed_files 2>/dev/null >"$files_tmp" || true
  pr_body_text >"$body_tmp" 2>/dev/null || true
  local out line
  out="$("$PYTHON_BIN" "${SCRIPT_DIR}/da_treeverify_lib.py" profile \
    --profile "$profile_tsv" --files "$files_tmp" --body "$body_tmp" 2>/dev/null || true)"
  rm -f "$files_tmp" "$body_tmp"
  line="$(printf '%s\n' "$out" | grep -E '^DA-TREEVERIFY-PROFILE:' | head -n 1 || true)"
  if [[ -n "$line" ]]; then
    printf '%s\n' "$line"
  else
    printf 'DA-TREEVERIFY-PROFILE: DEEP-TREE\n'
  fi
}

required_anchor_ids_for_changed_files() {
  local anchors_tsv="${SCRIPT_DIR}/doctrine_anchors.tsv"
  local triggers_tsv="${SCRIPT_DIR}/anchor_triggers.tsv"
  if [[ -n "${FIXTURE_DIR:-}" && -f "${FIXTURE_DIR}/doctrine_anchors.tsv" ]]; then
    anchors_tsv="${FIXTURE_DIR}/doctrine_anchors.tsv"
  fi
  if [[ -n "${FIXTURE_DIR:-}" && -f "${FIXTURE_DIR}/anchor_triggers.tsv" ]]; then
    triggers_tsv="${FIXTURE_DIR}/anchor_triggers.tsv"
  fi
  local files
  files="$(changed_files 2>/dev/null || true)"
  "$PYTHON_BIN" - "$anchors_tsv" "$triggers_tsv" "$files" <<'PY'
import csv
import fnmatch
import sys
from pathlib import PurePosixPath

anchors_tsv, triggers_tsv, files_blob = sys.argv[1], sys.argv[2], sys.argv[3]
files = [ln.strip().replace("\\", "/") for ln in files_blob.splitlines() if ln.strip()]

def glob_match(path: str, pattern: str) -> bool:
    g = pattern.strip().replace("\\", "/")
    if not g:
        return False
    p = PurePosixPath(path)
    if p.match(g):
        return True
    if "**" in g:
        prefix = g.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, g.replace("**", "*"))

domains = set()
try:
    with open(triggers_tsv, encoding="utf-8", newline="") as fh:
        for row in csv.DictReader(fh, delimiter="\t"):
            glob_pat = (row.get("glob") or "").strip()
            if not glob_pat:
                continue
            if any(glob_match(path, glob_pat) for path in files):
                for d in (row.get("trigger_domains") or "").split(","):
                    d = d.strip()
                    if d:
                        domains.add(d)
except FileNotFoundError:
    pass

ids = []
try:
    with open(anchors_tsv, encoding="utf-8", newline="") as fh:
        for row in csv.DictReader(fh, delimiter="\t"):
            aid = (row.get("anchor_id") or "").strip()
            if not aid:
                continue
            row_domains = {d.strip() for d in (row.get("trigger_domains") or "").split(",") if d.strip()}
            if domains & row_domains:
                ids.append(aid)
except FileNotFoundError:
    pass
print(",".join(sorted(ids)))
PY
}

emit_required_anchors_line() {
  local ids
  ids="$(required_anchor_ids_for_changed_files || true)"
  if [[ -n "$ids" ]]; then
    printf 'REQUIRED-ANCHORS: %s\n' "$ids"
  else
    printf 'REQUIRED-ANCHORS: none\n'
  fi
}

emit_verdict() {
  local kind="$1"
  local detail="${2:-}"
  case "$kind" in
    clearable)
      VERDICT="CLEARANCE-VERDICT: ORCHESTRATOR-CLEARABLE"
      ;;
    reserve)
      VERDICT="CLEARANCE-VERDICT: DA-RESERVE(${detail})"
      ;;
    fail)
      VERDICT="CLEARANCE-VERDICT: FAIL(${detail})"
      ;;
    *)
      kind="reserve"
      detail="harness-error"
      VERDICT="CLEARANCE-VERDICT: DA-RESERVE(harness-error)"
      ;;
  esac
  printf '%s\n' "$VERDICT"
  if [[ "$kind" == "reserve" ]]; then
    emit_treeverify_profile_line
    emit_required_anchors_line
  fi
}

glob_matches_any() {
  local file="$1"
  local globs="$2"
  "$PYTHON_BIN" - "$file" "$globs" <<'PY'
import fnmatch
import sys
from pathlib import PurePosixPath

path = sys.argv[1].replace("\\", "/")
globs = sys.argv[2].split("|")
p = PurePosixPath(path)
for g in globs:
    g = g.strip().replace("\\", "/")
    if not g:
        continue
    if p.match(g):
        sys.exit(0)
    if "**" in g:
        prefix = g.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            sys.exit(0)
    if fnmatch.fnmatch(path, g.replace("**", "*")):
        sys.exit(0)
sys.exit(1)
PY
}

load_tsv() {
  local path="$1"
  local min_cols="$2"
  if [[ ! -f "$path" ]]; then
    return 1
  fi
  "$PYTHON_BIN" - "$path" "$min_cols" <<'PY'
import sys

path, min_cols = sys.argv[1], int(sys.argv[2])
rows = []
with open(path, encoding="utf-8", newline="") as f:
    for i, line in enumerate(f, 1):
        line = line.rstrip("\n\r")
        if not line.strip():
            continue
        if i == 1 and "\t" not in line and "class_id" not in line and "rung" not in line and "verdict" not in line:
            continue
        parts = line.split("\t")
        if len(parts) < min_cols:
            print(f"MALFORMED:{path}:{i}:{len(parts)}", file=sys.stderr)
            sys.exit(1)
        rows.append(parts)
print(len(rows))
PY
}

parse_args() {
  if [[ $# -eq 0 ]]; then
    usage
  fi
  case "${1:-}" in
    --selftest)
      FIXTURE_MODE="selftest"
      ;;
    --fixture)
      [[ $# -ge 2 ]] || usage
      FIXTURE_MODE="fixture"
      FIXTURE_DIR="${FIXTURES_ROOT}/${2}"
      ;;
    --pr)
      [[ $# -ge 2 ]] || usage
      PR_NUMBER="$2"
      ;;
    --range)
      [[ $# -ge 2 ]] || usage
      RANGE_SPEC="$2"
      ;;
    -h|--help)
      usage
      ;;
    *)
      if [[ "$1" =~ ^[0-9]+$ ]]; then
        PR_NUMBER="$1"
      else
        usage
      fi
      ;;
  esac
}

resolve_paths() {
  local classes="${SCRIPT_DIR}/precedented_classes.tsv"
  local binding="${SCRIPT_DIR}/binding_conditions.tsv"
  local ledger="${SCRIPT_DIR}/clearance_ledger.tsv"
  local triage="${SCRIPT_DIR}/triage_log.tsv"
  local predicates="${SCRIPT_DIR}/class_predicates.tsv"
  if [[ -n "$FIXTURE_DIR" && -d "$FIXTURE_DIR" ]]; then
    [[ -f "${FIXTURE_DIR}/precedented_classes.tsv" ]] && classes="${FIXTURE_DIR}/precedented_classes.tsv"
    [[ -f "${FIXTURE_DIR}/binding_conditions.tsv" ]] && binding="${FIXTURE_DIR}/binding_conditions.tsv"
    [[ -f "${FIXTURE_DIR}/triage_log.tsv" ]] && triage="${FIXTURE_DIR}/triage_log.tsv"
    [[ -f "${FIXTURE_DIR}/class_predicates.tsv" ]] && predicates="${FIXTURE_DIR}/class_predicates.tsv"
  fi
  printf '%s\n%s\n%s\n%s\n%s' "$classes" "$binding" "$ledger" "$triage" "$predicates"
}

class_predicates_path() {
  local predicates="${SCRIPT_DIR}/class_predicates.tsv"
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/class_predicates.tsv" ]]; then
    predicates="${FIXTURE_DIR}/class_predicates.tsv"
  fi
  printf '%s' "$predicates"
}

read_fixture_file() {
  local name="$1"
  local default="${2:-}"
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/${name}" ]]; then
    cat "${FIXTURE_DIR}/${name}"
    return 0
  fi
  printf '%s' "$default"
}

mark_requested_target() {
  if [[ -n "$PR_NUMBER" || -n "$RANGE_SPEC" ]]; then
    REQUESTED_TARGET=1
    return 0
  fi
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/target_mode.txt" ]]; then
    REQUESTED_TARGET=1
    return 0
  fi
  return 1
}

resolve_pr_changed_files() {
  local files repo
  if ! command -v gh >/dev/null 2>&1; then
    return 1
  fi
  files="$(gh pr diff "$PR_NUMBER" --name-only 2>/dev/null || true)"
  if [[ -n "$files" ]]; then
    CHANGED_FILES_LIST="$files"
    return 0
  fi
  repo="${GITHUB_REPOSITORY:-}"
  if [[ -z "$repo" ]]; then
    repo="$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || true)"
  fi
  if [[ -n "$repo" ]]; then
    files="$(gh api "repos/${repo}/pulls/${PR_NUMBER}/files" --paginate \
      --jq '.[].filename' 2>/dev/null || true)"
    if [[ -n "$files" ]]; then
      CHANGED_FILES_LIST="$files"
      return 0
    fi
  fi
  return 1
}

resolve_changed_files_once() {
  if [[ "$CHANGED_FILES_RESOLVED" -eq 1 ]]; then
    return 0
  fi
  CHANGED_FILES_RESOLVED=1
  mark_requested_target || true

  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/changed_files.txt" ]]; then
    CHANGED_FILES_LIST="$(sed '/^[[:space:]]*$/d' "${FIXTURE_DIR}/changed_files.txt" || true)"
    return 0
  fi
  if [[ -n "$RANGE_SPEC" ]]; then
    local base="${RANGE_SPEC%%..*}"
    local head="${RANGE_SPEC##*..}"
    CHANGED_FILES_LIST="$(git -C "$REPO_ROOT" diff --name-only "${base}" "${head}" 2>/dev/null || true)"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]]; then
    if resolve_pr_changed_files; then
      return 0
    fi
    PR_RESOLUTION_FAILED=1
    return 1
  fi
  return 1
}

changed_files_nonempty() {
  resolve_changed_files_once || true
  printf '%s\n' "$CHANGED_FILES_LIST" | sed '/^[[:space:]]*$/d' | grep -q .
}

changed_files() {
  resolve_changed_files_once || true
  printf '%s\n' "$CHANGED_FILES_LIST"
}

print_pr_resolution_remedy() {
  printf 'Unable to resolve PR diff locally; pass --range <base>..<head> or run in GHA with PR metadata.\n' >&2
}

pr_body_text() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/pr_body.txt" ]]; then
    cat "${FIXTURE_DIR}/pr_body.txt"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]] && command -v gh >/dev/null 2>&1; then
    gh pr view "$PR_NUMBER" --json body -q .body 2>/dev/null || true
    return 0
  fi
  printf ''
}

check_self_application() {
  local file
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    local gate
    for gate in "${GATE_WIRING_PATHS[@]}"; do
      if [[ "$file" == "$gate" ]]; then
        return 0
      fi
    done
    if [[ "$file" == scripts/ci/fixtures/clearance/* ]] ||
       [[ "$file" == scripts/ci/fixtures/doc_budget/* ]] ||
       [[ "$file" == scripts/ci/fixtures/agents_stub/* ]]; then
      return 0
    fi
  done < <(changed_files 2>/dev/null || true)
  return 1
}

detect_classes() {
  local classes_tsv="$1"
  local predicates_tsv
  predicates_tsv="$(class_predicates_path)"
  local files
  files="$(changed_files 2>/dev/null || true)"
  "$PYTHON_BIN" - "$classes_tsv" "$predicates_tsv" "$files" <<'PY'
import csv
import fnmatch
import sys
from pathlib import PurePosixPath

classes_tsv, predicates_tsv, files_blob = sys.argv[1], sys.argv[2], sys.argv[3]
files = [f.strip().replace("\\", "/") for f in files_blob.splitlines() if f.strip()]

def glob_match(path: str, pattern: str) -> bool:
    p = PurePosixPath(path)
    if p.match(pattern):
        return True
    if "**" in pattern:
        prefix = pattern.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, pattern.replace("**", "*"))

def any_match(path_list, globs_blob):
    globs = [g.strip() for g in globs_blob.split("|") if g.strip()]
    if not globs:
        return False
    return any(any(glob_match(path, g) for g in globs) for path in path_list)

def all_match(path_list, globs_blob):
    globs = [g.strip() for g in globs_blob.split("|") if g.strip()]
    if not globs or not path_list:
        return False
    return all(any(glob_match(path, g) for g in globs) for path in path_list)

# Data-driven class predicates (HU-CLEARANCE-DSL-0): match_any + detect_mode.
predicates = {}
try:
    with open(predicates_tsv, encoding="utf-8", newline="") as fh:
        for row in csv.reader(fh, delimiter="\t"):
            if not row or row[0] == "class_id":
                continue
            if len(row) < 6:
                continue
            cid, match_any, scope, forbidden, mode, prio = row[:6]
            predicates[cid] = {
                "match_any": match_any,
                "scope": scope,
                "forbidden": forbidden,
                "mode": mode.strip(),
                "priority": int(prio) if str(prio).strip().isdigit() else 0,
            }
except FileNotFoundError:
    predicates = {}

def dsl_detect(cid, pred):
    if not any_match(files, pred["match_any"]):
        return False
    mode = pred["mode"]
    if mode == "all_in_scope":
        return all_match(files, pred["scope"])
    if mode == "any_then_envelope":
        return any_match(files, pred["scope"])
    return False

# Shape signal (match_any only): blocks workshop-candidate collision even when
# all_in_scope detect fails (out-of-envelope files → unclassified, not workshop).
dsl_shape_ids = {
    cid for cid, pred in predicates.items() if any_match(files, pred["match_any"])
}
dsl_hits = []
for cid, pred in predicates.items():
    if dsl_detect(cid, pred):
        dsl_hits.append((pred["priority"], cid))
dsl_hits.sort(reverse=True)
dsl_primary = dsl_hits[0][1] if dsl_hits else ""
dsl_matched_ids = {cid for _, cid in dsl_hits}

primary = ""
for f in files:
    if "tp_fleet_movement_0.rs" in f:
        primary = "tp-fleet-movement-rung"
        break
    if "tp_palma_reach_0.rs" in f:
        primary = "tp-palma-reach-rung"
        break
    if "tp_fronts_authoring_0.rs" in f:
        primary = "tp-fronts-authoring-rung"
        break
    if "tp_diplomacy_flow_0.rs" in f:
        primary = "tp-diplomacy-flow-rung"
        break
    if "suspended_demo" in f:
        primary = "tp-suspended-demo"
        break
if not primary and dsl_primary:
    primary = dsl_primary

rows = []
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id", "rung", "verdict"):
            continue
        rows.append(row)

class_ids = set()
has_corpus_sweep_result = any(
    f.startswith("docs/tests/cc_sweep_") and f.endswith("_results.md") for f in files
)
has_corpus_sweep_inventory = any(
    f == "scripts/ci/test_inventory.tsv"
    for f in files
)
has_corpus_sweep_test = any(
    f.startswith("crates/") and "/tests/" in f and f.endswith(".rs") for f in files
)
has_module_marker_sweep_result = any(
    f.startswith("docs/tests/cc_sweep_") and f.endswith("_module_markers_results.md")
    for f in files
)
has_module_marker_sweep_inventory = "scripts/ci/test_inventory.tsv" in files
has_crate_src_or_tests_edit = any(
    f.startswith("crates/") and ("/src/" in f or "/tests/" in f) for f in files
)
has_corpus_baseline_result = "docs/tests/cc_baseline_0_results.md" in files
has_docs_ladder_shape = any(
    f.startswith("docs/design_") and f.endswith(".md")
    or f == "docs/orchestrator_orientation.md"
    or (
        f.startswith("docs/tests/")
        and f.endswith("_readiness_0_results.md")
    )
    for f in files
)
has_tp_workshop_candidate_shape = any(
    (
        f.startswith("crates/simthing-workshop/src/tp_")
        and f.endswith(".rs")
    )
    or (
        f.startswith("crates/simthing-workshop/tests/tp_")
        and f.endswith(".rs")
    )
    for f in files
)
for row in rows:
    if len(row) < 6:
        continue
    class_id, scope_globs, _env, _reqs, status, _blocker = row[:6]
    if status == "retired":
        continue
    # DSL classes: match only via class_predicates.tsv engine (not legacy scope loop).
    if class_id in predicates:
        if class_id not in dsl_matched_ids:
            continue
        if primary and class_id != primary:
            continue
        class_ids.add(class_id)
        continue
    if class_id == "corpus-sweep" and not (
        has_corpus_sweep_result and has_corpus_sweep_inventory and has_corpus_sweep_test
    ):
        continue
    if class_id == "corpus-module-marker-sweep" and not (
        has_module_marker_sweep_result
        and has_module_marker_sweep_inventory
        and not has_crate_src_or_tests_edit
    ):
        continue
    if class_id == "corpus-baseline" and not has_corpus_baseline_result:
        continue
    if class_id == "docs-ladder-pointer-correction":
        if not has_docs_ladder_shape or not files:
            continue
        globs = [g.strip() for g in scope_globs.split("|") if g.strip()]
        if not all(any(glob_match(path, g) for g in globs) for path in files):
            continue
    if class_id == "tp-workshop-candidate-proof":
        if not has_tp_workshop_candidate_shape:
            continue
        # Do not collide with data-driven TP admitted class shapes.
        if dsl_shape_ids:
            continue
    if primary and class_id != primary:
        continue
    for path in files:
        for glob_pat in scope_globs.split("|"):
            glob_pat = glob_pat.strip()
            if glob_pat and glob_match(path, glob_pat):
                class_ids.add(class_id)
                break

for cid in sorted(class_ids):
    print(cid)
PY
}

# Generic engine: scope_globs / forbidden_globs from class_predicates.tsv.
# Returns 0 when envelope ok; on violation emits reserve and returns 1.
# No-op (return 0) when class has no predicate row.
check_class_predicate_envelope() {
  local class_id="$1"
  local predicates_tsv
  predicates_tsv="$(class_predicates_path)"
  [[ -f "$predicates_tsv" ]] || return 0
  local files
  files="$(changed_files 2>/dev/null || true)"
  local result
  result="$("$PYTHON_BIN" - "$predicates_tsv" "$class_id" "$files" <<'PY'
import csv
import fnmatch
import sys
from pathlib import PurePosixPath

predicates_tsv, class_id, files_blob = sys.argv[1], sys.argv[2], sys.argv[3]
files = [f.strip().replace("\\", "/") for f in files_blob.splitlines() if f.strip()]

def glob_match(path: str, pattern: str) -> bool:
    p = PurePosixPath(path)
    if p.match(pattern):
        return True
    if "**" in pattern:
        prefix = pattern.split("**", 1)[0]
        if prefix and path.startswith(prefix):
            return True
    return fnmatch.fnmatch(path, pattern.replace("**", "*"))

def any_glob(path, globs):
    return any(glob_match(path, g) for g in globs)

pred = None
with open(predicates_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] == "class_id":
            continue
        if row[0] == class_id and len(row) >= 6:
            pred = {
                "scope": [g.strip() for g in row[2].split("|") if g.strip()],
                "forbidden": [g.strip() for g in row[3].split("|") if g.strip()],
                "mode": row[4].strip(),
            }
            break
if pred is None:
    print("ok")
    sys.exit(0)

for path in files:
    if pred["forbidden"] and any_glob(path, pred["forbidden"]):
        print("envelope")
        sys.exit(0)
    if pred["mode"] == "any_then_envelope" and pred["scope"]:
        if not any_glob(path, pred["scope"]):
            print("envelope")
            sys.exit(0)
print("ok")
PY
)"
  if [[ "$result" == "envelope" ]]; then
    emit_verdict reserve "class-envelope-violation"
    return 1
  fi
  return 0
}

class_for_rung() {
  local class_id="$1"
  case "$class_id" in
    tp-fleet-movement-rung) printf 'TP-FLEET-MOVEMENT-0' ;;
    tp-palma-reach-rung) printf 'TP-PALMA-REACH-0' ;;
    tp-fronts-authoring-rung) printf 'TP-FRONTS-AUTHORING-0' ;;
    tp-diplomacy-flow-rung) printf 'TP-DIPLOMACY-FLOW-0' ;;
    tp-workshop-candidate-proof) printf 'TP-WORKSHOP-CANDIDATE-CLASS-0' ;;
    tp-admitted-clause-api-composition) printf 'TP-ADMITTED-CLAUSE-API-CLASS-0' ;;
    tp-studio-clause-picker) printf 'TP-STUDIO-CLAUSE-PICKER-CLASS-0' ;;
    tp-workshop-scenario-rung)
      local files
      files="$(changed_files 2>/dev/null || true)"
      if echo "$files" | grep -q 'tp_fleet_movement'; then printf 'TP-FLEET-MOVEMENT-0'
      elif echo "$files" | grep -q 'tp_palma_reach'; then printf 'TP-PALMA-REACH-0'
      elif echo "$files" | grep -q 'tp_fronts_authoring'; then printf 'TP-FRONTS-AUTHORING-0'
      elif echo "$files" | grep -q 'tp_diplomacy_flow'; then printf 'TP-DIPLOMACY-FLOW-0'
      else printf 'TP-WORKSHOP-SCENARIO-RUNG'
      fi
      ;;
    *) printf "$class_id" ;;
  esac
}

check_binding_conditions() {
  local binding_tsv="$1"
  local rung="$2"
  "$PYTHON_BIN" - "$binding_tsv" "$rung" <<'PY'
import csv
import sys

binding_tsv, rung = sys.argv[1], sys.argv[2]
with open(binding_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("rung", "verdict"):
            continue
        if len(row) < 4:
            continue
        cond_rung, _cond, _set_by, status = row[:4]
        if cond_rung == rung and status == "open":
            sys.exit(0)
sys.exit(1)
PY
}

class_status() {
  local classes_tsv="$1"
  local class_id="$2"
  "$PYTHON_BIN" - "$classes_tsv" "$class_id" <<'PY'
import csv
import sys

classes_tsv, class_id = sys.argv[1], sys.argv[2]
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id",):
            continue
        if row[0] == class_id and len(row) >= 5:
            print(row[4])
            sys.exit(0)
sys.exit(0)
PY
}

class_requirements() {
  local classes_tsv="$1"
  local class_id="$2"
  "$PYTHON_BIN" - "$classes_tsv" "$class_id" <<'PY'
import csv
import sys

classes_tsv, class_id = sys.argv[1], sys.argv[2]
with open(classes_tsv, encoding="utf-8", newline="") as fh:
    for row in csv.reader(fh, delimiter="\t"):
        if not row or row[0] in ("class_id",):
            continue
        if row[0] == class_id and len(row) >= 4:
            print(row[3])
            sys.exit(0)
sys.exit(0)
PY
}

check_workshop_only() {
  local file
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
      # workshop candidate + results/inventory; design/orientation stamps for TP ladder
      crates/simthing-workshop/*|docs/tests/*|scripts/ci/test_inventory.tsv|docs/design_*|docs/orchestrator_orientation.md)
        ;;
      *)
        return 1
        ;;
    esac
  done < <(changed_files 2>/dev/null || true)
  return 0
}

check_no_engine_crate() {
  local file prefix
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    for prefix in "${ENGINE_CRATE_PREFIXES[@]}"; do
      if [[ "$file" == "$prefix"* ]]; then
        return 1
      fi
    done
  done < <(changed_files 2>/dev/null || true)
  return 0
}

check_no_engine_src() {
  check_no_engine_crate
}

inventory_removed_rows() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/inventory_removed_rows.tsv" ]]; then
    cat "${FIXTURE_DIR}/inventory_removed_rows.tsv"
    return 0
  fi
  if [[ -n "$RANGE_SPEC" ]]; then
    local base="${RANGE_SPEC%%..*}"
    local head="${RANGE_SPEC##*..}"
    git -C "$REPO_ROOT" diff --unified=0 "$base" "$head" -- scripts/ci/test_inventory.tsv 2>/dev/null \
      | sed -n '/^-[^-]/s/^-//p'
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]] && command -v gh >/dev/null 2>&1; then
    gh pr diff "$PR_NUMBER" --patch 2>/dev/null \
      | awk '
          /^diff --git a\/scripts\/ci\/test_inventory.tsv b\/scripts\/ci\/test_inventory.tsv$/ { in_file=1; next }
          /^diff --git / { in_file=0 }
          in_file && /^-[^-]/ { sub(/^-/, ""); print }
        '
    return 0
  fi
  return 0
}

check_module_marker_inventory_deletions() {
  local removed
  removed="$(inventory_removed_rows | sed '/^[[:space:]]*$/d' || true)"
  if [[ -z "$removed" ]]; then
    return 1
  fi
  MODULE_MARKER_REMOVED_ROWS="$removed" "$PYTHON_BIN" - <<'PY'
import csv
import os
import sys

rows = [row for row in csv.reader(os.environ.get("MODULE_MARKER_REMOVED_ROWS", "").splitlines(), delimiter="\t") if row]
if not rows:
    sys.exit(1)

for row in rows:
    if row[0] == "crate":
        sys.exit(1)
    if len(row) < 10:
        sys.exit(1)
    crate, file_path, test_name, kind, class_id, _boundary, verdict, note, promotion_target = row[:9]
    if not crate or not file_path.startswith(f"crates/{crate}/src/"):
        sys.exit(1)
    if not test_name.startswith("cfg_test_mod::"):
        sys.exit(1)
    if kind != "unit" or class_id != "deletion-candidate" or verdict != "AUDIT":
        sys.exit(1)
    text = f"{note}\t{promotion_target}".lower()
    if not (
        "module-marker" in text
        or "module marker" in text
        or "mod marker" in text
    ):
        sys.exit(1)
    if "ledger-only" not in text:
        sys.exit(1)
sys.exit(0)
PY
}

check_required_pr_body_fields() {
  local body="$1"
  local missing=()
  if ! echo "$body" | grep -qiE 'tested_code_sha:[[:space:]]*[0-9a-f]{8,}'; then
    missing+=("tested_code_sha")
  fi
  if ! echo "$body" | grep -qiE 'coverage_basis:[[:space:]]*PASS'; then
    missing+=("coverage_basis")
  fi
  if [[ "${#missing[@]}" -eq 0 ]]; then
    return 0
  fi
  local joined="" m token
  for m in "${missing[@]}"; do
    token="${m//_/-}"
    if [[ -z "$joined" ]]; then
      joined="$token"
    else
      joined="${joined},${token}"
    fi
  done
  emit_verdict fail "missing-${joined}: add tested_code_sha and coverage_basis"
  return 1
}

# Explicit novelty claim only — never a generic unmatched-diff fallback.
# When claimed, novelty overrides matched-class clearable routing.
check_explicit_novelty_claim() {
  local body="$1"
  echo "$body" | grep -qiE 'novelty_claim:[[:space:]]*YES'
}

check_explicit_novelty_basis() {
  local body="$1"
  echo "$body" | grep -qiE 'novelty_basis:[[:space:]]*[^[:space:]].+'
}

# Admitted-scope router gap (CLEARANCE-ADMITTED-SCOPE-GAP-0): no class match,
# but PR claims proof-present work inside a prior Owner/DA admission envelope.
check_admitted_envelope_claim() {
  local body="$1"
  echo "$body" | grep -qiE 'admitted_envelope:[[:space:]]*YES'
}

# Returns 0 when all required admitted-scope-gap body fields are present.
# On missing fields emits FAIL(missing-admitted-scope-router-gap-fields: ...) and returns 1.
check_admitted_scope_gap_fields() {
  local body="$1"
  local missing=()

  if ! echo "$body" | grep -qiE 'admitting_pr:[[:space:]]*#?[0-9]+' \
    && ! echo "$body" | grep -qiE 'admitting_rung:[[:space:]]*[^[:space:]].+'; then
    missing+=("admitting_pr|admitting_rung")
  fi
  if ! echo "$body" | grep -qiE 'admitted_surfaces:[[:space:]]*[^[:space:]].+'; then
    missing+=("admitted_surfaces")
  fi
  if ! echo "$body" | grep -qiE 'forbidden_surfaces:[[:space:]]*[^[:space:]].+'; then
    missing+=("forbidden_surfaces")
  fi
  if ! echo "$body" | grep -qiE 'tested_code_sha:[[:space:]]*[0-9a-f]{8,}'; then
    missing+=("tested_code_sha")
  fi
  if ! echo "$body" | grep -qiE 'coverage_basis:[[:space:]]*PASS'; then
    missing+=("coverage_basis")
  fi
  if ! echo "$body" | grep -qiE 'ci_green:[[:space:]]*PASS'; then
    missing+=("ci_green")
  fi

  if [[ "${#missing[@]}" -gt 0 ]]; then
    local joined="" m
    for m in "${missing[@]}"; do
      if [[ -z "$joined" ]]; then
        joined="$m"
      else
        joined="${joined}, ${m}"
      fi
    done
    emit_verdict fail "missing-admitted-scope-router-gap-fields: ${joined}"
    return 1
  fi
  return 0
}

# Textual forbidden-surface checks for admitted-scope claims.
# Does not clear or bypass forbidden surfaces; never returns clearable.
# Returns 0 when no forbidden surface appears in the changed-file set.
check_admitted_scope_forbidden_surfaces() {
  local body="$1"
  local forbidden_line file
  forbidden_line="$(echo "$body" | grep -iE '^[[:space:]]*forbidden_surfaces:[[:space:]]*' | head -n 1 || true)"
  forbidden_line="$(printf '%s' "$forbidden_line" | sed -E 's/^[[:space:]]*forbidden_surfaces:[[:space:]]*//I')"

  local check_engine=0
  local check_gamemode_rf=0
  local check_closeout=0
  local check_picker=0
  local fl_lower
  fl_lower="$(printf '%s' "$forbidden_line" | tr '[:upper:]' '[:lower:]')"

  # Keyword hints from the body claim.
  if echo "$fl_lower" | grep -qE 'runtime|gpu|kernel|engine'; then
    check_engine=1
  fi
  if echo "$fl_lower" | grep -qE 'gamemode|game.?mode|\brf\b|live[-_ ]?run'; then
    check_gamemode_rf=1
  fi
  if echo "$fl_lower" | grep -qE 'closeout'; then
    check_closeout=1
  fi
  if echo "$fl_lower" | grep -qE 'picker|ui picker|file.?dialog'; then
    check_picker=1
  fi

  # Always apply high-risk denylist for admitted-envelope claims so forbidden
  # runtime/closeout/GameMode surfaces cannot silently route as router-gap only.
  check_engine=1
  check_gamemode_rf=1
  check_closeout=1

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    file="${file//\\//}"
    if [[ "$check_engine" -eq 1 ]]; then
      case "$file" in
        crates/simthing-kernel/*|crates/simthing-sim/src/*|crates/simthing-gpu/*|crates/simthing-driver/*|crates/simthing-spec/src/*)
          emit_verdict reserve "class-envelope-violation"
          return 1
          ;;
      esac
    fi
    if [[ "$check_gamemode_rf" -eq 1 ]]; then
      case "$file" in
        *[Gg]ame[Mm]ode*|*gamemode*|*live_run*|*live-run*|*LiveRun*|*rf_attach*|*RFAttach*)
          emit_verdict reserve "class-envelope-violation"
          return 1
          ;;
      esac
    fi
    if [[ "$check_closeout" -eq 1 ]]; then
      case "$file" in
        *closeout*|*track_closeout*)
          emit_verdict reserve "class-envelope-violation"
          return 1
          ;;
      esac
    fi
    if [[ "$check_picker" -eq 1 ]]; then
      case "$file" in
        *picker*|*FileDialog*|*file_dialog*)
          emit_verdict reserve "class-envelope-violation"
          return 1
          ;;
      esac
    fi
  done < <(changed_files 2>/dev/null || true)
  return 0
}

check_recorded_gpu_proof() {
  local body="$1"
  if echo "$body" | grep -q 'DOCTRINE-TESTS-VERDICT:[[:space:]]*PASS'; then
    return 0
  fi
  if echo "$body" | grep -qiE 'DOCTRINE-TESTS-VERDICT'; then
    emit_verdict fail "missing-gpu-proof: add citable DOCTRINE-TESTS-VERDICT bound to tested_code_sha"
    return 1
  fi
  emit_verdict fail "missing-gpu-proof: add citable DOCTRINE-TESTS-VERDICT bound to tested_code_sha"
  return 1
}

inspect_delta_scan_ids() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/inspect_delta_scan_ids.txt" ]]; then
    sed '/^[[:space:]]*$/d' "${FIXTURE_DIR}/inspect_delta_scan_ids.txt"
    return 0
  fi

  local base_sha="" head_sha=""
  if [[ -n "$RANGE_SPEC" ]]; then
    base_sha="${RANGE_SPEC%%..*}"
    head_sha="${RANGE_SPEC##*..}"
  elif [[ -n "$PR_NUMBER" ]] && command -v gh >/dev/null 2>&1; then
    local pr_json
    pr_json="$(gh pr view "$PR_NUMBER" --json baseRefOid,headRefOid 2>/dev/null || true)"
    if [[ -n "$pr_json" ]]; then
      base_sha="$("$PYTHON_BIN" - <<'PY' "$pr_json"
import json, sys
data = json.loads(sys.argv[1])
print(data.get("baseRefOid", ""))
PY
)"
      head_sha="$("$PYTHON_BIN" - <<'PY' "$pr_json"
import json, sys
data = json.loads(sys.argv[1])
print(data.get("headRefOid", ""))
PY
)"
    fi
  fi

  if [[ -z "$base_sha" || -z "$head_sha" ]]; then
    return 0
  fi

  local scan_out
  scan_out="$(DOCTRINE_SCAN_SKIP_DRIFT=1 bash "${SCRIPT_DIR}/doctrine_scan.sh" --pr-delta "$base_sha" "$head_sha" 2>/dev/null || true)"
  printf '%s\n' "$scan_out" | awk '$2 == "INSPECT" && $3 ~ /^[1-9]/ { print $1 }' | sed '/^$/d'
}

# Evidence-tail paths after a triage SHA (Immutability Law #1169 / OH-IMMUTABLE-EVIDENCE-0):
# docs/tests/**, scripts/ci/triage_log.tsv, *_results.md — not code.
is_evidence_tail_path() {
  local path="$1"
  path="${path//\\//}"
  [[ "$path" == docs/tests/* ]] && return 0
  [[ "$path" == "scripts/ci/triage_log.tsv" ]] && return 0
  [[ "$path" == *_results.md ]] && return 0
  return 1
}

# Returns 0 when triage_sha..head touches only evidence-tail paths (or empty).
# Fixture mode: optional evidence_tail_paths.txt / code_delta_paths.txt override git.
triage_delta_is_evidence_tail_only() {
  local triage_sha="$1"
  local head_sha="$2"
  if [[ -n "$FIXTURE_DIR" ]]; then
    if [[ -f "${FIXTURE_DIR}/code_delta_paths.txt" ]]; then
      return 1
    fi
    if [[ -f "${FIXTURE_DIR}/evidence_tail_paths.txt" ]]; then
      local p
      while IFS= read -r p; do
        p="$(printf '%s' "$p" | tr -d '\r' | sed '/^[[:space:]]*$/d')"
        [[ -z "$p" ]] && continue
        if ! is_evidence_tail_path "$p"; then
          return 1
        fi
      done <"${FIXTURE_DIR}/evidence_tail_paths.txt"
      return 0
    fi
    # Fixture without override: treat non-hex / fixture commits as unbound (valid).
    if [[ ! "$triage_sha" =~ ^[0-9a-fA-F]{7,40}$ ]]; then
      return 0
    fi
  fi
  if [[ -z "$triage_sha" || -z "$head_sha" ]]; then
    return 0
  fi
  if [[ "$triage_sha" == "$head_sha" ]]; then
    return 0
  fi
  # Require ancestor relationship when both look like SHAs.
  if [[ "$triage_sha" =~ ^[0-9a-fA-F]{7,40}$ && "$head_sha" =~ ^[0-9a-fA-F]{7,40}$ ]]; then
    if ! git merge-base --is-ancestor "$triage_sha" "$head_sha" 2>/dev/null; then
      return 1
    fi
    local paths
    paths="$(git diff --name-only "${triage_sha}..${head_sha}" 2>/dev/null || true)"
    local p
    while IFS= read -r p; do
      [[ -z "$p" ]] && continue
      if ! is_evidence_tail_path "$p"; then
        return 1
      fi
    done <<<"$paths"
    return 0
  fi
  # Non-SHA fixture commits (e.g. fixture01) remain valid without git ancestry.
  return 0
}

resolve_pr_head_sha() {
  if [[ -n "$FIXTURE_DIR" && -f "${FIXTURE_DIR}/head_sha.txt" ]]; then
    tr -d '\r' <"${FIXTURE_DIR}/head_sha.txt" | head -n 1
    return 0
  fi
  if [[ -n "$RANGE_SPEC" && "$RANGE_SPEC" == *..* ]]; then
    printf '%s\n' "${RANGE_SPEC##*..}"
    return 0
  fi
  if [[ -n "$PR_NUMBER" ]] && command -v gh >/dev/null 2>&1; then
    local pr_json
    pr_json="$(gh pr view "$PR_NUMBER" --json headRefOid 2>/dev/null || true)"
    if [[ -n "$pr_json" ]]; then
      "$PYTHON_BIN" - <<'PY' "$pr_json"
import json, sys
print(json.loads(sys.argv[1]).get("headRefOid", ""))
PY
      return 0
    fi
  fi
  git rev-parse HEAD 2>/dev/null || true
}

# Emit covered scan_ids whose triage commit is head, an ancestor with evidence-tail-only
# delta, or a non-SHA fixture token. Code deltas after triage SHA invalidate coverage.
triage_covered_scan_ids() {
  local triage_tsv="$1"
  local head_sha
  head_sha="$(resolve_pr_head_sha | tr -d '\r' | head -n 1)"
  TRIAGE_TSV_PATH="$triage_tsv" HEAD_SHA="$head_sha" FIXTURE_DIR_ENV="${FIXTURE_DIR:-}" \
    "$PYTHON_BIN" - <<'PY'
import os
import re
import subprocess
import sys

path = os.environ.get("TRIAGE_TSV_PATH", "")
head_sha = (os.environ.get("HEAD_SHA") or "").strip()
fixture_dir = (os.environ.get("FIXTURE_DIR_ENV") or "").strip()
PLACEHOLDER_RE = re.compile(
    r"^(?:tbd|todo|n/?a|none|pending|fixme|wip|placeholder|\.{1,3}|-+)$",
    re.IGNORECASE,
)
VALID = {"delete", "green", "escalate"}
SHA_RE = re.compile(r"^[0-9a-fA-F]{7,40}$")

def reason_valid(text: str) -> bool:
    text = (text or "").strip()
    return bool(text) and not PLACEHOLDER_RE.match(text)

def is_evidence_tail(p: str) -> bool:
    p = p.replace("\\", "/")
    if p.startswith("docs/tests/"):
        return True
    if p == "scripts/ci/triage_log.tsv":
        return True
    if p.endswith("_results.md"):
        return True
    return False

def evidence_tail_only(triage_sha: str) -> bool:
    if fixture_dir:
        code = os.path.join(fixture_dir, "code_delta_paths.txt")
        if os.path.isfile(code):
            return False
        evid = os.path.join(fixture_dir, "evidence_tail_paths.txt")
        if os.path.isfile(evid):
            with open(evid, encoding="utf-8") as fh:
                for line in fh:
                    p = line.strip()
                    if p and not is_evidence_tail(p):
                        return False
            return True
        if not SHA_RE.match(triage_sha):
            return True
    if not triage_sha or not head_sha:
        return True
    if triage_sha == head_sha:
        return True
    if SHA_RE.match(triage_sha) and SHA_RE.match(head_sha):
        r = subprocess.run(
            ["git", "merge-base", "--is-ancestor", triage_sha, head_sha],
            capture_output=True,
        )
        if r.returncode != 0:
            return False
        r2 = subprocess.run(
            ["git", "diff", "--name-only", f"{triage_sha}..{head_sha}"],
            capture_output=True,
            text=True,
        )
        for p in (r2.stdout or "").splitlines():
            p = p.strip()
            if p and not is_evidence_tail(p):
                return False
        return True
    return True

if not path or not os.path.isfile(path):
    sys.exit(0)

covered = set()
with open(path, encoding="utf-8-sig", newline="") as fh:
    for i, line in enumerate(fh, 1):
        line = line.rstrip("\n\r")
        if not line.strip():
            continue
        if i == 1 and "scan-id" in line.lower():
            continue
        if "|" in line:
            parts = [p.strip() for p in line.split("|")]
        else:
            parts = [p.strip() for p in line.split("\t")]
        if len(parts) < 5:
            print(f"MALFORMED:{path}:{i}", file=sys.stderr)
            sys.exit(2)
        scan_id, _branch, outcome, reason, commit = parts[:5]
        if outcome not in VALID or not reason_valid(reason) or not scan_id:
            continue
        if evidence_tail_only(commit):
            covered.add(scan_id)

for scan_id in sorted(covered):
    print(scan_id)
PY
}

check_triage_requirement() {
  local triage_tsv="$1"
  local delta_ids covered_ids
  delta_ids="$(inspect_delta_scan_ids | sed '/^$/d' || true)"
  if [[ -z "$delta_ids" ]]; then
    return 0
  fi

  if [[ ! -f "$triage_tsv" ]]; then
    emit_verdict reserve "triage-missing"
    return 1
  fi

  local covered_status
  covered_status=0
  covered_ids="$(triage_covered_scan_ids "$triage_tsv" 2>/dev/null)" || covered_status=$?
  if [[ "$covered_status" -eq 2 ]]; then
    emit_verdict fail "triage-table"
    return 1
  fi

  local scan_id
  while IFS= read -r scan_id; do
    [[ -z "$scan_id" ]] && continue
    if ! printf '%s\n' "$covered_ids" | grep -Fxq "$scan_id"; then
      emit_verdict reserve "triage-missing"
      return 1
    fi
  done <<<"$delta_ids"
  return 0
}

check_ci_status() {
  if [[ -n "$FIXTURE_DIR" ]]; then
    local status
    status="$(read_fixture_file ci_status.txt green)"
    if [[ "$status" != "green" ]]; then
      emit_verdict fail "ci-not-green: rerun/fix failing checks before clearance"
      return 1
    fi
    return 0
  fi
  if [[ -z "$PR_NUMBER" ]] || ! command -v gh >/dev/null 2>&1; then
    return 0
  fi
  local failing
  failing="$(gh pr checks "$PR_NUMBER" 2>/dev/null | awk '$2 != "pass" && $2 != "skipping" && NF {print}' || true)"
  if [[ -n "$failing" ]]; then
    emit_verdict fail "ci-not-green: rerun/fix failing checks before clearance"
    return 1
  fi
  return 0
}

append_ledger() {
  local ledger="$1"
  local class="$2"
  local pr="$3"
  local sha="$4"
  local sketch="${5:-}"
  local date
  date="$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u)"
  if [[ ! -f "$ledger" ]]; then
    printf 'verdict\tclass\tpr\tsha\tdate\tsketch\n' >"$ledger"
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$VERDICT" "$class" "$pr" "$sha" "$date" "$sketch" >>"$ledger"
}

route_clearance() {
  local classes_tsv="$1"
  local binding_tsv="$2"
  local ledger_tsv="$3"
  local triage_tsv="$4"

  resolve_changed_files_once || true

  if ! load_tsv "$classes_tsv" 6 >/dev/null 2>&1; then
    emit_verdict reserve "harness-error"
    return 0
  fi
  if ! load_tsv "$binding_tsv" 5 >/dev/null 2>&1; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  if [[ "$PR_RESOLUTION_FAILED" -eq 1 ]]; then
    print_pr_resolution_remedy
    emit_verdict reserve "harness-error"
    return 0
  fi

  if [[ "$REQUESTED_TARGET" -eq 1 ]] && ! changed_files_nonempty; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  if check_self_application; then
    emit_verdict reserve "gate-wiring"
    return 0
  fi

  local body
  body="$(pr_body_text)"

  # Explicit novelty overrides matched-class clearance; requires novelty_basis.
  if check_explicit_novelty_claim "$body"; then
    if ! check_explicit_novelty_basis "$body"; then
      emit_verdict fail "missing-novelty-basis: add novelty_basis explaining the unanticipated implementation discovery"
      return 0
    fi
    emit_verdict reserve "novelty"
    return 0
  fi

  local classes
  classes="$(detect_classes "$classes_tsv")"
  if [[ -z "$classes" ]]; then
    # Empty-class split (CLEARANCE-ADMITTED-SCOPE-GAP-0 / #1242 Option A):
    # novelty already handled above; gate-wiring already handled above.
    # 1) admitted_envelope claim + required fields → admitted-scope-router-gap
    #    (missing fields → FAIL; forbidden surfaces → class-envelope-violation)
    # 2) else → unclassified-scope (narrowed: no class + no valid admitted claim)
    if check_admitted_envelope_claim "$body"; then
      if ! check_admitted_scope_gap_fields "$body"; then
        return 0
      fi
      if ! check_admitted_scope_forbidden_surfaces "$body"; then
        return 0
      fi
      emit_verdict reserve "admitted-scope-router-gap"
      return 0
    fi
    emit_verdict reserve "unclassified-scope"
    return 0
  fi

  local class_count
  class_count="$(printf '%s\n' "$classes" | sed '/^$/d' | wc -l | tr -d ' ')"
  if [[ "$class_count" -gt 1 ]]; then
    emit_verdict reserve "harness-error"
    return 0
  fi

  local class_id
  class_id="$(printf '%s\n' "$classes" | head -n 1)"
  local status
  status="$(class_status "$classes_tsv" "$class_id")"
  if [[ "$status" == "suspended" ]]; then
    emit_verdict reserve "class-suspended"
    return 0
  fi

  local rung
  rung="$(class_for_rung "$class_id")"
  if check_binding_conditions "$binding_tsv" "$rung"; then
    emit_verdict reserve "binding-conditions"
    return 0
  fi

  local reqs
  reqs="$(class_requirements "$classes_tsv" "$class_id")"

  # Engine-scope before workshop envelope so engine touches name the precise reason.
  if [[ "$reqs" == *no_engine_crate* ]] && ! check_no_engine_crate; then
    emit_verdict reserve "engine-scope-violation"
    return 0
  fi
  if [[ "$reqs" == *no_engine_src* ]] && ! check_no_engine_src; then
    emit_verdict reserve "engine-scope-violation"
    return 0
  fi
  if [[ "$reqs" == *workshop_only* ]] && ! check_workshop_only; then
    emit_verdict reserve "class-envelope-violation"
    return 0
  fi
  if [[ "$class_id" == "corpus-module-marker-sweep" ]] && ! check_module_marker_inventory_deletions; then
    emit_verdict reserve "module-marker-shape-mismatch"
    return 0
  fi

  # Data-driven path envelope (scope_all / forbidden) for DSL-migrated classes.
  if ! check_class_predicate_envelope "$class_id"; then
    return 0
  fi

  if [[ "$reqs" == *tested_code_sha* || "$reqs" == *coverage_basis* ]]; then
    if ! check_required_pr_body_fields "$body"; then
      return 0
    fi
  fi

  if [[ "$reqs" == *gpu_proof* ]]; then
    if ! check_recorded_gpu_proof "$body"; then
      return 0
    fi
  fi

  if [[ "$reqs" == *ci_green* ]]; then
    if ! check_ci_status; then
      return 0
    fi
  fi

  if ! check_triage_requirement "$triage_tsv"; then
    return 0
  fi

  emit_verdict clearable
}

reset_clearance_state() {
  REQUESTED_TARGET=0
  CHANGED_FILES_RESOLVED=0
  CHANGED_FILES_LIST=""
  PR_RESOLUTION_FAILED=0
}

run_fixture() {
  local name="$1"
  reset_clearance_state
  FIXTURE_DIR="${FIXTURES_ROOT}/${name}"
  [[ -d "$FIXTURE_DIR" ]] || { echo "missing fixture: $name" >&2; return 1; }
  local expected got
  expected="$(cat "${FIXTURE_DIR}/expected_verdict.txt" | tr -d '\r' | head -n 1)"
  local paths
  paths="$(resolve_paths)"
  local classes_tsv binding_tsv ledger_tsv triage_tsv
  classes_tsv="$(echo "$paths" | sed -n '1p')"
  binding_tsv="$(echo "$paths" | sed -n '2p')"
  ledger_tsv="$(mktemp "${TMPDIR:-/tmp}/clearance-ledger-XXXXXX")"
  triage_tsv="$(echo "$paths" | sed -n '4p')"
  printf 'verdict\tclass\tpr\tsha\tdate\tsketch\n' >"$ledger_tsv"
  local out
  out="$(route_clearance "$classes_tsv" "$binding_tsv" "$ledger_tsv" "$triage_tsv" || true)"
  got="$(printf '%s\n' "$out" | grep -E '^CLEARANCE-VERDICT:' | head -n 1 || true)"
  if [[ "$got" != "$expected" ]]; then
    echo "FAIL ${name}"
    echo "  expected: ${expected}"
    echo "  got:      ${got}"
    return 1
  fi
  # DA-RESERVE must also fold treeverify profile; CLEARABLE/FAIL must not.
  if [[ "$got" == CLEARANCE-VERDICT:\ DA-RESERVE* ]]; then
    if ! printf '%s\n' "$out" | grep -qE '^DA-TREEVERIFY-PROFILE:'; then
      echo "FAIL ${name}"
      echo "  expected: DA-TREEVERIFY-PROFILE on DA-RESERVE"
      echo "  got:      (missing)"
      return 1
    fi
    if ! printf '%s\n' "$out" | grep -qE '^REQUIRED-ANCHORS:'; then
      echo "FAIL ${name}"
      echo "  expected: REQUIRED-ANCHORS on DA-RESERVE"
      echo "  got:      (missing)"
      return 1
    fi
    if [[ -f "${FIXTURE_DIR}/expected_required_anchors.txt" ]]; then
      local want got_req
      want="$(tr -d '\r' <"${FIXTURE_DIR}/expected_required_anchors.txt" | head -n 1)"
      got_req="$(printf '%s\n' "$out" | grep -E '^REQUIRED-ANCHORS:' | head -n 1 || true)"
      if [[ "$want" == contains:* ]]; then
        local needle="${want#contains:}"
        if ! printf '%s\n' "$got_req" | grep -qF "$needle"; then
          echo "FAIL ${name}"
          echo "  expected REQUIRED-ANCHORS to contain: ${needle}"
          echo "  got:      ${got_req}"
          return 1
        fi
      elif [[ "$got_req" != "REQUIRED-ANCHORS: ${want}" && "$got_req" != "$want" ]]; then
        echo "FAIL ${name}"
        echo "  expected: REQUIRED-ANCHORS: ${want}"
        echo "  got:      ${got_req}"
        return 1
      fi
    fi
  else
    if printf '%s\n' "$out" | grep -qE '^DA-TREEVERIFY-PROFILE:'; then
      echo "FAIL ${name}"
      echo "  expected: no DA-TREEVERIFY-PROFILE on non-reserve"
      echo "  got:      $(printf '%s\n' "$out" | grep -E '^DA-TREEVERIFY-PROFILE:' | head -n 1)"
      return 1
    fi
  fi
  echo "PASS ${name}"
  return 0
}

run_selftest() {
  local fixtures=(
    clearance_selftest_clearable_1150_shape
    clearance_selftest_clearable_1151_shape
    clearance_selftest_clearable_1152_shape
    clearance_selftest_reserve_1154_binding_conditions
    clearance_selftest_fail_closed_malformed_tsv
    clearance_selftest_fail_closed_ambiguous_class
    clearance_selftest_gate_wiring_self_application
    clearance_selftest_gate_wiring_doctrine_anchors_tsv
    clearance_selftest_gate_wiring_anchor_triggers_tsv
    clearance_selftest_required_anchors_kernel_path
    clearance_selftest_suspended_class
    clearance_selftest_missing_required_proof_fields
    clearance_selftest_fail_closed_empty_requested_diff
    clearance_selftest_fail_triage_missing
    clearance_selftest_pass_triage_present
    clearance_selftest_gate_wiring_handoff_template
    clearance_selftest_gate_wiring_agent_onboarding
    clearance_selftest_gate_wiring_da_treeverify
    clearance_selftest_clearable_corpus_sweep_shape
    clearance_selftest_corpus_sweep_doc_only_no_match
    clearance_selftest_retired_corpus_baseline_no_match
    clearance_selftest_corpus_sweep_rejects_engine_src
    clearance_selftest_clearable_module_marker_sweep
    clearance_selftest_module_marker_without_result_no_match
    clearance_selftest_module_marker_bad_inventory_no_match
    clearance_selftest_module_marker_source_edit_rejected
    clearance_selftest_unclassified_scope_not_novelty
    clearance_selftest_docs_ladder_pointer_clearable
    clearance_selftest_engine_scope_violation_not_novelty
    clearance_selftest_explicit_novelty_claim_reserved
    clearance_selftest_matched_class_explicit_novelty_reserved
    clearance_selftest_explicit_novelty_missing_basis_fails
    clearance_selftest_tp_workshop_candidate_clearable
    clearance_selftest_tp_workshop_candidate_rejects_mapeditor_src
    clearance_selftest_tp_workshop_candidate_rejects_engine_src
    clearance_selftest_tp_workshop_candidate_missing_tested_sha
    clearance_selftest_tp_workshop_candidate_missing_coverage
    clearance_selftest_tp_workshop_candidate_missing_ci_green
    clearance_selftest_admitted_clause_api_clearable
    clearance_selftest_admitted_clause_api_missing_admitted_flag
    clearance_selftest_admitted_clause_api_ui_picker_yes
    clearance_selftest_admitted_clause_api_tp_defaults_yes
    clearance_selftest_admitted_clause_api_session_hydrate_missing
    clearance_selftest_admitted_clause_api_rejects_runtime_src
    clearance_selftest_workshop_candidate_not_stolen_by_admitted_api
    clearance_selftest_admitted_scope_true_unknown
    clearance_selftest_admitted_scope_api_gap
    clearance_selftest_admitted_scope_picker_gap
    clearance_selftest_admitted_scope_missing_proof_fields
    clearance_selftest_admitted_scope_forbidden_surface
    clearance_selftest_admitted_scope_novelty_preserved
    clearance_selftest_admitted_scope_gate_wiring
    clearance_selftest_picker_class_clearable
    clearance_selftest_picker_class_missing_fields
    clearance_selftest_picker_class_production_api_only_no
    clearance_selftest_picker_class_tp_defaults_yes
    clearance_selftest_picker_class_duplicate_parse_rebind_yes
    clearance_selftest_picker_class_closeout_yes
    clearance_selftest_picker_class_rejects_runtime_src
    clearance_selftest_picker_class_admitted_scope_gap
    clearance_selftest_picker_class_api_nonregression
    clearance_selftest_picker_class_gate_wiring
    clearance_selftest_dsl_forbidden_glob_hit
    clearance_selftest_dsl_treeverify_profile_on_reserve
    clearance_selftest_studio_clock_class_clearable
    clearance_selftest_studio_clock_class_missing_tested_sha
    clearance_selftest_studio_clock_class_missing_coverage
    clearance_selftest_studio_clock_class_missing_ci_green
    clearance_selftest_studio_clock_class_rejects_transport_ui
    clearance_selftest_studio_clock_class_rejects_live_bridge
    clearance_selftest_studio_clock_class_rejects_library_ui
    clearance_selftest_studio_clock_class_rejects_runtime_src
    clearance_selftest_studio_clock_class_api_nonregression
    clearance_selftest_studio_clock_class_picker_nonregression
    clearance_selftest_studio_clock_class_gate_wiring
    clearance_selftest_live_ops_class_clearable
    clearance_selftest_live_ops_class_phase10_refine_clearable
    clearance_selftest_live_ops_class_rejects_driver
    clearance_selftest_live_ops_class_rejects_kernel_sim_wgsl
    clearance_selftest_live_ops_class_rejects_workshop
    clearance_selftest_live_ops_class_rejects_gate_wiring
    clearance_selftest_live_ops_class_rejects_spec_clause
    clearance_selftest_live_ops_class_missing_tested_sha
    clearance_selftest_live_ops_class_missing_coverage
    clearance_selftest_live_ops_class_missing_ci_green
    clearance_selftest_live_ops_class_explicit_novelty
    clearance_selftest_live_ops_class_admitted_scope_gap
    clearance_selftest_live_ops_class_clock_substrate_nonregression
    clearance_selftest_tp_closed_track_no_longer_clearable
    clearance_selftest_triage_ancestor_evidence_tail_ok
    clearance_selftest_triage_ancestor_code_delta_missing
  )
  local name
  for name in "${fixtures[@]}"; do
    if ! run_fixture "$name"; then
      SELFTEST_FAILURES=$((SELFTEST_FAILURES + 1))
    fi
  done
  if [[ "$SELFTEST_FAILURES" -eq 0 ]]; then
    emit_verdict clearable
    echo "CLEARANCE-SELFTEST: PASS (${#fixtures[@]} fixtures)"
    return 0
  fi
  emit_verdict reserve "harness-error"
  echo "CLEARANCE-SELFTEST: FAIL (${SELFTEST_FAILURES} fixtures)"
  return 1
}

main() {
  parse_args "$@"
  if [[ "$FIXTURE_MODE" == "selftest" ]]; then
    run_selftest
    exit $?
  fi
  if [[ "$FIXTURE_MODE" == "fixture" ]]; then
    reset_clearance_state
    local paths
    paths="$(resolve_paths)"
    route_clearance "$(echo "$paths" | sed -n '1p')" "$(echo "$paths" | sed -n '2p')" "$(echo "$paths" | sed -n '3p')" "$(echo "$paths" | sed -n '4p')"
    exit 0
  fi

  reset_clearance_state
  local paths
  paths="$(resolve_paths)"
  local classes_tsv binding_tsv ledger_tsv triage_tsv
  classes_tsv="$(echo "$paths" | sed -n '1p')"
  binding_tsv="$(echo "$paths" | sed -n '2p')"
  ledger_tsv="$(echo "$paths" | sed -n '3p')"
  triage_tsv="$(echo "$paths" | sed -n '4p')"
  route_clearance "$classes_tsv" "$binding_tsv" "$ledger_tsv" "$triage_tsv"

  if [[ "${CLEARANCE_LEDGER_APPEND:-}" == "1" && -n "$PR_NUMBER" ]]; then
    local sha class_id
    sha="${GITHUB_SHA:-$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)}"
    class_id="$(detect_classes "$classes_tsv" | head -n 1 || echo unknown)"
    append_ledger "$ledger_tsv" "$class_id" "$PR_NUMBER" "$sha" ""
  fi
}

main "$@"
