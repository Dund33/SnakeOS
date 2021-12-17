#![no_std]
#![no_main]

mod gdt;
mod gfx;
mod misc;
mod idt;

use core::panic::PanicInfo;
use core::alloc;
use core::arch::asm;
use crate::gfx::{Screen, Color, ColorData};
use crate::idt::setup_idt;
use crate::misc::halt;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(){

    let idt = setup_idt();

    let mut screen = Screen::init();
    let color = ColorData{front_color: Color::Magenta, back_color: Color::White};
    let text = b"Coobra <3 Womnszownica";
    screen.print_str(text, &color);
    halt()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
