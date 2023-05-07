#[allow(dead_code)]
mod code;
mod cpu;
mod instructions;
mod io;
mod memory;
#[allow(dead_code)]
mod runner;
mod tests;

pub use runner::{run, run_debug};
