use super::instructions::*;

pub const CODE: &[u32] = STACK_TESTING;

const POWERS_OF_2: &[u32] = &[
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
];

const STACK_TESTING: &[u32] = &[MOV_LIT_REG, 0x1234, PSH_REG, MOV_LIT_REG, 0, PUL_REG];
