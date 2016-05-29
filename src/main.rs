#![no_std]
#![allow(improper_ctypes)]

mod drivers;
use drivers::interrupts;
use drivers::vga;

mod display;

// TODO: Assembly code is unable to call this function if it is
// inside interrupts.rs, hence this workaround. Is exposing
// a function to assembly different if the function is inside
// a module?
#[no_mangle]
pub extern "C" fn keyboard_handler_main() {
    drivers::interrupts::keypress_main();
}

fn print_char(c: u8) {
    display::print_char(c);
}

#[no_mangle]
pub fn main() {
    display::init(vga::Color::Black, vga::Color::LightGreen);
    interrupts::init();
    interrupts::set_keyboard_fn(print_char);
    
    display::print("hello");
}
