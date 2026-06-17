//! Procedural starburst texture — render-only presentation metadata.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const STARBURST_RENDER_ONLY_NOTE: &'static str =
    "starburst sprite scale/texture/emissive are presentation-only; structural (col,row) remain authoritative";

/// Billboard forward for a star quad facing the camera (render-only helper).
pub fn starburst_billboard_forward(star_pos: Vec3, camera_pos: Vec3) -> Vec3 {
    (camera_pos - star_pos).normalize_or_zero()
}

pub fn generate_starburst_image(size: u32) -> Image {
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 * 0.5;
    let inv_center = 1.0 / center.max(1.0);
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt() * inv_center;
            let angle = dy.atan2(dx);
            let rays = ((angle * 8.0 / std::f32::consts::PI).sin().abs() * 0.65 + 0.35).powf(0.55);
            let radial = (1.0 - dist).max(0.0).powf(2.6);
            let alpha = (radial * rays).clamp(0.0, 1.0);
            let core = (1.0 - dist * 2.8).max(0.0).powf(0.55);
            let intensity = (alpha * 0.65 + core * 0.70).clamp(0.0, 1.0);
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = (255.0 * intensity) as u8;
            data[idx + 1] = (255.0 * (0.85 + intensity * 0.15)) as u8;
            data[idx + 2] = 255;
            data[idx + 3] = (255.0 * alpha) as u8;
        }
    }
    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}

pub fn generate_star_aura_image(size: u32) -> Image {
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 * 0.5;
    let inv_center = 1.0 / center.max(1.0);
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt() * inv_center;
            let radial = (1.0 - dist).max(0.0).powf(3.8);
            let alpha = radial.clamp(0.0, 1.0);
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = (120.0 * alpha) as u8;
            data[idx + 1] = (190.0 * alpha) as u8;
            data[idx + 2] = (255.0 * alpha) as u8;
            data[idx + 3] = (255.0 * alpha) as u8;
        }
    }
    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}

pub fn generate_star_circle_image(size: u32) -> Image {
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 * 0.5;
    let inv_center = 1.0 / center.max(1.0);
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt() * inv_center;
            let edge = if dist <= 0.94 {
                1.0
            } else if dist <= 1.0 {
                (1.0 - dist) / 0.06
            } else {
                0.0
            };
            let alpha = edge.clamp(0.0, 1.0);
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 245;
            data[idx + 1] = 250;
            data[idx + 2] = 255;
            data[idx + 3] = (255.0 * alpha) as u8;
        }
    }
    Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starburst_render_meta_is_render_only() {
        assert!(STARBURST_RENDER_ONLY_NOTE.contains("presentation-only"));
        assert!(STARBURST_RENDER_ONLY_NOTE.contains("structural"));
    }

    #[test]
    fn starburst_billboard_faces_camera_helper() {
        let forward = starburst_billboard_forward(Vec3::ZERO, Vec3::new(0.0, 0.0, 10.0));
        assert!(forward.z > 0.9);
    }

    #[test]
    fn crisp_circle_texture_has_opaque_center_and_transparent_corner() {
        let image = generate_star_circle_image(16);
        let center_idx = ((8 * 16 + 8) * 4 + 3) as usize;
        let corner_idx = 3;
        assert!(image.data.as_ref().expect("image bytes")[center_idx] > 240);
        assert_eq!(image.data.as_ref().expect("image bytes")[corner_idx], 0);
    }
}
