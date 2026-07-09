# TP-STUDIO-CLAUSE-API-ADMISSION-0 Results

## Status

**DONE — DA-DENIED / KEEP WORKSHOP (Option B)** — 2026-07-09 executive DA admission decision;
status stamp merged [#1224](https://github.com/khorum08/SimThing/pull/1224) @ `72a29193cdb48f8e446a5a79547429d9966ef9c6`.
No production `simthing-mapeditor` ClauseScript ingest API admitted during 0.0.8.5.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-STUDIO-CLAUSE-API-ADMISSION-0` |
| PR (status stamp) | [#1224](https://github.com/khorum08/SimThing/pull/1224) |
| Merge SHA | `72a29193cdb48f8e446a5a79547429d9966ef9c6` |
| Kind | Owner/DA admission decision (no implementation) |
| Prior | #1222 workshop candidate @ `bcbc2f4389`; status stamp #1223 |
| Decision | **Option B — Deny / keep workshop** |

## Ruling (verbatim)

```text
DA REJECTION / KEEP WORKSHOP: TP-STUDIO-CLAUSE-API-ADMISSION-0 does not admit production
mapeditor ClauseScript ingest during 0.0.8.5. The #1222 workshop-homed TP candidate remains
expirable scenario evidence. Any future production mapeditor API must wait for
closeout/admission review or a separate Owner-authorized substrate-widening rung after
STEAD rebind / named projection policy readiness.

Stack note: parse/hydrate/serde are already production. Elevation of a mapeditor
composition API is deferred not because the spine is missing, but because projection
completeness and lowerer heuristic elegance are insufficient for future scenario widenings.
```

## Stack audit (summary)

| Layer | Home | Semantic-free? | Elevation status |
|---|---|---|---|
| `parse_raw_document` | clausething | Yes | Already production |
| `hydrate_scenario` | clausething | Partial (e.g. owner-key fleet posture) | Already production; residual debt |
| Pack → Spec projection | workshop | Algorithm mostly generic; packaging TP-coupled | Not production mapeditor surface |
| Authority serde | simthing-spec | Yes | Already production |
| JSON scenario_io | mapeditor | Yes (Spec JSON only) | Already production |

## Admitted surfaces

**None new** in `simthing-mapeditor`.

Kept as today:

- clausething parse / hydrate
- simthing-spec authority serde
- mapeditor JSON `scenario_io`
- workshop TP candidate (`tp_studio_clause_ingest`)

## Forbidden surfaces

- Production mapeditor `.clause` ingest API
- UI `.clause` picker (`TP-STUDIO-CLAUSE-PICKER-0` blocked)
- Production TP / `tp_base_disc_1500` / `{{FIXTURE_JSON}}` defaults in mapeditor
- Language: “production Studio ClauseScript ingest API”

## Why not Option A / C

- Empty-placement projection is candidate residue, not finished Studio authority.
- Lowerer still has scenario-shaped heuristics (`owner == "pirate"` posture paths).
- Trait-only shell without rebind/projection contract invites premature dual homes.

Option A may reopen after STEAD-rebind readiness names projection modes and caller-only resolvers.

## Next

```text
TP-STUDIO-STEAD-REBIND-READINESS-0
```

Then re-open `TP-STUDIO-CLAUSE-API-ADMISSION-0` if Owner wants production API with explicit constraints.

**Not next:** picker, closeout (unless Owner declares workplan complete).
