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

declare -a REPORT_LINES=()

die_scanner() {
  echo "scanner/data error: $*" >&2
  scanner_errors=$((scanner_errors + 1))
}

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
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

  case "$relpath" in
    allow/sealed_producers.txt)
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
    allow/inert_buffer_handles.txt)
      if [[ "$door" != "inert-util" ]]; then
        die_scanner "${relpath}:${line_num}: invalid door-class '${door}' (inert_buffer_handles: inert-util only)"
      fi
      ;;
    allow/kernel_surface.txt)
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
    line="$(trim "$line")"
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

parse_fields() {
  local line="$1"
  local -n _out="$2"
  local rest="$line"
  _out=()
  local i part
  for ((i = 0; i < 7; i++)); do
    if [[ "$i" -eq 6 ]]; then
      part="$(trim "$rest")"
      _out+=("$part")
      break
    fi
    if [[ "$rest" != *"${FIELD_SEP}"* ]]; then
      return 1
    fi
    part="${rest%%"${FIELD_SEP}"*}"
    _out+=("$(trim "$part")")
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
      part="$(trim "$rest")"
      _out+=("$part")
      break
    fi
    if [[ "$rest" != *"${FIELD_SEP}"* ]]; then
      return 1
    fi
    part="${rest%%"${FIELD_SEP}"*}"
    _out+=("$(trim "$part")")
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
    pat="$(trim "$pat")"
    [[ -z "$pat" ]] && continue
    if printf '%s\n' "$content" | rg -q -e "$pat" 2>/dev/null; then
      return 0
    fi
    if [[ "$content" != "$line" ]] && printf '%s\n' "$line" | rg -q -e "$pat" 2>/dev/null; then
      return 0
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
  local cfg_line=0
  local entry n
  while IFS= read -r entry; do
    [[ -z "$entry" ]] && continue
    n="${entry%%:*}"
    [[ "$n" =~ ^[0-9]+$ ]] || continue
    if [[ "$n" -lt "$line_num" && "$n" -gt "$cfg_line" ]]; then
      cfg_line="$n"
    fi
  done < <(rg -n '#\[cfg\(test\)\]' "$abs" 2>/dev/null || true)
  [[ "$cfg_line" -eq 0 ]] && return 1
  sed -n "${cfg_line},$((cfg_line + 4))p" "$abs" | rg -q 'mod tests' 2>/dev/null || return 1
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
  local current_file=""
  local line
  while IFS= read -r line; do
    line="${line//$'\r'/}"
    if [[ "$line" =~ ^\+\+\+\ b/(.+)$ ]]; then
      current_file="${BASH_REMATCH[1]}"
      current_file="$(normalize_match_path "$current_file")"
    elif [[ "$line" =~ ^@@\ .*\+([0-9]+)(,([0-9]+))?\ @@ ]]; then
      local start="${BASH_REMATCH[1]}"
      local count="${BASH_REMATCH[3]:-1}"
      local i
      for ((i = 0; i < count; i++)); do
        PR_DELTA_LINE_MAP["${current_file}:$((start + i))"]=1
      done
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
    rg_args+=(-g '!**/tests/**' -g '!**/test/**' -g '!**/*_tests.rs')
  else
    rg_args+=(-g "$target_glob")
    if [[ "$severity" == "HEURISTIC" ]]; then
      rg_args+=(-g '!**/tests/**' -g '!**/test/**' -g '!**/*_tests.rs')
    fi
    search_paths=(".")
  fi

  local rg_out=""
  local rg_status=0
  set +e
  if [[ "$use_relative_paths" -eq 1 ]]; then
    rg_out="$(cd "$REPO_ROOT" && rg "${rg_args[@]}" "${search_paths[@]}" 2>&1)"
  else
    rg_out="$(cd "$REPO_ROOT" && rg "${rg_args[@]}" "${search_paths[0]}" 2>&1)"
  fi
  rg_status=$?
  set -e

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
  set +e
  out="$(python "$script" "$mode" 2>&1)"
  py_status=$?
  set -e

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
    set +e
    rg -U --multiline -q -e "$pattern" "${REPO_ROOT}/${f}" 2>/dev/null
    rg_status=$?
    set -e
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

run_scans() {
  local line_num=0
  while IFS= read -r line || [[ -n "$line" ]]; do
    line_num=$((line_num + 1))
    line="$(trim "$line")"
    [[ -z "$line" || "$line" == \#* ]] && continue

    local fields=()
    if ! parse_fields "$line" fields; then
      die_scanner "scans.tsv:${line_num}: malformed record (expected 7 fields)"
      continue
    fi
    if [[ "${#fields[@]}" -ne 7 ]]; then
      die_scanner "scans.tsv:${line_num}: malformed record (expected 7 fields, got ${#fields[@]})"
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

    if [[ -z "$scan_id" || -z "$severity" || -z "$pattern" || -z "$doctrine_ref" ]]; then
      die_scanner "scans.tsv:${line_num}: empty required field in '${scan_id}'"
      continue
    fi

    case "$severity" in
      RELIABLE|HEURISTIC) ;;
      *)
        die_scanner "scans.tsv:${line_num}: invalid severity '${severity}' in '${scan_id}'"
        continue
        ;;
    esac

    if [[ "$severity" == "RELIABLE" && -z "$promotion_blocker" ]]; then
      die_scanner "scans.tsv:${line_num}: RELIABLE scan '${scan_id}' missing promotion-blocker"
      continue
    fi

    if [[ "$pattern" == @REQUIRE:* ]]; then
      require_mode=1
      pattern="${pattern#@REQUIRE:}"
    elif [[ "$pattern" == @ALLOWLIST:* ]]; then
      allowlist_mode="${pattern#@ALLOWLIST:}"
    fi

    local matches=()
    local run_status=0
    if [[ -n "$allowlist_mode" ]]; then
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
  done <"$SCANS_TSV"
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
  echo "  --- results ---"
  local entry
  for entry in "${REPORT_LINES[@]}"; do
    echo "  ${entry}"
  done
  echo "  --- summary ---"
  echo "  hard failures: ${hard_failures}   inspect flags: ${inspect_flags}   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only"
  echo "DOCTRINE-SCAN-VERDICT: ${verdict}  failures=${hard_failures} inspect=${inspect_flags} selftest=${selftest_status}"
}

main() {
  if [[ "${1:-}" == "--pr-delta" ]]; then
    PR_DELTA_MODE=1
    PR_BASE_SHA="${2:-}"
    PR_HEAD_SHA="${3:-HEAD}"
    if [[ -z "$PR_BASE_SHA" ]]; then
      die_scanner "missing base SHA for --pr-delta"
      emit_report
      exit 1
    fi
    shift 3
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
  if [[ "$scanner_errors" -eq 0 ]]; then
    run_scans
  fi
  emit_report

  if [[ "$scanner_errors" -gt 0 || "$hard_failures" -gt 0 ]]; then
    exit 1
  fi
  exit 0
}

main "$@"
