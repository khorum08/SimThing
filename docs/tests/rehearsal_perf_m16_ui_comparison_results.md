# M16 B1 UI comparison — evidence collection in progress

Status: PROBATION / physical comparison pending / OPEN / UNMERGED.
Dispatch: Board 5654699942; DA endpoint ruling 5651557467; Owner loop 5653843173.
Base and frozen runtime: `a8c117313dbe3796aa160ba12cad1cd677bdc5e6`.
Branch: `codex/0088-m16-ui-comparison`. HD-RECEIPT: 47010c34edf2.
Carried coding ORIENT-RECEIPT: 28f56884d309.

This draft records the frozen protocol, collected groups and interrupted Owner attempts. It is not a
completed comparison, proof-present PR, budget pass or migration recommendation.
The runtime contains the merged Leaf A and B0a instruments. No Mapeditor
production code, renderer, dependency or simulation authority changes are made.

## Frozen matrix and quantity boundaries

The same Meridian source is used for both clients: source
`fnv1a64:ee4e4df9e8c9fbd9:5798`, profile `fnv1a64:bfcbc44323b304bf:23530`,
base JSON `fnv1a64:c49f9ca3c8c75e77:20370`, dependency manifest
`fnv1a64:2f064bfb3e043aa0:72`. Seven systems, six links, two owners, two sites,
four cohorts per site, thirteen energy RF participants, two recipes and four
material loci. No workload variant or large-list scaling claim is introduced.

Five conditions each have three paired repetitions (30 captures). Pair order
is Egui/Native, Native/Egui, Egui/Native. Captures target at least 15 seconds;
the fixed minimum usable duration is 10 seconds. All raw samples and rejected
attempts remain. Owner waits at least five seconds after setup before F9;
that warmup is operator-qualified, not independently timestamped.

| Condition | Fixed semantic operation | Physical capture status |
| --- | --- | --- |
| Paused | Pinned G0, 1x/TPS10, steady visible pane | Two six-capture runs complete; fresh-load setup qualification pending |
| Running | Reload G0 before each repetition; Play 1x/TPS10; warmup; F9; F10 before Pause | PENDING, 0/6 |
| Numeric editing | Paused G0; three Apply TPS pairs 12.5 then 10 through the selected client | COMPLETE: 6/6 valid, three paired repetitions, same resident; earlier failures retained |
| Resize | Paused G0; three OS maximize/restore cycles with the same starting/ending size | 6/6 frozen capture checks pass, same resident; shorter duration/dwell qualification pending |
| List interaction | Same actual row-order sequence 1,2,3,4,5,7,6; map picks for Egui, list/Down for Native | PENDING; new row-order Egui passes, Native starts with F selected; zero valid pairs |

The Owner launcher uses the existing Windowed setting and 1600x900 startup
size in a copied, process-scoped settings directory. Other copied presentation
settings remain unchanged; runtime physical size and scale are recorded. The
resize condition changes geometry through existing OS controls and requires
the same observed size sequence on each side. The list group uses the same
Overhead view on both sides. These are named B1 presentation conditions;
they are not falsely described as the 1920x1080 B0b capture's window condition.

Native-on retains the egui acceptance pane. Whole-frame/process measurements
characterize coexistence overhead, not an isolated replacement client. Map
picking and a native scrolling list are different adapters over the same
selection owner; no equal-widget construction-cost claim is made.

| Quantity | Instrument and reporting rule | Current evidence |
| --- | --- | --- |
| Raw full-frame cadence/tails | Merged B0a `Time<Real>` unsmoothed intervals; all raw samples; p50/p95/p99/max; nearest rank; p95 <=16.7 ms characterization | Partial: Paused, numeric and resize groups recorded with stated qualifications; p95 budget not met |
| Shared CPU projection | Same `shared_clock_and_observation_projection` computation; host wall elapsed, not OS CPU accounting | Partial: exact same-frame/publication pairs in Native-on captures |
| Adapter CPU | Each named native/egui scope separately with its existing boundaries; never sum nested scopes | Partial: per-scope distributions retained in group artifacts |
| Observation freshness | Publication to first matching client consumption, full scene/resident/generation stamp; pre-capture publication ages retained separately | No new publication during Paused/numeric/resize G0 captures; Running pending |
| Process memory | Read-only PID private bytes and working set immediately after durable raw export, before analyzer; process total including capture/export allocation history | Partial: PID-bound snapshots and signed pair differences retained |
| Input latency | INSTRUMENT-INVALID / UNAVAILABLE; no qualified endpoint; p95 <=100 ms UNEVALUATED | Existing B0b/R1 failure proof |
| First actual display | No qualified matching displayed endpoint | UNAVAILABLE |
| Direct render/GPU cost | No directly admitted per-client timing instrument | UNAVAILABLE |
| Separate simulation/presentation GPU memory | No direct separately measured allocation pair | UNAVAILABLE |

Same-publication freshness residuals and, when both calls fall in one actual
recorded frame with unchanged publication and runtime state, shared CPU
projection residuals use Native minus Egui and retain negative values. Other
capture statistics/memory are paired by condition/repetition. Any chronological
frame-ordinal pairs are explicitly descriptive independent-run pairs, with
unmatched tails retained, never a causal frame association or latency join.
Within-capture publication freshness is separate from the age of a paused
publication that predates F9. No smoothed FPS, residual GPU estimate or
submission/prepare timestamp substitutes for a requested quantity.

## First Owner attempt and instruction repair

Owner reported the first Paused group failed. Four v1 launch attempts are
preserved, all with Studio exit code 0: `b1-20260913-125517-9191e485` closed
before export; `b1-20260913-125805-347332eb` logged F10 export with no retained
capture; `b1-20260913-130004-18284eea` logged F9 start refused because a capture
was already retained. Those logs establish the instrument state at the command,
not why an earlier key was absent or repeated. None produced a raw export.

`b1-20260913-130128-dab634ef` exported the first Egui case: 45.2129166 seconds,
3,288 frames, Native enabled throughout all recorded runtime facts, and 3,291
native shared-projection calls. Studio lost focus at +9.2013466 seconds and
regained it at +20.2998492 seconds. The three failures are correct: unexpected
Native state, Native projection in the disabled baseline, and lost focus.
Source/profile/dependency identities, paused G0, TPS10 and 1600x900 geometry
matched. No sample is salvaged, relabeled Native, trimmed or counted as a pass.
The frozen v1 launcher's `completed_captures: 1` counts its analyzed export;
the report has `valid: false`, so that v1 checkpoint had **0 accepted captures**.

