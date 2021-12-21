use crate::gfx::windows::Window;
use crate::Screen;

pub mod windows;
pub mod screen;

pub static mut SCREEN: Screen = Screen::init();

pub fn redraw_window(window: &Window) {
    unsafe {
        SCREEN.disp_window(window);
    }
}