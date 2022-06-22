//! File system in os
mod inode;
mod stdio;
/// stat
pub mod stat;

use crate::mm::UserBuffer;
use stat::StatMode;
/// File trait
pub trait File: Send + Sync {
    /// If readable
    fn readable(&self) -> bool;
    /// If writable
    fn writable(&self) -> bool;
    /// Read file to `UserBuffer`
    fn read(&self, buf: UserBuffer) -> usize;
    /// Write `UserBuffer` to file
    fn write(&self, buf: UserBuffer) -> usize;
    /// fstat
    fn fstat(&self) -> (u64, StatMode, u32);
}

pub use inode::{list_apps, open_file, OSInode, OpenFlags, linkat, unlinkat};
pub use stdio::{Stdin, Stdout};