The v1 card said to select the client with F8 alongside the explicit checkbox
state. F8 toggles, so that instruction could undo a correct setting; causation
is not established from the raw state alone. Revision `m16-b1-ovl-v2` uses the
mouse checkbox state, states that F9 starts silently, requires one F9/F10 press
per capture, and makes the existing focused-window rule explicit. Console
inspection and screenshots occur after F10. Only external card/guide/protocol
wording changes; Studio, analyzer, settings, operation sequences, thresholds,
sample retention and acceptance rules are byte-identical or verified unchanged.
No production edit or new status pane is introduced.

| Artifact | SHA-256 |
| --- | --- |
| Failed raw `01-paused-r1-egui.json` | `0a88c49b3757b740972817a3dda5283377dec8d77b21c1bb267573c20a358a03` |
| Original rejection report | `f42b0522996fda2d87cea459e81dec7f64e34bce98ced97355a6535f95f1b148` |
| Owner screenshot | `06f6994e836cd7b3c1c1e4c38d2c100403bb9eaaca16c363a13cb48f0207befc` |
| Detailed failure audit | `de1ad587c5f05fe1f7c5900be88abc6d7ebe2de3781a0542a8be35ce7f89b00c` |
| v2 bundle manifest | `07ab54edbf8665a92bb00fcc4ead124defb243a3378acc2ee2bcb79ee4a866f3` |
| v2 protocol | `eca99546e881053c518e12161f005408e0a92a1e461acf512b608a5b3d062bd8` |

The common gitdir's `0088-m16-b1-owner-failure` directory retains the screenshot,
audit/reproducer and v2 verification. All **73 original indexed run entries**
verified unchanged, and the complete v1 bundle is archived under
`SimThing-0088-OVL-B1/versions/m16-b1-ovl-v1`. Replaying the actual failed raw
capture under v1 and v2 produces identical analysis and rejection. This is
instruction repair validated locally; the next attempt is recorded below.

## Valid first capture; launcher transition failure

Run `b1-20260913-131121-9db4662f` passed the first Paused/Egui capture checks:
32.4866474 seconds, 2,433 frames, Native disabled, focused throughout observed
facts, paused G0, correct source/profile/dependencies and 1600x900 geometry.
The post-export memory sample belongs to the same PID/start identity (16140).
The launcher then failed before preparing the second card, with
`File.Replace` reporting "The path is not of a legal form." Studio exited 0.
The old active config and next staged config both remained intact.

At that checkpoint this was **one individually valid unpaired capture, zero complete pairs**.
It is retained in full as interrupted-run evidence, not silently discarded or
paired across a fresh process. Its raw-frame p95 is **42.3209 ms**, above the
16.7 ms characterization threshold; analyzer validity is not a frame-budget pass.
Process private bytes were 974,757,888 and working set 713,322,496, sampled
0.0980549 seconds after the raw file's last write, at the declared post-export
boundary. These single-run observations do not meet the repetition requirement
or support a client comparison.

The original config activation statement reproduces the same failure under
the actual Windows PowerShell 5.1 launcher host. Its `$null` backup argument
is bound to an empty path. Launcher revision `m16-b1-ovl-v3` supplies a unique
nonempty backup path and preserves the prior active config on every replacement.
Raw metadata also pins launcher revision/hash. Studio, analyzer and the complete
v2 measurement protocol are unchanged. No production edit or acceptance change.

Real filesystem checks exercised the actual launcher activation statement:
the old first-to-second failure; all six case activations/five replacements in
the frozen Egui/Native, Native/Egui, Egui/Native order; exact current/previous
config bytes; and safe refusal with a locked destination followed by successful
replacement after release. All eight check rows PASS in Windows PowerShell 5.1.
This verifies config progression, not six physical Studio captures.

| Artifact | SHA-256 |
| --- | --- |
| Valid raw `01-paused-r1-egui.json` | `4a451459423959c03b16d7d927a973cb731fbb8202d5f48a33fc8856211c69a1` |
| PID memory snapshot | `c17d60b08f5b217201c0715ecee5accae09e6d0a8c973ffa86b1cf5701f76de4` |
| Owner transition-failure screenshot | `9ba872c3ad0d1ed5ac631126ca6b58909d245bd08788dc581345fe0a954c06d4` |
| Preserved-runs audit | `8e17bae1efd8c9e1a5ffce476236964532018c9528e94a11577f7bef8dd44ee6` |
| Actual filesystem transition checks | `99739015d9d6561feaa8c2c9e304dc8afc9bac297644d45e341a394e9c17bb96` |
| v3 launcher | `6757113ef68ffb5166a2ac24eb14e4539025224da67c1c50c8140f1962755010` |
| v3 bundle manifest | `c0edd89964574ed41e3fabb490e0175fe5f3b86bd4a734e4ff569ab6970df77f` |

All **97 original indexed entries** across five runs verified unchanged.
Both existing raw reports reproduce identically with the unchanged analyzer;
the rejected v1 capture remains rejected and the successful v2 capture remains
valid. The full v2 bundle is archived under `versions/m16-b1-ovl-v2`. Audit,
screenshot and reproducer are in the common gitdir's
`0088-m16-b1-config-replace` directory. The repaired bundle has 14 frozen files.
The next Paused attempt was instructed to restart from case 1 to preserve the
one-process setup. Subsequent collection is recorded below.

## Group 1 collection and paired setup qualification

Owner reports Group 1 finished and supplies Egui/Native screenshots. Two runs
completed all six captures, each with no launcher failure and Studio exit 0:
`b1-20260913-132828-6183d8fe` (PID 35464, 9,867 frames) and
`b1-20260913-133909-27418d42` (PID 25708, 7,601 frames). Both are retained;
neither is selected based on performance. All 12 individual captures reproduce
their valid analyzer reports: pinned workload, focused, paused generation 0,
1x/TPS10, appropriate Native state, physical 1600x900 and scale 1. The later
Egui repetitions 2 and 3 lasted 12.3766909 and 12.9113857 seconds: below the
15-second target but above the frozen 10-second usable minimum. No threshold
change is made to admit them.

**Protocol difference:** each completed run's scene/resident identities progress
1 through 6. The source was re-admitted before every capture, although the
Paused card intended a single load per group. The guide's instruction to repeat
the load/setup step was ambiguous. Both clients used the same pinned workload
and started at G0, but these are **fresh-load Paused pairs**, not proof of the
planned single-load condition. Orchestration qualification remains pending.
No IDs are normalized into a fictional shared resident; no new resident is
joined to an earlier publication. The grouping below describes actual runs.

The intermediate `b1-20260913-133654-20ab13e7` run has one valid fresh-load pair
(2,383 frames) followed by `M16 export: no retained capture`; its Studio exit
is 0. It remains a separate interrupted repeat. The earlier unpaired valid
capture and rejected capture also remain. Partial repeats do not fill missing
repetitions in another process. Across all eight attempts, **242 indexed files**
verified unchanged, and every saved raw report reproduced identically.

