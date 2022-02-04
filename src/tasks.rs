#[repr(C, packed)]
pub struct Task{
    pub edi: u32,
    pub esi: u32,
    pub edx: u32,
    pub ecx: u32,
    pub ebx: u32,
    pub instruction_pointer: u32,
    pub eax: u32,
    pub stack_pointer: u32,
}

impl Task{
    pub fn new(tgt: unsafe extern "C" fn(), stack: u32) -> Self{
        Task{stack_pointer: stack, instruction_pointer: (tgt as *const () ) as u32, eax: 0x1, ebx: 0x22, ecx: 0x333, edx: 0x4444, esi: 0x55555, edi: 0x666666}
    }

    pub const fn empty() -> Self{
        Task{stack_pointer: 0, instruction_pointer: 0, eax: 0x11, ebx: 0x222, ecx: 0x333, edx: 0x4444, esi: 0x55555, edi: 0x666666}
    }
}