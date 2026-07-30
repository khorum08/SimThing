//! STUDIO-FLEET-ICONS-0 — renderer-agnostic fleet icon base + narrow renderer seam.
//!
//! Descriptor math is pure data (no Bevy / mesh / material types). The production
//! mesh backend implements `FleetIconRenderer` and is the only path that turns
//! descriptors into draw plans; Bevy only applies the seam frame.

use std::collections::{BTreeMap, HashMap, HashSet};

use simthing_spec::{FleetPresenceLocation, FleetPresenceRecord};

use crate::star_render::{
    compute_star_distance_visual, star_max_layer_scale, StarBillboardRenderSettings,
};
use crate::view_model::{StudioGalaxyRenderMeta, StudioStarView, StudioSystemRenderAnchor};

/// Neutral tint when owner color is absent (matches nameplate neutral; no Spec).
const NEUTRAL_FLEET_ICON_RGBA: [f32; 4] = [0.92, 0.96, 1.0, 1.0];

/// Icons must stay ≤ this fraction of the admitted base max star-blur size.
pub const FLEET_ICON_MAX_STAR_BLUR_FRACTION: f32 = 0.75;
/// In-transit placement fraction along source → destination hyperlane geometry.
pub const FLEET_ICON_TRANSIT_ALONG_LANE_FRACTION: f32 = 0.30;
/// Anchored offset from star center as a fraction of that star's base max blur.
pub const FLEET_ICON_ANCHOR_OFFSET_FRACTION: f32 = 1.15;
/// Default requested scale as a fraction of the anchor star's base max blur (still capped).
pub const FLEET_ICON_DEFAULT_SCALE_FRACTION: f32 = 0.55;
/// Local silhouette nose in unit mesh space (toward +X on the map plane).
pub const FLEET_ICON_LOCAL_NOSE: [f32; 3] = [1.0, 0.0, 0.0];
/// Local mesh plane normal (silhouette lies flat on XZ map plane; normal +Y for top-down legibility).
pub const FLEET_ICON_LOCAL_PLANE_NORMAL: [f32; 3] = [0.0, 1.0, 0.0];

// ─── One-site silhouette DATA (change look here only) ─────────────────────────

/// Unit-space destroyer / rocket silhouette. Nose at +X; renderer scales + yaws.
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

pub const FLEET_ICON_DEFAULT_SILHOUETTE_ID: &str = FLEET_ICON_SILHOUETTE_DESTROYER.id;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FleetIconSilhouetteSpec {
    pub id: &'static str,
    pub outline_xy: &'static [(f32, f32)],
}

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
    Right,
    Left,
    Transit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FleetIconPlacement {
    Anchored {
        system_id: u32,
        side: FleetIconSide,
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
    TowardAnchorStar,
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
    /// World scale; always ≤ 75% of `anchor_star_blur`.
    pub scale: f32,
    /// Admitted base max star-blur for the placement anchor (presentation input).
    pub anchor_star_blur: f32,
}

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
            } => ("anchored", format!("system {system_id} side={side:?}")),
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

pub fn default_fleet_icon_scale(base_max_star_blur: f32) -> f32 {
    clamp_fleet_icon_scale(
        base_max_star_blur * FLEET_ICON_DEFAULT_SCALE_FRACTION,
        base_max_star_blur,
    )
}

/// Admitted **base** max star blur (unselected, near depth) for one star's visual size.
/// Uses the same radius/blur composition as the galaxy star path — not the selection multiplier alone.
pub fn admitted_base_max_star_blur_world(
    sprite_scale: f32,
    render_meta: &StudioGalaxyRenderMeta,
) -> f32 {
    let settings = StarBillboardRenderSettings::from_meta(render_meta);
    let visual = compute_star_distance_visual(0.0, false, false, &settings, true);
    let layer = star_max_layer_scale(visual, settings.render_mode);
    let scale = if sprite_scale.is_finite() && sprite_scale > 0.0 {
        sprite_scale
    } else {
        1.0
    };
    (scale * layer).max(1e-4)
}

/// Per-system admitted base max star blur from Studio star views + render meta.
pub fn admitted_base_star_blur_by_system(
    stars: &[StudioStarView],
    render_meta: &StudioGalaxyRenderMeta,
) -> HashMap<u32, f32> {
    let mut map = HashMap::new();
    for star in stars {
        map.insert(
            star.system_id,
            admitted_base_max_star_blur_world(star.sprite_scale, render_meta),
        );
    }
    map
}

