use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    getpid,
    io::{
        ipc::{read_port_rust, write_port_rust},
        time::sleep_ms,
    },
    println,
    shm::{SHM_SLOT_SIZE, USER_SHM_BASE, shm_open_rust},
};

pub static mut PRIV_IPC_NAME: Option<String> = None;

#[repr(C)]
pub struct Window {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub shm_id: u64,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_window(width: usize, height: usize) -> Window {
    write_port_rust("window_manager".to_string(), "request_priv_ipc".to_string());
    let pid = getpid();
    let priv_ipc_name = loop {
        if let Some((sender, msg)) = read_port_rust(format!("wm_priv_{}", getpid()), -1) {
            println!("{}: {}", sender, msg);

            if msg.starts_with("ack_request_priv_ipc") {
                let parts = msg.split_whitespace().collect::<Vec<&str>>();

                println!("{}", parts[1].trim().parse::<isize>().unwrap_or(-1) == pid);

                if parts[1].trim().parse::<isize>().unwrap_or(-1) == pid {
                    break parts[2].to_string();
                }
            }
        }

        unsafe { sleep_ms(1) };
    };

    println!("Private IPC Name: {}", priv_ipc_name);

    unsafe {
        PRIV_IPC_NAME = Some(priv_ipc_name.clone());
    }

    write_port_rust(
        priv_ipc_name.clone(),
        format!("request_window_buf {} {}", width, height),
    );

    let (width, height, x, y, shm_name, shm_id) = loop {
        if let Some((_, msg)) = read_port_rust(priv_ipc_name.clone(), -1) {
            if msg.starts_with("ack_request_window_buf") {
                let parts = msg.split_whitespace().collect::<Vec<&str>>();
                let width = parts[1].parse::<usize>().unwrap_or(0);
                let height = parts[2].parse::<usize>().unwrap_or(0);
                let x = parts[3].parse::<usize>().unwrap_or(0);
                let y = parts[4].parse::<usize>().unwrap_or(0);
                let shm_id = parts[6].parse::<u64>().unwrap_or(0);
                break (width, height, x, y, parts[5].to_string(), shm_id);
            }
        };
        unsafe { sleep_ms(1) };
    };

    shm_open_rust(
        shm_name.clone(),
        (width * height * size_of::<u32>() + 1) as u64,
    );

    Window {
        width,
        height,
        x,
        y,
        shm_id,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn draw_buffer_to_window(
    buffer: *const u32,
    shm_id: u64,
    src_width: usize,
    src_height: usize,
    window_width: usize,
    window_height: usize,
) -> i32 {
    let ptr = (USER_SHM_BASE + shm_id * SHM_SLOT_SIZE) as *mut u32;

    for wy in 0..window_height {
        for wx in 0..window_width {
            let src_x = wx * src_width / window_width;
            let src_y = wy * src_height / window_height;

            let src_pixel = unsafe { *buffer.add(src_y * src_width + src_x) };
            unsafe { *ptr.add(wy * window_width + wx) = src_pixel };
        }
    }

    set_dirty();

    0
}

pub fn set_dirty() {
    write_port_rust(
        #[allow(static_mut_refs)]
        unsafe {
            PRIV_IPC_NAME.as_ref().unwrap().clone()
        },
        "set_dirty".to_string(),
    );
}