All 12 completed-run frame p95 values exceed 16.7 ms. Values below are nearest
rank on the full raw intervals; signed residuals are Native minus Egui.

| Run | Rep | Egui p95 ms | Native p95 ms | Delta p95 ms | Shared projection mean delta ns | Private bytes delta MiB | Working set delta MiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 132828 | 1 | 39.8964 | 39.4535 | -0.4429 | +2.652 | +27.082 | +28.816 |
| 132828 | 2 | 42.2687 | 41.3685 | -0.9002 | +62.871 | -4.035 | -4.586 |
| 132828 | 3 | 39.8933 | 41.5156 | +1.6223 | -17.752 | +1.680 | +0.824 |
| 133909 | 1 | 42.5438 | 42.4652 | -0.0786 | +27.077 | +97.504 | +97.578 |
| 133909 | 2 | 42.5901 | 42.5901 | 0.0000 | +10.026 | +60.656 | +61.445 |
| 133909 | 3 | 42.5011 | 40.6004 | -1.9007 | +97.349 | +1.777 | +1.922 |

These capture-statistic residuals compare the named computation and actual
fresh-load conditions; native-on still includes Egui. Memory is process total
after export, with re-admission/capture allocation history, not a UI-only or GPU
cost. Separate raw adapter scopes remain in the original JSON and derived
packet; nested scope durations are never summed. No generic Native speed or
memory advantage is inferred from mixed-sign, order-dependent observations.

The Native-on captures also retain **8,817 direct shared-projection CPU pairs**
inside the same recorded frame/publication identity. Every capture was paused
at G0, so no new publication-delivery samples exist; pre-capture publication ages
remain separate. The descriptive ordinal residual artifact retains **7,929
cross-capture frame pairs and all 1,610 unmatched tails** for the completed runs,
plus the interrupted pair separately. Ordinal pairing has no causal frame or
latency identity. Required displayed latency remains unavailable/unevaluated.

| Preserved artifact | SHA-256 |
| --- | --- |
| First completed run raw index | `11fa6f5f50bf91395b8657ca0d2f308af386d0c318df7c639f2999cf55178525` |
| Second completed run raw index | `e5b7ea8ba8499cd0ed30c8a1dc744abf70d03ce0a71c8b1f812f71a146cd3548` |
| Paused results/audit JSON | `d1bfdabab880920bd3439ac78bb02d084c8a14615cd6bba4de2e64cc6e558240` |
| Ordinal residuals including tails | `0e826484f1eb1f22b6527fa56105086bba091fd7855e8904fa8df989178b62d4` |
| Audit/aggregation script | `5909ea2a0d5f97cdf2e964b18d6276006a927253d234cabc7bf19ec2b934409f` |
| Owner Egui screenshot | `5b035d4b3622a8b56602721589a73fd31c998fcc22ce9618cf242c148ee96a0c` |
| Owner Native screenshot | `8540585633ca06249cb57ba021ca5670b55ee99eea9dc2dcdb6534d900784f5e` |

The common gitdir's `0088-m16-b1-paused-results` directory holds the reproducer,
derived machine-readable results, signed residual arrays and screenshots.
Screenshots show client configuration but no run ID or measurement timestamp;
quantitative geometry/provenance comes from the raw packet, not image dimensions.
Portable raw-artifact packaging remains part of final B1 handback. No frozen
bundle, analyzer, protocol or production code changes in this collection return.
Groups 2-5 remain pending; Group 2 should load once at group start, then retain
that resident through all six captures. Only the Running group reloads per capture.

## First numeric capture: no applied TPS transitions

Owner supplied the Group 2 rejection screenshot for
`b1-20260913-213947-06892d58`. The first Egui capture exported successfully,
then failed only the required numeric sequence check: **observed `[10]`**,
required `[10,12.5,10,12.5,10,12.5,10]`. Duration 15.1010401 seconds, 1,131
frames, zero Runtime fact changes, correct pinned source/profile/dependencies,
focused, Native disabled, paused G0/1x and 1600x900 scale 1. Studio exited 0.
PID 36136/start identity matches the retained post-export memory sample.
No numeric capture is accepted or relabeled as a Paused sample.

B0a records applied TPS, not draft text, editor keyboard focus, key/button
history or the timing of edits outside the capture. This raw file therefore
does not establish why no change was applied. Owner clarification was requested
about whether the draft and applied readout visibly changed; the physical cause
remains undetermined at this checkpoint.

Source inspection at the frozen revision confirms `app/ui.rs:2024` uses an
Egui single-line text edit; lines 2025-2029 apply on focus loss, Enter or the
Apply TPS button. The shared transport parses a finite positive 12.5 normally.
Pinned **egui 0.32.3**, `src/widgets/text_edit/builder.rs:109` and `:1026`,
documents and implements surrendering single-line field focus on Enter.
The original card omitted explicit refocusing before each subsequent value.
That is a verified instruction gap, not a proven explanation for the whole
observed `[10]` sequence.

External bundle **v4**, measurement protocol **v3**, clarifies: after F9,
perform exactly six edits `12.5,10,12.5,10,12.5,10`; click inside the selected
client's editable box before every value, Ctrl+A/type/Enter, verify the applied
readout below changes to 12.500 or 10.000, and wait at least one second. If the
draft/readout fails to change, export and retain a Studio screenshot showing
both. The guide also resolves load-once wording for groups 1-4; Running alone
reloads per capture. These instructions do not claim a physical fix or upgrade
the earlier fresh-load Paused qualification.

Only README/protocol wording changes; launcher, Studio, analyzer, semantic value
sequence, quantities, durations and all acceptance rules remain unchanged.
All **264 indexed entries across nine runs** verified unchanged. All **17 saved
reports** replay identically against the clarified protocol (15 valid, two
rejected); no prior result is rehabilitated. Complete v3 bundle archived under
`versions/m16-b1-ovl-v3`. Updated bundle validates all 14 frozen files.

| Artifact | SHA-256 |
| --- | --- |
| Rejected numeric raw JSON | `c010402441b2db7ccc01ddb2bd79f40c3170bad9af5b7a9e3a6ec2e64dec9eb8` |
| Original rejection report | `23aebea31e76336779dbd857caee369ab59ae90c6fa6a7ede00a13dcdefc6877` |
| Owner screenshot | `c563b417804808a1edba9669e4279f8d9ab5812cc3b126042d347d70a7e59df5` |
| Read-only failure audit | `d69ef121f99c6963471e641f3ac69b5b52203286e4fa2546dcf70683bef239e5` |
| v4 manifest | `6848fee39bf5333d962d8172a5f72a6b3c9154d18c39019b60c696bdbb4c8478` |
| Clarified v3 protocol | `d724c3ee3ca8e022bb76f2a9508220f059536aeb473b6c51cec4e13f30518e3d` |

