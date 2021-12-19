use crate::kbrd::Key::{Control, Letter};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Key {
    Letter(u8),
    Control(u8),
    None,
}

pub fn scan2ascii(scancode: u8) -> Key {
    match scancode {
        0x01 => Letter(0x1B),
        0x02 => Letter(b'1'),
        0x03 => Letter(b'2'),
        0x04 => Letter(b'3'),
        0x05 => Letter(b'4'),
        0x06 => Letter(b'5'),
        0x07 => Letter(b'6'),
        0x08 => Letter(b'7'),
        0x09 => Letter(b'8'),
        0x0A => Letter(b'9'),
        0x0B => Letter(b'0'),
        0x0E => Control(0x08),
        0x10 => Letter(b'Q'),
        0x11 => Letter(b'W'),
        0x12 => Letter(b'E'),
        0x13 => Letter(b'R'),
        0x14 => Letter(b'T'),
        0x15 => Letter(b'Y'),
        0x16 => Letter(b'U'),
        0x17 => Letter(b'I'),
        0x18 => Letter(b'O'),
        0x19 => Letter(b'P'),
        0x1E => Letter(b'A'),
        0x1F => Letter(b'S'),
        0x20 => Letter(b'D'),
        0x21 => Letter(b'F'),
        0x22 => Letter(b'G'),
        0x23 => Letter(b'H'),
        0x24 => Letter(b'J'),
        0x25 => Letter(b'K'),
        0x26 => Letter(b'L'),
        0x2C => Letter(b'Z'),
        0x2D => Letter(b'X'),
        0x2E => Letter(b'C'),
        0x2F => Letter(b'V'),
        0x30 => Letter(b'B'),
        0x31 => Letter(b'N'),
        0x32 => Letter(b'M'),
        0x39 => Letter(b' '),
        _ => Key::None
    }
}
