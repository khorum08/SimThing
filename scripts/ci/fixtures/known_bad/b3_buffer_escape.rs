// CI fixture: B3-BUFFER-ESCAPE — public authoritative buffer handle escape.
use wgpu::Buffer;

pub struct LeakProbe;

impl LeakProbe {
    pub fn leak_buffer(&self) -> &Buffer {
        unimplemented!("fixture only")
    }
}
