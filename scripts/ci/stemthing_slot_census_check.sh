#!/usr/bin/env bash
# STEMTHING-SLOT-CENSUS — reproducibility checker for scripts/ci/stemthing_slot_census.tsv.
#
# Sibling of observation_bypass_census.sh / plan_struct_typing_census.sh /
# write_door_band_delta_census.sh: the census must be re-derivable, not a frozen claim.
#
#   --check    (CI-safe, default) reconcile the TSV against the pinned universe:
#              every universe file appears in exactly one row's non-[analysis] evidence;
#              zero universe files unassigned; verdict classes closed-set; BLOCKER count 0
#              unless the Tier-2 ruling is amended.
#   --harvest  (LOCAL ONLY; refuses on dirty tree) re-derive the universe: temporarily
#              deprecate SlotIndex::{new,raw,as_usize} with note CENSUS, cargo check the
#              workspace, keep only warnings naming SlotIndex+CENSUS, restore the file,
#              and diff against the pinned universe. A diff means the census is STALE
#              (SlotIndex consumers changed — e.g. 6.4 implementation) and must be
#              re-reconciled, not hand-edited.
set -euo pipefail
cd "$(dirname "$0")/../.."
MODE="${1:---check}"

if [[ "$MODE" == "--harvest" ]]; then
  if [[ -n "$(git status --porcelain crates/simthing-core/src/slot_index.rs)" ]]; then
    echo "CENSUS-HARVEST-VERDICT: FAIL(dirty-slot-index)"; exit 1
  fi
  trap 'git checkout -q crates/simthing-core/src/slot_index.rs' EXIT
  python - <<'PY'
p = 'crates/simthing-core/src/slot_index.rs'
s = open(p, encoding='utf-8').read()
for m in ['pub fn new(raw: u32) -> Self {', 'pub fn raw(self) -> u32 {', 'pub fn as_usize(self) -> usize {']:
    s = s.replace(m, '#[deprecated(note = "CENSUS")]\n    ' + m, 1)
open(p, 'w', encoding='utf-8').write(s)
PY
  touch crates/simthing-core/src/lib.rs
  cargo check --workspace --message-format=short 2>&1 \
    | { grep -E "warning: use of deprecated.*SlotIndex.*CENSUS" || true; } \
    | grep -oE "crates[\\\\/][a-z-]+[\\\\/](src|tests)[\\\\/][a-zA-Z_0-9/\\\\]+\.rs" \
    | sort -u \
    | sed 's|crates[\\/]simthing-||; s|[\\/]src[\\/]|:|' | tr '\134' '/' > /tmp/census_universe_fresh.txt
  if diff -u scripts/ci/stemthing_slot_census_universe.txt /tmp/census_universe_fresh.txt; then
    echo "CENSUS-HARVEST-VERDICT: PASS (universe unchanged, $(wc -l < /tmp/census_universe_fresh.txt) files)"
  else
    echo "CENSUS-HARVEST-VERDICT: STALE (universe drifted; re-reconcile the TSV, do not hand-edit)"; exit 1
  fi
  exit 0
fi

python - <<'PY'
import re, sys
universe = set(l.strip() for l in open('scripts/ci/stemthing_slot_census_universe.txt', encoding='utf-8') if l.strip())
rows, assigned, verdicts = 0, {}, set()
CLOSED = {"REBIND-FREE","REBIND-AT-BOUNDARY","ORDER-PIN","REPLAY-RECORD","BLOCKER"}
blockers = 0
for line in open('scripts/ci/stemthing_slot_census.tsv', encoding='utf-8'):
    if line.startswith('#') or '\t' not in line: continue
    cols = line.rstrip('\n').split('\t')
    rows += 1
    verdict = cols[-1].strip()
    verdicts.add(verdict)
    if verdict == "BLOCKER": blockers += 1
    # non-[analysis] evidence tokens only: strip everything from any [analysis] marker onward per token group
    ev = cols[1]
    core_ev = ev.split('+ [analysis]')[0] if '+ [analysis]' in ev else ('' if ev.strip().startswith('[analysis]') else ev)
    for tok in re.findall(r'[a-z-]+:[A-Za-z_0-9/]+\.rs', core_ev):
        if tok in universe:
            assigned.setdefault(tok, []).append(cols[0])
bad = {f: r for f, r in assigned.items() if len(r) > 1}
missing = sorted(universe - set(assigned))
unknown_verdicts = verdicts - CLOSED
ok = not bad and not missing and not unknown_verdicts and blockers == 0
print(f"rows={rows} universe={len(universe)} assigned={len(assigned)} dup={len(bad)} missing={len(missing)} blockers={blockers}")
if bad: print("DUPS:", bad)
if missing: print("MISSING:", missing)
if unknown_verdicts: print("UNKNOWN VERDICTS:", unknown_verdicts)
print("CENSUS-CHECK-VERDICT:", "PASS" if ok else "FAIL")
sys.exit(0 if ok else 1)
PY
