pub const CLOCK_FREQ: usize = 12500000;

pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x2000)];

pub type BlockDeviceImpl = crate::drivers::block::VirtIOBlock;

pub type GpuImpl = crate::drivers::gpu::VirtioGpu;
