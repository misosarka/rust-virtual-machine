use super::cpu::CPU;
use std::{thread::sleep, time::Duration};

const MEMORY_LOCATION: usize = 0x80000000;
const MEMORY_OFFSET: usize = 0x00000010;
const PAUSE_MS: u64 = 0;
const PAUSE_DURATION: Duration = Duration::from_millis(PAUSE_MS);

pub fn run(start: &[u8]) {
    let mut c = CPU::new(start);
    while c.execute() {}
}

pub fn run_debug(start: &[u8]) {
    let mut c = CPU::new(start);
    c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    while c.execute() {
        if PAUSE_MS != 0 {
            sleep(PAUSE_DURATION);
        }
        c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    }
}
