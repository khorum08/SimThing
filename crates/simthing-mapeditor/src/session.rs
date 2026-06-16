//! Studio session — typed generated data trusted by the editor (not Bevy-owned truth).

use simthing_mapgenerator::GenerationReport;

use crate::generation::{GenerationProfile, GenerationRunOutput};
use crate::view_model::StudioGalaxyViewModel;

#[derive(Debug, Clone)]
pub struct StudioSession {
    pub profile: GenerationProfile,
    pub output: GenerationRunOutput,
    pub view_model: StudioGalaxyViewModel,
    pub report_path: Option<std::path::PathBuf>,
    pub scenario_path: Option<std::path::PathBuf>,
}

impl StudioSession {
    pub fn from_generation(profile: GenerationProfile, output: GenerationRunOutput) -> Self {
        let view_model = StudioGalaxyViewModel::from_generation(&output.result, &output.report);
        Self {
            profile,
            output,
            view_model,
            report_path: None,
            scenario_path: None,
        }
    }

    pub fn report(&self) -> &GenerationReport {
        &self.output.report
    }

    pub fn galaxy_name(&self) -> &str {
        &self.output.galaxy_display_name
    }
}
