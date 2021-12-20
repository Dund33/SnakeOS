#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::BootInfo;

use crate::gfx::{Color, DEFAULT_COLOR, Screen};
use crate::idt::setup_idt;
use crate::kbrd::{Key, scan2ascii};
use crate::kbrd::Key::{Control, Letter};
use crate::misc::halt;
use crate::mem::mem_total;

mod gdt;
mod gfx;
mod misc;
mod idt;
mod kbrd;
mod mem;

static mut SCREEN: Screen = Screen::init();
static mut TIME: u64 = 0;
static HELLO_STRING: &[u8; 11] = b"=|SnakeOS|=";

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) {
    let idt = setup_idt();
    let idt_addr = idt.as_ptr() as u64;
    let mem_size = mem_total(&_boot_info.memory_map);
    unsafe {
        SCREEN.print_str_nl(HELLO_STRING, &DEFAULT_COLOR, false);
        SCREEN.print_str(b"idt@", &DEFAULT_COLOR, false);
        SCREEN.print_num(idt_addr, &DEFAULT_COLOR, false);
        SCREEN.newline();
        SCREEN.print_str(b"memsize=", &DEFAULT_COLOR, false);
        SCREEN.print_num(mem_size, &DEFAULT_COLOR, false);
        SCREEN.newline();
        SCREEN.sync_cursor();
    }

    halt();
}

#[no_mangle]
pub unsafe extern fn kbrd_handler(scancode: u8) {
    match scan2ascii(scancode) {
        Letter(ascii) => {
            let text: [u8; 1] = [ascii];
            SCREEN.print_str(&text, &DEFAULT_COLOR, true);
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
        SCREEN.print_num_at((TIME / 20) as u64,
                            &DEFAULT_COLOR,
                            75,
                            0,
                            false);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    let text = b"___PANIK!___";
    unsafe {
        SCREEN.print_str(text, &DEFAULT_COLOR, false);
    }
    halt();
    loop {}
}
