pub struct Screen{
    pub(crate) pos: isize
}

impl Screen{
    pub fn print_str(mut self, str: &[u8]){
        let mut vga_mem = (0xb8000 + self.pos) as *mut u8;
        unsafe {
            for char in str{
                *vga_mem.offset(self.pos) = char.clone();
                self.pos += 1;
                *vga_mem.offset(self.pos) = 0x0A;
                self.pos += 1;
            }
        }
    }
}
