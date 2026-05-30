use core::ffi::{CStr, c_char};

use alloc::{
    ffi::CString,
    string::{String, ToString},
    vec,
};
use bitflags::bitflags;

use crate::syscall::{IPC_CREATE, IPC_MANAGE, IPC_READ, IPC_WRITE, syscall2, syscall3};

bitflags! {
    #[derive(Debug)]
    pub struct Permissions: u32 {
        const READ   = 1 << 0;
        const WRITE  = 1 << 1;
        const MANAGE = 1 << 2;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_port(name_ptr: *const u8, default_permissions: u32) -> isize {
    if name_ptr.is_null() {
        return -1;
    }

    return unsafe { syscall2(IPC_CREATE, name_ptr as isize, default_permissions as isize) };
}

pub fn create_port_rust(name: String, default_permissions: Permissions) -> bool {
    let name_cstring_opt = CString::new(name);
    if let Ok(name_cstring) = name_cstring_opt {
        match create_port(
            name_cstring.as_ptr() as *const u8,
            default_permissions.bits(),
        ) {
            -1 => false,
            _ => true,
        }
    } else {
        return false;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn read_port(name_ptr: *const u8, output_ptr: *mut u8, from: i64) -> isize {
    if name_ptr.is_null() || output_ptr.is_null() {
        return -1;
    }

    return unsafe {
        syscall3(
            IPC_READ,
            name_ptr as isize,
            output_ptr as isize,
            from as isize,
        )
    };
}

pub fn read_port_rust(name: String, from: i64) -> Option<(u64, String)> {
    let name_cstring = CString::new(name).ok()?;

    let mut buf = vec![0u8; 256];
    let sender = read_port(name_cstring.as_ptr() as *const u8, buf.as_mut_ptr(), from);

    if sender <= 0 {
        return None;
    }

    let cstr = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) };
    let msg = cstr.to_str().ok()?.to_string();
    Some((sender as u64, msg))
}

#[unsafe(no_mangle)]
pub extern "C" fn write_port(name_ptr: *const u8, message: *const u8) -> isize {
    if name_ptr.is_null() || message.is_null() {
        return -1;
    }

    return unsafe { syscall2(IPC_WRITE, name_ptr as isize, message as isize) };
}

pub fn write_port_rust(name: String, message: String) -> bool {
    let name_cstring_opt = CString::new(name);
    if let Ok(name_cstring) = name_cstring_opt {
        let message_cstring_opt = CString::new(message);
        if let Ok(message_cstring) = message_cstring_opt {
            match write_port(
                name_cstring.as_ptr() as *const u8,
                message_cstring.as_ptr() as *const u8,
            ) {
                -1 => false,
                _ => true,
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn manage_port(name_ptr: *const u8, pid_to_set: u64, new_permissions: u32) -> isize {
    if name_ptr.is_null() {
        return -1;
    }

    return unsafe {
        syscall3(
            IPC_MANAGE,
            name_ptr as isize,
            pid_to_set as isize,
            new_permissions as isize,
        )
    };
}

pub fn manage_port_rust(name: String, pid_to_set: u64, new_permissions: Permissions) -> bool {
    let name_cstring_opt = CString::new(name);
    if let Ok(name_cstring) = name_cstring_opt {
        match manage_port(
            name_cstring.as_ptr() as *const u8,
            pid_to_set,
            new_permissions.bits(),
        ) {
            -1 => false,
            _ => true,
        }
    } else {
        return false;
    }
}
