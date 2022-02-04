use crate::gfx::screen::get_color_byte;
use crate::gfx::Color::{BrightWhite, LightBlue};
use crate::gfx::ColorData;
use crate::Screen;
use core::sync::atomic::AtomicBool;

pub struct Window {
    pub pos_x: isize,
    pub pos_y: isize,
    pub size_x: isize,
    pub size_y: isize,
    pub color: ColorData,
    pub internal_buffer: [u8; 2048],
    pub screen: Screen,
    pub present: bool,
}

impl Window {
    pub fn new(pos_x: isize, pos_y: isize, size_x: isize, size_y: isize) -> Self {
        let color = ColorData {
            front_color: BrightWhite,
            back_color: LightBlue,
        };

        let mut buffer = [0; 2048];
        let color_byte = get_color_byte(&color);
        for i in 0..buffer.len() {
            if i % 2 == 1 {
                buffer[i] = color_byte;
            }
        }

        let screen = Screen {
            color,
            mem: 0 as *mut u8,
            pos_x: 0,
            pos_y: 0,
            size_x,
            size_y,
            busy: AtomicBool::new(false),
        };

        let mut window = Window {
            pos_x,
            pos_y,
            size_x,
            size_y,
            color,
            internal_buffer: buffer,
            screen,
            present: true,
        };
        window.screen.mem = window.internal_buffer.as_mut_ptr() as *mut u8;
        window
    }
}
