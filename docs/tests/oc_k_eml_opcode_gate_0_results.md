# OC-K-EML-OPCODE-GATE-0 Results

## Status
**DONE / PROBATION** — closed EvalEML opcode/combine registration door; DA deep audit/merge.

## PR / branch / head
See PR body `tested_code_sha` after push.

## STOP-gate forgeability
**PASS** — `oc_k_eml_opcode_gate_0_unwhitelisted_opcode_registers_today`: raw `EmlNode.opcode: u32` still freely constructible; unwhitelisted opcode payloads assemble and reach `register_formula` API (admission hard-errors). Pre-door, `EmlGpuProgramTable::upload_trees` accepted raw nodes without vocabulary check — that path is now gated. Residual: free POD node construction.

## What changed
- `crates/simthing-kernel/src/eml_opcode_gate.rs` — closed vocab, gate, Tier-2 parity, semantic reject, SoftStep policy
- `upload_trees` + `PackedAccumulatorUpload::from_gpu_ops` closed-vocab hard-gate
- `eml-extension-ladder` §2.1 pathway payload; design K4 stamp; inventory; triage; kernel_surface

## Door
`OpcodeRegistrationGate`: existing closed opcode/combine OR generic Tier-2 + `CpuOracleParityProof` (still closed until DA expansion) OR semantic hard-reject. GPU tree upload and packed combine admit closed sets only.

## Pathway
`eml-extension-ladder` §2.1: Anchor B + core §1.1 + §4.1 ladder + gadget catalogue + SoftStep policy conditional.

## Worked SoftStep
`SoftStepPolicyConditional`: `out = B + soft(x)*(A-B)`; branchless columns; closed opcodes only.

## seal_residue_risk
**B residual:** `EmlNode.opcode` remains public `u32` (POD residual; admission rejects unlisted). WGSL interpreter default arm permanent. B1–B3, B5–B8 clean for this rung. Not baseline-zero.

## Scan ledger
No scan deleted (no redundant scan owned this door). kernel_surface +exports. Net 0 scan rows.

## TEST-BUDGET (6 tests)
`scripts/ci/triage_log.tsv` row `oc-k-eml-opcode-gate-0`. Permanent:seal-proof; delete-at-closeout: no.

## Scope ledger
Implemented: door + Tier-2 parity type + semantic reject + pathway + SoftStep. Proxied: full EmlNode newtype seal → backlog. Deferred: closeout C / 0.0.8.6.

## Sticky / ACKs
See PR after push.
