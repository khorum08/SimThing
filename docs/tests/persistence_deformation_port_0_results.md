# PERSISTENCE-DEFORMATION-PORT-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED / no 15.1**.
Coding has not merged, graduated Phase 15, moved the pointer, begun response
compression, or closed the track.

## Provenance and receipts

- Board handoff: `5520547969`
- exact branch base / `origin/master` at dispatch:
  `98cd61ad528b717ecaffa5ce4929c98434965f3f`
- branch: `codex/persistence-deformation-port-0`
- `HD-RECEIPT: 71366d4e7eac`
- `ORIENT-RECEIPT: af3aaff27ad7`
- orientation rule stamp: `fc67cda50e225c32`
- orientation digest:
  `b90f08e633395bcc994608afd5688a07de49753d9a1fda0129bbba72a10dd0b1`
- final exact head, PR, hosted run ids, fresh clearance, and relay-lint are
  recorded in the immutable Board return packet after this report is committed

All 63 rendered required anchors were acknowledged through
`scripts/ci/anchor_query.sh`; the append-only reach ledger carries those exact
receipts.

## Archaeology-first map

| Surface | Authority found | 15.2 disposition |
|---|---|---|
| CPU Current-to-Next | `RuntimeRfDemandGenerationAuthority` owns `mint_current_to_next`; `produce_runtime_rf_next_generation_demands` clears Current and invokes one crate-private `carry_unresolved_demand_to_next_generation` | optional binding is looked up inside this existing mint; its atomic second-mint fence is unchanged |
| native U carry | the crate-private carry checked-adds sealed `UnresolvedDemandObservation::unresolved` to independently authored N+1 demand | neutral U becomes admitted `deform(U)` only when a binding exists; absent policy retains the original integer path |
| CPU vendorized mirror | driver `produce_runtime_rf_next_generation_demands_for_tick` delegates directly to the spec door and accepts its authority token | the same sealed program and evaluator execute; no separate policy representation or approximation |
| resident production mint | exact Q writes canonical `T_s`; `ResidentClearingLiveHead::encode_append_and_n_plus_one` appends it and copies the same representation into private recursive intake | the optional variant appends the same canonical `T_s`, then applies row-local EML only while creating the private N+1 intake copy |
| frozen 14.5 exact Q | `resident_clearing_apportionment.wgsl` plus qualification fingerprint `0x1c3ca3cf8e625e48` | byte-for-byte unchanged; 15.2 is not inserted into Q |
| ordinary EML | sealed `TransformOp` stores admitted postfix `EmlNodeGpu` | `PersistenceDeformationProgram` adds a closed-domain range proof over that existing representation, not a new opcode family |
| ClauseScript vehicle | `parse_script_value` produces ordered `RateFormulaSpec` modifiers and `compile_value_formula_eml` lowers the existing flat chain | the persistence compiler reuses the same parser and shared modifier-appender, with `base` multiplying PARAM(0)=U and literal modifiers retaining authored order |
| 15.0 consequence chain | `AuthoredPersistenceValuation.value_program -> CostBand -> PersistenceOverlayBinding` under `fund_unresolved_persistence` | remains consequence-only beside the port; demand reinjection remains 0 and unmigrated remains 0 |

## Sealed port and admission contract

`PersistenceDeformationProgram` wraps the ordinary admitted `TransformOp` and
an immutable integer cap. Admission performs a conservative binary32 interval
proof over the closed input domain `0..=cap` and refuses:

- cap values above `16_777_216`, the largest consecutive exact binary32 integer;
- empty, malformed, stack-overflowing, unsupported-opcode, or non-PARAM(0) EML;
- non-finite literals, possible non-finite arithmetic, or possible division by zero;
- any program whose range can become negative or exceed the admitted cap; and
- any program with `deform(0) != 0`, sealing “authored deforms; substrate creates.”

Runtime retains fail-closed input/output checks. It validates the unquantized
result before the single declared floor-to-u32 projection; there is no clamp,
fallback, or mutable policy state. Bindings are immutable and keyed by the
native full scope plus claimant on the CPU door. The resident executor already
has one fixed semantic scope, so its application binding is claimant-keyed and
is lowered to semantic rows only when the root plan is minted.

## Production call graph

```text
ClauseScript script_value
  -> parse_script_value -> RateFormulaSpec (ordered modifiers)
  -> shared modifier-chain EML appender -> TransformOp::admit_eml
  -> PersistenceDeformationProgram::admit(cap)

CpuVendorizedOracle:
  RuntimeRfDemandGenerationAuthority::mint_current_to_next (atomic once)
  -> existing clear -> sealed U observation
  -> existing crate-private carry -> optional program.deform(U)
  -> d_authored(N+1) + carried U

ResidentRequired:
  unchanged continuous R -> unchanged exact Q -> canonical T_s
  -> existing live-head schedule append
  -> absent: original byte-copy mint
     bound: generic bounded-EML mint copies T_s and replaces only U with deform(U)
  -> existing private recursive intake -> unchanged exact Q at N+1
```

The bound resident path submits N, N+1, and N+2 before the first proof
readback. There is no host projection, authored interior demand, second intake,
or translated product type. The new WGSL contains only generic transform/EML
vocabulary and the engine retains zero dependencies on ClauseThing.

## Positive transcript

Focused terminal output:

```text
PERSISTENCE-DEFORMATION-PORT identity=PASS decay=100->80->64 saturation=80->100 expiry=50->0 resident=PASS cpu-oracle=PASS bounds=PASS atomic-refusal=PASS zero-red=PASS clausescript-long-chain=PASS
test result: ok. 3 passed; 0 failed
```

