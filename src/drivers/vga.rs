use super::util;

static SCREEN_LOC: u32 = 0xB8000;

pub static X_WIDTH: u32 = 80;
pub static Y_HEIGHT: u32 = 25;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Color {
    Black       = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White
}

impl Color {
    fn encoded(fg: Color, bg: Color) -> u16 {
        fg as u16 | (bg as u16) << 4
    }
    
    fn encoded_char(c: u8, fg: Color, bg: Color) -> u16 {
        c as u16 | Color::encoded(fg, bg) << 8
    }
}

pub fn init(bg: Color) {
    let out_char = Color::encoded_char(' ' as u8, Color::White, bg);
    
    for y in 0..Y_HEIGHT {
        for x in 0..X_WIDTH {
            let coord = y * X_WIDTH + x;
            unsafe {
                *((SCREEN_LOC + coord * 2) as *mut u16) = out_char;
            }            
        }
    }
}

pub fn putchar(c: u8, x: u8, y: u8, fg: Color, bg: Color) {
    if x > 80 || y > 25 {
        return;
    }
    let coord = y as u32 * X_WIDTH + x as u32;
    unsafe {
        *((SCREEN_LOC + coord * 2) as *mut u16) = Color::encoded_char(c as u8, fg, bg);
    }
}

pub fn set_cursor(x: u8, y: u8) {
    let coord = y as u32 * X_WIDTH + x as u32;
    unsafe {
        util::outb(0x3D4, 14);
        util::outb(0x3D5, (coord >> 8) as u8);
        util::outb(0x3D4, 15);
        util::outb(0x3D5, coord as u8);
    }
}

