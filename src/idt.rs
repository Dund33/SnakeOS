use core::arch::global_asm;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset1: u16,
    segment_selector: u16,
    flags: u16,
    offset2: u16,
    offset3: u32,
    reserved: u32
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
    fn pit_handler();
    fn _setup_pic();
    fn _load_idt(idtr: u64);
    fn _setup_pit();
}

fn isr_for_fn(handler: unsafe extern "C" fn()) -> IdtEntry {
    let addr = (handler as *const ()) as u64;
    let offset1 = (addr & 0xFFFF) as u16;
    let offset2 = ((addr >> 16) & 0xFFFF) as u16;
    let offset3 = (addr >> 32) as u32;
    let segment_selector = 0x10;
    let flags = 0xFFFF8E00;
    let reserved = 0;
    IdtEntry {
        offset1,
        segment_selector,
        flags,
        offset2,
	offset3,
	reserved
    }
}

pub fn setup_idt() -> [IdtEntry; 256] {
    let dummy_entry = isr_for_fn(_isr_bus);
    let mut idt = [dummy_entry; 256];

    idt[32] = isr_for_fn(pit_handler);
    idt[33] = isr_for_fn(_kbrd_isr);

    let idt_addr = idt.as_ptr() as u64;
    let idtr = Idtr {
        base: idt_addr,
        limit: 2048,
    };
    let idtr_addr = (&idtr as *const Idtr) as u64;
    unsafe {
        _setup_pic();
        _load_idt(idtr_addr);
        _setup_pit();
    }
    idt
}