fn blur_for_location(
    location: &FleetPresenceLocation,
    star_blur_by_system: &HashMap<u32, f32>,
) -> f32 {
    match location {
        FleetPresenceLocation::Anchored(system_id) => star_blur_by_system
            .get(system_id)
            .copied()
            .unwrap_or(1.0)
            .max(1e-4),
        FleetPresenceLocation::InTransit {
            source_system_id,
            dest_system_id,
        } => {
            let a = star_blur_by_system
                .get(source_system_id)
                .copied()
                .unwrap_or(1.0);
            let b = star_blur_by_system
                .get(dest_system_id)
                .copied()
                .unwrap_or(1.0);
            a.max(b).max(1e-4)
        }
    }
}

pub fn anchored_fleet_side(
    fleet_owner_id: Option<&str>,
    selected_owner_id: Option<&str>,
) -> FleetIconSide {
    match (selected_owner_id, fleet_owner_id) {
        (Some(selected), Some(fleet)) if selected == fleet => FleetIconSide::Right,
        _ => FleetIconSide::Left,
    }
}

/// Build descriptors from 12.4 records + **per-system** admitted star-blur sizes.
pub fn fleet_icon_descriptors_from_records(
    records: &[FleetPresenceRecord],
    selected_owner_id: Option<&str>,
    owner_tint_by_id: &HashMap<String, [f32; 4]>,
    star_blur_by_system: &HashMap<u32, f32>,
) -> Vec<FleetIconDescriptor> {
    let silhouette_id = FLEET_ICON_DEFAULT_SILHOUETTE_ID;
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
            .unwrap_or(NEUTRAL_FLEET_ICON_RGBA);
        let anchor_star_blur = blur_for_location(&record.location, star_blur_by_system);
        let scale = default_fleet_icon_scale(anchor_star_blur);

        let (placement, side, orientation) = match &record.location {
            FleetPresenceLocation::Anchored(system_id) => {
                let side = anchored_fleet_side(owner_id.as_deref(), selected_owner_id);
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
            anchor_star_blur,
        });
    }
    out
}

pub fn fleet_presence_records_flat(
    by_system_id: &BTreeMap<u32, Vec<FleetPresenceRecord>>,
) -> Vec<FleetPresenceRecord> {
    let mut out = Vec::new();
    for records in by_system_id.values() {
        out.extend(records.iter().cloned());
    }
    out.sort_by_key(|r| r.fleet_simthing_id_raw);
    out.dedup_by_key(|r| r.fleet_simthing_id_raw);
    out
}

/// Shared fleet-presence source selection for render + Studio_ops telemetry.
///
/// Law (Remand-3):
/// - **attached** live bridge → `live_presence` is authoritative **even when empty**;
/// - **unattached** → optional session fallback map;
/// - never infer attachment from map emptiness / total_fleets.
pub fn select_fleet_presence_records_for_icons(
    bridge_attached: bool,
    live_presence: &crate::studio_fleet_presence::StudioFleetPresenceMap,
    session_fallback: Option<&crate::studio_fleet_presence::StudioFleetPresenceMap>,
) -> Vec<FleetPresenceRecord> {
    if bridge_attached {
        return fleet_presence_records_flat(&live_presence.by_system_id);
    }
    match session_fallback {
        Some(map) => fleet_presence_records_flat(&map.by_system_id),
        None => Vec::new(),
    }
}

pub fn fleet_icon_ops_telemetry_rows(
    descriptors: &[FleetIconDescriptor],
) -> Vec<FleetIconOpsTelemetryRow> {
    descriptors.iter().map(|d| d.ops_telemetry_row()).collect()
}

// ─── World pose (still no Bevy types) ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FleetIconWorldPose {
    pub world_position: [f32; 3],
    /// Yaw about +Y matching Bevy `Quat::from_rotation_y` so local +X faces the target.
    pub yaw_radians: f32,
    pub scale: f32,
}

fn anchor_world(anchors: &[StudioSystemRenderAnchor], system_id: u32) -> Option<[f32; 3]> {
    anchors
        .iter()
        .find(|a| a.system_id == system_id)
        .map(|a| a.world_position)
}

