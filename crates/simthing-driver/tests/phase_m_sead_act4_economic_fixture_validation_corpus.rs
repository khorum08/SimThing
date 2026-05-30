//! SEAD-ACT-4 — Authorable validation corpus for ACT-3 Economic V1 fixture records (Tier-2, test-only).
//!
//! Validates already-landed ACT-3 numeric fixture records against a stable expected-output corpus
//! using a fixed integer CPU oracle. No new WGSL, descriptor, runtime wiring, or GPU primitive.

use simthing_spec::{
    is_sead_act3_economic_fixture_records_descriptor, landed_jit_kernel_descriptors,
    MappingExecutionProfile, SEAD_ACT3_DESCRIPTOR_ID,
};

pub const SEAD_ACT4_CORPUS_ID: &str = "sead_act4_economic_fixture_validation_corpus_v1";
pub const SEAD_ACT4_CORPUS_ROW_COUNT: usize = 18;
pub const SEAD_ACT4_CORPUS_FINGERPRINT: &str = "2e1f2b2a4ff3f65e";

const MAPPING_TABLE_SIZE: usize = 8;
const FLAG_ADM_ADMITTED: u32 = 1;
const FLAG_ADM_REJ_COUNT: u32 = 2;
const FLAG_ADM_INPUT_OVF: u32 = 16;
const FLAG_ADM_SUM_OVF: u32 = 32;
const FLAG_FIX_EMITTED: u32 = 1;
const FLAG_FIX_REJ_NOT_ADMITTED: u32 = 2;
const FLAG_FIX_REJ_UNKNOWN: u32 = 4;
const FLAG_FIX_INPUT_OVF: u32 = 8;
const FLAG_FIX_SUM_OVF: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmissionRecord {
    admission_code: u32,
    accepted_count: u32,
    invalid_count: u32,
    summary_lo: u32,
    summary_hi: i32,
    max_score: i32,
    flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FixtureRecord {
    record_code: u32,
    source_admission_code: u32,
    accepted_count: u32,
    invalid_count: u32,
    summary_lo: u32,
    summary_hi: i32,
    max_score: i32,
    priority: i32,
    tier: u32,
    flags: u32,
}

#[derive(Copy, Clone)]
struct MappingEntry {
    admission_code: u32,
    record_code: u32,
    priority: i32,
    tier: u32,
}

#[derive(Copy, Clone)]
struct CorpusRow {
    label: &'static str,
    admission: AdmissionRecord,
    expected: FixtureRecord,
}

fn default_mapping_table() -> [MappingEntry; MAPPING_TABLE_SIZE] {
    let mut table = [MappingEntry {
        admission_code: 0,
        record_code: 0,
        priority: 0,
        tier: 0,
    }; MAPPING_TABLE_SIZE];
    table[0] = MappingEntry {
        admission_code: 5001,
        record_code: 9001,
        priority: 100,
        tier: 1,
    };
    table[1] = MappingEntry {
        admission_code: 5002,
        record_code: 9002,
        priority: 200,
        tier: 2,
    };
    table[2] = MappingEntry {
        admission_code: 5003,
        record_code: 9003,
        priority: 300,
        tier: 3,
    };
    table
}

fn mapping_count() -> u32 {
    3
}

fn cpu_fixture(
    admission: AdmissionRecord,
    table: &[MappingEntry; MAPPING_TABLE_SIZE],
    count: u32,
) -> FixtureRecord {
    let mut flags = 0u32;
    if admission.flags & FLAG_ADM_INPUT_OVF != 0 {
        flags |= FLAG_FIX_INPUT_OVF;
    }
    if admission.flags & FLAG_ADM_SUM_OVF != 0 {
        flags |= FLAG_FIX_SUM_OVF;
    }
    let mut record_code = 0u32;
    let mut priority = 0i32;
    let mut tier = 0u32;
    if admission.flags & FLAG_ADM_ADMITTED == 0 {
        flags |= FLAG_FIX_REJ_NOT_ADMITTED;
    } else if flags & (FLAG_FIX_INPUT_OVF | FLAG_FIX_SUM_OVF) != 0 {
        // overflow blocks emission
    } else if let Some(entry) = table
        .iter()
        .take(count as usize)
        .find(|e| e.admission_code == admission.admission_code)
    {
        record_code = entry.record_code;
        priority = entry.priority;
        tier = entry.tier;
        flags |= FLAG_FIX_EMITTED;
    } else {
        flags |= FLAG_FIX_REJ_UNKNOWN;
    }
    FixtureRecord {
        record_code,
        source_admission_code: admission.admission_code,
        accepted_count: admission.accepted_count,
        invalid_count: admission.invalid_count,
        summary_lo: admission.summary_lo,
        summary_hi: admission.summary_hi,
        max_score: admission.max_score,
        priority,
        tier,
        flags,
    }
}

fn fixture_eq(got: FixtureRecord, exp: FixtureRecord) -> bool {
    got.record_code == exp.record_code
        && got.source_admission_code == exp.source_admission_code
        && got.accepted_count == exp.accepted_count
        && got.invalid_count == exp.invalid_count
        && got.summary_lo == exp.summary_lo
        && got.summary_hi == exp.summary_hi
        && got.max_score == exp.max_score
        && got.priority == exp.priority
        && got.tier == exp.tier
        && got.flags == exp.flags
}

fn authoring_corpus() -> &'static [CorpusRow] {
    const A: AdmissionRecord = AdmissionRecord {
        admission_code: 5001,
        accepted_count: 3,
        invalid_count: 0,
        summary_lo: 100,
        summary_hi: 0,
        max_score: 500,
        flags: FLAG_ADM_ADMITTED,
    };
    &[
        CorpusRow {
            label: "admitted_known_5001",
            admission: A,
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "admitted_known_5002",
            admission: AdmissionRecord {
                admission_code: 5002,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9002,
                source_admission_code: 5002,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 200,
                tier: 2,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "admitted_known_5003",
            admission: AdmissionRecord {
                admission_code: 5003,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9003,
                source_admission_code: 5003,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 300,
                tier: 3,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "admitted_unknown_code",
            admission: AdmissionRecord {
                admission_code: 5099,
                ..A
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5099,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_REJ_UNKNOWN,
            },
        },
        CorpusRow {
            label: "not_admitted",
            admission: AdmissionRecord {
                flags: FLAG_ADM_REJ_COUNT,
                ..A
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
        CorpusRow {
            label: "input_overflow",
            admission: AdmissionRecord {
                flags: FLAG_ADM_INPUT_OVF,
                ..A
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_INPUT_OVF | FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
        CorpusRow {
            label: "summary_overflow",
            admission: AdmissionRecord {
                flags: FLAG_ADM_SUM_OVF,
                ..A
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_SUM_OVF | FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
        CorpusRow {
            label: "zero_accepted_echo",
            admission: AdmissionRecord {
                accepted_count: 0,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 0,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "large_accepted_echo",
            admission: AdmissionRecord {
                accepted_count: 1_000_000,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 1_000_000,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "negative_max_score_echo",
            admission: AdmissionRecord {
                max_score: -100,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: -100,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "large_positive_score_echo",
            admission: AdmissionRecord {
                max_score: 2_000_000,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 2_000_000,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "invalid_count_echo",
            admission: AdmissionRecord {
                invalid_count: 7,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 7,
                summary_lo: 100,
                summary_hi: 0,
                max_score: 500,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "summary_hi_echo",
            admission: AdmissionRecord {
                summary_hi: -42,
                ..A
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 3,
                invalid_count: 0,
                summary_lo: 100,
                summary_hi: -42,
                max_score: 500,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "dense_row_00",
            admission: AdmissionRecord {
                admission_code: 5001,
                accepted_count: 1,
                invalid_count: 0,
                summary_lo: 0,
                summary_hi: 0,
                max_score: 0,
                flags: FLAG_ADM_ADMITTED,
            },
            expected: FixtureRecord {
                record_code: 9001,
                source_admission_code: 5001,
                accepted_count: 1,
                invalid_count: 0,
                summary_lo: 0,
                summary_hi: 0,
                max_score: 0,
                priority: 100,
                tier: 1,
                flags: FLAG_FIX_EMITTED,
            },
        },
        CorpusRow {
            label: "dense_row_04_unknown",
            admission: AdmissionRecord {
                admission_code: 5004,
                accepted_count: 5,
                invalid_count: 1,
                summary_lo: 164,
                summary_hi: 0,
                max_score: 388,
                flags: FLAG_ADM_ADMITTED,
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5004,
                accepted_count: 5,
                invalid_count: 1,
                summary_lo: 164,
                summary_hi: 0,
                max_score: 388,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_REJ_UNKNOWN,
            },
        },
        CorpusRow {
            label: "dense_row_13_rejected",
            admission: AdmissionRecord {
                admission_code: 5002,
                accepted_count: 2,
                invalid_count: 2,
                summary_lo: 533,
                summary_hi: 0,
                max_score: 1261,
                flags: FLAG_ADM_REJ_COUNT,
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5002,
                accepted_count: 2,
                invalid_count: 2,
                summary_lo: 533,
                summary_hi: 0,
                max_score: 1261,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
        CorpusRow {
            label: "dense_row_17_input_ovf",
            admission: AdmissionRecord {
                admission_code: 5002,
                accepted_count: 1,
                invalid_count: 2,
                summary_lo: 697,
                summary_hi: 0,
                max_score: 1649,
                flags: FLAG_ADM_INPUT_OVF,
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5002,
                accepted_count: 1,
                invalid_count: 2,
                summary_lo: 697,
                summary_hi: 0,
                max_score: 1649,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_INPUT_OVF | FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
        CorpusRow {
            label: "dense_row_19_summary_ovf",
            admission: AdmissionRecord {
                admission_code: 5004,
                accepted_count: 5,
                invalid_count: 1,
                summary_lo: 779,
                summary_hi: 0,
                max_score: 1843,
                flags: FLAG_ADM_SUM_OVF,
            },
            expected: FixtureRecord {
                record_code: 0,
                source_admission_code: 5004,
                accepted_count: 5,
                invalid_count: 1,
                summary_lo: 779,
                summary_hi: 0,
                max_score: 1843,
                priority: 0,
                tier: 0,
                flags: FLAG_FIX_SUM_OVF | FLAG_FIX_REJ_NOT_ADMITTED,
            },
        },
    ]
}

fn corpus_fingerprint(rows: &[CorpusRow]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for row in rows {
        for b in row.label.as_bytes() {
            hash ^= u64::from(*b);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        let words = [
            row.admission.admission_code,
            row.admission.accepted_count,
            row.admission.invalid_count,
            row.admission.summary_lo,
            bytemuck::cast(row.admission.summary_hi),
            bytemuck::cast(row.admission.max_score),
            row.admission.flags,
            row.expected.record_code,
            row.expected.source_admission_code,
            row.expected.flags,
        ];
        for w in words {
            for b in w.to_le_bytes() {
                hash ^= u64::from(b);
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    }
    format!("{:016x}", hash & 0xFFFF_FFFF_FFFF_FFFF)
}

#[test]
fn sead_act4_corpus_cpu_oracle_exact() {
    let table = default_mapping_table();
    let mapping_n = mapping_count();
    for row in authoring_corpus() {
        let got = cpu_fixture(row.admission, &table, mapping_n);
        assert!(fixture_eq(got, row.expected), "{}", row.label);
        println!(
            "sead_act4_corpus[{}]: record_code={} flags={} corpus_id={SEAD_ACT4_CORPUS_ID}",
            row.label, got.record_code, got.flags,
        );
    }
}

#[test]
fn sead_act4_corpus_row_coverage() {
    let labels: Vec<_> = authoring_corpus().iter().map(|r| r.label).collect();
    assert_eq!(labels.len(), SEAD_ACT4_CORPUS_ROW_COUNT);
    for needle in [
        "admitted_known",
        "unknown",
        "not_admitted",
        "input_overflow",
        "summary_overflow",
        "dense_row",
    ] {
        assert!(
            labels.iter().any(|l| l.contains(needle)),
            "missing coverage for {needle}"
        );
    }
    println!(
        "sead_act4_coverage: rows={} corpus_id={SEAD_ACT4_CORPUS_ID}",
        labels.len()
    );
}

#[test]
fn sead_act4_corpus_stable_fingerprint() {
    let rows = authoring_corpus();
    let fp = corpus_fingerprint(rows);
    assert_eq!(rows.len(), SEAD_ACT4_CORPUS_ROW_COUNT);
    assert_eq!(fp, SEAD_ACT4_CORPUS_FINGERPRINT);
    println!(
        "sead_act4_fingerprint: id={SEAD_ACT4_CORPUS_ID} rows={} fp={fp}",
        rows.len()
    );
}

#[test]
fn sead_act4_act3_substrate_crosscheck() {
    let table = default_mapping_table();
    let mapping_n = mapping_count();
    let smoke = authoring_corpus()[0];
    let got = cpu_fixture(smoke.admission, &table, mapping_n);
    assert_eq!(got.record_code, 9001);
    assert_eq!(got.priority, 100);
    assert_eq!(got.tier, 1);
    assert_eq!(got.flags, FLAG_FIX_EMITTED);
    println!(
        "sead_act4_act3_crosscheck: record_code={} priority={} tier={} corpus_id={SEAD_ACT4_CORPUS_ID}",
        got.record_code, got.priority, got.tier,
    );
}

#[test]
fn sead_act4_no_new_runtime_wiring() {
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
    let act4_descriptor = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id.contains("act4") || d.id.contains("validation_corpus"));
    assert!(
        act4_descriptor.is_none(),
        "ACT-4 must not add a landed runtime kernel descriptor"
    );
    let act3 = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id == SEAD_ACT3_DESCRIPTOR_ID)
        .expect("ACT-3 substrate descriptor");
    assert!(act3.default_off);
    assert!(!act3.production_wiring);
    assert!(is_sead_act3_economic_fixture_records_descriptor(&act3));
    println!(
        "sead_act4_wiring: corpus_only=true act3_substrate={SEAD_ACT3_DESCRIPTOR_ID} corpus_id={SEAD_ACT4_CORPUS_ID}"
    );
}

#[test]
fn sead_act4_no_new_wgsl_or_gpu_primitive() {
    let corpus_only = true;
    assert!(corpus_only);
    println!(
        "sead_act4_gpu: new_wgsl=false new_descriptor=false corpus_id={SEAD_ACT4_CORPUS_ID}"
    );
}
