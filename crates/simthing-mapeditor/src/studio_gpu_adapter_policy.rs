//! STUDIO-GPU-ADAPTER-ENFORCE-0 — exact Windows Studio adapter policy and telemetry mapping.

use std::fmt;

use bevy::render::{
    renderer::RenderAdapterInfo,
    settings::{Backends, InstanceFlags, PowerPreference, WgpuSettings},
};

use crate::studio_performance_telemetry::StudioPerformanceTelemetry;

pub const REQUIRED_STUDIO_GPU_ADAPTER_NAME: &str = "NVIDIA GeForce RTX 4080 Laptop GPU";
pub const REQUIRED_STUDIO_GPU_VENDOR_ID: u32 = 0x10de;
pub const REQUIRED_STUDIO_GPU_DEVICE_TYPE: &str = "DiscreteGpu";
pub const BLOCKED_STUDIO_GPU_BACKEND: &str = "Dx12";
pub const STUDIO_GPU_POLICY_SATISFIED_PREFIX: &str =
    "satisfied: exact NVIDIA GeForce RTX 4080 Laptop GPU / NVIDIA / DiscreteGpu / backend";

/// Bevy 0.16 automatic adapter requests leave `force_fallback_adapter` at its `false` default.
/// The exact post-init validator below then rejects every non-required adapter rather than falling
/// back or silently honoring an environment-selected downgrade.
pub const STUDIO_GPU_FORCE_FALLBACK_ADAPTER: bool = false;

/// Renderer settings that allow Bevy-supported backends except the load-crashing DX12 path.
///
/// `VALIDATION` stays enabled. `DEBUG` is intentionally not requested because optional backend debug
/// layers are host-dependent and must not turn a missing developer component into a startup warning.
pub fn required_studio_wgpu_settings() -> WgpuSettings {
    let mut settings = WgpuSettings::default();
    let mut backends = Backends::all();
    backends.remove(Backends::DX12);
    settings.backends = Some(backends);
    settings.power_preference = PowerPreference::HighPerformance;
    settings.instance_flags = InstanceFlags::VALIDATION;
    settings
}

/// Pure snapshot of the adapter identity Bevy actually initialized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGpuAdapterSnapshot {
    pub name: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: String,
    pub backend: String,
}

impl StudioGpuAdapterSnapshot {
    pub fn new(
        name: impl Into<String>,
        vendor_id: u32,
        device_id: u32,
        device_type: impl Into<String>,
        backend: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            vendor_id,
            device_id,
            device_type: device_type.into(),
            backend: backend.into(),
        }
    }

    pub fn from_render_adapter_info(adapter_info: &RenderAdapterInfo) -> Self {
        let info = &adapter_info.0;
        Self::new(
            info.name.clone(),
            info.vendor,
            info.device,
            format!("{:?}", info.device_type),
            format!("{:?}", info.backend),
        )
    }

    fn observed_details(&self) -> String {
        format!(
            "name={:?}, vendor={:#06x}, device={:#06x}, device_type={}, backend={}",
            self.name, self.vendor_id, self.device_id, self.device_type, self.backend
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGpuAdapterPolicyViolation {
    mismatches: Vec<&'static str>,
    observed: StudioGpuAdapterSnapshot,
}

impl StudioGpuAdapterPolicyViolation {
    pub fn observed(&self) -> &StudioGpuAdapterSnapshot {
        &self.observed
    }
}

impl fmt::Display for StudioGpuAdapterPolicyViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Studio GPU adapter policy mismatch ({}) — required name={:?}, vendor={:#06x}, \
             device_type={}, blocked_backend={}; observed {}",
            self.mismatches.join(", "),
            REQUIRED_STUDIO_GPU_ADAPTER_NAME,
            REQUIRED_STUDIO_GPU_VENDOR_ID,
            REQUIRED_STUDIO_GPU_DEVICE_TYPE,
            BLOCKED_STUDIO_GPU_BACKEND,
            self.observed.observed_details(),
        )
    }
}

impl std::error::Error for StudioGpuAdapterPolicyViolation {}

pub fn validate_studio_gpu_adapter(
    observed: &StudioGpuAdapterSnapshot,
) -> Result<(), StudioGpuAdapterPolicyViolation> {
    let mut mismatches = Vec::new();
    if observed.name != REQUIRED_STUDIO_GPU_ADAPTER_NAME {
        mismatches.push("adapter name");
    }
    if observed.vendor_id != REQUIRED_STUDIO_GPU_VENDOR_ID {
        mismatches.push("vendor");
    }
    if observed.device_type != REQUIRED_STUDIO_GPU_DEVICE_TYPE {
        mismatches.push("device type");
    }
    if observed.backend == BLOCKED_STUDIO_GPU_BACKEND {
        mismatches.push("blocked backend");
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(StudioGpuAdapterPolicyViolation {
            mismatches,
            observed: observed.clone(),
        })
    }
}

/// Copies only the actual initialized adapter snapshot into presentation telemetry, then validates
/// it. Even a rejected adapter leaves explicit observed identity and mismatch text for diagnostics.
pub fn populate_and_validate_studio_gpu_telemetry(
    observed: &StudioGpuAdapterSnapshot,
    telemetry: &mut StudioPerformanceTelemetry,
) -> Result<(), StudioGpuAdapterPolicyViolation> {
    telemetry.gpu_name = Some(observed.name.clone());
    telemetry.gpu_backend = Some(observed.backend.clone());
    telemetry.gpu_vendor_id = Some(observed.vendor_id);
    telemetry.gpu_device_id = Some(observed.device_id);
    telemetry.gpu_device_type = Some(observed.device_type.clone());

    match validate_studio_gpu_adapter(observed) {
        Ok(()) => {
            telemetry.gpu_adapter_policy_status =
                format!("{STUDIO_GPU_POLICY_SATISFIED_PREFIX}: {}", observed.backend);
            Ok(())
        }
        Err(violation) => {
            telemetry.gpu_adapter_policy_status = format!("mismatch: {violation}");
            Err(violation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_context_settings_lines;

    fn exact_adapter() -> StudioGpuAdapterSnapshot {
        StudioGpuAdapterSnapshot::new(
            REQUIRED_STUDIO_GPU_ADAPTER_NAME,
            REQUIRED_STUDIO_GPU_VENDOR_ID,
            0x2860,
            REQUIRED_STUDIO_GPU_DEVICE_TYPE,
            "Vulkan",
        )
    }
}
