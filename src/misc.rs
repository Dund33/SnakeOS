use core::arch::asm;

pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

pub const fn num_length(num: u64) -> usize {
    let mut length = 0;
    let mut num_mut = num;

    while num_mut > 0 {
        num_mut /= 10;
        length += 1;
    }
    length
}