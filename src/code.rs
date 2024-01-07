use super::instructions::*;

/// Change to true to see the state of the CPU after each instruction is executed.
///
/// Key:
/// * I = instruction pointer
/// * RFA = (general) register, flag (shows 'F' when true), address register
/// * SF = stack pointer, frame pointer
/// * frame = lowest 16 bytes of the current stack frame
pub(crate) const DEBUG: bool = false;

/// Change to run a different program defined below.
pub(crate) const CODE: &[u8] = COUNTDOWN;

const POWERS_OF_2: &[u8] = &[
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x01,
    MOV_REG_MEM,
    0x80,
    0x00,
    0x00,
    0x00,
    MOV_MEM_REG,
    0x80,
    0x00,
    0x00,
    0x00,
    ADD_MEM,
    0x80,
    0x00,
    0x00,
    0x00,
    JNF,
    0x00,
    0x00,
    0x00,
    0x05,
];

const STACK_TESTING: &[u8] = &[
    MOV_LIT_REG,
    0x12,
    0x34,
    0x56,
    0x78,
    PSH_REG,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x00,
    PUL_REG,
];

const ADD_LITERAL: &[u8] = &[
    MOV_LIT_REG,
    0x80,
    0x00,
    0x00,
    0x00,
    ADD_LIT,
    0x00,
    0x00,
    0x80,
    0x00,
];

const HELLO_WORLD: &[u8] = &[
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x48,
    OUT,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x65,
    OUT,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x6c,
    OUT,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x6c,
    OUT,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x6f,
    OUT,
];

const WAIT_FOR_SPACE: &[u8] = &[
    IPA, AND_LIT, 0x00, 0x00, 0x00, 0x01, JZE, 0x00, 0x00, 0x00, 0x00,
];

const COUNTDOWN: &[u8] = &[
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x06,
    DEC,
    ADD_LIT,
    0x00,
    0x00,
    0x00,
    0x30,
    OUT,
    PSH_REG,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x00,
    0x0a,
    OUT,
    MOV_LIT_REG,
    0x00,
    0x00,
    0x03,
    0xe8,
    SLP,
    PUL_REG,
    SUB_LIT,
    0x00,
    0x00,
    0x00,
    0x30,
    JNZ,
    0x00,
    0x00,
    0x00,
    0x05,
];

const RANDOM: &[u8] = &[RND];

const UTC_TIME: &[u8] = &[UTC];

const SLEEP: &[u8] = &[MOV_LIT_REG, 0x00, 0x00, 0x01, 0x00, SLP, TIM];
