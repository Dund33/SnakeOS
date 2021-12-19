#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::BootInfo;

use crate::gfx::{Color, DEFAULT_COLOR, Screen};
use crate::idt::setup_idt;
use crate::kbrd::{Key, scan2ascii};
use crate::kbrd::Key::{Control, Letter};
use crate::misc::halt;

mod gdt;
mod gfx;
mod misc;
mod idt;
mod kbrd;

static mut SCREEN: Screen = Screen::init();
static mut TIME: u64 = 0;

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) {
    let _idt = setup_idt();

    let text = b"SnakeOS";
    unsafe {
        SCREEN.print_str_nl(text, &DEFAULT_COLOR);
    }
    loop {}
}

#[no_mangle]
pub unsafe extern fn kbrd_handler(scancode: u8) {
    match scan2ascii(scancode) {
        Letter(ascii) => {
            let text: [u8; 1] = [ascii];
            SCREEN.print_str(&text, &DEFAULT_COLOR);
        }

        Control(code) => {
            SCREEN.control(code);
        }

        Key::None => {}
    };
}

#[no_mangle]
pub unsafe extern fn pit_handler() {
    TIME += 1;
    if TIME % 20 == 0 {
        SCREEN.print_num_at((TIME / 20) as u64, &DEFAULT_COLOR, 75, 0);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    let text = b"PANIK!";
    unsafe {
        SCREEN.print_str(text, &DEFAULT_COLOR);
    }
    halt();
    loop {}
}
