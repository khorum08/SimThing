//! Workshop-homed thin wrapper around production StructuralRebindReady rebind.

use simthing_clausething::{
    rebind_authority_tree_candidate as generic_rebind,
    rebind_pack_to_structural_rebind_ready as generic_rebind_pack, HydratedScenarioPack,
};
use simthing_spec::SimThingScenarioSpec;
use thiserror::Error;

use crate::tp_studio_clause_ingest::TpStudioClauseIngestError;

pub const PROJECTION_MODE_AUTHORITY_TREE_CANDIDATE: &str = "AuthorityTreeCandidate";
pub const PROJECTION_MODE_STRUCTURAL_REBIND_READY: &str = "StructuralRebindReady";

#[derive(Debug, Error)]
pub enum TpStudioSteadRebindError {
    #[error("TP STEAD rebind error: {0}")]
    Message(String),
    #[error("TP STEAD rebind ingest error: {0}")]
    Ingest(#[from] TpStudioClauseIngestError),
}

#[derive(Debug, Clone)]
pub struct TpStudioSteadRebindReport {
    pub projection_mode: &'static str,
    pub map_container_id: String,
    pub placement_count: usize,
    pub link_count: usize,
    pub links_residue: Option<String>,
    pub stead_validation: String,
}

#[derive(Debug, Clone)]
pub struct TpStudioSteadRebindResult {
    pub scenario: SimThingScenarioSpec,
    pub report: TpStudioSteadRebindReport,
}

pub fn rebind_pack_to_structural_rebind_ready(
    pack: &HydratedScenarioPack,
) -> Result<TpStudioSteadRebindResult, TpStudioSteadRebindError> {
    let (scenario, report) = generic_rebind_pack(pack)
        .map_err(|e| TpStudioSteadRebindError::Message(e.message))?;
    Ok(TpStudioSteadRebindResult {
        scenario,
        report: TpStudioSteadRebindReport {
            projection_mode: report.projection_mode,
            map_container_id: report.map_container_id,
            placement_count: report.placement_count,
            link_count: report.link_count,
            links_residue: report.links_residue,
            stead_validation: report.stead_validation,
        },
    })
}

pub fn rebind_authority_tree_candidate(
    candidate: &SimThingScenarioSpec,
    pack: &HydratedScenarioPack,
) -> Result<TpStudioSteadRebindResult, TpStudioSteadRebindError> {
    let (scenario, report) = generic_rebind(candidate, pack)
        .map_err(|e| TpStudioSteadRebindError::Message(e.message))?;
    Ok(TpStudioSteadRebindResult {
        scenario,
        report: TpStudioSteadRebindReport {
            projection_mode: report.projection_mode,
            map_container_id: report.map_container_id,
            placement_count: report.placement_count,
            link_count: report.link_count,
            links_residue: report.links_residue,
            stead_validation: report.stead_validation,
        },
    })
}