Audit/reproducer/screenshot are in the common gitdir's
`0088-m16-b1-numeric-failure` directory. Physical numeric success remains pending;
source inspection and report replay are not desktop interaction proof.

## Numeric retry: Egui valid, Native has an extra transition

Owner supplied Egui, Native and console screenshots for
`b1-20260913-215610-e01f7c05`. Egui repetition 1 is valid: 39.1228876 seconds,
2,931 frames, exact applied sequence `[10,12.5,10,12.5,10,12.5,10]`.
Native repetition 1 has the same six correct changes **followed by another
12.5**: `[10,12.5,10,12.5,10,12.5,10,12.5]`. The sole rejection is the
extra transition; 47.5077329 seconds and 3,559 frames are preserved in full.

| Native time from F9, seconds | Applied TPS |
| ---: | ---: |
| 0 | 10 |
| 8.0358980 | 12.5 |
| 12.2128722 | 10 |
| 27.0991893 | 12.5 |
| 31.9002873 | 10 |
| 36.0913877 | 12.5 |
| 41.7435028 | 10 — required sixth change complete |
| 45.1050675 | 12.5 — extra seventh change |
| 47.5077329 | F10 stop; last observed TPS remains 12.5 |

The final Native screenshot shows applied 12.500, consistent with that ending.
B0a records the applied changes, not the exact originating physical key/button;
no claim is made about the cause of the additional change. Both captures kept
the same scene/resident 1, pinned source, paused G0, focus, 1x, physical 1600x900
and scale 1; only the expected Native flag differs in their initial facts.
PID 41284/start identity matches both memory snapshots. Studio exited 0.

This provides one valid **unpaired** Egui numeric capture. The Native capture's
passing prefix is not trimmed into acceptance, and its failed result is not
paired as a valid comparison. Egui p95 is 39.5591 ms; rejected Native p95 is
39.5766 ms, retained only as failed-attempt characterization. Neither implies
a 16.7 ms budget pass or qualified client difference.

All **293 indexed entries across ten runs** verified unchanged, and both new
reports replay identically. No launcher/protocol/analyzer/production changes
are warranted by this exact-sequence finding. The existing six-edit card remains
current: after the third return to 10, stop editing; if needed wait at 10 until
the minimum target duration, then F10. Physical Group 2 completion remains pending.

| Artifact | SHA-256 |
| --- | --- |
| Valid Egui raw JSON | `9c567bbad50136de4a8fdc513701fd667040f640d2a63a509ea1ee628a96e98b` |
| Rejected Native raw JSON | `1e7fdec2bf9fcb40785ec6e577c357158dabca4b03d15ce3b6ff9cf2bc0da17e` |
| Egui report | `fd5180dddd7a582b5297c7c216343494cd4a72975513a8c5182a5b8fb3c9d1fe` |
| Native report | `c9240d69f1b92e10b208fcdd362267cc5f85766c6b35dd8bd8bf1459034875e9` |
| Detailed audit | `2fa27e0c7c5073d8d7fb776d9c49f11d090118161a6336c7c22d9d43e2ca9455` |

The common gitdir's `0088-m16-b1-native-extra-edit` directory retains the audit,
reproducer and all three screenshots with hashes. Earlier failures remain intact.

## Numeric group complete: three valid pairs on one loaded resident

Owner reported OVL success with Egui and Native screenshots for
`b1-20260913-221404-2c9b6380`. All six original reports replay identically and
are valid. Each capture records exactly `[10,12.5,10,12.5,10,12.5,10]`, with
at least 2.922759 seconds between successive applied edits. All exceed the
15-second target. The sequence and timing come from applied Runtime facts;
the instrument does not record individual keystrokes or field-focus history.

All six captures kept **scene/resident 1**, paused G0, pinned source/profile/
dependencies, 1x, focused, panels visible, no selection, physical 1600x900 and
scale 1. The Native flag is the only differing initial fact. This group meets
the planned single-load setup. It does not resolve Group 1's fresh-load
qualification. PID **35336** and process start identity bind all six post-export
memory snapshots. Studio exited 0; the log has no M16 refusal/export warnings.

The order was Egui/Native, Native/Egui, Egui/Native. All **12,109 full-frame
intervals** remain, without smoothing, shortening or tail removal. Frame
statistics below are milliseconds; p95 does **not** meet 16.7 ms in any capture.
Valid collection is distinct from passing that performance budget.

| Repetition/client | Duration, seconds | Frames | p50 | p95 | p99 | Maximum |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 Egui | 26.5656849 | 1,989 | 3.9104 | 41.7662 | 42.7793 | 45.5682 |
| 1 Native | 34.3822358 | 2,574 | 3.8281 | 40.7399 | 42.2896 | 45.2894 |
| 2 Egui | 26.8383319 | 2,010 | 3.8084 | 41.8931 | 42.9368 | 47.3578 |
| 2 Native | 24.8906034 | 1,864 | 3.7762 | 39.6973 | 41.8541 | 44.9998 |
| 3 Egui | 23.6243451 | 1,767 | 3.9745 | 42.1089 | 43.1128 | 45.8320 |
| 3 Native | 25.4513520 | 1,905 | 4.0177 | 42.1362 | 43.1970 | 45.7423 |

| Repetition | Native minus Egui frame p95, ms | Private bytes difference | Working-set bytes difference |
| --- | ---: | ---: | ---: |
| 1 | -1.0263 | +11,882,496 | +11,390,976 |
| 2 | -2.1958 | -663,552 | -675,840 |
| 3 | +0.0273 | +1,118,208 | +856,064 |

Memory values describe the same process after each raw export, including its
allocation history, not isolated client or GPU memory. All absolute samples and
export lags are in `numeric-results.json`. Native-on still coexists with Egui.
Different manual edit timing and capture duration limit causal interpretation
of independent-capture differences. Both positive and negative values remain.

The frozen analyzer admits **5,488 exact same-frame/same-publication shared
projection CPU pairs** within the three Native-on captures, excluding frames
with Runtime changes or ambiguous calls. These are host wall elapsed timings
of the same computation, not OS CPU accounting or adapter totals.

| Native capture | Direct pairs | Minimum residual, ns | p50, ns | p95, ns | p99, ns | Maximum, ns |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Repetition 1 | 2,168 | -3,400 | +100 | +500 | +900 | +12,800 |
| Repetition 2 | 1,641 | -4,600 | +100 | +600 | +1,300 | +8,400 |
| Repetition 3 | 1,679 | -3,100 | +100 | +600 | +1,000 | +4,300 |

