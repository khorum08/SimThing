#!/usr/bin/env bash
# CI-A-SELFTEST-INSPECT-REPAIR-0 — real INSPECT spam bound checker.
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

usage() {
  echo "usage: $0 <branch>"
  echo "       $0 --prove"
  exit 2
}

branch="${1:-}"

if [[ -z "$branch" ]]; then
  usage
fi

# --- analysis helpers (real git history delta counting) ---

inspect_added_pattern='(faction|combat|terran|pirate|diplomacy|\.kind\b|data\[[0-9]+\])'
reliable_added_pattern='(unsafe fn|pub fn (from_boundary|for_kernel|for_boundary))'

get_base() {
  local br="$1"
  # Robust for fresh git init (may be main or the branch itself) and no remotes
  git merge-base origin/master "$br" 2>/dev/null || \
  git merge-base origin/main "$br" 2>/dev/null || \
  git merge-base master "$br" 2>/dev/null || \
  git merge-base main "$br" 2>/dev/null || \
  git rev-list --max-parents=0 HEAD 2>/dev/null || echo "HEAD~10"
}

count_branch_introduced_inspect() {
  local br="$1"
  local base
  base="$(get_base "$br")"
  local n
  n=$(git log --no-color --pretty=format: -p "$base..$br" -- . 2>/dev/null |
    awk '/^\+[^+].*(faction|combat|terran|pirate|diplomacy|\.kind\>|data\[[0-9]+\])/ { n++ } END { print n + 0 }')
  printf '%s' "$n"
}

same_symbol_multiple_heuristics() {
  local br="$1"
  local base
  base="$(get_base "$br")"
  local hits
  hits="$(git log --no-color --pretty=format: -p "$base..$br" -- . 2>/dev/null |
    awk '
      /^\+\+\+ b\// {
        file = substr($0, 7)
        next
      }
      /^\+[^+]/ {
        line = substr($0, 2)
        scan = ""
        if (line ~ /(faction|combat|terran|pirate|diplomacy)/) scan = "HEUR-SEMANTIC"
        if (line ~ /\.kind\>/) scan = "HEUR-KIND"
        if (line ~ /data\[[0-9]+\]/) scan = "HEUR-DATA"
        if (scan == "") next
        symbol = file
        if (match(line, /fn[ \t]+[A-Za-z_][A-Za-z0-9_]*/)) {
          symbol = file ":" substr(line, RSTART, RLENGTH)
        }
        print symbol "|" scan
      }
    ' |
    sort -u |
    awk -F'|' '{ seen[$1]++ } END { for (s in seen) if (seen[s] >= 2) n++; print n + 0 }' ||
    echo 0)"
  [[ "$hits" -gt 0 ]]
}

inspect_rising_while_reliable_open() {
  local br="$1"
  local base
  base="$(get_base "$br")"
  local reliable_seen=0
  local reliable_open=0
  local inspect_rise=0
  local line
  while IFS= read -r line; do
    case "$line" in
      "+++"*) continue ;;
      "---"*) continue ;;
      +*)
        if [[ "$line" =~ unsafe[[:space:]]+fn || "$line" =~ pub[[:space:]]+fn[[:space:]]+(from_boundary|for_kernel|for_boundary) ]]; then
          reliable_seen=1
          reliable_open=1
        fi
        if [[ "$line" =~ (faction|combat|terran|pirate|diplomacy|\.kind|data\[[0-9]+\]) && "$reliable_open" -eq 1 ]]; then
          inspect_rise=$((inspect_rise + 1))
        fi
        ;;
      -*)
        if [[ "$line" =~ unsafe[[:space:]]+fn || "$line" =~ pub[[:space:]]+fn[[:space:]]+(from_boundary|for_kernel|for_boundary) ]]; then
          reliable_open=0
        fi
        ;;
    esac
  done < <(git log --reverse --no-color --pretty=format: -p "$base..$br" -- . 2>/dev/null || true)
  [[ "$reliable_open" -eq 1 && "$reliable_seen" -eq 1 && "$inspect_rise" -gt 0 ]]
}

new_temp_repo() {
  local prefix="$1"
  local td
  td="$(mktemp -d "/tmp/${prefix}-XXXX")" || exit 1
  git -C "$td" init -q
  git -C "$td" config user.email "ci@example.invalid"
  git -C "$td" config user.name "CI Proof"
  git -C "$td" commit --allow-empty -m "init" -q
  printf '%s' "$td"
}

