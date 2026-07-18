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
pub const STUDIO_GPU_POLICY_SATISFIED_PREFIX: &str =
    "satisfied: exact NVIDIA GeForce RTX 4080 Laptop GPU / NVIDIA / DiscreteGpu / backend";

/// Bevy 0.16 automatic adapter requests leave `force_fallback_adapter` at its `false` default.
/// The exact post-init validator below then rejects every non-required adapter rather than falling
/// back or silently honoring an environment-selected downgrade.
pub const STUDIO_GPU_FORCE_FALLBACK_ADAPTER: bool = false;

/// Renderer settings that allow Bevy-supported backends while preferring the discrete adapter.
///
/// `VALIDATION` stays enabled. `DEBUG` is intentionally not requested because optional backend debug
/// layers are host-dependent and must not turn a missing developer component into a startup warning.
pub fn required_studio_wgpu_settings() -> WgpuSettings {
    let mut settings = WgpuSettings::default();
    settings.backends = Some(Backends::all());
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
             device_type={}; observed {}",
            self.mismatches.join(", "),
            REQUIRED_STUDIO_GPU_ADAPTER_NAME,
            REQUIRED_STUDIO_GPU_VENDOR_ID,
            REQUIRED_STUDIO_GPU_DEVICE_TYPE,
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
            "Dx12",
        )
    }

    #[test]
    fn exact_adapter_policy_accepts_supported_backends_and_rejects_identity_mismatches() {
        assert_eq!(validate_studio_gpu_adapter(&exact_adapter()), Ok(()));

        let vulkan_adapter = StudioGpuAdapterSnapshot::new(
            REQUIRED_STUDIO_GPU_ADAPTER_NAME,
            REQUIRED_STUDIO_GPU_VENDOR_ID,
            0x2860,
            REQUIRED_STUDIO_GPU_DEVICE_TYPE,
            "Vulkan",
        );
        assert_eq!(validate_studio_gpu_adapter(&vulkan_adapter), Ok(()));

        let rejected = [
            StudioGpuAdapterSnapshot::new(
                "Intel(R) Iris(R) Xe Graphics",
                0x8086,
                0x0001,
                "IntegratedGpu",
                "Dx12",
            ),
            StudioGpuAdapterSnapshot::new(
                "Microsoft Basic Render Driver",
                0x1414,
                0x008c,
                "Cpu",
                "Dx12",
            ),
            StudioGpuAdapterSnapshot::new(
                "NVIDIA GeForce RTX 4090 Laptop GPU",
                REQUIRED_STUDIO_GPU_VENDOR_ID,
                0x2718,
                REQUIRED_STUDIO_GPU_DEVICE_TYPE,
                "Vulkan",
            ),
        ];
        for adapter in rejected {
            let violation = validate_studio_gpu_adapter(&adapter)
                .expect_err("every non-exact adapter identity must fail closed");
            assert_eq!(violation.observed(), &adapter);
            assert!(violation.to_string().contains("observed"));
        }
    }

    #[test]
    fn renderer_settings_allow_all_backends_and_remain_high_performance_validated_nonfallback() {
        let settings = required_studio_wgpu_settings();
        assert_eq!(settings.backends, Some(Backends::all()));
        assert_eq!(settings.power_preference, PowerPreference::HighPerformance);
        assert_eq!(settings.instance_flags, InstanceFlags::VALIDATION);
        assert!(!STUDIO_GPU_FORCE_FALLBACK_ADAPTER);
    }

    #[test]
    fn supplied_adapter_populates_every_identity_field_and_policy_row() {
        let adapter = exact_adapter();
        let mut telemetry = StudioPerformanceTelemetry::default();
        populate_and_validate_studio_gpu_telemetry(&adapter, &mut telemetry)
            .expect("exact adapter must satisfy policy");

        assert_eq!(telemetry.gpu_name.as_deref(), Some(adapter.name.as_str()));
        assert_eq!(telemetry.gpu_vendor_id, Some(adapter.vendor_id));
        assert_eq!(telemetry.gpu_device_id, Some(adapter.device_id));
        assert_eq!(
            telemetry.gpu_device_type.as_deref(),
            Some(REQUIRED_STUDIO_GPU_DEVICE_TYPE)
        );
        assert_eq!(
            telemetry.gpu_backend.as_deref(),
            Some(adapter.backend.as_str())
        );
        assert_eq!(
            telemetry.gpu_adapter_policy_status,
            format!("{STUDIO_GPU_POLICY_SATISFIED_PREFIX}: {}", adapter.backend)
        );
        assert!(gpu_context_settings_lines(&telemetry)
            .iter()
            .take(5)
            .all(|line| !line.contains("unavailable")));
    }
}