The original reports retain every direct residual and unpaired-call count.
Each named adapter scope has its own distribution in the aggregate; nested
scopes are never summed. Separate cross-capture shared-projection statistic
differences are explicitly identified in `pairs`. The additional chronological
frame-ordinal file preserves **5,620 descriptive pairs and 869 unmatched tail
intervals**, with no causal event alignment or latency claim.

All consumed publications are G0 and predate their captures. There are **zero
within-capture fresh publication delivery samples**; retained pre-capture ages
do not substitute for freshness or first display. Required input latency remains
INSTRUMENT-INVALID / UNAVAILABLE, p95 <=100 ms UNEVALUATED. This group makes no
migration recommendation or historical performance-improvement claim.

All **350 indexed entries across eleven runs** verify unchanged. Six new reports
reproduce exactly, and all 14 frozen bundle files plus pinned source/Python files
verify. Earlier failures and the valid unpaired Egui retry remain separate; no
failed prefix is promoted into this complete group. Bundle v4, protocol v3,
analyzer, launcher and production executable are unchanged.

| Artifact | SHA-256 |
| --- | --- |
| This run's raw index | `62c5c6cc135d673166470903978657419f35f820a24ad99d2d099cb2c98038a1` |
| Numeric aggregate/audit | `8cdb96a4e5f2da41f09cd721afc6b22a2b21e416939dfb7ccac0296fdeaccf65` |
| Chronological residuals and unmatched tails | `07f78ded0453a45a2dc94b913298dbf67bfad25eafaaacb9de8cc885aefa2612` |
| Read-only aggregate script | `637183dad298da6b990285ab88bd136d662355ba2c592a6e188c5e05b238f3d4` |
| Owner Egui screenshot | `986a11e3a326b2e042eb0ae61dd2947bc79663a8c16b262275a1949325057372` |
| Owner Native screenshot | `112e93d06cef948085ff44a968b73b7c672319eab0db3e018733bcc70b1ad17b` |

The common gitdir's `0088-m16-b1-numeric-results` directory contains the aggregate,
reproducer, residuals and copied screenshots. Original raw/config/report/memory
files remain in the identified OVL run directory, with their hashes in both
the raw index and aggregate. Both screenshots show applied TPS10; Native also
shows its editable field focused. They corroborate visual client state but lack
visible run/time identifiers and do not independently bind quantitative samples.

Group 2 collection is complete. Groups 3-5, Group 1 disposition, final portable
packaging/comparison/decision, hosted Doctrine/full INSPECT and fresh Clearance
remain pending. Next Owner group is **3 — Window maximize and restore**, through
the unchanged launcher, loading the source once for all six captures.

## First resize capture: four cycles and an ambiguous card

Owner supplied the Group 3 rejection screenshot for
`b1-20260913-223431-74f244c0`. The first Egui capture contains **four complete
maximize/restore cycles**, where the frozen semantic sequence requires exactly
three. Its physical size starts and ends at 1600x900, alternating with 1920x1032.
The original report replays identically; this is its only rejection reason.

| Seconds after F9 | Physical size | Observed cycle position |
| ---: | --- | --- |
| 0 | 1600x900 | Start |
| 4.2671287 | 1920x1032 | First maximize |
| 8.1178417 | 1600x900 | First restore |
| 10.4816585 | 1920x1032 | Second maximize |
| 13.2015028 | 1600x900 | Second restore |
| 15.2294664 | 1920x1032 | Third maximize |
| 17.5231567 | 1600x900 | Third restore; required count complete |
| 19.8161152 | 1920x1032 | Fourth maximize |
| 22.0845719 | 1600x900 | Fourth restore |
| 23.7898112 | 1600x900 | F10 stop |

All other recorded facts stayed constant: pinned source/profile/dependencies,
scene/resident 1, paused G0/TPS10/1x, focus, Native disabled, visible panels,
no selection and scale 1. PID **19420** and start identity match the retained
post-export memory sample; Studio exited 0 and was closed when inspected.
All **1,772 frame intervals** remain, including maximum **97.7145 ms**, p95
**42.3075 ms** and p99 **44.3233 ms**, as failed-attempt characterization only.
No three-cycle prefix is extracted into a valid trial or client comparison.

The old action described one cycle and then said **"Repeat three times."**
That card wording is ambiguous about the total count. Coding owns this
instruction defect; the raw record establishes four geometry cycles without
identifying the exact keystrokes or proving why the fourth cycle occurred.
The named physical maximize/restore path changed geometry and returned to the
starting size, but Group 3 has **zero accepted captures / zero valid pairs**.

External **bundle v5 / protocol v4** now says **exactly three cycles TOTAL**,
counts restores **1, 2, 3**, and explicitly stops resizing after the third
restore. If needed, wait idle at the original size until at least 15 seconds
since F9, then F10 once. The README enumerates the three cycles. Only README
and the resize action wording/version metadata change. All other protocol
fields compare equal after normalizing that wording. Studio, launcher, analyzer,
measurement quantities, required semantic sequence and acceptance rules remain
unchanged. The full v4 bundle is archived under `versions/m16-b1-ovl-v4`.

All **372 indexed entries across twelve runs** verify unchanged. All **26 saved
reports** replay identically under the clarified protocol: 22 valid and four
rejected, with their original group qualifications preserved. All 14 frozen
files plus pinned source/Python files verify. This is instruction/replay
verification, not a completed physical resize group or an upgraded prior result.
Group 2 remains complete; retry Group 3 with the same launcher.

| Artifact | SHA-256 |
| --- | --- |
| Rejected resize raw | `b949121299595e33c61b667c180858da527ff356fe309c1513a67d96cc991ae0` |
| Original report | `5a980560dce82e84ca2f6b9447a95d3714299eb60dffde47091cd0d3072c94ef` |
| Run raw index | `76c964fb6fce821143e7b7d83795990935a2457899dfc5946691a9e67323f372` |
| Detailed audit | `26c1784a8145980d4aa6eb810f982523d39f5bd60edf309f58ae56a437dffc13` |
| Owner screenshot | `73b6db406fd2a09508ac3c75f3dabade5dfa2637fdf2dfa22c5fb07237d5a1c6` |
| v5 bundle manifest | `017cd23acda60eed8e90176fcb7f020073a6daaf61a4c4cc374b5ece46505486` |
| Clarified v4 protocol | `e9fe47fdc20804cdff2028241e98f085164a02c64cfcc7f8890ad940f251b81d` |
| v5 verification/replay results | `7e86d781f32bdd8113b4986b863b57dab1a947a816b2cd9fb6636ccf2ad69d65` |