commit_file() {
  local td="$1"
  local path="$2"
  local content="$3"
  local message="$4"
  printf '%s\n' "$content" > "${td}/${path}"
  git -C "$td" add "$path"
  git -C "$td" commit -q -m "$message"
}

run_proof_case() {
  local td="$1"
  local br="$2"
  local expected_rc="$3"
  local label="$4"
  local out
  local rc
  out=$(cd "$td" && "${BASH:-bash}" "$SCRIPT_DIR/inspect_spam_check.sh" "$br" 2>&1)
  rc=$?
  echo "${label}: ${out}"
  echo "RC=${rc}"
  if [[ "$rc" -ne "$expected_rc" ]]; then
    echo "${label}: expected RC=${expected_rc}, got RC=${rc}"
    return 1
  fi
  return 0
}

# --- main ---

if [[ "$branch" == "--prove" ]]; then
  echo "=== INSPECT-SPAM-PROOF CASES ==="
  proof_failed=0

  # 1. single-gray-zone -> OK
  td=$(new_temp_repo "spam-proof-gray")
  git -C "$td" checkout -q -b proof-single-gray-zone
  commit_file "$td" "gray.rs" "gray zone comment // faction in doc only" "single gray zone INSPECT"
  run_proof_case "$td" "proof-single-gray-zone" 0 "single-gray-zone" || proof_failed=1
  rm -rf "$td"

  # 2. symbol-walking -> SPAM (same symbol under >=2 HEUR)
  td=$(new_temp_repo "spam-proof-symbol")
  git -C "$td" checkout -q -b proof-symbol-walking
  commit_file "$td" "s.rs" 'fn x() { /* faction */ }' "HEUR-SEMANTIC"
  commit_file "$td" "s.rs" 'fn x() { let k = thing.kind; }' "HEUR-KIND on same sym"
  run_proof_case "$td" "proof-symbol-walking" 1 "symbol-walking" || proof_failed=1
  rm -rf "$td"

  # 3. >3 branch-introduced INSPECT -> SPAM
  td=$(new_temp_repo "spam-proof-many")
  git -C "$td" checkout -q -b proof-many-inspect
  for i in 1 2 3 4 5; do
    commit_file "$td" "f${i}.rs" "ins${i}: faction here" "INSPECT ${i}"
  done
  run_proof_case "$td" "proof-many-inspect" 1 ">3-inspect" || proof_failed=1
  rm -rf "$td"

  # 4. branch-name alias quarantine -> OK (name alone is not proof)
  td=$(new_temp_repo "spam-proof-alias")
  git -C "$td" checkout -q -b spam
  run_proof_case "$td" "spam" 0 "branch-name-alias-only" || proof_failed=1
  rm -rf "$td"

  # 5. INSPECT rising while RELIABLE FAIL open -> SPAM
  td=$(new_temp_repo "spam-proof-rising")
  git -C "$td" checkout -q -b proof-rising-while-reliable
  commit_file "$td" "bad.rs" 'unsafe fn bad() {}' "RELIABLE open"
  commit_file "$td" "i1.rs" "faction1" "INSPECT 1"
  commit_file "$td" "i2.rs" "faction2" "INSPECT 2"
  commit_file "$td" "i3.rs" "faction3" "INSPECT 3 rising"
  run_proof_case "$td" "proof-rising-while-reliable" 1 "rising-while-reliable" || proof_failed=1
  rm -rf "$td"

  if [[ "$proof_failed" -ne 0 ]]; then
    echo "INSPECT-SPAM-PROOF: FAIL"
    exit 1
  fi
  echo "INSPECT-SPAM-PROOF: PASS"
  exit 0
fi

# Real analysis on provided branch name
n_ins=$(count_branch_introduced_inspect "$branch")
if [[ "$n_ins" -gt 3 ]]; then
  echo "INSPECT-SPAM-CHECK: SPAM (>3 branch-introduced)"
  exit 1
fi

if same_symbol_multiple_heuristics "$branch"; then
  echo "INSPECT-SPAM-CHECK: SPAM (same symbol >=2 HEUR)"
  exit 1
fi

if inspect_rising_while_reliable_open "$branch"; then
  echo "INSPECT-SPAM-CHECK: SPAM (INSPECT rising while RELIABLE open)"
  exit 1
fi

echo "INSPECT-SPAM-CHECK: OK"
exit 0
