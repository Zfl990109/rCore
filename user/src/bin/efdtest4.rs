#![no_std]
#![no_main]

use user_lib::{eventfd, fork, sleep};

#[macro_use]
extern crate user_lib;
extern crate alloc;
extern crate byteorder;
use alloc::vec;
use user_lib::{read, write};
use byteorder::{BigEndian, ByteOrder};
/// 信号量模式，阻塞模式，直接读取数据
#[no_mangle]
pub fn main() -> i32 {
    let efd = eventfd(0, 1);
    let mut u: u64 = 0;
    let ref mut buffer = vec![0u8; 8];
    let pid = fork();
    if pid == 0 {
        sleep(100);
        u = 100;
        BigEndian::write_u64(buffer, u);
        write(efd as usize, buffer);
    } else {
        read(efd as usize, buffer);
        u = BigEndian::read_u64(buffer);
        assert_eq!(1, u);
        println!("efdtest4 ok");
    }
    0
}
