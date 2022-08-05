#[allow(dead_code)]
mod code;
mod cpu;
mod instructions;
mod memory;
#[allow(dead_code)]
mod runner;
mod tests;

use code::CODE;
use runner::run_debug;

fn main() {
    run_debug(CODE);
}
