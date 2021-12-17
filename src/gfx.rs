#[derive(Copy,Clone)]
pub enum Color{
    Black           = 0x00,
    Blue            = 0x01,
    Green           = 0x02,
    Cyan            = 0x03,
    Red             = 0x04,
    Magenta         = 0x05,
    Brown           = 0x06,
    White           = 0x07,
    Gray            = 0x08,
    LightBlue       = 0x09,
    LightGreen      = 0x0A,
    LightCyan       = 0x0B,
    LightRed        = 0x0C,
    LightMagenta    = 0x0D,
    Yellow          = 0x0E,
    BrightWhite     = 0x0F
}

pub struct ColorData{
    pub front_color: Color,
    pub back_color: Color
}

pub struct Screen{
    mem: *mut u8,
    pub(crate) pos_x: isize,
    pub(crate) pos_y: isize,
    pub(crate) size_x: isize,
    pub(crate) size_y: isize
}

fn get_color_byte(data: &ColorData) -> u8{
    ((data.back_color as u8) << 4) + data.front_color as u8
}

impl Screen{
    fn advance_pos(&mut self){
        (self.pos_y, self.pos_x) =
            if self.pos_x < self.size_x {(self.pos_y, self.pos_x+1)}
            else {(self.pos_y+1, 0)};
    }

    fn get_color_addr(&self) -> *mut u8{
        let total_pos = self.pos_x + self.size_x * self.pos_y;
        unsafe{
            self.mem.add((total_pos * 2 + 1) as usize)
        }
    }

    fn get_text_addr(&self) -> *mut u8{
        let total_pos = self.pos_x + self.size_x * self.pos_y;
        unsafe{
            self.mem.add((total_pos * 2) as usize)
        }
    }

    pub fn init() -> Screen{
        Screen{mem: 0xb8000 as *mut u8, pos_x: 0, pos_y: 0, size_x: 80, size_y:25}
    }

    pub fn print_str(&mut self, string: &[u8], color: &ColorData){
        let color_byte = get_color_byte(color);
        unsafe {
            for char in string{
                self.get_text_addr().write_volatile(*char);
                self.get_color_addr().write_volatile(color_byte);
                self.advance_pos();
            }
        }
    }
}
