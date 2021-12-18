use crate::Screen;

pub struct Keyboard{
    pub last_key: u8,
    pub buffer: [u8; 16]
}

impl Keyboard{
    pub const fn init_default() -> Keyboard{
        Keyboard{last_key:0, buffer: [0; 16]}
    }
}