/// Bevy Y-up: `Quat::from_rotation_y(yaw) * Vec3::X = (cos yaw, 0, -sin yaw)`.
/// Choose yaw so that equals the horizontal direction `(dx, dz)`.
pub fn yaw_toward_xz(dx: f32, dz: f32) -> f32 {
    if dx.abs() < 1e-8 && dz.abs() < 1e-8 {
        0.0
    } else {
        // sin = -dz/len, cos = dx/len → yaw = atan2(sin, cos) = atan2(-dz, dx)
        (-dz).atan2(dx)
    }
}

/// Rotate local vector by pose yaw about +Y (Bevy convention).
pub fn rotate_yaw_y(local: [f32; 3], yaw_radians: f32) -> [f32; 3] {
    let (s, c) = yaw_radians.sin_cos();
    let x = local[0];
    let y = local[1];
    let z = local[2];
    [c * x + s * z, y, -s * x + c * z]
}

pub fn fleet_icon_nose_world_dir(pose: &FleetIconWorldPose) -> [f32; 3] {
    rotate_yaw_y(FLEET_ICON_LOCAL_NOSE, pose.yaw_radians)
}

pub fn fleet_icon_plane_normal_world(pose: &FleetIconWorldPose) -> [f32; 3] {
    rotate_yaw_y(FLEET_ICON_LOCAL_PLANE_NORMAL, pose.yaw_radians)
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-8 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Resolve descriptor → world pose (offset uses this icon's anchor star blur).
pub fn resolve_fleet_icon_world_pose(
    descriptor: &FleetIconDescriptor,
    anchors: &[StudioSystemRenderAnchor],
    right_axis_xz: [f32; 2],
) -> Option<FleetIconWorldPose> {
    let blur = descriptor.anchor_star_blur.max(1e-4);
    let scale = clamp_fleet_icon_scale(descriptor.scale, blur);
    let offset = blur * FLEET_ICON_ANCHOR_OFFSET_FRACTION;
    let (rx, rz) = {
        let len = (right_axis_xz[0] * right_axis_xz[0] + right_axis_xz[1] * right_axis_xz[1])
            .sqrt();
        if len < 1e-8 {
            (1.0, 0.0)
        } else {
            (right_axis_xz[0] / len, right_axis_xz[1] / len)
        }
    };
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
            let yaw = yaw_toward_xz(star[0] - pos[0], star[2] - pos[2]);
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
            let yaw = yaw_toward_xz(dst[0] - src[0], dst[2] - src[2]);
            Some(FleetIconWorldPose {
                world_position: pos,
                yaw_radians: yaw,
                scale,
            })
        }
    }
}

/// Nose must face target; returns false if edge-on or reversed.
pub fn fleet_icon_nose_faces_target(pose: &FleetIconWorldPose, target: [f32; 3]) -> bool {
    let toward = normalize3([
        target[0] - pose.world_position[0],
        target[1] - pose.world_position[1],
        target[2] - pose.world_position[2],
    ]);
    if toward[0].abs() + toward[1].abs() + toward[2].abs() < 1e-6 {
        return false;
    }
    let nose = normalize3(fleet_icon_nose_world_dir(pose));
    dot3(nose, toward) > 0.99
}

/// Silhouette plane is legible when its normal is not near-parallel to view (not edge-on).
pub fn fleet_icon_plane_legible_to_view(pose: &FleetIconWorldPose, view_dir: [f32; 3]) -> bool {
    let n = normalize3(fleet_icon_plane_normal_world(pose));
    let v = normalize3(view_dir);
    // |n·v| near 1 ⇒ facing camera; near 0 ⇒ edge-on.
    dot3(n, v).abs() > 0.25
}

// ─── Narrow renderer seam ─────────────────────────────────────────────────────

/// Pure context for the seam (no Bevy types).
#[derive(Debug, Clone)]
pub struct FleetIconRenderContext<'a> {
    pub anchors: &'a [StudioSystemRenderAnchor],
    pub right_axis_xz: [f32; 2],
}

/// Mesh-outline draw plan (still no Bevy handles).
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconMeshDrawPlan {
    pub fleet_simthing_id_raw: u32,
    pub silhouette_id: &'static str,
    pub outline_xy: &'static [(f32, f32)],
    pub tint_rgba: [f32; 4],
    pub pose: FleetIconWorldPose,
    pub side: FleetIconSide,
    pub owner_id: Option<String>,
}

/// Canonical production frame produced by the mesh seam.
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconRenderFrame {
    pub descriptors: Vec<FleetIconDescriptor>,
    pub draw_plans: Vec<FleetIconMeshDrawPlan>,
}

