use core::borrow::Borrow;

use crate::Color::{Black, BrightWhite};
use crate::misc::num_length;

pub static DEFAULT_COLOR: ColorData = ColorData { front_color: BrightWhite, back_color: Black };
pub static BLACK_COLOR: ColorData = ColorData { front_color: Black, back_color: Black };

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    White = 0x07,
    Gray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    BrightWhite = 0x0F,
}

pub struct ColorData {
    pub front_color: Color,
    pub back_color: Color,
}

pub struct Screen {
    mem: *mut u8,
    pub(crate) pos_x: isize,
    pub(crate) pos_y: isize,
    pub(crate) size_x: isize,
    pub(crate) size_y: isize,
}

fn get_color_byte(data: &ColorData) -> u8 {
    ((data.back_color as u8) << 4) + data.front_color as u8
}

impl Screen {
    fn advance_pos(&mut self) {
        (self.pos_y, self.pos_x) =
            if self.pos_x < self.size_x - 1 { (self.pos_y, self.pos_x + 1) } else { (self.pos_y + 1, 0) };
    }

    fn go_back(&mut self) {
        (self.pos_y, self.pos_x) =
            if self.pos_x > 0 { (self.pos_y, self.pos_x - 1) } else { (self.pos_y - 1, self.size_x - 1) };
    }

    pub(crate) fn newline(&mut self) {
        self.pos_x = 0;
        self.pos_y += 1;
    }

    fn get_color_addr(&self) -> *mut u8 {
        let total_pos = self.pos_x + self.size_x * self.pos_y;
        unsafe {
            self.mem.add((total_pos * 2 + 1) as usize)
        }
    }

    fn get_text_addr(&self) -> *mut u8 {
        let total_pos = self.pos_x + self.size_x * self.pos_y;
        unsafe {
            self.mem.add((total_pos * 2) as usize)
        }
    }

    pub const fn init() -> Screen {
        Screen { mem: 0xb8000 as *mut u8, pos_x: 0, pos_y: 0, size_x: 80, size_y: 25 }
    }

    pub fn print_str(&mut self, string: &[u8], color: &ColorData) {
        let color_byte = get_color_byte(color);
        unsafe {
            for char in string {
                self.get_text_addr().write_volatile(*char);
                self.get_color_addr().write_volatile(color_byte);
                self.advance_pos();
            }
        }
    }

    pub fn print_str_nl(&mut self, string: &[u8], color: &ColorData) {
        self.print_str(string, color);
        self.newline();
    }

    pub fn print_at(&self, string: &[u8], color: &ColorData, pos_x: isize, pos_y: isize) {
        let color_byte = get_color_byte(color);
        let total_pos = pos_x + self.size_x * pos_y;

        unsafe {
            let addr = self.mem.add((total_pos * 2) as usize);
            for (i, char) in string.iter().enumerate() {
                addr.add(2 * i).write_volatile(*char);
                addr.add(2 * i + 1).write_volatile(color_byte);
            }
        }
    }

    pub fn print_num_at(&mut self, num: u64, color: &ColorData, pos_x: isize, pos_y: isize) {
        let length = num_length(num);

        if length > 16 {
            return;
        }
        let mut mut_num = num;
        let mut buf = [0u8; 16];

        for i in 0..length {
            buf[i] = (mut_num % 10) as u8 + b'0';
            mut_num /= 10;
        }
        let mut buf_slice = &mut buf[..length];
        buf_slice.reverse();
        self.print_at(buf_slice, color, pos_x, pos_y);
    }

    pub fn control(&mut self, code: u8) {
        match code {
            0x08 => {
                self.go_back();
                self.print_at(&[b'A'], &BLACK_COLOR, self.pos_x, self.pos_y);
            }
            _ => {}
        }
    }
}
