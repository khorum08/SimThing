//! STUDIO-FLEET-ICONS-0 — renderer-agnostic fleet icon base + narrow renderer seam.
//!
//! Descriptor math is pure data (no Bevy / mesh / material types). Renderers consume
//! identical descriptors; silhouette shape is DATA at one site for one-edit look changes.

use std::collections::HashMap;

use simthing_spec::{FleetPresenceLocation, FleetPresenceRecord};

use crate::studio_faction_nameplates::NEUTRAL_NAMEPLATE_RGBA;
use crate::view_model::StudioSystemRenderAnchor;

/// Icons must stay ≤ this fraction of the admitted base max star-blur size.
pub const FLEET_ICON_MAX_STAR_BLUR_FRACTION: f32 = 0.75;
/// In-transit placement fraction along source → destination hyperlane geometry.
pub const FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION: f32 = 0.30;
/// Anchored offset from star center as a fraction of base max star-blur size.
pub const FLEET_ICON_ANCHOR_OFFSET_FRACTION: f32 = 1.15;
/// Default requested scale as a fraction of base max star-blur size (capped by max).
pub const FLEET_ICON_DEFAULT_SCALE_FRACTION: f32 = 0.55;

// ─── One-site silhouette DATA (change look here only) ─────────────────────────

/// Unit-space destroyer / rocket silhouette. Nose at +X; renderer scales + yaws.
/// Edit this table (or add a registry entry) to change the icon look without
/// touching placement math or renderer seam call sites.
pub const FLEET_ICON_SILHOUETTE_DESTROYER: FleetIconSilhouetteSpec = FleetIconSilhouetteSpec {
    id: "fleet.destroyer_v1",
    outline_xy: &[
        (0.55, 0.0),   // nose
        (-0.15, 0.22), // starboard mid
        (-0.45, 0.10), // starboard aft
        (-0.30, 0.0),  // notch
        (-0.45, -0.10),
        (-0.15, -0.22),
    ],
};

/// Canonical silhouette id used by descriptor construction.
pub const FLEET_ICON_DEFAULT_SILHOUETTE_ID: &str = FLEET_ICON_SILHOUETTE_DESTROYER.id;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FleetIconSilhouetteSpec {
    pub id: &'static str,
    /// Closed polygon in unit local space (nose toward +X).
    pub outline_xy: &'static [(f32, f32)],
}

/// One-site silhouette registry. Adding a look = one table entry here.
pub fn fleet_icon_silhouette_by_id(id: &str) -> Option<&'static FleetIconSilhouetteSpec> {
    match id {
        id if id == FLEET_ICON_SILHOUETTE_DESTROYER.id => Some(&FLEET_ICON_SILHOUETTE_DESTROYER),
        _ => None,
    }
}

pub fn default_fleet_icon_silhouette() -> &'static FleetIconSilhouetteSpec {
    &FLEET_ICON_SILHOUETTE_DESTROYER
}

