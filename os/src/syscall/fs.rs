//! File and filesystem-related syscalls
use crate::fs::{open_file, OpenFlags, linkat, unlinkat};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer, translated_refmut};
use crate::task::{current_task, current_user_token};
use crate::fs::stat::*;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_linkat(oldpath: *const u8, newpath: *const u8, _flags: u32) -> isize {
    let token = current_user_token();
    let koldpath = translated_str(token, oldpath);
    let knewpath = translated_str(token, newpath);
    if koldpath == knewpath {
        return -1;
    }
    linkat(koldpath.as_str(), knewpath.as_str());
    0
}

pub fn sys_unlinkat(_dirfd: usize, path: *const u8, _flags: u32) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    unlinkat(path.as_str())
}

pub fn sys_stat(fd: usize, st: *mut Stat) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    if let Some(inode) = &inner.fd_table[fd] {
        let stat = translated_refmut(token, st) as *mut Stat;
        let (ino, mode, nlink) = inode.fstat();
        unsafe {
            (*stat).dev = 0;
            (*stat).ino = ino;
            (*stat).mode = mode;
            (*stat).nlink = nlink;
            // (*stat).pad = [0 as u64; 7];
        }
        0
    } else {
        -1
    }

}
