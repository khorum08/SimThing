//! Procedural starburst texture — render-only presentation metadata.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub const STARBURST_RENDER_ONLY_NOTE: &'static str =
    "starburst sprite scale/texture/emissive are presentation-only; structural (col,row) remain authoritative";

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
            let rays = ((angle * 8.0 / std::f32::consts::PI).sin().abs() * 0.55 + 0.45).powf(0.35);
            let radial = (1.0 - dist).max(0.0);
            let alpha = (radial * rays).clamp(0.0, 1.0);
            let core = (1.0 - dist * 1.2).max(0.0);
            let intensity = (alpha * 0.75 + core * 0.35).clamp(0.0, 1.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starburst_render_meta_is_render_only() {
        assert!(STARBURST_RENDER_ONLY_NOTE.contains("presentation-only"));
        assert!(STARBURST_RENDER_ONLY_NOTE.contains("structural"));
    }
}
