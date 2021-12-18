#[repr(C, packed)]
struct GDT {
    limit: u16,
    base: u16,
    base2: u8,
    access: u8,
    flag_limit: u8,
    base3: u8,
}

impl GDT {}