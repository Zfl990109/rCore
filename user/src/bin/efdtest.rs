#![no_std]
#![no_main]

use user_lib::{eventfd, fork, exit, sleep};

#[macro_use]
extern crate user_lib;
extern crate alloc;
use alloc::vec;
use user_lib::{read, write};
#[no_mangle]
pub fn main() -> i32 {
    let efd = eventfd(0, 0);

    let ref mut buffer = [1u8; 8];
    let pid = fork();
    if pid == 0 {
        write(efd as usize, buffer);
    } else {
        sleep(20);
        println!("{}", read(efd as usize, buffer));
        for i in buffer {
            println!("{}", i);
        }
    }    
    0
}
