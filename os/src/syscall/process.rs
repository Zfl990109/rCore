//! Process management syscalls

use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;
use crate::task::{mmap, munmap};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get current time
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

/// map
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    mmap(start, len, prot)
}

/// unmap
pub fn sys_munmap(start: usize, len: usize) -> isize {
    munmap(start, len)
}