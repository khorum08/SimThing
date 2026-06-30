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

door_class_matches_grammar() {
  local symbol="$1"
  local door="$2"
  local tail
  tail="$(symbol_tail "$symbol")"
  case "$door" in
    read) [[ "$tail" =~ ^read_ ]] ;;
    dispatch) [[ "$tail" =~ ^dispatch_ ]] || [[ "$tail" == "dispatch" ]] ;;
    apply) [[ "$tail" =~ ^apply_ ]] ;;
    cpu_oracle) [[ "$tail" =~ ^cpu_oracle_ ]] ;;
    inert-util) true ;;
    *) false ;;
  esac
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
    case "$door" in
      read|dispatch|apply|cpu_oracle|inert-util) ;;
      *)
        die_scanner "${relpath}:${line_num}: invalid door-class '${door}' (must be read|dispatch|apply|cpu_oracle|inert-util)"
        continue
        ;;
    esac
    if ! door_class_matches_grammar "$symbol" "$door"; then
      die_scanner "${relpath}:${line_num}: symbol '${symbol}' does not match door-class '${door}' grammar"
    fi
  done <"$file"
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

run_rg_scan() {
  local pattern="$1"
  local target_glob="$2"
  local excludes="$3"
  local -n _matches_out="$4"
  _matches_out=()

  local rg_out=""
  local rg_status=0
  set +e
  rg_out="$(cd "$REPO_ROOT" && rg -U --multiline --no-heading --line-number --with-filename -g "$target_glob" -e "$pattern" . 2>&1)"
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
    _matches_out+=("$line")
  done <<<"$rg_out"
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

    if [[ -z "$scan_id" || -z "$severity" || -z "$target_glob" || -z "$pattern" || -z "$doctrine_ref" ]]; then
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
    fi

    local matches=()
    local run_status=0
    if [[ "$require_mode" -eq 1 ]]; then
      run_require_scan "$pattern" "$target_glob" matches
      run_status=$?
    else
      run_rg_scan "$pattern" "$target_glob" "$excludes" matches
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
