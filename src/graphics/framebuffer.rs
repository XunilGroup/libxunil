use crate::syscall::{MAP_FRAMEBUFFER, syscall0};

pub const USER_FB_BASE: u64 = 0x0000_7F00_0000_0000;

#[repr(C)]
pub struct UserFrameBuffer {
    pub buf_virt: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

impl UserFrameBuffer {
    pub unsafe fn load_from_ptr(
        &mut self,
        src_ptr: *const u32,
        src_width: usize,
        src_height: usize,
    ) {
        let _buf = unsafe { core::ptr::read_volatile(&self.buf_virt) };
        for dy in 0..self.height {
            let sy = dy * src_height / self.height;

            for dx in 0..self.width {
                let sx = dx * src_width / self.width;

                let src_pixel = unsafe { *src_ptr.add(sy * src_width + sx) };

                unsafe { *self.buf_virt.add(dy * self.pitch + dx) = src_pixel };
            }
        }
    }
}

pub unsafe fn map_framebuffer() {
    unsafe { syscall0(MAP_FRAMEBUFFER) };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn draw_buffer(buffer: *const u32, width: u32, height: u32) -> i32 {
    let fb_ptr = USER_FB_BASE as *mut UserFrameBuffer;
    unsafe { (*fb_ptr).load_from_ptr(buffer, width as usize, height as usize) };

    0
}
