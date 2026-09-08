# BOUND-MARKET-POLICY-SINGULARITY-0 — 15.13 proof packet

Status: PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED.

Dispatch [5588702219](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5588702219)
resumes the same branch and draft PR #2001 under DA ruling
[5588539713](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5588539713).
Updated master `25eeb5c985c57941777184286fbd2557a90fbbf4` was merged normally at
`d9b16f02f9a0e25bfc0e88efaa5553d61a3a9f1b`; no history was rewritten.

HD-RECEIPT: 54102b656e43
ORIENT-RECEIPT: da3e92f7e86f
role: coding
orientation_rule_stamp: 5127541ebd64b1ab
orientation_digest_sha: 0c167ca5128bf1b49604f4195548d42269fa36f0f3fefcc4fadb976967957c6e

The amended HD was rendered and read. Orientation freshness passes; its 48
rendered anchor ACKs are unchanged and carried in the Board/PR return. The
original ingress STOP at `b0181f9b72ce08d7dc5b1702c853a9d70fa989b4` and Board
[5587753005](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5587753005)
remain preserved as historical evidence, not an F1 semantic RED.

## Preserved commit chain

| Stage | Full SHA | Evidence |
| --- | --- | --- |
| 5R canonical baseline | `4e0e7d892235533e390f5ec4d88baf3ca31f9d30` | Exactly the two existing literals; zero semantic production edits; 4/4 mutations; clean-checkout frozen witness PASS |
| F1 semantic RED | `a00754b4fb51a71be406cc311f4b02990fe4eddf` | Clean-checkout constant-score one-market/two-verdict split |
| F2 semantic RED | `da279918be0b8122dc12fc63520d39b6cabdc8b5` | Separate clean-checkout priority-sensitive score split |
| Final semantic edit | `046d362a61dd697e56721d7dc6dd22f7c1a8e6ae` | Delete bound score gate; retain exact admitted Kernel plan/basis; add F3 |
| Sixth semantic E8 roll | `e8d6410cef0d9a1ffb3c858c112b6f63b6bb1e92` | Exactly the same two literals; 4/4 mutations; clean-checkout frozen witness PASS |
| Test observation completion | `9d73015440c44bda7539688c032702def3c79469` | F1/F2/F3 PASS using existing grant and neutral-termination facts; no sealed-source edit |

Two earlier F1 fixture diagnostics are retained: `66dfcec0` found actual bases
`[0.5,0.5]`; `e7ca31f5` then found that opening directly in CPU posture does not
populate the resident-only qualification accessor. Neither is a semantic RED.
The actual F1/F2 fixtures author root intrinsic flow 2 and observe live leaf
bases `[1,1]`; they admit the same resident market before selecting the backend
through the existing pre-execution posture door.

## E8 capture and reproducibility

Required tuple: `EML_RESOURCE_PROFILING`; `rustc 1.95.0 (59807616e 2026-04-14)`,
host `x86_64-pc-windows-msvc`, LLVM 22.1.2. Qualification commands explicitly
enable `eml-resource-profiling` (or its dependency-qualified feature name).
The default workspace battery is separately reported and is not substituted
for qualification evidence.

| Capture | Required pin | Observed pin | Semantic bundle |
| --- | --- | --- | --- |
| Resumed canonical baseline, before economics | `0x1537_b17e_9388_b047` | `0x349c_02b1_c265_e372` | `e8df874da7e17163` |
| Final semantic source, before economics | `0x349c_02b1_c265_e372` | `0xb295_851d_f402_d50b` | `7ffe849db0b2aea4` |

Both captures use the unchanged frozen test at actual `SimSession::open`:

```text
5R capture: ResidentClearing(LiveHead(UnqualifiedAdapter { required: 1528885755714777159, observed: 3790907948833039218 }))
Sixth capture: ResidentClearing(LiveHead(UnqualifiedAdapter { required: 3790907948833039218, observed: 12868337873975432459 }))
```

Each roll changes only `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` in the existing
GPU runtime and `QUALIFIED_RECORD_FINGERPRINT` in the existing independent
parity referee, to identical captured values. Both commits are two insertions
and two deletions. The 5R repair is the DA-ruled replacement of the invalid fifth
capture, not the sixth semantic roll. No seventh roll occurred.

Both cited roll SHAs were checked out detached in a separate worktree. A raw
byte comparison verified all 32 sealed components equal their committed LF Git
blobs before and after each proof; working status was clean. Commands:

```sh
git checkout --detach <5R-SHA-or-sixth-roll-SHA>
cargo test -p simthing-gpu --features eml-resource-profiling resident_clearing_runtime -- --nocapture --test-threads=1
cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_session_integration_conformance_0 ordinary_session_identity_half_and_registry_permutation_cross_real_generations -- --exact --nocapture --test-threads=1
```

For both roll SHAs: mutations **4 passed / 0 failed**, frozen witness **1 passed /
0 failed / 9 filtered**. The ABI mutant captures the actual qualified fingerprint;
the successful frozen session admits that same production pin and executes real
N/N+1 identity and half carry, in both arena orders.

