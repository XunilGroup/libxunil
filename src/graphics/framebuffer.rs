use crate::{
    io::window::Window,
    shm::{SHM_SLOT_SIZE, USER_SHM_BASE},
};

pub struct WindowFrameBuffer {
    pub ptr: *mut u32,
    pub width: usize,
    pub height: usize,
}

impl WindowFrameBuffer {
    pub fn from_window(window: &Window) -> Self {
        let ptr = (USER_SHM_BASE + window.shm_id * SHM_SLOT_SIZE) as *mut u32;
        WindowFrameBuffer {
            ptr,
            width: window.width,
            height: window.height,
        }
    }

    #[inline(always)]
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        if idx >= self.width * self.height {
            return;
        }
        unsafe { self.ptr.add(idx).write(color) };
    }

    #[inline(always)]
    pub fn fill_span(&mut self, x: usize, y: usize, len: usize, color: u32) {
        if y >= self.height || x >= self.width || len == 0 {
            return;
        }
        let len = core::cmp::min(len, self.width - x);
        let start = y * self.width + x;
        unsafe {
            let slice = core::slice::from_raw_parts_mut(self.ptr.add(start), len);
            slice.fill(color);
        }
    }

    pub fn clear(&mut self, color: u32) {
        unsafe {
            let slice = core::slice::from_raw_parts_mut(self.ptr, self.width * self.height);
            slice.fill(color);
        }
    }
}