// ─── Descriptor layer (no render types) ───────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FleetIconSide {
    /// Anchored fleet owned by the currently selected owner.
    Right,
    /// Anchored hostile / neutral / no-owner-selected fleets.
    Left,
    /// In transit along a hyperlane (not a star-side slot).
    Transit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FleetIconPlacement {
    Anchored {
        system_id: u32,
        side: FleetIconSide,
        /// Stable stack index among fleets sharing the same star+side.
        stack_index: u32,
    },
    InTransit {
        source_system_id: u32,
        dest_system_id: u32,
        along_fraction: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleetIconOrientation {
    /// Nose points at the anchor star (from the side slot).
    TowardAnchorStar,
    /// Nose points toward the transit destination.
    TowardTransitDestination,
}

/// Renderer-agnostic per-fleet icon descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconDescriptor {
    pub fleet_simthing_id_raw: u32,
    pub silhouette_id: &'static str,
    pub owner_id: Option<String>,
    pub owner_tint_rgba: [f32; 4],
    pub placement: FleetIconPlacement,
    pub side: FleetIconSide,
    pub orientation: FleetIconOrientation,
    /// Absolute scale in star-blur size units; always ≤ max fraction of base max blur.
    pub scale: f32,
}

/// Ops-telemetry row for Owner OVL (read-only presentation).
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconOpsTelemetryRow {
    pub fleet_simthing_id_raw: u32,
    pub owner_id: Option<String>,
    pub placement_kind: &'static str,
    pub side: FleetIconSide,
    pub scale: f32,
    pub system_or_lane: String,
}

impl FleetIconDescriptor {
    pub fn ops_telemetry_row(&self) -> FleetIconOpsTelemetryRow {
        let (placement_kind, system_or_lane) = match &self.placement {
            FleetIconPlacement::Anchored {
                system_id, side, ..
            } => (
                "anchored",
                format!("system {system_id} side={side:?}"),
            ),
            FleetIconPlacement::InTransit {
                source_system_id,
                dest_system_id,
                along_fraction,
            } => (
                "in_transit",
                format!("{source_system_id}->{dest_system_id} t={along_fraction:.2}"),
            ),
        };
        FleetIconOpsTelemetryRow {
            fleet_simthing_id_raw: self.fleet_simthing_id_raw,
            owner_id: self.owner_id.clone(),
            placement_kind,
            side: self.side,
            scale: self.scale,
            system_or_lane,
        }
    }
}

/// Clamp requested scale so icons never exceed 75% of base max star blur.
pub fn clamp_fleet_icon_scale(requested: f32, base_max_star_blur: f32) -> f32 {
    let base = if base_max_star_blur.is_finite() && base_max_star_blur > 0.0 {
        base_max_star_blur
    } else {
        1.0
    };
    let cap = base * FLEET_ICON_MAX_STAR_BLUR_FRACTION;
    let req = if requested.is_finite() {
        requested.max(0.0)
    } else {
        0.0
    };
    req.min(cap)
}

/// Default icon scale for a given base max star-blur size (still ≤ 75% cap).
pub fn default_fleet_icon_scale(base_max_star_blur: f32) -> f32 {
    clamp_fleet_icon_scale(
        base_max_star_blur * FLEET_ICON_DEFAULT_SCALE_FRACTION,
        base_max_star_blur,
    )
}

/// Which side an anchored fleet occupies given selected owner (if any).
pub fn anchored_fleet_side(
    fleet_owner_id: Option<&str>,
    selected_owner_id: Option<&str>,
) -> FleetIconSide {
    match (selected_owner_id, fleet_owner_id) {
        (Some(selected), Some(fleet)) if selected == fleet => FleetIconSide::Right,
        _ => FleetIconSide::Left,
    }
}

/// Build renderer-agnostic descriptors from the admitted 12.4 presence records.
///
/// - Anchored fleets of the selected owner → Right; all others Left (including no selection).
/// - InTransit → ~30% along source→dest; orientation toward destination.
/// - Scale always ≤ 75% of `base_max_star_blur`.
/// - Silhouette id is the one-site default destroyer table.
pub fn fleet_icon_descriptors_from_records(
    records: &[FleetPresenceRecord],
    selected_owner_id: Option<&str>,
    owner_tint_by_id: &HashMap<String, [f32; 4]>,
    base_max_star_blur: f32,
) -> Vec<FleetIconDescriptor> {
    let scale = default_fleet_icon_scale(base_max_star_blur);
    let silhouette_id = FLEET_ICON_DEFAULT_SILHOUETTE_ID;

    // Stable order: by fleet id, then stack indices per (system, side).
    let mut sorted: Vec<&FleetPresenceRecord> = records.iter().collect();
    sorted.sort_by_key(|r| r.fleet_simthing_id_raw);

    let mut stack_counts: HashMap<(u32, FleetIconSide), u32> = HashMap::new();
    let mut out = Vec::with_capacity(sorted.len());

    for record in sorted {
        let owner_id = record
            .owner_ref
            .as_ref()
            .map(|owner| owner.as_str().to_string());
        let tint = owner_id
            .as_ref()
            .and_then(|id| owner_tint_by_id.get(id).copied())
            .unwrap_or(NEUTRAL_NAMEPLATE_RGBA);

        let (placement, side, orientation) = match &record.location {
            FleetPresenceLocation::Anchored(system_id) => {
                let side =
                    anchored_fleet_side(owner_id.as_deref(), selected_owner_id);
                let key = (*system_id, side);
                let stack_index = *stack_counts.entry(key).or_insert(0);
                *stack_counts.get_mut(&key).expect("just inserted") += 1;
                (
                    FleetIconPlacement::Anchored {
                        system_id: *system_id,
                        side,
                        stack_index,
                    },
                    side,
                    FleetIconOrientation::TowardAnchorStar,
                )
            }
            FleetPresenceLocation::InTransit {
                source_system_id,
                dest_system_id,
            } => (
                FleetIconPlacement::InTransit {
                    source_system_id: *source_system_id,
                    dest_system_id: *dest_system_id,
                    along_fraction: FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION,
                },
                FleetIconSide::Transit,
                FleetIconOrientation::TowardTransitDestination,
            ),
        };

        out.push(FleetIconDescriptor {
            fleet_simthing_id_raw: record.fleet_simthing_id_raw,
            silhouette_id,
            owner_id,
            owner_tint_rgba: tint,
            placement,
            side,
            orientation,
            scale,
        });
    }
    out
}

/// Flatten a system-keyed presence map into a record list for descriptor build.
pub fn fleet_presence_records_flat(
    by_system_id: &std::collections::BTreeMap<u32, Vec<FleetPresenceRecord>>,
) -> Vec<FleetPresenceRecord> {
    let mut out = Vec::new();
    for records in by_system_id.values() {
        out.extend(records.iter().cloned());
    }
    out.sort_by_key(|r| r.fleet_simthing_id_raw);
    out.dedup_by_key(|r| r.fleet_simthing_id_raw);
    out
}

pub fn fleet_icon_ops_telemetry_rows(
    descriptors: &[FleetIconDescriptor],
) -> Vec<FleetIconOpsTelemetryRow> {
    descriptors.iter().map(|d| d.ops_telemetry_row()).collect()
}

// ─── World pose resolve (still no Bevy types — pure f32 arrays) ───────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FleetIconWorldPose {
    pub world_position: [f32; 3],
    /// Yaw about +Y so local +X (silhouette nose) faces the orientation target.
    pub yaw_radians: f32,
    pub scale: f32,
}

