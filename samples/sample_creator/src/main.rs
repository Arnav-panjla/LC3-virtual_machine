use std::fs::File;
use std::io::Write;

fn main() {
    let program = [
        0x30, 0x00, // Origin 0x3000
        0xF0, 0x21, // TRAP OUT
        0x20, 0x02,
        0xF0, 0x22, // TRAP PUTS
        0xF0, 0x25, // TRAP HALT
        0x00, 0x41, // 'A'
        0x00, 0x48, // 'H'
        0x00, 0x69, // 'i'
        0x00, 0x00, // NULL
    ];

    let mut file = File::create("../test.obj").unwrap();
    file.write_all(&program).unwrap();
}

