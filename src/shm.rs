use alloc::{ffi::CString, string::String};

use crate::syscall::{SHM_OPEN, syscall2};

pub const USER_SHM_BASE: u64 = 0x0000_4000_0000_0000;
pub const SHM_SLOT_SIZE: u64 = 64 * 1024 * 1024;

#[unsafe(no_mangle)]
pub extern "C" fn shm_open(name_ptr: *const u8, size: u64) -> isize {
    if name_ptr.is_null() {
        return -1;
    }

    return unsafe { syscall2(SHM_OPEN, name_ptr as isize, size as isize) };
}

pub fn shm_open_rust(name: String, size: u64) -> isize {
    let name_cstring_opt = CString::new(name);
    if let Ok(name_cstring) = name_cstring_opt {
        return shm_open(name_cstring.as_ptr() as *const u8, size);
    } else {
        return -1;
    }
}
