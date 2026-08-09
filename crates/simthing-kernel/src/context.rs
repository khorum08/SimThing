//! GPU device/queue lifecycle. One `GpuContext` per session.

use thiserror::Error;
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, DeviceType, Features, Instance,
    InstanceDescriptor, MemoryHints, PowerPreference, Queue, RequestAdapterOptions,
};

#[derive(Debug, Error)]
pub enum GpuInitError {
    #[error("no suitable GPU adapter found")]
    NoAdapter,
    #[error("device request failed: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}

pub struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    timestamp_supported: bool,
    encoder_timestamp_supported: bool,
    timestamp_period_ns: f32,
}

impl GpuContext {
    /// Blocking init — for tests and one-shot setup. Uses pollster.
    pub fn new_blocking() -> Result<Self, GpuInitError> {
        pollster::block_on(Self::new())
    }

    pub async fn new() -> Result<Self, GpuInitError> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        // ALWAYS use a discrete GPU when one is present. Do not silently fall back to the
        // integrated adapter: `PowerPreference::default()` (the old behavior) selected the iGPU on
        // dual-GPU machines, so every GPU parity test ran on integrated graphics and never on the
        // discrete/target device — undermining cross-adapter determinism (I8) and real-target
        // fidelity. Enumerate first and prefer the first `DiscreteGpu`; only if none exists fall
        // back to a high-performance adapter request (integrated-only or headless hosts).
        let adapter = match instance
            .enumerate_adapters(Backends::PRIMARY)
            .into_iter()
            .find(|a| a.get_info().device_type == DeviceType::DiscreteGpu)
        {
            Some(discrete) => discrete,
            None => instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: None,
                })
                .await
                .ok_or(GpuInitError::NoAdapter)?,
        };

        let adapter_features = adapter.features();
        let timestamp_supported = adapter_features.contains(Features::TIMESTAMP_QUERY);
        let encoder_timestamp_supported = adapter_features
            .contains(Features::TIMESTAMP_QUERY | Features::TIMESTAMP_QUERY_INSIDE_ENCODERS);
        let mut required_features = Features::empty();
        if timestamp_supported {
            required_features |= Features::TIMESTAMP_QUERY;
        }
        if encoder_timestamp_supported {
            required_features |= Features::TIMESTAMP_QUERY_INSIDE_ENCODERS;
        }

        let mut limits = adapter.limits();
        // C-8a EvalEML adds two read-only storage bindings (8–9); need >8 total.
        limits.max_storage_buffers_per_shader_stage =
            limits.max_storage_buffers_per_shader_stage.max(10);

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("simthing-gpu device"),
                    required_features,
                    required_limits: limits,
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await?;

        let timestamp_period_ns = queue.get_timestamp_period();

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            timestamp_supported,
            encoder_timestamp_supported,
            timestamp_period_ns,
        })
    }

    pub fn timestamp_supported(&self) -> bool {
        self.timestamp_supported
    }

    pub fn encoder_timestamp_supported(&self) -> bool {
        self.encoder_timestamp_supported
    }

    pub fn timestamp_period_ns(&self) -> f32 {
        self.timestamp_period_ns
    }
}
