#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![crate_type = "staticlib"]
#![feature(asm_const)]
#![feature(const_mut_refs)]
use core::panic::PanicInfo;

use crate::gfx::screen::{Screen, DEFAULT_COLOR};
use crate::gfx::Color::{Black, Red};
use crate::gfx::Console;
use crate::gfx::{redraw_window, ColorData, TextInterface, SCREEN};
use crate::idt::setup_idt;
use crate::misc::{cli, halt, num_to_ascii, sti};
use volatile::access::ReadOnly;
use volatile::access::ReadWrite;
use volatile::Volatile;
use crate::tasks::init_tasks;

mod gdt;
mod gfx;
mod idt;
mod kbrd;
mod misc;
mod tasks;

#[no_mangle]
static mut TIME: u64 = 0;
static HELLO_STRING: &[u8; 11] = b"=|SnakeOS|=";
static mut VOLATILE_TIME: Volatile<&'static mut u64, ReadWrite> = unsafe{
    Volatile::new(&mut TIME)
};

#[no_mangle]
pub unsafe extern "C" fn _kernel() {
    init_tasks();
    SCREEN.print_strln(HELLO_STRING, Some(DEFAULT_COLOR));
    SCREEN.print_str(b"IDT@", None);
    let idt = setup_idt();
    cli();
    let idt_addr = idt.as_ptr() as u32;
    let idt_addr_str = num_to_ascii(idt_addr as u64);
    SCREEN.print_strln(&idt_addr_str, Some(DEFAULT_COLOR));
    SCREEN.print_strln(b"kernel running", Some(DEFAULT_COLOR));
    SCREEN.enable_console_mode();
    SCREEN.print_strln(b"kernel done", Some(DEFAULT_COLOR));
    sti();
    halt();
}

#[no_mangle]
pub unsafe extern "C" fn test() {
    loop {
        SCREEN.print_str_at(b"HELLO", 1, 1, Some(DEFAULT_COLOR));
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

pub unsafe fn tick(){
    let res = VOLATILE_TIME.read();
    VOLATILE_TIME.write(res+1);
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

        let line = info.location().unwrap().line() as u64;
        let line_text = num_to_ascii(line);
        SCREEN.print_strln(&line_text, Some(color));

        let file = info.location().unwrap().file();
        SCREEN.newline();
        SCREEN.print_str(file.as_bytes(), Some(color));
    }
    halt();
    loop {}
}
