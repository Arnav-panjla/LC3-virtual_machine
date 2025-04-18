pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if ((x >> (bit_count - 1)) & 1) != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

pub fn get_bits(value: u16, start: usize, count: usize) -> u16 {
    (value >> start) & ((1 << count) - 1)
}
