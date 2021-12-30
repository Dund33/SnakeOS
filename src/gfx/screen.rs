use core::arch::global_asm;
use core::cmp::{max, min};

use crate::gfx::windows::Window;
use crate::gfx::Color::{Black, BrightWhite};
use crate::gfx::{Color, ColorData, TextInterface, WindowInterface};

pub static DEFAULT_COLOR: ColorData = ColorData {
    front_color: Color::BrightWhite,
    back_color: Color::Black,
};
global_asm!(include_str!("vga.s"));

extern "C" {
    fn _move_cursor(pos: u16);
}

pub struct Screen {
    pub(crate) color: ColorData,
    pub(crate) mem: *mut u8,
    pub(crate) pos_x: isize,
    pub(crate) pos_y: isize,
    pub(crate) size_x: isize,
    pub(crate) size_y: isize,
}

pub const fn get_color_byte(data: &ColorData) -> u8 {
    ((data.back_color as u8) << 4) + data.front_color as u8
}

pub fn get_color_data(addr: *const u8) -> ColorData {
    unsafe {
        let val = addr.read_volatile();

        let foreground = Color::try_from(val & 0xF).unwrap();
        let background = Color::try_from(val >> 4).unwrap();
        ColorData {
            back_color: background,
            front_color: foreground,
        }
    }
}

impl TextInterface for Screen {
    fn set_default_color(&mut self, color: ColorData) {
        self.color = color;
    }

    fn print_str(&mut self, string: &[u8], color: Option<ColorData>) {
        let text_color = color.unwrap_or_else(|| {
            let color_address = self.get_color_addr();
            get_color_data(color_address)
        });
        let color_byte = get_color_byte(&text_color);
        unsafe {
            for char in string {
                self.get_text_addr().write_volatile(*char);
                self.get_color_addr().write_volatile(color_byte);
                self.advance_pos();
            }
        }
        self.sync_cursor();
    }

    fn print_strln(&mut self, string: &[u8], color: Option<ColorData>) {
        self.print_str(string, color);
        self.newline();
        self.sync_cursor();
    }

    fn print_str_at(
        &mut self,
        string: &[u8],
        pos_x: isize,
        pos_y: isize,
        color: Option<ColorData>,
    ) {
        let old_coords = (self.pos_x, self.pos_y);
        (self.pos_x, self.pos_y) = (pos_x, pos_y);
        self.print_str(string, color);
        (self.pos_x, self.pos_y) = old_coords;
    }
}

impl WindowInterface for Screen {
    fn draw_window(&mut self, window: &Window) {
        for y in 0..window.size_y {
            let row_begin_pos_window = (y * window.size_x) as usize;
            let row_end_pos_window = row_begin_pos_window + window.size_x as usize - 1;
            let row_begin_pos_screen = (window.pos_x + self.size_x * (y + window.pos_y)) as usize;
            let buffer = &window.internal_buffer[row_begin_pos_window * 2..row_end_pos_window * 2];
            unsafe {
                self.mem
                    .add(row_begin_pos_screen * 2)
                    .copy_from_nonoverlapping(buffer.as_ptr(), window.size_x as usize * 2);
            }
        }
    }
}

impl Screen {
    pub(crate) const fn init() -> Self {
        let color = ColorData {
            back_color: Black,
            front_color: BrightWhite,
        };
        Screen {
            mem: 0xb8000 as *mut u8,
            pos_x: 0,
            pos_y: 0,
            size_x: 80,
            size_y: 25,
            color,
        }
    }

    pub fn sync_cursor(&self) {
        unsafe {
            let total_pos = (self.pos_x + self.pos_y * self.size_x) as u16;
            _move_cursor(total_pos);
        }
    }

    fn advance_pos(&mut self) {
        (self.pos_y, self.pos_x) = if self.pos_x < self.size_x - 1 {
            (self.pos_y, self.pos_x + 1)
        } else {
            (self.pos_y + 1, 0)
        };
        self.pos_y = min(self.pos_y, self.size_y);
    }

    fn go_back(&mut self) {
        (self.pos_y, self.pos_x) = if self.pos_x > 0 {
            (self.pos_y, self.pos_x - 1)
        } else {
            (self.pos_y - 1, self.size_x - 1)
        };
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
        unsafe { self.mem.add((total_pos * 2 + 1) as usize) }
    }

    fn get_text_addr(&self) -> *mut u8 {
        let total_pos = self.pos_x + self.size_x * self.pos_y;
        unsafe { self.mem.add((total_pos * 2) as usize) }
    }

    pub fn control(&mut self, code: u8) {
        match code {
            0x08 => {
                self.go_back();
                self.print_str(&[0], None);
                self.go_back();
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
