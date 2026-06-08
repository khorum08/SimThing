# NVIDIA FP temporary battery result template

**Temporary file:** `docs/tests/nvidia_fp_temp_<NN>_<name>.md`  
**Track:** `docs/nvidia_fp_determinism_test.md`  
**Date:** 2026-06-03  
**Battery:** `<NN> - <name>`  
**Status:** `PENDING | PASS | FAIL | BLOCKED`

> Cleanup rule: this file is temporary. Keep it only while walking the NVIDIA FP determinism track. Delete all `docs/tests/nvidia_fp_temp*.md` files together after durable conclusions are folded into permanent docs and the principal approves cleanup.

## 1. Commands

```powershell
# paste exact command(s) here
```

## 2. Adapter evidence

```text
requested_adapter_substring:
require_adapter_match:
adapter_inventory:
selected_adapter_name:
adapter_target_matched:
selected_adapter_is_discrete_rtx:
selected_adapter_is_intel:
gpu_tier_ran:
```

Accepted adapter must contain one of:

```text
NVIDIA
RTX
4080
```

If selected adapter is Intel/RaptorLake/Iris/UHD, this battery is not accepted for NVIDIA coverage.

## 3. Results

```text
passed:
failed:
ignored/skipped:
filtered out:
```

Paste the final Cargo result line here:

```text

```

## 4. Tolerance / parity standard

```text
standard: GpuVerified tolerance | ExactDeterministic bit-exact | CPU-only confirmation | other
threshold:
max_error:
bit_exact_entries:
```

## 5. Raw output excerpt

Paste only the decisive excerpts:

```text

```

## 6. Failures / blocked reason

```text
none
```

If blocked, include adapter inventory and exact failure.

## 7. Interpretation

- Replaces prior Intel-only evidence: `yes | no | partial`
- Durable conclusion to fold into permanent docs:
  - `<write one sentence>`

## 8. §0.5 check

Evidence-only temporary battery. No gameplay resource-flow behavior, no recursive allocation change, no CPU planner behavior, no `simthing-sim` semantic expansion, no default session wiring.
