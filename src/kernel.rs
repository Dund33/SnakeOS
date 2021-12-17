#![no_std]
#![no_main]

mod gdt;
mod gfx;
mod misc;

use core::panic::PanicInfo;
use core::alloc;
use core::arch::asm;
use crate::gfx::Screen;
use crate::misc::halt;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(){
    let mut screen = Screen::init();
    let text = b"Womnsze to peuzajom one";
    screen.print_str(text);
    halt()
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}