fn anchor_world(
    anchors: &[StudioSystemRenderAnchor],
    system_id: u32,
) -> Option<[f32; 3]> {
    anchors
        .iter()
        .find(|a| a.system_id == system_id)
        .map(|a| a.world_position)
}

/// Yaw about +Y so local +X (silhouette nose) faces the (dx, dz) direction in XZ.
fn yaw_xz(dx: f32, dz: f32) -> f32 {
    if dx.abs() < 1e-8 && dz.abs() < 1e-8 {
        0.0
    } else {
        dx.atan2(dz)
    }
}

/// Resolve a descriptor into world pose using hyperlane/star anchors (render-only).
///
/// `right_axis_xz` is the camera-right (or default [1,0]) used for left/right slots.
pub fn resolve_fleet_icon_world_pose(
    descriptor: &FleetIconDescriptor,
    anchors: &[StudioSystemRenderAnchor],
    right_axis_xz: [f32; 2],
    base_max_star_blur: f32,
) -> Option<FleetIconWorldPose> {
    let scale = clamp_fleet_icon_scale(descriptor.scale, base_max_star_blur);
    let offset = base_max_star_blur.max(1e-3) * FLEET_ICON_ANCHOR_OFFSET_FRACTION;
    let (rx, rz) = {
        let len = (right_axis_xz[0] * right_axis_xz[0] + right_axis_xz[1] * right_axis_xz[1])
            .sqrt();
        if len < 1e-8 {
            (1.0, 0.0)
        } else {
            (right_axis_xz[0] / len, right_axis_xz[1] / len)
        }
    };
    // Perpendicular in XZ for stack (rotate right 90° → forward-ish).
    let (px, pz) = (-rz, rx);

    match &descriptor.placement {
        FleetIconPlacement::Anchored {
            system_id,
            side,
            stack_index,
        } => {
            let star = anchor_world(anchors, *system_id)?;
            let side_sign = match side {
                FleetIconSide::Right => 1.0,
                FleetIconSide::Left => -1.0,
                FleetIconSide::Transit => 0.0,
            };
            let stack = *stack_index as f32 * (scale * 0.85);
            let pos = [
                star[0] + rx * offset * side_sign + px * stack,
                star[1],
                star[2] + rz * offset * side_sign + pz * stack,
            ];
            // Nose toward star.
            let yaw = yaw_xz(star[0] - pos[0], star[2] - pos[2]);
            Some(FleetIconWorldPose {
                world_position: pos,
                yaw_radians: yaw,
                scale,
            })
        }
        FleetIconPlacement::InTransit {
            source_system_id,
            dest_system_id,
            along_fraction,
        } => {
            let src = anchor_world(anchors, *source_system_id)?;
            let dst = anchor_world(anchors, *dest_system_id)?;
            let t = along_fraction.clamp(0.0, 1.0);
            let pos = [
                src[0] + (dst[0] - src[0]) * t,
                src[1] + (dst[1] - src[1]) * t,
                src[2] + (dst[2] - src[2]) * t,
            ];
            let yaw = yaw_xz(dst[0] - src[0], dst[2] - src[2]);
            Some(FleetIconWorldPose {
                world_position: pos,
                yaw_radians: yaw,
                scale,
            })
        }
    }
}

