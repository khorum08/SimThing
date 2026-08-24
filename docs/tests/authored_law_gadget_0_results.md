# AUTHORED-LAW-GADGET-0 results

- Track: 0.0.8.7 RF arena modernization (remedial 11.1f)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/authored-law-gadget-0`
- Reconciled live-master base: `5210c9d22df30102a76a9eca78a7724231e03597`
- Implementation / tested_code_sha: `49cbf0f52729faf9a1690cfd390f46cc4656380e`
- Evidence-only final head and hosted workflow IDs: bound in the PR and Board return
- Board dispatch: `5402287672`; DA authority/amendments A1-A2: `5402217538`
- HD-RECEIPT: `0ac008a37a7e`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- Expected route: `DA-RESERVE(gate-wiring)`

## FIRST-STEP archaeology and A2 mechanism

The authored vocabulary is `EmlGadgetInstanceSpec` in
`crates/simthing-spec/src/spec/eml_gadget.rs`. Its compile-side mirror is
`EmlGadgetKind`, and the lowering attachment is `kind_from_instance` plus
`compile_gadget_instance` in
`crates/simthing-spec/src/compile/eml_gadget.rs`. Before this rung both enums
had nine members and neither appeared in `constitutional_surfaces.tsv`.
`EmlGadgetStackSpec` already entered production through its public compiler and
Vendor Door re-export; compiled EML already entered the exact-primitive
`EmlExpressionRegistry`, GPU program table, and ordinary `AccumulatorOpSession`
EvalEML consumer. No second evaluator was necessary.

The existing exact-primitive tree gate makes A2 decidable. Although
`CLAMP_FLOORED` is in the opcode vocabulary, the established shape-2 primitive
admission form recognizes an immediately preceding `CLAMP_BOUNDED` whose two
endpoints lie in the primitive domain. The new authored law therefore requires
`input_floor`; admission accepts it only when it is a positive finite normal in
the pinned LN domain. The emitted input expression is structurally
`CLAMP_BOUNDED(x, input_floor, EML_LN_DOMAIN_MAX)`. This floor is authored
saturated-law semantics, not a runtime repair or NaN guard. Zero, negative,
subnormal, NaN, and infinite floors fail before node emission with
`power_law_ln_domain_uncertified`; non-finite exponents fail with
`power_law_exponent_non_finite`.

The fenced kernel `LnConsumerGadgets::power_law_nodes` already demonstrated the
same admitted shape and remained unchanged. The spec lowering uses core's
pinned endpoint constants and existing nodes only; there is no kernel, WGSL,
executor, or opcode delta.

## Product and canonical identities

`EmlGadgetInstanceSpec::PowerLaw` adds exactly one law-stating authored member:

`{ id, input_col, output_col, exponent, input_floor }`.

Its eight-node postfix identity is:

`SLOT_VALUE(x), CLAMP_BOUNDED(input_floor, LN_MAX), LN, LITERAL_F32(k), MUL, CLAMP_BOUNDED(EXP_MIN, EXP_MAX), EXP, RETURN_TOP`.

The two guards are the existing admitted exact-primitive call-site form. The
law itself is `EXP(k * LN(x))`; `MUL` and both guards were already in the closed
vocabulary. `POW` remains absent. The pinned EXP/LN functions, endpoint bits,
qualification artifacts, execution classes, call-site admission shapes,
registry validation, opcode values, stack evaluator, kernel interpreter, and
WGSL are byte-unchanged by this rung.

`oracle_power_law` is the CPU twin of that exact composition. A table-driven
spec proof covers the complete paired vocabulary, canonical lowering,
id-independent semantics, unsafe authoring cases, and a staircase mutant. The
production semantic validator requires the canonical opcode/operand shape; a
`CMP_GE`/`SELECT` ladder attempting the law-stating role fails with
`power_law_intrinsic_semantics_required`. A canonical law with the deliberately
piecewise-looking id `piecewise-looking-name-does-not-control-semantics` passes,
so the decision is semantic rather than spelling-based.

## Existing production consumer witness

`authored_power_law_executes_through_production_accumulator_eml` starts with
vendor-style RON, deserializes one `PowerLaw` through
`EmlGadgetStackSpec`, compiles the stack, and presents the nodes to the existing
`EmlExpressionRegistry`. That existing gate independently verifies both
exact-primitive call sites. The same admitted identity is uploaded through the
existing `EmlGpuProgramTable`, packed as an ordinary `CombineFn::EvalEML`
`AccumulatorOp`, and executed by an attached `AccumulatorOpSession` on the live
NVIDIA Vulkan adapter. The GPU result is bit-identical to `oracle_power_law`.
The proof does not call a new helper-only executor and adds no production
consumer.

## Paired constitutional seal and mutants

`constitutional_surfaces.tsv` now carries two ten-member enum rows:

- `EML-GADGET-INSTANCE-VOCABULARY` for `EmlGadgetInstanceSpec`;
- `EML-GADGET-KIND-VOCABULARY` for `EmlGadgetKind`.

Their admitted-member sets are identical, and the table-driven Rust proof
checks the 1:1 name/parse/lowering correspondence. Two independent planted
mutants were run and restored:

- instance-only `PlantedUnlistedInstance` produced expected RED:
  `EML-GADGET-INSTANCE-VOCABULARY: registry drift added=['PlantedUnlistedInstance']`;
- kind-only `PlantedUnlistedKind` produced expected RED:
  `EML-GADGET-KIND-VOCABULARY: registry drift added=['PlantedUnlistedKind']`.

After restoration the constitutional check reports both gadget censuses at 10,
`EML-OPCODE-LIBRARY=25`, and `EML-STACK-LIBRARY=9`. A standing integration
test pins the pre-rung 25-opcode row and proves neither its registry member list
nor `eml_nodes.rs` contains `POW`.

## Exact-code verification

All passing commands below ran against the clean implementation checkpoint
`49cbf0f52729faf9a1690cfd390f46cc4656380e`.

| Command | Result |
|---|---|
| `cargo check -p simthing-spec -p simthing-driver` | PASS |
| `cargo test -p simthing-spec -- --test-threads=1` | PASS — complete package plus 8 compile-fail doctests |
| `cargo test -p simthing-driver --test authored_law_gadget_0 -- --nocapture` | PASS — 2/2; live NVIDIA/Vulkan production EvalEML witness |
| `cargo test -p simthing-kernel eml_ln_primitive_0 --lib -- --test-threads=1` | PASS — 2/2 pinned LN qualification/admission |
| `cargo test -p simthing-kernel eml_exp_primitive_0 --lib -- --test-threads=1` | PASS — 8/8 pinned EXP qualification/admission/lowering |
| exact-primitive domain call-site mutant-sensitive unit filter | PASS — 1/1 |
| `cargo test -p simthing-driver --lib -- --test-threads=1` | PASS — 16/16 |
| `cargo test -p simthing-driver --test field_sweep_session_seam_0 -- --test-threads=1` | PASS — 3/3; ordinary-session GPU blast radius |
| `cargo test -p simthing-driver --test cpu_gpu_parity_matrix_0 cpu_gpu_parity_matrix -- --test-threads=1` | PASS — 2/2; all matrix cases and planted defects |
| `cargo test -p simthing-embedder --test vendor_door_triad_surface_0 -- --test-threads=1` | PASS — 2/2 |
| constitutional check + selftest | PASS — paired 10/10, opcode 25 unchanged, stack 9 unchanged; 7 selftest plants |
| inventory check + drift prove + lifecycle schema/scheduled | PASS — 1,317 rows / 1,317 discovered; zero expired |
| paired census plants | EXPECTED RED twice as recorded above; both restored |
| `agent_scan.sh --base 5210c9d2 --head 49cbf0f5` | PASS — zero hard failures and zero inspect flags |

The rendered 52-anchor set was acknowledged before governed edits. Newly
triggered load-bearing anchors were read through `anchor_query.sh`:
`accumulator-exact-vs-soft-semantics`,
`candidate-f-exhaustive-proof-method`, `core-property-value-model`,
`exact-numeric-candidate-f`, and `one-tree-owners-never-spatial`. The complete
rendered ActionBand, EML, property, overlay, RF, STEAD, slot-identity,
orientation, scanner, convergence, and workshop anchor family remains binding.

## Scope disposition

Return **PROBATION / proof-present / DA-review-pending**. The structural
certificate is **owed at graduation** because this rung adds a lowering path
that emits into production EML. Coding did not run clearance or relay lint,
merge, change the pointer, edit #1803/11.2 or any guide/exemplar, begin 11.3+,
11.4, or 12.x, add `POW`, alter an opcode/kernel/WGSL path, or weaken an
existing gate.
