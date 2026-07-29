#!/usr/bin/env bash
# Shared brace-balanced `#[cfg(test)] mod tests` span helpers (WRITE-DOOR remand-4).
# Sourced by censuses that must exclude in-module test hits while keeping
# post-closing-brace production hits visible.
#
# shellcheck shell=bash

# Net `{`/`}` delta for one source line (// comments stripped; string-agnostic heuristic).
line_brace_delta() {
  local s="$1"
  if [[ "$s" == *"//"* ]]; then
    s="${s%%//*}"
  fi
  local i ch
  local delta=0
  for ((i = 0; i < ${#s}; i++)); do
    ch="${s:i:1}"
    if [[ "$ch" == "{" ]]; then
      delta=$((delta + 1))
    elif [[ "$ch" == "}" ]]; then
      delta=$((delta - 1))
    fi
  done
  printf '%s' "$delta"
}

# Emit "open_line close_line" pairs for every brace-balanced `#[cfg(test)] mod tests`.
cfg_test_mod_spans() {
  local file="$1"
  [[ -f "$file" ]] || return 0
  local -a lines=()
  mapfile -t lines < "$file"
  local n=${#lines[@]}
  local i=0
  while ((i < n)); do
    local l="${lines[i]}"
    if [[ ! "$l" =~ ^[[:space:]]*#\[[Cc]fg\(test\)\] ]]; then
      i=$((i + 1))
      continue
    fi
    local j
    local mod_idx=-1
    for ((j = i; j < n && j <= i + 4; j++)); do
      if [[ "${lines[j]}" =~ mod[[:space:]]+tests ]]; then
        mod_idx=$j
        break
      fi
    done
    if ((mod_idx < 0)); then
      i=$((i + 1))
      continue
    fi
    local k
    local open_idx=-1
    local depth=0
    for ((k = mod_idx; k < n; k++)); do
      local delta
      delta="$(line_brace_delta "${lines[k]}")"
      if ((open_idx < 0)); then
        if [[ "${lines[k]}" == *"{"* ]]; then
          open_idx=$k
          depth=$delta
          if ((depth <= 0)); then
            printf '%s %s\n' "$((open_idx + 1))" "$((open_idx + 1))"
            break
          fi
        fi
        continue
      fi
      depth=$((depth + delta))
      if ((depth <= 0)); then
        printf '%s %s\n' "$((open_idx + 1))" "$((k + 1))"
        break
      fi
    done
    if ((open_idx >= 0 && depth > 0)); then
      printf '%s %s\n' "$((open_idx + 1))" "$n"
    fi
    i=$((i + 1))
  done
}

# True when path:line sits inside a brace-balanced `#[cfg(test)] mod tests { ... }` body.
# Hits after the module's closing brace remain visible to the census.
in_cfg_test_mod_region() {
  local hit="$1"
  local file line_num
  if [[ "$hit" =~ ^(.+):([0-9]+): ]]; then
    file="${BASH_REMATCH[1]}"
    line_num="${BASH_REMATCH[2]}"
  else
    return 1
  fi
  local open close
  while read -r open close; do
    [[ -z "$open" ]] && continue
    if ((line_num >= open && line_num <= close)); then
      return 0
    fi
  done < <(cfg_test_mod_spans "$file")
  return 1
}

# Drop hits that live inside brace-balanced `#[cfg(test)] mod tests` bodies.
filter_cfg_test_mod_hits() {
  local hits="$1"
  local kept=""
  local hit
  declare -A span_cache=()
  while IFS= read -r hit || [[ -n "$hit" ]]; do
    [[ -z "$hit" ]] && continue
    local file line_num
    if [[ "$hit" =~ ^(.+):([0-9]+): ]]; then
      file="${BASH_REMATCH[1]}"
      line_num="${BASH_REMATCH[2]}"
    else
      kept+="${hit}"$'\n'
      continue
    fi
    if [[ -z "${span_cache[$file]+x}" ]]; then
      span_cache[$file]="$(cfg_test_mod_spans "$file")"
    fi
    local in_mod=1
    local open close
    while read -r open close; do
      [[ -z "$open" ]] && continue
      if ((line_num >= open && line_num <= close)); then
        in_mod=0
        break
      fi
    done <<< "${span_cache[$file]}"
    if [[ "$in_mod" -eq 0 ]]; then
      continue
    fi
    kept+="${hit}"$'\n'
  done <<< "$hits"
  printf '%s' "$kept" | sed '/^$/d' || true
}
