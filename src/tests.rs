#[cfg(test)]
mod tests {
    use crate::instructions::*;
    use crate::runner::run;

    #[test]
    #[should_panic(expected = "stack overflow")]
    fn stack_overflow() {
        run(&[PSH_LIT, 0x12345678, JMP, 0x00000000]);
    }
}
