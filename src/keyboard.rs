use crate::syscall::{KBD_READ, syscall2};

pub const ESCAPE: u16 = 0;
pub const F1: u16 = 1;
pub const F2: u16 = 2;
pub const F3: u16 = 3;
pub const F4: u16 = 4;
pub const F5: u16 = 5;
pub const F6: u16 = 6;
pub const F7: u16 = 7;
pub const F8: u16 = 8;
pub const F9: u16 = 9;
pub const F10: u16 = 10;
pub const F11: u16 = 11;
pub const F12: u16 = 12;
pub const PRINT_SCREEN: u16 = 13;
pub const SYS_RQ: u16 = 14;
pub const SCROLL_LOCK: u16 = 15;
pub const PAUSE_BREAK: u16 = 16;
pub const OEM8: u16 = 17;
pub const KEY1: u16 = 18;
pub const KEY2: u16 = 19;
pub const KEY3: u16 = 20;
pub const KEY4: u16 = 21;
pub const KEY5: u16 = 22;
pub const KEY6: u16 = 23;
pub const KEY7: u16 = 24;
pub const KEY8: u16 = 25;
pub const KEY9: u16 = 26;
pub const KEY0: u16 = 27;
pub const OEM_MINUS: u16 = 28;
pub const OEM_PLUS: u16 = 29;
pub const BACKSPACE: u16 = 30;
pub const INSERT: u16 = 31;
pub const HOME: u16 = 32;
pub const PAGE_UP: u16 = 33;
pub const NUMPAD_LOCK: u16 = 34;
pub const NUMPAD_DIVIDE: u16 = 35;
pub const NUMPAD_MULTIPLY: u16 = 36;
pub const NUMPAD_SUBTRACT: u16 = 37;
pub const TAB: u16 = 38;
pub const Q: u16 = 39;
pub const W: u16 = 40;
pub const E: u16 = 41;
pub const R: u16 = 42;
pub const T: u16 = 43;
pub const Y: u16 = 44;
pub const U: u16 = 45;
pub const I: u16 = 46;
pub const O: u16 = 47;
pub const P: u16 = 48;
pub const OEM4: u16 = 49;
pub const OEM6: u16 = 50;
pub const OEM5: u16 = 51;
pub const OEM7: u16 = 52;
pub const DELETE: u16 = 53;
pub const END: u16 = 54;
pub const PAGE_DOWN: u16 = 55;
pub const NUMPAD7: u16 = 56;
pub const NUMPAD8: u16 = 57;
pub const NUMPAD9: u16 = 58;
pub const NUMPAD_ADD: u16 = 59;
pub const CAPS_LOCK: u16 = 60;
pub const A: u16 = 61;
pub const S: u16 = 62;
pub const D: u16 = 63;
pub const F: u16 = 64;
pub const G: u16 = 65;
pub const H: u16 = 66;
pub const J: u16 = 67;
pub const K: u16 = 68;
pub const L: u16 = 69;
pub const OEM1: u16 = 70;
pub const OEM3: u16 = 71;
pub const RETURN: u16 = 72;
pub const NUMPAD4: u16 = 73;
pub const NUMPAD5: u16 = 74;
pub const NUMPAD6: u16 = 75;
pub const L_SHIFT: u16 = 76;
pub const Z: u16 = 77;
pub const X: u16 = 78;
pub const C: u16 = 79;
pub const V: u16 = 80;
pub const B: u16 = 81;
pub const N: u16 = 82;
pub const M: u16 = 83;
pub const OEM_COMMA: u16 = 84;
pub const OEM_PERIOD: u16 = 85;
pub const OEM2: u16 = 86;
pub const R_SHIFT: u16 = 87;
pub const ARROW_UP: u16 = 88;
pub const NUMPAD1: u16 = 89;
pub const NUMPAD2: u16 = 90;
pub const NUMPAD3: u16 = 91;
pub const NUMPAD_ENTER: u16 = 92;
pub const L_CONTROL: u16 = 93;
pub const L_WIN: u16 = 94;
pub const L_ALT: u16 = 95;
pub const SPACEBAR: u16 = 96;
pub const R_ALT_GR: u16 = 97;
pub const R_WIN: u16 = 98;
pub const APPS: u16 = 99;
pub const R_CONTROL: u16 = 100;
pub const ARROW_LEFT: u16 = 101;
pub const ARROW_DOWN: u16 = 102;
pub const ARROW_RIGHT: u16 = 103;
pub const NUMPAD0: u16 = 104;
pub const NUMPAD_PERIOD: u16 = 105;
pub const OEM9: u16 = 106;
pub const OEM10: u16 = 107;
pub const OEM11: u16 = 108;
pub const OEM12: u16 = 109;
pub const OEM13: u16 = 110;
pub const PREV_TRACK: u16 = 111;
pub const NEXT_TRACK: u16 = 112;
pub const MUTE: u16 = 113;
pub const CALCULATOR: u16 = 114;
pub const PLAY: u16 = 115;
pub const STOP: u16 = 116;
pub const VOLUME_DOWN: u16 = 117;
pub const VOLUME_UP: u16 = 118;
pub const WWW_HOME: u16 = 119;
pub const POWER_ON_TEST_OK: u16 = 120;
pub const TOO_MANY_KEYS: u16 = 121;
pub const R_CONTROL2: u16 = 122;
pub const R_ALT2: u16 = 123;

#[repr(C)]
#[derive(Clone, Debug, Copy, Default)]
pub struct KeyboardEvent {
    pub state: u8,
    pub _pad1: u8,
    pub key: u16,
    pub mods: u16,
    pub _pad2: u16,
    pub unicode: u32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kbd_read(buf: *mut KeyboardEvent, n: u8) -> i32 {
    let len = unsafe { syscall2(KBD_READ, buf as isize, n as isize) };
    if len < 0 {
        return -1;
    } else {
        return len as i32;
    }
}
