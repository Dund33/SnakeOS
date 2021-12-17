#![no_std]
#![no_main]

mod gdt;
mod gfx;
mod misc;

use core::panic::PanicInfo;
use core::alloc;
use core::arch::asm;
use crate::gfx::{Screen, Color, ColorData};
use crate::misc::halt;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(){
    let mut screen = Screen::init();
    let color = ColorData{front_color: Color::Magenta, back_color: Color::White};
    let text = b"Womnsze to peuzajom one";
    screen.print_str(text, &color);
    screen.print_str(text, &color);
    halt()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
