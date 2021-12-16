#![no_std]
#![no_main]

mod gdt;
mod gfx;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use core::alloc;
use crate::gfx::Screen;

entry_point!(_krnl);


#[no_mangle]
fn _krnl(info: &'static mut BootInfo) -> !{
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    let mut screen = Screen{pos: 0};
    let text = "Hello, world!".to_ascii_uppercase();
    screen.print_str();
    loop {}
}