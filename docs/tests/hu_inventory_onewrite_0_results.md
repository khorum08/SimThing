# HU-INVENTORY-ONEWRITE-0 — STOP (no-op regen impossible)

**STOPPED.** Did not force-fit policy or rewrite `test_lifecycle_boundary_rows.tsv`. No green merge candidate.

ORIENT-RECEIPT: `4921e84c2b89` · stamp `27baba147e3f156c`.

## Pure derivation rule tried

Inventory row → boundary row **iff** `superseding_boundary` ∈ `test_lifecycle_boundaries.tsv` (B-T*).
Fields from inventory+policy only (no free-text carry from committed boundary table).

## Live counts

| Surface | N |
|---|---|
| inventory | 1037 |
| committed boundary_rows | **651** (not 629) |
| policy B-T* | 78 |
| inv ∩ policy | 744 |
| inv with **non-policy** boundary id | **293** |
| pure-derived | 744 |

## Delta (regen ≠ no-op)

| Bucket | N |
|---|---|
| pure-derived **ADD** vs committed | **93** (all `B-T7-SEAL-PROOF-FIXTURES` fixtures) |
| committed only | 0 |
| intersection field rewrites | **167** (`note` 167, `confidence` 46, `promotion_required` 9) |
| structural class/tier key mismatches | **0** |

### Non-policy `superseding_boundary` (293) — no B-T* join

TP-STUDIO-CLAUSE-PICKER-CLASS-0 (40), TP-ADMITTED-CLAUSE-API-CLASS-0 (28), CLEARANCE-ADMITTED-SCOPE-GAP-0 (28), DA-TREEVERIFY-0 (27), OH-IMMUTABLE-EVIDENCE-0 (26), OH-TRIAGE / OH-CLEARANCE-ROUTER / TP-WORKSHOP-CANDIDATE (24 each), CC-SWEEP-* / CC-RELAY / OH-DOCS-SUNSET / HU-CLEARANCE-DSL-0 (remainder).

### Free-text not policy-keyed

Distinct notes per (class, boundary_id) in committed table: oracle-parity/**48**, behavior-regression/**8**, golden-byte/**5**, seal-proof/**4**. Only 10/651 notes equal inventory `note`.

### Pre-existing

`test_lifecycle_boundary_check.sh` already **FAIL**: missing 386 inv→boundary maps (= 93 + 293).

## Not done

No boundary_rows rewrite · no invented OH/TP policy rows · no drift weaken · no “green” one-write PR.

## DA decisions to unstick

1. **293 non-policy:** remap inv → B-T*, or mint policy rows, or change who needs a boundary_row (fence).
2. **93 seal-proof gaps:** accept derive-ADD once note policy fixed.
3. **notes/confidence/promo:** (a) inventory owns free text 1:1, (b) keep hand boundary (blocks pure one-write), or (c) modal canned notes (lossy).

Reproduce: `python scripts/ci/_tmp_onewrite_delta.py` (scratch analyzer, not a gate).