Audit, screenshot, freeze/replay scripts and checks are in the common gitdir's
`0088-m16-b1-resize-four-cycles` directory. Original raw/report/config/memory
files remain in the identified OVL run. Input latency remains unavailable;
no budget pass, migration decision, arbitrary resize or multi-monitor claim
is made from this failed capture.

## Resize collection: six captures pass frozen checks; pacing qualification pending

Owner reported OVL success with Egui and Native screenshots for
`b1-20260913-224522-352c334b`. All six original reports replay identically and
pass the frozen capture checks. Each records exactly three cycles:
`1600x900 -> 1920x1032 -> 1600x900`, repeated three times, with the same
starting, maximized and final physical sizes across both clients and all pairs.
All share scene/resident 1, paused G0/TPS10/1x, pinned source/profile/dependencies,
focus, visible panels, no selection and scale 1. Only the expected Native flag
differs in initial facts; during capture only geometry changes. PID **8560**
and start identity match every post-export memory sample. Studio exited 0,
with no M16 refusal/export warning. The planned single-load setup is satisfied.

**Five captures are below the 15-second target**, though all exceed the frozen
10-second usable minimum. Further, **28 of 36 observed geometry dwell intervals
are below two seconds**, ranging overall from **1.0142679 to 3.3485731 seconds**.
These six dwell intervals per capture run from first maximize through the
successive geometry changes and final F10 stop. They measure observed size
residency, not exact keystroke timing. They do not establish the instructed
two-second waits. The frozen analyzer checks count/geometry/minimum duration,
not those individual dwell times; its acceptance is unchanged.

This is complete collection of three descriptive pairs under the **actual
shorter, variably paced condition**, not an unqualified pass of the planned
15-second/two-second-wait procedure. **Orchestration disposition is requested**:
qualify these results at their actual pacing, or require additional target-paced
resize evidence. No threshold is relaxed, no duration is padded, and no prefix
or tail is removed. The earlier four-cycle attempt remains rejected.

All **6,060 frame intervals** remain. Statistics below are milliseconds, with
all resize tails included. Every capture exceeds the p95 16.7 ms frame budget.

| Repetition/client | Duration, seconds | Frames | p50 | p95 | p99 | Maximum |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 Egui | 18.0539349 | 1,343 | 3.6724 | 41.7395 | 45.1395 | 110.9532 |
| 1 Native | 13.9745124 | 1,038 | 3.7580 | 41.5607 | 45.8799 | 91.7699 |
| 2 Egui | 11.8803585 | 878 | 3.6034 | 41.7075 | 49.7139 | 91.4260 |
| 2 Native | 12.0982545 | 895 | 3.9303 | 41.8172 | 52.0408 | 89.0274 |
| 3 Egui | 10.9782936 | 812 | 3.3418 | 41.5791 | 50.5329 | 91.7625 |
| 3 Native | 14.7168723 | 1,094 | 3.4374 | 41.9724 | 46.4529 | 89.4676 |

| Repetition | Native minus Egui frame p95, ms | Private bytes difference | Working-set bytes difference |
| --- | ---: | ---: | ---: |
| 1 | -0.1788 | +21,512,192 | +12,550,144 |
| 2 | +0.1097 | -6,729,728 | -864,256 |
| 3 | +0.3933 | -258,772,992 | +1,454,080 |

All absolute memory values and post-export lags are retained. In pair 3,
Egui private bytes were 1,313,988,608 and Native private bytes 1,055,215,616;
the large negative difference is kept as observed, without attributing it to
client savings. These are one process's totals with allocation/export history,
not isolated client or GPU memory. Native-on continues to coexist with Egui.

The three Native-on captures contain **2,720 exact same-frame/same-publication
shared projection CPU pairs**. Signed residuals are Native minus Egui; all
negative values and positive tails remain in the original reports.

| Native capture | Direct pairs | Minimum residual, ns | p50, ns | p95, ns | p99, ns | Maximum, ns |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Repetition 1 | 938 | -2,100 | +100 | +600 | +1,000 | +3,700 |
| Repetition 2 | 809 | -4,300 | +100 | +500 | +900 | +12,800 |
| Repetition 3 | 973 | -2,200 | +100 | +600 | +1,300 | +79,900 |

These host wall timings compare the same projection computation with Runtime
change frames excluded by the frozen analyzer. Named adapter scopes remain
separate and nested scopes are never summed. Cross-capture statistics are
descriptive: different resize timing and durations prevent causal event-cost
alignment. The ordinal file preserves **2,728 descriptive frame pairs and 604
unmatched tail intervals**, with no causal frame or input-latency association.
There are **zero fresh publication delivery samples**: all consumed publications
are G0 and predate capture. Pre-capture ages remain separate from freshness.

All **429 indexed entries across thirteen runs** verify unchanged, and all six
new reports reproduce exactly. All 14 frozen bundle files plus source/Python
pins verify. Bundle v5 / protocol v4, analyzer, launcher and Studio are unchanged.

| Artifact | SHA-256 |
| --- | --- |
| Run raw index | `e6ab2bf90e430af2d805c00e944027f3b2ead5c492d4f6925b1d4bc7433a29ba` |
| Resize aggregate/audit | `a6d02a956228467785568ae9a3f84697e9681c126974596ba6bf271ea0efd64d` |
| Chronological residuals and unmatched tails | `8bbc09194a4fb55f8851f75f77ecc3dd3d163d1dd27a8874d66cdf831eb957bc` |
| Read-only aggregate script | `eac7af161ec3da28ff75f24923c6f955242e6a9d3d3f8a52c54b9ba22949abf4` |
| Owner Egui screenshot | `2b8435ed9da5a59d5265e5cdf1de5078848f7a190f8462744c9670598deb2f80` |
| Owner Native screenshot | `fa9a6846ec2a7eea582a00bcbd25dd8d2a9da6f313caa18a69934a564c32c02e` |

Aggregate, reproducer, residuals and copied screenshots are in the common
gitdir's `0088-m16-b1-resize-results` directory. Original raw/config/report/memory
files remain in the identified OVL run. Screenshots show both client panes in
their final visible state; they have no run/time identifier, and screenshot
dimensions are not substituted for recorded physical geometry.

Next Owner collection is **Group 4 — Seven-system selection and native list**,
using the unchanged launcher, one source load for all six captures and the
card's Overhead view. Group 1 and Group 3 qualifications, Groups 4-5, final
portable packaging/comparison/decision and final hosted CI/Clearance remain
pending. Required input latency remains INSTRUMENT-INVALID / UNAVAILABLE,
p95 <=100 ms UNEVALUATED; arbitrary resize and multi-monitor DPI remain
unqualified. This checkpoint makes no migration recommendation.

## First list pair: native row order differs from the test's assumed ID order