/// Fingerprint of the draw contract (descriptors + resolved pose/tint/silhouette).
pub fn fleet_icon_frame_contract_fingerprint(frame: &FleetIconRenderFrame) -> Vec<(u32, u64)> {
    frame
        .draw_plans
        .iter()
        .map(|p| {
            let mut h: u64 = 0xcbf29ce484222325;
            let mix = |h: &mut u64, bytes: &[u8]| {
                for b in bytes {
                    *h ^= u64::from(*b);
                    *h = h.wrapping_mul(0x100000001b3);
                }
            };
            mix(&mut h, &p.fleet_simthing_id_raw.to_le_bytes());
            mix(&mut h, p.silhouette_id.as_bytes());
            mix(&mut h, &p.pose.world_position[0].to_bits().to_le_bytes());
            mix(&mut h, &p.pose.world_position[1].to_bits().to_le_bytes());
            mix(&mut h, &p.pose.world_position[2].to_bits().to_le_bytes());
            mix(&mut h, &p.pose.yaw_radians.to_bits().to_le_bytes());
            mix(&mut h, &p.pose.scale.to_bits().to_le_bytes());
            for c in p.tint_rgba {
                mix(&mut h, &c.to_bits().to_le_bytes());
            }
            (p.fleet_simthing_id_raw, h)
        })
        .collect()
}

/// Narrow renderer seam. Production mesh impl is the sole plan producer for Bevy apply.
pub trait FleetIconRenderer {
    type Frame;

    fn render_descriptors(
        &mut self,
        descriptors: &[FleetIconDescriptor],
        context: &FleetIconRenderContext<'_>,
    ) -> Self::Frame;
}

fn build_mesh_draw_plans(
    descriptors: &[FleetIconDescriptor],
    context: &FleetIconRenderContext<'_>,
) -> Vec<FleetIconMeshDrawPlan> {
    let mut plans = Vec::new();
    for desc in descriptors {
        let Some(silhouette) = fleet_icon_silhouette_by_id(desc.silhouette_id) else {
            continue;
        };
        let Some(pose) =
            resolve_fleet_icon_world_pose(desc, context.anchors, context.right_axis_xz)
        else {
            continue;
        };
        plans.push(FleetIconMeshDrawPlan {
            fleet_simthing_id_raw: desc.fleet_simthing_id_raw,
            silhouette_id: silhouette.id,
            outline_xy: silhouette.outline_xy,
            tint_rgba: desc.owner_tint_rgba,
            pose,
            side: desc.side,
            owner_id: desc.owner_id.clone(),
        });
    }
    plans
}

/// Production mesh-outline backend (existing Mesh/StandardMaterial apply target).
#[derive(Debug, Default, Clone)]
pub struct MeshOutlineFleetIconRenderer {
    pub render_calls: u32,
}

impl FleetIconRenderer for MeshOutlineFleetIconRenderer {
    type Frame = FleetIconRenderFrame;

    fn render_descriptors(
        &mut self,
        descriptors: &[FleetIconDescriptor],
        context: &FleetIconRenderContext<'_>,
    ) -> Self::Frame {
        self.render_calls = self.render_calls.saturating_add(1);
        FleetIconRenderFrame {
            descriptors: descriptors.to_vec(),
            draw_plans: build_mesh_draw_plans(descriptors, context),
        }
    }
}

/// Dummy second backend — must consume the same descriptors and produce an equal contract fingerprint.
#[derive(Debug, Default, Clone)]
pub struct DummySecondFleetIconBackend {
    pub accepted: Vec<FleetIconDescriptor>,
    pub last_frame: Option<FleetIconRenderFrame>,
}

impl FleetIconRenderer for DummySecondFleetIconBackend {
    type Frame = FleetIconRenderFrame;

    fn render_descriptors(
        &mut self,
        descriptors: &[FleetIconDescriptor],
        context: &FleetIconRenderContext<'_>,
    ) -> Self::Frame {
        self.accepted = descriptors.to_vec();
        let frame = FleetIconRenderFrame {
            descriptors: descriptors.to_vec(),
            draw_plans: build_mesh_draw_plans(descriptors, context),
        };
        self.last_frame = Some(frame.clone());
        frame
    }
}

/// Recording backend for tests that only care about descriptor identity.
#[derive(Debug, Default, Clone)]
pub struct RecordingFleetIconRenderer {
    pub last_descriptors: Vec<FleetIconDescriptor>,
    pub render_calls: u32,
}

