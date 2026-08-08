//! ACTIONBAND-COMPOSITION-PROBE-0 — born-mortal workshop structural comparison.
//!
//! Classification and candidate A/B determination only. This module does **not**
//! define a production `ActionBand` type, trait, registry, opcode, planner, or
//! shared potential→claim execution helper. Witness runners live in the
//! integration test and stay independent.

/// One stage on the hypothesized composition spine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipelineStage {
    Potential,
    Eml,
    Claim,
    ClearDisburse,
    CostBand,
    Consequence,
}

impl PipelineStage {
    pub const ALL: [PipelineStage; 6] = [
        PipelineStage::Potential,
        PipelineStage::Eml,
        PipelineStage::Claim,
        PipelineStage::ClearDisburse,
        PipelineStage::CostBand,
        PipelineStage::Consequence,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PipelineStage::Potential => "potential",
            PipelineStage::Eml => "EML",
            PipelineStage::Claim => "claim",
            PipelineStage::ClearDisburse => "clear/disburse",
            PipelineStage::CostBand => "CostBand",
            PipelineStage::Consequence => "consequence",
        }
    }
}

/// How a witness's landed symbol relates to the hypothesized spine stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageClass {
    /// Same landed symbol/path participates in this stage for the witness.
    Literal,
    /// A related but distinct symbol is composed beside the stage.
    Analogous,
    /// Stage is absent, optional, or a materially different seam.
    SpecialSeam,
}

