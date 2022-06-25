
use crate::sync::UPSafeCell;
use virtio_drivers::{VirtIOGpu, VirtIOHeader};

#[allow(unused)]
const VIRTIO0: usize = 0x10002000;

pub struct VirtioGpu(UPSafeCell<VirtIOGpu<'static>>);


impl VirtioGpu {
    #[allow(unused)]
    pub fn new() -> Self {
        let mut virt_head_ptr = VIRTIO0 as *mut VirtIOHeader;
        unsafe {
            // virt_head_ptr = virt_head_ptr.add(1);
            Self(UPSafeCell::new(
                VirtIOGpu::new(&mut *(virt_head_ptr)).unwrap(),
            ))
        }
    }

    pub fn test(&self) {
        let mut inner = self.0.exclusive_access();
        let fb = inner.setup_framebuffer().unwrap();
        for y in 0..768 {
            for x in 0..1024 {
                let idx = (y * 1024 + x) * 4;
                fb[idx] = x as u8;
                fb[idx + 1] = y as u8;
                fb[idx + 2] = (x + y) as u8;
            }
        }
        drop(inner);
        println!("virtio-gpu test finished");
    }


}

impl Default for VirtioGpu {
    fn default() -> Self {
        Self::new()
    }
}

