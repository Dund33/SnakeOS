pub struct Screen{
    mem: *mut u8,
    pub(crate) pos_x: isize,
    pub(crate) pos_y: isize,
    pub(crate) size_x: isize,
    pub(crate) size_y: isize
}

impl Screen{
    fn advance_pos(&mut self){
        (self.pos_x, self.pos_y) =
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

    pub fn print_str(&mut self, string: &[u8]){
        unsafe {
            for char in string{
                self.get_text_addr().write_volatile(*char);
                self.get_color_addr().write_volatile(0xB as u8);
                self.advance_pos();
            }
        }
    }
}