impl StageClass {
    pub fn label(self) -> &'static str {
        match self {
            StageClass::Literal => "LITERAL",
            StageClass::Analogous => "ANALOGOUS",
            StageClass::SpecialSeam => "SPECIAL-SEAM",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageCitation {
    pub stage: PipelineStage,
    /// Exact crate::module::symbol (or honest absence note).
    pub symbol_path: &'static str,
    pub class: StageClass,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WitnessStructuralTable {
    pub witness_id: &'static str,
    pub rows: [StageCitation; 6],
}

/// Lawful probe outcomes — never inconclusive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CandidateDisposition {
    /// Literal common path across witnesses with zero domain branch.
    A,
    /// Materially different action semantics / seams — ActionBand not core law.
    B,
}

/// Measured path tables for the four witnesses (exact symbols, no shared helper).
pub fn measured_structural_tables() -> [WitnessStructuralTable; 4] {
    [
        WitnessStructuralTable {
            witness_id: "deficit_resource_satisfaction",
            rows: [
                StageCitation {
                    stage: PipelineStage::Potential,
                    symbol_path: "simthing_core::evaluate::Evaluator::evaluate (attractor Amount)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Eml,
                    symbol_path: "(absent on receive_command_deficits_from_disbursement)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::Claim,
                    symbol_path: "simthing_driver::CommandDeficit → RuntimeOwnerSiloDemandBucket",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::ClearDisburse,
                    symbol_path: "simthing_spec::apply_owner_silo_runtime_disburse_down_cpu + apply_runtime_local_allocations_from_disburse_down",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::CostBand,
                    symbol_path: "simthing_core::cost_band_depth_one (composed beside reception; unit=1 structural inside reception)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Consequence,
                    symbol_path: "simthing_core::deliver_deficit_directive",
                    class: StageClass::Literal,
                },
            ],
        },
        WitnessStructuralTable {
            witness_id: "linkgraph_relational_action",
            rows: [
                StageCitation {
                    stage: PipelineStage::Potential,
                    symbol_path: "simthing_gpu::FieldAdjacency::link_graph + ComparativeEmitterClass value_col",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::Eml,
                    symbol_path: "simthing_driver::admit_comparative_projections / comparative_projection_cpu_oracle",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::Claim,
                    symbol_path: "(no dedicated relational claim type; dominance_col readout only)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::ClearDisburse,
                    symbol_path: "(comparative path does not call owner-silo disburse-down)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::CostBand,
                    symbol_path: "simthing_core::cost_band_depth_one (composed on dominance margin; not intrinsic to comparative admit)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Consequence,
                    symbol_path: "dominance_col / border_col numeric binding (no ActionKind branch)",
                    class: StageClass::Analogous,
                },
            ],
        },
        WitnessStructuralTable {
            witness_id: "derivation_fission",
            rows: [
                StageCitation {
                    stage: PipelineStage::Potential,
                    symbol_path: "activating property Amount plane (ThresholdRegistration slot/col)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Eml,
                    symbol_path: "(fission path is threshold scan, not EML arbitration)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::Claim,
                    symbol_path: "simthing_sim::ThresholdSemantic::FissionTrigger + ThresholdEvent",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::ClearDisburse,
                    symbol_path: "(not intrinsic to resolve_fission_fusion; RF enroll is post-hoc)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::CostBand,
                    symbol_path: "simthing_sim::ThresholdRegistry::push_with_cost_band / resolve_cost_band_draw (parallel table)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Consequence,
                    symbol_path: "simthing_sim::fission::resolve_fission_fusion → boundary child spawn",
                    class: StageClass::Literal,
                },
            ],
        },
        WitnessStructuralTable {
            witness_id: "movement_7_1_readonly",
            rows: [
                StageCitation {
                    stage: PipelineStage::Potential,
                    symbol_path: "sealed StructuralCommitment slot/col/value (upstream field/threshold; consumed in MovementCommitment::admit)",
                    class: StageClass::Analogous,
                },
                StageCitation {
                    stage: PipelineStage::Eml,
                    symbol_path: "(absent inside crates/simthing-sim/src/movement_ingress.rs; triad lives upstream)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::Claim,
                    symbol_path: "simthing_sim::MovementCommitment::admit (destination rebound from sealed locus)",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::ClearDisburse,
                    symbol_path: "(movement does not call owner-silo disburse-down)",
                    class: StageClass::SpecialSeam,
                },
                StageCitation {
                    stage: PipelineStage::CostBand,
                    symbol_path: "simthing_core::cost_band_quantize inside MovementCommitment::admit + validate_movement_cost_band",
                    class: StageClass::Literal,
                },
                StageCitation {
                    stage: PipelineStage::Consequence,
                    symbol_path: "simthing_sim::apply_movement_commitments → BoundaryRequest::Reparent + bind_owner + AttachOverlay",
                    class: StageClass::Literal,
                },
            ],
        },
    ]
}

/// Candidate A requires every stage LITERAL on every witness **and** identical
/// `symbol_path` strings across witnesses for each stage (literal common path)
/// with no special-seam/domain-branch gaps. Otherwise B.
pub fn determine_candidate(tables: &[WitnessStructuralTable]) -> CandidateDisposition {
    if tables.len() < 2 {
        return CandidateDisposition::B;
    }
    for stage in PipelineStage::ALL {
        let mut paths = Vec::with_capacity(tables.len());
        for table in tables {
            let row = table
                .rows
                .iter()
                .find(|r| r.stage == stage)
                .expect("every witness table carries all six stages");
            if row.class != StageClass::Literal {
                return CandidateDisposition::B;
            }
            paths.push(row.symbol_path);
        }
        let first = paths[0];
        if paths.iter().any(|p| *p != first) {
            return CandidateDisposition::B;
        }
    }
    CandidateDisposition::A
}

/// Probe-fixed disposition from the measured tables.
pub fn probe_candidate() -> CandidateDisposition {
    determine_candidate(&measured_structural_tables())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measured_tables_conclude_candidate_b() {
        assert_eq!(probe_candidate(), CandidateDisposition::B);
        let tables = measured_structural_tables();
        assert_eq!(tables.len(), 4);
        for table in &tables {
            assert_eq!(table.rows.len(), 6);
            let special = table
                .rows
                .iter()
                .any(|r| r.class == StageClass::SpecialSeam);
            assert!(
                special,
                "{} must expose at least one SPECIAL-SEAM vs a literal shared spine",
                table.witness_id
            );
        }
    }

    #[test]
    fn candidate_a_requires_identical_literal_paths() {
        let literal = |stage, path| StageCitation {
            stage,
            symbol_path: path,
            class: StageClass::Literal,
        };
        let shared = WitnessStructuralTable {
            witness_id: "w0",
            rows: [
                literal(PipelineStage::Potential, "P"),
                literal(PipelineStage::Eml, "E"),
                literal(PipelineStage::Claim, "C"),
                literal(PipelineStage::ClearDisburse, "D"),
                literal(PipelineStage::CostBand, "B"),
                literal(PipelineStage::Consequence, "X"),
            ],
        };
        let mut other = shared.clone();
        other.witness_id = "w1";
        assert_eq!(
            determine_candidate(&[shared.clone(), other.clone()]),
            CandidateDisposition::A
        );
        other.rows[2].symbol_path = "C_other";
        assert_eq!(
            determine_candidate(&[shared, other]),
            CandidateDisposition::B
        );
    }
}
