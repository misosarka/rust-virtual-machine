use super::cpu::Cpu;
use std::{thread::sleep, time::Duration};

const PAUSE_MS: u64 = 0;
const PAUSE_DURATION: Duration = Duration::from_millis(PAUSE_MS);

pub fn run(start: &[u8]) {
    let mut c = Cpu::new(start);
    while c.execute() {}
}

pub fn run_debug(start: &[u8]) {
    let mut c = Cpu::new(start);
    c.print_info();
    while c.execute() {
        if PAUSE_MS != 0 {
            sleep(PAUSE_DURATION);
        }
        c.print_info();
    }
}
