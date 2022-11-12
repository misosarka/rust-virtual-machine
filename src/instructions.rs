pub const END: u8 = 0x00;

pub const MOV_LIT_REG: u8 = 0x10; // val
pub const MOV_LIT_AR: u8 = 0x11; // adr
pub const PSH_LIT: u8 = 0x12; // psv

pub const MOV_MEM_REG: u8 = 0x20; // get
pub const MOV_MEM_AR: u8 = 0x21; // adf

pub const MOV_REG_MEM: u8 = 0x30; // set
pub const MOV_REG_AR: u8 = 0x31; // tra
pub const MOV_REG_AAR: u8 = 0x32; // sea
pub const PSH_REG: u8 = 0x33; // psr

pub const MOV_AR_REG: u8 = 0x40; // tar
pub const PSH_AR: u8 = 0x41; // psa

pub const MOV_AAR_REG: u8 = 0x50; // gea
pub const MOV_AAR_AR: u8 = 0x51; // ada

pub const MOV_FP_REG: u8 = 0x60; // tfr
pub const MOV_FP_AR: u8 = 0x61; // tfa
pub const PSH_FP: u8 = 0x62; // psf

pub const PUL_REG: u8 = 0x70; // plr
pub const PUL_AR: u8 = 0x71; // pla

pub const ADD_LIT: u8 = 0x80;
pub const SUB_LIT: u8 = 0x81;
pub const MUL_LIT: u8 = 0x82;
pub const AND_LIT: u8 = 0x83;
pub const ORB_LIT: u8 = 0x84;
pub const XOR_LIT: u8 = 0x85;
pub const SHL_LIT: u8 = 0x86;
pub const SHR_LIT: u8 = 0x87;

pub const ADD_MEM: u8 = 0x88;
pub const SUB_MEM: u8 = 0x89;
pub const MUL_MEM: u8 = 0x8a;
pub const AND_MEM: u8 = 0x8b;
pub const ORB_MEM: u8 = 0x8c;
pub const XOR_MEM: u8 = 0x8d;
pub const SHL_MEM: u8 = 0x8e;
pub const SHR_MEM: u8 = 0x8f;

pub const ADD_AAR: u8 = 0x90;
pub const SUB_AAR: u8 = 0x91;
pub const MUL_AAR: u8 = 0x92;
pub const AND_AAR: u8 = 0x93;
pub const ORB_AAR: u8 = 0x94;
pub const XOR_AAR: u8 = 0x95;
pub const SHL_AAR: u8 = 0x96;
pub const SHR_AAR: u8 = 0x97;

pub const ADD_PUL: u8 = 0x98;
pub const SUB_PUL: u8 = 0x99;
pub const MUL_PUL: u8 = 0x9a;
pub const AND_PUL: u8 = 0x9b;
pub const ORB_PUL: u8 = 0x9c;
pub const XOR_PUL: u8 = 0x9d;
pub const SHL_PUL: u8 = 0x9e;
pub const SHR_PUL: u8 = 0x9f;

pub const AAD_LIT: u8 = 0xa0;
pub const ASB_LIT: u8 = 0xa1;
pub const AML_LIT: u8 = 0xa2;
pub const AAN_LIT: u8 = 0xa3;
pub const AOR_LIT: u8 = 0xa4;
pub const AXR_LIT: u8 = 0xa5;
pub const ASL_LIT: u8 = 0xa6;
pub const ASR_LIT: u8 = 0xa7;

pub const AAD_MEM: u8 = 0xa8;
pub const ASB_MEM: u8 = 0xa9;
pub const AML_MEM: u8 = 0xaa;
pub const AAN_MEM: u8 = 0xab;
pub const AOR_MEM: u8 = 0xac;
pub const AXR_MEM: u8 = 0xad;
pub const ASL_MEM: u8 = 0xae;
pub const ASR_MEM: u8 = 0xaf;

pub const AAD_REG: u8 = 0xb0;
pub const ASB_REG: u8 = 0xb1;
pub const AML_REG: u8 = 0xb2;
pub const AAN_REG: u8 = 0xb3;
pub const AOR_REG: u8 = 0xb4;
pub const AXR_REG: u8 = 0xb5;
pub const ASL_REG: u8 = 0xb6;
pub const ASR_REG: u8 = 0xb7;

pub const AAD_PUL: u8 = 0xb8;
pub const ASB_PUL: u8 = 0xb9;
pub const AML_PUL: u8 = 0xba;
pub const AAN_PUL: u8 = 0xbb;
pub const AOR_PUL: u8 = 0xbc;
pub const AXR_PUL: u8 = 0xbd;
pub const ASL_PUL: u8 = 0xbe;
pub const ASR_PUL: u8 = 0xbf;

pub const INC: u8 = 0xc0;
pub const DEC: u8 = 0xc1;
pub const NOT: u8 = 0xc2;

pub const JMP: u8 = 0xd0;
pub const JFL: u8 = 0xd1;
pub const JNF: u8 = 0xd2;
pub const JZE: u8 = 0xd3;
pub const JNZ: u8 = 0xd4;
pub const JMA: u8 = 0xd5;

pub const CAL: u8 = 0xe0;
pub const RET: u8 = 0xe1;
