# CLEARANCE-UNCLASSIFIED-SCOPE-REDUCTION-0 Results

## Status

**DONE — DA-ADOPTED (2026-07-09, Option A)** — process/policy ruling.  
Router implementation is **not** in this decision; follow-on harness rung implements the new reserve reason + selftests.

## Identity

| Field | Value |
|---|---|
| Rung | `CLEARANCE-UNCLASSIFIED-SCOPE-REDUCTION-0` |
| Kind | Owner/DA process ruling (clearance-router policy) |
| Trigger | #1230 / #1239 admitted proof-present PRs bounced to DA as router friction |

## Tree-verified diagnosis

`clearance_check.sh` route when `detect_classes` returns empty:

```text
emit_verdict reserve "unclassified-scope"
```

That single bucket currently collapses:

1. Genuinely novel / unadmitted work (true DA design residue)
2. Already-admitted implementation lacking a precedented class (router debt)
3. Hygiene / body-field issues (should be `FAIL(remedy)` or orchestrator fix — not a design ruling)

Recent evidence:

| PR | Substance | Router bounce |
|---|---|---|
| #1230 API-1 | DA-admitted limited production API | `engine-scope-violation` class collision, then class fix |
| #1239 picker-0 | DA-admitted narrow UI | `unclassified-scope` (no picker class) |

Principle reaffirmed:

```text
DA decides gates.
Orchestrator merges precedented, admitted, proof-present implementation PRs.
Router gaps should be fixed, not normalized into repeated DA ceremony.
```

## Decision: **Option A** (with Option C later)

### Rationale

- Option B (docs only) is necessary but **insufficient** — agents route on verdict **names**; keeping one `unclassified-scope` token invites the same over-escalation.
- Option C (admitted-envelope registry) is the **strongest** long-term fix but is larger harness surface; do after Option A vocabulary + selftests land.
- Option A splits the token **now** so orchestration can refuse to treat admitted-scope friction as a fresh design relay.

## New / retained verdict vocabulary

| Verdict | Meaning | Orchestrator action |
|---|---|---|
| `DA-RESERVE(unclassified-scope)` | **Retained, narrowed:** resolved non-empty diff with **no** class match **and** no valid admitted-envelope claim | True DA design / admission residue only |
| `DA-RESERVE(admitted-scope-router-gap)` | **New (to implement):** proof-present shape inside a cited prior Owner/DA admission, but no precedented class covers the files yet | **Not** a fresh design ruling. Prefer class-hardening PR; optional one-time merge only if DA explicitly grants with class follow-up required |
| `DA-RESERVE(novelty)` | Unchanged — explicit novelty claim | DA review |
| `FAIL(...)` | Missing proof fields / hygiene | Fix remedy; no DA design relay |
| `ORCHESTRATOR-CLEARABLE` | Class matched + envelope + proofs | Merge |

### Body fields required when claiming admitted-scope gap (implementation must enforce)

```text
admitted_envelope: YES
admitting_pr: #<n>   # or admitting_rung: <ID>
admitted_surfaces: <one-line>
forbidden_surfaces: <one-line or NO>
```

Plus existing proof fields as applicable (`tested_code_sha`, `coverage_basis`, `ci_green`, …).

Missing claim → remain `unclassified-scope` (fail-closed).  
False claim / forbidden surface touch → envelope violation or FAIL, not silent clearable.

## Required documentation changes (this stamp + follow-on)

| Surface | Change |
|---|---|
| Orientation | Classify `DA-RESERVE` before DA relay; name `admitted-scope-router-gap` policy |
| `agent_onboarding` / handoff | Orchestrator must not DA-relay pure router debt |
| `ci_screening_surface` | Compact pointer when DOC-BUDGET allows |
| Follow-on results | Router selftests for Option A cases |

## Required router/selftest changes (follow-on only)

Rung: **`CLEARANCE-ADMITTED-SCOPE-GAP-0`** (gate-wiring).

1. True unknown → `unclassified-scope`
2. Admitted API-shaped lacking class + body claim → `admitted-scope-router-gap`
3. Admitted picker-shaped lacking class + body claim → `admitted-scope-router-gap`
4. Admitted-shaped missing proof fields → `FAIL(...)`
5. Forbidden surface → envelope / reserve (not clearable)
6. Novelty claim still → novelty
7. Gate-wiring still → gate-wiring

**Not** in this decision PR: no `clearance_check.sh` code change until that rung.

## Effect on admitted implementation PRs

| Before | After policy (docs now; machine after follow-on) |
|---|---|
| Admitted + no class → generic `unclassified-scope` → DA design relay | Classify as **router gap**; open/land class-hardening; do not re-open admission |
| Repeated same shape | Mandatory class-hardening, not normalized DA ceremony |
| True novelty | Still `unclassified-scope` / novelty → DA |

Until the router lands, orchestrators **must still apply the classification rule by hand** when they see `unclassified-scope` on admitted work (cite admitting PR/rung).

## Next rung

```text
CLEARANCE-ADMITTED-SCOPE-GAP-0
```

Then (track-local):

```text
TP-STUDIO-CLAUSE-PICKER-CLASS-0
```

so #1239-shaped diffs become clearable.

**Not next:** closeout (Owner-triggered only).

## Ruling (verbatim)

```text
DA DECISION: unclassified-scope is too broad for admitted implementation work. Future
proof-present PRs inside an already-admitted Owner/DA envelope must not be treated as
fresh DA design decisions merely because no precedented class exists. The clearance
router should distinguish true unclassified scope from admitted-scope router gaps.
Adopt a new reserve reason, DA-RESERVE(admitted-scope-router-gap), and require the PR
body or orchestration relay to cite the admitting PR/rung, admitted surfaces, forbidden
surfaces, and proof fields. Repeated admitted-scope router gaps should be closed with a
class-hardening PR rather than normalized as DA relay ceremony. Option C
(admitted-envelope registry) remains a later hardening rung after Option A is live.
```