// ─── Narrow renderer seam ─────────────────────────────────────────────────────

/// Narrow renderer seam. Current sole production impl draws via existing mesh/material
/// paths; a dummy second backend consumes identical descriptors for forward-compat proof.
pub trait FleetIconRenderer {
    type Frame;

    fn render_descriptors(&mut self, descriptors: &[FleetIconDescriptor]) -> Self::Frame;
}

/// Production-shaped recording backend used as the first concrete impl surface in tests
/// (and as a stand-in when a full Bevy frame is not available). No Bevy types.
#[derive(Debug, Default, Clone)]
pub struct RecordingFleetIconRenderer {
    pub last_frame: Vec<FleetIconDescriptor>,
    pub render_calls: u32,
}

impl FleetIconRenderer for RecordingFleetIconRenderer {
    type Frame = Vec<FleetIconDescriptor>;

    fn render_descriptors(&mut self, descriptors: &[FleetIconDescriptor]) -> Self::Frame {
        self.render_calls = self.render_calls.saturating_add(1);
        self.last_frame = descriptors.to_vec();
        self.last_frame.clone()
    }
}

/// Dummy second backend — proves descriptors are backend-agnostic.
#[derive(Debug, Default, Clone)]
pub struct DummySecondFleetIconBackend {
    pub accepted: Vec<FleetIconDescriptor>,
}

impl FleetIconRenderer for DummySecondFleetIconBackend {
    type Frame = usize;

    fn render_descriptors(&mut self, descriptors: &[FleetIconDescriptor]) -> Self::Frame {
        self.accepted = descriptors.to_vec();
        self.accepted.len()
    }
}

/// Mesh-outline draw plan derived from a descriptor (still no Bevy handles).
/// The Windows galaxy_render path turns these into existing Mesh/StandardMaterial entities.
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconMeshDrawPlan {
    pub fleet_simthing_id_raw: u32,
    pub silhouette_id: &'static str,
    pub outline_xy: &'static [(f32, f32)],
    pub tint_rgba: [f32; 4],
    pub pose: FleetIconWorldPose,
}

