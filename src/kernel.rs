#![no_std]
#![no_main]

mod gdt;
mod gfx;
mod misc;
mod idt;
mod kbrd;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::alloc;
use core::arch::asm;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use crate::gfx::{Screen, Color, ColorData};
use crate::idt::{IdtEntry, Idtr, setup_idt};
use crate::kbrd::Keyboard;
use crate::misc::halt;

static mut SCREEN: Screen = Screen::init();
static mut KEYBOARD: Keyboard = Keyboard::init_default();

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo){

    let idt = setup_idt();

    let color = ColorData{front_color: Color::BrightWhite, back_color: Color::Black};
    let text = b"SnakeOS";
    unsafe {
        SCREEN.print_str(text, &color);
    }

    loop {}
}

#[no_mangle]
pub unsafe extern fn kbrd_handler(scancode: u8){
   KEYBOARD.last_key = 34;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {

    let color = ColorData{front_color: Color::Magenta, back_color: Color::Black};
    let text = b"PANIK!";
    unsafe {
        SCREEN.print_str(text, &color);
    }
    halt();
    loop{}
}
