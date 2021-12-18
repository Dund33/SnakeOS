pub struct Keyboard {
    pub last_key: u8,
    pub buffer: [u8; 16],
}

impl Keyboard {
    pub const fn init_default() -> Keyboard {
        Keyboard { last_key: 0, buffer: [0; 16] }
    }
}

pub fn scan2ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x01 => Some(0x1B),
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        0x10 => Some(b'Q'),
        0x11 => Some(b'W'),
        0x12 => Some(b'E'),
        0x13 => Some(b'R'),
        0x14 => Some(b'T'),
        0x15 => Some(b'Y'),
        0x16 => Some(b'U'),
        0x17 => Some(b'I'),
        0x18 => Some(b'O'),
        0x19 => Some(b'P'),
        0x1E => Some(b'A'),
        0x1F => Some(b'S'),
        0x20 => Some(b'D'),
        0x21 => Some(b'F'),
        0x22 => Some(b'G'),
        0x23 => Some(b'H'),
        0x24 => Some(b'J'),
        0x25 => Some(b'K'),
        0x26 => Some(b'L'),
        0x2C => Some(b'Z'),
        0x2D => Some(b'X'),
        0x2E => Some(b'C'),
        0x2F => Some(b'V'),
        0x30 => Some(b'B'),
        0x31 => Some(b'N'),
        0x32 => Some(b'M'),
        0x39 => Some(b' '),
        _ => None
    }
}
