use core::arch::global_asm;
use core::cmp::{max, min};

use crate::gfx::windows::Window;
use crate::misc::num_length;

pub static DEFAULT_COLOR: ColorData = ColorData {
    front_color: Color::BrightWhite,
    back_color: Color::Black,
};
global_asm!(include_str!("vga.s"));

extern "C" {
    fn _move_cursor(pos: u16);
}

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
    pub(crate) mem: *mut u8,
    pub(crate) pos_x: isize,
    pub(crate) pos_y: isize,
    pub(crate) size_x: isize,
    pub(crate) size_y: isize,
}

pub const fn get_color_byte(data: &ColorData) -> u8 {
    ((data.back_color as u8) << 4) + data.front_color as u8
}

impl Screen {
    pub fn disp_window(&mut self, window: &Window) {
        for y in 0..window.size_y {
            let row_begin_pos_window = (y * window.size_x) as usize;
            let row_end_pos_window = row_begin_pos_window + window.size_x as usize - 1;
            let row_begin_pos_screen = (window.pos_x + self.size_x * (y + window.pos_y)) as usize;
            let buffer = &window.internal_buffer[row_begin_pos_window * 2..row_end_pos_window * 2];
            unsafe {
                self.mem.add(row_begin_pos_screen * 2)
                    .copy_from_nonoverlapping(buffer.as_ptr(), window.size_x as usize * 2);
            }
        }
    }

    pub fn sync_cursor(&self) {
        unsafe {
            let total_pos = (self.pos_x + self.pos_y * self.size_y) as u16;
            _move_cursor(total_pos);
        }
    }

    fn advance_pos(&mut self) {
        (self.pos_y, self.pos_x) =
            if self.pos_x < self.size_x - 1 { (self.pos_y, self.pos_x + 1) } else { (self.pos_y + 1, 0) };
        self.pos_y = min(self.pos_y, self.size_y);
    }

    fn go_back(&mut self) {
        (self.pos_y, self.pos_x) =
            if self.pos_x > 0 { (self.pos_y, self.pos_x - 1) } else { (self.pos_y - 1, self.size_x - 1) };
        self.pos_y = max(0, self.pos_y);
    }

    fn down(&mut self) {
        self.pos_y = min(self.pos_y + 1, self.size_y);
    }

    fn up(&mut self) {
        self.pos_y = min(max(0, self.pos_y - 1), self.size_y);
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

    pub const fn init() -> Self {
        Screen { mem: 0xb8000 as *mut u8, pos_x: 0, pos_y: 0, size_x: 80, size_y: 25 }
    }

    pub fn print_str(&mut self, string: &[u8], color: &ColorData, sync_cursor: bool) {
        let color_byte = get_color_byte(color);
        unsafe {
            for char in string {
                self.get_text_addr().write_volatile(*char);
                self.get_color_addr().write_volatile(color_byte);
                self.advance_pos();
            }
        }
        if sync_cursor {
            self.sync_cursor();
        }
    }

    pub fn print_num(&mut self, num: u64,
                     color: &ColorData,
                     sync_cursor: bool) {
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
        let buf_slice = &mut buf[..length];
        buf_slice.reverse();

        self.print_str(buf_slice, color, sync_cursor);
    }

    pub fn print_str_nl(&mut self, string: &[u8], color: &ColorData, sync_cursor: bool) {
        self.print_str(string, color, false);
        self.newline();

        if sync_cursor {
            self.sync_cursor();
        }
    }

    pub fn print_at(&mut self,
                    string: &[u8],
                    color: &ColorData,
                    pos_x: isize,
                    pos_y: isize,
                    sync_cursor: bool) {
        let old_coords = (self.pos_x, self.pos_y);
        (self.pos_x, self.pos_y) = (pos_x, pos_y);
        self.print_str(string, color, sync_cursor);
        (self.pos_x, self.pos_y) = old_coords;
    }

    pub fn print_num_at(&mut self, num: u64,
                        color: &ColorData,
                        pos_x: isize,
                        pos_y: isize,
                        sync_cursor: bool) {
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
        let buf_slice = &mut buf[..length];
        buf_slice.reverse();
        self.print_at(buf_slice, color, pos_x, pos_y, sync_cursor);
    }

    pub fn control(&mut self, code: u8) {
        match code {
            0x08 => {
                self.go_back();
                self.print_at(&[0],
                              &DEFAULT_COLOR,
                              self.pos_x,
                              self.pos_y,
                              false);
                self.sync_cursor();
            }
            0x0D => {
                self.newline();
                self.sync_cursor();
            }
            0xF1 => {
                self.up();
                self.sync_cursor();
            }
            0xF2 => {
                self.go_back();
                self.sync_cursor();
            }
            0xF3 => {
                self.down();
                self.sync_cursor();
            }
            0xF4 => {
                self.advance_pos();
                self.sync_cursor();
            }
            _ => {}
        }
    }
}

