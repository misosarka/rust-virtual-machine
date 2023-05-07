#[allow(dead_code)]
mod code;
mod cpu;
mod instructions;
mod io;
mod memory;
#[allow(dead_code)]
mod runner;
mod tests;

use code::{CODE, DEBUG};
use runner::{run, run_debug};

fn main() {
    if DEBUG {
        run_debug(CODE);
    } else {
        run(CODE);
    }
}
