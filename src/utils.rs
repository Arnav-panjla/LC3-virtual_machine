// bit_count-wide signed value x to a full 16-bit value
pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if ((x >> (bit_count - 1)) & 1) != 0 {
        // if number is 11010 (-6) in 5 bits
        //output is 11111111111111010 (-6) in 16 bits
        x | (0xFFFF << bit_count) 
    } else {
        x
    }
}

// extracts "count" bits from value starting at bit position "start"
pub fn get_bits(value: u16, start: usize, count: usize) -> u16 {
    (value >> start) & ((1 << count) - 1)
}