| Mutation | 5R qualified → mutant | Sixth qualified → mutant |
| --- | --- | --- |
| ABI tuple | `349c02b1c265e372` → `5396c9bacd552d93` | `b295851df402d50b` → `939abe14e9138aea` |
| Child-share component | `e8df874da7e17163` → `b73ce42ab1a6cd66` | `7ffe849db0b2aea4` → `9c6af116b4033b9b` |
| Planner component | `e8df874da7e17163` → `63cd90222bcf3ef2` | `7ffe849db0b2aea4` → `a1bb379134959d1f` |
| Temporal component | `e8df874da7e17163` → `9baa83fec4a25c6c` | `7ffe849db0b2aea4` → `e2536d65f89f35f1` |

The original fifth capture depended on historical CRLF bytes in two Driver
files. DA ruled that committed-tree baseline non-reproducible. This resumed
work never reenacted those bytes. No Driver source, build normalization,
component list, comparator, ABI/record shape, dynamic route, or third literal
site was changed. There is no sealed-source delta after the sixth-roll commit.

## F1/F2: genuine REDs and final GREEN

Each RED SHA above was separately checked out clean under the same profiled
tuple and ran its named test with `--exact --nocapture --test-threads=1`.
Both exit 101 at the intended cross-posture assertion, after reaching actual
economics. For each of Step, Run, and Record:

| Program | Scores at priorities `[0,1]` | ResidentRequired before repair | CpuVendorizedOracle before repair | Both after repair |
| --- | --- | --- | --- | --- |
| F1 `Set(1.0)` | `[1,1]` | ACCEPT G/U `[(1,0),(0,1)]` | REFUSE `ResidentBasis` | ACCEPT G/U `[(1,0),(0,1)]` |
| F2 admitted `PARAM(1)` | `[0,1]` | ACCEPT G/U `[(1,0),(0,1)]` | REFUSE `ResidentBasis` | ACCEPT G/U `[(1,0),(0,1)]` |

The actual outer refusal is
`GpuSync(GrowthEntitlement("ordinary growth clearing failed: resident CPU reference input: scope/generation/batch mismatch"))`.
It is the rendered `ConstrainedClearingError::ResidentBasis` from the bound
gate. S=1, requests `[1,1]`, priorities `[0,1]`, same cloned scope/IDs and
observed Current bases `[1,1]` apply to every matrix cell. The GREEN reader
checks published G and the existing neutral-termination fact's canonical G/U.

## F3: cap saturation into the ordinary temporal continuation

The same authored scope/IDs run both postures, Step/Run/Record, and physical
child orders A,B/B,A (observed slot order `[1,2]` / `[2,1]`). Same-band priorities
and actual Current bases `[1,1]` remain fixed. S=51:

| Generation | Authored demand | Prior U | Effective demand | G | U |
| --- | --- | --- | --- | --- | --- |
| 1 | `[1,100]` | `[0,0]` | `[1,100]` | `[1,50]` | `[0,50]` |
| 2 | `[2,3]` | `[0,50]` | `[2,53]` | `[2,49]` | `[0,4]` |
| 3 | `[0,0]` | `[0,4]` | `[0,4]` | `[0,4]` | `[0,0]` |
| 4 | `[0,0]` | `[0,0]` | `[0,0]` | `[0,0]` | `[0,0]` |

The CPU continuation's final G/U is private until ordinary neutral termination.
The test therefore runs complete uninterrupted prefixes of lengths 1–4,
retiring each only after its final settlement to observe that generation's
actual products through the existing fact. Each prefix keeps one session,
lease, execution identity and continuation throughout all of its generations.
Every intermediate published G is checked too; resident materializations also
check intermediate U. Prior-U observations are used only in test assertions,
never fed into execution. Next demand is authored only after the preceding
generation completes. The 3rd/4th generations detect duplicate or delayed carry.

All 48 endpoint/posture/loop/order cells pass. History remains append-only,
each canonical final product appears exactly once, actual grant totals stay
within 51 and demand caps, zero products remain represented, and no refusal or
structural placement row appears. Representative actual transcript:

```text
15.13 F3 ResidentRequired/Step/reverse=false slots=[1, 2] N=1: authored=[1, 100] observed_prior_U=[0, 0] effective=[1, 100] canonical G/U=[(1, 0), (50, 50)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 ResidentRequired/Step/reverse=false slots=[1, 2] N=2: authored=[2, 3] observed_prior_U=[0, 50] effective=[2, 53] canonical G/U=[(2, 0), (49, 4)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 ResidentRequired/Step/reverse=false slots=[1, 2] N=3: authored=[0, 0] observed_prior_U=[0, 4] effective=[0, 4] canonical G/U=[(0, 0), (4, 0)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 ResidentRequired/Step/reverse=false slots=[1, 2] N=4: authored=[0, 0] observed_prior_U=[0, 0] effective=[0, 0] canonical G/U=[(0, 0), (0, 0)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 CpuVendorizedOracle/Step/reverse=false slots=[1, 2] N=1: authored=[1, 100] observed_prior_U=[0, 0] effective=[1, 100] canonical G/U=[(1, 0), (50, 50)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 CpuVendorizedOracle/Step/reverse=false slots=[1, 2] N=2: authored=[2, 3] observed_prior_U=[0, 50] effective=[2, 53] canonical G/U=[(2, 0), (49, 4)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 CpuVendorizedOracle/Step/reverse=false slots=[1, 2] N=3: authored=[0, 0] observed_prior_U=[0, 4] effective=[0, 4] canonical G/U=[(0, 0), (4, 0)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
15.13 F3 CpuVendorizedOracle/Step/reverse=false slots=[1, 2] N=4: authored=[0, 0] observed_prior_U=[0, 0] effective=[0, 0] canonical G/U=[(0, 0), (0, 0)]; actual bases=[1,1]; uninterrupted prefix retired at N+1
```

