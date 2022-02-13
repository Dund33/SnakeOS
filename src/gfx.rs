use crate::gfx::windows::Window;
use crate::Screen;

pub mod screen;
pub mod windows;

pub static mut SCREEN: Screen = Screen::init();

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

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Color::Black),
            0x01 => Ok(Color::Blue),
            0x02 => Ok(Color::Green),
            0x03 => Ok(Color::Cyan),
            0x04 => Ok(Color::Red),
            0x05 => Ok(Color::Magenta),
            0x06 => Ok(Color::Brown),
            0x07 => Ok(Color::White),
            0x08 => Ok(Color::Gray),
            0x09 => Ok(Color::LightBlue),
            0x0A => Ok(Color::LightGreen),
            0x0B => Ok(Color::LightCyan),
            0x0C => Ok(Color::LightRed),
            0x0D => Ok(Color::LightMagenta),
            0x0E => Ok(Color::Yellow),
            0x0F => Ok(Color::BrightWhite),
            _ => Result::Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ColorData {
    pub front_color: Color,
    pub back_color: Color,
}

pub fn redraw_window(window: &Window) {
    unsafe {
        SCREEN.draw_window(window);
    }
}

pub trait TextInterface {
    fn set_default_color(&mut self, color: ColorData);
    fn print_str(&mut self, string: &[u8], color: Option<ColorData>);
    fn print_strln(&mut self, string: &[u8], color: Option<ColorData>);
    fn print_str_at(&mut self, string: &[u8], pos_x: isize, pos_y: isize, color: Option<ColorData>);
    fn keypress(&mut self, string: u8);
}

pub trait WindowInterface {
    fn draw_window(&mut self, window: &Window);
}

pub trait Console {
    fn enable_console_mode(&mut self);
    fn control_console(&mut self, code: u8);
    fn go_back_console(&mut self);
    fn newline_console(&mut self);
}
