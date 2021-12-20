use core::ops::Deref;

use bootloader::bootinfo::MemoryMap;

pub fn mem_total(mem_map: &MemoryMap) -> u64 {
    mem_map.deref().iter()
        .map(|x| { x.range.end_addr() - x.range.start_addr() })
        .sum()
}
