# Test Residue Class Legend

This legend binds permanent-residue classes to owning doctrine. A class label is not enough by itself; the row must point at the live owner and remain the canonical survivor for that owner.

| Class | Owning doctrine | Membership test |
|---|---|---|
| `oracle-parity` | Core section 4 / constitution section 0.7 | Bit-exact CPU-to-GPU, CPU-to-kernel, or CPU-to-live-op oracle parity for the live execution path. |
| `stead-required` | `docs/stead_spatial_contract.md` section 8 | Row lives inside the section 8 named STEAD/mapgen required suites or a directly cited helper surface. |
| `seal-proof` | Admission-substrate/kernel sealed-boundary contract | Canonical compile-fail, trybuild, or scanner/probe fixture proving a live sealed boundary. |
| `golden-byte` | Determinism and canonical-corpus doctrine | Byte-identity, canonical artifact, deterministic diagnostic, or deterministic replay proof for a live emitted surface. |
| `doc-named-invariant` | `docs/invariants.md` or another live non-archive doctrine doc | The row is explicitly named by a live doctrine document as the invariant proof. |
| `determinism` | Constitution section 0.7 | Deterministic replay, order, or byte behavior not already represented as `golden-byte`. |
| `behavior-regression` | TIER5 judgment class | KEEP rows must name the exact regression with `catches: <specific regression / bug-ref nothing else owns>`. Boilerplate is not enough. |
| `escaped-bug` | TIER5 judgment class | KEEP rows must name the exact escaped bug with `catches: <specific regression / bug-ref nothing else owns>`. Boilerplate is not enough. |
| `custom_layout_ethics_axis` | Owner/DA carveout recorded in `docs/invariants.md` | Retained unless DA explicitly amends the carveout. |

Current `behavior-regression` rows in the inventory are AUDIT rows, not permanent survivor shields. They may be processed by later boundary waves unless promoted to KEEP with a specific `catches:` note and an owning boundary.