Owner supplied the Group 4 console screenshot for
`b1-20260913-230204-f9063a95`. Original Egui repetition 1 passes:
**20.2700475 seconds / 1,517 frames**, selection `[null,1,2,3,4,5,6,7]`.
Original Native repetition 1 fails only the ascending-ID sequence check:
**12.8155533 seconds / 960 frames**, selection `[null,1,2,3,4,5,7,6]`.
Native exceeds the 10-second usable minimum but is below the 15-second target.
Its observed selections after F9 occur at +1.5996587 (1), +3.3063770 (2),
+4.2817758 (3), +5.2269742 (4), +6.1333353 (5), +7.1481692 (7) and
+8.1625973 (6) seconds. B0a does not record the originating physical keys.

Both captures have the same pinned source/profile/dependencies, scene/resident
1, paused G0/TPS10/1x, focus, visible panels, no initial selection and physical
1600x900 at scale 1. Only the expected Native flag differs. PID **29004** and
start identity match both memory samples. Studio exited 0 and was closed when
inspected. Egui p95 **39.1802 ms** and Native p95 **41.6306 ms** remain separate
capture characterizations; the different selection sequences prohibit a paired
client result. Original status remains one valid unpaired Egui capture, one
rejected Native capture and zero valid list pairs.

Source archaeology at the unchanged runtime revision explains the order:

- `clause_scenario_ingest.rs` loads the StructuralRebindReady scenario through
  the existing ClauseScript rebind path and `StudioSession::from_loaded_scenario`.
- `clause_scenario_projection.rs:227` sorts placements by **(row, col, system_id)**.
- `hydration.rs:448` and `view_model.rs:157` preserve that placement order in the
  displayed star vector. Native labels and activation index that vector, and
  `rehearsal_studio_native_ui.rs:331` advances its index for Down.
- In the pinned base, G/7 is at **col 5, row 0** and F/6 at **col 4, row 1**.
  Therefore the native row order is **A,B,C,D,E,G,F**, IDs **1,2,3,4,5,7,6**.

This source-derived order exactly matches the recorded Native sequence. The
card/analyzer incorrectly assumed ascending IDs were the Native row order.
This is not evidence of a Down-key defect. Owner clarification about exact
physical inputs was requested and remains pending at this checkpoint; source
inspection is not a new runtime trace or independent physical-key proof.
All six inspected source files are hashed in the audit and unchanged from
`a8c11731`. No ClauseThing, scenario authority, Mapeditor or selection code is edited.

**Bundle v6 / protocol v5** freezes a prospective replacement Group 4 comparison:
both clients must select `[null,1,2,3,4,5,7,6]` on the same seven-system workload.
Egui picks **A,B,C,D,E,G,F** on the map; Native clicks A then makes six separate
Down taps, releasing between taps. Both end at **F/6**. Keep about two seconds
between selections and wait idle if needed until 15 seconds before F10.
No corrective selections or extra visits are part of the sequence.

This changes Group 4's procedural order and its exact-sequence check **for fresh
captures on both clients**. It preserves the workload, seven selections per
capture, command/observation ownership, measurement definitions, thresholds,
other groups and all runtime/launcher bytes. The old pair is not reinterpreted
or stitched into the new comparison. The new analyzer requires the new list
protocol revision and matching operation metadata; old list captures are
refused by it and must be analyzed using their archived original analyzer and
protocol. Full bundle v5 is archived under `versions/m16-b1-ovl-v5`.

All **458 indexed entries across fourteen runs** verify unchanged. All **34
original reports** replay identically using their original analyzers/protocols
(29 valid, five rejected, with existing group qualifications retained). The
**32 non-list reports** also remain identical under the new analyzer. Five
focused software checks cover both original list metadata refusals, a transformed
in-memory future-metadata fixture with the exact new sequence, and refusals for
the former order and a skipped system. The transformed fixture is explicitly
software verification, not a fresh physical capture or a retroactive pass.
Actual analyzer CLI refusal of the old Native raw and actual PowerShell launcher
`-ValidateOnly` both pass; the launcher validates 14 frozen files.

| Artifact | SHA-256 |
| --- | --- |
| Original valid Egui raw | `a38051692ec237e130ab4738ab0d6261cf0e754918d197dbaec2cc1c98243888` |
| Original rejected Native raw | `271bd9597d1955eb4e9cfa39b81fd7d18105bf6afd2e5ac8c650ba88e55a8f67` |
| Detailed original-result/order audit | `3c120f0ab5cde2a3ef16e210edab01a349d9ae2c88845ff356892cabc7f233e9` |
| Owner screenshot | `ae6b16d93f995935bd902dd9dba88c1090dd673fd6a2ef875489de7f66d5c54d` |
| v6 bundle manifest | `1feee6ba0357ec8fd36873a53c737639db43b1ae1eea9b4cdd21af86db3117db` |
| New v5 protocol | `0e00ec2031a3e8878fc2008de250f37138bdb6b94930c70b31bb2ef234c8b375` |
| New external analyzer | `e829eb2d3ac954fa24d9b7789f4ccd62a60a287fe79b29af03674ebabf03402a` |
| v6 verification/replay results | `c0ceed29b4b175dbf5811fc70f6f17ef9639c4b000bca85f584b0f2fa0d5a72e` |

Audit, source bindings, screenshot, freeze/replay script, software checks and
CLI refusal report are in the common gitdir's `0088-m16-b1-list-order` directory.
All original raw/config/report/memory files remain in the identified OVL run.
Restart **Group 4** through the same launcher for fresh paired captures using
**G before F on both clients**. Other collected groups keep their existing
results and qualifications. Required input latency remains unavailable;
no budget pass, long-list scaling claim or migration recommendation is made.

## List retry: correct Native order, but F selected at capture start

Two subsequent v6-bundle runs are preserved. In
`b1-20260913-232035-e26f7c3d`, Egui recorded the former ascending order
`[null,1,2,3,4,5,6,7]` and correctly failed the new row-order check:
39.9417721 seconds, 2,993 frames, p95 39.3099 ms, PID 37208.

In the Owner's latest screenshot run, `b1-20260913-232212-fb723200`, Egui is
valid: **27.1788779 seconds / 2,035 frames**, `[null,1,2,3,4,5,7,6]`.
Native records the correct seven selections after F9, but **starts with F/6
already selected**: `[6,1,2,3,4,5,7,6]`. Its sole rejection is the exact-sequence
check's required clear initial state. Duration is **33.3471206 seconds / 2,497
frames**. The initial 6 is retained, not replaced with null or trimmed away.

