#!/usr/bin/env bash
# CI-A-RUNNER-0 — thin doctrine-scan engine (reads scans.tsv + allow/*; no invariant patterns here).
set -uo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
readonly SCANS_TSV="${SCRIPT_DIR}/scans.tsv"
readonly ALLOW_DIR="${SCRIPT_DIR}/allow"
readonly FIELD_SEP=' | '

scanner_errors=0
hard_failures=0
inspect_flags=0
selftest_status="SKIPPED"
PR_DELTA_MODE=0
PR_BASE_SHA=""
PR_HEAD_SHA=""
declare -A PR_DELTA_LINE_MAP=()
declare -A PR_DELTA_ADDED_LINE_TEXT=()
TRACK_DOC=""
TRACK_DOC_REL=""
ADDENDUM_SCANS_TSV=""
ADDENDUM_ALLOW_DIR=""
PROVE_ADDENDUM=0

declare -a REPORT_LINES=()

JUSTIF_FILE="${SCRIPT_DIR}/inspect_justifications.tsv"
declare -A JUSTIFS=()
declare -A GLOBAL_SCAN_IDS=()
declare -A GLOBAL_ALLOW_KEYS=()

usage() {
  cat >&2 <<'EOF'
usage: doctrine_scan.sh [--pr-delta BASE [HEAD]] [--track-doc PATH]
       doctrine_scan.sh --prove-addendum
EOF
}

load_justifications() {
  if [[ ! -f "$JUSTIF_FILE" ]]; then
    return
  fi
  while IFS= read -r line || [[ -n "$line" ]]; do
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue
    # key on scan-id for simplicity (first field)
    key="${line%% | *}"
    JUSTIFS["$key"]="$line"
  done <"$JUSTIF_FILE"
}

die_scanner() {
  echo "scanner/data error: $*" >&2
  scanner_errors=$((scanner_errors + 1))
}


# Restore the caller's prior errexit state. Helpers that temporarily use
# `set +e` to capture nonzero status must not leave `set -e` enabled —
# this script's default is `set -uo pipefail` (no errexit), and a leaked
# `set -e` aborts stock-gate `return 1` paths before emit_report.
errexit_is_on() {
  case $- in *e*) return 0 ;; *) return 1 ;; esac
}

restore_errexit() {
  if [[ "${1:-0}" == "1" ]]; then
    set -e
  else
    set +e
  fi
}

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

# Fork-free trim: sets global _T instead of echoing, so callers avoid the
# $(...) command-substitution subshell. On git-bash every $() forks an
# emulated subshell (~15-30ms), which is negligible on Linux CI but turns
# per-field parsing loops into minutes on Windows. Hot parse paths use this.
trimv() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  _T="$s"
}

symbol_tail() {
  local sym="$1"
  if [[ "$sym" == *"::"* ]]; then
    printf '%s' "${sym##*::}"
  else
    printf '%s' "$sym"
  fi
}

door_class_matches_producer_grammar() {
  local symbol="$1"
  local door="$2"
  local tail
  tail="$(symbol_tail "$symbol")"
  case "$door" in
    read) [[ "$tail" =~ ^(read_|readback_) ]] ;;
    dispatch) [[ "$tail" =~ ^dispatch_ ]] || [[ "$tail" == "dispatch" ]] ;;
    apply) [[ "$tail" =~ ^apply_ ]] ;;
    cpu_oracle) [[ "$tail" =~ ^cpu_oracle_ ]] || [[ "$tail" == "execute_ops_cpu_with_emissions" ]] || [[ "$tail" == "execute_threshold_ops_cpu" ]] ;;
    *) false ;;
  esac
}

validate_allow_record() {
  local relpath="$1"
  local line_num="$2"
  local symbol="$3"
  local door="$4"
  local rationale="$5"
  local blocker="$6"

  local allow_name="${relpath##*/}"
  case "$allow_name" in
    sealed_producers.txt)
      case "$door" in
        read|dispatch|apply|cpu_oracle) ;;
        *)
          die_scanner "${relpath}:${line_num}: invalid door-class '${door}' (sealed_producers: read|dispatch|apply|cpu_oracle only)"
          return
          ;;
      esac
      if ! door_class_matches_producer_grammar "$symbol" "$door"; then
        die_scanner "${relpath}:${line_num}: symbol '${symbol}' does not match door-class '${door}' grammar"
      fi
      ;;
    inert_buffer_handles.txt)
      if [[ "$door" != "inert-util" ]]; then
        die_scanner "${relpath}:${line_num}: invalid door-class '${door}' (inert_buffer_handles: inert-util only)"
      fi
      ;;
    kernel_surface.txt)
      case "$door" in
        surface-inert|authority-export|sealed-export) ;;
        inert-util)
          die_scanner "${relpath}:${line_num}: inert-util is forbidden in kernel_surface.txt (use surface-inert|authority-export|sealed-export)"
          ;;
        *)
          die_scanner "${relpath}:${line_num}: invalid door-class '${door}' (kernel_surface: surface-inert|authority-export|sealed-export only)"
          ;;
      esac
      ;;
    *)
      die_scanner "${relpath}:${line_num}: unknown allowlist file"
      ;;
  esac
}

validate_allow_file() {
  local file="$1"
  local relpath="${file#"${SCRIPT_DIR}/"}"
  local line_num=0
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_num=$((line_num + 1))
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue
    local fields=()
    if ! parse_allow_fields "$line" fields; then
      die_scanner "${relpath}:${line_num}: expected 4 fields (symbol | door-class | rationale | promotion-blocker)"
      continue
    fi
    if [[ "${#fields[@]}" -ne 4 ]]; then
      die_scanner "${relpath}:${line_num}: expected 4 fields, got ${#fields[@]}"
      continue
    fi
    local symbol="${fields[0]}"
    local door="${fields[1]}"
    local rationale="${fields[2]}"
    local blocker="${fields[3]}"
    if [[ -z "$symbol" || -z "$door" || -z "$rationale" || -z "$blocker" ]]; then
      die_scanner "${relpath}:${line_num}: empty symbol, door-class, rationale, or promotion-blocker"
      continue
    fi
    validate_allow_record "$relpath" "$line_num" "$symbol" "$door" "$rationale" "$blocker"
  done <"$file"
}

