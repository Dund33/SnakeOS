#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![crate_type = "staticlib"]

use core::arch::asm;
use core::panic::PanicInfo;

use crate::gfx::screen::{Screen, DEFAULT_COLOR};
use crate::gfx::windows::Window;
use crate::gfx::Color::{Black, Red};
use crate::gfx::{redraw_window, ColorData, TextInterface, SCREEN};
use crate::idt::setup_idt;
use crate::kbrd::Key::{Control, Letter};
use crate::kbrd::{scan2ascii, Key};
use crate::misc::{halt, num_to_ascii};

mod gdt;
mod gfx;
mod idt;
mod kbrd;
mod misc;
mod tasks;

static mut TIME: u64 = 0;
static HELLO_STRING: &[u8; 11] = b"=|SnakeOS|=";
static mut CURRENT_TASK: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn _kernel() {

    SCREEN.print_strln(HELLO_STRING, Some(DEFAULT_COLOR));
    SCREEN.print_str(b"IDT@", None);
    let idt = setup_idt();
    let idt_addr = idt.as_ptr() as u32;
    let idt_addr_str = num_to_ascii(idt_addr as u64);

    //let mut window1 = Window::new(30, 5, 15, 3);
    //let mut window2 = Window::new(45, 15, 15, 3);
    SCREEN.print_strln(&idt_addr_str, None);
    //window1.screen.print_str(b"First window in SnakeOS", None);
    //window2
    //    .screen
    //    .print_str(b"Multiple windows also work!", None);
    //redraw_window(&window1);
    //redraw_window(&window2);
    //SCREEN.sync_cursor();
    halt();
}

#[no_mangle]
pub unsafe extern "C" fn kbrd_handler() {
    let mut scancode: u8;
    asm!("in {}, 0x60", out(reg_byte) scancode);
    match scan2ascii(scancode as u8) {
        Letter(ascii) => {
            let text: [u8; 1] = [ascii];
            //SCREEN.print_str(&text, None);
        }

        Control(code) => {
            //SCREEN.control(code);
        }

        Key::None => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn test(){
    loop{
        SCREEN.print_str_at(b"HELLO", 1,1,Some(DEFAULT_COLOR));
    }
}

#[no_mangle]
pub unsafe extern "C" fn test2(){
    loop{
        SCREEN.print_str_at(b"KENOBI",10,1, None);
    }
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
