mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use std::{thread::sleep, time::Duration};

const MEMORY_LOCATION: usize = 0x80000000;
const MEMORY_OFFSET: usize = 0x00000010;
const PAUSE_MS: u64 = 0;
const PAUSE_DURATION: Duration = Duration::from_millis(PAUSE_MS);

fn main() {
    /*
    // 16-bit code for powers of 2
    let mut c = CPU::new(vec![
        0x12, 0x01, // val 01
        0x11, 0x80, 0x00, // set 8000
        0x10, 0x80, 0x00, // get 8000
        0x30, 0x80, 0x00, // add 8000
        0x52, 0x00, 0x02, // jnf 0002
    ]); */
    /*
    // 8-bit code for powers of 2
    let mut c = CPU::new(vec![
        0x12, 0x01, // val 01
        0x11, 0x80, // set 80
        0x10, 0x80, // get 80
        0x30, 0x80, // add 80
        0x52, 0x02, // jnf 02
    ]); */
    // 32-bit code for powers of 2
    let mut c = CPU::new(vec![
        0x12, 0x01, // val 01
        0x11, 0x80, 0x00, 0x00, 0x00, // set 80000000
        0x10, 0x80, 0x00, 0x00, 0x00, // get 80000000
        0x30, 0x80, 0x00, 0x00, 0x00, // add 80000000
        0x52, 0x00, 0x00, 0x00, 0x02, // jnf 00000002
    ]);
    c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    while c.execute() {
        if PAUSE_MS != 0 {
            sleep(PAUSE_DURATION);
        }
        c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    }
}
