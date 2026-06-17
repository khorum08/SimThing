//! Studio session — typed generated data trusted by the editor (not Bevy-owned truth).

use simthing_mapgenerator::GenerationReport;

use crate::generation::{GenerationProfile, GenerationRunOutput};
use crate::hydration::{
    hydrate_generation_into_studio_grid, StudioHydrationBoundary, StudioHydrationError,
};
use crate::view_model::StudioGalaxyViewModel;

#[derive(Debug, Clone)]
pub struct StudioSession {
    pub profile: GenerationProfile,
    pub output: GenerationRunOutput,
    pub hydration: StudioHydrationBoundary,
    pub view_model: StudioGalaxyViewModel,
    pub report_path: Option<std::path::PathBuf>,
    pub scenario_path: Option<std::path::PathBuf>,
}

impl StudioSession {
    pub fn from_generation(
        profile: GenerationProfile,
        output: GenerationRunOutput,
    ) -> Result<Self, StudioHydrationError> {
        let hydration = hydrate_generation_into_studio_grid(&output)?;
        let view_model = StudioGalaxyViewModel::from_hydration(&hydration);
        Ok(Self {
            profile,
            output,
            hydration,
            view_model,
            report_path: None,
            scenario_path: None,
        })
    }

    pub fn report(&self) -> &GenerationReport {
        &self.output.report
    }

    pub fn galaxy_name(&self) -> &str {
        &self.output.galaxy_display_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::run_generation;

    #[test]
    fn studio_session_requires_hydrated_grid() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("hydrate session");

        assert_eq!(
            session.hydration.grid.occupied_cells as usize,
            session.output.result.placement.systems.len()
        );
        assert_eq!(
            session.view_model.stars.len(),
            session.hydration.grid.gridcells.len()
        );
    }

    #[test]
    fn failed_hydration_does_not_adopt_session() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let mut output = run_generation(&profile).expect("generate");
        output.result.placement.systems[1].coord = output.result.placement.systems[0].coord;

        let session = StudioSession::from_generation(profile, output);

        assert!(session.is_err());
    }
}
