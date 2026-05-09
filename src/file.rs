use core::ptr::null_mut;

use crate::syscall::{CLOSE, LSEEK, OPEN, READ, WRITE, syscall1, syscall2, syscall3, syscall4};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE {
    pub fd: i64,
}

impl FILE {
    pub const fn zeroed() -> FILE {
        FILE { fd: -1 }
    }
}

pub type Fd = i64;
const MAX_FD: usize = 16;

fn fd_ok(fd: Fd) -> bool {
    fd >= 0 && (fd as usize) < MAX_FD
}

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

static mut STDERR_FILE: FILE = FILE::zeroed();
static mut STDOUT_FILE: FILE = FILE::zeroed();
static mut STDIN_FILE: FILE = FILE::zeroed();

#[unsafe(no_mangle)]
pub static mut stderr: *mut FILE = unsafe { &raw mut STDERR_FILE };
#[unsafe(no_mangle)]
pub static mut stdin: *mut FILE = unsafe { &raw mut STDIN_FILE };
#[unsafe(no_mangle)]
pub static mut stdout: *mut FILE = unsafe { &raw mut STDOUT_FILE };

#[unsafe(no_mangle)]
pub extern "C" fn fopen(path: *const i8, mode: *const i8) -> *mut FILE {
    if path.is_null() || mode.is_null() {
        return null_mut();
    }

    let fd = unsafe { syscall2(OPEN, path as isize, mode as isize) };
    if fd < 0 {
        return null_mut();
    }

    let mut new_file = FILE::zeroed();
    new_file.fd = fd as i64;

    let boxed = alloc::boxed::Box::new(new_file);
    alloc::boxed::Box::into_raw(boxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn fclose(file_ptr: *mut FILE) -> i32 {
    if file_ptr.is_null() || !fd_ok(unsafe { (*file_ptr).fd }) {
        return -1;
    }

    return unsafe { syscall1(CLOSE, (*file_ptr).fd as isize) } as i32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(file_ptr: *mut FILE, fmt: *const u8, args: ...) -> i32 {
    if fmt.is_null() || file_ptr.is_null() || !fd_ok(unsafe { (*file_ptr).fd }) {
        return -1;
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fread(ptr: *mut u8, size: usize, nmemb: usize, fp: *mut FILE) -> usize {
    if size == 0 || nmemb == 0 || ptr.is_null() || fp.is_null() || !fd_ok(unsafe { (*fp).fd }) {
        return 0;
    }

    let ret = unsafe {
        syscall4(
            READ,
            ptr as isize,
            size as isize,
            nmemb as isize,
            (*fp).fd as isize,
        )
    };

    if ret < 0 { 0 } else { ret as usize }
}

#[unsafe(no_mangle)]
pub extern "C" fn fseek(stream: *mut FILE, offset: i64, whence: i32) -> i32 {
    if stream.is_null() || !fd_ok(unsafe { (*stream).fd }) {
        return -1;
    }

    return unsafe {
        syscall3(
            LSEEK,
            (*stream).fd as isize,
            offset as isize,
            whence as isize,
        )
    } as i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn fwrite(ptr: *mut u8, size: usize, count: usize, fp: *mut FILE) -> usize {
    if ptr.is_null() || fp.is_null() || !fd_ok(unsafe { (*fp).fd }) {
        return 0;
    }
    return unsafe {
        syscall4(
            WRITE,
            ptr as isize,
            size as isize,
            count as isize,
            (*fp).fd as isize,
        )
    } as usize;
}

#[unsafe(no_mangle)]
pub extern "C" fn ftell(stream: *mut FILE) -> i64 {
    if stream.is_null() || unsafe { (*stream).fd < 0 || (*stream).fd >= 16 } {
        return -1;
    }
    return unsafe { syscall3(LSEEK, (*stream).fd as isize, 0, SEEK_CUR as isize) } as i64;
}

#[unsafe(no_mangle)]
pub extern "C" fn fflush(file_ptr: *mut FILE) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn mkdir(path: *const u8, mode: *const u8) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn remove(path: *const i8) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn rename(path: *const u8, new_path: *const u8) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf(stream: *const u8, format: *const u8, args: ...) -> i32 {
    0
}
