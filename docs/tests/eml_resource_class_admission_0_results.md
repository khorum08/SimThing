# EML-RESOURCE-CLASS-ADMISSION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.7)
- Status: **COMPLETE — DA-GRADUATED / merged #1537 @ a7c05715.** Substrate accepted. The coder STOP was CORRECT: it exhausted authorized branches, weakened no threshold, and averaged no failing case away. PALMA passes (1.0891x median, 0.7996x worst — faster than bespoke at worst case). Gu-Yang 4.7593x/5.4648x is recorded as MEASURED PERFORMANCE DEBT, not a failed branch: DA verified the generated WGSL does naive per-edge GLOBAL gathers with zero `var<workgroup>`/`workgroupBarrier` in the JIT, while a bespoke 4-neighbour stencil tiles and reuses each cell ~4x — almost exactly the gap. The JIT knows the offset table at compile time and can emit the same tiling; tiling changes WHERE values load, never the ORDER they fold, so bit-exactness survives. Threshold UNWEAKENED and carried as the exit of successor `FIELD-SWEEP-TILED-GATHER-0`. Production already runs the generic path (FIELD-SWEEP-LEGACY-CALLERS green), so acceptance un-ships nothing.
- ORIENT-RECEIPT: `e563c4399d73`; rule stamp: `5319c193d38da6ce`
- HD-RECEIPT: `85bfbf0e0e33`
- Remand authorities: Board comments `5148327947` and `5148515094`
- Adapter: `NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan`, driver `595.79`, Vulkan `1.4.329`
- Pointer unchanged; no graduation, merge, 5.8, or DA relay is claimed.

## Implemented substrate

`EmlResourceClass` is one closed authority shared by ordinary EML and field sweeps:

| Class | Stack slots | Per-tree nodes | Selection |
|---|---:|---:|---|
| `CompactStack4` | 4 | 16 | smallest class covering the measured 5.4–5.6 census (peak stack 3, maximum tree 9) |
| `LegacyFixed32` | 32 | 32 | explicit compatibility class preserving the former admitted envelope |

Registration and replacement derive the smallest fitting class from validated node count and peak
stack. Field callers no longer carry a resource-size request. CPU twins, node-buffer capacity, ordinary
EML tables, field sessions, and AccumulatorOp sessions consume the derived class. Both classes are JIT
specializations of one canonical WGSL interpreter: specialization changes exactly one audited
`EML_STACK_MAX` constant and never branches on algebra, field, or operator identity.

The sealed `ExactPrimitiveAdmissionDoor` requires independent determinism, compiled-cost, and
concrete-consumer keys. Missing backend replay/domain/reference, cost improvement, or measured
consumer necessity rejects. The admitted primitive count remains **zero**; the opcode set is unchanged.

## Exact-adapter measurement door

A bounded test/profiling adapter queries `VK_KHR_pipeline_executable_properties` over the canonical
field interpreter. `VK_KHR_performance_query` is not advertised and enumerates zero counters. NVIDIA
pipeline-executable statistics are available and are the truthful equivalent resource-effect evidence.

| Class | Subgroup | Registers | Binary bytes | Stack bytes | Raw NVIDIA “Local Memory Size” | Shared bytes |
|---|---:|---:|---:|---:|---:|---:|
| stack-4 | 32 | 31 | 21,504 | 0 | 68,719,476,736 | 0 |
| legacy-32 | 32 | 25 | 14,720 | 0 | 68,719,476,864 | 0 |

The local-memory values are reported verbatim as driver `UINT64` values; no timing-derived proxy or
undocumented reinterpretation is substituted. Stack-4 and legacy-32 outputs are bit-identical for
matched PALMA and Gu-Yang.

## Performance-gate adjudication

Matched samples include upload plus GPU-resident dispatch, no readback, four warmups, and nine samples.

| Case | Class | Dispatches | Median µs | Worst µs | Bespoke median µs | Median ratio | Worst ratio | Gate |
|---|---|---:|---:|---:|---:|---:|---:|---|
| PALMA 16×16, 8 iterations | stack-4 | 24 | 1,537.8 | 1,914.3 | 390.0 | 3.9431× | 3.1086× | **FAIL** |
| PALMA 16×16, 8 iterations | legacy-32 | 24 | 1,625.6 | 2,161.5 | — | — | — | diagnostic |
| Gu-Yang 16×16, two stages | stack-4 | 6 | 529.3 | 2,481.3 | 34.2 | 15.4766× | 50.1273× | **FAIL** |
| Gu-Yang 16×16, two stages | legacy-32 | 6 | 494.6 | 584.5 | — | — | — | diagnostic |

The inherited thresholds are median `<= 1.25×` and supported-adapter worst `<= 1.5×`. Both fail by
wide margins. This does not weaken determinism, authorize a bespoke kernel, admit a primitive, or
abandon the field IR. Per the handoff, failed measurement is a coder STOP for orchestration.

## Focused proof

```text
eml_resource_class_admission_0: 2 passed; 32/33 boundaries and smallest-fit green
exact_primitive_: 2 passed; independent-key/domain/backend/cost/consumer negatives green
eml_resource_class_capability_census_0: 2 passed on exact NVIDIA/Vulkan adapter
eml_resource_class_measurement_0: 1 passed; class parity and matched rows emitted
```

