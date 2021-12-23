use core::arch::global_asm;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset1: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset2: u16,
}

#[repr(C, packed)]
pub struct Idtr {
    pub(crate) limit: u16,
    pub(crate) base: u32,
}

global_asm!(include_str!("idt.s"));
global_asm!(include_str!("isr.s"));

extern "C" {
    fn _isr_bus();
    fn _kbrd_isr();
    fn _pit_isr();
    fn _setup_pic();
    fn _load_idt(idtr: u32);
    fn _setup_pit();
}

fn isr_for_fn(handler: unsafe extern "C" fn()) -> IdtEntry {
    let addr = (handler as *const ()) as u32;
    let offset1 = (addr & 0xFFFF) as u16;
    let offset2 = ((addr >> 16) & 0xFFFF) as u16;
    let segment_selector = 0x10;
    let flags = 0x8E;
    let reserved = 0;
    IdtEntry { offset1, segment_selector, reserved, flags, offset2}
}

pub fn setup_idt() -> [IdtEntry; 256] {
    let dummy_entry = isr_for_fn(_isr_bus);
    let mut idt = [dummy_entry; 256];

    idt[32] = isr_for_fn(_pit_isr);
    idt[33] = isr_for_fn(_kbrd_isr);

    let idt_addr = idt.as_ptr() as u32;
    let idtr = Idtr { base: idt_addr, limit: 2048 };
    let idtr_addr = (&idtr as *const Idtr) as u32;
    unsafe {
        _setup_pic();
        _load_idt(idtr_addr);
        _setup_pit();
    }
    idt
}