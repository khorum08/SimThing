# ANCHOR-DISPOSITION-ADMISSION-0 — total property admission disposition

> **Lifecycle: COMPLETE — DA-GRADUATED / merged #1485 @ 06103edf** — Typed admission disposition, canonical install reporting,
> and deterministic dark-inventory evidence landed. Pending DA review; no merge or
> promotion claimed.

**Date:** 2026-07-28
**Handoff comment:** `5110303912` on issue `#1332`
**ORIENT-RECEIPT:** `16b366e49528`
**HD-RECEIPT:** `34df5f006278`
**Base:** `81ab1508b5e3b68bab4cc76ee6ca3c5588d361d3`
**Tested code SHA:** `3198afc03dc7576f7912eeb1a1d46c3b4566a604`
**Final head / PR:** bound by the draft PR

## Landed contract

- Every resource-bearing property has a total
  `PropertyAdmissionDisposition::{Anchored, Unobserved { reason, source_span_token }}`
  disposition. Omission defaults to `Anchored`; there is no deferred state.
- Clausething accepts the ordinary-property and flow-property forms, preserves the
  explicit reason and source span, and rejects blank unobserved reasons.
- The canonical registry derives a stable `PropertyAdmissionReport`; the ordinary
  hydrate/compile/install path projects it into `SpecSessionState` before install
  success.
- Board JSON/Markdown and generated orientation consume the checked-in canonical
  inventory rather than maintaining a second count.

## Canonical inventory

The canonical Terran Pirate install reports `25` anchored, `0` unobserved, `25`
total resource-bearing properties. The generated artifact is
`scripts/ci/property_admission_inventory.tsv`.

## Proof battery

| Proof | Result |
|---|---|
| Focused admission suite | PASS — 6 passed, 0 failed, 1 generator ignored |
| `cargo build --workspace` | PASS |
| `simthing-core` tests and doctests | PASS — 37 passed |
| `simthing-spec` full battery | PASS |
| `simthing-sim` full battery | PASS — 35 passed |
| Adapter-pinned `simthing-driver` full battery | PASS — 123 passed, 0 failed, 13 intentionally ignored |
| Inventory drift gate | PASS — 1,637 ledger rows, 1,635 discovered, 0 unledgered, 0 stale |
| Execution-status census | PASS — 124 classified, 0 mixed ruled |
| Orientation generation/check | PASS |
| Document budget | PASS |
| Anchor check and digest check | PASS |
| Changed shell syntax checks | PASS |
| `git diff --check` | PASS |
| Final delta `agent_scan` | PASS — 0 hard failures, 0 delta INSPECT |

The repository-wide formatting check remains non-green from pre-existing untouched
format drift, so this rung does not claim a whole-tree `cargo fmt --check` pass.
The broad positive-control doctrine scan likewise retains ambient historical
heuristics; the rung delta scan is clean.

## Scope and fences

Clausething hydration and public property-literal compatibility updates in dependent
crates/tests were required to carry the typed field through the canonical path; they
do not expand simulation semantics. No 5.2 write door, 5.3 table, 5.4+ feature,
WGSL shader, EvalEML opcode, or CPU-side admission decision was added.

The handoff predicted `DA-RESERVE(unclassified-scope)`. Local clearance over the
tested range emits `DA-RESERVE(gate-wiring)` with `DA-TREEVERIFY-PROFILE:
DEEP-TREE`, because the required generated inventory is wired into Board,
orientation, and handoff rendering. PR-bound clearance is the authoritative final
route.

## EML/JIT reach disposition

The EML/JIT path was reviewed for fit. This rung only establishes admission
metadata and reporting, so a shader or opcode extension would cross its fences
without improving the horizon target. No WGSL/EvalEML reach entry is warranted.

## DA status

**COMPLETE** — DA-GRADUATED / merged #1485 @ 06103edf; DA reproduced full corpus green on live GPU.
