mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use std::{thread::sleep, time::Duration};

const MEMORY_LOCATION: u16 = 0x8000;
const MEMORY_OFFSET: u16 = 0x0010;
const PAUSE_MS: u64 = 0;
const PAUSE_DURATION: Duration = Duration::from_millis(PAUSE_MS);

fn main() {
    let mut c = CPU::new(vec![
        0x12, 0x01, // val 01
        0x11, 0x80, 0x00, // set 8000
        0x10, 0x80, 0x00, // get 8000
        0x30, 0x80, 0x00, // add 8000
        0x52, 0x00, 0x02, // jnf 0002
    ]);
    c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    while c.execute() {
        if PAUSE_MS != 0 {
            sleep(PAUSE_DURATION);
        }
        c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    }
}
