#![allow(unused_variables, unused_unsafe, unused_mut)]

pub const READ: usize = 0;
pub const WRITE: usize = 1;
pub const OPEN: usize = 2;
pub const CLOSE: usize = 3;
pub const STAT: usize = 4;
pub const LSEEK: usize = 8;
pub const MMAP: usize = 9;
pub const MUNMAP: usize = 11;
pub const BRK: usize = 12;
pub const GETPID: usize = 39;
pub const FORK: usize = 57;
pub const EXECVE: usize = 59;
pub const EXIT: usize = 60;
pub const WAIT4: usize = 61;
pub const KILL: usize = 62;
pub const CHDIR: usize = 80;
pub const MKDIR: usize = 83;
pub const UNLINK: usize = 87;
pub const GETDENTS64: usize = 217;
pub const CLOCK_GETTIME: usize = 228;
pub const EXIT_GROUP: usize = 231;
pub const INPUT_READ: usize = 666;
pub const SLEEP: usize = 909090; // zzz haha
pub const IPC_CREATE: usize = 500;
pub const IPC_READ: usize = 501;
pub const IPC_WRITE: usize = 502;
pub const IPC_MANAGE: usize = 503;
pub const SHM_OPEN: usize = 600;
pub const MAP_FRAMEBUFFER: usize = 5555;
pub const FRAMEBUFFER_SWAP: usize = 6666;

#[inline(always)]
pub unsafe fn syscall0(num: usize) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            lateout("x0") ret,
            options(nostack)
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall1(num: usize, arg0: isize) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg0,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") arg0 => ret,
            options(nostack)
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall2(num: usize, arg0: isize, arg1: isize) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") arg0 => ret,
            in("x1") arg1,
            options(nostack)
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall3(num: usize, arg0: isize, arg1: isize, arg2: isize) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") arg0 => ret,
            in("x1") arg1,
            in("x2") arg2,
            options(nostack)
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall4(num: usize, arg0: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") arg0 => ret,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            options(nostack)
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall5(
    num: usize,
    arg0: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
) -> isize {
    let mut ret: isize = 0;
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            in("r8") arg4,
            clobber_abi("sysv64"),
            options(nostack)
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") num,
            inlateout("x0") arg0 => ret,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            in("x4") arg4,
            options(nostack)
        );
    }

    ret
}
