mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use instructions::*;
use std::{thread::sleep, time::Duration};

const MEMORY_LOCATION: usize = 0x80000000;
const MEMORY_OFFSET: usize = 0x00000010;
const PAUSE_MS: u64 = 0;
const PAUSE_DURATION: Duration = Duration::from_millis(PAUSE_MS);

fn main() {
    let mut c = CPU::new(&[
        MOV_LIT_REG,
        0x01,
        MOV_REG_MEM,
        0x80000000,
        MOV_MEM_REG,
        0x80000000,
        ADD_MEM,
        0x80000000,
        JNF,
        0x02,
    ]);
    c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    while c.execute() {
        if PAUSE_MS != 0 {
            sleep(PAUSE_DURATION);
        }
        c.print_info(MEMORY_LOCATION, MEMORY_OFFSET);
    }
}
