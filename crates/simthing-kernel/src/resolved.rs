//! Sealed resolved-state GPU column buffers (KERNEL-WRITE-SEAL-0).

use wgpu::Buffer;

/// Sealed resolved-state GPU column buffers — buffer handles are not public fields.
pub struct ResolvedGpuBuffers {
    values: Buffer,
    previous_values: Buffer,
    output_vectors: Buffer,
    previous_output_vectors: Buffer,
}

impl ResolvedGpuBuffers {
    pub fn new(
        values: Buffer,
        previous_values: Buffer,
        output_vectors: Buffer,
        previous_output_vectors: Buffer,
    ) -> Self {
        Self {
            values,
            previous_values,
            output_vectors,
            previous_output_vectors,
        }
    }

    #[doc(hidden)]
    pub fn values(&self) -> &Buffer {
        &self.values
    }

    #[doc(hidden)]
    pub fn previous_values(&self) -> &Buffer {
        &self.previous_values
    }

    #[doc(hidden)]
    pub fn output_vectors(&self) -> &Buffer {
        &self.output_vectors
    }

    #[doc(hidden)]
    pub fn previous_output_vectors(&self) -> &Buffer {
        &self.previous_output_vectors
    }

    #[doc(hidden)]
    pub fn set_values(&mut self, buffer: Buffer) {
        self.values = buffer;
    }

    #[doc(hidden)]
    pub fn set_previous_values(&mut self, buffer: Buffer) {
        self.previous_values = buffer;
    }

    #[doc(hidden)]
    pub fn set_output_vectors(&mut self, buffer: Buffer) {
        self.output_vectors = buffer;
    }

    #[doc(hidden)]
    pub fn set_previous_output_vectors(&mut self, buffer: Buffer) {
        self.previous_output_vectors = buffer;
    }
}
