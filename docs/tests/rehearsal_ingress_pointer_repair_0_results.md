# 0088 current-active-track pointer referee repair

PROBATION / proof-present / OPEN / UNMERGED; return to Orchestration only.
Revised dispatch Board 5634398555; DA countersign 5634372709;
prior bounded dispatch 5634333792; HD-RECEIPT 70df16e1babd.
Coding ORIENT-RECEIPT 28f56884d309; stamp 73e54d6b56b7266b;
inherited 48-anchor ACK Board 5612017867.
Current source base: a6099f37c6aeaca25c7769e4cc8ad5c2b4b619f0.
PR #2041 remains unchanged as the blocked certification record.

## Committed RED

The paused bookkeeping-only e1aecc4fd4e78a47acee1bf42ada260710b83135 was
rebased onto current master, whose intervening commits restored the referee
following a connector incident. No assertion changed in that bookkeeping rebase.
The focused test at e85faa371171721bedcd94d08627a3999e56935e then failed:

```text
board_and_orientation_render_property_admission_inventory
Board pointer must equal the design doc's active open rung
left:  String("0088-INGRESS-FIDELITY-0")
right: String("none")
```

The old expected-pointer lookup selects the completed 0.0.8.7 design document;
Board instead reads the current orientation. The source base reproduces the
mismatch even though its restored referee formatting differs from the earlier
#2041 source base. The RED is an actual committed, focused execution.

## Bounded correction

Only the named Board/orientation referee changes. It invokes the existing
read-only `gen_orientation.sh --check`, which resolves `active_track.txt` and
validates the checked-in orientation against that active workplan. It then
compares Board's pointer to the current, freshly validated orientation pointer.
No prior-track path or current rung literal selects the expected value.

The existing backtick extraction and completed-track `none` fallback are retained:
a quoted active rung stays that rung, quoted `none` stays `none`, and unquoted
completed-track text maps to `none`. A missing pointer row or stale orientation
fails explicitly; another active track cannot be hidden by a completed 0087 row.
Generator code and output are unchanged. This consumes the existing pointer
authority instead of duplicating its track/ladder selection rules in the referee.

Property-admission generator freshness, orientation count rendering, Board
anchored/unobserved/total counts and the live dark-inventory count assertions
remain intact. Every other helper and referee in this file is byte-identical to
the current source base. No added test or ingress class trigger is needed.

## Consumption and certification

Only the explicitly consumed Board/orientation referee moves from the pen to the
active inventory, retaining its identity, original birth/class/verdict and the
single DSU 0 -> 1 renewal demonstrated by #2041. The downstream-utility names this
1.1 certification and separately authorized repair; no other row is renewed.

Focused reproduction: `cargo test -p simthing-clausething --test
anchor_disposition_admission_0 board_and_orientation_render_property_admission_inventory
-- --exact --test-threads=1`. Full certification reruns ClauseThing and mapeditor
with `--no-fail-fast -- --test-threads=1`, covering all 14 surviving
`rehearsal_ingress_*` witnesses. The PR/Board return binds final focused/full
results, lifecycle/inventory, local scan and hosted evidence to its final
committed `tested_code_sha`.

No production semantics, spec/engine, B2, census, gate/workflow/router/class table,
Cargo, assertion bypass/ignore or graduation edit is made. Per the countersign,
Clearance is expected to remain `DA-RESERVE(unclassified-scope)`. Orchestration
relays the completed GREEN repair to DA as the named harness-residual increment;
the final 1.1 D-row stamp is separate. Coding does not contact DA or self-merge.
