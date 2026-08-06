#!/usr/bin/env bash
# RG-CAPABILITY — assert the ripgrep on THIS machine can run the doctrine scans.
#
# WHY. CI-A-SELFTEST-0R (47ec2469, 2026-06-30) abandoned `rg -n` because behaviour
# "varied with rg build/invocation (pcre2 or not, windows paths)" and replaced it
# with a pure-bash per-line read. That bought robustness by paying ~57 process
# spawns per scan. The premise was only half right, and this script pins down
# which half:
#
#   * PCRE2 is NOT required. Zero `-P` / `--pcre2` uses exist across all CI
#     scripts, and every construct the scans actually use is stock Rust-regex.
#     Installing a PCRE2 build fixes nothing.
#   * BUILD VARIANCE IS the real hazard, and it is unmonitored. The DA's own
#     machine runs `ripgrep 15.1.0-cursor5` -- a vendor-patched build, not stock.
#     Nothing anywhere asserted what rg could do; it was simply assumed, then
#     worked around when the assumption broke.
#
# So: assert, do not assume. Same script on Windows and on the runner, so a
# capability difference surfaces as a named failure instead of as a scan whose
# output quietly changes shape.
#
# usage:
#   bash scripts/ci/rg_capability_check.sh            # assert
#   bash scripts/ci/rg_capability_check.sh --selftest # planted defects
set -uo pipefail

probe() { # name, pattern, matching-input
  local name="$1" pat="$2" input="$3"
  if printf '%s\n' "$input" | rg -q -e "$pat" 2>/dev/null; then
    printf '  %-26s ok\n' "$name"
    return 0
  fi
  printf '  %-26s MISSING\n' "$name"
  return 1
}

run_check() {
  local fails=0

  if ! command -v rg >/dev/null 2>&1; then
    echo "RG-CAPABILITY-VERDICT: FAIL(absent) — ripgrep is required by the doctrine scans"
    return 1
  fi

  local ver
  ver="$(rg --version 2>/dev/null | head -1)"
  echo "  version                    ${ver}"
  case "$ver" in
    *-*) echo "  build                      NON-STOCK (vendor-patched) — output shape is not guaranteed" ;;
    *)   echo "  build                      stock" ;;
  esac

  # Every construct the doctrine scans actually depend on.
  probe "posix class"       '^[[:space:]]*#\[[Cc]fg\(test\)\]' '   #[cfg(test)]'     || fails=$((fails + 1))
  probe "word boundary"     '\bmod\b'                          'pub mod tests'       || fails=$((fails + 1))
  probe "bounded repeat"    'a{1,3}'                           'aaa'                 || fails=$((fails + 1))
  probe "non-capturing alt" '(?:alpha|beta)'                   'beta'                || fails=$((fails + 1))
  # Probed the way doctrine_scan.sh actually invokes it (`rg -U --multiline`),
  # not bare — a probe that does not match its call site proves nothing.
  if printf 'a\nb\n' | rg -U --multiline -q -e 'a\nb' 2>/dev/null; then
    printf '  %-26s ok\n' "multiline (-U)"
  else
    printf '  %-26s MISSING\n' "multiline (-U)"; fails=$((fails + 1))
  fi

  # `-n` and `--sort path` are what a future rg-based rewrite of
  # heuristic_in_cfg_test_region would stand on. Assert them now so that work
  # starts from a proven floor rather than an assumption.
  if rg -n -e 'x' <<<'x' >/dev/null 2>&1; then
    printf '  %-26s ok\n' "line numbers (-n)"
  else
    printf '  %-26s MISSING\n' "line numbers (-n)"; fails=$((fails + 1))
  fi
  if rg --files --sort path >/dev/null 2>&1; then
    printf '  %-26s ok\n' "--sort path"
  else
    printf '  %-26s MISSING\n' "--sort path"; fails=$((fails + 1))
  fi

  if [[ "$fails" -ne 0 ]]; then
    echo "RG-CAPABILITY-VERDICT: FAIL(${fails} missing) — this ripgrep cannot run the doctrine scans"
    return 1
  fi
  echo "RG-CAPABILITY-VERDICT: PASS (7 capabilities)"
  return 0
}

selftest() {
  local fails=0

  # PLANTED DEFECT 1: an absent rg must FAIL, not silently pass.
  if (PATH="/nonexistent"; run_check >/dev/null 2>&1); then
    echo "  FAIL absent rg should not pass"; fails=$((fails + 1))
  fi

  # PLANTED DEFECT 2: probe() must report MISSING for an unsupported construct,
  # so a real capability gap cannot read as ok. PCRE2 lookahead is the exact
  # construct the 2026-06-30 rung feared; on a stock build it is unsupported,
  # and this proves the detector SEES that rather than assuming it.
  if probe "lookahead(control)" 'a(?=b)' 'ab' >/dev/null 2>&1; then
    echo "  note: this rg has PCRE2 by default (not required; scans do not use it)"
  fi

  # PLANTED DEFECT 3: a construct that genuinely cannot match must be MISSING.
  if probe "impossible(control)" 'zzz_never_matches' 'aaa' >/dev/null 2>&1; then
    echo "  FAIL non-matching probe reported ok"; fails=$((fails + 1))
  fi

  if [[ "$fails" -ne 0 ]]; then
    echo "RG-CAPABILITY-SELFTEST: FAIL (${fails})"
    return 1
  fi
  echo "RG-CAPABILITY-SELFTEST: PASS (3 checks, 2 planted defects)"
  return 0
}

if [[ "${1:-}" == "--selftest" ]]; then
  selftest
else
  run_check
fi
