#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{get_task_info, taskinfo::TaskInfo};

#[no_mangle]
fn main() -> i32 {
    let mut ts = TaskInfo::new();
    get_task_info(3, &mut ts as *mut TaskInfo);
    ts.print_task_info();

    0
}