#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IdtEntry{
    offset1: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset2: u16,
    offset3: u32
}

extern "C"{
    fn isr_bus();
}

pub fn setup_idt() -> [IdtEntry; 256]{
    let bus_addr = 0 as usize;
    let offset1 = (bus_addr & 0xFFFF) as u16;
    let offset2 = ((bus_addr >> 16) & 0xFFFF) as u16;
    let offset3 = ((bus_addr >> 32) & 0xFFFF) as u32;
    let segment_selector = 0u16;
    let flags = 0x8E;
    let reserved = 0u8;
    [IdtEntry{offset1, offset2, offset3, segment_selector, flags, reserved}; 256]
}