# EML-RESOURCE-CLASS-ADMISSION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.7)
- Status: **CODER STOP — inherited performance gate failed on the supported adapter**
- ORIENT-RECEIPT: `e563c4399d73`; rule stamp: `5319c193d38da6ce`
- HD-RECEIPT: `85bfbf0e0e33`
- Remand authority: Board comment `5148327947`
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
