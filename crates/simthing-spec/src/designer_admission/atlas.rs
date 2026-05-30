//! C-2 — Bounded Atlas Admission Relaxation (algebraic-G=0 only).
//!
//! This module implements the designer/spec layer admission relaxation for
//! bounded algebraic tile-local mask G=0 atlas batching.
//!
//! It is metadata/admission only. No production runtime, no default-on atlas,
//! no sparse scheduler, no active mask/source identity, and no physical gutter
//! as an accepted C-2 path.

use serde::{Deserialize, Serialize};

use super::diagnostic::{
    designer_admission_diagnostic, DesignerAdmissionDiagnostic, DesignerAdmissionDiagnosticCode,
};
use super::v7_8_line_scenarios::V78AtlasVramBudget;

/// The isolation mode admitted by C-2 (algebraic G=0 only for this gate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AtlasIsolationAdmissionMode {
    /// Algebraic tile-local mask G=0 (preferred, ~1.0× VRAM).
    AlgebraicTileLocalMaskG0,
}

/// Profile hint for budget modeling (used in tests and diagnostics).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AtlasAdmissionProfile {
    /// Typical huge commodity player-facing profile (e.g. ~128x128 map, 1000 stars, 5x5 surfaces).
    TypicalHugeCommodity,
    /// Horizon / dedicated server stress profile (2000-star 200x150 target from C-1).
    HorizonDedicatedServerStress,
}

/// A designer-authored atlas admission spec for C-2.
///
/// Only specs that are homogeneous-square, use algebraic G=0, are protocol-oracle-backed,
/// fit the active budget, and declare multiplier reporting can be admitted.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtlasAdmissionSpec {
    pub request_atlas_batching: bool,
    pub profile: AtlasAdmissionProfile,
    pub tile_width: u32,
    pub tile_height: u32,
    pub homogeneous_square_tiles: bool,
    pub isolation: AtlasIsolationAdmissionMode,
    pub protocol_oracle_backed: bool,
    pub active_vram_budget: V78AtlasVramBudget,
    pub multiplier_reporting_required: bool,

    // Explicit negatives to make intent clear at authoring time
    pub production_runtime: bool,
    pub default_on: bool,
    pub active_mask: bool,
    pub source_identity: bool,
    pub physical_gutter: bool,
}

/// Result of attempting to admit an atlas spec at the C-2 designer layer.
#[derive(Clone, Debug, PartialEq)]
pub struct AtlasAdmissionDecision {
    pub admitted: bool,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
    pub estimated_algebraic_bytes: Option<u64>,
    pub estimated_gutter_bytes: Option<u64>,
    pub active_budget_bytes: u64,
}

impl AtlasAdmissionSpec {
    /// Evaluate this spec for C-2 admission.
    ///
    /// Accepts only the narrow bounded algebraic-G=0 case that satisfies all C-2 conditions.
    pub fn evaluate(&self) -> AtlasAdmissionDecision {
        let mut diags = Vec::new();

        // Hard requirements for C-2
        if !self.request_atlas_batching {
            return AtlasAdmissionDecision {
                admitted: false,
                diagnostics: vec![designer_admission_diagnostic(
                    DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate,
                    "atlas batching requested without C-2 gate",
                    Some("use bounded algebraic-G=0 homogeneous square spec with budget fit for C-2"),
                )],
                estimated_algebraic_bytes: None,
                estimated_gutter_bytes: None,
                active_budget_bytes: self.active_vram_budget.max_bytes,
            };
        }

        if self.tile_width != self.tile_height || !self.homogeneous_square_tiles {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecNotHomogeneousSquareRejected,
                "atlas tiles must be homogeneous square for C-2",
                Some("set identical tile_width == tile_height and homogeneous_square_tiles = true"),
            ));
        }

        if self.isolation != AtlasIsolationAdmissionMode::AlgebraicTileLocalMaskG0 {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecUnsupportedIsolationRejected,
                "only algebraic G=0 is admitted in C-2",
                Some("use AlgebraicTileLocalMaskG0"),
            ));
        }

        if !self.protocol_oracle_backed {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecMissingProtocolOracleRejected,
                "protocol-oracle-backed requirement not met for C-2",
                Some("set protocol_oracle_backed = true"),
            ));
        }

        if !self.multiplier_reporting_required
            || !self.active_vram_budget.multiplier_reporting_required
        {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecMissingMultiplierReportingRejected,
                "multiplier reporting is mandatory for C-2",
                Some("set multiplier_reporting_required = true on both spec and budget"),
            ));
        }

        // Budget fit (using C-1 style effective accounting: 128 bytes per payload cell for algebraic)
        let total_dense_cells = self.estimate_total_dense_cells();
        let algebraic_bytes = total_dense_cells * 128; // C-0/C-1 effective algebraic G=0 accounting

        let fits_budget = algebraic_bytes <= self.active_vram_budget.max_bytes;

        if !fits_budget {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecOverActiveBudgetRejected,
                "atlas spec exceeds active V78AtlasVramBudget",
                Some("reduce scope, raise active budget, or declare sparse residency (later gate)"),
            ));
        }

        // Explicitly rejected paths in C-2
        if self.physical_gutter {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasSpecPhysicalGutterRequiresRaisedGateRejected,
                "physical gutter not admitted in C-2",
                Some("use algebraic G=0 or request raised budget + later gutter gate"),
            ));
        }
        if self.active_mask {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate,
                "active mask without separate gate",
                Some("M-6A remains deferred"),
            ));
        }
        if self.source_identity {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate,
                "source identity without separate gate",
                Some("M-5 remains deferred"),
            ));
        }
        if self.production_runtime || self.default_on {
            diags.push(designer_admission_diagnostic(
                DesignerAdmissionDiagnosticCode::AtlasProductionRuntimeRejected,
                "production runtime / default-on not authorized in C-2",
                Some("separate later gate for production atlas runtime and sparse scheduler"),
            ));
        }

        let admitted = diags.is_empty();

        AtlasAdmissionDecision {
            admitted,
            diagnostics: diags,
            estimated_algebraic_bytes: Some(algebraic_bytes),
            estimated_gutter_bytes: Some((algebraic_bytes as f64 * 6.76).ceil() as u64),
            active_budget_bytes: self.active_vram_budget.max_bytes,
        }
    }

    /// Rough cell count model for budget checking (re-uses C-1 style logic, simplified for admission).
    fn estimate_total_dense_cells(&self) -> u64 {
        // For C-2 we use a conservative model. Real scenarios will provide exact numbers via the claim.
        // Here we provide a reasonable upper-bound style estimate based on profile.
        match self.profile {
            AtlasAdmissionProfile::TypicalHugeCommodity => {
                // ~128x128 map, 1000 stars, 5x5 surfaces, 5 planets/star, 5 sats/planet
                let starmap = 128u64 * 128;
                let star_local = 1000u64 * 100;
                let planets = 1000u64 * 5;
                let orbital = planets * 100;
                let sats = planets * 5;
                let surfaces = (planets + sats) * 25; // 5x5
                starmap + star_local + orbital + surfaces
            }
            AtlasAdmissionProfile::HorizonDedicatedServerStress => {
                // 200x150 + 2000x10x10 + 10k x10x10 + 60k x10x10 = 7.23M (from C-1)
                7_230_000
            }
        }
    }
}

// Extend existing diagnostic codes for C-2 specific rejections.
impl DesignerAdmissionDiagnosticCode {
    // These are added for C-2 clarity (extend the enum in diagnostic.rs if not present)
}