pub fn fleet_icon_mesh_draw_plans(
    descriptors: &[FleetIconDescriptor],
    anchors: &[StudioSystemRenderAnchor],
    right_axis_xz: [f32; 2],
    base_max_star_blur: f32,
) -> Vec<FleetIconMeshDrawPlan> {
    let mut plans = Vec::new();
    for desc in descriptors {
        let Some(silhouette) = fleet_icon_silhouette_by_id(desc.silhouette_id) else {
            continue;
        };
        let Some(pose) =
            resolve_fleet_icon_world_pose(desc, anchors, right_axis_xz, base_max_star_blur)
        else {
            continue;
        };
        plans.push(FleetIconMeshDrawPlan {
            fleet_simthing_id_raw: desc.fleet_simthing_id_raw,
            silhouette_id: silhouette.id,
            outline_xy: silhouette.outline_xy,
            tint_rgba: desc.owner_tint_rgba,
            pose,
        });
    }
    plans
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_spec::{FleetPresenceLocation, FleetPresenceRecord, OwnerRef};

    fn rec(
        id: u32,
        owner: Option<&str>,
        loc: FleetPresenceLocation,
    ) -> FleetPresenceRecord {
        FleetPresenceRecord {
            fleet_simthing_id_raw: id,
            owner_ref: owner.map(OwnerRef::new),
            posture: None,
            location: loc,
        }
    }

    #[test]
    fn selected_owner_anchored_right_others_left_mirror() {
        let records = vec![
            rec(1, Some("terran"), FleetPresenceLocation::Anchored(10)),
            rec(2, Some("pirate"), FleetPresenceLocation::Anchored(10)),
            rec(3, None, FleetPresenceLocation::Anchored(11)),
        ];
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("terran"), &HashMap::new(), 2.0);
        assert_eq!(descs.len(), 3);
        let by_id: HashMap<_, _> = descs
            .iter()
            .map(|d| (d.fleet_simthing_id_raw, d.side))
            .collect();
        assert_eq!(by_id[&1], FleetIconSide::Right);
        assert_eq!(by_id[&2], FleetIconSide::Left);
        assert_eq!(by_id[&3], FleetIconSide::Left);
    }

    #[test]
    fn no_selected_owner_all_anchored_fleets_left() {
        let records = vec![
            rec(1, Some("terran"), FleetPresenceLocation::Anchored(1)),
            rec(2, Some("pirate"), FleetPresenceLocation::Anchored(2)),
        ];
        let descs = fleet_icon_descriptors_from_records(&records, None, &HashMap::new(), 1.0);
        assert!(descs.iter().all(|d| d.side == FleetIconSide::Left));
    }

    #[test]
    fn transit_places_at_thirty_percent_toward_destination() {
        let records = vec![rec(
            9,
            Some("pirate"),
            FleetPresenceLocation::InTransit {
                source_system_id: 1,
                dest_system_id: 2,
            },
        )];
        let descs = fleet_icon_descriptors_from_records(&records, Some("pirate"), &HashMap::new(), 1.0);
        assert_eq!(descs.len(), 1);
        assert_eq!(descs[0].side, FleetIconSide::Transit);
        assert_eq!(
            descs[0].orientation,
            FleetIconOrientation::TowardTransitDestination
        );
        match &descs[0].placement {
            FleetIconPlacement::InTransit {
                source_system_id,
                dest_system_id,
                along_fraction,
            } => {
                assert_eq!(*source_system_id, 1);
                assert_eq!(*dest_system_id, 2);
                assert!((along_fraction - 0.30).abs() < 1e-6);
            }
            other => panic!("expected InTransit, got {other:?}"),
        }
    }

    #[test]
    fn arrival_snap_uses_anchored_slot_not_transit_fraction() {
        // Same fleet id, location becomes Anchored → descriptor snaps to star side.
        let arrived = rec(9, Some("terran"), FleetPresenceLocation::Anchored(2));
        let descs =
            fleet_icon_descriptors_from_records(&[arrived], Some("terran"), &HashMap::new(), 1.0);
        match &descs[0].placement {
            FleetIconPlacement::Anchored {
                system_id, side, ..
            } => {
                assert_eq!(*system_id, 2);
                assert_eq!(*side, FleetIconSide::Right);
            }
            other => panic!("arrival must snap to Anchored, got {other:?}"),
        }
    }

    #[test]
    fn scale_never_exceeds_seventy_five_percent_of_base_max_star_blur() {
        let base = 4.0;
        let cap = base * FLEET_ICON_MAX_STAR_BLUR_FRACTION;
        assert!((clamp_fleet_icon_scale(100.0, base) - cap).abs() < 1e-6);
        assert!(default_fleet_icon_scale(base) <= cap + 1e-6);
        let records = vec![rec(1, Some("a"), FleetPresenceLocation::Anchored(1))];
        let descs = fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), base);
        assert!(descs[0].scale <= cap + 1e-6);
    }

    #[test]
    fn dummy_second_backend_consumes_identical_descriptors() {
        let records = vec![
            rec(1, Some("terran"), FleetPresenceLocation::Anchored(1)),
            rec(
                2,
                Some("pirate"),
                FleetPresenceLocation::InTransit {
                    source_system_id: 1,
                    dest_system_id: 2,
                },
            ),
        ];
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("terran"), &HashMap::new(), 1.5);
        let mut primary = RecordingFleetIconRenderer::default();
        let mut dummy = DummySecondFleetIconBackend::default();
        let frame_a = primary.render_descriptors(&descs);
        let count_b = dummy.render_descriptors(&descs);
        assert_eq!(frame_a, descs);
        assert_eq!(dummy.accepted, descs);
        assert_eq!(count_b, descs.len());
        assert_eq!(primary.last_frame, dummy.accepted);
    }

    #[test]
    fn silhouette_is_one_site_data_and_default_resolves() {
        let sil = default_fleet_icon_silhouette();
        assert_eq!(sil.id, FLEET_ICON_DEFAULT_SILHOUETTE_ID);
        assert!(sil.outline_xy.len() >= 3);
        assert!(fleet_icon_silhouette_by_id(sil.id).is_some());
        assert!(fleet_icon_silhouette_by_id("missing").is_none());
    }

    #[test]
    fn world_pose_transit_is_thirty_percent_along_lane() {
        let anchors = vec![
            StudioSystemRenderAnchor {
                system_id: 1,
                structural_col: 0,
                structural_row: 0,
                world_position: [0.0, 0.0, 0.0],
                render_height: 0.0,
            },
            StudioSystemRenderAnchor {
                system_id: 2,
                structural_col: 1,
                structural_row: 0,
                world_position: [10.0, 0.0, 0.0],
                render_height: 0.0,
            },
        ];
        let desc = FleetIconDescriptor {
            fleet_simthing_id_raw: 1,
            silhouette_id: FLEET_ICON_DEFAULT_SILHOUETTE_ID,
            owner_id: None,
            owner_tint_rgba: NEUTRAL_NAMEPLATE_RGBA,
            placement: FleetIconPlacement::InTransit {
                source_system_id: 1,
                dest_system_id: 2,
                along_fraction: FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION,
            },
            side: FleetIconSide::Transit,
            orientation: FleetIconOrientation::TowardTransitDestination,
            scale: 0.5,
        };
        let pose = resolve_fleet_icon_world_pose(&desc, &anchors, [1.0, 0.0], 1.0).expect("pose");
        assert!((pose.world_position[0] - 3.0).abs() < 1e-5);
        assert!((pose.world_position[2]).abs() < 1e-5);
    }

    #[test]
    fn world_pose_right_and_left_are_mirror_symmetric() {
        let anchors = vec![StudioSystemRenderAnchor {
            system_id: 5,
            structural_col: 0,
            structural_row: 0,
            world_position: [0.0, 1.0, 0.0],
            render_height: 0.0,
        }];
        let base = 2.0;
        let right = FleetIconDescriptor {
            fleet_simthing_id_raw: 1,
            silhouette_id: FLEET_ICON_DEFAULT_SILHOUETTE_ID,
            owner_id: Some("a".into()),
            owner_tint_rgba: NEUTRAL_NAMEPLATE_RGBA,
            placement: FleetIconPlacement::Anchored {
                system_id: 5,
                side: FleetIconSide::Right,
                stack_index: 0,
            },
            side: FleetIconSide::Right,
            orientation: FleetIconOrientation::TowardAnchorStar,
            scale: default_fleet_icon_scale(base),
        };
        let left = FleetIconDescriptor {
            fleet_simthing_id_raw: 2,
            side: FleetIconSide::Left,
            placement: FleetIconPlacement::Anchored {
                system_id: 5,
                side: FleetIconSide::Left,
                stack_index: 0,
            },
            ..right.clone()
        };
        let pr = resolve_fleet_icon_world_pose(&right, &anchors, [1.0, 0.0], base).unwrap();
        let pl = resolve_fleet_icon_world_pose(&left, &anchors, [1.0, 0.0], base).unwrap();
        assert!((pr.world_position[0] + pl.world_position[0]).abs() < 1e-5);
        assert!((pr.world_position[2] - pl.world_position[2]).abs() < 1e-5);
    }
}