No LinkGraph cap, dense-region reach, semantic opcode, production vendor dependency, or second
interpreter was added.

## Remand 2: canonical IR-generated JIT branch

The complete Remand-1 tree was checkpointed and pushed before further edits:

- branch: `codex/eml-resource-class-admission-0`
- checkpoint: `ba6d28cbc041adb4e4e250bae34d4a42327e5644`

The authorized branch replaces field execution's stack-token-only specialization with one generic
postfix-IR-to-WGSL compiler. Its collision-safe cache key contains the sealed resource class and the
complete canonical map/fold/post word sequence; the stable 64-bit digest is report identity only.
Generated functions remove the storage-backed node loop and operand scratch. There is no field,
algebra, operator, scenario, PALMA, or Gu-Yang branch in the compiler.

Mechanically adjacent stages and iterations share one command submission. The only single-dispatch
fusion is proof-gated: a transient producer must be immediately followed by the exact consumer bound
by its sealed adjacency/layout certificate. The compiler substitutes the producer IR at the consumer's
transient reads and still writes the producer result to the transient lane. The original GPU
interpreter remains profiling-only for parity proof.

### Identity and full-census parity

| Generated case | Class | Canonical identity | Cache identity | Dispatches |
|---|---|---|---|---:|
| PALMA | stack-4 | `9cff5d0094629a49` (93 words) | `431a78d14f217f24` | 8 |
| Gu-Yang fused pair | stack-4 | `859e56cee971cc5a` (200 words) | `b86ebbdf456e1eb8` | 1 |

CPU oracle, preserved interpreted GPU, and generated JIT are bit-identical across PALMA,
normalized, source-capped, gradient-XY, fused saturating-flux, three-stage saturating-flux with
choke output, W-impedance, and a planted `LegacyFixed32` peak-stack-5 program. The existing 32/33
node and stack falsifiers remain green. ExactPrimitiveAdmission remains sealed with zero primitives.

### Generated-pipeline resource effects

Raw `VK_KHR_pipeline_executable_properties` statistics on the named adapter:

| Pipeline | Subgroup | Registers | Binary bytes | Stack bytes | Raw NVIDIA "Local Memory Size" | Shared bytes |
|---|---:|---:|---:|---:|---:|---:|
| PALMA generated stack-4 | 32 | 18 | 4,096 | 0 | 68,719,476,736 | 0 |
| Gu-Yang fused generated stack-4 | 32 | 24 | 7,040 | 0 | 68,719,476,736 | 0 |
| Gu-Yang unmodified bespoke reference | 32 | 23 | 15,872 | 0 | 68,719,476,736 | 0 |

The driver values are recorded verbatim. The generated Gu-Yang pipeline uses one more register than
the bespoke reference, while stack, reported local/shared memory, and subgroup size are identical;
its binary is smaller, not larger.

### Final matched performance adjudication

Identical adapter pin, work envelopes, unmodified bespoke references, four warmups, nine samples,
upload plus dispatch/wait, and no timed readback:

| Case | Class | Dispatches / submissions | Median us | Worst us | Bespoke median us | Median ratio | Worst ratio | Gate |
|---|---|---:|---:|---:|---:|---:|---:|---|
| PALMA 16x16, 8 iterations | stack-4 | 8 / 1 | 294.5 | 340.4 | 270.4 | 1.0891x | 0.7996x | **PASS** |
| PALMA diagnostic | legacy-32 | 8 / 1 | 283.7 | 387.1 | — | — | — | bit-exact |
| Gu-Yang 16x16 fused pair | stack-4 | 1 / 1 | 140.4 | 178.7 | 29.5 | **4.7593x** | **5.4648x** | **FAIL (median and worst)** |
| Gu-Yang diagnostic | legacy-32 | 1 / 1 | 153.4 | 215.6 | — | — | — | bit-exact |

The binding gate is median `<=1.25x` and worst `<=1.5x` for every case. PALMA passes, but Gu-Yang's
median and worst fail, so the authorized JIT branch cannot complete the rung.

Cause decomposition is bounded by measured/static evidence:

- Dispatch overhead is not the differentiator: generated Gu-Yang and bespoke each use one dispatch
  and one submission for the same 256-cell step.
- Compiled footprint is close: generated uses 24 registers versus 23 bespoke, with identical
  stack/local/shared values. Binary size is smaller generated (7,040 versus 15,872 bytes).
- The remaining measured structural difference is generic sparse-adjacency metadata traffic. The
  fused 16x16 N4 shader performs 1,472 logical range-row reads and 5,576 logical input-row reads;
  the coordinate-specialized bespoke reference performs zero such metadata reads. The counts are
  derived exactly from the admitted degree buckets and emitted producer-substitution schedule.

Per remand `5148515094`, this is the final measured coder STOP. No further semantic path is invented;
the pointer remains unchanged, no primitive or bespoke permanence is admitted, and no 5.8 work,
merge, or DA self-route is claimed.
