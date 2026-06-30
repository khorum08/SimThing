// CI fixture: ALLOW-BUFFER-HANDLES — unsanctioned public buffer handle.
use wgpu::Buffer;

pub struct BufferLeak;

impl BufferLeak {
    pub fn leak(&self) -> &Buffer {
        unimplemented!("fixture only")
    }
}
