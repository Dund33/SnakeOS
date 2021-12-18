#![no_std]
#![no_main]

mod gdt;
mod gfx;
mod misc;
mod idt;

use core::panic::PanicInfo;
use core::alloc;
use core::arch::asm;
use bootloader::BootInfo;
use crate::gfx::{Screen, Color, ColorData};
use crate::idt::{Idtr, setup_idt};
use crate::misc::halt;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo){

    let idt = setup_idt();
    let mut screen = Screen::init();
    let color = ColorData{front_color: Color::Magenta, back_color: Color::Black};
    let text = b"Coobra <3 Womnszownica";
    screen.print_str(text, &color);
    halt()
}

#[no_mangle]
pub extern "C" fn farayad(){
    let mut screen = Screen::init();
    let color = ColorData{front_color: Color::Magenta, back_color: Color::Black};
    let text = b"DOOPA";
    screen.print_str(text, &color);
    halt()
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