| Native seconds after F9 | Selected system |
| ---: | --- |
| 0 | 6/F — initial selection must instead be clear |
| 2.3480025 | 1/A |
| 6.1363679 | 2/B |
| 7.0964539 | 3/C |
| 7.9507668 | 4/D |
| 8.8297215 | 5/E |
| 9.7098574 | 7/G |
| 10.8307679 | 6/F |

Latest Egui and Native retain the same pinned source/profile/dependencies,
scene/resident 1, paused G0/TPS10/1x, focus, visible panels and 1600x900/scale1.
Their initial facts differ only in Native enablement and the leftover selection.
PID **38184** and start identity match both memory samples. Both new processes
exited 0 and were closed when inspected. Egui p95 **39.4016 ms** / Native p95
**39.1732 ms** remain separate characterizations; there is no valid list pair
or 16.7 ms budget pass. All three captures exceed the 15-second target.

The raw files do not identify which physical setup action left F selected.
Source confirms the existing **Clear selection (Esc)** button in the right-hand
Selected system section directly invokes `state.selection.clear()`
(`app/ui.rs:3091-3092`). Native Escape can release field focus before map Escape
clears selection; no key history is available to attribute this attempt to that
path. A visible setup check removes the ambiguity: **before every capture**, click
the existing clear-selection button if shown, verify the Selected system section
disappears and Native's readout says **selected: None**, then wait the setup warmup
and F9. Click A only after F9. Keep the source loaded across the group.

The required clear initial state and row-order sequence are unchanged. No
bundle/protocol/analyzer/launcher/runtime edit is made. All **509 indexed entries
across sixteen runs** verify unchanged, all three new reports replay identically,
and all 14 frozen bundle files plus source pins verify. Valid Egui remains
unpaired; failed runs are not stitched into a new process's paired comparison.

| Artifact | SHA-256 |
| --- | --- |
| Earlier wrong-order Egui raw | `856b1097a0ee130986cb56693e66dd9c66b8bdb659f13a3e64d0431c048d80bc` |
| Latest valid Egui raw | `4795d4732bc668cd8e5b87c83080f0f5cf575368b4de01cfd4ab27c7ae1a6289` |
| Latest rejected Native raw | `4204d1298721d85b93768bb8778a23607b0e528b08f35bc4985def07bf561831` |
| Detailed audit | `99861f06113201b0f33c682500b7e56fcce75ec5cc03bff221cbaa86a4dd6e6e` |
| Audit script | `ec86de3357841aef58ab673dd9f1c049f13c421e7c3becff8e489c5a845e353f` |
| Owner screenshot | `f2113e7eba3d425c95df8ac39252eaf5a94c5730edfbca71d06a83312fb4db80` |

The common gitdir's `0088-m16-b1-list-initial-selection` directory contains the
audit, reproducer and screenshot. Originals remain in the two identified OVL
runs. Restart Group 4 with the same launcher; verify clear selection before
every F9. The matched sequence stays **A,B,C,D,E,G,F** on both clients.

## Required unavailable input-latency evidence

DA 5651557467 defines app input ingestion QPC to the first matching present
reported DISPLAYED by Windows. The latency quantity remains required. The
physical run `ovl-20260913-163144-1436aab9` had six inputs per client and the
unchanged 0,4,0,4,0,4 sequence. Containment found zero joins among 2,481 Egui
and 1,427 Native app spans (Board 5654603968).

R1, authorized by Board 5654621290 and returned in 5654672574, tested strictly
`S_i.end_qpc < QPCTime < S_(i+1).begin_qpc` on unchanged raw evidence:

| R1 diagnostic | Egui | Native |
| --- | ---: | ---: |
| Adjacent span pairs | 2,480 | 1,426 |
| Empty / unique / ambiguous windows | 1,240 / 620 / 620 | 713 / 143 / 570 |
| Reused rows | 0 | 0 |
| Unmatched in-domain rows | 1,860 | 1,283 |
| Credited response endpoints | 0/6 | 0/6 |

PID, chain and runtime were consistent. Egui's six local response candidates
did not establish the full bijection. Five Native response windows were empty;
the other had two candidates. Final spans were uncredited. K falsifier NOT RUN
because the endpoint prerequisite failed. No additional join heuristic or
rerun of the invalid instrument is part of B1.

| Preserved artifact | SHA-256 |
| --- | --- |
| PresentMon 2.5.1 x64 tool | `9bec3083069f58f911e6a512f4806db51a27bd096103087bc1d05ef54c80a191` |
| B0b process-filtered CSV | `9a4cc51f441f483951f8786cd4247eb8c6b54de4faf72ddd268fc1d801c54e51` |
| B0b Egui raw JSON | `1a89f58a9e88c447e5ef51c00c65325e0c686531ef360c5fd9c86b84bd54e058` |
| B0b Native raw JSON | `5e6db8cac454944d9f9d6dc0ce2cae6b081fe61556e6f4c31c9d38882dc03dbb` |
| R1 detailed diagnostic | `cf07561c1387d730bb107cf64fdfda05e1cefed61ed7dea62be6d23a2c6a412a` |
| R1 diagnostic script | `a1741218e9e73feafe90d4fb619c95dde5981096689eb75a5a5164178f46fedc` |

Original evidence remains in `SimThing-0088-OVL-B0b/runs/ovl-20260913-163144-1436aab9`
and the common gitdir's `0088-m16-b0b-r1-causal-successor` packet. All original
31 raw-index entries were verified in R1. PresentMon source is the Intel
GameTechDev/PresentMon v2.5.1 release. B1 does not extend that failed run's
result to a newly tested window geometry; latency stays unavailable/unevaluated.

## Behavior, decision and remaining work

Leaf A's bounded numeric editing and shared command/observation ownership are
carried as established semantic/desktop evidence. Native desktop Down,
seven-row traversal and the named maximize/restore cycle are explicit Owner
checks here. General text accessibility, arbitrary dragging/resizing, large
lists, changed DPI and multiple monitors remain unqualified unless separately
demonstrated. Synthetic dispatcher tests are not physical qualification.

No migration decision is issued in this pending-data draft. Required latency
is unavailable, so a positive migration recommendation cannot be supported
by treating it as passed or optional. The final single decision will bind the
collected behavior/cost results and that uncertainty. No dependency, Bevy
upgrade, present-path change or UI migration is implemented.

The external Owner bundle, exact executable hash, compiler/build log, machine
qualification, case configs, raw JSON, memory snapshots and per-capture reports
will bind into the completed packet. Current preparation checks exercise the
external analyzer's refusal semantics and process/logging/serialization behavior;
they are not physical M16 results. Local proof, hosted Doctrine, complete INSPECT
and fresh Clearance on the final evidence head remain required before handback.
All collection/reporting scaffolding is rung-local and retires with the 1.3
experiment. No protected test, lifecycle row, gate, census or class is edited.
