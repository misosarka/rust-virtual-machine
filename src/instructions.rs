pub const END: u32 = 0x00;

pub const MOV_LIT_REG: u32 = 0x10; // val
pub const MOV_LIT_AR: u32 = 0x11; // adr
pub const PSH_LIT: u32 = 0x12; // psv

pub const MOV_MEM_REG: u32 = 0x20; // get
pub const MOV_MEM_AR: u32 = 0x21; // adf

pub const MOV_REG_MEM: u32 = 0x30; // set
pub const MOV_REG_AR: u32 = 0x31; // tra
pub const MOV_REG_AAR: u32 = 0x32; // sea
pub const PSH_REG: u32 = 0x33; // psr

pub const MOV_AR_REG: u32 = 0x40; // tar
pub const PSH_AR: u32 = 0x41; // psa

pub const MOV_AAR_REG: u32 = 0x50; // gea
pub const MOV_AAR_AR: u32 = 0x51; // ada

pub const MOV_SP_REG: u32 = 0x60; // tsr
pub const MOV_SP_AR: u32 = 0x61; // tsa
pub const PSH_SP: u32 = 0x62; // pss

pub const PUL_REG: u32 = 0x70; // plr
pub const PUL_AR: u32 = 0x71; // pla

pub const ADD_LIT: u32 = 0x80;
pub const SUB_LIT: u32 = 0x81;
pub const SBF_LIT: u32 = 0x82;
pub const MUL_LIT: u32 = 0x83;
pub const AND_LIT: u32 = 0x84;
pub const ORB_LIT: u32 = 0x85;
pub const XOR_LIT: u32 = 0x86;

pub const ADD_MEM: u32 = 0x90;
pub const SUB_MEM: u32 = 0x91;
pub const SBF_MEM: u32 = 0x92;
pub const MUL_MEM: u32 = 0x93;
pub const AND_MEM: u32 = 0x94;
pub const ORB_MEM: u32 = 0x95;
pub const XOR_MEM: u32 = 0x96;

pub const INC: u32 = 0xc0;
pub const DEC: u32 = 0xc1;
pub const NOT: u32 = 0xc2;

pub const JMP: u32 = 0xd0;
pub const JFL: u32 = 0xd1;
pub const JNF: u32 = 0xd2;
pub const JZE: u32 = 0xd3;
pub const JNZ: u32 = 0xd4;
pub const JMA: u32 = 0xd5;

pub const CAL: u32 = 0xe0;
pub const RET: u32 = 0xe1;
