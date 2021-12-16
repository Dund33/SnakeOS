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

    let mut screen = Screen{pos: 0};
    let text: [u8; 13]= [
        72 ,101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33
    ];
    screen.print_str(&text);
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}