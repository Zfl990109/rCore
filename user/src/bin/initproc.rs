#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exec, fork, wait, yield_};

#[no_mangle]
fn main() -> i32 {
    // fork() 返回值为0，表示子进程，直接调用 exec 执行 user_shell
    if fork() == 0 {
        exec("user_shell\0");
    } else {
        // initproc 进程，等待它的子进程并回收资源，如果没有子进程结束，则让出 CPU
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!(
                "[initproc] Released a zombie process, pid={}, exit_code={}",
                pid, exit_code,
            );
        }
    }
    0
}
