pub(crate) const END: u8 = 0x00;

pub(crate) const MOV_LIT_REG: u8 = 0x10; // val
pub(crate) const MOV_LIT_AR: u8 = 0x11; // adr
pub(crate) const PSH_LIT: u8 = 0x12; // psv

pub(crate) const MOV_MEM_REG: u8 = 0x20; // get
pub(crate) const MOV_MEM_AR: u8 = 0x21; // adf

pub(crate) const MOV_REG_MEM: u8 = 0x30; // set
pub(crate) const MOV_REG_AR: u8 = 0x31; // tra
pub(crate) const MOV_REG_AAR: u8 = 0x32; // sea
pub(crate) const PSH_REG: u8 = 0x33; // psr

pub(crate) const MOV_AR_REG: u8 = 0x40; // tar
pub(crate) const PSH_AR: u8 = 0x41; // psa

pub(crate) const MOV_AAR_REG: u8 = 0x50; // gea
pub(crate) const MOV_AAR_AR: u8 = 0x51; // ada

pub(crate) const MOV_FP_REG: u8 = 0x60; // tfr
pub(crate) const MOV_FP_AR: u8 = 0x61; // tfa
pub(crate) const PSH_FP: u8 = 0x62; // psf

pub(crate) const PUL_REG: u8 = 0x70; // plr
pub(crate) const PUL_AR: u8 = 0x71; // pla

pub(crate) const ADD_LIT: u8 = 0x80;
pub(crate) const SUB_LIT: u8 = 0x81;
pub(crate) const MUL_LIT: u8 = 0x82;
pub(crate) const AND_LIT: u8 = 0x83;
pub(crate) const ORB_LIT: u8 = 0x84;
pub(crate) const XOR_LIT: u8 = 0x85;
pub(crate) const SHL_LIT: u8 = 0x86;
pub(crate) const SHR_LIT: u8 = 0x87;

pub(crate) const ADD_MEM: u8 = 0x88;
pub(crate) const SUB_MEM: u8 = 0x89;
pub(crate) const MUL_MEM: u8 = 0x8a;
pub(crate) const AND_MEM: u8 = 0x8b;
pub(crate) const ORB_MEM: u8 = 0x8c;
pub(crate) const XOR_MEM: u8 = 0x8d;
pub(crate) const SHL_MEM: u8 = 0x8e;
pub(crate) const SHR_MEM: u8 = 0x8f;

pub(crate) const ADD_AAR: u8 = 0x90;
pub(crate) const SUB_AAR: u8 = 0x91;
pub(crate) const MUL_AAR: u8 = 0x92;
pub(crate) const AND_AAR: u8 = 0x93;
pub(crate) const ORB_AAR: u8 = 0x94;
pub(crate) const XOR_AAR: u8 = 0x95;
pub(crate) const SHL_AAR: u8 = 0x96;
pub(crate) const SHR_AAR: u8 = 0x97;

pub(crate) const ADD_PUL: u8 = 0x98;
pub(crate) const SUB_PUL: u8 = 0x99;
pub(crate) const MUL_PUL: u8 = 0x9a;
pub(crate) const AND_PUL: u8 = 0x9b;
pub(crate) const ORB_PUL: u8 = 0x9c;
pub(crate) const XOR_PUL: u8 = 0x9d;
pub(crate) const SHL_PUL: u8 = 0x9e;
pub(crate) const SHR_PUL: u8 = 0x9f;

pub(crate) const AAD_LIT: u8 = 0xa0;
pub(crate) const ASB_LIT: u8 = 0xa1;
pub(crate) const AML_LIT: u8 = 0xa2;
pub(crate) const AAN_LIT: u8 = 0xa3;
pub(crate) const AOR_LIT: u8 = 0xa4;
pub(crate) const AXR_LIT: u8 = 0xa5;
pub(crate) const ASL_LIT: u8 = 0xa6;
pub(crate) const ASR_LIT: u8 = 0xa7;

pub(crate) const AAD_MEM: u8 = 0xa8;
pub(crate) const ASB_MEM: u8 = 0xa9;
pub(crate) const AML_MEM: u8 = 0xaa;
pub(crate) const AAN_MEM: u8 = 0xab;
pub(crate) const AOR_MEM: u8 = 0xac;
pub(crate) const AXR_MEM: u8 = 0xad;
pub(crate) const ASL_MEM: u8 = 0xae;
pub(crate) const ASR_MEM: u8 = 0xaf;

pub(crate) const AAD_REG: u8 = 0xb0;
pub(crate) const ASB_REG: u8 = 0xb1;
pub(crate) const AML_REG: u8 = 0xb2;
pub(crate) const AAN_REG: u8 = 0xb3;
pub(crate) const AOR_REG: u8 = 0xb4;
pub(crate) const AXR_REG: u8 = 0xb5;
pub(crate) const ASL_REG: u8 = 0xb6;
pub(crate) const ASR_REG: u8 = 0xb7;

pub(crate) const AAD_PUL: u8 = 0xb8;
pub(crate) const ASB_PUL: u8 = 0xb9;
pub(crate) const AML_PUL: u8 = 0xba;
pub(crate) const AAN_PUL: u8 = 0xbb;
pub(crate) const AOR_PUL: u8 = 0xbc;
pub(crate) const AXR_PUL: u8 = 0xbd;
pub(crate) const ASL_PUL: u8 = 0xbe;
pub(crate) const ASR_PUL: u8 = 0xbf;

pub(crate) const INC: u8 = 0xc0;
pub(crate) const DEC: u8 = 0xc1;
pub(crate) const NOT: u8 = 0xc2;

pub(crate) const JMP: u8 = 0xd0;
pub(crate) const JFL: u8 = 0xd1;
pub(crate) const JNF: u8 = 0xd2;
pub(crate) const JZE: u8 = 0xd3;
pub(crate) const JNZ: u8 = 0xd4;
pub(crate) const JMA: u8 = 0xd5;

pub(crate) const CAL: u8 = 0xe0;
pub(crate) const RET: u8 = 0xe1;

pub(crate) const IPA: u8 = 0xf0;
pub(crate) const IPB: u8 = 0xf1;
pub(crate) const IPC: u8 = 0xf2;
pub(crate) const OUT: u8 = 0xf3;
pub(crate) const SLP: u8 = 0xf4;
