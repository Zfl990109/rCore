use alloc::sync::Arc;
use crate::sync::UPSafeCell;
use super::File;
use crate::mm::UserBuffer;
use crate::task::suspend_current_and_run_next;


extern crate byteorder;
extern crate alloc;
use byteorder::{BigEndian, ByteOrder};

pub struct Eventfd {
	readable: bool,
	writable: bool,
	inner: Arc<UPSafeCell<EventfdInner>>,
}
struct EventfdInner {
	pub is_sema: bool,
    pub is_nonblock: bool,
	pub val: u64,
}

const EFD_SEMAPHORE: i32 = 1;
const EFD_NONBLOCK: i32 = 2048;
impl Eventfd {
	pub fn new(val: u64, flag: i32) -> Arc<Self> {
		let mut is_sema = true;
        let mut is_nonblock = true;
		if (flag & EFD_SEMAPHORE) == 0 {
			is_sema = false;
		}
        if (flag & EFD_NONBLOCK) == 0 {
			is_nonblock = false;
		}
		Arc::new(Self {
			readable: true,
			writable: true,
			inner: Arc::new(unsafe {
				UPSafeCell::new(EventfdInner {
					is_sema: is_sema,
                    is_nonblock: is_nonblock,
					val: val,
				})
			}),
		})
	}
}

impl File for Eventfd {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        loop {
            if inner.val == 0 {     // 读取失败
                if inner.is_nonblock {
                    return usize::MAX;
                } else {    // 应该阻塞线程
                    suspend_current_and_run_next();
                    continue;
                }
            } else {
                let mut res = 0u64;
                if inner.is_sema {
                    res = 1u64;
                    inner.val -= 1;
                } else {
                    res = inner.val;
                    inner.val = 0;
                }
                for slice in buf.buffers.iter_mut() {
                    assert_eq!(8, slice.len());
                    BigEndian::write_u64(*slice, res);
                }
                return 0;
            }
        }
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.exclusive_access();
        // 先将缓冲区的数据转化成 u64
        for slice in buf.buffers.iter() {
            assert_eq!(8, slice.len());
            if inner.is_sema {
                inner.val += 1;
            } else {
                let val = BigEndian::read_u64(*slice);
                inner.val += val;
            }
        }
        0
    }
}