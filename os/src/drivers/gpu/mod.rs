mod virtio_gpu;

pub use virtio_gpu::VirtioGpu;

use crate::board::GpuImpl;
use alloc::sync::Arc;
use lazy_static::*;


lazy_static! {
    pub static ref GPU_DEVICE: Arc<GpuImpl> = Arc::new(GpuImpl::new());
}

#[allow(unused)]
pub fn gpu_device_test() {
    let ref mut gpu_device = GPU_DEVICE.clone();
    gpu_device.test();
    loop {};
}
