# MOBILITY-AUDIT-0 - owner OrderBand depth budget audit

Date: 2026-06-01

## Verdict

**PASS**

The accepted MOBILITY-SCENARIO-0 first slice fits the current OrderBand ceiling without narrowing.

| Item | Value |
| --- | --- |
| Scenario | `mobility_scenario0_v7_9_first_slice` |
| Routing | `NarrowedAdversarialFirstSlice` |
| Spatial depth | 4 |
| `max_factions_per_cell` | 4 |
| Routing EML node budget | 16 |
| Theater | 48 cells |
| Soak | 34k entities |
| Current `max_orderband_depth` | 16 |
| Required OrderBands | 13 |
| Slack | 3 |

## Budget Model

The accepted D=4 spine has three inter-level spans (`D - 1`). MOBILITY-AUDIT-0 counts each hard/soft circulation family as a distinct ordered budget slot or spine pass, with hard fixed-point Band Alpha settled before any soft float Band Beta read.

| Family | Bands | Rationale |
| --- | ---: | --- |
| modifier-down | 1 | Blockade-immune owner modifier down-broadcast overlay refresh. |
| hard fixed-point Band Alpha | 1 | Hard conserved/decision quantities settle first. |
| economy-up | 3 | D=4 up-sweep over the accepted spatial spine. |
| economy-down | 3 | D=4 down-broadcast over the accepted spatial spine. |
| research-up | 3 | Owner research/progress aggregation over the same spine. |
| thresholds | 1 | Threshold evaluation after hard economy/read dependencies settle. |
| soft float Band Beta | 1 | Soft floats read settled hard state and do not gate structure. |

Total: `1 + 1 + 3 + 3 + 3 + 1 + 1 = 13`, which is within the current ceiling of 16.

## Audit Attestations

- Alpha precedes Beta: yes.
- Hard/soft quantities silently mixed in one pass: no.
- Owner entities assumed to be spatial parents: no.
- Owner relations include faction flow-pooling plus species/blueprint/tech down-broadcast overlays: yes.
- ALLOC/REENROLL/IDROUTE/ECON/OWNER implementation ladders opened by this audit: no.
- Runtime implementation authorized: no.
- GPU kernels, allocator, reparenting, routing, economy, owner-overlay runtime, production `SimSession` wiring, default-on flags, or invariant edits: no.

## Next Posture

MOBILITY-AUDIT-0 is accepted as an audit-only PASS. No separate OrderBand-depth expansion scenario is required for the accepted first slice. The next candidate ladder by the production-track sequence is ALLOC, but this report does not open ALLOC or any runtime implementation gate.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```
