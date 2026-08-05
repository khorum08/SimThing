# EML-LN-PRIMITIVE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.12)
- Status: **STOP** — Candidate-F (LNCF) probe RED: vendor-`log` seed leaves the ±1 ULP
  correction envelope on the certified tuple (DA `5186354130` / remand `5186386924`)
- Branch: `grok/eml-ln-primitive-0`
- ORIENT-RECEIPT: `d950cd858719`
- HD-RECEIPT: `a28f739a7fdb`
- Exact implementation base: `822de227c5666a15a5f7a0d6f80273c90ef2ec9c`
- Expected route: `DA-RESERVE(gate-wiring)` — coding does not `/clearance`, merge, or flip pointers

## ANCHOR-ACK

`admission-ladder-necessity-test@4bedf826f6f7, core-gpu-residency@8db4198cbc29,
core-overlays@54df7604a49d, core-property-value-model@17cd41a567b7,
core-rf-arenas@d171614211e9, eml-admission-shapes@d3c9259c4244,
eml-extension-ladder@7755bc72ffbe, eml-integration-plan@8eba54b02320,
eml-triad-integration@dada7d680557, exact-numeric-candidate-f@6938a2efadb5,
field-policy-time-decisions@993c7d0560e8, field-sweep-preservation@acc521a5a361,
founding-ontology-invariants@46802793fba7, orientation-harness-core@8a365d1c0864,
rf-arena-substrate@17b5f1e5c2ba, scanner-selftest-delta-gate@34fb2662baae,
seal-residue-cross-crate@49ee7c4ba6f4, simthing-0087-binding-laws@a59203b79425,
simthing-0087-pillars@61487cba1f9e, stead-events-are-rf@525388344ef2,
stead-rejected-shapes@3752549ff106, stead-shared-surface-ledger@87eaa1e7bb9c,
stead-spatial-contract-core@8585db4ac631, stemthing-binding-laws@6787a118c3ca,
stemthing-lane-not-leg@a9e9caa27a0f, stemthing-slot-identity-ruling@02c87b9126e1,
structural-execution-convergence@6b4cedec482b, workshop-candidate-homing@3e584f0ad175` — ACK.

## Frozen STOP history — LN1C (do not erase)

**Certified tuple:** NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79 /
`rustc 1.95.0` + `wgpu 22.1.0` / `naga 22.1.0` (Cargo.lock).

Under the former handoff fence **`no vendor transcendental`**, five same-shape
Newton/`Lg*` reconstruction families were tried. Retained failure:

| Item | Value |
|---|---|
| Tag | **LN1C** (`0x4C4E3143`) |
| Algorithm identity | **`0x108443cfaeeaadfe`** |
| Probe (~6414) | bit-exact |
| Characterization | max_ulp=1, nonfinite=0 |
| Exhaustive interpreted / JIT | **RED** at first divergence **`0x008dcb6b`** (1 ULP; both arms) |

DA `5186354130` withdrew deferral: the fence that banned vendor `log` forced the
unfixable reconstruction shape; Candidate F lifts the fence for the **seed only**.

## Candidate-F (LNCF) — active executable candidate

Authority: DA `5186354130`; process `docs/workshop/sqrt_candidates.md` §§3,4,6,7,8;
remand `5186386924`.

| Item | Value |
|---|---|
| Tag | **LNCF** (`0x4C4E4346`) |
| Sequence version | `2` |
| Algorithm identity | **`0xbc2f8faa558bb920`** (binds to live EXP identity) |
| Method | vendor `log` seed → `EXP` images of `{y−ulp,y,y+ulp}` vs `x` → ±1 ULP snap |
| Standalone artifact | `crates/simthing-driver/tests/wgsl/eml_ln_cf_candidate.wgsl` |
| Exact-class admission | **closed** (`opcode_allowed_in_exact` excludes LN; door rejects unpinned digests) |
| CLOSED_OPCODES | retained as three-arm **harness vocabulary only** — not exact-authoritative |
| Exhaustive digests | **`0x0`** until local three-arm replay pins them |
| Admitted-domain size | **2130706432** |

### Required falsifiers
- seed-as-authority mutant RED
- correction-direction invert mutant RED
- skip-neighbor-EXP decision-bypass mutant RED
- WGSL twin drift / frozen-artifact identity invalidates qualification
- edges: `min_normal`, `1.0 -> +0.0`, domain max — bit-identical across arms

### Measurement log — STOP

| Measurement | Result |
|---|---|
| Three-way probe (6411) | **RED** — jit first divergence at **`0x00800000`** (domain min_normal): GPU `got=0xc2aeac4e` vs CPU `want=0xc2aeac50` (**2 ULP**); **1478** mismatches before abort |
| Characterization | not reached (probe RED) |
| Exhaustive CPU / interpreted / JIT | not reached; digests remain **`0x0`** |

**Diagnosis (within DA fences):** the EXP-domain ±1 ULP snap cannot reconcile CPU
`f32::ln` and NVIDIA Vulkan WGSL `log` seeds when they disagree by more than one
ULP. Widening the correction loop, redefining exactness, shrinking the domain, or
measured-ULP admission are all forbidden. Falsifiers (seed-as-authority /
correction-direction / skip-neighbor-EXP) still RED as required.

## What must not happen
- No measured-ULP admission; no domain shrink; no EXP mutation; no POW opcode
- No `/clearance`, merge, pointer flip, or StemThing-A from coding
- Do not re-open exact-class LN admission merely because LNCF is implemented — DA
  adjudicates promotion after exhaustive proof return
