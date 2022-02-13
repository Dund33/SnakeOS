#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![crate_type = "staticlib"]
#![feature(asm_const)]
use core::panic::PanicInfo;

use crate::gfx::screen::{Screen, DEFAULT_COLOR};
use crate::gfx::Color::{Black, Red};
use crate::gfx::Console;
use crate::gfx::{redraw_window, ColorData, TextInterface, SCREEN};
use crate::idt::setup_idt;
use crate::misc::{halt, num_to_ascii};
use volatile::access::ReadOnly;
use volatile::Volatile;

mod gdt;
mod gfx;
mod idt;
mod kbrd;
mod misc;

#[no_mangle]
static TIME: u64 = 0;
static HELLO_STRING: &[u8; 11] = b"=|SnakeOS|=";
static mut VOLATILE_TIME: Volatile<&u64, ReadOnly> = Volatile::new_read_only(&TIME);

#[no_mangle]
pub unsafe extern "C" fn _kernel() {
    SCREEN.print_strln(HELLO_STRING, Some(DEFAULT_COLOR));
    SCREEN.print_str(b"IDT@", None);
    let idt = setup_idt();
    let idt_addr = idt.as_ptr() as u32;
    let idt_addr_str = num_to_ascii(idt_addr as u64);
    SCREEN.print_strln(&idt_addr_str, Some(DEFAULT_COLOR));
    SCREEN.print_strln(b"kernel running", Some(DEFAULT_COLOR));
    SCREEN.enable_console_mode();
    SCREEN.print_strln(b"kernel done", Some(DEFAULT_COLOR));
    halt();
}

#[no_mangle]
pub unsafe extern "C" fn test() {
    loop {
        SCREEN.print_str_at(b"HELLO", 1, 1, Some(DEFAULT_COLOR));
        delay(10);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test2() {
    loop {
        SCREEN.print_str_at(b"KENOBI", 1, 2, Some(DEFAULT_COLOR));
        delay(10);
    }
}

pub unsafe fn delay(period: u64) {
    let time1 = VOLATILE_TIME.read();
    while VOLATILE_TIME.read() < time1 + period {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let color = ColorData {
        front_color: Red,
        back_color: Black,
    };
    unsafe {
        let text = info
            .message()
            .unwrap()
            .as_str()
            .unwrap_or_else(|| "Cannot read message string")
            .as_bytes();
        SCREEN.print_strln(text, Some(color));

        /*let line = info.location().unwrap().line() as u64;
        SCREEN.print_num(line, color, false);

        let file = info.location().unwrap().file();
        SCREEN.newline();
        SCREEN.print_str(file.as_bytes(), Some(color));*/
    }
    halt();
    loop {}
}