repo_relpath() {
  local path="$1"
  path="${path//\\//}"
  path="${path#./}"
  local root="${REPO_ROOT//\\//}"
  if [[ "$path" == "$root"/* ]]; then
    path="${path#"${root}/"}"
  fi
  printf '%s' "$path"
}

parse_fields() {
  local line="$1"
  local -n _out="$2"
  local rest="$line"
  _out=()
  local i part
  for ((i = 0; i < 7; i++)); do
    if [[ "$i" -eq 6 ]]; then
      trimv "$rest"; part="$_T"
      _out+=("$part")
      break
    fi
    if [[ "$rest" != *"${FIELD_SEP}"* ]]; then
      return 1
    fi
    part="${rest%%"${FIELD_SEP}"*}"
    trimv "$part"; _out+=("$_T")
    rest="${rest#*"${FIELD_SEP}"}"
  done
  return 0
}

parse_allow_fields() {
  local line="$1"
  local -n _out="$2"
  local rest="$line"
  _out=()
  local i part
  for ((i = 0; i < 4; i++)); do
    if [[ "$i" -eq 3 ]]; then
      trimv "$rest"; part="$_T"
      _out+=("$part")
      break
    fi
    if [[ "$rest" != *"${FIELD_SEP}"* ]]; then
      return 1
    fi
    part="${rest%%"${FIELD_SEP}"*}"
    trimv "$part"; _out+=("$_T")
    rest="${rest#*"${FIELD_SEP}"}"
  done
  return 0
}

load_and_validate_allowlists() {
  local f
  for f in "${ALLOW_DIR}/sealed_producers.txt" "${ALLOW_DIR}/inert_buffer_handles.txt" "${ALLOW_DIR}/kernel_surface.txt"; do
    if [[ ! -f "$f" ]]; then
      die_scanner "missing allowlist file: ${f#"${SCRIPT_DIR}/"}"
      continue
    fi
    validate_allow_file "$f"
  done
}

load_global_keys() {
  GLOBAL_SCAN_IDS=()
  GLOBAL_ALLOW_KEYS=()

  local line line_num=0 fields=()
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_num=$((line_num + 1))
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue
    if ! parse_fields "$line" fields; then
      die_scanner "scans.tsv:${line_num}: malformed record (expected 7 fields)"
      continue
    fi
    GLOBAL_SCAN_IDS["${fields[0]}"]=1
  done <"$SCANS_TSV"

  local f rel base allow_fields=()
  for f in "${ALLOW_DIR}/sealed_producers.txt" "${ALLOW_DIR}/inert_buffer_handles.txt" "${ALLOW_DIR}/kernel_surface.txt"; do
    [[ -f "$f" ]] || continue
    rel="${f#"${SCRIPT_DIR}/"}"
    base="${rel##*/}"
    line_num=0
    while IFS= read -r line || [[ -n "$line" ]]; do
      line_num=$((line_num + 1))
      trimv "$line"; line="$_T"
      [[ -z "$line" || "$line" == \#* ]] && continue
      if ! parse_allow_fields "$line" allow_fields; then
        die_scanner "${rel}:${line_num}: expected 4 fields (symbol | door-class | rationale | promotion-blocker)"
        continue
      fi
      GLOBAL_ALLOW_KEYS["${base}:${allow_fields[0]}"]=1
    done <"$f"
  done

  f="${ALLOW_DIR}/sealed_types.txt"
  if [[ -f "$f" ]]; then
    line_num=0
    while IFS= read -r line || [[ -n "$line" ]]; do
      line_num=$((line_num + 1))
      trimv "$line"; line="$_T"
      [[ -z "$line" || "$line" == \#* ]] && continue
      GLOBAL_ALLOW_KEYS["sealed_types.txt:${line}"]=1
    done <"$f"
  fi
}

resolve_track_addendum() {
  [[ -z "$TRACK_DOC" ]] && return
  TRACK_DOC_REL="$(repo_relpath "$TRACK_DOC")"
  if [[ ! -f "${REPO_ROOT}/${TRACK_DOC_REL}" ]]; then
    die_scanner "--track-doc is not a repo file: ${TRACK_DOC}"
    return
  fi
  ADDENDUM_SCANS_TSV="${REPO_ROOT}/${TRACK_DOC_REL}.ci.tsv"
  ADDENDUM_ALLOW_DIR="${REPO_ROOT}/${TRACK_DOC_REL}.ci.allow"
}

validate_track_scan_addendum() {
  [[ -n "$ADDENDUM_SCANS_TSV" && -f "$ADDENDUM_SCANS_TSV" ]] || return

  local line line_num=0 fields=()
  declare -A local_ids=()
  local rel="${TRACK_DOC_REL}.ci.tsv"
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_num=$((line_num + 1))
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue
    if ! parse_fields "$line" fields; then
      die_scanner "${rel}:${line_num}: malformed record (expected 7 fields)"
      continue
    fi
    local scan_id="${fields[0]}"
    if [[ -n "${GLOBAL_SCAN_IDS[$scan_id]:-}" ]]; then
      die_scanner "${rel}:${line_num}: track addendum redefines global scan-id '${scan_id}'"
    fi
    if [[ -n "${local_ids[$scan_id]:-}" ]]; then
      die_scanner "${rel}:${line_num}: duplicate track addendum scan-id '${scan_id}'"
    fi
    local_ids["$scan_id"]=1
  done <"$ADDENDUM_SCANS_TSV"
}

validate_track_allow_addendum() {
  [[ -n "$ADDENDUM_ALLOW_DIR" && -d "$ADDENDUM_ALLOW_DIR" ]] || return

  local f base rel line line_num fields=()
  declare -A local_keys=()
  shopt -s nullglob
  for f in "${ADDENDUM_ALLOW_DIR}"/*; do
    base="${f##*/}"
    rel="$(repo_relpath "$f")"
    case "$base" in
      sealed_producers.txt|inert_buffer_handles.txt|kernel_surface.txt|sealed_types.txt) ;;
      *)
        die_scanner "${rel}: unknown track addendum allow file"
        continue
        ;;
    esac

    line_num=0
    while IFS= read -r line || [[ -n "$line" ]]; do
      line_num=$((line_num + 1))
      trimv "$line"; line="$_T"
      [[ -z "$line" || "$line" == \#* ]] && continue
      if [[ "$base" == "sealed_types.txt" ]]; then
        if [[ "$line" == *" | "* || "$line" == *" "* || "$line" == *$'\t'* ]]; then
          die_scanner "${rel}:${line_num}: malformed sealed type (expected one bare name)"
          continue
        fi
        fields=("$line")
      else
        if ! parse_allow_fields "$line" fields; then
          die_scanner "${rel}:${line_num}: expected 4 fields (symbol | door-class | rationale | promotion-blocker)"
          continue
        fi
        if [[ "${#fields[@]}" -ne 4 || -z "${fields[0]}" || -z "${fields[1]}" || -z "${fields[2]}" || -z "${fields[3]}" ]]; then
          die_scanner "${rel}:${line_num}: empty symbol, door-class, rationale, or promotion-blocker"
          continue
        fi
        validate_allow_record "$rel" "$line_num" "${fields[0]}" "${fields[1]}" "${fields[2]}" "${fields[3]}"
      fi

      local key="${base}:${fields[0]}"
      if [[ -n "${GLOBAL_ALLOW_KEYS[$key]:-}" ]]; then
        die_scanner "${rel}:${line_num}: track addendum duplicates global allow key '${key}'"
      fi
      if [[ -n "${local_keys[$key]:-}" ]]; then
        die_scanner "${rel}:${line_num}: duplicate track addendum allow key '${key}'"
      fi
      local_keys["$key"]=1
    done <"$f"
  done
  shopt -u nullglob
}

line_matches_any_exclude() {
  local line="$1"
  local excludes="$2"
  [[ -z "$excludes" ]] && return 1
  local content="$line"
  if [[ "$line" =~ ^(.+):([0-9]+):(.*)$ ]]; then
    content="${BASH_REMATCH[3]}"
  fi
  local pat
  IFS=';' read -ra _ex_arr <<<"$excludes"
  for pat in "${_ex_arr[@]}"; do
    trimv "$pat"; pat="$_T"
    [[ -z "$pat" ]] && continue
    if [[ "$pat" == ^* ]]; then
      # fast path for ^ excludes, avoid rg spawn
      if [[ "$content" =~ $pat ]]; then
        return 0
      fi
    elif [[ "$pat" != *[.\\\*\+\?\|\(\)\[\]\{\}\^$]* ]]; then
      # simple literal (no regex metachars), in-process contains
      if [[ "$content" == *"$pat"* ]]; then
        return 0
      fi
    else
      if printf '%s\n' "$content" | rg -q -e "$pat" 2>/dev/null; then
        return 0
      fi
      if [[ "$content" != "$line" ]] && printf '%s\n' "$line" | rg -q -e "$pat" 2>/dev/null; then
        return 0
      fi
    fi
  done
  return 1
}

heuristic_in_cfg_test_region() {
  local line="$1"
  local file line_num rel
  if [[ "$line" =~ ^(.+):([0-9]+): ]]; then
    file="${BASH_REMATCH[1]}"
    line_num="${BASH_REMATCH[2]}"
  else
    return 1
  fi
  file="${file//\\//}"
  file="${file#./}"
  if [[ "$file" == crates/* ]]; then
    rel="$file"
  elif [[ "$file" == */crates/* ]]; then
    rel="${file#*/crates/}"
    rel="crates/${rel}"
  else
    return 1
  fi
  local abs="${REPO_ROOT}/${rel}"
  [[ -f "$abs" ]] || return 1
  # Use bash to find last #[cfg(test)] before line_num (no rg -n, robust across builds)
  local cfg_line=0
  local n=0
  local l
  while IFS= read -r l || [[ -n "$l" ]]; do
    n=$((n + 1))
    if [[ "$l" =~ ^[[:space:]]*#\[[Cc]fg\(test\)\] ]]; then
      if [[ "$n" -lt "$line_num" && "$n" -gt "$cfg_line" ]]; then
        cfg_line=$n
      fi
    fi
  done < "$abs"
  [[ "$cfg_line" -eq 0 ]] && return 1
  # check following lines contain mod tests (pure bash, no sed/rg spawn)
  local has_mod=1
  local i=0
  while IFS= read -r l || [[ -n "$l" ]]; do
    i=$((i + 1))
    if [[ $i -lt $cfg_line ]]; then continue; fi
    if [[ $i -gt $((cfg_line + 4)) ]]; then break; fi
    if [[ "$l" =~ mod[[:space:]]+tests ]]; then
      has_mod=0
      break
    fi
  done < "$abs"
  [[ $has_mod -eq 0 ]] || return 1
  [[ "$line_num" -gt "$cfg_line" ]]
}

normalize_match_path() {
  local file="$1"
  file="${file//\\//}"
  file="${file#./}"
  local root="${REPO_ROOT//\\//}"
  if [[ "$file" == "$root"/* ]]; then
    file="${file#"${root}/"}"
  fi
  printf '%s' "$file"
}

load_pr_delta_line_map() {
  PR_DELTA_LINE_MAP=()
  PR_DELTA_ADDED_LINE_TEXT=()
  local current_file=""
  local current_new_line=0
  local line
  while IFS= read -r line; do
    line="${line//$'\r'/}"
    if [[ "$line" =~ ^\+\+\+\ b/(.+)$ ]]; then
      current_file="${BASH_REMATCH[1]}"
      current_file="$(normalize_match_path "$current_file")"
    elif [[ "$line" =~ ^@@\ .*\+([0-9]+)(,([0-9]+))?\ @@ ]]; then
      local start="${BASH_REMATCH[1]}"
      local count="${BASH_REMATCH[3]:-1}"
      current_new_line="$start"
      local i
      for ((i = 0; i < count; i++)); do
        PR_DELTA_LINE_MAP["${current_file}:$((start + i))"]=1
      done
    elif [[ "$line" == "+"* && "$line" != "+++"* && -n "$current_file" && "$current_new_line" -gt 0 ]]; then
      PR_DELTA_ADDED_LINE_TEXT["${current_file}:${current_new_line}"]="${line#+}"
      current_new_line=$((current_new_line + 1))
    elif [[ "$line" == "-"* && "$line" != "---"* ]]; then
      :
    elif [[ -n "$current_file" && "$current_new_line" -gt 0 ]]; then
      current_new_line=$((current_new_line + 1))
    fi
  done < <(git -C "$REPO_ROOT" diff -U0 "${PR_BASE_SHA}".."${PR_HEAD_SHA}" 2>/dev/null || true)
}

pr_delta_scope_files() {
  local target_glob="$1"
  local changed globbed f
  declare -A seen=()
  while IFS= read -r changed; do
    [[ -z "$changed" ]] && continue
    changed="${changed//$'\r'/}"
    changed="$(normalize_match_path "$changed")"
    seen["$changed"]=1
  done < <(git -C "$REPO_ROOT" diff --name-only "${PR_BASE_SHA}".."${PR_HEAD_SHA}" 2>/dev/null || true)

  while IFS= read -r globbed; do
    [[ -z "$globbed" ]] && continue
    globbed="$(normalize_match_path "$globbed")"
    if [[ -n "${seen[$globbed]:-}" ]]; then
      printf '%s\n' "$globbed"
    fi
  done < <(cd "$REPO_ROOT" && rg --files -g "$target_glob" . 2>/dev/null || true)
}

match_line_in_pr_delta() {
  local match="$1"
  local file line
  if [[ "$match" =~ ^(.+):([0-9]+): ]]; then
    file="$(normalize_match_path "${BASH_REMATCH[1]}")"
    line="${BASH_REMATCH[2]}"
    [[ -n "${PR_DELTA_LINE_MAP[${file}:${line}]:-}" ]]
  else
    false
  fi
}

heuristic_target_includes_tests() {
  local target_glob="$1"
  [[ "$target_glob" == *tests* || "$target_glob" == *test* || "$target_glob" == *"_tests.rs"* ]]
}

# HC-HORIZON-ENTRY-CONVENTION-0: greppable dated marker exempts GUARD-KABUKI only while fresh.
# Shape: HORIZON-ENTRY(<YYYY-MM-DD>): <intended consumer / design ref>
# Never a bare token forever-pass; stale/malformed/missing keep the finding (INSPECT).
# Window days: HORIZON_ENTRY_STALE_DAYS (default 90). Override only for selftest.
HORIZON_ENTRY_STALE_DAYS="${HORIZON_ENTRY_STALE_DAYS:-90}"

# Filter kabuki matches: drop only those whose preceding symbol block carries a well-formed
# FRESH HORIZON-ENTRY marker. Returns filtered list via nameref.
filter_guard_kabuki_horizon_exemptions() {
  local -n _kabuki_matches="$1"
  local filtered=()
  if [[ "${#_kabuki_matches[@]}" -eq 0 ]]; then
    return 0
  fi
  if ! command -v python >/dev/null 2>&1; then
    # Without python, cannot assess dates — keep all findings (fail-closed for exemption).
    return 0
  fi
  # Program on argv (-c); matches on stdin. A heredoc would steal stdin and
  # silently drop every finding (false forever-pass) — never do that here.
  local py_prog
  py_prog='
import os, re, sys
from datetime import date, datetime
from pathlib import Path

repo = Path(os.environ.get("REPO_ROOT", "."))
stale_days = int(os.environ.get("HORIZON_ENTRY_STALE_DAYS", "90"))
today = date.today()
pin = os.environ.get("HORIZON_ENTRY_TODAY", "").strip()
if pin:
    today = datetime.strptime(pin, "%Y-%m-%d").date()

MARKER_RE = re.compile(r"HORIZON-ENTRY\((\d{4}-\d{2}-\d{2})\):\s*(\S.+?)\s*$")
COMMENT_RE = re.compile(r"^\s*(?:///?|/\*+|\*+)\s*(.*?)(?:\*/)?\s*$")
ATTR_RE = re.compile(r"^\s*#\[")
MATCH_RE = re.compile(r"^(.+):(\d+):(.*)$")
FN_RE = re.compile(r"^\s*pub\s+fn\s+")

def parse_marker(text):
    m = MARKER_RE.search(text)
    if not m:
        return None
    try:
        d = datetime.strptime(m.group(1), "%Y-%m-%d").date()
    except ValueError:
        return None
    ref = m.group(2).strip()
    if not ref:
        return None
    return d, ref

def fresh(d):
    age = (today - d).days
    return 0 <= age <= stale_days

def enclosing_symbol_line(lines, line_num):
    if line_num < 1 or line_num > len(lines):
        return line_num
    for i in range(line_num - 1, -1, -1):
        if FN_RE.match(lines[i]):
            return i + 1
    return line_num

def symbol_has_fresh_marker(path, line_num):
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return False
    if line_num < 1 or line_num > len(lines):
        return False
    anchor = enclosing_symbol_line(lines, line_num)
    i = anchor - 2
    while i >= 0:
        raw = lines[i]
        if raw.strip() == "":
            i -= 1
            continue
        if ATTR_RE.match(raw):
            i -= 1
            continue
        cm = COMMENT_RE.match(raw)
        if cm:
            parsed = parse_marker(cm.group(1))
            if parsed is None:
                parsed = parse_marker(raw)
            if parsed is not None:
                d, _ref = parsed
                return fresh(d)
            i -= 1
            continue
        break
    if 0 <= anchor - 1 < len(lines):
        parsed = parse_marker(lines[anchor - 1])
        if parsed is not None:
            return fresh(parsed[0])
    return False

for match in sys.stdin.read().splitlines():
    if not match.strip():
        continue
    m = MATCH_RE.match(match)
    if not m:
        print(match)
        continue
    rel = m.group(1).replace("\\", "/")
    if rel.startswith("./"):
        rel = rel[2:]
    try:
        line_num = int(m.group(2))
    except ValueError:
        print(match)
        continue
    path = Path(rel)
    if not path.is_absolute():
        path = repo / rel
    if not path.is_file():
        path = repo / rel.lstrip("/")
    if symbol_has_fresh_marker(path, line_num):
        continue
    print(match)
'
  local kept
  kept="$(
    printf '%s\n' "${_kabuki_matches[@]}" | \
    HORIZON_ENTRY_STALE_DAYS="$HORIZON_ENTRY_STALE_DAYS" \
    HORIZON_ENTRY_TODAY="${HORIZON_ENTRY_TODAY:-}" \
    REPO_ROOT="$REPO_ROOT" \
    python -c "$py_prog"
  )"
  if [[ -n "$kept" ]]; then
    while IFS= read -r line; do
      [[ -z "$line" ]] && continue
      filtered+=("$line")
    done <<<"$kept"
  fi
  if [[ "${#filtered[@]}" -eq 0 ]]; then
    _kabuki_matches=()
  else
    _kabuki_matches=("${filtered[@]}")
  fi
}

run_rg_scan() {
  local pattern="$1"
  local target_glob="$2"
  local excludes="$3"
  local severity="$4"
  local -n _matches_out="$5"
  _matches_out=()

  local rg_args=(-U --multiline --no-heading --line-number --with-filename -e "$pattern")
  local search_paths=()
  local use_relative_paths=0
  if [[ "$severity" == "HEURISTIC" && "$PR_DELTA_MODE" -eq 1 ]]; then
    local scope_file
    while IFS= read -r scope_file; do
      [[ -z "$scope_file" ]] && continue
      search_paths+=("$scope_file")
    done < <(pr_delta_scope_files "$target_glob")
    if [[ "${#search_paths[@]}" -eq 0 ]]; then
      return 0
    fi
    use_relative_paths=1
    if ! heuristic_target_includes_tests "$target_glob"; then
      rg_args+=(-g '!**/tests/**' -g '!**/test/**' -g '!**/*_tests.rs')
    fi
  else
    rg_args+=(-g "$target_glob")
    if [[ "$severity" == "HEURISTIC" ]]; then
      if ! heuristic_target_includes_tests "$target_glob"; then
        rg_args+=(-g '!**/tests/**' -g '!**/test/**' -g '!**/*_tests.rs')
      fi
    fi
    search_paths=(".")
  fi

  local rg_out=""
  local rg_status=0
  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  if [[ "$use_relative_paths" -eq 1 ]]; then
    rg_out="$(cd "$REPO_ROOT" && rg "${rg_args[@]}" "${search_paths[@]}" 2>&1)"
  else
    rg_out="$(cd "$REPO_ROOT" && rg "${rg_args[@]}" "${search_paths[0]}" 2>&1)"
  fi
  rg_status=$?
  restore_errexit "$_errexit_was_on"

  if [[ "$rg_status" -eq 2 ]]; then
    die_scanner "ripgrep error (exit ${rg_status}) for pattern '${pattern}' on '${target_glob}': ${rg_out}"
    return 2
  fi
  if [[ "$rg_status" -eq 1 ]]; then
    return 0
  fi

  local line
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    if line_matches_any_exclude "$line" "$excludes"; then
      continue
    fi
    if [[ "$severity" == "HEURISTIC" ]] && heuristic_in_cfg_test_region "$line"; then
      continue
    fi
    if [[ "$severity" == "HEURISTIC" && "$PR_DELTA_MODE" -eq 1 ]] && ! match_line_in_pr_delta "$line"; then
      continue
    fi
    _matches_out+=("$line")
  done <<<"$rg_out"
  return 0
}

run_allowlist_scan() {
  local mode="$1"
  local -n _matches_out="$2"
  _matches_out=()

  if ! command -v python >/dev/null 2>&1; then
    die_scanner "python not found on PATH (required for @ALLOWLIST scans)"
    return 2
  fi

  local script="${SCRIPT_DIR}/scan_allowlists.py"
  if [[ ! -f "$script" ]]; then
    die_scanner "missing scan_allowlists.py"
    return 2
  fi

  local out=""
  local py_status=0
  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(python "$script" "$mode" 2>&1)"
  py_status=$?
  restore_errexit "$_errexit_was_on"

  if [[ "$py_status" -eq 2 ]]; then
    die_scanner "allowlist scan error (${mode}): ${out}"
    return 2
  fi

  if [[ -n "$out" ]]; then
    while IFS= read -r line; do
      [[ -z "$line" ]] && continue
      _matches_out+=("$line")
    done <<<"$out"
  fi
  return 0
}

file_has_table_driven_form() {
  local file="$1"
  local abs="${REPO_ROOT}/${file}"
  [[ -f "$abs" ]] || return 1
  rg -q -e 'const[[:space:]]+[A-Z0-9_]*CASES|static[[:space:]]+[A-Z0-9_]*CASES|for[[:space:]]+.*[[:space:]]+in[[:space:]]+.*CASES|for[[:space:]]+.*[[:space:]]+in[[:space:]]+cases|table_driven|TEST_BUDGET_TABLE_DRIVEN_OK' "$abs" 2>/dev/null
}

run_test_budget_scan() {
  local target_glob="$1"
  local -n _matches_out="$2"
  _matches_out=()

  [[ "$PR_DELTA_MODE" -eq 1 ]] || return 0

  declare -A added_counts=()
  local key file line text
  for key in "${!PR_DELTA_ADDED_LINE_TEXT[@]}"; do
    file="${key%:*}"
    text="${PR_DELTA_ADDED_LINE_TEXT[$key]}"
    [[ "$file" == *.rs ]] || continue
    case "$file" in
      crates/*) ;;
      *) continue ;;
    esac
    if [[ "$text" =~ ^[[:space:]]*#\[[[:space:]]*((tokio|async_std)::)?test(\(|\]) ]]; then
      added_counts["$file"]=$(( ${added_counts["$file"]:-0} + 1 ))
    fi
  done

  for file in "${!added_counts[@]}"; do
    if [[ "${added_counts[$file]}" -le 3 ]]; then
      continue
    fi
    if file_has_table_driven_form "$file"; then
      continue
    fi
    _matches_out+=("${file}: added ${added_counts[$file]} #[test] functions without table-driven form")
  done
}

run_require_scan() {
  local pattern="$1"
  local target_glob="$2"
  local -n _matches_out="$3"
  _matches_out=()

  local files=()
  local f
  while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    files+=("$f")
  done < <(cd "$REPO_ROOT" && rg --files -g "$target_glob" . 2>/dev/null || true)

  if [[ "${#files[@]}" -eq 0 ]]; then
    die_scanner "require scan: no paths for glob '${target_glob}'"
    return 2
  fi

  for f in "${files[@]}"; do
    local rg_status=0
    _errexit_was_on=0
    errexit_is_on && _errexit_was_on=1
    set +e
    rg -U --multiline -q -e "$pattern" "${REPO_ROOT}/${f}" 2>/dev/null
    rg_status=$?
    restore_errexit "$_errexit_was_on"
    if [[ "$rg_status" -eq 2 ]]; then
      die_scanner "ripgrep error on require scan for '${f}'"
      return 2
    fi
    if [[ "$rg_status" -eq 1 ]]; then
      _matches_out+=("${f}: missing required pattern")
    fi
  done
  return 0
}

run_scan_file() {
  local scan_file="$1"
  local scan_label="$2"
  local line_num=0
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_num=$((line_num + 1))
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue

    local fields=()
    if ! parse_fields "$line" fields; then
      die_scanner "${scan_label}:${line_num}: malformed record (expected 7 fields)"
      continue
    fi
    if [[ "${#fields[@]}" -ne 7 ]]; then
      die_scanner "${scan_label}:${line_num}: malformed record (expected 7 fields, got ${#fields[@]})"
      continue
    fi

    local scan_id="${fields[0]}"
    local severity="${fields[1]}"
    local target_glob="${fields[2]}"
    local pattern="${fields[3]}"
    local excludes="${fields[4]}"
    local doctrine_ref="${fields[5]}"
    local promotion_blocker="${fields[6]}"
    local require_mode=0
    local allowlist_mode=""
    local test_budget_mode=0

    if [[ "${DOCTRINE_SCAN_ONLY_TEST_BUDGET:-0}" == "1" && "$scan_id" != "TEST-BUDGET" ]]; then
      continue
    fi

    if [[ -z "$scan_id" || -z "$severity" || -z "$pattern" || -z "$doctrine_ref" ]]; then
      die_scanner "${scan_label}:${line_num}: empty required field in '${scan_id}'"
      continue
    fi

    case "$severity" in
      RELIABLE|HEURISTIC) ;;
      *)
        die_scanner "${scan_label}:${line_num}: invalid severity '${severity}' in '${scan_id}'"
        continue
        ;;
    esac

    if [[ "$severity" == "RELIABLE" && -z "$promotion_blocker" ]]; then
      die_scanner "${scan_label}:${line_num}: RELIABLE scan '${scan_id}' missing promotion-blocker"
      continue
    fi

    if [[ "$pattern" == @REQUIRE:* ]]; then
      require_mode=1
      pattern="${pattern#@REQUIRE:}"
    elif [[ "$pattern" == @ALLOWLIST:* ]]; then
      allowlist_mode="${pattern#@ALLOWLIST:}"
    elif [[ "$pattern" == @TEST_BUDGET ]]; then
      test_budget_mode=1
    fi

    local matches=()
    local run_status=0
    if [[ "$test_budget_mode" -eq 1 ]]; then
      run_test_budget_scan "$target_glob" matches
      run_status=$?
    elif [[ -n "$allowlist_mode" ]]; then
      run_allowlist_scan "$allowlist_mode" matches
      run_status=$?
    elif [[ "$require_mode" -eq 1 ]]; then
      run_require_scan "$pattern" "$target_glob" matches
      run_status=$?
    else
      run_rg_scan "$pattern" "$target_glob" "$excludes" "$severity" matches
      run_status=$?
    fi
    if [[ "$run_status" -eq 2 ]]; then
      continue
    fi

    # HC-6: GUARD-KABUKI-TRIPWIRE exempts only a symbol bearing a well-formed FRESH
    # HORIZON-ENTRY(<iso-date>): <consumer/ref> marker. Unmarked, bare-token, malformed,
    # or stale-dated sites stay FLAGGED (never a silent forever-pass).
    if [[ "$scan_id" == "GUARD-KABUKI-TRIPWIRE" && "${#matches[@]}" -gt 0 ]]; then
      filter_guard_kabuki_horizon_exemptions matches
    fi

    local count="${#matches[@]}"
    local verdict="PASS"
    if [[ "$count" -gt 0 ]]; then
      if [[ "$severity" == "RELIABLE" ]]; then
        verdict="FAIL"
        hard_failures=$((hard_failures + count))
      else
        verdict="INSPECT"
        inspect_flags=$((inspect_flags + count))
      fi
    fi

    local path_summary=""
    if [[ "$count" -gt 0 && "$count" -le 8 ]]; then
      path_summary="$(printf ' %s' "${matches[@]}")"
    elif [[ "$count" -gt 8 ]]; then
      path_summary="$(printf ' %s' "${matches[@]:0:8}") ..."
    fi

    REPORT_LINES+=("${scan_id}  ${verdict}  ${count}  ${doctrine_ref}${path_summary}")

    if [[ "$verdict" == "FAIL" ]]; then
      echo "remedy: if this is a legitimate new door, add a conforming record to scripts/ci/allow/<file>.txt with rationale and promotion-blocker; do not edit the scanner" >&2
    fi
  done <"$scan_file"
}

run_scans() {
  run_scan_file "$SCANS_TSV" "scans.tsv"
  if [[ -n "$ADDENDUM_SCANS_TSV" && -f "$ADDENDUM_SCANS_TSV" ]]; then
    run_scan_file "$ADDENDUM_SCANS_TSV" "${TRACK_DOC_REL}.ci.tsv"
  fi
}

run_inventory_drift_gate() {
  [[ "${DOCTRINE_SCAN_SKIP_DRIFT:-0}" == "1" ]] && return 0
  local drift_script="${SCRIPT_DIR}/test_inventory_drift_check.sh"
  [[ -f "$drift_script" ]] || {
    die_scanner "missing test inventory drift gate: scripts/ci/test_inventory_drift_check.sh"
    return 1
  }
  local out status
  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "$drift_script" 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  if [[ "$status" -ne 0 ]]; then
    die_scanner "test inventory drift gate failed"
    while IFS= read -r line; do
      [[ -z "$line" ]] && continue
      REPORT_LINES+=("TEST-INVENTORY-DRIFT  FAIL  1  ${line}")
    done <<<"$out"
    return 1
  fi
  REPORT_LINES+=("TEST-INVENTORY-DRIFT  PASS  0  stock gate: inventory matches discovered tests and KEEP rows are owned")
}

run_stock_gate_script() {
  local scan_id="$1"
  local script_rel="$2"
  local script="${SCRIPT_DIR}/${script_rel}"
  [[ -f "$script" ]] || {
    die_scanner "missing stock gate: ${script_rel}"
    return 1
  }
  local out status verdict
  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "$script" --check 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  verdict="$(printf '%s\n' "$out" | grep -E '^[A-Z0-9_-]+-VERDICT:' | tail -n 1 || true)"
  if [[ -z "$verdict" ]]; then
    die_scanner "${scan_id} gate produced no verdict"
    REPORT_LINES+=("${scan_id}  FAIL  1  missing verdict footer")
    hard_failures=$((hard_failures + 1))
    return 1
  fi
  if [[ "$verdict" == *FAIL* ]]; then
    REPORT_LINES+=("${scan_id}  FAIL  1  ${verdict}")
    hard_failures=$((hard_failures + 1))
    return 1
  fi
  if [[ "$verdict" == *INSPECT* ]]; then
    local n
    n="$(printf '%s' "$verdict" | sed -n 's/.*expiry-candidates=\([0-9]*\).*/\1/p')"
    [[ -z "$n" ]] && n=1
    REPORT_LINES+=("${scan_id}  INSPECT  ${n}  ${verdict}")
    inspect_flags=$((inspect_flags + n))
    return 0
  fi
  REPORT_LINES+=("${scan_id}  PASS  0  ${verdict}")
  return 0
}

run_closing_rung_stock_gates() {
  # Selftest sandboxes copy a minimal CI bundle and set DOCTRINE_SCAN_SKIP_DRIFT=1.
  [[ "${DOCTRINE_SCAN_SKIP_DRIFT:-0}" == "1" ]] && return 0
  run_doc_budget_gate
  run_rule_expiry_gate
  run_agents_stub_gate
  run_da_treeverify_lifecycle_gate
}

run_doc_budget_gate() {
  run_stock_gate_script "DOC-BUDGET" "doc_budget_check.sh"
}

run_rule_expiry_gate() {
  run_stock_gate_script "RULE-EXPIRY" "rule_expiry_check.sh"
}

run_agents_stub_gate() {
  run_stock_gate_script "AGENTS-STUB" "agents_stub_check.sh"
}

run_da_treeverify_lifecycle_gate() {
  # Non-core da_review_profile.tsv rows must carry expires_on and be deleted/retired after expiry.
  local out rc
  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "${SCRIPT_DIR}/da_treeverify.sh" --check-lifecycle 2>&1)"
  rc=$?
  restore_errexit "$_errexit_was_on"
  printf '%s\n' "$out"
  if [[ $rc -ne 0 ]]; then
    echo "DA-TREEVERIFY-LIFECYCLE gate failed" >&2
    return 1
  fi
  return 0
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  local label="$3"
  if [[ "$haystack" != *"$needle"* ]]; then
    echo "addendum proof failed (${label}): missing '${needle}'" >&2
    return 1
  fi
}

assert_not_contains() {
  local haystack="$1"
  local needle="$2"
  local label="$3"
  if [[ "$haystack" == *"$needle"* ]]; then
    echo "addendum proof failed (${label}): unexpected '${needle}'" >&2
    return 1
  fi
}

run_addendum_proof() {
  local tmp_rel="scripts/ci/tmp_track_addendum_proof_$$"
  local tmp_abs="${REPO_ROOT}/${tmp_rel}"
  rm -rf "$tmp_abs"
  mkdir -p "${tmp_abs}/active" "${tmp_abs}/active_track.md.ci.allow" "${tmp_abs}/inactive_track.md.ci.allow"
  trap "rm -rf '${tmp_abs}'" EXIT

  printf '# active track proof fixture\n' >"${tmp_abs}/active_track.md"
  printf '# clean track proof fixture\n' >"${tmp_abs}/clean_track.md"
  printf '# inactive track proof fixture\n' >"${tmp_abs}/inactive_track.md"
  printf 'fn proof() { let _x = "TRACK_LOCAL_BAD_TOKEN"; }\n' >"${tmp_abs}/active/bad.rs"
  printf 'TRACK-LOCAL-BAD | RELIABLE | %s/active/** | TRACK_LOCAL_BAD_TOKEN |  | addendum proof local reliable | proof fixture promotion blocker\n' "$tmp_rel" >"${tmp_abs}/active_track.md.ci.tsv"
  printf 'ACTIVE_TRACK_ONLY_TYPE\n' >"${tmp_abs}/active_track.md.ci.allow/sealed_types.txt"
  printf 'INACTIVE-BAD | RELIABLE | %s/active/** | TRACK_LOCAL_BAD_TOKEN |  | inactive addendum proof | proof fixture promotion blocker\n' "$tmp_rel" >"${tmp_abs}/inactive_track.md.ci.tsv"
  printf 'INACTIVE_TRACK_ONLY_TYPE\n' >"${tmp_abs}/inactive_track.md.ci.allow/sealed_types.txt"

  local global_id=""
  local line fields=()
  while IFS= read -r line || [[ -n "$line" ]]; do
    trimv "$line"; line="$_T"
    [[ -z "$line" || "$line" == \#* ]] && continue
    parse_fields "$line" fields || continue
    global_id="${fields[0]}"
    break
  done <"$SCANS_TSV"
  if [[ -z "$global_id" ]]; then
    echo "addendum proof failed: no global scan-id found" >&2
    return 1
  fi
  printf '# redefining track proof fixture\n' >"${tmp_abs}/redefine_track.md"
  printf '%s | HEURISTIC | %s/active/** | TRACK_LOCAL_BAD_TOKEN |  | forbidden global redefine proof | proof fixture blocker\n' "$global_id" "$tmp_rel" >"${tmp_abs}/redefine_track.md.ci.tsv"

  local out status digest_path

  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "${SCRIPT_DIR}/doctrine_scan.sh" --track-doc "${tmp_rel}/active_track.md" 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  if [[ "$status" -eq 0 ]]; then
    echo "addendum proof failed (active addendum): expected nonzero scan status" >&2
    echo "$out" >&2
    return 1
  fi
  assert_contains "$out" "TRACK-LOCAL-BAD  FAIL  1" "active addendum" || return 1

  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "${SCRIPT_DIR}/doctrine_scan.sh" 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  if [[ "$status" -ne 0 ]]; then
    echo "addendum proof failed (no opt-in): expected global scan to pass" >&2
    echo "$out" >&2
    return 1
  fi
  assert_not_contains "$out" "TRACK-LOCAL-BAD" "no opt-in" || return 1

  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "${SCRIPT_DIR}/doctrine_scan.sh" --track-doc "${tmp_rel}/clean_track.md" 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  if [[ "$status" -ne 0 ]]; then
    echo "addendum proof failed (inactive detached): expected clean active track to pass" >&2
    echo "$out" >&2
    return 1
  fi
  assert_not_contains "$out" "INACTIVE-BAD" "inactive detached" || return 1

  _errexit_was_on=0
  errexit_is_on && _errexit_was_on=1
  set +e
  out="$(bash "${SCRIPT_DIR}/doctrine_scan.sh" --track-doc "${tmp_rel}/redefine_track.md" 2>&1)"
  status=$?
  restore_errexit "$_errexit_was_on"
  if [[ "$status" -eq 0 ]]; then
    echo "addendum proof failed (forbidden global redefine): expected nonzero status" >&2
    echo "$out" >&2
    return 1
  fi
  assert_contains "$out" "track addendum redefines global scan-id '${global_id}'" "forbidden global redefine" || return 1

  digest_path="${tmp_rel}/active_digest.md"
  out="$(bash "${SCRIPT_DIR}/gen_digest.sh" --track-doc "${tmp_rel}/active_track.md" --output "$digest_path" 2>&1)" || {
    echo "$out" >&2
    return 1
  }
  out="$(bash "${SCRIPT_DIR}/gen_digest.sh" --track-doc "${tmp_rel}/active_track.md" --output "$digest_path" --check 2>&1)" || {
    echo "$out" >&2
    return 1
  }
  local digest_text
  digest_text="$(<"${REPO_ROOT}/${digest_path}")"
  assert_contains "$digest_text" "ACTIVE_TRACK_ONLY_TYPE" "track digest active allow" || return 1
  assert_contains "$digest_text" "TRACK-LOCAL-BAD" "track digest active scan" || return 1
  assert_not_contains "$digest_text" "INACTIVE_TRACK_ONLY_TYPE" "track digest inactive allow" || return 1
  assert_not_contains "$digest_text" "INACTIVE-BAD" "track digest inactive scan" || return 1

  echo "doctrine_scan --prove-addendum: PASS"
}

emit_report() {
  local sha ts verdict
  sha="$(git -C "$REPO_ROOT" rev-parse --short HEAD 2>/dev/null || echo unknown)"
  ts="$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date -u)"

  if [[ "$scanner_errors" -gt 0 ]]; then
    verdict="FAIL"
  elif [[ "$hard_failures" -gt 0 ]]; then
    verdict="FAIL"
  elif [[ "$inspect_flags" -gt 0 ]]; then
    verdict="INSPECT"
  else
    verdict="PASS"
  fi

  echo "DOCTRINE SCAN REPORT  (commit ${sha}, ${ts})"
  echo "  scanner self-test: ${selftest_status}"
  if [[ "$PR_DELTA_MODE" -eq 1 ]]; then
    local base_short head_short
    base_short="$(git -C "$REPO_ROOT" rev-parse --short "${PR_BASE_SHA}" 2>/dev/null || echo "$PR_BASE_SHA")"
    head_short="$(git -C "$REPO_ROOT" rev-parse --short "${PR_HEAD_SHA}" 2>/dev/null || echo "$PR_HEAD_SHA")"
    echo "  scan mode: PR delta (${base_short}..${head_short})"
    echo "  reliable scope: whole-tree"
    echo "  heuristic scope: changed files / changed lines"
  else
    echo "  scan mode: whole-tree"
    echo "  reliable scope: whole-tree"
    echo "  heuristic scope: whole-tree"
  fi
  if [[ -n "$TRACK_DOC_REL" ]]; then
    echo "  track doc: ${TRACK_DOC_REL}"
    if [[ -n "$ADDENDUM_SCANS_TSV" && -f "$ADDENDUM_SCANS_TSV" ]]; then
      echo "  track addendum: ${TRACK_DOC_REL}.ci.tsv"
    else
      echo "  track addendum: none (global floor only)"
    fi
  fi
  echo "  --- results ---"
  local entry
  for entry in "${REPORT_LINES[@]}"; do
    echo "  ${entry}"
  done
  echo "  --- summary ---"
  echo "  hard failures: ${hard_failures}   inspect flags: ${inspect_flags}   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only"
  echo "DOCTRINE-SCAN-VERDICT: ${verdict}  failures=${hard_failures} inspect=${inspect_flags} selftest=${selftest_status}"

  echo "  --- inspect justifications ---"
  if [[ ${#JUSTIFS[@]} -gt 0 ]]; then
    echo "  justifications file present with ${#JUSTIFS[@]} entries"
  else
    echo "  no justifications file present (INSPECTs report as unresolved)"
  fi
  if (( inspect_flags > 0 )); then
    echo "  INSPECT findings present; per-INSPECT status: check justifications file or report for unresolved"
    echo "  INSPECT-JUSTIFICATION:"
    echo "    scan-id: <HEURISTIC_SCAN_ID>"
    echo "    location: <file:line or symbol>"
    if [[ ${#JUSTIFS[@]} -gt 0 ]]; then
      echo "    status: provided via inspect_justifications.tsv"
    else
      echo "    status: unresolved"
    fi
  fi
}

main() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --pr-delta)
        PR_DELTA_MODE=1
        PR_BASE_SHA="${2:-}"
        if [[ -n "${3:-}" && "${3:-}" != --* ]]; then
          PR_HEAD_SHA="$3"
        else
          PR_HEAD_SHA="HEAD"
        fi
        if [[ -z "$PR_BASE_SHA" ]]; then
          die_scanner "missing base SHA for --pr-delta"
          emit_report
          exit 1
        fi
        shift
        shift
        if [[ $# -gt 0 && "${1:-}" != --* ]]; then
          shift
        fi
        ;;
      --track-doc)
        TRACK_DOC="${2:-}"
        if [[ -z "$TRACK_DOC" ]]; then
          die_scanner "missing path for --track-doc"
          emit_report
          exit 1
        fi
        shift 2
        ;;
      --prove-addendum)
        PROVE_ADDENDUM=1
        shift
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        usage
        die_scanner "unknown argument: $1"
        emit_report
        exit 1
        ;;
    esac
  done

  if [[ "$PROVE_ADDENDUM" -eq 1 ]]; then
    run_addendum_proof
    exit $?
  fi

  if [[ "$PR_DELTA_MODE" -eq 1 ]]; then
    if ! git -C "$REPO_ROOT" rev-parse --verify "${PR_BASE_SHA}^{commit}" >/dev/null 2>&1; then
      die_scanner "invalid base SHA for --pr-delta: ${PR_BASE_SHA}"
      emit_report
      exit 1
    fi
    if ! git -C "$REPO_ROOT" rev-parse --verify "${PR_HEAD_SHA}^{commit}" >/dev/null 2>&1; then
      die_scanner "invalid head SHA for --pr-delta: ${PR_HEAD_SHA}"
      emit_report
      exit 1
    fi
    load_pr_delta_line_map
  fi

  if ! command -v rg >/dev/null 2>&1; then
    die_scanner "ripgrep (rg) not found on PATH"
    emit_report
    exit 1
  fi
  if [[ ! -f "$SCANS_TSV" ]]; then
    die_scanner "missing scans.tsv"
    emit_report
    exit 1
  fi

  load_and_validate_allowlists
  load_global_keys
  resolve_track_addendum
  validate_track_scan_addendum
  validate_track_allow_addendum
  load_justifications
  if [[ "$scanner_errors" -eq 0 ]]; then
    run_scans
    run_inventory_drift_gate
    run_closing_rung_stock_gates
  fi
  emit_report

  if [[ "$scanner_errors" -gt 0 || "$hard_failures" -gt 0 ]]; then
    exit 1
  fi
  exit 0
}

main "$@"