impl FleetIconRenderer for RecordingFleetIconRenderer {
    type Frame = Vec<FleetIconDescriptor>;

    fn render_descriptors(
        &mut self,
        descriptors: &[FleetIconDescriptor],
        _context: &FleetIconRenderContext<'_>,
    ) -> Self::Frame {
        self.render_calls = self.render_calls.saturating_add(1);
        self.last_descriptors = descriptors.to_vec();
        self.last_descriptors.clone()
    }
}

/// Canonical production seam entry used by Bevy sync — the only plan producer.
pub fn production_fleet_icon_render_frame(
    descriptors: &[FleetIconDescriptor],
    context: &FleetIconRenderContext<'_>,
) -> FleetIconRenderFrame {
    MeshOutlineFleetIconRenderer::default().render_descriptors(descriptors, context)
}

// ─── Pure entity lifecycle ops (Bevy applies; tests bite without GPU) ─────────

#[derive(Debug, Clone, PartialEq)]
pub enum FleetIconEntityOp {
    Spawn(FleetIconMeshDrawPlan),
    Update(FleetIconMeshDrawPlan),
    Despawn { fleet_simthing_id_raw: u32 },
}

/// Diff live fleet ids against the production frame.
pub fn fleet_icon_entity_ops(
    frame: &FleetIconRenderFrame,
    live_fleet_ids: &[u32],
) -> Vec<FleetIconEntityOp> {
    let wanted: HashMap<u32, &FleetIconMeshDrawPlan> = frame
        .draw_plans
        .iter()
        .map(|p| (p.fleet_simthing_id_raw, p))
        .collect();
    let live: HashSet<u32> = live_fleet_ids.iter().copied().collect();
    let mut ops = Vec::new();
    for id in live_fleet_ids {
        match wanted.get(id) {
            Some(plan) => ops.push(FleetIconEntityOp::Update((*plan).clone())),
            None => ops.push(FleetIconEntityOp::Despawn {
                fleet_simthing_id_raw: *id,
            }),
        }
    }
    for plan in &frame.draw_plans {
        if !live.contains(&plan.fleet_simthing_id_raw) {
            ops.push(FleetIconEntityOp::Spawn(plan.clone()));
        }
    }
    ops
}

/// Headless scene state for lifecycle proofs (no Bevy).
#[derive(Debug, Default, Clone)]
pub struct FleetIconSceneState {
    /// fleet_id → last applied plan
    pub by_id: HashMap<u32, FleetIconMeshDrawPlan>,
}

impl FleetIconSceneState {
    pub fn live_ids(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.by_id.keys().copied().collect();
        ids.sort_unstable();
        ids
    }

    pub fn apply_frame(&mut self, frame: &FleetIconRenderFrame) {
        let ops = fleet_icon_entity_ops(frame, &self.live_ids());
        for op in ops {
            match op {
                FleetIconEntityOp::Spawn(plan) | FleetIconEntityOp::Update(plan) => {
                    self.by_id.insert(plan.fleet_simthing_id_raw, plan);
                }
                FleetIconEntityOp::Despawn {
                    fleet_simthing_id_raw,
                } => {
                    self.by_id.remove(&fleet_simthing_id_raw);
                }
            }
        }
    }

    /// Simulate root tracking after full scene replacement cleanup (old entities gone).
    pub fn clear_for_scene_cleanup(&mut self) {
        self.by_id.clear();
    }
}

/// Shared galaxy-scene cleanup collector — production `scene_cleanup` and headless
/// tests MUST use this single law so omitting `fleet_icons` fails the test.
pub fn collect_galaxy_scene_cleanup_entities<E: Copy>(
    stars: impl IntoIterator<Item = (u32, E)>,
    nameplates: impl IntoIterator<Item = E>,
    fleet_icons: impl IntoIterator<Item = (u32, E)>,
    hyperlane_buckets: impl IntoIterator<Item = Option<E>>,
    highlight: Option<E>,
    core_glow: Option<E>,
) -> Vec<E> {
    let mut entities = Vec::new();
    entities.extend(stars.into_iter().map(|(_, e)| e));
    entities.extend(nameplates);
    entities.extend(fleet_icons.into_iter().map(|(_, e)| e));
    entities.extend(hyperlane_buckets.into_iter().flatten());
    if let Some(e) = highlight {
        entities.push(e);
    }
    if let Some(e) = core_glow {
        entities.push(e);
    }
    entities
}

