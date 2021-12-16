#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
mod gdt;
entry_point!(_krnl);


#[no_mangle]
fn _krnl(info: &'static mut BootInfo) -> !{
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}