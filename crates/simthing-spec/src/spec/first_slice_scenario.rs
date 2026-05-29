//! Phase M first-slice scenario fixture wrapper (narrow product/acceptance shape).
//!
//! Wraps an admitted [`RegionFieldSpec`] plus explicit [`MappingExecutionProfile`].
//! Not a general scenario engine; does not wire default SimSession.

use serde::{Deserialize, Serialize};

use super::region_field::{MappingExecutionProfile, RegionFieldSpec};

/// Designer-authored first-slice scenario fixture (explicit execution profile + region field).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirstSliceScenarioSpec {
    pub name: String,
    pub mapping_execution_profile: MappingExecutionProfile,
    pub region_field: RegionFieldSpec,
}
