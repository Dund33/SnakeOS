#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(array_methods)]
#![feature(panic_info_message)]

use core::panic::PanicInfo;

use bootloader::BootInfo;

use crate::gfx::{redraw_window, SCREEN};
use crate::gfx::screen::{ColorData, DEFAULT_COLOR, Screen};
use crate::gfx::screen::Color::{Black, Red};
use crate::gfx::windows::Window;
use crate::idt::setup_idt;
use crate::kbrd::{Key, scan2ascii};
use crate::kbrd::Key::{Control, Letter};
use crate::mem::mem_total;
use crate::misc::halt;

mod gdt;
mod gfx;
mod misc;
mod idt;
mod kbrd;
mod mem;

static mut TIME: u64 = 0;
static HELLO_STRING: &[u8; 11] = b"=|SnakeOS|=";

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) {
    let idt = setup_idt();
    let idt_addr = idt.as_ptr() as u64;
    let mem_size = mem_total(&_boot_info.memory_map);
    let mut window = Window::new(30, 5, 15, 3);
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
    window.screen.print_str(b"First window in SnakeOS",
                            &window.color,
                            false);
    redraw_window(&window);
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
fn panic(info: &PanicInfo) -> ! {
    let color = &ColorData { front_color: Red, back_color: Black };
    unsafe {
        let text = info.message()
            .unwrap()
            .as_str()
            .unwrap_or_else(|| { "Cannot read message string" })
            .as_bytes();
        SCREEN.print_str_nl(text, color, false);

        let line = info.location().unwrap().line() as u64;
        SCREEN.print_num(line, color, false);

        let file = info.location().unwrap().file();
        SCREEN.newline();
        SCREEN.print_str(file.as_bytes(), color, false);
    }
    halt();
    loop {}
}
