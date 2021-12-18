#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::BootInfo;

use crate::gfx::{Color, ColorData, Screen};
use crate::idt::setup_idt;
use crate::kbrd::{Keyboard, scan2ascii};
use crate::misc::halt;

mod gdt;
mod gfx;
mod misc;
mod idt;
mod kbrd;

static mut SCREEN: Screen = Screen::init();
static mut KEYBOARD: Keyboard = Keyboard::init_default();

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) {
    let _idt = setup_idt();

    let color = ColorData { front_color: Color::BrightWhite, back_color: Color::Black };
    let text = b"SnakeOS";
    unsafe {
        SCREEN.print_str(text, &color);
    }

    loop {}
}

#[no_mangle]
pub unsafe extern fn kbrd_handler(scancode: u8) {
    if let Some(ascii) = scan2ascii(scancode) {
        let text: [u8; 1] = [ascii];
        let color = ColorData { front_color: Color::BrightWhite, back_color: Color::Black };
        SCREEN.print_str(&text, &color);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    let color = ColorData { front_color: Color::Magenta, back_color: Color::Black };
    let text = b"PANIK!";
    unsafe {
        SCREEN.print_str(text, &color);
    }
    halt();
    loop {}
}
