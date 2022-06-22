use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASKINFO: usize = 410;


fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

#[derive(Copy, Clone)]
pub struct SyscallInfo {
	pub id: usize,
	pub times: usize,
}

impl SyscallInfo {
	pub fn new() -> Self{
		SyscallInfo {
			id: 0,
			times: 0,
		}
	}
}

pub const MAX_SYSCALL_NUM: usize = 100;
use super::taskinfo::TaskInfo;
pub fn sys_get_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    syscall(SYSCALL_TASKINFO, [id as usize, ts as usize, 0])
}