/// Convenience wrapper for headless u64 entity-id fixtures (same collector as production).
pub fn galaxy_scene_cleanup_entity_ids(
    stars: &[(u32, u64)],
    nameplates: &[u64],
    fleet_icons: &[(u32, u64)],
    hyperlane_buckets: &[Option<u64>],
    highlight: Option<u64>,
    core_glow: Option<u64>,
) -> Vec<u64> {
    collect_galaxy_scene_cleanup_entities(
        stars.iter().copied(),
        nameplates.iter().copied(),
        fleet_icons.iter().copied(),
        hyperlane_buckets.iter().copied(),
        highlight,
        core_glow,
    )
}

// ─── Pure mesh/transform data (Bevy conversion consumes these) ────────────────

/// Map-plane silhouette geometry: outline (x,z) → positions with y=0, normals +Y.
#[derive(Debug, Clone, PartialEq)]
pub struct FleetIconOutlineGeometry {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

/// Pure geometry data consumed by the production mesh builder (no Bevy types).
pub fn fleet_icon_outline_geometry(outline_xy: &[(f32, f32)]) -> FleetIconOutlineGeometry {
    if outline_xy.len() < 3 {
        return FleetIconOutlineGeometry {
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        };
    }
    let mut positions = Vec::with_capacity(outline_xy.len() + 1);
    let mut normals = Vec::with_capacity(outline_xy.len() + 1);
    let mut uvs = Vec::with_capacity(outline_xy.len() + 1);
    let mut indices = Vec::with_capacity((outline_xy.len() - 2) * 3);
    let mut cx = 0.0f32;
    let mut cz = 0.0f32;
    for &(x, z) in outline_xy {
        cx += x;
        cz += z;
    }
    let n = outline_xy.len() as f32;
    cx /= n;
    cz /= n;
    positions.push([cx, 0.0, cz]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);
    for &(x, z) in outline_xy {
        positions.push([x, 0.0, z]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([(x + 0.5).clamp(0.0, 1.0), (z + 0.5).clamp(0.0, 1.0)]);
    }
    for i in 1..=outline_xy.len() {
        let next = if i == outline_xy.len() { 1 } else { i + 1 };
        indices.extend_from_slice(&[0u32, i as u32, next as u32]);
    }
    FleetIconOutlineGeometry {
        positions,
        normals,
        uvs,
        indices,
    }
}

/// Pure transform data applied by Bevy `Transform` conversion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FleetIconTransformData {
    pub translation: [f32; 3],
    pub yaw_radians: f32,
    pub scale: f32,
}

pub fn fleet_icon_transform_data(plan: &FleetIconMeshDrawPlan) -> FleetIconTransformData {
    FleetIconTransformData {
        translation: plan.pose.world_position,
        yaw_radians: plan.pose.yaw_radians,
        scale: plan.pose.scale.max(1e-4),
    }
}

/// Rotate local +X by transform yaw (matches Bevy `Quat::from_rotation_y * Vec3::X`).
pub fn fleet_icon_transform_local_x_world(data: &FleetIconTransformData) -> [f32; 3] {
    rotate_yaw_y(FLEET_ICON_LOCAL_NOSE, data.yaw_radians)
}

/// Rotate local +Y plane normal (map-plane) by transform yaw.
pub fn fleet_icon_transform_local_y_world(data: &FleetIconTransformData) -> [f32; 3] {
    rotate_yaw_y(FLEET_ICON_LOCAL_PLANE_NORMAL, data.yaw_radians)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_spec::{FleetPresenceLocation, FleetPresenceRecord, OwnerRef};

    fn rec(id: u32, owner: Option<&str>, loc: FleetPresenceLocation) -> FleetPresenceRecord {
        FleetPresenceRecord {
            fleet_simthing_id_raw: id,
            owner_ref: owner.map(OwnerRef::new),
            posture: None,
            location: loc,
        }
    }

    fn blur_map(pairs: &[(u32, f32)]) -> HashMap<u32, f32> {
        pairs.iter().copied().collect()
    }

    fn ctx<'a>(anchors: &'a [StudioSystemRenderAnchor]) -> FleetIconRenderContext<'a> {
        FleetIconRenderContext {
            anchors,
            right_axis_xz: [1.0, 0.0],
        }
    }
}
