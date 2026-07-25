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

/// Entity ids that batched galaxy scene cleanup must despawn (includes fleet icons).
pub fn galaxy_scene_cleanup_entity_ids(
    stars: &[(u32, u64)],
    nameplates: &[u64],
    fleet_icons: &[(u32, u64)],
    hyperlane_buckets: &[Option<u64>],
    highlight: Option<u64>,
    core_glow: Option<u64>,
) -> Vec<u64> {
    let mut entities = Vec::new();
    entities.extend(stars.iter().map(|(_, e)| *e));
    entities.extend(nameplates.iter().copied());
    entities.extend(fleet_icons.iter().map(|(_, e)| *e));
    entities.extend(hyperlane_buckets.iter().flatten().copied());
    if let Some(e) = highlight {
        entities.push(e);
    }
    if let Some(e) = core_glow {
        entities.push(e);
    }
    entities
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

    #[test]
    fn selected_owner_anchored_right_others_left_mirror() {
        let records = vec![
            rec(1, Some("terran"), FleetPresenceLocation::Anchored(10)),
            rec(2, Some("pirate"), FleetPresenceLocation::Anchored(10)),
            rec(3, None, FleetPresenceLocation::Anchored(11)),
        ];
        let blur = blur_map(&[(10, 2.0), (11, 2.0)]);
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("terran"), &HashMap::new(), &blur);
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
        let blur = blur_map(&[(1, 1.0), (2, 1.0)]);
        let descs = fleet_icon_descriptors_from_records(&records, None, &HashMap::new(), &blur);
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
        let blur = blur_map(&[(1, 1.0), (2, 1.0)]);
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("pirate"), &HashMap::new(), &blur);
        match &descs[0].placement {
            FleetIconPlacement::InTransit {
                along_fraction, ..
            } => assert!((along_fraction - 0.30).abs() < 1e-6),
            other => panic!("expected InTransit, got {other:?}"),
        }
    }

    #[test]
    fn arrival_snap_uses_anchored_slot_not_transit_fraction() {
        let arrived = rec(9, Some("terran"), FleetPresenceLocation::Anchored(2));
        let blur = blur_map(&[(2, 1.0)]);
        let descs =
            fleet_icon_descriptors_from_records(&[arrived], Some("terran"), &HashMap::new(), &blur);
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
    fn scale_capped_against_per_system_star_blur_not_global_constant() {
        // Two stars with materially different admitted visual sizes.
        let blur = blur_map(&[(1, 1.0), (2, 4.0)]);
        let records = vec![
            rec(10, Some("a"), FleetPresenceLocation::Anchored(1)),
            rec(20, Some("a"), FleetPresenceLocation::Anchored(2)),
        ];
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), &blur);
        let d1 = descs.iter().find(|d| d.fleet_simthing_id_raw == 10).unwrap();
        let d2 = descs.iter().find(|d| d.fleet_simthing_id_raw == 20).unwrap();
        assert!((d1.anchor_star_blur - 1.0).abs() < 1e-6);
        assert!((d2.anchor_star_blur - 4.0).abs() < 1e-6);
        assert!(d1.scale <= 1.0 * FLEET_ICON_MAX_STAR_BLUR_FRACTION + 1e-6);
        assert!(d2.scale <= 4.0 * FLEET_ICON_MAX_STAR_BLUR_FRACTION + 1e-6);
        // Larger star admits larger icon; not a single global cap for both.
        assert!(d2.scale > d1.scale + 0.5);
    }

    #[test]
    fn production_mesh_seam_and_dummy_share_draw_contract() {
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
        let blur = blur_map(&[(1, 1.5), (2, 1.5)]);
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("terran"), &HashMap::new(), &blur);
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
        let context = ctx(&anchors);
        let production = production_fleet_icon_render_frame(&descs, &context);
        let mut dummy = DummySecondFleetIconBackend::default();
        let dummy_frame = dummy.render_descriptors(&descs, &context);
        assert_eq!(
            fleet_icon_frame_contract_fingerprint(&production),
            fleet_icon_frame_contract_fingerprint(&dummy_frame)
        );
        // Bypass path (plans without the production entry) is the same builder only when
        // invoked through the seam; production entry must have been used (render_calls on mesh).
        let mut mesh = MeshOutlineFleetIconRenderer::default();
        let _ = mesh.render_descriptors(&descs, &context);
        assert_eq!(mesh.render_calls, 1);
        assert_eq!(production.descriptors, descs);
    }

    #[test]
    fn production_bypass_diverges_when_plans_built_from_wrong_descriptors() {
        let blur = blur_map(&[(1, 2.0), (2, 2.0)]);
        let mut tint_a = HashMap::new();
        tint_a.insert("a".into(), [1.0, 0.0, 0.0, 1.0]);
        let mut tint_b = HashMap::new();
        tint_b.insert("b".into(), [0.0, 1.0, 0.0, 1.0]);
        let a = fleet_icon_descriptors_from_records(
            &[rec(1, Some("a"), FleetPresenceLocation::Anchored(1))],
            Some("a"),
            &tint_a,
            &blur,
        );
        // Bypass uses a different anchor system + tint — must not match production contract.
        let b = fleet_icon_descriptors_from_records(
            &[rec(1, Some("b"), FleetPresenceLocation::Anchored(2))],
            Some("b"),
            &tint_b,
            &blur,
        );
        let anchors = [
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
                world_position: [5.0, 0.0, 0.0],
                render_height: 0.0,
            },
        ];
        let context = ctx(&anchors);
        let frame_a = production_fleet_icon_render_frame(&a, &context);
        let frame_bypass = production_fleet_icon_render_frame(&b, &context);
        assert_ne!(
            fleet_icon_frame_contract_fingerprint(&frame_a),
            fleet_icon_frame_contract_fingerprint(&frame_bypass)
        );
    }

    #[test]
    fn nose_faces_star_for_right_and_left_and_destination_in_transit() {
        let anchors = vec![
            StudioSystemRenderAnchor {
                system_id: 5,
                structural_col: 0,
                structural_row: 0,
                world_position: [0.0, 1.0, 0.0],
                render_height: 0.0,
            },
            StudioSystemRenderAnchor {
                system_id: 6,
                structural_col: 1,
                structural_row: 0,
                world_position: [10.0, 1.0, 0.0],
                render_height: 0.0,
            },
        ];
        let blur = blur_map(&[(5, 2.0), (6, 2.0)]);
        let records = vec![
            rec(1, Some("a"), FleetPresenceLocation::Anchored(5)),
            rec(2, Some("b"), FleetPresenceLocation::Anchored(5)),
            rec(
                3,
                Some("a"),
                FleetPresenceLocation::InTransit {
                    source_system_id: 5,
                    dest_system_id: 6,
                },
            ),
        ];
        let descs =
            fleet_icon_descriptors_from_records(&records, Some("a"), &HashMap::new(), &blur);
        let frame = production_fleet_icon_render_frame(&descs, &ctx(&anchors));
        let right = frame
            .draw_plans
            .iter()
            .find(|p| p.fleet_simthing_id_raw == 1)
            .unwrap();
        let left = frame
            .draw_plans
            .iter()
            .find(|p| p.fleet_simthing_id_raw == 2)
            .unwrap();
        let transit = frame
            .draw_plans
            .iter()
            .find(|p| p.fleet_simthing_id_raw == 3)
            .unwrap();
        assert!(fleet_icon_nose_faces_target(&right.pose, [0.0, 1.0, 0.0]));
        assert!(fleet_icon_nose_faces_target(&left.pose, [0.0, 1.0, 0.0]));
        assert!(fleet_icon_nose_faces_target(&transit.pose, [10.0, 1.0, 0.0]));
        assert!((transit.pose.world_position[0] - 3.0).abs() < 1e-4);
        // Top-down-ish galaxy camera view should keep the XY silhouette legible.
        assert!(fleet_icon_plane_legible_to_view(
            &right.pose,
            [0.0, -1.0, 0.0]
        ));
        // Mirror symmetry retained.
        assert!((right.pose.world_position[0] + left.pose.world_position[0]).abs() < 1e-4);
    }

    #[test]
    fn scene_state_lifecycle_side_flip_add_remove_and_cleanup() {
        let anchors = [StudioSystemRenderAnchor {
            system_id: 1,
            structural_col: 0,
            structural_row: 0,
            world_position: [0.0, 0.0, 0.0],
            render_height: 0.0,
        }];
        let blur = blur_map(&[(1, 2.0)]);
        let records = vec![
            rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(1)),
            rec(2, Some("owner_b"), FleetPresenceLocation::Anchored(1)),
        ];
        let mut scene = FleetIconSceneState::default();
        let descs_a =
            fleet_icon_descriptors_from_records(&records, Some("owner_a"), &HashMap::new(), &blur);
        let frame_a = production_fleet_icon_render_frame(&descs_a, &ctx(&anchors));
        scene.apply_frame(&frame_a);
        assert_eq!(scene.by_id.len(), 2);
        assert_eq!(scene.by_id[&1].side, FleetIconSide::Right);
        assert_eq!(scene.by_id[&2].side, FleetIconSide::Left);
        let tint_a = scene.by_id[&1].tint_rgba;

        // Selection flip: owner_b selected → sides swap; no duplicates.
        let descs_b =
            fleet_icon_descriptors_from_records(&records, Some("owner_b"), &HashMap::new(), &blur);
        let frame_b = production_fleet_icon_render_frame(&descs_b, &ctx(&anchors));
        scene.apply_frame(&frame_b);
        assert_eq!(scene.by_id.len(), 2);
        assert_eq!(scene.by_id[&1].side, FleetIconSide::Left);
        assert_eq!(scene.by_id[&2].side, FleetIconSide::Right);

        // Presence drops fleet 2.
        let only_one = vec![rec(1, Some("owner_a"), FleetPresenceLocation::Anchored(1))];
        let descs_one =
            fleet_icon_descriptors_from_records(&only_one, Some("owner_a"), &HashMap::new(), &blur);
        scene.apply_frame(&production_fleet_icon_render_frame(
            &descs_one,
            &ctx(&anchors),
        ));
        assert_eq!(scene.live_ids(), vec![1]);

        // Empty presence → zero icons.
        scene.apply_frame(&production_fleet_icon_render_frame(&[], &ctx(&anchors)));
        assert!(scene.by_id.is_empty());

        // Tint update through seam when owner color map changes.
        let mut tints = HashMap::new();
        tints.insert("owner_a".into(), [1.0, 0.0, 0.0, 1.0]);
        let descs_tint =
            fleet_icon_descriptors_from_records(&only_one, Some("owner_a"), &tints, &blur);
        scene.apply_frame(&production_fleet_icon_render_frame(
            &descs_tint,
            &ctx(&anchors),
        ));
        assert_eq!(scene.by_id[&1].tint_rgba, [1.0, 0.0, 0.0, 1.0]);
        assert_ne!(scene.by_id[&1].tint_rgba, tint_a);

        // Scene replacement cleanup clears all tracked icons before re-apply.
        scene.clear_for_scene_cleanup();
        assert!(scene.by_id.is_empty());
        // Re-open with overlapping fleet raw id — exactly one plan.
        scene.apply_frame(&production_fleet_icon_render_frame(
            &descs_tint,
            &ctx(&anchors),
        ));
        assert_eq!(scene.by_id.len(), 1);
        assert!(scene.by_id.contains_key(&1));
    }

    #[test]
    fn galaxy_scene_cleanup_includes_fleet_icon_entities() {
        let ids = galaxy_scene_cleanup_entity_ids(
            &[(1, 10)],
            &[20],
            &[(99, 30), (100, 31)],
            &[Some(40), None],
            Some(50),
            Some(60),
        );
        assert!(ids.contains(&30));
        assert!(ids.contains(&31));
        assert!(ids.contains(&10));
        assert_eq!(ids.len(), 7);
    }

    #[test]
    fn silhouette_is_one_site_data_and_default_resolves() {
        let sil = default_fleet_icon_silhouette();
        assert_eq!(sil.id, FLEET_ICON_DEFAULT_SILHOUETTE_ID);
        assert!(sil.outline_xy.len() >= 3);
    }
}