The first post-roll test attempt used a resident-only materialization reader for
CPU results and failed that observer assertion after CPU execution accepted.
The test-only reader correction uses existing grant and neutral-termination
facts. No F3 arithmetic, continuation or qualification change was needed.

## Deletion, legacy byte identity and singular authority

Only `clear_resident_oracle_input` changes semantically. The pairwise score map,
score validation call and nested score/precedence comparison are deleted. No
disabled, shadow or replacement equivalence gate remains. The Kernel call
still receives exactly `&input.plan`, `&input.values`, and `input.n_dims`.
The immutable admitted plan controls precedence and Current live AllocatedFlow
controls share. Score evaluation occurs after settlement as metadata, with
signed zero canonicalization retained; invalid score metadata cannot refuse
bound economics.

Against `085dd36d2225ff9088f706d4ee17cfc63b8b0794`, the complete file prefix before
the bound function (11985 bytes) and suffix after it
(26014 bytes) are byte-identical. This includes the legacy
score method, unqualified oracle entry/body and its original dispatch guard:

```text
prefix SHA256 691a19ee518c1b27a2f9b80c114b9fc6f08957bf475a5ee1b3b8abcb2d1a39a3
suffix SHA256 84390a4d566ed052f8603e511afb893147f2cb5e97d80d16926693fb01aa6ce8
```

No authority, door, plan representation, Draw law, cap/Q149/Hamilton law, seal,
zero/departure/subset law, U recurrence/once-mint, continuation, history or
consequence ingress was added or altered. These remain the existing production
owners; the only economic change removes the bound score's competing veto.

## Final verification and scope ledger

The focused three-test matrix passes at `9d73015440c44bda7539688c032702def3c79469`.
The Board/PR return supplies the exact final tested head, actual full/frozen
test totals, structural results, hosted artifact identities, and any INSPECTs.
The final battery uses these commands, with `CARGO_BUILD_JOBS=2`:

```sh
cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test bound_market_policy_singularity_0 -- --nocapture --test-threads=1
cargo test -p simthing-spec --features simthing-gpu/eml-resource-profiling --all-targets --no-fail-fast -- --test-threads=1
cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_session_integration_conformance_0 --test generation_abort_safety_0 --test tree_execution_authority_lifetime_0 --test exact_cap_projection_0 --test resident_clearing_parity_0 --test resident_clearing_apportionment_0 --test resident_filter_substrate_binding_0 --test recursive_resource_filter_formalization_0 --test persistence_deformation_port_0 --test consequence_ingress_0 --test departing_stream_disposal_0 --test departure_live_basis_convergence_0 --test recursion_axis_conformance_0 --no-fail-fast -- --nocapture --test-threads=1
cargo test -p simthing-spec --doc constrained_clearing -- --nocapture
cargo test -p simthing-core --doc generation_stamp -- --nocapture
cargo test --workspace --all-targets --no-fail-fast -j 2 --quiet
```

Structural certificates cover inventory/drift (including its proof), lifecycle,
constitutional census/self-test, sanctioned-surface digest, anchors/self-test,
detachability/self-test, Plan/observation/slot/Overlay censuses, deletion guard,
committed-head Agent Scan and actual hosted Doctrine Scan/Exec artifacts.
No coding self-triage, final local clearance, relay-lint or merge is performed.

| Changed file | Scope |
| --- | --- |
| `crates/simthing-spec/src/spec/constrained_clearing.rs` | Bound-function downclassification only |
| `crates/simthing-gpu/src/resident_clearing_runtime.rs` | Existing pin literal only, 5R then sixth |
| `crates/simthing-workshop/tests/resident_clearing_parity_0.rs` | Independent existing pin literal only, 5R then sixth |
| `crates/simthing-workshop/tests/bound_market_policy_singularity_0.rs` | F1/F2/F3 actual-session proof |
| `docs/tests/bound_market_policy_singularity_0_results.md` | This packet and historical diagnostics |
| `docs/tests/departing_stream_disposal_0_results.md` | Accepted bounded historical-header correction; semantic evidence unchanged |
| `scripts/ci/test_inventory.tsv` | Three required test rows |
| `scripts/ci/anchor_reach_log.tsv` | Original actual changed-anchor query row |
