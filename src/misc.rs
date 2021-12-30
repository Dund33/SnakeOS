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

pub fn num_to_ascii(num: u64) -> [u8; 8] {
    let length = num_length(num);

    if length > 8 {
        panic!()
    }
    let mut mut_num = num;
    let mut buf = [0u8; 8];

    for i in 0..length {
        buf[i] = (mut_num % 10) as u8 + b'0';
        mut_num /= 10;
    }
    buf.reverse();
    buf
}
