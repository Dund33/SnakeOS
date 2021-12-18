use core::arch::global_asm;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset1: u16,
    segment_selector: u16,
    reserved: u8,
    flags: u8,
    offset2: u16,
    offset3: u32,
    reserved2: u32,
}

#[repr(C, packed)]
pub struct Idtr {
    pub(crate) limit: u16,
    pub(crate) base: u64,
}

global_asm!(include_str!("idt.s"));

extern "C" {
    fn _isr_bus();
    fn _kbrd_isr();
    fn _setup_pic();
    fn _load_idt(idtr: u64);
}

fn isr_for_fn(handler: unsafe extern "C" fn()) -> IdtEntry {
    let addr = (handler as *const ()) as usize;
    let offset1 = (addr & 0xFFFF) as u16;
    let offset2 = ((addr >> 16) & 0xFFFF) as u16;
    let offset3 = (addr >> 32) as u32;
    let segment_selector = 0x8;
    let flags = 0x8E;
    let reserved = 0;
    let reserved2 = 0;
    IdtEntry { offset1, segment_selector, reserved, flags, offset2, offset3, reserved2 }
}

pub fn setup_idt() -> [IdtEntry; 256] {
    let dummy_entry = isr_for_fn(_isr_bus);
    let mut idt = [dummy_entry; 256];

    idt[33] = isr_for_fn(_kbrd_isr);

    let idt_addr = idt.as_ptr() as u64;
    let idtr = Idtr { base: idt_addr, limit: 4096 };
    let idtr_addr = (&idtr as *const Idtr) as u64;
    unsafe {
        _setup_pic();
        _load_idt(idtr_addr);
    }
    idt
}