- absent policy and an explicitly empty binding table produce equal stamped
  CPU products; request 10 against supply 4 plus authored N+1 demand 2 remains
  the frozen result 8;
- an actual parsed ClauseScript chain with eight ordered modifiers lowers to
  more than 16 ordinary EML nodes; 0.8 policy yields U `100 -> 80 -> 64` on
  both the real resident path and the CPU-vendorized mirror;
- bounded `min(2*U,100)` saturates `80 -> 100` through the same port;
- constant-zero expiry removes carried U while leaving independently authored
  N+1 demand intact;
- generation N remains the unchanged Q product; deformation appears only in
  the already-existing N+1 intake mint.

Frozen proof transcripts remain green:

```text
RECURSIVE-RESOURCE-FILTER-FORMALIZATION R=PASS Q-compose=PASS P-F=sufficient-no-storage exact-tuple=existing lambda=implicit spatial=PASS temporal=PASS triad=reused-born-state
RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 1c3ca3cf8e625e48
resident_clearing_parity_terminal_referee: ok
RESIDENT-CLEARING-CUTOVER causal=PASS ...
RESIDENT-CLEARING-CUTOVER two_tree=PASS ...
test result: ok. 3 passed; 0 failed
```

Thus frozen Q, exact `T_s`, 15.0 `Q compose R`, physical/workgroup/partition
invariance, literal self-consumption, divergent-generation tree isolation,
generation pacing, and the 14.6 causal cases are unchanged.

## Negative matrix

| Plant / failure | Typed or mechanical result |
|---|---|
| `2*U` without a bounding operation | `MayExceedCap` at admission; no silent clamp |
| NaN literal | `NonFiniteLiteral` at admission |
| possible divide by zero / overflow to non-finite | admission refusal before binding |
| nonzero result at U=0 | `MayCreateWithoutUnresolved`; policy cannot create substrate U |
| duplicate full-scope/claimant binding | `DuplicateClaimantBinding` |
| runtime U above the admitted cap | `DemandCurrentToNextRejected`; no N+1 vector is returned |
| retry after that failed mint | `DemandCurrentToNextAlreadyProduced`; no second carry |
| planted `ShadowPersistence` type | compile-fail `E0432` plus source-census plant RED |
| planted second native carry definition | single-port source-census plant RED |
| any row-local transform runtime failure | every private intake row receives failure status before the intake can be consumed; no successful partial vector |

## Consequence-only census

The focused source referee finds one native carry definition and one caller in
the Current-to-Next authority. `fund_unresolved_persistence` still contains no
`RuntimeOwnerSiloDemandBucket` or `PersistenceDeformationBindings`. No
`ShadowPersistence`, persistence market, reinjection adapter, migration table,
PALMA/Gu-Yang solve, descendant walk, field cache, or same-generation clear was
added.

## Verification ledger

| Command | Result |
|---|---|
| touched-package `cargo check` for core/spec/kernel/gpu/driver/clausething/workshop | PASS |
| `cargo test -p simthing-core persistence_deformation -- --nocapture` | PASS — 2/2 |
| `cargo test -p simthing-core --doc persistence_deformation` | PASS — planted shadow compile-fail |
| `cargo test -p simthing-workshop --test persistence_deformation_port_0 -- --nocapture` | PASS — 3/3, real GPU |
| frozen `recursive_resource_filter_formalization_0` | PASS — 1/1 |
| frozen row-11 `unresolved_demand_recurs_once_at_n_plus_one_and_drains_without_authored_path` | PASS — 1/1 |
| frozen `resident_clearing_parity_0` | PASS — 1/1; qualification unchanged |
| frozen `resident_clearing_cutover_0` | PASS — 3/3 |
| test inventory / drift | PASS — `1397/1397`, missing 0, extra 0, unledgered 0, stale 0 |
| detachability | PASS — production coupling 0, proof coupling 0, ceiling 0 |
| doctrine anchors | PASS — pending healthy 4, orphaned 0, stale 0, curation 88 |
| constitutional authority/surface census | PASS — production authority 1, duplicate settlement/economic adapter/global coupling/private solver all 0; public ClauseThing lowerers explicitly contained at 23 |
| repaired-harness Agent Scan / hosted Doctrine Scan+Exec / fresh clearance+relay-lint | recorded at exact pushed head in the Board return packet |

## Changed-file census

Twenty-three files: two ClauseScript lowering/re-export files; two shared-core sealed
program/re-export files; one driver resident binding file; two GPU facade/live-
head mint files; four kernel plan/transform/re-export files (including one new
scenario-free WGSL file); four spec binding/mint/re-export files; one focused
workshop referee; this result; the current-evidence index; and the two lawful
test/reach TSV ledgers, the existing constitutional-surface data row, and two
conforming allowlist rationale rows for the generic non-field shader and its
typed kernel surface. The
graduated 14.5 apportionment WGSL, workflows, CI shell/Python,
canonical product ABI, and frozen referee sources have zero diff.

The `simthing-core` token is the shared sealed admission proof required for one
program contract on both CPU and GPU without dependency inversion. The
`simthing-gpu` edit is the archaeology-discovered owner of the existing
Current-to-Next live-head mint; it extends that mint rather than adding a new
one. These two ownership edges are the only files beyond the HD's predicted
surface list.

FULL structural ZERO-RED and hosted identifiers are attached to the final
exact-head Board packet. Pointer/status remains 15.2 probation; 15.1 is not in
flight.
