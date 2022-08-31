#[cfg(test)]
mod tests {
    use crate::instructions::*;
    use crate::runner::run;

    #[test]
    #[should_panic(expected = "stack overflow")]
    fn stack_overflow() {
        run(&[PSH_LIT, 0x12, 0x34, 0x56, 0x78, JMP, 0x00]);
    }
}
